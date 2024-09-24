use leptos::{component, view, IntoView, SignalGet, SignalUpdate};
use leptos::{For, ReadSignal, SignalSet, WriteSignal};
use web_sys::File;

use crate::components::file::file_item::FileItem;

#[component]
pub fn FileList(
    files_reader: ReadSignal<Vec<File>>,
    files_setter: WriteSignal<Vec<File>>,
    transfer_pending: ReadSignal<bool>,
    dropped_setter: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <ul role="list">
            <For each=move || files_reader.get() key=|f| f.name() let:file>
            <FileItem
                name=file.name()
                size=file.size()
                processing_reader=transfer_pending
                on_remove=move |_| {
                    files_setter.update(|files| {
                        let index = files.iter().position(|f_iter| f_iter.name().eq(&file.name())).unwrap();
                        files.remove(index);
                    });

                    if files_reader.get().len() == 0 {
                        dropped_setter.set(false);
                    }
                }
            />
            </For>
        </ul>
    }
}
