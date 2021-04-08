use std::{cell::Cell, mem};

use crate::{
    common_method::{DifferentialMethod, UTriplePoint},
    ConstParams, UDiffPairPoint, UDiffType, VariableParams, MIN_FRAMERATE, MIN_TABULATION_SIZE,
};

fn calc_tabulation_size(a: f64, l: f64) -> usize {
    MIN_TABULATION_SIZE.max((2.0 * l / a * MIN_FRAMERATE + 1.0).ceil() as usize)
}

pub struct CharacteristicsMethod {
    var_params: VariableParams,
    floor_u: Vec<Cell<f64>>,
    floor_udiff: Vec<Cell<UDiffPairPoint>>,
    floor_udiff_buffer: Vec<Cell<UDiffPairPoint>>,
    const_params: ConstParams,
    cur_t: f64,
}

impl DifferentialMethod for CharacteristicsMethod {
    fn new(var_params: VariableParams, const_params: ConstParams) -> CharacteristicsMethod {
        let (floor_u, floor_udiff) = Self::generate_floor(&const_params);
        let mut floor_udiff_buffer = Vec::new();
        floor_udiff_buffer.resize(floor_udiff.len(), Cell::default());
        Self {
            cur_t: 0.0,
            var_params,
            floor_u,
            floor_udiff,
            floor_udiff_buffer,
            const_params,
        }
    }

    fn get_min_dt(&self) -> f64 {
        2.0 * self.const_params.l / (self.const_params.a * (self.floor_udiff.len() - 1) as f64)
    }

    fn step_forward(&mut self) {
        let dt = self.get_min_dt();
        self.cur_t += dt;

        let Self {
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

    fn reset(&mut self, const_params: ConstParams) {
        let (floor_u, floor_udiff) = Self::generate_floor(&const_params);
        self.const_params = const_params;
        self.floor_u = floor_u;
        self.floor_udiff = floor_udiff;
        self.floor_udiff_buffer
            .resize(self.floor_udiff.len(), Cell::default());
    }

    fn set_var_params(&mut self, var_params: VariableParams) {
        self.var_params = var_params;
    }

    fn const_params(&self) -> ConstParams {
        self.const_params
    }

    fn cur_t(&self) -> f64 {
        self.cur_t
    }

    type TripleIterator<'s> = TripleIterator<'s>;

    fn iter<'s>(&'s self) -> TripleIterator<'s> {
        TripleIterator {
            iterable: &self,
            idx: 0,
        }
    }
}

impl CharacteristicsMethod {
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
}

pub struct TripleIterator<'s> {
    iterable: &'s CharacteristicsMethod,
    idx: usize,
}

impl Iterator for TripleIterator<'_> {
    type Item = UTriplePoint;

    fn next(&mut self) -> Option<UTriplePoint> {
        let i = self.idx;
        self.idx += 1;
        let udiff = self.iterable.floor_udiff.get(i)?.get();
        Some(UTriplePoint {
            u: self.iterable.floor_u.get(i)?.get(),
            ut: udiff.ut,
            ux: udiff.ux,
        })
    }
}
