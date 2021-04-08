use std::borrow::Cow;

use seed::{prelude::*, *};

use crate::inputs;

pub struct Model {
    left_side_u: (DiffBy, inputs::functional::Model),
    right_side_u: (DiffBy, inputs::functional::Model),
}

impl Model {
    pub fn init() -> Model {
        Model {
            left_side_u: (
                DiffBy::T,
                inputs::functional::Model::new(
                    select![
                        ev(Ev::Change, |_| todo!()),
                        option!["Ut(0, t) = ", attrs![At::Value => "T", At::Selected => ""]],
                        option!["Ux(0, t) = ", attrs![At::Value => "X"]],
                    ],
                    Cow::Borrowed("x"),
                ),
            ),
            right_side_u: (
                DiffBy::T,
                inputs::functional::Model::new(
                    Node::Text(virtual_dom::Text::new("Ut(x, 0) = ")),
                    Cow::Borrowed("x"),
                ),
            ),
        }
    }

    pub fn view(&self) -> Node<Msg> {
        div![
            self.u_from_x_t0.view().map_msg(Msg::UFromXT0),
            self.ut_from_x_t0.view().map_msg(Msg::UtFromXT0),
            self.a.view().map_msg(Msg::A),
            self.l.view().map_msg(Msg::L),
        ]
    }
}

pub enum Msg {
    UFromXT0(inputs::functional::Msg),
    UtFromXT0(inputs::functional::Msg),
    A(inputs::numeric::Msg),
    L(inputs::numeric::Msg),
}

impl Msg {
    pub fn update(self, model: &mut Model) {
        match self {
            Msg::UFromXT0(msg) => msg.update(&mut model.u_from_x_t0),
            Msg::UtFromXT0(msg) => msg.update(&mut model.ut_from_x_t0),
            Msg::A(msg) => msg.update(&mut model.a),
            Msg::L(msg) => msg.update(&mut model.l),
        }
    }
}

pub enum DiffBy {
    T,
    X,
}
