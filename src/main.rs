// Copyright © 2018 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// CREDITS
// Project: https://github.com/actix/examples/tree/master/websocket-chat/
// Copyright (c) 2017 Nikolay Kim (fafhrd91@gmail.com)
// License (MIT) https://github.com/actix/actix-web/blob/master/LICENSE-MIT

#![allow(unused_variables)]
extern crate rand;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate actix;
extern crate actix_web;

use actix::*;
use actix_web::server::HttpServer;
use actix_web::{ws, App, Error, HttpRequest, HttpResponse};

mod server;

const SERVER_ADDRESS: &str = "0.0.0.0:8080";
// const SERVER_ADDRESS: &str = "127.0.0.1:8080";
const WS_PATH: &str = "/ws/";

/// This is our websocket route state, this state is shared with all route
/// instances via `HttpContext::state()`
struct WsChatSessionState {
    addr: Addr<Syn, server::ChatServer>,
}

/// Entry point for our route
fn chat_route(req: HttpRequest<WsChatSessionState>) -> Result<HttpResponse, Error> {
    ws::start(
        req,
        WsChatSession {
            id: 0,
            room: "Main".to_owned(),
        },
    )
}

struct WsChatSession {
    /// unique session id
    id: usize,
    /// joined room
    room: String,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self, WsChatSessionState>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr: Addr<Syn, _> = ctx.address();
        ctx.state()
            .addr
            .send(server::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ok(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        // notify chat server
        ctx.state().addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<server::SessionMessage> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: server::SessionMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<ws::Message, ws::ProtocolError> for WsChatSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        // println!("WEBSOCKET MESSAGE: {:?}", msg);
        match msg {
            ws::Message::Text(text) => {
                let m = text.trim();
                let msg = m.to_owned();
                // send message to chat server
                ctx.state().addr.do_send(server::Message {
                    id: self.id,
                    msg: msg,
                    room: self.room.clone(),
                })
            }
            _ => {
                ctx.stop();
            }
        }
    }
}

fn main() {
    let sys = actix::System::new("simple-rust-websocket");

    // Start chat server actor in separate thread
    let server: Addr<Syn, _> = Arbiter::start(|_| server::ChatServer::default());

    // Create Http server with websocket support
    HttpServer::new(move || {
        // Websocket sessions state
        let state = WsChatSessionState {
            addr: server.clone(),
        };

        // websocket
        App::with_state(state).resource(WS_PATH, |r| r.route().f(chat_route))
    }).bind(SERVER_ADDRESS)
        .unwrap()
        .start();

    println!("Started http server: {}", SERVER_ADDRESS);
    let _ = sys.run();
}
