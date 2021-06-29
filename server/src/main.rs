//! Implementation of the WebSocket server which broadcasts messages from the
//! one client to the many other connected clients.

mod session;
mod sessions_store;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_web::{
    web::{resource, Data, Path, Payload},
    App, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::{ws, ws::WebsocketContext};

use crate::sessions_store::SessionsStore;

use self::session::WsSession;

/// Context of the WebSocket [`HttpServer`].
///
/// Stores [`SessionsStore`]s for the all users of this server.
#[derive(Debug, Clone)]
struct Context(Arc<Mutex<HashMap<String, SessionsStore>>>);

impl Context {
    /// Returns new empty [`Context`].
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }

    /// Returns [`SessionsStore`] for the user with a provided ID.
    pub fn get_sessions_store(&self, id: String) -> SessionsStore {
        let mut store = self.0.lock().unwrap();

        // Borrow checker doesn't allows mutation inside 'map_or_else'.
        #[allow(clippy::option_if_let_else)]
        if let Some(sessions) = store.get(&id) {
            sessions.clone()
        } else {
            let sessions = SessionsStore::new();
            store.insert(id, sessions.clone());
            sessions
        }
    }
}

/// WebSocket index handler.
///
/// Starts new [`WsSession`] with a [`SessionsStore`] of the user with a
/// provided ID.
async fn ws_index(
    request: HttpRequest,
    id: Path<String>,
    state: Data<Context>,
    payload: Payload,
) -> actix_web::Result<HttpResponse> {
    let ws = WsSession::new(state.get_sessions_store(id.into_inner()));
    Ok(
        ws::handshake(&request)?.streaming(WebsocketContext::with_codec(
            ws,
            payload,
            actix_http::ws::Codec::new(),
        )),
    )
}

/// Runs WebSocket [`HttpServer`].
async fn run() {
    let ctx = Context::new();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(ctx.clone()))
            .configure(|cfg| {
                cfg.service(
                    resource("/{id}").route(actix_web::web::get().to(ws_index)),
                );
            })
    })
    .bind("0.0.0.0:8000")
    .unwrap()
    .run()
    .await
    .unwrap();
}

#[actix::main]
async fn main() {
    run().await;
}
