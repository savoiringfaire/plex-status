use crate::{Count, components::status_box::StatusBox};
use leptos::*;
use leptos_server_signal::create_server_signal;

#[component]
pub fn Link(
    cx: Scope,
    title: String,
    link: String,
) -> impl IntoView {
    view! {
        cx,
        <a href={link}>
            <div class="bg-stone-600 hover:bg-stone-500 text-stone-300 w-64 flex items-center text-center p-5 rounded-md gap-5 justify-center">
                <p>{title}</p>
            </div>
        </a>
    }
}
