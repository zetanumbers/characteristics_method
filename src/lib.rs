#![allow(incomplete_features)]
#![feature(generic_associated_types)]

mod characteristics_method;
mod common_method;
mod explicit_method;

use core::{cell::Cell, mem};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const MIN_FRAMERATE: f64 = 60.0;
const MIN_TABULATION_SIZE: usize = 257;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "(arg: number) => number")]
    pub type RealFunction;

    #[wasm_bindgen(method, js_name = "call")]
    pub fn call_impl(f: &RealFunction, this: JsValue, arg: f64) -> f64;

    // #[wasm_bindgen(js_namespace = console)]
    // pub fn log(msg: &str);
}

impl RealFunction {
    fn call(&self, arg: f64) -> f64 {
        self.call_impl(JsValue::NULL, arg)
    }
}

#[wasm_bindgen]
pub struct CurveView {
    visible: bool,
    color: JsValue,
}

#[wasm_bindgen]
impl CurveView {
    #[wasm_bindgen(constructor)]
    pub fn new(visible: bool, color: JsValue) -> CurveView {
        CurveView { visible, color }
    }
}

#[wasm_bindgen]
pub struct CurveViews {
    u: CurveView,
    ut: CurveView,
    ux: CurveView,
}

#[wasm_bindgen]
impl CurveViews {
    #[wasm_bindgen(constructor)]
    pub fn new(u: CurveView, ut: CurveView, ux: CurveView) -> Self {
        Self { u, ut, ux }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum UDiffType {
    Ut = "Ut",
    Ux = "Ux",
}

#[wasm_bindgen]
pub struct UDiff {
    pub ty: UDiffType,
    func: RealFunction,
}

#[wasm_bindgen]
impl UDiff {
    #[wasm_bindgen(constructor)]
    pub fn new(ty: UDiffType, func: RealFunction) -> Self {
        UDiff { ty, func }
    }
}

#[wasm_bindgen]
pub struct ConstParams {
    bottom_u: RealFunction,
    bottom_u_t: RealFunction,
    a: f64,
    l: f64,
}

#[wasm_bindgen]
impl ConstParams {
    #[wasm_bindgen(constructor)]
    pub fn new(bottom_u: RealFunction, bottom_u_t: RealFunction, a: f64, l: f64) -> Self {
        Self {
            bottom_u,
            bottom_u_t,
            a,
            l,
        }
    }
}

#[wasm_bindgen]
pub struct VariableParams {
    left: UDiff,
    right: UDiff,
}

#[wasm_bindgen]
impl VariableParams {
    #[wasm_bindgen(constructor)]
    pub fn new(left: UDiff, right: UDiff) -> Self {
        Self { left, right }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct UDiffPairPoint {
    ut: f64,
    ux: f64,
}

#[wasm_bindgen]
pub struct Renderer {
    var_params: VariableParams,
    floor_u: Vec<Cell<f64>>,
    floor_udiff: Vec<Cell<UDiffPairPoint>>,
    floor_udiff_buffer: Vec<Cell<UDiffPairPoint>>,
    const_params: ConstParams,
    rem_t: f64,
    cur_t: f64,
    views: CurveViews,
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn new(
        var_params: VariableParams,
        const_params: ConstParams,
        views: CurveViews,
    ) -> Renderer {
        let (floor_u, floor_udiff) = Self::generate_floor(&const_params);
        let mut floor_udiff_buffer = Vec::new();
        floor_udiff_buffer.resize(floor_udiff.len(), Cell::default());
        Self {
            rem_t: 0.0,
            cur_t: 0.0,
            var_params,
            floor_u,
            floor_udiff,
            floor_udiff_buffer,
            const_params,
            views,
        }
    }

    pub fn reset(&mut self, const_params: ConstParams) {
        let (floor_u, floor_udiff) = Self::generate_floor(&const_params);
        self.const_params = const_params;
        self.floor_u = floor_u;
        self.floor_udiff = floor_udiff;
        self.floor_udiff_buffer
            .resize(self.floor_udiff.len(), Cell::default());
    }

    pub fn advance(&mut self, mut dt: f64) {
        dt += self.rem_t;
        let step_dt = self.step_dt();
        self.rem_t = dt % step_dt;
        for _ in 0..(dt / step_dt) as _ {
            self.step_forward()
        }
    }

    fn step_dt(&self) -> f64 {
        2.0 * self.const_params.l / (self.const_params.a * (self.floor_udiff.len() - 1) as f64)
    }

    fn calc_point(&self, left: UDiffPairPoint, right: UDiffPairPoint) -> UDiffPairPoint {
        let a = self.const_params.a;
        UDiffPairPoint {
            ux: (left.ux - left.ut / a + right.ux + right.ut / a) / 2.0,
            ut: (left.ut - left.ux * a + right.ut + right.ux * a) / 2.0,
        }
    }

    fn left_calc_point(&self, t: f64, point: UDiffPairPoint) -> UDiffPairPoint {
        let u_diff = self.var_params.left.func.call(t);
        match self.var_params.left.ty {
            UDiffType::Ut => {
                let u_t = u_diff;
                UDiffPairPoint {
                    ux: point.ux - (u_t - point.ut) / self.const_params.a,
                    ut: u_t,
                }
            }
            UDiffType::Ux => {
                let u_x = u_diff;
                UDiffPairPoint {
                    ut: point.ut - (u_x - point.ux) * self.const_params.a,
                    ux: u_x,
                }
            }
            _ => unreachable!(),
        }
    }

    fn right_calc_point(&self, t: f64, point: UDiffPairPoint) -> UDiffPairPoint {
        let u_diff = self.var_params.right.func.call(t);
        match self.var_params.right.ty {
            UDiffType::Ut => {
                let u_t = u_diff;
                UDiffPairPoint {
                    ux: point.ux + (u_t - point.ut) / self.const_params.a,
                    ut: u_t,
                }
            }
            UDiffType::Ux => {
                let u_x = u_diff;
                UDiffPairPoint {
                    ut: point.ut + (u_x - point.ux) * self.const_params.a,
                    ux: u_x,
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn render_canvas(&self, ctx: &web_sys::CanvasRenderingContext2d) -> Result<(), JsValue> {
        const CANVAS_WIDTH: u32 = 480;
        const CANVAS_HEIGHT: u32 = 480;

        ctx.clear_rect(0.0, 0.0, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);

        let n = self.floor_udiff.len();

        let x_from_idx = |i| CANVAS_WIDTH as f64 * i as f64 / (n - 1) as f64;
        let t_y = |y| CANVAS_HEIGHT as f64 * (0.5 - y / self.const_params.l);

        let u_path = web_sys::Path2d::new()?;
        let u_x_path = web_sys::Path2d::new()?;
        let u_t_path = web_sys::Path2d::new()?;

        let init_point = self.floor_u.first().unwrap();
        u_path.move_to(0.0, t_y(init_point.get()));

        let init_point = self.floor_udiff.first().unwrap();
        u_x_path.move_to(0.0, t_y(init_point.get().ux));
        u_t_path.move_to(0.0, t_y(init_point.get().ut));

        self.floor_u
            .iter()
            .zip(self.floor_udiff.iter())
            .enumerate()
            .skip(1)
            .for_each(|(i, (u, p))| {
                u_path.line_to(x_from_idx(i), t_y(u.get()));
                u_x_path.line_to(x_from_idx(i), t_y(p.get().ux));
                u_t_path.line_to(x_from_idx(i), t_y(p.get().ut));
            });

        if self.views.u.visible {
            ctx.set_stroke_style(&self.views.u.color);
            ctx.stroke_with_path(&u_path);
        }
        if self.views.ux.visible {
            ctx.set_stroke_style(&self.views.ux.color);
            ctx.stroke_with_path(&u_x_path);
        }
        if self.views.ut.visible {
            ctx.set_stroke_style(&self.views.ut.color);
            ctx.stroke_with_path(&u_t_path);
        }

        Ok(())
    }

    #[wasm_bindgen(setter)]
    pub fn set_var_params(&mut self, var_params: VariableParams) {
        self.var_params = var_params;
    }

    #[wasm_bindgen(setter)]
    pub fn set_views(&mut self, views: CurveViews) {
        self.views = views;
    }

    fn generate_floor(
        ConstParams {
            bottom_u,
            bottom_u_t,
            a,
            l,
        }: &ConstParams,
    ) -> (Vec<Cell<f64>>, Vec<Cell<UDiffPairPoint>>) {
        let n = calc_tabulation_size(*a, *l);
        let h = l / (n - 1) as f64;
        let bottom_u_x = |x| (bottom_u.call(x + h) - bottom_u.call(x)) / h;

        (0..n)
            .map(|i| {
                let x = i as f64 / (n - 1) as f64 * l;
                (
                    Cell::new(bottom_u.call(x)),
                    Cell::new(UDiffPairPoint {
                        ut: bottom_u_t.call(x),
                        ux: bottom_u_x(x),
                    }),
                )
            })
            .unzip()
    }

    fn step_forward(&mut self) {
        let dt = self.step_dt();
        self.cur_t += dt;

        let Renderer {
            cur_t,
            floor_u,
            floor_udiff,
            floor_udiff_buffer,
            ..
        } = &*self;

        let n = floor_udiff.len();
        debug_assert_eq!(n, floor_udiff_buffer.len());

        // Euler method for tabulation of U
        floor_u
            .iter()
            .zip(floor_udiff.iter())
            .for_each(|(u, u_diff)| {
                u.replace(u.get() + dt * u_diff.get().ut / 2.0);
            });

        // Characteristics method
        floor_udiff_buffer[0].replace(self.left_calc_point(*cur_t, floor_udiff[1].get()));
        floor_udiff_buffer[1..n - 1]
            .iter()
            .zip(floor_udiff.iter().zip(floor_udiff.iter().skip(2)))
            .for_each(|(out_point, (a, b))| {
                out_point.replace(self.calc_point(a.get(), b.get()));
            });
        floor_udiff_buffer[n - 1].replace(self.right_calc_point(*cur_t, floor_udiff[n - 2].get()));

        mem::swap(&mut self.floor_udiff, &mut self.floor_udiff_buffer);
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
}
