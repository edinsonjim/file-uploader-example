use leptos::{component, view, IntoView};

#[component]
pub fn CheckIcon(#[prop()] inner_class: &'static str) -> impl IntoView {
    view! {
        <svg class=inner_class class="text-green-500 h-24 w-24" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
        </svg>
    }
}
