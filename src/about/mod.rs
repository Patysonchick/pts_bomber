use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="panel flex-element"><img src="../public/icon.svg" alt="Icon" class="w-24 h-24"/></div>
        <div class="panel flex-element justify-around bg-red-700 font-bold">Данные проект несёт исключительно образовательней цели и вообще создаётся как портфолио.</div>
        <div class="panel flex-element justify-around">Первый SMS/звонки бомбер написанный на Rust</div>
        <div class="panel flex-element justify-around">Используемые технологии</div>
        <ul class="panel flex-element flex-auto justify-around">
            <li>Rust</li>
            <li>Tauri</li>
            <li>Leptos</li>
            <li>Git</li>
            <li>GitHub</li>
            <li>GitHub Actions</li>
            <li>Arch Linux</li>
        </ul>
    }
}
