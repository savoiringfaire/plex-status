use cfg_if::cfg_if;
use leptos::*;

// boilerplate to run in different modes
cfg_if! {
  if #[cfg(feature = "ssr")] {
    use axum::{
        debug_handler,
        Router,
        routing::get,
        extract::ws::{WebSocketUpgrade, WebSocket, Message},
                response::{Response, IntoResponse},
        extract::{Path, State, RawQuery},
        http::{Request, header::HeaderMap},
        body::Body as AxumBody,
    };
    use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
    use plex_status::fallback::file_and_error_handler;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use tokio::sync::watch;
    use uuid::Uuid;
    use futures_util::{StreamExt, SinkExt};
    use plex_status::*;
    use tokio_stream::wrappers::*;
    use leptos_server_signal::ServerSignal;
    use serde::{Deserialize, Serialize};
    use serde_aux::prelude::*;
    use tracing_subscriber;
    use tower_http::trace::TraceLayer;


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

    // TODO: Delete the two of these? believe they're not needed in the scenario and we can
    //       go back to the usual routes option.
    async fn server_fn_handler(State(app_state): State<AppState>, path: Path<String>, headers: HeaderMap, raw_query: RawQuery,
    request: Request<AxumBody>) -> impl IntoResponse {

        log!("{:?}", path);

        handle_server_fns_with_context(path, headers, raw_query, move |cx| {
        }, request).await
    }

    async fn leptos_routes_handler(State(app_state): State<AppState>, req: Request<AxumBody>) -> Response{
            let handler = leptos_axum::render_app_to_stream_with_context(app_state.leptos_options.clone(),
            move |cx| {
            },
            |cx| view! { cx, <App/> }
        );
        handler(req).await.into_response()
    }

    async fn update_data(tx: &mut watch::Sender<Count>, tautulli_url: &String) -> Result<(), ServerFnError> {
        let resp = reqwest::get(tautulli_url)
            .await?
            .json::<TautulliResponse<TautulliDataResponse<TautulliActivityData>>>()
            .await
            .map_err(|e| ServerFnError::from(e))
            .map(|r| r.response)?;


        let mut stats = Count {
            total_bandwidth: resp.data.wan_bandwidth,
            stream_count: resp.data.stream_count,
        };

        tx.send(stats);

        Ok(())
    }

    #[tokio::main]
    async fn main() {
        tracing_subscriber::fmt::init();

        let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_addr;
        let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

        let (mut tx, mut rx) = watch::channel(Count{ total_bandwidth: 0, stream_count: 10 });
        let app_state = AppState { tx: rx, leptos_options: leptos_options.clone() };

        // build our application with a route
        let app = Router::new()
        .route("/ws", get(websocket_handler))
        .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler) )
        .fallback(file_and_error_handler)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

        let tautulli_domain = std::env::var("TAUTULLI_DOMAIN").expect("TAUTULLI_DOMAIN must be set");
        let tautulli_api_key = std::env::var("TAUTULLI_API_KEY").expect("TAUTULLI_API_KEY must be set");
        let tautulli_url = format!("{}/api/v2?apikey={}&cmd=get_activity", tautulli_domain, tautulli_api_key);

        let broadcaster = tokio::spawn(async move {
            loop {
                update_data(&mut tx, &tautulli_url).await;
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        });

        // run our app with hyper
        // `axum::Server` is a re-export of `hyper::Server`
        let server = axum::Server::bind(&addr)
            .serve(app.into_make_service());

        tokio::join!(server, broadcaster);
    }

    async fn websocket_handler(
        ws: WebSocketUpgrade,
        State(state): State<AppState>
    ) -> impl IntoResponse {
        ws.on_upgrade(|socket| websocket(socket, state))
    }

    async fn websocket(mut socket: WebSocket, state: AppState) {
        let mut rx = state.tx.clone();
        let mut count = ServerSignal::<Count>::new("counter").unwrap();

        while rx.changed().await.is_ok() {
            let result = count.with(&mut socket, |count| {
                count.total_bandwidth = (17000 - rx.borrow().total_bandwidth) / 1000;
                count.stream_count = rx.borrow().stream_count;
            }).await;

            if result.is_err() {
                break;
            }
        }
    }
  }

    // client-only stuff for Trunk
    else {
        use plex_status::*;

        pub fn main() {
            mount_to_body(|cx| {
                view! { cx, <App/> }
            });
        }
    }
}
