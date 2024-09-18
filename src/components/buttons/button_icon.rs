use leptos::ev::MouseEvent;
use leptos::{component, view, Callable, Callback, IntoView, ReadSignal, SignalGet};

#[component]
pub fn ButtonIcon<F, IV>(
    busy_reader: ReadSignal<bool>,
    #[prop(into)] on_click: Callback<MouseEvent>,
    inner_icon: F,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <button
            disabled=move || busy_reader.get()
            class="inline-flex items-center text-base font-semibold text-gray-500 cursor-pointer ml-2 p-2 rounded-full hover:text-violet-500 hover:bg-violet-100"
            on:click=move |ev| {
                if !busy_reader.get() {
                    on_click.call(ev);
                };
            }>
            {inner_icon()}
        </button>
    }
}
