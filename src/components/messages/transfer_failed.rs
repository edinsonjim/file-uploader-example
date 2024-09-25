use leptos::{component, create_signal, view, IntoView};
use web_sys::MouseEvent;

use crate::components::{buttons::button::Button, icons::info_icon::InfoIcon};

#[component]
pub fn TransferFailed<F>(on_try_again: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let (busy_reader, _) = create_signal(false);

    view! {
        <div class="w-full max-w-lg p-3 bg-white border border-gray-200 rounded-lg md:p-6 sm:p-2">
            <div class="flex flex-col items-center gap-y-8">
                <InfoIcon inner_class="h-24 w-24 text-rose-500" />

                <div class="flex flex-col items-center">
                    <div>Upload failed</div>
                    <div>Sorry! Something went wrong.</div>
                </div>

                <Button
                    busy_reader=busy_reader
                    on_click=on_try_again
                    label="Try again" />
            </div>
        </div>
    }
}
