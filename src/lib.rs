use cfg_if::cfg_if;
use leptos::{component, view, IntoView, Scope, AdditionalAttributes, create_signal};
use leptos_meta::{Stylesheet, Body, provide_meta_context};

pub mod components;
pub mod error_template;
pub mod fallback;

use components::current_viewers::CurrentViewers;
use components::link::Link;

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
        use leptos::LeptosOptions;

        #[derive(Clone, FromRef)]
        pub struct AppState {
            pub tx: watch::Receiver<Count>, 
            pub leptos_options: LeptosOptions,
        }
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    leptos_server_signal::provide_websocket(cx, "ws://localhost:3000/ws").unwrap();
    provide_meta_context(cx);

    let (name, _) = create_signal(cx, "Make a Request".to_string());
    let name: String = "test".into();

    view! {
        cx,
        <>
            <Body attributes=AdditionalAttributes::from([("class", "bg-stone-700")])/>
            <Stylesheet id="leptos" href="/pkg/plex_status.css"/>
            <div class="grid h-screen place-items-center">
                <div class="flex flex-col items-center text-center gap-5">
                    <CurrentViewers />
                    <div class="flex gap-5">
                        <Link title="Make a Request".into() link="https://requests.hhra.uk".into() />
                        <Link title="Head To Plex".into() link="https://plex.tv/web".into() />
                    </div>
                </div>
            </div>
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
