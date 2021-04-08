use crate::{ConstParams, VariableParams};

#[derive(Default, Debug, Clone, Copy)]
pub struct UTriplePoint {
    pub u: f64,
    pub ut: f64,
    pub ux: f64,
}

pub trait DifferentialMethod {
    type TripleIterator<'s>: Iterator<Item = UTriplePoint>;

    fn new(var_params: VariableParams, const_params: ConstParams) -> Self;
    fn get_min_dt(&self) -> f64;
    fn step_forward(&mut self);
    fn iter<'s>(&'s self) -> Self::TripleIterator<'s>;
    fn const_params(&self) -> ConstParams;
    fn reset(&mut self, const_params: ConstParams);
    fn set_var_params(&mut self, var_params: VariableParams);
    fn cur_t(&self) -> f64;
    fn sync(&mut self, to: f64) {}
}
