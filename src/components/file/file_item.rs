use human_bytes::human_bytes;
use leptos::ev::MouseEvent;
use leptos::{component, view, IntoView, ReadSignal};

use crate::components::buttons::button_icon::ButtonIcon;
use crate::components::icons::trash_icon::TrashIcon;

#[component]
pub fn FileItem<F>(
    name: String,
    size: f64,
    processing_reader: ReadSignal<bool>,
    on_remove: F,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
        <li class="border border-gray-200 rounded-lg mb-2 p-3">
            <div class="flex items-center">
                <div class="flex-1 min-w-0">
                    <p class="text-sm font-medium text-gray-900 truncate">
                        {name}
                    </p>
                    <p class="text-sm text-gray-500 truncate">
                        Size: {human_bytes(size)}
                    </p>
                </div>
                <ButtonIcon
                    busy_reader=processing_reader
                    on_click=on_remove
                    inner_icon=||view! { <TrashIcon inner_class="w-6 h-6" /> }
                    >
                </ButtonIcon>
            </div>
        </li>
    }
}
