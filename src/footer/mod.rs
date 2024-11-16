use leptos::*;
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
            <button class="panel flex-element rounded-b-none">
                <a href="http://127.0.0.1:3229/about">
                    <span class="material-symbols-rounded">"settings"</span>
                </a>
            </button>
        </div>
    }
}
