use leptos::{component, create_signal, view, IntoView};
use web_sys::MouseEvent;

use crate::components::{buttons::button::Button, icons::check_icon::CheckIcon};

#[component]
pub fn TransferComplete<F>(on_continue: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let (busy_reader, _) = create_signal(false);

    view! {
        <div class="w-full max-w-lg p-3 bg-white border border-gray-200 rounded-lg md:p-6 sm:p-2">
            <div class="flex flex-col items-center gap-y-8">
                <CheckIcon inner_class="text-green-500 h-24 w-24" />

                <div class="flex flex-col items-center">
                    <div>Upload complete</div>
                    <div>Your files have been uploaded successfully.</div>
                </div>

                <Button
                    busy_reader=busy_reader
                    on_click=on_continue
                    label="Continue" />
            </div>
        </div>
    }
}
