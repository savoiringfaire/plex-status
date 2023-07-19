use cfg_if::cfg_if;
use leptos::{component, view, IntoView, Scope, AdditionalAttributes};
use leptos_meta::{Stylesheet, Body};

pub mod components;
pub mod error_template;
pub mod fallback;

use components::current_viewers::CurrentViewers;

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize, Copy)]
pub struct Count {
    pub total_bandwidth: u32,
    pub stream_count: u32,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::{
            Router,
            routing::get,
            extract::State,
            extract::ws::{WebSocketUpgrade, WebSocket, Message},
            response::{IntoResponse}
        };
        use tokio::sync::watch;
        use axum::extract::FromRef;

        #[derive(Clone, FromRef)]
        pub struct AppState {
            pub tx: watch::Receiver<Count>, 
            pub leptos_options: LeptosOptions,
        }
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    leptos_server_signal::provide_websocket(cx, "wss://plex.hhra.uk/ws").unwrap();

    view! {
        cx,
        <>
            <Body attributes=AdditionalAttributes::from([("class", "bg-stone-700")])/>
            <Stylesheet id="leptos" href="/pkg/plex_status.css"/>
            <CurrentViewers />
        </>
    }
}

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            leptos::mount_to_body(move |cx| {
                view! { cx, <App/> }
            });
        }
    }
}
