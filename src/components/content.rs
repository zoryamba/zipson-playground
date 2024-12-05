use crate::components::{TabContent, Tabs};
use indexmap::IndexMap;
use leptos::{component, view, IntoView, View};
use zipson_playground::{decode, encode};

#[component]
pub fn Content() -> impl IntoView {
    let arr: [(&str, View); 2] = [
        ("Encode", view! { <TabContent initial_text="{\"key\":[\"value\"]}".to_string() convert=encode /> }),
        ("Decode", view! { <TabContent initial_text="{¨key¨|¨value¨÷}".to_string() convert=decode /> }),
    ];

    let tabs: IndexMap<&str, View> = IndexMap::from(arr);

    view! {
        <main class="max-w-screen-xl px-4 m-auto">
            <div>
                <Tabs tabs />
            </div>
            <section class="pt-10 pb-4">
                <h3 class="pb-2 text-xl">"What is this?"</h3>
                <p class="pb-2">
                    <a
                        href="https://www.npmjs.com/package/zipson"
                        class="text-blue-600 dark:text-blue-500 hover:underline"
                        target="_blank"
                    >
                        "Zipson"
                    </a>
                    " is a drop-in alternative to JSON.parse/stringify with added compression and streaming support written in JavaScript."
                </p>
                <p class="pb-2">
                    <a
                        href="https://github.com/zoryamba/serde_zipson"
                        class="text-blue-600 dark:text-blue-500 hover:underline"
                        target="_blank"
                    >
                        "serde_zipson"
                    </a>
                    " is "
                    <a
                        href="https://serde.rs/"
                        class="text-blue-600 dark:text-blue-500 hover:underline"
                        target="_blank"
                    >
                        "serde"
                    </a>
                    "-compatible Rust implementation of zipson compression format."
                </p>
            </section>
            <section class="pt-10 pb-4">
                <h3 class="pb-2 text-xl">"Known issues"</h3>
                <ul class="list-disc pl-4">
                    <li>
                        "initial conversion for some reason takes 40+ ms while consequent conversions take up to 1 ms"
                    </li>
                </ul>
            </section>
            <section class="pt-10 pb-4">
                <h3 class="pb-2 text-xl">
                    <code>"serde_zipson"</code>
                    " known issues"
                </h3>
                <ul class="list-disc pl-4">
                    <li>
                        <code>"serialize_struct"</code>" / "<code>"deserialize_struct"</code>
                        " are not implemented yet, so serde "
                        <code>"derive"</code>
                        " doesn't work for structs"
                    </li>
                    <li>
                        <code>"serialize_enum"</code>" / "<code>"deserialize_enum"</code>
                        " are not implemented yet, so serde "
                        <code>"derive"</code>
                        " doesn't work for enums"
                    </li>
                    <li>
                        "panics on integer overflow"
                    </li>
                    <li>
                        "object template feature not working yet, so "
                        <code>"[{\"key\":\"value1\"},{\"key\":\"value2\"}]"</code>
                        " ends up in "
                        <code>"|{¨key¨¨value1¨}{ß0¨value2¨}÷"</code>
                        " instead of "
                        <code>"|¦¨key¨‡¨value1¨¨value2¨—÷"</code>
                    </li>
                </ul>
            </section>
        </main>
    }
}