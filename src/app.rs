use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div data-tauri-drag-region class="flex-element flex-row w-full justify-between">
            <div data-tauri-drag-region class="flex-element flex-row">
                <span data-tauri-drag-region class="panel flex-element font-bold">"pts_bomber"</span>
                <span class="panel flex-element font-light text"><em>"v2.0.0"</em></span>
            </div>
            <span data-tauri-drag-region class="panel w-full font-bold">"Is idle"</span>
            <div class="panel material-symbols-rounded flex-element flex-row">
                <button id="titlebar-minimize">"minimize"</button><button id="titlebar-close">"close"</button>
            </div>
        </div>
        <div class="panel flex-element flex-auto w-full center-elements justify-around">
            <div class="w-full flex-element center-elements">
                <div class="bg-black font-bold p-2 m-1 rounded-2xl">"Enter russian number"</div>
                <form class="flex-element flex-row center-elements">
                    <span class="panel bg-black">"🇷🇺"</span>
                    <input type="text" placeholder="+7 (9xx) xxx xx-xx" class="bg-black text-center font-bold w-full p-1 border-2 border-green-600 rounded-xl" />
                    <button type="submit" class="button material-symbols-rounded">"send"</button>
                </form>
            </div>
            <div class="w-full flex-element center-elements">
                <div class="bg-black font-bold p-2 m-1 rounded-2xl">"Logs"</div>
                <textarea readonly class="panel bg-black w-full">"Logs will be here"</textarea>
            </div>
        </div>
        <div class="flex-element flex-row w-full justify-around">
            <a href="https://github.com/Patysonchick/pts_bomber" class="panel flex-element rounded-b-none">
                <img src="public/github-mark.svg" alt="GitHub" class="h-7" />
            </a>
            <a href="https://t.me/pts_bomber" class="panel flex-element rounded-b-none">
                <img src="public/telegram.svg" alt="Telegram" class="h-7" />
            </a>
            <button class="panel flex-element rounded-b-none"><span class="material-symbols-rounded">"settings"</span></button>
        </div>
    }
}
