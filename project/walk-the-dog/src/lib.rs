use std::ops::Add;
use std::ops::Mul;

use rand::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // windowオブジェクトの取得
    let window = web_sys::window().expect("no global `window` exists");
    // documentオブジェクトの取得
    let document = window.document().expect("should have a document on window");
    // canvas要素の取得 (Element型)
    let canvas = document.get_element_by_id("canvas").unwrap();
    // Element 型の canvas を HtmlCanvasElement にキャスト
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    // コンテキストの取得
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let triangle = Triangle {
        p1: Point { x: 300.0, y: 0. },
        p2: Point { x: 0., y: 600.0 },
        p3: Point { x: 600.0, y: 600.0 },
    };

    let mut rng = rand::thread_rng();
    let color = Color {
        r: rng.gen_range(0..255),
        g: rng.gen_range(0..255),
        b: rng.gen_range(0..255),
    };

    console::log_1(&color.to_string().into());

    draw_sierpinski(&context, &triangle, 6, &color);

    Ok(())
}

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("rgb({},{},{})", self.r, self.g, self.b)
    }
}

struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, triangle: &Triangle, color: &Color) {
    context.move_to(triangle.p1.x, triangle.p1.y);
    context.begin_path();
    context.line_to(triangle.p2.x, triangle.p2.y);
    context.line_to(triangle.p3.x, triangle.p3.y);
    context.line_to(triangle.p1.x, triangle.p1.y);
    context.close_path();
    context.stroke();
    context.set_fill_style(&color.to_string().into());
    context.fill();
}

fn draw_sierpinski(
    context: &web_sys::CanvasRenderingContext2d,
    triangle: &Triangle,
    depth: usize,
    color: &Color,
) {
    if depth <= 0 {
        return;
    }

    draw_triangle(context, triangle, &color);

    let center_p1_p2 = (triangle.p1 + triangle.p2) * 0.5;
    let center_p2_p3 = (triangle.p2 + triangle.p3) * 0.5;
    let center_p3_p1 = (triangle.p3 + triangle.p1) * 0.5;

    let new_triangle_1 = Triangle {
        p1: triangle.p1,
        p2: center_p1_p2,
        p3: center_p3_p1,
    };
    let new_triangle_2 = Triangle {
        p1: center_p1_p2,
        p2: triangle.p2,
        p3: center_p2_p3,
    };
    let new_triangle_3 = Triangle {
        p1: center_p3_p1,
        p2: center_p2_p3,
        p3: triangle.p3,
    };

    let mut rng = rand::thread_rng();
    let color = Color {
        r: rng.gen_range(0..255),
        g: rng.gen_range(0..255),
        b: rng.gen_range(0..255),
    };

    console::log_1(&color.to_string().into());

    draw_sierpinski(context, &new_triangle_1, depth - 1, &color);
    draw_sierpinski(context, &new_triangle_2, depth - 1, &color);
    draw_sierpinski(context, &new_triangle_3, depth - 1, &color);
}
