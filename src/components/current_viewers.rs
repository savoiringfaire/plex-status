use serde_aux::prelude::*;

use crate::Count;
use leptos::*;
use leptos_server_signal::create_server_signal;
use serde::{Deserialize, Serialize};

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
                <div class="bg-stone-600 text-stone-300 w-64 flex flex-col items-center text-center p-5 rounded-md gap-5">
                    {move || if count().total_bandwidth > 7 {
                        view!{cx,
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                fill="currentColor"
                                class="w-16 h-16"
                                class=("text-emerald-600", move || count().total_bandwidth > 12)
                                class=("text-orange-600", move || count().total_bandwidth <= 12 && count().total_bandwidth > 7)
                            >
                              <path fill-rule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm13.36-1.814a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd" />
                            </svg>
                        }
                    } else {
                        view! {cx,
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                fill="currentColor"
                                class="w-16 h-16 text-red-600"
                            >
                                  <path fill-rule="evenodd" d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25zm-1.72 6.97a.75.75 0 10-1.06 1.06L10.94 12l-1.72 1.72a.75.75 0 101.06 1.06L12 13.06l1.72 1.72a.75.75 0 101.06-1.06L13.06 12l1.72-1.72a.75.75 0 10-1.06-1.06L12 10.94l-1.72-1.72z" clip-rule="evenodd" />
                            </svg>
                        }
                    }}

                    <p class="w-full">"Stream Count: " {move || count().stream_count.to_string()}</p>
                    <p class="w-full">"Available Bandwidth: " {move || count().total_bandwidth.to_string()} "Mbps"</p>
                </div>
            </div>
        </>
    }
}
