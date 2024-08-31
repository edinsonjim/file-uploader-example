use leptos::{component, mount_to_body, view, IntoView};

#[component]
fn App() -> impl IntoView {
    view! {
        <p>Hello World</p>
    }
}

fn main() {
    mount_to_body(App)
}