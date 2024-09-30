use leptos::*;

mod app;

use app::App;

fn main() {
    mount_to_body(|| {
        view! { <App /> }
    });
}