use std::borrow::Cow;

use seed::{prelude::*, *};

use crate::inputs;

pub struct Model {
    u_from_x_t0: inputs::functional::Model,
    ut_from_x_t0: inputs::functional::Model,
    a: inputs::numeric::Model,
    l: inputs::numeric::Model,
}

impl Model {
    pub fn init() -> Model {
        Model {
            u_from_x_t0: inputs::functional::Model::new(
                Node::Text(virtual_dom::Text::new("U(x, 0) = ")),
                Cow::Borrowed("x"),
            ),
            ut_from_x_t0: inputs::functional::Model::new(
                Node::Text(virtual_dom::Text::new("Ut(x, 0) = ")),
                Cow::Borrowed("x"),
            ),
            a: inputs::numeric::Model::new("a = ".to_string(), 1.0),
            l: inputs::numeric::Model::new("l = ".to_string(), 1.0),
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
