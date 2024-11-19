use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;
}
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="flex-element flex-row w-full justify-around">
            <a href="https://github.com/Patysonchick/pts_bomber" class="panel flex-element rounded-b-none" target="_blank">
                <img src="../public/github-mark.svg" alt="GitHub" class="h-7" />
            </a>
            <a href="https://t.me/pts_bomber" class="panel flex-element rounded-b-none" target="_blank">
                <img src="../public/telegram.svg" alt="Telegram" class="h-7" />
            </a>
            <button class="panel flex-element rounded-b-none" on:click=move |_| {
                spawn_local(async move {
                    invoke_without_args("show_about_window").await;
                });
            }>
                <span class="material-symbols-rounded">"settings"</span>
            </button>
        </div>
    }
}
