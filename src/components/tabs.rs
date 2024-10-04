use indexmap::IndexMap;
use leptos::{component, create_signal, view, IntoView, SignalGet, SignalSet, View};

#[component]
pub fn Tabs(tabs: IndexMap<&'static str, View>) -> impl IntoView
{
    let (active, set_active) = create_signal(0_usize);

    let mut tab_heads = vec![];
    let mut tab_contents = vec![];

    for (i, (key, value)) in tabs.into_iter().enumerate() {
        tab_heads.push(view! {
            <li class="me-2" on:click=move |_| set_active.set(i)>
                <a href="#"
                   class="inline-block p-4 border-b-2 rounded-t-lg tab"
                   class=("active", move || active.get() == i)
                >{ key }</a>
            </li>
        });

        tab_contents.push(view! {
            <div
                class="justify-center mt-8"
                class=("hidden", move || active.get() != i)
            >
                { value }
            </div>
        });
    }

    view! {
        <section>
            <div class="mb-2 text-sm font-medium text-center text-gray-500 border-b border-gray-200 dark:text-gray-400 dark:border-gray-700">
                <ul class="flex flex-wrap -mb-px">
                    { tab_heads }
                </ul>
            </div>
        </section>

        <section>
            { tab_contents }
        </section>
    }
}
