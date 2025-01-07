use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Status {
    Idling,
    Attacking,
    Stopping,
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
            <span data-tauri-drag-region class="panel font-bold">
                "pts_bomber"
            </span>
            <em class="panel font-light whitespace-nowrap">{version}</em>
            <span
                data-tauri-drag-region
                class="panel w-full font-bold"
                class:bg-neutral-800=move || title.get() == Status::Idling
                class:bg-red-700=move || title.get() == Status::Attacking
                class:bg-yellow-400=move || title.get() == Status::Stopping
            >
                {move || match title.get() {
                    Status::Idling => "Простаивает",
                    Status::Attacking => "Атака",
                    Status::Stopping => "Остановка",
                }}
            </span>
            <div class="panel material-symbols-rounded">
                <button on:click=move |_| { minimize() }>"minimize"</button>
                <button on:click=move |_| { close() }>"close"</button>
            </div>
        </div>
    }
}
