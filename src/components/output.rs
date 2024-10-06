use leptos::{component, view, IntoView, ReadSignal, Show, SignalGet};

#[component]
pub fn Output(output: impl Fn() -> String + 'static, error: ReadSignal<Option<String>>) -> impl IntoView {
    view! {
        <div class="w-full border border-gray-200 rounded-lg dark:border-blue-300 bg-grey relative">
            <div class="bg-slate-200 px-4 py-2 rounded-t-lg h-96">
                <textarea
                    disabled
                    rows="4"
                    class="bg-slate-200 h-full outline-none w-full px-0 text-sm text-gray-900 resize-none border-0  focus:ring-0 dark:text-dark dark:placeholder-gray-400"
                    placeholder="Write a comment..."
                    required
                    prop:value=output
                ></textarea>
            </div>
            <Show when=move || { error.get().is_some() }>
                <div class="absolute left-0 right-0 top-0 bottom-0 grid place-items-center">
                    <div class="bg-red-400 w-auto h-auto p-2">{error.get().unwrap()}</div>
                </div>
            </Show>
        </div>
    }
}