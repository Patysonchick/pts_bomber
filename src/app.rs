use crate::about::About;
use crate::footer::Footer;
use crate::titlebar::{Status, Titlebar};
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
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
    cycles: u64,
}

#[derive(Serialize, Deserialize)]
struct ShowDialogErrorArgs<'a> {
    e: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (input_field, set_input_field) = signal(String::new());
    let (cycles, set_cycles) = signal(1u64);
    let (title, set_title) = signal(Status::IsIdling);

    let attack = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if title.get_untracked() != Status::Attacking {
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
                        cycles: cycles.get_untracked(),
                    })
                    .unwrap();
                    invoke("attack", args).await;

                    set_title.set(Status::IsIdling);
                    log("Ended");
                } else {
                    let args =
                        serde_wasm_bindgen::to_value(&ShowDialogErrorArgs { e: &formated_phone })
                            .unwrap();
                    invoke("show_dialog_error", args).await;
                }
            } else {
                log("Tried attack while attacking");
                let args = serde_wasm_bindgen::to_value(&ShowDialogErrorArgs { e: "2" }).unwrap();
                invoke("show_dialog_error", args).await;
            }
        });
    };

    let stop_attack = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            invoke_without_args("stop_attack").await;
        });
    };

    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route
                    path=path!("/")
                    view=move || {
                        view! {
                            <Titlebar title=title />
                            <div class="panel flex-element w-full h-full center-elements">
                                <div class="flex-element w-full center-elements">
                                    <div class="bg-black p-2 m-1 rounded-2xl">
                                        "Введите номер"
                                    </div>
                                    <form
                                        class="flex-element flex-row center-elements"
                                        on:submit=attack
                                    >
                                        <span class="panel bg-black">"🇷🇺"</span>
                                        <input
                                            type="tel"
                                            placeholder="+7 (9xx) xxx xx-xx"
                                            class="w-full button p-1 border-2 border-green-600 rounded-xl"
                                            on:input=move |ev| {
                                                let v = event_target_value(&ev);
                                                set_input_field.set(v);
                                            }
                                        />
                                        <input
                                            type="number"
                                            value="1"
                                            min="1"
                                            step="1"
                                            class="button w-14"
                                            on:input=move |ev| {
                                                let v = event_target_value(&ev);
                                                set_cycles.set(v.parse().unwrap());
                                            }
                                        />
                                        <button
                                            type="submit"
                                            class="button material-symbols-rounded"
                                            class:bg-neutral-800=move || title.get() == Status::IsIdling
                                            class:bg-red-700=move || title.get() == Status::Attacking
                                            // on:click=move |_| if title.get() == Status::Attacking { stop_attack; }
                                        >
                                            {move || match title.get() {
                                                Status::IsIdling => "send",
                                                Status::Attacking => "block",
                                            }}
                                        </button>
                                    </form>
                                </div>
                            </div>
                            <Footer />
                        }
                    }
                />
                <Route path=path!("/about") view=About />
            </Routes>
        </Router>
    }
}
