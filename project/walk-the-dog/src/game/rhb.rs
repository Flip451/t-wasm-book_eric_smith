use web_sys::HtmlImageElement;

use crate::engine::Renderer;

use super::{sprite::SpriteSheet, Point};

const FLOOR: f32 = 475.;
const RUNNING_SPEED: f32 = 3.;

// フレーム名
const IDLE_FRAME_NAME: &str = "Idle";
const RUNNING_FRAME_NAME: &str = "Run";

// フレーム数
const IDLE_FRAME_COUNT: u8 = 29;
const RUNNING_FRAME_COUNT: u8 = 24;

pub struct RedHatBoy {
    state_machine: RedHatBoyStateMachine,
    sprite_sheet: SpriteSheet,
    image: HtmlImageElement,
}

impl RedHatBoy {
    pub fn new(sprite_sheet: SpriteSheet, image: HtmlImageElement) -> Self {
        Self {
            state_machine: RedHatBoyStateMachine::Idle(RedHatBoyState::<Idle>::new()),
            sprite_sheet,
            image,
        }
    }

    pub fn draw(&self, renderer: &Renderer) {
        let frame_name = format!(
            "{} ({}).png",
            self.state_machine.frame_name(),
            self.state_machine.context().frame / 3 + 1
        );

        // シートの中から指定の画像（Run (*).png）の位置を取得
        let sprite = self
            .sprite_sheet
            .frames
            .get(&frame_name)
            .expect("Cell not found");

        // キャンバスに指定の画像を描画
        renderer.draw_image(
            &self.image,
            &sprite.to_rect_on_sheet(),
            &sprite.to_rect_on_canvas(
                self.state_machine.context().position.x,
                self.state_machine.context().position.y,
            ),
        );
    }

    pub fn update(&mut self) {
        self.state_machine.update();
    }

    pub fn run_right(&mut self) {
        self.state_machine.transition(Event::RunRight);
    }

    pub fn run_left(&mut self) {
        self.state_machine.transition(Event::RunLeft);
    }
}

// すべての状態に共通する情報
#[derive(Clone)]
pub struct RedHatBoyContext {
    frame: u8,
    position: Point,
    velocity: Point,
}

impl RedHatBoyContext {
    fn update_frame(&mut self, frame_count: u8) {
        self.frame = (self.frame + 1) % frame_count;
    }

    fn reset_frame(&mut self) {
        self.frame = 0;
    }

    fn update_position(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    fn run_right(&mut self) {
        self.velocity.x = RUNNING_SPEED;
    }

    fn run_left(&mut self) {
        self.velocity.x = -RUNNING_SPEED;
    }
}

// RHB の状態を表す構造体
struct RedHatBoyState<S> {
    context: RedHatBoyContext,
    _state: S,
}

impl<S> RedHatBoyState<S> {
    fn context(&self) -> &RedHatBoyContext {
        &self.context
    }
}

// 状態を表す型
struct Idle;
impl RedHatBoyState<Idle> {
    fn frame_name(&self) -> &str {
        IDLE_FRAME_NAME
    }

    fn update(&mut self) {
        self.context.update_frame(IDLE_FRAME_COUNT);
        self.context.update_position();
    }
}

struct Running;
impl RedHatBoyState<Running> {
    fn frame_name(&self) -> &str {
        RUNNING_FRAME_NAME
    }

    fn update(&mut self) {
        self.context.update_frame(RUNNING_FRAME_COUNT);
        self.context.update_position();
    }
}

// ステートマシーン本体
enum RedHatBoyStateMachine {
    Idle(RedHatBoyState<Idle>),
    Running(RedHatBoyState<Running>),
}

impl RedHatBoyStateMachine {
    fn frame_name(&self) -> &str {
        match self {
            RedHatBoyStateMachine::Idle(state) => state.frame_name(),
            RedHatBoyStateMachine::Running(state) => state.frame_name(),
        }
    }

    fn context(&self) -> &RedHatBoyContext {
        match self {
            RedHatBoyStateMachine::Idle(state) => &state.context(),
            RedHatBoyStateMachine::Running(state) => &state.context(),
        }
    }

    fn update(&mut self) {
        match self {
            RedHatBoyStateMachine::Idle(ref mut state) => {
                state.update();
            }
            RedHatBoyStateMachine::Running(ref mut state) => {
                state.update();
            }
        }
    }
}

// 状態と列挙子を関連付ける
impl From<RedHatBoyState<Idle>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Idle>) -> Self {
        RedHatBoyStateMachine::Idle(state)
    }
}

impl From<RedHatBoyState<Running>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Running>) -> Self {
        RedHatBoyStateMachine::Running(state)
    }
}

// 初期状態の定義
impl RedHatBoyState<Idle> {
    fn new() -> Self {
        Self {
            context: RedHatBoyContext {
                frame: 0,
                position: Point { x: 0., y: FLOOR },
                velocity: Point { x: 0., y: 0. },
            },
            _state: Idle,
        }
    }
}

// 状態遷移を定義
impl RedHatBoyState<Idle> {
    fn run_right(&self) -> RedHatBoyState<Running> {
        let mut context = self.context.clone();
        context.reset_frame();
        context.run_right();
        RedHatBoyState {
            context: context,
            _state: Running,
        }
    }
}

impl RedHatBoyState<Running> {
    fn run_right(&self) -> RedHatBoyState<Running> {
        let mut context = self.context.clone();
        context.reset_frame();
        context.run_right();
        RedHatBoyState {
            context: context,
            _state: Running,
        }
    }

    fn run_left(&self) -> RedHatBoyState<Running> {
        let mut context = self.context.clone();
        context.reset_frame();
        context.run_left();
        RedHatBoyState {
            context: context,
            _state: Running,
        }
    }
}

// イベント
enum Event {
    RunRight,
    RunLeft,
}

// イベントを受け取って状態遷移を行うメソッド
impl RedHatBoyStateMachine {
    fn transition(&mut self, event: Event) {
        match (&self, event) {
            (&RedHatBoyStateMachine::Idle(ref state), Event::RunRight) => {
                *self = state.run_right().into()
            }
            (&RedHatBoyStateMachine::Running(ref state), Event::RunLeft) => {
                *self = state.run_left().into()
            }
            (&RedHatBoyStateMachine::Running(ref state), Event::RunRight) => {
                *self = state.run_right().into()
            }
            _ => {}
        };
    }
}
