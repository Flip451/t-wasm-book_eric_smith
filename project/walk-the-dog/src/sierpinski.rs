use std::ops::Add;
use std::ops::Mul;

use rand::Rng;
use web_sys::console;

#[derive(Clone, Copy)]
pub(super) struct Point {
    pub(super) x: f64,
    pub(super) y: f64,
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

pub(super) struct Color {
    pub(super) r: u8,
    pub(super) g: u8,
    pub(super) b: u8,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("rgb({},{},{})", self.r, self.g, self.b)
    }
}

pub(super) struct Triangle {
    pub(super) p1: Point,
    pub(super) p2: Point,
    pub(super) p3: Point,
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

pub(super) fn draw_sierpinski(
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
