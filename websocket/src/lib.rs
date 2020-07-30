mod jsonrpc;
mod server;

use serde_json::{json, Value};
use std::fmt;

use actix::prelude::*;
use ewf_core::error::Error as EwfError;
use ewf_core::{Bus, Call, CallQuery, Event, Module, StartNotify};
use wallet_common::prepare::{ModStatusPullParam, ModStatus};

use crate::server::WSServer;

pub struct WebSocketModule {
    bind_transport: String,
    bus_addr: Option<Addr<Bus>>,
}

impl WebSocketModule {
    pub fn new(bind_transport: String) -> Self {
        Self {
            bind_transport,
            bus_addr: None,
        }
    }
}

impl Actor for WebSocketModule {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {}
}

impl Handler<Call> for WebSocketModule {
    type Result = ResponseFuture<Result<Value, EwfError>>;
    fn handle(&mut self, _msg: Call, _ctx: &mut Context<Self>) -> Self::Result {
        let bus_addr = self.bus_addr.clone().unwrap();
        Box::pin(async move {
            let mut split_iter = _msg.method.split(|c| c == '.');
            let mod_name = match split_iter.next() {
                Some(mod_name) => mod_name.to_string(),
                None => return Err(EwfError::MethodNotFoundError),
            };
            let method = match split_iter.next() {
                Some(method) => method.to_string(),
                None => return Err(EwfError::MethodNotFoundError),
            };

            let module = bus_addr.send(CallQuery { module: mod_name }).await??;

            module
                .send(Call {
                    method: method,
                    args: _msg.args,
                })
                .await?
        })
    }
}

impl Handler<Event> for WebSocketModule {
    type Result = ResponseFuture<Result<(), EwfError>>;
    fn handle(&mut self, _msg: Event, _ctx: &mut Context<Self>) -> Self::Result {
        let bind_transport = self.bind_transport.clone();
        let bus_addr = self.bus_addr.clone().unwrap();
        let mod_name = self.name();
        let self_addr = _ctx.address();

        Box::pin(async move {
            let event: &str = &_msg.event;
            match event {
                "Start" => {
                    actix::spawn(async move {
                        match WSServer::bind(bind_transport).await {
                            Ok(ws_server) => ws_server.listen_loop(self_addr).await,
                            Err(err) => panic!("ws_server bind error, {:?}", err),
                        }
                    });

                    let prepare = bus_addr
                        .send(CallQuery {
                            module: "prepare".to_string(),
                        })
                        .await??;
                    prepare
                        .send(Call {
                            method: "inital".to_string(),
                            args: json!(ModStatusPullParam { mod_name, is_prepare: ModStatus::InitalSuccess }),
                        })
                        .await??;
                }
                // no care this event, ignore
                _ => return Ok(()),
            }

            Ok(())
        })
    }
}

impl Handler<StartNotify> for WebSocketModule {
    type Result = ();
    fn handle(&mut self, _msg: StartNotify, _ctx: &mut Context<Self>) -> Self::Result {
        self.bus_addr = Some(_msg.addr);
    }
}

impl fmt::Debug for WebSocketModule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "{{ {} {} {} }}",
            self.name(),
            self.version(),
            self.bind_transport
        ))
    }
}

impl Module for WebSocketModule {
    fn name(&self) -> String {
        "webscoket_jsonrpc".to_string()
    }

    fn version(&self) -> String {
        "0.1".to_string()
    }
}
