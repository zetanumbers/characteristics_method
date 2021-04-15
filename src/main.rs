#![allow(incomplete_features)]
#![feature(generic_associated_types, min_type_alias_impl_trait)]

use method::{characteristics::CharacteristicsMethod, display::Display};

mod html;
mod method;

fn main() {
    console_error_panic_hook::set_once();

    let root = Box::leak(Box::new(html::Root::new()));
    root.set_disabled(false);
    Display::<CharacteristicsMethod>::leak_from_html(root);
}
