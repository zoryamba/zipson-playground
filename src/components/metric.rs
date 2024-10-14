use leptos::{component, view, IntoView};

#[component]
pub fn Metric(name: &'static str, units: &'static str, tooltip_text: &'static str, value: impl Fn() -> f32 + 'static) -> impl IntoView {
    view! {
        <div class="pr-5">
            <p class="flex ">
                <p class="pr-1">{name}</p>
                <div class="relative [--arrow-size:5px] [--tooltip-color:gray]">
                    <div id="grid" class=" ">
                        <div
                            id="gridItem"
                            class="relative h-3 w-3 cursor-pointer items-center shadow-md hover:z-50"
                            data-tooltip="LOOK HERE"
                        >
                            <svg
                                class="w-6 h-6 text-blue-300 dark:text-blue-300"
                                aria-hidden="true"
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                fill="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M2 12C2 6.477 6.477 2 12 2s10 4.477 10 10-4.477 10-10 10S2 17.523 2 12Zm9.008-3.018a1.502 1.502 0 0 1 2.522 1.159v.024a1.44 1.44 0 0 1-1.493 1.418 1 1 0 0 0-1.037.999V14a1 1 0 1 0 2 0v-.539a3.44 3.44 0 0 0 2.529-3.256 3.502 3.502 0 0 0-7-.255 1 1 0 0 0 2 .076c.014-.398.187-.774.48-1.044Zm.982 7.026a1 1 0 1 0 0 2H12a1 1 0 1 0 0-2h-.01Z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </div>

                    </div>
                    <div class="absolute right-0 bottom-[calc(theme(padding.8)+theme(padding.4)-.25rem)] w-72 origin-bottom -translate-x-2 translate-y-2 rounded-[.3rem] bg-[--tooltip-color] p-2 text-center text-xs text-white transition-transform scale-0 [#grid:has(#gridItem:nth-child(1):hover)~&]:scale-100">
                        {tooltip_text}
                    </div>

                </div>
            </p>
            <p>{value} <span class="pl-1">{units}</span></p>
        </div>
    }
}