use leptos::{component, view, IntoView};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="flex flex-col justify-center bg-blue-300 w-full h-auto pt-8 pb-8 mb-10 font-mono">
            <div class="flex title justify-center text-5xl pb-4 text-white">"Zipson"</div>
            <div class="flex description justify-center text-xl pb-4 text-white">
                "JSON parse and stringify with compression
"            </div>
            <div class="flex justify-center">
                <a
                    type="button"
                    href="https://github.com/zoryamba/serde_zipson"
                    target="_blank"
                    class="py-2.5 w-auto px-5 me-2 mb-2 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-white-100 hover:text-white-700 focus:z-10 focus:ring-4 focus:ring-gray-100 dark:focus:ring-gray-700  dark:hover:text-white dark:hover:bg-gray-700"
                >
                    "serde_zipson"
                </a>
                <a
                    type="button"
                    href="https://www.npmjs.com/package/zipson"
                    target="_blank"
                    class="py-2.5 w-auto px-5 me-2 mb-2 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-white-100 hover:text-white-700 focus:z-10 focus:ring-4 focus:ring-gray-100 dark:focus:ring-gray-700  dark:hover:text-white dark:hover:bg-gray-700"
                >
                    "NPM"
                </a>
            </div>
        </header>
    }
}