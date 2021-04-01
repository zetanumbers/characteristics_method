use seed::{prelude::*, *};

pub mod line_visuals;

pub struct Model {
    u: line_visuals::Model,
    ut: line_visuals::Model,
    ux: line_visuals::Model,
}

impl Model {
    pub fn init() -> Model {
        Model {
            u: line_visuals::Model {
                visiblity: true,
                color: "#ff0000".to_string(),
                label: "U".to_string(),
            },
            ut: line_visuals::Model {
                visiblity: false,
                color: "#00ff00".to_string(),
                label: "Ut".to_string(),
            },
            ux: line_visuals::Model {
                visiblity: false,
                color: "#0000ff".to_string(),
                label: "Ux".to_string(),
            },
        }
    }

    pub fn view(&self) -> Node<Msg> {
        div![
            C!["visuals"],
            "Visuals",
            self.u.view().map_msg(Msg::UMsg),
            self.ut.view().map_msg(Msg::UtMsg),
            self.ux.view().map_msg(Msg::UxMsg),
        ]
    }
}

pub enum Msg {
    UMsg(line_visuals::Msg),
    UtMsg(line_visuals::Msg),
    UxMsg(line_visuals::Msg),
}

impl Msg {
    pub fn update(self, model: &mut Model) {
        match self {
            Msg::UMsg(msg) => msg.update(&mut model.u),
            Msg::UtMsg(msg) => msg.update(&mut model.ut),
            Msg::UxMsg(msg) => msg.update(&mut model.ux),
        }
    }
}
