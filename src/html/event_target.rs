use js_sys::Function;
use wasm_bindgen::JsValue;

#[allow(unused_imports)]
use super::{SideFunction, VarParams, VisualParams, Visuals};

pub trait EventTarget {
    fn add_event_listener(&self, type_: &str, listener: &Function) -> Result<(), JsValue>;
    fn remove_event_listener(&self, type_: &str, listener: &Function) -> Result<(), JsValue>;
}

impl EventTarget for web_sys::EventTarget {
    fn add_event_listener(&self, type_: &str, listener: &Function) -> Result<(), JsValue> {
        self.add_event_listener_with_callback(type_, listener)
    }

    fn remove_event_listener(&self, type_: &str, listener: &Function) -> Result<(), JsValue> {
        self.remove_event_listener_with_callback(type_, listener)
    }
}

#[allow(unused_macros)]
macro_rules! impl_event_target {
    ($name:path { $($field:ident),* }) => {
        impl_event_target!($name { $($field),* } ignore {});
    };
    ($name:path { $($field:ident),* } ignore { $($field_ignore:ident),* }) => {
        impl EventTarget for $name {
            fn add_event_listener(&self, type_: &str, listener: &Function) -> Result<(), JsValue> {
                let Self { $($field,)* $($field_ignore: _,)* } = self;
                Ok({
                    $($field.add_event_listener(type_, listener)?;)*
                })
            }

            fn remove_event_listener(&self, type_: &str, listener: &Function) -> Result<(), JsValue> {
                Ok({
                    $(self.$field.remove_event_listener(type_, listener)?;)*
                })
            }
        }
    };
}

// impl_event_target!(VarParams { left, right } ignore { pause_button });
// impl_event_target!(SideFunction { ty, function });
// impl_event_target!(Visuals { visibility, color });
// impl_event_target!(VisualParams { u, ut, ux });
