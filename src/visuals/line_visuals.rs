use crate::util::get_event_value;
use seed::{prelude::*, *};

#[derive(Debug)]
pub struct Model {
    pub visiblity: bool,
    pub color: String,
    pub label: String,
}

impl Model {
    pub fn view(&self) -> Node<Msg> {
        div![
            C!["line-visuals"],
            input![
                C!["visibility"],
                attrs![
                    At::Type => "checkbox",
                ],
                IF!(
                    self.visiblity =>
                        attrs![
                            At::Checked => "",
                        ]
                ),
                ev(Ev::Change, |_| Msg::FlipVisiblity),
            ],
            div![C!["label"], &self.label],
            input![
                C!["color"],
                attrs![
                    At::Type => "color",
                    At::Value => &self.color,
                ],
                ev(Ev::Change, |e| Msg::SetColor(get_event_value(&e)))
            ]
        ]
    }
}

#[derive(Debug)]
pub enum Msg {
    FlipVisiblity,
    SetColor(String),
}

impl Msg {
    pub fn update(self, model: &mut Model) {
        match self {
            Msg::FlipVisiblity => model.visiblity = false,
            Msg::SetColor(color) => model.color = color,
        }
    }
}
