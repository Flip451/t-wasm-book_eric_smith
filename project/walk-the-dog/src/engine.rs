use anyhow::Result;
use async_trait::async_trait;
use std::{cell::RefCell, rc::Rc};

use crate::browser::{self, LoopClosure};

use self::{
    key_state::{prepare_input, process_input, KeyState},
    renderer::Renderer,
};

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&self) -> Result<Box<dyn Game>>;
    fn update(&mut self, keystate: &KeyState);
    fn draw(&self, renderer: &Renderer);
}

const FRAME_SIZE: f32 = 1. / 60. * 1000.;

pub struct GameLoop {
    // 直前のフレームがリクエストされた時刻
    last_frame: f64,
    // 最後に draw されてからの累積時間
    accumulated_delta: f32,
}

type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

impl GameLoop {
    pub async fn start(game: impl Game + 'static) -> Result<()> {
        // キー入力を受け取るチャンネルを作成
        let mut keyevent_receiver = prepare_input()?;
        // キー入力の状態を保持する構造体を作成
        let mut key_state = KeyState::new();

        let mut game = game.initialize().await?;

        let mut game_loop = GameLoop {
            last_frame: browser::now()?,
            accumulated_delta: 0.,
        };

        let renderer = Renderer::new()?;

        // js における以下のコードを模したもの
        //   (なお requestAnimationFrameは渡した関数をブラウザの表示を邪魔しないタイミングで処理されるようにする関数)
        //   <https://rustwasm.github.io/docs/wasm-bindgen/examples/request-animation-frame.html> を参照せよ
        //
        // function animate(now) {
        //     update();
        //     draw(now);
        //     requestAnimationFrame(animate);
        // }
        //
        // requestAnimationFrame(animate);
        //
        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(browser::create_wasm_closure(move |perf: f64| {
            // perf は、このコールバック関数が呼び出された時点の performance.now() の値（＝その時点の時刻）
            // この値を用いて、前回のフレームからの経過時間を計算し、それを累積時間に加算する
            game_loop.accumulated_delta += (perf - game_loop.last_frame) as f32;

            // キー入力を処理する
            process_input(&mut key_state, &mut keyevent_receiver);

            // 累積時間分だけ update を繰り返す
            while game_loop.accumulated_delta > FRAME_SIZE {
                game.update(&key_state);
                game_loop.accumulated_delta -= FRAME_SIZE;
            }

            game_loop.last_frame = perf;

            game.draw(&renderer);

            browser::request_animation_frame(f.borrow().as_ref().unwrap()).unwrap();
        }));

        browser::request_animation_frame(g.borrow().as_ref().unwrap())?;

        Ok(())
    }
}

pub mod renderer {
    use anyhow::{anyhow, Result};
    use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

    use crate::browser;

    // HtmlRenderingContext2d のラッパー
    pub struct Renderer {
        context: CanvasRenderingContext2d,
    }

    impl Renderer {
        pub fn new() -> Result<Self> {
            Ok(Self {
                context: browser::context()?,
            })
        }

        pub fn clear(&self, rect: &Rect) {
            self.context.clear_rect(
                rect.x().into(),
                rect.y().into(),
                rect.w.into(),
                rect.h.into(),
            );
        }

        pub fn draw_rect(&self, rect: &Rect) {
            self.context.begin_path();
            self.context.set_stroke_style(&"red".into());
            self.context.rect(
                rect.x().into(),
                rect.y().into(),
                rect.w.into(),
                rect.h.into(),
            );
            self.context.stroke();
        }

        pub fn draw_image(
            &self,
            image: &HtmlImageElement,
            frame: &Rect,
            destination: &Rect,
        ) -> Result<()> {
            self.context
                .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &image,
                    frame.x() as f64,
                    frame.y() as f64,
                    frame.w as f64,
                    frame.h as f64,
                    destination.x() as f64,
                    destination.y() as f64,
                    destination.w as f64,
                    destination.h as f64,
                )
                .map_err(|js_value| anyhow!("Error drawing image {:#?}", js_value))?;

            Ok(())
        }

        pub fn draw_entire_image(&self, image: &HtmlImageElement, position: &Point) -> Result<()> {
            self.context
                .draw_image_with_html_image_element(&image, position.x as f64, position.y as f64)
                .map_err(|js_value| anyhow!("Error drawing image {:#?}", js_value))?;

            Ok(())
        }
    }

    #[derive(Clone)]
    pub struct Rect {
        position: Point,
        pub w: i16,
        pub h: i16,
    }

    impl Rect {
        pub const fn new(position: Point, width: i16, height: i16) -> Self {
            Self {
                position,
                w: width,
                h: height,
            }
        }

        pub const fn new_from_x_y(x: i16, y: i16, width: i16, height: i16) -> Self {
            Self::new(Point { x, y }, width, height)
        }

        pub fn intersects(&self, other: &Rect) -> bool {
            self.x() < other.right()
                && self.right() > other.x()
                && self.y() < other.bottom()
                && self.bottom() > other.y()
        }

        pub fn x(&self) -> i16 {
            self.position.x
        }

        pub fn y(&self) -> i16 {
            self.position.y
        }

        pub fn set_x(&mut self, x: i16) {
            self.position.x = x;
        }

        pub fn set_y(&mut self, y: i16) {
            self.position.y = y;
        }

        pub fn right(&self) -> i16 {
            self.x() + self.w
        }

        pub fn bottom(&self) -> i16 {
            self.y() + self.h
        }

        pub fn move_by(&mut self, diff: Point) {
            self.position.move_by(diff)
        }
    }

    #[derive(Clone, Copy)]
    pub struct Point {
        pub x: i16,
        pub y: i16,
    }

    impl Point {
        pub fn move_by(&mut self, rhs: Self) {
            *self = Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    pub mod image {
        use anyhow::{anyhow, Result};
        use futures::channel::oneshot;
        use std::{rc::Rc, sync::Mutex};
        use wasm_bindgen::{JsCast, JsValue};
        use web_sys::HtmlImageElement;

        use super::{Point, Renderer};
        use crate::browser;

        pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
            // ImageHtmlElement の作成
            let image = browser::new_image()?;

            // 送受信機の作成
            let (complete_tx, complete_rx) = oneshot::channel::<Result<()>>();
            let success_tx = Rc::new(Mutex::new(Some(complete_tx)));
            let error_tx = success_tx.clone();

            // 画像の読み込みが完了したことを通知するコールバック関数の作成
            let success_callback = browser::closure_once(move || {
                if let Some(success_tx) = success_tx.lock().ok().and_then(|mut tx| tx.take()) {
                    success_tx.send(Ok(()));
                }
            });
            // 画像の読み込みが完了したら上記のコールバック関数を呼び出すように設定
            image.set_onload(Some(success_callback.as_ref().unchecked_ref()));

            // 画像の読み込みが失敗したことを通知するコールバック関数の作成
            let error_callback = browser::closure_once::<_, JsValue, ()>(move |err| {
                if let Some(error_tx) = error_tx.lock().ok().and_then(|mut tx| tx.take()) {
                    error_tx.send(Err(anyhow!("Error loading image: {:#?}", err)));
                }
            });
            // 画像の読み込みが失敗したら上記のコールバック関数を呼び出すように設定
            image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));

            // 画像の読み込み開始
            image.set_src(source);

            // 画像の読み込み完了を待機
            complete_rx.await??;

            Ok(image)
        }

        // Renderer 上の画像を表す構造体
        pub struct Image {
            element: HtmlImageElement,
            position: Point,
        }

        impl Image {
            pub fn new(element: HtmlImageElement, position: Point) -> Self {
                Self { element, position }
            }

            // Renderer 上に画像を実体化する
            pub fn draw(&self, renderer: &Renderer) -> Result<()> {
                renderer.draw_entire_image(&self.element, &self.position)
            }

            pub fn position(&self) -> &Point {
                &self.position
            }

            pub fn width(&self) -> i16 {
                self.element.width() as i16
            }

            pub fn height(&self) -> i16 {
                self.element.height() as i16
            }

            pub fn move_horizontally(&mut self, velocity: i16) {
                self.position.x += velocity;
            }

            pub fn set_x(&mut self, x: i16) {
                self.position.x = x;
            }

            pub(crate) fn right(&self) -> i16 {
                self.position.x + self.width()
            }
        }
    }

    pub mod sprite {
        use anyhow::Result;
        use serde::Deserialize;
        use std::collections::HashMap;
        use web_sys::HtmlImageElement;

        use crate::engine::renderer::Rect;

        use super::Renderer;

        #[derive(Deserialize, Clone)]
        struct SheetRect {
            x: i16,
            y: i16,
            w: i16,
            h: i16,
        }

        #[derive(Deserialize, Clone)]
        #[serde(rename_all = "camelCase")]
        pub struct Cell {
            frame: SheetRect,
            sprite_source_size: SheetRect,
        }

        impl Cell {
            pub fn to_rect_on_canvas(&self, x: i16, y: i16, w: i16, h: i16) -> Rect {
                Rect::new_from_x_y(
                    x + self.sprite_source_size.x,
                    y + self.sprite_source_size.y,
                    w,
                    h,
                )
            }

            pub fn x(&self) -> i16 {
                self.frame.x
            }

            pub fn y(&self) -> i16 {
                self.frame.y
            }

            pub fn width(&self) -> i16 {
                self.frame.w
            }

            pub fn height(&self) -> i16 {
                self.frame.h
            }
        }

        #[derive(Deserialize)]
        pub struct SpriteSheet {
            pub frames: HashMap<String, Cell>,
        }

        pub struct Sprite {
            sprite_sheet: SpriteSheet,
            image: HtmlImageElement,
        }

        impl Sprite {
            pub fn new(sprite_sheet: SpriteSheet, image: HtmlImageElement) -> Self {
                Self {
                    sprite_sheet,
                    image,
                }
            }

            pub fn cell(&self, name: &str) -> Option<&Cell> {
                self.sprite_sheet.frames.get(name)
            }

            pub fn draw(
                &self,
                renderer: &Renderer,
                source: &Rect,
                destination: &Rect,
            ) -> Result<()> {
                renderer.draw_image(&self.image, source, destination)
            }
        }
    }
}

pub mod key_state {
    use anyhow::Result;
    use futures::channel::mpsc::{unbounded, UnboundedReceiver};
    use std::{cell::RefCell, collections::HashMap, rc::Rc};
    use wasm_bindgen::JsCast;

    use crate::browser;

    pub(super) enum KeyPress {
        KeyUp(web_sys::KeyboardEvent),
        KeyDown(web_sys::KeyboardEvent),
    }

    pub(super) fn prepare_input() -> Result<UnboundedReceiver<KeyPress>> {
        let (keyevent_tx, keyevent_rx) = unbounded();
        let keydown_tx = Rc::new(RefCell::new(keyevent_tx));
        let keyup_tx = keydown_tx.clone();

        let canvas = browser::canvas().expect("Canvas not found");

        let onkeydown = browser::create_wasm_closure(move |keycode: web_sys::KeyboardEvent| {
            keydown_tx
                .borrow_mut()
                .start_send(KeyPress::KeyDown(keycode));
        });
        canvas.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));

        let onkeyup = browser::create_wasm_closure(move |keycode: web_sys::KeyboardEvent| {
            keyup_tx.borrow_mut().start_send(KeyPress::KeyUp(keycode));
        });
        canvas.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));

        onkeydown.forget();
        onkeyup.forget();

        Ok(keyevent_rx)
    }

    pub(super) fn process_input(
        state: &mut KeyState,
        keyevent_receiver: &mut UnboundedReceiver<KeyPress>,
    ) {
        loop {
            match keyevent_receiver.try_next() {
                Ok(None) => break,
                Err(_) => break,
                Ok(Some(event)) => match event {
                    KeyPress::KeyDown(keyboard_event) => {
                        state.set_pressed(&keyboard_event.code(), keyboard_event)
                    }
                    KeyPress::KeyUp(keyboard_event) => state.set_released(&keyboard_event.code()),
                },
            }
        }
    }

    pub struct KeyState {
        pressed_keys: HashMap<String, web_sys::KeyboardEvent>,
    }

    impl KeyState {
        pub(super) fn new() -> Self {
            Self {
                pressed_keys: HashMap::new(),
            }
        }

        pub fn is_pressed(&self, keycode: &str) -> bool {
            self.pressed_keys.contains_key(keycode)
        }

        fn set_pressed(&mut self, keycode: &str, event: web_sys::KeyboardEvent) {
            self.pressed_keys.insert(keycode.to_string(), event);
        }

        fn set_released(&mut self, keycode: &str) {
            self.pressed_keys.remove(keycode);
        }
    }
}
