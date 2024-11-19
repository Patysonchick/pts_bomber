use crate::about::About;
use crate::footer::Footer;
use crate::titlebar::{Status, Titlebar};
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
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

#[derive(Serialize, Deserialize)]
struct EchoArgs<'a> {
    msg: &'a str,
}

#[derive(Serialize, Deserialize)]
struct FormatPhoneRuArgs<'a> {
    numbers: &'a str,
}

#[derive(Serialize, Deserialize)]
struct AttackArgs<'a> {
    phone: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (input_field, set_input_field) = create_signal(String::new());
    let (title, set_title) = create_signal(Status::IsIdling);
    let (logs, set_logs) = create_signal(String::from("Логи будут здесь..."));

    let update_input = move |ev| {
        let v = event_target_value(&ev);
        set_input_field.set(v);
    };

    let attack = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let input_field = input_field.get_untracked();

            let args = serde_wasm_bindgen::to_value(&FormatPhoneRuArgs {
                numbers: &input_field,
            })
            .unwrap();
            let formated_phone = invoke("format_phone_ru", args).await.as_string().unwrap();
            log(&formated_phone);

            if formated_phone != "0" && formated_phone != "1" {
                set_title.set(Status::Attacking);
                log("Attacking");
                let args = serde_wasm_bindgen::to_value(&AttackArgs {
                    phone: &formated_phone,
                })
                .unwrap();
                invoke("attack", args).await;
                set_title.set(Status::IsIdling);
                log("Ended");
            }
        });
    };

    view! {
        <Router>
            <Routes>
                <Route path="/" view=move || view! {
                    <Titlebar title=title/>
                    <div class="panel flex-element flex-auto w-full center-elements justify-around">
                        <div class="w-full flex-element center-elements">
                            <div class="bg-black font-bold p-2 m-1 rounded-2xl">"Введи российский номер"</div>
                            <form class="flex-element flex-row center-elements" on:submit=attack>
                                <span class="panel bg-black">"🇷🇺"</span>
                                <input type="text" placeholder="+7 (9xx) xxx xx-xx" class="bg-black text-center font-bold w-full p-1 border-2 border-green-600 rounded-xl" on:input=update_input/>
                                <button type="submit" class="button material-symbols-rounded">"send"</button>
                            </form>
                        </div>
                        <div class="h-full w-full flex-element center-elements">
                            <div class="bg-black font-bold p-2 m-1 rounded-2xl">"Логи"</div>
                            <textarea readonly class="panel bg-black h-full w-full">{ logs }</textarea>
                        </div>
                    </div>
                    <Footer/>
                }/>
                <Route path="/about" view=About/>
            </Routes>
        </Router>
    }
}
