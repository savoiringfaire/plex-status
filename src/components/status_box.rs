use leptos::{Scope, IntoView, view, component, Children};
use leptos_heroicons::size_24::solid::CheckCircle;

#[component]
pub fn StatusBox(
    cx: Scope,
    children: Children
) -> impl IntoView
{
    view! {
        cx,
        <div class="bg-stone-600 text-stone-300 w-64 flex flex-col items-center text-center p-5 rounded-md gap-5">
            {children(cx)}
        </div>
    }
}
