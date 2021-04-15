pub mod animation;
pub mod event_target;
mod util;

use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, HtmlInputElement, HtmlSelectElement};

pub struct Root {
    pub canvas: HtmlCanvasElement,
    pub controls: Controls,
}

impl Root {
    pub fn new() -> Self {
        Root {
            canvas: util::document()
                .get_element_by_id("display")
                .unwrap()
                .unchecked_into(),
            controls: Controls::new(),
        }
    }

    pub fn set_disabled(&self, value: bool) {
        let Self {
            controls,
            canvas: _,
        } = self;
        controls.set_disabled(value);
    }
}

pub struct Controls {
    pub var_params: VarParams,
    pub const_params: ConstParams,
    pub visual_params: VisualParams,
}

impl Controls {
    fn new() -> Self {
        Controls {
            var_params: VarParams::new(),
            const_params: ConstParams::new(),
            visual_params: VisualParams::new(),
        }
    }

    fn set_disabled(&self, value: bool) {
        let Self {
            var_params,
            const_params,
            visual_params,
        } = self;
        var_params.set_disabled(value);
        const_params.set_disabled(value);
        visual_params.set_disabled(value);
    }
}

pub struct VarParams {
    pub left: SideFunction,
    pub right: SideFunction,
    pub pause_button: HtmlInputElement,
}

impl VarParams {
    fn new() -> Self {
        VarParams {
            left: SideFunction::new("left-function"),
            right: SideFunction::new("right-function"),
            pause_button: util::document()
                .get_element_by_id("pause-button")
                .unwrap()
                .unchecked_into(),
        }
    }

    fn set_disabled(&self, value: bool) {
        let Self {
            left,
            right,
            pause_button,
        } = self;
        left.set_disabled(value);
        right.set_disabled(value);
        pause_button.set_disabled(value);
    }

    pub fn set_onchange(&self, f: Option<&js_sys::Function>) {
        let Self {
            left,
            right,
            pause_button,
        } = self;
        left.set_onchange(f);
        right.set_onchange(f);
        pause_button.set_onclick(f);
    }
}

pub struct SideFunction {
    pub ty: HtmlSelectElement,
    pub function: HtmlInputElement,
}

impl SideFunction {
    fn new(tr_id: &str) -> Self {
        let tr = util::document().get_element_by_id(tr_id).unwrap();
        SideFunction {
            ty: tr
                .query_selector("td:nth-child(1) > select:nth-child(1)")
                .unwrap()
                .unwrap()
                .unchecked_into(),
            function: tr
                .query_selector("td:nth-child(2) > input:nth-child(1)")
                .unwrap()
                .unwrap()
                .unchecked_into(),
        }
    }

    fn set_disabled(&self, value: bool) {
        let Self {
            ty: diff_type,
            function,
        } = self;
        diff_type.set_disabled(value);
        function.set_disabled(value);
    }

    fn set_onchange(&self, f: Option<&js_sys::Function>) {
        let Self { ty, function } = self;
        ty.set_onchange(f);
        function.set_onchange(f);
    }
}

pub struct ConstParams {
    pub u_xt0: HtmlInputElement,
    pub ut_xt0: HtmlInputElement,
    pub a: HtmlInputElement,
    pub l: HtmlInputElement,
    pub set_button: HtmlInputElement,
}

impl ConstParams {
    fn new() -> Self {
        let document = util::document();
        ConstParams {
            u_xt0: document
                .get_element_by_id("u_xt0")
                .unwrap()
                .unchecked_into(),
            ut_xt0: document
                .get_element_by_id("ut_xt0")
                .unwrap()
                .unchecked_into(),
            a: document
                .get_element_by_id("a-param")
                .unwrap()
                .unchecked_into(),
            l: document
                .get_element_by_id("l-param")
                .unwrap()
                .unchecked_into(),
            set_button: document
                .get_element_by_id("set-button")
                .unwrap()
                .unchecked_into(),
        }
    }

    fn set_disabled(&self, value: bool) {
        let Self {
            u_xt0,
            ut_xt0,
            a,
            l,
            set_button,
        } = self;
        u_xt0.set_disabled(value);
        ut_xt0.set_disabled(value);
        a.set_disabled(value);
        l.set_disabled(value);
        set_button.set_disabled(value);
    }
}

pub struct VisualParams {
    pub u: Visuals,
    pub ut: Visuals,
    pub ux: Visuals,
}

impl VisualParams {
    fn new() -> Self {
        VisualParams {
            u: Visuals::new("u-visual"),
            ut: Visuals::new("ut-visual"),
            ux: Visuals::new("ux-visual"),
        }
    }

    fn set_disabled(&self, value: bool) {
        let Self { u, ut, ux } = self;
        u.set_disabled(value);
        ut.set_disabled(value);
        ux.set_disabled(value);
    }

    pub fn set_onchange(&self, f: Option<&js_sys::Function>) {
        let Self { u, ut, ux } = self;
        u.set_onchange(f);
        ut.set_onchange(f);
        ux.set_onchange(f);
    }
}

pub struct Visuals {
    pub visibility: HtmlInputElement,
    pub color: HtmlInputElement,
}

impl Visuals {
    fn new(tr_id: &str) -> Self {
        let tr = util::document().get_element_by_id(tr_id).unwrap();
        Visuals {
            visibility: tr
                .query_selector("td:nth-child(1) > input:nth-child(1)")
                .unwrap()
                .unwrap()
                .unchecked_into(),
            color: tr
                .query_selector("td:nth-child(2) > input:nth-child(1)")
                .unwrap()
                .unwrap()
                .unchecked_into(),
        }
    }

    fn set_disabled(&self, value: bool) {
        let Self { visibility, color } = self;
        visibility.set_disabled(value);
        color.set_disabled(value);
    }

    fn set_onchange(&self, f: Option<&js_sys::Function>) {
        let Self { visibility, color } = self;
        visibility.set_onclick(f);
        color.set_onchange(f);
    }
}
