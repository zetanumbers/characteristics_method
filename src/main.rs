use std::borrow::Cow;

use seed::{prelude::*, *};

mod function_input;
mod numeric_input;
mod util;
mod visuals;

struct Model {
    visuals: visuals::Model,
    test: function_input::Model,
    test_value: f64,
}

impl Model {
    fn init(_: Url, _orders: &mut impl Orders<Msg>) -> Model {
        Model {
            visuals: visuals::Model::init(),
            test: function_input::Model::new(div!("test"), Cow::Borrowed("x")),
            test_value: 0.0,
        }
    }

    fn view(&self) -> Node<Msg> {
        div![
            C!["app"],
            canvas![attrs![
                At::Width => 600,
                At::Height => 600,
            ]],
            div![
                C!["controls"],
                self.visuals.view().map_msg(Msg::VisualsMsg),
                self.test.view().map_msg(Msg::TestMsg),
                self.test_value,
            ]
        ]
    }
}

enum Msg {
    VisualsMsg(visuals::Msg),
    TestMsg(function_input::Msg),
}

impl Msg {
    fn update(self, model: &mut Model, _orders: &mut impl Orders<Msg>) {
        match self {
            Msg::VisualsMsg(msg) => msg.update(&mut model.visuals),
            Msg::TestMsg(msg) => {
                msg.update(&mut model.test);
                model.test_value = model.test.eval_update(13.0).unwrap_or(std::f64::NAN);
            }
        }
    }
}

fn main() {
    App::start(body(), Model::init, Msg::update, Model::view);
}

// region: Old code

// use core::{cell::Cell, mem};
// use wasm_bindgen::prelude::*;

// const MIN_FRAMERATE: f64 = 60.0;
// const MIN_TABULATION_SIZE: usize = 257;

// fn calc_tabulation_size(a: f64, l: f64) -> usize {
//     MIN_TABULATION_SIZE.max((2.0 * l / a * MIN_FRAMERATE + 1.0).ceil() as usize)
// }

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(typescript_type = "(arg: number) => number")]
//     pub type RealFunction;

//     #[wasm_bindgen(method, js_name = "call")]
//     pub fn call_impl(f: &RealFunction, this: JsValue, arg: f64) -> f64;

//     // #[wasm_bindgen(js_namespace = console)]
//     // pub fn log(msg: &str);
// }

// impl RealFunction {
//     fn call(&self, arg: f64) -> f64 {
//         self.call_impl(JsValue::NULL, arg)
//     }
// }

// #[wasm_bindgen]
// pub struct CurveView {
//     visible: bool,
//     color: JsValue,
// }

// #[wasm_bindgen]
// impl CurveView {
//     #[wasm_bindgen(constructor)]
//     pub fn new(visible: bool, color: JsValue) -> CurveView {
//         CurveView { visible, color }
//     }
// }

// #[wasm_bindgen]
// #[derive(Debug, Clone, Copy)]
// pub enum UDiffType {
//     Ut = "Ut",
//     Ux = "Ux",
// }

// #[wasm_bindgen]
// pub struct UDiff {
//     pub ty: UDiffType,
//     func: RealFunction,
// }

// #[wasm_bindgen]
// impl UDiff {
//     #[wasm_bindgen(constructor)]
//     pub fn new(ty: UDiffType, func: RealFunction) -> Self {
//         UDiff { ty, func }
//     }
// }

// #[derive(Default, Debug, Clone, Copy)]
// struct UDiffPairPoint {
//     u_t: f64,
//     u_x: f64,
// }

// #[wasm_bindgen]
// pub struct Renderer {
//     left: UDiff,
//     right: UDiff,
//     floor_u: Vec<Cell<f64>>,
//     floor_udiff: Vec<Cell<UDiffPairPoint>>,
//     floor_udiff_buffer: Vec<Cell<UDiffPairPoint>>,
//     pub a: f64,
//     pub l: f64,
//     rem_t: f64,
//     cur_t: f64,
//     // Visuals
//     u_view: CurveView,
//     u_x_view: CurveView,
//     u_t_view: CurveView,
// }

// #[wasm_bindgen]
// impl Renderer {
//     #[wasm_bindgen(constructor)]
//     pub fn new(
//         left: UDiff,
//         right: UDiff,
//         bottom_u: RealFunction,
//         bottom_u_t: RealFunction,
//         a: f64,
//         l: f64,
//         u_view: CurveView,
//         u_x_view: CurveView,
//         u_t_view: CurveView,
//     ) -> Renderer {
//         let (floor_u, floor_udiff) = Self::generate_floor(bottom_u, bottom_u_t, a, l);
//         let mut floor_udiff_buffer = Vec::new();
//         floor_udiff_buffer.resize(floor_udiff.len(), Cell::default());
//         Self {
//             rem_t: 0.0,
//             cur_t: 0.0,
//             left,
//             right,
//             floor_u,
//             floor_udiff,
//             floor_udiff_buffer,
//             a,
//             l,
//             u_view,
//             u_x_view,
//             u_t_view,
//         }
//     }

//     pub fn reset(&mut self, bottom_u_x: RealFunction, bottom_u_t: RealFunction, a: f64, l: f64) {
//         self.a = a;
//         self.l = l;
//         let (floor_u, floor_udiff) = Self::generate_floor(bottom_u_x, bottom_u_t, a, l);
//         self.floor_u = floor_u;
//         self.floor_udiff = floor_udiff;
//         self.floor_udiff_buffer
//             .resize(self.floor_udiff.len(), Cell::default());
//     }

//     pub fn advance(&mut self, dt: f64) {
//         let step_dt = self.step_dt();
//         self.rem_t = dt % step_dt;
//         if let None = self.nth((dt / step_dt) as _) {
//             unreachable!()
//         }
//     }

//     fn step_dt(&self) -> f64 {
//         2.0 * self.l / (self.a * (self.floor_udiff.len() - 1) as f64)
//     }

//     fn calc_point(&self, left: UDiffPairPoint, right: UDiffPairPoint) -> UDiffPairPoint {
//         let a = self.a;
//         UDiffPairPoint {
//             u_x: (left.u_x - left.u_t / a + right.u_x + right.u_t / a) / 2.0,
//             u_t: (left.u_t - left.u_x * a + right.u_t + right.u_x * a) / 2.0,
//         }
//     }

//     fn left_calc_point(&self, t: f64, point: UDiffPairPoint) -> UDiffPairPoint {
//         let u_diff = self.left.func.call(t);
//         match self.left.ty {
//             UDiffType::Ut => {
//                 let u_t = u_diff;
//                 UDiffPairPoint {
//                     u_x: point.u_x - (u_t - point.u_t) / self.a,
//                     u_t,
//                 }
//             }
//             UDiffType::Ux => {
//                 let u_x = u_diff;
//                 UDiffPairPoint {
//                     u_t: point.u_t - (u_x - point.u_x) * self.a,
//                     u_x,
//                 }
//             }
//             _ => unreachable!(),
//         }
//     }

//     fn right_calc_point(&self, t: f64, point: UDiffPairPoint) -> UDiffPairPoint {
//         let u_diff = self.right.func.call(t);
//         match self.right.ty {
//             UDiffType::Ut => {
//                 let u_t = u_diff;
//                 UDiffPairPoint {
//                     u_x: point.u_x + (u_t - point.u_t) / self.a,
//                     u_t,
//                 }
//             }
//             UDiffType::Ux => {
//                 let u_x = u_diff;
//                 UDiffPairPoint {
//                     u_t: point.u_t + (u_x - point.u_x) * self.a,
//                     u_x,
//                 }
//             }
//             _ => unreachable!(),
//         }
//     }

//     pub fn render_canvas(&self, ctx: &web_sys::CanvasRenderingContext2d) -> Result<(), JsValue> {
//         const CANVAS_WIDTH: u32 = 480;
//         const CANVAS_HEIGHT: u32 = 480;

//         ctx.clear_rect(0.0, 0.0, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);

//         let n = self.floor_udiff.len();

//         let x_from_idx = |i| CANVAS_WIDTH as f64 * i as f64 / (n - 1) as f64;
//         let t_y = |y| CANVAS_HEIGHT as f64 * (0.5 - y / self.l);

//         let u_path = web_sys::Path2d::new()?;
//         let u_x_path = web_sys::Path2d::new()?;
//         let u_t_path = web_sys::Path2d::new()?;

//         let init_point = self.floor_u.first().unwrap();
//         u_path.move_to(0.0, t_y(init_point.get()));

//         let init_point = self.floor_udiff.first().unwrap();
//         u_x_path.move_to(0.0, t_y(init_point.get().u_x));
//         u_t_path.move_to(0.0, t_y(init_point.get().u_t));

//         self.floor_u
//             .iter()
//             .zip(self.floor_udiff.iter())
//             .enumerate()
//             .skip(1)
//             .for_each(|(i, (u, p))| {
//                 u_path.line_to(x_from_idx(i), t_y(u.get()));
//                 u_x_path.line_to(x_from_idx(i), t_y(p.get().u_x));
//                 u_t_path.line_to(x_from_idx(i), t_y(p.get().u_t));
//             });

//         if self.u_view.visible {
//             ctx.set_stroke_style(&self.u_view.color);
//             ctx.stroke_with_path(&u_path);
//         }
//         if self.u_x_view.visible {
//             ctx.set_stroke_style(&self.u_x_view.color);
//             ctx.stroke_with_path(&u_x_path);
//         }
//         if self.u_t_view.visible {
//             ctx.set_stroke_style(&self.u_t_view.color);
//             ctx.stroke_with_path(&u_t_path);
//         }

//         Ok(())
//     }

//     #[wasm_bindgen(setter)]
//     pub fn set_left_ty(&mut self, ty: UDiffType) {
//         self.left.ty = ty;
//     }

//     #[wasm_bindgen(setter)]
//     pub fn set_right_ty(&mut self, ty: UDiffType) {
//         self.right.ty = ty;
//     }

//     #[wasm_bindgen(setter)]
//     pub fn set_left_func(&mut self, func: RealFunction) {
//         self.left.func = func;
//     }

//     #[wasm_bindgen(setter)]
//     pub fn set_right_func(&mut self, func: RealFunction) {
//         self.right.func = func;
//     }

//     #[wasm_bindgen(setter)]
//     pub fn set_u_visible(&mut self, vis: bool) {
//         self.u_view.visible = vis;
//     }
//     #[wasm_bindgen(setter)]
//     pub fn set_u_color(&mut self, color: JsValue) {
//         self.u_view.color = color;
//     }
//     #[wasm_bindgen(setter)]
//     pub fn set_u_x_visible(&mut self, vis: bool) {
//         self.u_x_view.visible = vis;
//     }
//     #[wasm_bindgen(setter)]
//     pub fn set_u_x_color(&mut self, color: JsValue) {
//         self.u_x_view.color = color;
//     }
//     #[wasm_bindgen(setter)]
//     pub fn set_u_t_visible(&mut self, vis: bool) {
//         self.u_t_view.visible = vis;
//     }
//     #[wasm_bindgen(setter)]
//     pub fn set_u_t_color(&mut self, color: JsValue) {
//         self.u_t_view.color = color;
//     }

//     fn generate_floor(
//         bottom_u: RealFunction,
//         bottom_u_t: RealFunction,
//         a: f64,
//         l: f64,
//     ) -> (Vec<Cell<f64>>, Vec<Cell<UDiffPairPoint>>) {
//         let n = calc_tabulation_size(a, l);
//         let h = l / (n - 1) as f64;
//         let bottom_u_x = |x| (bottom_u.call(x + h) - bottom_u.call(x)) / h;

//         (0..n)
//             .map(|i| {
//                 let x = i as f64 / (n - 1) as f64 * l;
//                 (
//                     Cell::new(bottom_u.call(x)),
//                     Cell::new(UDiffPairPoint {
//                         u_t: bottom_u_t.call(x),
//                         u_x: bottom_u_x(x),
//                     }),
//                 )
//             })
//             .unzip()
//     }
// }

// impl Iterator for Renderer {
//     type Item = ();

//     fn next(&mut self) -> Option<()> {
//         let dt = self.step_dt();
//         self.cur_t += dt;

//         let Renderer {
//             cur_t,
//             floor_u,
//             floor_udiff,
//             floor_udiff_buffer,
//             ..
//         } = &*self;

//         let n = floor_udiff.len();
//         debug_assert_eq!(n, floor_udiff_buffer.len());

//         // Euler method for tabulation of U
//         floor_u
//             .iter()
//             .zip(floor_udiff.iter())
//             .for_each(|(u, u_diff)| {
//                 u.replace(u.get() + dt * u_diff.get().u_t / 2.0);
//             });

//         // Characteristics method
//         floor_udiff_buffer[0].replace(self.left_calc_point(*cur_t, floor_udiff[1].get()));
//         floor_udiff_buffer[1..n - 1]
//             .iter()
//             .zip(floor_udiff.iter().zip(floor_udiff.iter().skip(2)))
//             .for_each(|(out_point, (a, b))| {
//                 out_point.replace(self.calc_point(a.get(), b.get()));
//             });
//         floor_udiff_buffer[n - 1].replace(self.right_calc_point(*cur_t, floor_udiff[n - 2].get()));

//         mem::swap(&mut self.floor_udiff, &mut self.floor_udiff_buffer);
//         Some(())
//     }
// }

// endregion: Old code
