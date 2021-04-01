use crate::util::get_event_value;
use seed::{prelude::*, *};
use std::num::ParseFloatError;

pub struct Model {
    pub label: String,
    pub value: f64,
    pub fallback: String,
    pub error: Option<ParseFloatError>,
}

impl Model {
    pub fn new(label: String, value: f64) -> Model {
        Model {
            label,
            value,
            fallback: value.to_string(),
            error: None,
        }
    }

    pub fn view(&self) -> Node<Msg> {
        div![
            C!["numeric-input"],
            &self.label,
            input![
                attrs![
                    At::Type => "text",
                    At::Value => self.fallback,
                ],
                self.error.as_ref().map(|error| attrs![
                    At::Class => "input-error",
                    At::Title => error
                ]),
                ev(Ev::Input, |e| Msg(get_event_value(&e)))
            ]
        ]
    }
}

pub struct Msg(String);

impl Msg {
    pub fn update(self, model: &mut Model) {
        match self.0.parse::<f64>() {
            Ok(value) => {
                model.value = value;
                model.error = None;
            }
            Err(error) => model.error = Some(error),
        }
        model.fallback = self.0;
    }
}
