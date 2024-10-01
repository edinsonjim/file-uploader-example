use file_transfer_tutorial::pages::transfer_page::TransferPage;
use leptos::{component, mount_to_body, view, IntoView};

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-w-screen min-h-screen bg-violet-50">
            <TransferPage />
        </div>
    }
}

fn main() {
    mount_to_body(App)
}
