use leptos::{component, create_action, create_signal, view, IntoView, Show, SignalGet, SignalSet};
use web_sys::{File, MouseEvent};

use crate::{
    components::{
        dropzone::dropzone_previewer::DropzonePreview,
        messages::{transfer_complete::TransferComplete, transfer_failed::TransferFailed},
    },
    services,
};

#[component]
pub fn TransferPage() -> impl IntoView {
    let (files_reader, files_setter) = create_signal(Vec::<File>::new());

    let transfer_action = create_action(|files: &Vec<File>| {
        let selected_files = files.to_owned();
        async move { services::transferer::transfer_file(&selected_files).await }
    });
    let transfer_pending = transfer_action.pending();
    let transfer_reply = transfer_action.value();

    view! {
        <div class="flex items-center justify-center w-full p-2 md:p-8">
            <Show when=move || transfer_reply.get().is_none()>
            <DropzonePreview
                files_reader=files_reader
                files_setter=files_setter
                transfer_pending=transfer_pending
                on_transfer=move |ev: MouseEvent| {
                    ev.prevent_default();
                    transfer_action.dispatch(files_reader.get());
                } />
            </Show>

            <Show when=move || transfer_reply.get().is_some()>

                <Show when=move || transfer_reply.get().unwrap().is_ok()>
                <TransferComplete
                    on_continue=move |ev: MouseEvent| {
                        ev.prevent_default();
                        transfer_reply.set(None);
                    } />
                </Show>

                <Show when=move || transfer_reply.get().unwrap().is_err()>
                <TransferFailed
                    on_try_again=move |ev: MouseEvent| {
                        ev.prevent_default();
                        transfer_reply.set(None);
                    } />
                </Show>
            </Show>
        </div>
    }
}
