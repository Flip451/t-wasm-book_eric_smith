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

impl RedHatBoyState<Idle> {
    fn run(self) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context,
            _state: Running,
        }
    }
}