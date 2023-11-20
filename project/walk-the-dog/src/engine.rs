use std::{cell::RefCell, rc::Rc, sync::Mutex};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::channel::oneshot;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::browser::{self, LoopClosure};

pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
    // ImageHtmlElement の作成
    let image = browser::new_image()?;

    // 送受信機の作成
    let (complete_tx, complete_rx) = oneshot::channel::<Result<()>>();
    let success_tx = Rc::new(Mutex::new(Some(complete_tx)));
    let error_tx = Rc::clone(&success_tx);

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

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&self) -> Result<Box<dyn Game>>;
    fn update(&mut self);
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

        *g.borrow_mut() = Some(browser::create_raf_closure(move |perf: f64| {
            // perf は、このコールバック関数が呼び出された時点の performance.now() の値（＝その時点の時刻）
            // この値を用いて、前回のフレームからの経過時間を計算し、それを累積時間に加算する
            game_loop.accumulated_delta += (perf - game_loop.last_frame) as f32;

            // 累積時間分だけ update を繰り返す
            while game_loop.accumulated_delta > FRAME_SIZE {
                game.update();
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
        self.context
            .clear_rect(rect.x.into(), rect.y.into(), rect.w.into(), rect.h.into());
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
                frame.x as f64,
                frame.y as f64,
                frame.w as f64,
                frame.h as f64,
                destination.x as f64,
                destination.y as f64,
                destination.w as f64,
                destination.h as f64,
            )
            .map_err(|js_value| anyhow!("Error drawing image {:#?}", js_value))?;

        Ok(())
    }
}

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
