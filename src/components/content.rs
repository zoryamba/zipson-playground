use crate::components::{TabContent, Tabs};
use indexmap::IndexMap;
use leptos::{component, view, IntoView, View};
use zipson_playground::{decode, encode};

#[component]
pub fn Content() -> impl IntoView {
    let arr: [(&str, View); 2] = [
        ("Encode", view! {
            <TabContent initial_text = "{\"key\":[\"value\"]}".to_string() convert=encode />
        }),
        ("Decode", view! {
            <TabContent initial_text = "{¨key¨|¨value¨÷}".to_string() convert=decode />
        }),
    ];

    let tabs: IndexMap<&str, View> = IndexMap::from(arr);

    view! {
        <main class="max-w-screen-xl px-4 m-auto">
            <div>
                <Tabs tabs/>
            </div>
            <section class="pt-10 pb-4">
                <h3 class="pb-2 text-xl">"What is this?"</h3>
                <p class="pb-2">
                    <a href="https://www.npmjs.com/package/zipson" class="text-blue-600 dark:text-blue-500 hover:underline">"Zipson"</a>
                    " is a drop-in alternative to JSON.parse/stringify with added compression and streaming support written in JavaScript."
                </p>
                <p class="pb-2">
                    <a href="https://github.com/zoryamba/serde_zipson" class="text-blue-600 dark:text-blue-500 hover:underline">"serde_zipson"</a>
                    " is "<a href="https://serde.rs/" class="text-blue-600 dark:text-blue-500 hover:underline">"serde"</a>"-compatible Rust implementation of zipson compression format."
                </p>
            </section>
            <section class="pt-10 pb-4">
                <h3 class="pb-2 text-xl">"Known issues"</h3>
                <ul class="list-disc pl-4">
                     <li>"initial conversion for some reason takes 40+ ms while consequent conversions take up to 1 ms"</li>
                     <li><code>"serde_zipson"</code>" panics on integer overflow"</li>
                     <li><code>"serde_zipson"</code>" repeat feature not working yet, so "<code>[1,1,1,1,1,1,1,1,1]</code>" ends up in "<code>"|ÊÊÊÊÊÊÊÊÊ÷"</code>" instead of "<code>"|Êþþþ^5÷"</code></li>
                </ul>
            </section>
        </main>
    }
}