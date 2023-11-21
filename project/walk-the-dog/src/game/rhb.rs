use web_sys::HtmlImageElement;

use super::{sprite::SpriteSheet, Point};

struct RedHatBoy {
    state_machine: RedHatBoyStateMachine,
    sprite_sheet: SpriteSheet,
    image: HtmlImageElement,
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

// 状態を表す型
struct Idle;
struct Running;

// ステートマシーン本体
enum RedHatBoyStateMachine {
    Idle(RedHatBoyState<Idle>),
    Running(RedHatBoyState<Running>),
}

// 状態と列挙子を関連付ける
impl From<RedHatBoyState<Running>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Running>) -> Self {
        RedHatBoyStateMachine::Running(state)
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
