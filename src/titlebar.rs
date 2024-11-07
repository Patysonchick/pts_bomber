use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/window.js")]
extern "C" {
    #[wasm_bindgen]
    fn minimize();

    #[wasm_bindgen]
    fn toggleMaximize();

    #[wasm_bindgen]
    fn close();
}

#[component]
pub fn Titlebar() -> impl IntoView {
    view! {
        <div data-tauri-drag-region class="flex-element flex-row w-full justify-between">
            <div data-tauri-drag-region class="flex-element flex-row">
                <span data-tauri-drag-region class="panel flex-element font-bold">"pts_bomber"</span>
                <span class="panel flex-element font-light text"><em>"v2.0.0"</em></span>
            </div>
            <span data-tauri-drag-region class="panel w-full font-bold">"Is idle"</span>
            <div class="panel material-symbols-rounded flex-element flex-row">
                <button on:click=move |_| { minimize() }>"minimize"</button>
                <button on:click=move |_| { close() }>"close"</button>
            </div>
        </div>
    }
}
