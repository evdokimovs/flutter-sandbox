use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_web::{
    web::{resource, Data, Path, Payload},
    App, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::{ws, ws::WebsocketContext};
use bytestring::ByteString;

#[derive(Debug, Clone)]
struct Context {
    store: Arc<Mutex<HashMap<String, WsSessions>>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_sessions_for_id(&self, id: String) -> WsSessions {
        let mut store = self.store.lock().unwrap();
        if let Some(sessions) = store.get(&id) {
            sessions.clone()
        } else {
            let sessions = WsSessions::new();
            store.insert(id, sessions.clone());
            sessions
        }
    }
}

#[derive(Debug)]
struct WsBroadcaster {
    sessions: WsSessions,
    id: u32,
}

impl WsBroadcaster {
    pub fn new(sessions: WsSessions, addr: Addr<WsSession>) -> Self {
        let id = sessions.add_session(addr);
        Self { sessions, id }
    }

    pub fn text(&self, text: ByteString) {
        for session in self.sessions.get_sessions_except_id(self.id) {
            session.do_send(SendText(text.clone()));
        }
    }
}

impl Drop for WsBroadcaster {
    fn drop(&mut self) {
        self.sessions.remove(self.id);
    }
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct SendText(pub ByteString);

impl Handler<SendText> for WsSession {
    type Result = ();

    fn handle(
        &mut self,
        msg: SendText,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        ctx.text(msg.0);
    }
}

#[derive(Debug)]
struct Inner {
    store: HashMap<u32, Addr<WsSession>>,
    last_id: u32,
}

#[derive(Clone, Debug)]
struct WsSessions(Arc<Mutex<Inner>>);

impl WsSessions {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(Inner {
            store: HashMap::new(),
            last_id: 0,
        })))
    }

    pub fn add_session(&self, session: Addr<WsSession>) -> u32 {
        let mut inner = self.0.lock().unwrap();
        let id = inner.last_id;
        inner.last_id += 1;
        inner.store.insert(id, session);

        id
    }

    pub fn get_sessions_except_id(
        &self,
        excluded_id: u32,
    ) -> Vec<Addr<WsSession>> {
        self.0
            .lock()
            .unwrap()
            .store
            .iter()
            .filter_map(|(id, session)| {
                if *id != excluded_id {
                    Some(session.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn remove(&self, id: u32) {
        self.0.lock().unwrap().store.remove(&id);
    }
}

#[derive(Debug)]
struct WsSession {
    store: WsSessions,
    broadcaster: Option<WsBroadcaster>,
}

impl WsSession {
    pub fn new(store: WsSessions) -> Self {
        Self {
            store,
            broadcaster: None,
        }
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.broadcaster =
            Some(WsBroadcaster::new(self.store.clone(), ctx.address()));
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(msg) => match msg {
                ws::Message::Text(text) => {
                    self.broadcaster.as_ref().unwrap().text(text);
                }
                ws::Message::Ping(ping) => ctx.pong(&*ping),
                _ => {
                    println!("Received message of unknown type");
                }
            },
            Err(err) => {
                println!("StreamHandler error: {:?}", err);
            }
        }
    }
}

async fn ws_index(
    request: HttpRequest,
    id: Path<String>,
    state: Data<Context>,
    payload: Payload,
) -> actix_web::Result<HttpResponse> {
    let ws = WsSession::new(state.get_sessions_for_id(id.into_inner()));
    Ok(
        ws::handshake(&request)?.streaming(WebsocketContext::with_codec(
            ws,
            payload,
            actix_http::ws::Codec::new(),
        )),
    )
}

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
