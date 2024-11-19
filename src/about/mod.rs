use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="panel flex-element flex-auto justify-around"><img src="../public/icon.svg" alt="Icon" class="w-24 h-24"/></div>
        <div class="panel flex-element flex-auto justify-around bg-red-700 font-bold">Данные проект несёт исключительно образовательней цели и вообще создаётся как портфолио.</div>
        <div class="panel flex-element flex-auto justify-around">Первый SMS/звонки бомбер написанный на Rust</div>
    }
}
