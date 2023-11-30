use anyhow::Result;
use gloo_utils::format::JsValueSerdeExt;

use crate::browser;
use crate::engine::renderer::sprite::{Cell, Sprite, SpriteSheet};
use crate::engine::renderer::{image, Point, Rect, Renderer};

use self::red_hat_boy_states::*;
use super::bounding_box::BoundingBox;
use super::objects::GameObject;

// 座標系関連
pub const STARTING_POINT: i16 = -20;
pub const FLOOR: i16 = 479;

pub struct RedHatBoy {
    state_machine: RedHatBoyStateMachine,
    sprite: Sprite,
}

impl RedHatBoy {
    pub fn new(sprite: Sprite, position: Point) -> Self {
        Self {
            state_machine: RedHatBoyStateMachine::Idle(RedHatBoyState::<Idle>::new(position)),
            sprite,
        }
    }

    pub async fn load_sprite() -> Result<Sprite> {
        let json = browser::fetch_json("rhb.json").await?;

        // json を Sheet 型に変換
        // この際、JsValue 型の json を serde を用いてデシリアライズ
        // この部分の実装は
        // <https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html#an-alternative-approach---using-json>
        // を参考にした
        let sprite_sheet: SpriteSheet = json.into_serde()?;

        let image = image::load_image("rhb.png").await?;

        let sprite = Sprite::new(sprite_sheet, image);

        Ok(sprite)
    }
}

impl GameObject for RedHatBoy {
    fn bounding_box(&self) -> BoundingBox {
        const X_OFFSET: i16 = 18;
        const Y_OFFSET: i16 = 14;
        const WIDTH_OFFSET: i16 = -28;
        const HEIGHT_OFFSET: i16 = 0;

        let sprite = self.current_sprite();
        let mut raw_rect = sprite.to_rect_on_canvas(
            self.state_machine.context().position.x,
            self.state_machine.context().position.y,
            sprite.width(),
            sprite.height(),
        );
        raw_rect.set_x(raw_rect.x() + X_OFFSET);
        raw_rect.set_y(raw_rect.y() + Y_OFFSET);
        raw_rect.w += WIDTH_OFFSET;
        raw_rect.h += HEIGHT_OFFSET;

        let mut bounding_boxes = BoundingBox::new();
        bounding_boxes.add(raw_rect);
        bounding_boxes
    }

    fn draw(&self, renderer: &Renderer) -> Result<()> {
        // シートの中から指定の画像（Run (*).png）の位置を取得
        let sprite = self.current_sprite();

        // キャンバスに指定の画像を描画
        self.sprite.draw(
            &renderer,
            &&Rect::new_from_x_y(sprite.x(), sprite.y(), sprite.width(), sprite.height()),
            &sprite.to_rect_on_canvas(
                self.state_machine.context().position.x,
                self.state_machine.context().position.y,
                sprite.width(),
                sprite.height(),
            ),
        )?;

        // キャンバスに bounding box を描画
        #[cfg(feature = "collision_debug")]
        self.bounding_box().draw(renderer)?;

        Ok(())
    }
}

impl RedHatBoy {
    fn frame_name(&self) -> String {
        format!(
            "{} ({}).png",
            self.state_machine.frame_name(),
            self.state_machine.context().frame / 3 + 1
        )
    }

    fn current_sprite(&self) -> &Cell {
        let frame_name = self.frame_name();

        // シートの中から指定の画像（Run (*).png）の位置を取得
        self.sprite.cell(&frame_name).expect("Cell not found")
    }

    pub fn is_falling(&self) -> bool {
        self.state_machine.context().velocity.y > 0
    }

    pub fn walking_speed(&self) -> i16 {
        self.state_machine.context().velocity.x
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

    pub fn knock_out(&mut self) {
        self.state_machine.transition(Event::KnockOut);
    }

    pub fn land_on(&mut self, y: i16) {
        self.state_machine.transition(Event::Land(y));
    }
}

// ステートマシーン本体
enum RedHatBoyStateMachine {
    Idle(RedHatBoyState<Idle>),
    Running(RedHatBoyState<Running>),
    Sliding(RedHatBoyState<Sliding>),
    Jumping(RedHatBoyState<Jumping>),
    Falling(RedHatBoyState<Falling>),
    KnockedOut(RedHatBoyState<KnockedOut>),
}

impl RedHatBoyStateMachine {
    fn frame_name(&self) -> &str {
        match self {
            RedHatBoyStateMachine::Idle(state) => state.frame_name(),
            RedHatBoyStateMachine::Running(state) => state.frame_name(),
            RedHatBoyStateMachine::Sliding(state) => state.frame_name(),
            RedHatBoyStateMachine::Jumping(state) => state.frame_name(),
            RedHatBoyStateMachine::Falling(state) => state.frame_name(),
            RedHatBoyStateMachine::KnockedOut(state) => state.frame_name(),
        }
    }

    fn context(&self) -> &RedHatBoyContext {
        match self {
            RedHatBoyStateMachine::Idle(state) => &state.context(),
            RedHatBoyStateMachine::Running(state) => &state.context(),
            RedHatBoyStateMachine::Sliding(state) => &state.context(),
            RedHatBoyStateMachine::Jumping(state) => &state.context(),
            RedHatBoyStateMachine::Falling(state) => &state.context(),
            RedHatBoyStateMachine::KnockedOut(state) => &state.context(),
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
            // 衝突による状態遷移
            (RedHatBoyStateMachine::Running(ref state), Event::KnockOut) => {
                *self = state.knock_out().into()
            }
            (RedHatBoyStateMachine::Sliding(ref state), Event::KnockOut) => {
                *self = state.knock_out().into()
            }
            (RedHatBoyStateMachine::Jumping(ref state), Event::KnockOut) => {
                *self = state.knock_out().into()
            }
            // 踏み台のへの衝突による状態遷移
            (RedHatBoyStateMachine::Jumping(ref state), Event::Land(y)) => {
                *self = state.land_on(y).into()
            }
            (RedHatBoyStateMachine::Running(ref state), Event::Land(y)) => {
                *self = state.land_on(y).into()
            }
            (RedHatBoyStateMachine::Sliding(ref state), Event::Land(y)) => {
                *self = state.land_on(y).into()
            }
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
            (RedHatBoyStateMachine::Falling(ref state), Event::Update) => {
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

impl From<RedHatBoyState<Falling>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Falling>) -> Self {
        RedHatBoyStateMachine::Falling(state)
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

impl From<FallingEndState> for RedHatBoyStateMachine {
    fn from(state: FallingEndState) -> Self {
        match state {
            FallingEndState::Falling(state) => RedHatBoyStateMachine::Falling(state),
            FallingEndState::Complete(state) => RedHatBoyStateMachine::KnockedOut(state),
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
    KnockOut,
    Land(i16),
}

mod red_hat_boy_states {
    use crate::engine::renderer::Point;

    // 座標系関連
    use super::super::HEIGHT;
    use super::FLOOR;
    const PLAYER_HEIGHT: i16 = HEIGHT - FLOOR;
    const RUNNING_SPEED: i16 = 4;
    const JUMP_SPEED: i16 = -25;
    const GRAVITY: i16 = 1;
    const TERMINAL_VELOCITY: i16 = 20;

    // フレーム名
    const IDLE_FRAME_NAME: &str = "Idle";
    const RUNNING_FRAME_NAME: &str = "Run";
    const SLIDING_FRAME_NAME: &str = "Slide";
    const JUMPING_FRAME_NAME: &str = "Jump";
    const FALLING_FRAME_NAME: &str = "Dead";

    // フレーム数
    const IDLE_FRAME_COUNT: u8 = 29;
    const RUNNING_FRAME_COUNT: u8 = 24;
    const SLIDING_FRAME_COUNT: u8 = 14;
    const JUMPING_FRAME_COUNT: u8 = 35;
    const FALLING_FRAME_COUNT: u8 = 29;

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

    pub(super) struct Falling;
    impl RedHatBoyState<Falling> {
        pub(super) fn frame_name(&self) -> &str {
            FALLING_FRAME_NAME
        }
    }

    pub(super) struct KnockedOut;
    impl RedHatBoyState<KnockedOut> {
        pub(super) fn frame_name(&self) -> &str {
            FALLING_FRAME_NAME
        }
    }

    // すべての状態に共通する情報
    #[derive(Clone)]
    pub(super) struct RedHatBoyContext {
        pub(super) frame: u8,
        pub(super) position: Point,
        pub(super) velocity: Point,
    }

    impl RedHatBoyContext {
        fn update_frame(&mut self, frame_count: u8) {
            self.frame = (self.frame + 1) % frame_count;
        }

        fn reset_frame(&mut self) {
            self.frame = 0;
        }

        fn update_position(&mut self) {
            // self.position.x += self.velocity.x;
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

        fn land_on(&mut self, y: i16) {
            self.position.y = y;
        }

        fn fall(&mut self) {
            if self.velocity.y < TERMINAL_VELOCITY {
                self.velocity.y += GRAVITY;
            }
        }

        fn stop(&mut self) {
            self.velocity.x = 0;
            self.velocity.y = 0;
        }
    }

    // 初期状態の定義
    impl RedHatBoyState<Idle> {
        pub(super) fn new(position: Point) -> Self {
            Self {
                context: RedHatBoyContext {
                    frame: 0,
                    position,
                    velocity: Point { x: 0, y: 0 },
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
            if context.position.y >= FLOOR {
                context.land_on(FLOOR);
            }
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

        pub(super) fn land_on(&self, y: i16) -> RedHatBoyState<Running> {
            let mut context = self.context.clone();
            context.land_on(y - PLAYER_HEIGHT);
            RedHatBoyState {
                context,
                _state: Running,
            }
        }

        pub(super) fn knock_out(&self) -> RedHatBoyState<Falling> {
            let mut context = self.context.clone();
            context.reset_frame();
            context.stop();
            RedHatBoyState {
                context,
                _state: Falling,
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
            if context.position.y >= FLOOR {
                context.land_on(FLOOR);
            }
            if context.frame == 0 {
                context.reset_frame();
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

        pub(super) fn land_on(&self, y: i16) -> RedHatBoyState<Sliding> {
            let mut context = self.context.clone();
            context.land_on(y - PLAYER_HEIGHT);
            RedHatBoyState {
                context,
                _state: Sliding,
            }
        }

        pub(super) fn knock_out(&self) -> RedHatBoyState<Falling> {
            let mut context = self.context.clone();
            context.reset_frame();
            context.stop();
            RedHatBoyState {
                context,
                _state: Falling,
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
            if context.frame < JUMPING_FRAME_COUNT - 1 {
                context.update_frame(JUMPING_FRAME_COUNT);
            }
            context.update_position();
            context.fall();
            if context.position.y >= FLOOR {
                context.land_on(FLOOR);
                context.reset_frame();
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

        pub(super) fn knock_out(&self) -> RedHatBoyState<Falling> {
            let mut context = self.context.clone();
            context.reset_frame();
            context.stop();
            RedHatBoyState {
                context,
                _state: Falling,
            }
        }

        pub(super) fn land_on(&self, y: i16) -> RedHatBoyState<Running> {
            let mut context = self.context.clone();
            context.reset_frame();
            context.land_on(y - PLAYER_HEIGHT);
            RedHatBoyState {
                context,
                _state: Running,
            }
        }
    }

    pub(super) enum FallingEndState {
        Falling(RedHatBoyState<Falling>),
        Complete(RedHatBoyState<KnockedOut>),
    }

    impl RedHatBoyState<Falling> {
        pub(super) fn update(&self) -> FallingEndState {
            let mut context = self.context.clone();
            context.update_frame(FALLING_FRAME_COUNT);
            context.update_position();
            if context.position.y < FLOOR {
                context.fall();
            } else {
                context.land_on(FLOOR);
            }
            if context.frame == FALLING_FRAME_COUNT - 1 {
                FallingEndState::Complete(RedHatBoyState {
                    context,
                    _state: KnockedOut,
                })
            } else {
                FallingEndState::Falling(RedHatBoyState {
                    context,
                    _state: Falling,
                })
            }
        }
    }
}
