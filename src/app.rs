use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Home/>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center min-h-screen">
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
        </div>
    }
}