use std::borrow::Cow;

use crate::util::get_event_value;
use js_sys::{eval, Function};
use seed::{prelude::*, *};
use wasm_bindgen::{JsCast, JsValue};

pub struct Model {
    label: Node<Msg>,
    argument: Cow<'static, str>,
    function: Function,
    fallback: String,
    error: Option<JsValue>,
}

impl Model {
    pub fn new(label: Node<Msg>, argument: Cow<'static, str>) -> Self {
        let fallback = "0".to_string();
        Model {
            label,
            function: function_from_str(&argument, &fallback).unwrap(),
            fallback,
            argument,
            error: None,
        }
    }

    pub fn view(&self) -> Node<Msg> {
        div![
            self.label.clone(),
            input![
                attrs![
                    At::Type => "text",
                    At::Value => self.fallback,
                ],
                self.error.as_ref().map(|e| attrs![
                    At::Title => &format!("{:?}", e),
                    At::Class => "input-error",
                ]),
                ev(Ev::Input, |e| Msg::HandleInput(get_event_value(&e)))
            ]
        ]
    }

    fn recompute(&mut self) {
        match function_from_str(&self.argument, &self.fallback) {
            Ok(f) => {
                self.function = f;
                self.error = None;
            }
            Err(e) => self.error = Some(e),
        }
    }

    fn eval(&self, arg: f64) -> Result<f64, JsValue> {
        let result = self
            .function
            .call1(&JsValue::NULL, &JsValue::from_f64(arg))?;
        match result.as_f64() {
            Some(v) => Ok(v),
            None => Err((***js_sys::TypeError::new(&format!(
                "Expected number as a result value, but got: {:?}",
                result
            )))
            .clone()),
        }
    }

    pub fn eval_update(&mut self, arg: f64) -> Option<f64> {
        match self.eval(arg) {
            Ok(r) => Some(r),
            Err(e) => {
                self.error = Some(e);
                None
            }
        }
    }
}

fn function_from_str(arg: &str, s: &str) -> Result<Function, JsValue> {
    eval(&format!("{} => ({})", arg, s))?.dyn_into::<Function>()
}

pub enum Msg {
    SetArgument(Cow<'static, str>),
    HandleInput(String),
    Invalidate(JsValue),
}

impl Msg {
    pub fn update(self, model: &mut Model) {
        match self {
            Msg::SetArgument(arg) => model.argument = arg,
            Msg::HandleInput(input) => model.fallback = input,
            Msg::Invalidate(error) => {
                model.error = Some(error);
                return;
            }
        }
        model.recompute();
    }
}
