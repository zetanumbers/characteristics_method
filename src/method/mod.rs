use std::str::FromStr;

use crate::html;
use wasm_bindgen::JsCast;

pub mod characteristics;
pub mod display;

#[derive(Debug, Default, Clone, Copy)]
struct UPair {
    ut: f64,
    ux: f64,
}

pub struct ConstParams {
    u_xt0: js_sys::Function,
    ut_xt0: js_sys::Function,
    a: f64,
    l: f64,
}

impl ConstParams {
    fn from_html(params: &html::ConstParams) -> Self {
        Self {
            u_xt0: eval_js_function("x", params.u_xt0.value()),
            ut_xt0: eval_js_function("x", params.ut_xt0.value()),
            a: params.a.value_as_number(),
            l: params.l.value_as_number(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum DerivativeType {
    Ut,
    Ux,
}

impl FromStr for DerivativeType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Ut" => Ok(DerivativeType::Ut),
            "Ux" => Ok(DerivativeType::Ux),
            other => Err(other.to_owned()),
        }
    }
}

struct SideFunction {
    ty: DerivativeType,
    func: js_sys::Function,
}

impl SideFunction {
    fn from_html(side_function: &html::SideFunction) -> Self {
        Self {
            ty: side_function.ty.value().parse().unwrap(),
            func: eval_js_function("t", side_function.function.value()),
        }
    }
}

pub struct VariableParams {
    left: SideFunction,
    right: SideFunction,
}

impl VariableParams {
    fn from_html(params: &html::VarParams) -> Self {
        Self {
            left: SideFunction::from_html(&params.left),
            right: SideFunction::from_html(&params.right),
        }
    }
}

pub trait Method {
    type IterU<'a>: Iterator<Item = (f64, f64)> + 'a;
    type IterUt<'a>: Iterator<Item = (f64, f64)> + 'a;
    type IterUx<'a>: Iterator<Item = (f64, f64)> + 'a;

    fn new(var_params: VariableParams, const_params: ConstParams) -> Self;
    fn sync_to(&mut self, t: f64);
    fn iter_normalized<'a>(&'a self) -> (Self::IterU<'a>, Self::IterUt<'a>, Self::IterUx<'a>);
    fn reset(&mut self, params: ConstParams);
    fn set_var_params(&mut self, params: VariableParams);
}

fn eval_js_function(arg: &str, text: String) -> js_sys::Function {
    #[allow(unused_unsafe)]
    unsafe { js_sys::eval(&format!("({}) => {}", arg, text)) }
        .expect("text of evaluated function is mailformed")
        .unchecked_into()
}
