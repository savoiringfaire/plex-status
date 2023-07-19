

use crate::{Count, components::status_box::StatusBox};
use leptos::*;
use leptos_server_signal::create_server_signal;

use leptos_heroicons::size_24::solid::{CheckCircle, XCircle};

#[component]
pub fn CurrentViewers(cx: Scope) -> impl IntoView {
    let count = create_server_signal::<Count>(cx, "counter");

    view! {
        cx,
        <div class="grid h-screen place-items-center">
             <StatusBox>
                 { move || if count().total_bandwidth > 7 && count().total_bandwidth <= 12 {
                     view!{cx, <CheckCircle class="w-16 h-16 text-orange-600" /> }
                 } else if count().total_bandwidth > 12 {
                     view!{cx, <CheckCircle class="w-16 h-16 text-emerald-600" /> }
                 } else {
                     view!{cx, <XCircle class="w-16 h-16 text-red-600" /> }
                 }}
                 <p>"Stream Count: " {move || count().stream_count.to_string()}</p>
                 <p>"Available Bandwidth: " {move || count().total_bandwidth.to_string()} "Mbps"</p>
            </StatusBox>
        </div>
    }
}
