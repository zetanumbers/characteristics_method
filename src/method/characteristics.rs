use std::mem;

use super::{ConstParams, DerivativeType, Method, UPair, VariableParams};

use wasm_bindgen::JsValue;

struct Params {
    variable: VariableParams,
    constant: ConstParams,
}

pub struct CharacteristicsMethod {
    params: Params,
    cur_t: f64,
    floor_u: Vec<f64>,
    floor_udiff: Vec<UPair>,
    floor_udiff_buffer: Vec<UPair>,
}

impl Method for CharacteristicsMethod {
    fn new(var_params: VariableParams, const_params: ConstParams) -> Self {
        let mut out = Self {
            params: Params {
                variable: var_params,
                constant: const_params,
            },
            cur_t: 0.0,
            floor_u: Vec::new(),
            floor_udiff: Vec::new(),
            floor_udiff_buffer: Vec::new(),
        };
        out.reinit();
        out
    }

    fn sync_to(&mut self, t: f64) {
        let steps = ((t - self.cur_t) / self.calc_dt()) as i64;
        if steps >= 0 {
            for _ in 0..steps {
                self.step_forward()
            }
        } else {
            for _ in 0..-steps {
                self.step_back()
            }
        }
    }

    fn reset(&mut self, params: ConstParams) {
        self.params.constant = params;
        self.reinit();
    }

    fn set_var_params(&mut self, params: VariableParams) {
        self.params.variable = params;
    }

    type IterU<'a> = impl Iterator<Item = (f64, f64)> + 'a;
    type IterUt<'a> = impl Iterator<Item = (f64, f64)> + 'a;
    type IterUx<'a> = impl Iterator<Item = (f64, f64)> + 'a;

    fn iter_normalized<'a>(&'a self) -> (Self::IterU<'a>, Self::IterUt<'a>, Self::IterUx<'a>) {
        let n = self.floor_u.len();
        let l = self.params.constant.l;
        let ndx = 1.0 / (n - 1) as f64;
        let t = move |(i, y)| (i as f64 * ndx, 2.0 * y / l);
        (
            self.floor_u.iter().copied().enumerate().map(t),
            self.floor_udiff.iter().map(|up| up.ut).enumerate().map(t),
            self.floor_udiff.iter().map(|up| up.ux).enumerate().map(t),
        )
    }
}

impl CharacteristicsMethod {
    fn step_forward(&mut self) {
        let dt = self.calc_dt();
        self.cur_t += dt;

        let Self {
            cur_t,
            floor_u,
            floor_udiff,
            floor_udiff_buffer,
            params,
            ..
        } = &mut *self;

        let n = floor_udiff.len();
        debug_assert_eq!(n, floor_udiff_buffer.len());

        // Euler method for tabulation of U
        for (u, ud) in floor_u.iter_mut().zip(floor_udiff.iter()) {
            *u += dt * ud.ut;
        }

        // Characteristics method
        floor_udiff_buffer[0] = params.left_calc_point(*cur_t, floor_udiff[1]);
        for (out_ud, (left_ud, right_ud)) in floor_udiff_buffer[1..n - 1].iter_mut().zip(
            floor_udiff
                .iter()
                .copied()
                .zip(floor_udiff.iter().copied().skip(2)),
        ) {
            *out_ud = params.calc_point(left_ud, right_ud);
        }
        floor_udiff_buffer[n - 1] = params.right_calc_point(*cur_t, floor_udiff[n - 2]);

        mem::swap(&mut self.floor_udiff, &mut self.floor_udiff_buffer);
    }

    fn step_back(&mut self) {
        // maybe TODO later
    }

    fn calc_dt(&self) -> f64 {
        self.params.constant.l / (self.params.constant.a * (self.floor_udiff.len() - 1) as f64)
    }

    fn reinit(&mut self) {
        let ConstParams {
            a,
            l,
            ref u_xt0,
            ref ut_xt0,
        } = self.params.constant;
        let n = calc_tabulation_size(a, l);

        self.floor_u.resize_with(n, Default::default);
        self.floor_udiff.resize_with(n, Default::default);
        self.floor_udiff_buffer.resize_with(n, Default::default);

        let dx = 1.0 / (n - 1) as f64 * l;

        for (i, u) in self.floor_u.iter_mut().enumerate() {
            *u = js_call_f64_to_f64(&u_xt0, i as f64 * dx);
        }

        for (i, upair) in self.floor_udiff.iter_mut().enumerate() {
            upair.ut = js_call_f64_to_f64(&ut_xt0, i as f64 * dx);
            upair.ux = (js_call_f64_to_f64(&u_xt0, (i as f64 + 0.5) * dx)
                - js_call_f64_to_f64(&u_xt0, (i as f64 - 0.5) * dx))
                / dx;
        }

        self.cur_t = 0.0;

        fn calc_tabulation_size(a: f64, l: f64) -> usize {
            const MIN_FRAMERATE: f64 = 60.0;
            const MIN_TABULATION_SIZE: usize = 257;

            MIN_TABULATION_SIZE.max((2.0 * l / a * MIN_FRAMERATE + 1.0).ceil() as usize)
        }
    }
}

impl Params {
    fn calc_point(&self, left: UPair, right: UPair) -> UPair {
        let a = self.constant.a;
        UPair {
            ux: (left.ux - left.ut / a + right.ux + right.ut / a) / 2.0,
            ut: (left.ut - left.ux * a + right.ut + right.ux * a) / 2.0,
        }
    }

    fn left_calc_point(&self, t: f64, point: UPair) -> UPair {
        let u_diff = js_call_f64_to_f64(&self.variable.left.func, t);
        match self.variable.left.ty {
            DerivativeType::Ut => {
                let u_t = u_diff;
                UPair {
                    ux: point.ux - (u_t - point.ut) / self.constant.a,
                    ut: u_t,
                }
            }
            DerivativeType::Ux => {
                let u_x = u_diff;
                UPair {
                    ut: point.ut - (u_x - point.ux) * self.constant.a,
                    ux: u_x,
                }
            }
        }
    }

    fn right_calc_point(&self, t: f64, point: UPair) -> UPair {
        let u_diff = js_call_f64_to_f64(&self.variable.right.func, t);
        match self.variable.right.ty {
            DerivativeType::Ut => {
                let u_t = u_diff;
                UPair {
                    ux: point.ux + (u_t - point.ut) / self.constant.a,
                    ut: u_t,
                }
            }
            DerivativeType::Ux => {
                let u_x = u_diff;
                UPair {
                    ut: point.ut + (u_x - point.ux) * self.constant.a,
                    ux: u_x,
                }
            }
        }
    }
}

fn js_call_f64_to_f64(f: &js_sys::Function, v: f64) -> f64 {
    f.call1(&JsValue::NULL, &JsValue::from_f64(v))
        .expect("failed to call js function")
        .as_f64()
        .expect("result type is not f64")
}
