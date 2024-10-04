use leptos::{component, event_target_value, view, IntoView, SignalSet, WriteSignal};

#[component]
pub fn Input(initial_text: String, set_input: WriteSignal<String>) -> impl IntoView {
    view! {
        <div class="w-full border border-gray-200 rounded-lg  dark:border-blue-300">
            <div class="px-4 py-2 bg-white rounded-t-lg h-96">
                <textarea rows="4"
                          class="h-80 outline-none w-full px-0 text-sm text-gray-900 bg-white border-0 resize-none focus:ring-0 dark:text-dark dark:placeholder-gray-400"
                          placeholder="Write a comment..."
                          on:input=move |ev| { set_input.set(event_target_value(&ev)) }
                >
                    { initial_text }
                </textarea>
            </div>
        </div>
    }
}