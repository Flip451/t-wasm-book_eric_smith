use web_sys::HtmlImageElement;

use crate::engine::renderer::Renderer;

use self::red_hat_boy_states::*;
use super::sprite::SpriteSheet;

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
        self.state_machine.transition(Event::Update);
    }

    pub fn run_right(&mut self) {
        self.state_machine.transition(Event::RunRight);
    }

    pub fn run_left(&mut self) {
        self.state_machine.transition(Event::RunLeft);
    }

    pub fn slide(&mut self) {
        self.state_machine.transition(Event::Slide);
    }

    pub fn jump(&mut self) {
        self.state_machine.transition(Event::Jump);
    }
}

// ステートマシーン本体
enum RedHatBoyStateMachine {
    Idle(RedHatBoyState<Idle>),
    Running(RedHatBoyState<Running>),
    Sliding(RedHatBoyState<Sliding>),
    Jumping(RedHatBoyState<Jumping>),
}

impl RedHatBoyStateMachine {
    fn frame_name(&self) -> &str {
        match self {
            RedHatBoyStateMachine::Idle(state) => state.frame_name(),
            RedHatBoyStateMachine::Running(state) => state.frame_name(),
            RedHatBoyStateMachine::Sliding(state) => state.frame_name(),
            RedHatBoyStateMachine::Jumping(state) => state.frame_name(),
        }
    }

    fn context(&self) -> &RedHatBoyContext {
        match self {
            RedHatBoyStateMachine::Idle(state) => &state.context(),
            RedHatBoyStateMachine::Running(state) => &state.context(),
            RedHatBoyStateMachine::Sliding(state) => &state.context(),
            RedHatBoyStateMachine::Jumping(state) => &state.context(),
        }
    }

    // イベントを受け取って状態遷移を行うメソッド
    fn transition(&mut self, event: Event) {
        match (&self, event) {
            // キー入力による状態遷移
            (RedHatBoyStateMachine::Idle(ref state), Event::RunRight) => {
                *self = state.start_run().into()
            }
            (RedHatBoyStateMachine::Running(ref state), Event::RunLeft) => {
                *self = state.run_left().into()
            }
            (RedHatBoyStateMachine::Running(ref state), Event::RunRight) => {
                *self = state.run_right().into()
            }
            (RedHatBoyStateMachine::Running(ref state), Event::Slide) => {
                *self = state.slide().into()
            }
            (RedHatBoyStateMachine::Running(ref state), Event::Jump) => *self = state.jump().into(),
            // 時間経過による update 処理
            (RedHatBoyStateMachine::Idle(ref state), Event::Update) => {
                *self = state.update().into()
            }
            (RedHatBoyStateMachine::Running(ref state), Event::Update) => {
                *self = state.update().into()
            }
            (RedHatBoyStateMachine::Sliding(ref state), Event::Update) => {
                *self = state.update().into()
            }
            (RedHatBoyStateMachine::Jumping(ref state), Event::Update) => {
                *self = state.update().into()
            }
            _ => {}
        };
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

impl From<RedHatBoyState<Sliding>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Sliding>) -> Self {
        RedHatBoyStateMachine::Sliding(state)
    }
}

impl From<RedHatBoyState<Jumping>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Jumping>) -> Self {
        RedHatBoyStateMachine::Jumping(state)
    }
}

impl From<SlidngEndState> for RedHatBoyStateMachine {
    fn from(state: SlidngEndState) -> Self {
        match state {
            SlidngEndState::Sliding(state) => RedHatBoyStateMachine::Sliding(state),
            SlidngEndState::Complete(state) => RedHatBoyStateMachine::Running(state),
        }
    }
}

impl From<JumpEndState> for RedHatBoyStateMachine {
    fn from(state: JumpEndState) -> Self {
        match state {
            JumpEndState::Jumping(state) => RedHatBoyStateMachine::Jumping(state),
            JumpEndState::Complete(state) => RedHatBoyStateMachine::Running(state),
        }
    }
}

// イベント
enum Event {
    RunRight,
    RunLeft,
    Slide,
    Jump,
    Update,
}

mod red_hat_boy_states {
    use crate::engine::Point;

    // 座標系関連
    const FLOOR: f32 = 475.;
    const RUNNING_SPEED: f32 = 3.;
    const JUMP_SPEED: f32 = -25.;
    const GRAVITY: f32 = 1.;

    // フレーム名
    const IDLE_FRAME_NAME: &str = "Idle";
    const RUNNING_FRAME_NAME: &str = "Run";
    const SLIDING_FRAME_NAME: &str = "Slide";
    const JUMPING_FRAME_NAME: &str = "Jump";

    // フレーム数
    const IDLE_FRAME_COUNT: u8 = 29;
    const RUNNING_FRAME_COUNT: u8 = 24;
    const SLIDING_FRAME_COUNT: u8 = 14;
    const JUMPING_FRAME_COUNT: u8 = 35;

    // RHB の状態を表す構造体
    pub(super) struct RedHatBoyState<S> {
        context: RedHatBoyContext,
        _state: S,
    }

    impl<S> RedHatBoyState<S> {
        pub(super) fn context(&self) -> &RedHatBoyContext {
            &self.context
        }
    }

    // 状態を表す型
    pub(super) struct Idle;
    impl RedHatBoyState<Idle> {
        pub(super) fn frame_name(&self) -> &str {
            IDLE_FRAME_NAME
        }
    }

    pub(super) struct Running;
    impl RedHatBoyState<Running> {
        pub(super) fn frame_name(&self) -> &str {
            RUNNING_FRAME_NAME
        }
    }

    pub(super) struct Sliding;
    impl RedHatBoyState<Sliding> {
        pub(super) fn frame_name(&self) -> &str {
            SLIDING_FRAME_NAME
        }
    }

    pub(super) struct Jumping;
    impl RedHatBoyState<Jumping> {
        pub(super) fn frame_name(&self) -> &str {
            JUMPING_FRAME_NAME
        }
    }

    // すべての状態に共通する情報
    #[derive(Clone)]
    pub(super) struct RedHatBoyContext {
        pub(super) frame: u8,
        pub(super) position: Point,
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

        fn jump(&mut self) {
            self.velocity.y = JUMP_SPEED;
        }

        fn land(&mut self) {
            self.velocity.y = 0.;
            self.position.y = FLOOR;
        }

        fn fall(&mut self) {
            self.velocity.y += GRAVITY;
        }
    }

    // 初期状態の定義
    impl RedHatBoyState<Idle> {
        pub(super) fn new() -> Self {
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
        pub(super) fn update(&self) -> RedHatBoyState<Idle> {
            let mut context = self.context.clone();
            context.update_frame(IDLE_FRAME_COUNT);
            context.update_position();
            RedHatBoyState {
                context,
                _state: Idle,
            }
        }

        pub(super) fn start_run(&self) -> RedHatBoyState<Running> {
            let mut context = self.context.clone();
            context.reset_frame();
            context.run_right();
            RedHatBoyState {
                context,
                _state: Running,
            }
        }
    }

    impl RedHatBoyState<Running> {
        pub(super) fn update(&self) -> RedHatBoyState<Running> {
            let mut context = self.context.clone();
            context.update_frame(RUNNING_FRAME_COUNT);
            context.update_position();
            RedHatBoyState {
                context,
                _state: Running,
            }
        }

        pub(super) fn run_right(&self) -> RedHatBoyState<Running> {
            let mut context = self.context.clone();
            context.reset_frame();
            context.run_right();
            RedHatBoyState {
                context,
                _state: Running,
            }
        }

        pub(super) fn run_left(&self) -> RedHatBoyState<Running> {
            let mut context = self.context.clone();
            context.reset_frame();
            context.run_left();
            RedHatBoyState {
                context,
                _state: Running,
            }
        }

        pub(super) fn slide(&self) -> RedHatBoyState<Sliding> {
            let mut context = self.context.clone();
            context.reset_frame();
            RedHatBoyState {
                context,
                _state: Sliding,
            }
        }

        pub(super) fn jump(&self) -> RedHatBoyState<Jumping> {
            let mut context = self.context.clone();
            context.reset_frame();
            context.jump();
            RedHatBoyState {
                context,
                _state: Jumping,
            }
        }
    }

    pub(super) enum SlidngEndState {
        Sliding(RedHatBoyState<Sliding>),
        Complete(RedHatBoyState<Running>),
    }

    impl RedHatBoyState<Sliding> {
        pub(super) fn update(&self) -> SlidngEndState {
            let mut context = self.context.clone();
            context.update_frame(SLIDING_FRAME_COUNT);
            context.update_position();
            if context.frame == 0 {
                SlidngEndState::Complete(RedHatBoyState {
                    context,
                    _state: Running,
                })
            } else {
                SlidngEndState::Sliding(RedHatBoyState {
                    context,
                    _state: Sliding,
                })
            }
        }
    }

    pub(super) enum JumpEndState {
        Jumping(RedHatBoyState<Jumping>),
        Complete(RedHatBoyState<Running>),
    }

    impl RedHatBoyState<Jumping> {
        pub(super) fn update(&self) -> JumpEndState {
            let mut context = self.context.clone();
            context.update_frame(JUMPING_FRAME_COUNT);
            context.update_position();
            context.fall();
            if context.position.y >= FLOOR {
                context.land();
                JumpEndState::Complete(RedHatBoyState {
                    context,
                    _state: Running,
                })
            } else {
                JumpEndState::Jumping(RedHatBoyState {
                    context,
                    _state: Jumping,
                })
            }
        }
    }
}
