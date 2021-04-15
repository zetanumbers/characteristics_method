use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::CanvasRenderingContext2d;

use super::{ConstParams, Method, VariableParams};
use crate::html::{self, animation::Animation};

pub struct Visuals {
    visible: bool,
    color: String,
}

impl Visuals {
    fn from_html(params: &html::Visuals) -> Visuals {
        Visuals {
            visible: params.visibility.checked(),
            color: params.color.value(),
        }
    }
}

pub struct VisualParams {
    u: Visuals,
    ut: Visuals,
    ux: Visuals,
}

impl VisualParams {
    fn from_html(params: &html::VisualParams) -> VisualParams {
        VisualParams {
            u: Visuals::from_html(&params.u),
            ut: Visuals::from_html(&params.ut),
            ux: Visuals::from_html(&params.ux),
        }
    }
}

pub struct Display<T>
where
    T: Method,
{
    method: T,
    views: VisualParams,
    animation: Option<Rc<RefCell<Animation>>>,
    ctx: CanvasRenderingContext2d,
    canvas_sizes: (u32, u32),
}

impl<T> Display<T>
where
    T: Method + 'static,
{
    pub fn leak_from_html(html::Root { controls, canvas }: &'static html::Root) {
        let var_params = VariableParams::from_html(&controls.var_params);
        let const_params = ConstParams::from_html(&controls.const_params);

        let wrapped_self: &'static RefCell<Self> = Box::leak(Box::new(RefCell::new(Self {
            method: T::new(var_params, const_params),
            views: VisualParams::from_html(&controls.visual_params),
            animation: None,
            ctx: canvas.get_context("2d").unwrap().unwrap().unchecked_into(),
            canvas_sizes: (canvas.width(), canvas.height()),
        })));
        wrapped_self.borrow_mut().animation = Some(Animation::new(move |t| {
            let mut display = wrapped_self.borrow_mut();
            display.method.sync_to(t / 1000.0);
            display.render();
        }));
        wrapped_self.borrow().render();

        Box::leak(Box::new(DisplayHandlers::new(wrapped_self, controls)));
    }

    fn render(&self) {
        let (canvas_width, canvas_height) = self.canvas_sizes;
        let ctx = &self.ctx;

        ctx.clear_rect(0.0, 0.0, canvas_width as f64, canvas_height as f64);

        let t = |(x, y): (f64, f64)| {
            (
                x * canvas_width as f64,
                0.5 * (1.0 - y) * canvas_height as f64,
            )
        };

        let (u_iter, ut_iter, ux_iter) = self.method.iter_normalized();

        stroke_path(ctx, u_iter.map(t), &self.views.u);
        stroke_path(ctx, ut_iter.map(t), &self.views.ut);
        stroke_path(ctx, ux_iter.map(t), &self.views.ux);

        fn stroke_path(
            ctx: &CanvasRenderingContext2d,
            mut iter: impl Iterator<Item = (f64, f64)>,
            visuals: &Visuals,
        ) {
            if visuals.visible {
                ctx.begin_path();
                ctx.set_stroke_style(&JsValue::from_str(&visuals.color));
                let (x, y) = iter.next().unwrap();
                ctx.move_to(x, y);
                for (x, y) in iter {
                    ctx.line_to(x, y)
                }
                ctx.stroke();
            }
        }
    }

    fn pause(&mut self) {
        self.animation.as_ref().unwrap().borrow_mut().pause();
    }

    fn reset(&mut self, controls: &'static html::Controls) {
        if controls.var_params.pause_button.value() == "Pause" {
            controls.var_params.pause_button.set_value("Resume");
        }
        self.animation.as_ref().unwrap().borrow_mut().reset();
        self.method
            .reset(ConstParams::from_html(&controls.const_params));
        self.render();
    }

    fn resume(display: &'static RefCell<Self>) {
        display
            .borrow()
            .animation
            .as_ref()
            .unwrap()
            .borrow_mut()
            .resume();
    }
}

/// Use this as a singleton
pub struct DisplayHandlers {
    refresh_var_params: Closure<dyn Fn()>,
    reset: Closure<dyn Fn()>,
    refresh_visual_params: Closure<dyn Fn()>,
    pause: Closure<dyn Fn()>,
    controls: &'static html::Controls,
}

impl DisplayHandlers {
    pub fn new<M>(display: &'static RefCell<Display<M>>, controls: &'static html::Controls) -> Self
    where
        M: Method,
    {
        let out = Self {
            refresh_var_params: Closure::wrap(Box::new(move || {
                display
                    .borrow_mut()
                    .method
                    .set_var_params(VariableParams::from_html(&controls.var_params))
            })),
            refresh_visual_params: Closure::wrap(Box::new(move || {
                display.borrow_mut().views = VisualParams::from_html(&controls.visual_params)
            })),
            reset: Closure::wrap(Box::new(move || {
                display.borrow_mut().reset(controls);
            })),
            pause: Closure::wrap(Box::new(move || {
                if controls.var_params.pause_button.value() == "Pause" {
                    display.borrow_mut().pause();
                    controls.var_params.pause_button.set_value("Resume");
                } else {
                    Display::resume(display);
                    controls.var_params.pause_button.set_value("Pause");
                }
            })),
            controls,
        };

        // Set event handlers
        out.controls
            .var_params
            .set_onchange(Some(out.refresh_var_params.as_ref().unchecked_ref()));
        out.controls
            .visual_params
            .set_onchange(Some(out.refresh_visual_params.as_ref().unchecked_ref()));
        out.controls
            .const_params
            .set_button
            .set_onclick(Some(out.reset.as_ref().unchecked_ref()));
        out.controls
            .var_params
            .pause_button
            .set_onclick(Some(out.pause.as_ref().unchecked_ref()));

        out
    }
}

impl Drop for DisplayHandlers {
    fn drop(&mut self) {
        self.controls.var_params.set_onchange(None);
        self.controls.visual_params.set_onchange(None);
        self.controls.const_params.set_button.set_onclick(None);
        self.controls.var_params.pause_button.set_onclick(None);
    }
}
