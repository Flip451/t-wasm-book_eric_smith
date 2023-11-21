use web_sys::HtmlImageElement;

use crate::engine::Renderer;

use super::{sprite::SpriteSheet, Point};

const FLOOR: f32 = 475.;
const IDLE_FRAME_NAME: &str = "Idle";
const RUNNING_FRAME_NAME: &str = "Run";

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
}

// すべての状態に共通する情報
pub struct RedHatBoyContext {
    frame: u8,
    position: Point,
    velocity: Point,
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
}

struct Running;
impl RedHatBoyState<Running> {
    fn frame_name(&self) -> &str {
        RUNNING_FRAME_NAME
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
}

// 状態と列挙子を関連付ける
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
    fn run(self) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context,
            _state: Running,
        }
    }
}

// イベント
enum Event {
    Run,
}

// イベントを受け取って状態遷移を行うメソッド
impl RedHatBoyStateMachine {
    fn transition(self, event: Event) -> Self {
        match (self, event) {
            (RedHatBoyStateMachine::Idle(state), Event::Run) => state.run().into(),
            (state_machine, _) => state_machine,
        }
    }
}
