mod about;
mod app;
mod footer;
mod titlebar;

use app::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
