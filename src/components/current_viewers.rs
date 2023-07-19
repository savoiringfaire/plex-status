use serde_aux::prelude::*;

use crate::{Count, components::status_box::StatusBox};
use leptos::*;
use leptos_server_signal::create_server_signal;
use serde::{Deserialize, Serialize};
use leptos_heroicons::size_24::solid::{CheckCircle, XCircle};

#[derive(Serialize, Deserialize, Clone)]
pub struct TautulliResponse<T> {
    response: T,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TautulliDataResponse<T> {
    data: T,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TautulliActivityData {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    stream_count: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    wan_bandwidth: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TautulliActivityStreamData {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    bandwidth: u32,
    location: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Stats {
    total_bandwidth: u32,
    stream_count: u32,
}

#[component]
pub fn CurrentViewers(cx: Scope) -> impl IntoView {
    let count = create_server_signal::<Count>(cx, "counter");

    view! {
        cx,
        <>
        <div class="grid h-screen place-items-center">
             <StatusBox>
                 { move || if count().total_bandwidth > 7 && count().total_bandwidth <= 12 {
                     view!{cx, <CheckCircle class="w-16 h-16 text-orange-600" /> }
                 } else if count().total_bandwidth > 12 {
                     view!{cx, <CheckCircle class="w-16 h-16 text-emerald-600" /> }
                 } else {
                     view! {cx,
                         <XCircle class="w-16 h-16 text-red-600" /> }
                 }}
                 <p class="w-full">"Stream Count: " {move || count().stream_count.to_string()}</p>
                 <p class="w-full">"Available Bandwidth: " {move || count().total_bandwidth.to_string()} "Mbps"</p>
            </StatusBox>
        </div>
        </>
    }
}
