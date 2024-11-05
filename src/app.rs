use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use crate::titlebar::Titlebar;

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

#[derive(Serialize, Deserialize)]
struct EchoArgs<'a> {
    msg: &'a str,
}

#[derive(Serialize, Deserialize)]
struct FormatPhoneRuArgs<'a> {
    numbers: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (input_field, set_input_field) = create_signal(String::new());
    let (phone, set_phone) = create_signal(String::new());

    let update_input = move |ev| {
        let v = event_target_value(&ev);
        set_input_field.set(v);
    };

    let format_input = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let input_field = input_field.get_untracked();

            let args = serde_wasm_bindgen::to_value(&FormatPhoneRuArgs { numbers: &input_field }).unwrap();
            let formated_phone = invoke("format_phone_ru", args).await.as_string().unwrap();
            set_phone.set(formated_phone);
            log(&phone.get_untracked());
        });
    };

    view! {
        <Titlebar></Titlebar>
        <div class="panel flex-element flex-auto w-full center-elements justify-around">
            <div class="w-full flex-element center-elements">
                <div class="bg-black font-bold p-2 m-1 rounded-2xl">"Enter russian number"</div>
                <form class="flex-element flex-row center-elements" on:submit=format_input>
                    <span class="panel bg-black">"🇷🇺"</span>
                    <input type="text" placeholder="+7 (9xx) xxx xx-xx" class="bg-black text-center font-bold w-full p-1 border-2 border-green-600 rounded-xl" on:input=update_input />
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
