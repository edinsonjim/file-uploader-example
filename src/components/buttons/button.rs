use leptos::ev::MouseEvent;
use leptos::{component, view, Callable, Callback, IntoView, ReadSignal, Show, SignalGet};

use crate::components::icons::spin_icon::SpinIcon;

#[component]
pub fn Button(
    busy_reader: ReadSignal<bool>,
    #[prop(into)] on_click: Callback<MouseEvent>,
    #[prop(default = "")] label: &'static str,
    #[prop(default = "")] busy_label: &'static str,
) -> impl IntoView {
    view! {
        <button
            class="h-9 flex justify-center items-center space-x-4 w-full text-white bg-violet-700 hover:bg-violet-800 focus:ring-4 focus:outline-none focus:ring-violet-300 font-medium rounded-lg text-sm text-center"
            disabled=move || busy_reader.get()
            on:click=move |ev| {
                if !busy_reader.get() {
                    on_click.call(ev);
                };
            }>

            <Show when=move ||busy_reader.get()>
            <SpinIcon inner_class="animate-spin h-5 w-5 mr-2 text-white" />
            </Show>

            {move || if busy_reader.get() {
                busy_label.to_string()
            } else {
                label.to_string()
            }}
        </button>
    }
}
