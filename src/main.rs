use leptos::*;
use log::Level;

mod app;
mod components;

use app::App;

fn main() {
    let _ = console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App /> }
    });
}