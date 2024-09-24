use leptos::ev::MouseEvent;
use leptos::html::Label;
use leptos::{
    component, create_node_ref, create_signal, view, Callable, Callback, IntoView, ReadSignal,
    Show, SignalGet, SignalSet, SignalUpdate, WriteSignal,
};
use leptos_use::{use_drop_zone_with_options, UseDropZoneOptions, UseDropZoneReturn};
use web_sys::wasm_bindgen::JsCast;
use web_sys::{js_sys, Event, File, HtmlInputElement};

use crate::components::buttons::button::Button;
use crate::components::dropzone::dropzone_bar::DropzoneBar;
use crate::components::file::file_list::FileList;

#[component]
pub fn DropzonePreview(
    files_reader: ReadSignal<Vec<File>>,
    files_setter: WriteSignal<Vec<File>>,
    transfer_pending: ReadSignal<bool>,
    #[prop(into)] on_transfer: Callback<MouseEvent>,
) -> impl IntoView {
    let (dropped, set_dropped) = create_signal(false);

    let drop_zone_el = create_node_ref::<Label>();

    let UseDropZoneReturn {
        is_over_drop_zone: _,
        files: _,
    } = use_drop_zone_with_options(
        drop_zone_el,
        UseDropZoneOptions::default()
            .on_drop(move |ev| {
                files_setter.update(move |f| *f = ev.files);
                set_dropped.set(true);
            })
            .on_enter(move |_| set_dropped.set(false)),
    );

    let on_change_file = move |ev: Event| {
        ev.stop_propagation();

        let input_file_el = ev
            .target()
            .unwrap()
            .dyn_ref::<HtmlInputElement>()
            .unwrap()
            .clone();

        let selected_files: Vec<File> = input_file_el
            .files()
            .map(|f| js_sys::Array::from(&f).to_vec())
            .unwrap_or_default()
            .into_iter()
            .map(web_sys::File::from)
            .collect();

        files_setter.update(move |f| *f = selected_files);
        set_dropped.set(true);
    };

    view! {
        <div class="w-full max-w-lg p-3 bg-white border border-gray-200 rounded-lg md:p-6 sm:p-2">
            <div class="drop_zone_file_container">
                <label node_ref=drop_zone_el
                    for="drop_zone_input"
                    class="flex flex-col items-center justify-center w-full h-28 border-2 border-violet-300 border-dashed rounded-lg cursor-pointer bg-violet-50 hover:bg-violet-100">
                    <DropzoneBar />
                </label>

                <input id="drop_zone_input"
                    class="hidden"
                    type="file"
                    multiple
                    on:change=on_change_file />

                <Show when=move ||dropped.get()>
                <div class="flow-root mt-3">
                    <FileList
                        files_reader=files_reader
                        files_setter=files_setter
                        transfer_pending=transfer_pending
                        dropped_setter=set_dropped
                    />
                </div>
                </Show>


                <Show when=move ||dropped.get()>
                <div class="mt-3">
                    <Button
                        label="Transfer"
                        busy_label="Transferring..."
                        busy_reader=transfer_pending
                        on_click=move |ev| {
                            on_transfer.to_owned().call(ev);
                        }/>
                </div>
                </Show>
            </div>
        </div>
    }
}
