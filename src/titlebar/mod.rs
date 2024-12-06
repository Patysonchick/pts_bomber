use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Status {
    IsIdling,
    Attacking,
}

#[wasm_bindgen(module = "/src/titlebar/titlebar.js")]
extern "C" {
    #[wasm_bindgen]
    fn minimize();

    #[wasm_bindgen]
    fn toggleMaximize();

    #[wasm_bindgen]
    fn close();
}

#[component]
pub fn Titlebar(title: ReadSignal<Status>) -> impl IntoView {
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));

    view! {
        <div data-tauri-drag-region class="flex-element flex-row w-full justify-between">
            <div data-tauri-drag-region class="flex-element flex-row w-full">
                <span data-tauri-drag-region class="panel flex-element font-bold">"pts_bomber"</span>
                <span class="panel flex-element w-full font-light"><em>{version}</em></span>
                <span
                    data-tauri-drag-region
                    class="panel flex-element w-full font-bold"
                    class: bg-neutral-800 = move || title.get() == Status::IsIdling
                    class: bg-red-700 = move || title.get() == Status::Attacking
                >{
                    move || match title.get() {
                        Status::IsIdling => "Простаивает",
                        Status::Attacking => "Атака"
                    }
                }</span>
            </div>
            <div class="panel material-symbols-rounded flex-element flex-row">
                <button on:click=move |_| { minimize() }>"minimize"</button>
                <button on:click=move |_| { close() }>"close"</button>
            </div>
        </div>
    }
}
