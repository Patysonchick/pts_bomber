use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::rt::Event;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct EchoArgs<'a> {
    msg: &'a str,
}

#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    let input = create_signal(cx, String::new());

    let output = move |e: Event| {
        e.prevent_default();
        spawn_local_scoped(cx, async move {
            let test = input.get();
            log(format!("Test: {}", test).as_str());
            invoke("echo", to_value(&EchoArgs { msg: &test }).unwrap()).await;
        })
    };

    view! { cx,
        div(data-tauri-drag-region=true, class="flex-element flex-row w-full justify-between") {
            div(data-tauri-drag-region=true, class="flex-element flex-row") {
                span(data-tauri-drag-region=true, class="panel flex-element font-bold") { "pts_bomber" }
                span(class="panel flex-element font-light text") { em { "v2.0.0" } }
            }
            span(data-tauri-drag-region=true, class="panel w-full font-bold") { "Is idle" }
            div(class="panel material-symbols-rounded flex-element flex-row") {
                button { "minimize" } button { "close" }
            }
        }
        div(class="panel flex-element flex-auto w-full center-elements justify-around") {
            div(class="w-full flex-element center-elements") {
                div(class="bg-black font-bold p-2 m-1 rounded-2xl") { "Enter russian number" }
                form(class="flex-element flex-row center-elements", on:submit=output) {
                    span(class="panel bg-black"){ "🇷🇺" }
                    input(type="text", placeholder="+7 (9xx) xxx xx-xx", class="bg-black text-center font-bold w-full p-1 border-2 border-green-600 rounded-xl", bind:value=input)
                    button(type="submit", class="button material-symbols-rounded") { "send" }
                }
            }
            div(class="w-full flex-element center-elements") {
                div(class="bg-black font-bold p-2 m-1 rounded-2xl") { "Logs" }
                textarea(readonly=true, class="panel bg-black w-full") { "Logs will be here" }
            }
        }
        div(class="flex-element flex-row w-full justify-around") {
            a(href="https://github.com/Patysonchick/pts_bomber", class="panel flex-element rounded-b-none") {
                img(src="public/github-mark.svg", alt="GitHub", class="h-7")
            }
            a(href="https://t.me/pts_bomber", class="panel flex-element rounded-b-none") {
                img(src="public/telegram.svg", alt="Telegram", class="h-7")
            }
            button(class="panel flex-element rounded-b-none") { span(class="material-symbols-rounded") { "settings" } }
        }
    }
}
