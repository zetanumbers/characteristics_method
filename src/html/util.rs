pub fn window() -> web_sys::Window {
    web_sys::window().expect("no window found")
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("no document found inside window")
}

pub fn request_animation_frame(callback: &js_sys::Function) -> i32 {
    window()
        .request_animation_frame(callback)
        .expect("no request_animation_frame found inside window")
}

pub fn cancel_animation_frame(handle: i32) {
    window()
        .cancel_animation_frame(handle)
        .expect("no cancel_animation_frame found inside window")
}

pub fn performance() -> web_sys::Performance {
    window()
        .performance()
        .expect("no performance found inside window")
}
