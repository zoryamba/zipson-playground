use crate::components::{Input, Metric, Output};
use leptos::{component, create_memo, create_signal, view, IntoView, SignalGet, SignalSet};
use log::{info, warn};
use zipson_playground::ConversionResult;

#[component(transparent)]
pub fn TabContent(initial_text: String, convert: fn(String) -> anyhow::Result<ConversionResult>) -> impl IntoView {
    let (input, set_input) = create_signal(initial_text.clone());
    let (error, set_error) = create_signal(None);

    let output = create_memo(move |prev| {
        match convert(input.get()) {
            Ok(result) => {
                info!("converted successfully {:?}", result);
                if let Some(_) = error.get() {
                    set_error.set(None);
                }
                result
            }
            Err(error) => {
                warn!("conversion error {:?}", error);
                set_error.set(Some(error.to_string()));
                prev.map_or(Default::default(), |value: &ConversionResult| value.clone())
            }
        }
    });

    view! {
        <div class="flex">
            <Input initial_text set_input />
            <Output output=move || output.get().value error />
        </div>

        <div class="flex justify-between">
            <div>
                <p class="ms-auto text-lg text-gray-500 dark:text-gray-400">
                    "Input your own JSON and try it out!"
                </p>
            </div>
            <div class="flex">
                <Metric
                    name="Size reduced by"
                    units="%"
                    tooltip_text="The amount the output size was reduced by zipson compared to minified JSON of the same data. If the original data was of length 500, and the zipson output was of length 100, there was a size reduction of 80%."
                    value=move || output.get().size_reduced_by
                />
                <Metric
                    name="Compression time"
                    units="ms"
                    tooltip_text="The time it took to stringify and compress the data with zipson. Note that this is for a single iteration and is not an accurate benchmark of performance as there can be occasional spikes unrelated to actual zipson performance."
                    value=move || output.get().conversion_time
                />
            </div>
        </div>
    }
}