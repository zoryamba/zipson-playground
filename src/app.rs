use crate::components::{Content, Header};
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Header />
        <Content />
    }
}