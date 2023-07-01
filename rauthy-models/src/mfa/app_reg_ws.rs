use crate::app_state::AppState;
use crate::entity::mfa_app::{MfaApp, MfaAppReg};
use crate::response::MfaAppInitResponse;
use actix::ActorFutureExt;
use actix::{
    fut, Actor, ActorContext, AsyncContext, ContextFutureSpawner, Message, StreamHandler,
    WrapFuture,
};
use actix_web::web;
use actix_web_actors::ws;
use actix_web_actors::ws::{CloseCode, CloseReason};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use ring::digest;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tracing::{debug, error, info};

// TODO adjust before production
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(60);

// TODO adjust before production
const CLIENT_TIMEOUT: Duration = Duration::from_secs(90);

#[derive(Debug)]
pub enum WsRegRouteReq {
    New {
        email: String,
        addr: actix::Addr<RegWebSocket>,
    },
    Msg(String),
}

pub async fn ws_reg_router(mut rx: mpsc::Receiver<WsRegRouteReq>) {
    let mut clients = HashMap::new();

    loop {
        let res = rx.recv().await;
        if res.is_none() {
            debug!("Received 'None' in 'ws_reg_router'' - exiting");
            break;
        }

        match res.unwrap() {
            WsRegRouteReq::New { email, addr } => {
                debug!("New client registered in 'ws_reg_router': {}", &email);
                clients.insert(email, addr);
            }
            WsRegRouteReq::Msg(m) => {
                debug!("Msg in handle_reg_ws_brd: {}", &m);
                for (c, addr) in &clients {
                    // let r =  addr.send(BrdMsg { msg: m.clone() });
                    match addr.try_send(WsRegMsg { msg: m.clone() }) {
                        Ok(_) => {
                            debug!("Sent msg in 'ws_reg_router' to client '{}'", c);
                        }
                        Err(e) => {
                            error!(
                                "Error sending msg in 'ws_reg_router' to client '{}': {:?}",
                                c, e
                            );
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct InitRequest {
    pub app_id: String,
    pub challenge: String,
}

#[derive(Debug, Deserialize)]
struct VerifyRequest {
    pub verifier: String,
    pub secret: String,
}

#[derive(Debug, PartialEq)]
enum State {
    Open,
    Challenge,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsRegMsg {
    pub msg: String,
}

pub struct RegWebSocket {
    app_id: Option<String>,
    data: web::Data<AppState>,
    email: String,
    heartbeat: Instant,
    mfa_req: MfaAppReg,
    state: State,
}

impl RegWebSocket {
    pub fn new(data: web::Data<AppState>, email: String, mfa_req: MfaAppReg) -> Self {
        Self {
            app_id: None,
            data,
            email,
            heartbeat: Instant::now(),
            mfa_req,
            state: State::Open,
        }
    }

    fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.heartbeat) > CLIENT_TIMEOUT {
                info!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for RegWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!(
            "Started new WebSocket session with client '{:?}'",
            self.email
        );
        self.heartbeat(ctx);

        let addr = ctx.address();
        let email = self.email.clone();
        let tx = self.data.tx_ws_brd.clone();
        async move {
            tx.send(WsRegRouteReq::New { email, addr })
                .await
                .expect("register WS client")
        }
        .into_actor(self)
        .wait(ctx)
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        debug!("WebSocket session with client '{:?}' has ended", self.email);
    }
}

impl actix::Handler<WsRegMsg> for RegWebSocket {
    type Result = ();

    fn handle(&mut self, msg: WsRegMsg, ctx: &mut Self::Context) -> Self::Result {
        debug!("In BrdMsg Handler");
        ctx.text(msg.msg)
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for RegWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heartbeat = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                debug!("Received 'text': {:?}", text);
                let (cmd, payload) = text.split_once(' ').unwrap_or(("", ""));
                match cmd {
                    "REG" => {
                        // only accept /init when in state Open
                        if self.state != State::Open {
                            let mut reason = CloseReason::from(CloseCode::Invalid);
                            reason.description = Some(String::from("Session state != 'Open'"));
                            error!("WS close: {:?}", reason);
                            ctx.close(Some(reason));
                        }

                        // deserialize the payload
                        let res = serde_json::from_str::<InitRequest>(payload);
                        if res.is_err() {
                            let mut reason = CloseReason::from(CloseCode::Invalid);
                            reason.description =
                                Some(String::from("Cannot deserialize the request"));
                            error!("WS close: {:?}", reason);
                            ctx.close(Some(reason));
                        }

                        // handle the request
                        let email = self.email.clone();
                        let ws_req = res.unwrap();
                        let app_id = ws_req.app_id.clone();
                        let mfa_req = self.mfa_req.clone();
                        handle_init(email, mfa_req, ws_req)
                            .into_actor(self)
                            .then(|res, act, ctx| {
                                match res {
                                    Ok(resp) => {
                                        let ser = serde_json::to_string(&resp);
                                        if ser.is_err() {
                                            error!("Error serializing to JSON: {:?}", ser);
                                            ctx.close(None);
                                        }

                                        act.app_id = Some(app_id);
                                        act.state = State::Challenge;
                                        let resp = format!("CHA {}", ser.unwrap());
                                        ctx.text(resp);
                                    }
                                    Err(e) => {
                                        let mut reason = CloseReason::from(CloseCode::Invalid);
                                        reason.description = Some(e.message);
                                        error!("WS close: {:?}", reason);
                                        ctx.close(Some(reason));
                                    }
                                }

                                fut::ready(())
                            })
                            .wait(ctx);
                    }
                    "VFY" => {
                        // only accept /verify when in state Challenge
                        if self.state != State::Challenge {
                            let mut reason = CloseReason::from(CloseCode::Invalid);
                            reason.description = Some(String::from("Session state != 'Challenge'"));
                            error!("WS close: {:?}", reason);
                            ctx.close(Some(reason));
                        }

                        // deserialize the payload
                        let res = serde_json::from_str::<VerifyRequest>(payload);
                        if res.is_err() {
                            let mut reason = CloseReason::from(CloseCode::Invalid);
                            reason.description =
                                Some(String::from("Cannot deserialize the request"));
                            error!("WS close: {:?}", reason);
                            ctx.close(Some(reason));
                        }

                        // handle the request
                        let app_id = self.app_id.as_ref().unwrap().to_owned();
                        let email = self.email.clone();
                        let data = self.data.clone();
                        let mfa_req = self.mfa_req.clone();
                        handle_verify(app_id, email, data, mfa_req, res.unwrap())
                            .into_actor(self)
                            .then(|res, _act, ctx| {
                                match res {
                                    Ok(resp) => {
                                        let ser = serde_json::to_string(&resp);
                                        if ser.is_err() {
                                            error!("Error serializing to JSON: {:?}", ser);
                                            ctx.close(None);
                                        }
                                        let mut reason = CloseReason::from(CloseCode::Normal);
                                        reason.description = Some(String::from("accepted"));
                                        ctx.close(Some(reason));
                                    }
                                    Err(e) => {
                                        let mut reason = CloseReason::from(CloseCode::Invalid);
                                        reason.description = Some(e.message);
                                        error!("WS close: {:?}", reason);
                                        ctx.close(Some(reason));
                                    }
                                }

                                fut::ready(())
                            })
                            .wait(ctx);
                    }
                    _ => {
                        error!("Invalid WebSocket request");
                        ctx.close(Some(CloseReason::from(CloseCode::Invalid)));
                    }
                }
            }
            // Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
            }
            _ => ctx.stop(),
        }
    }
}

async fn handle_init(
    email: String,
    mfa_req: MfaAppReg,
    req: InitRequest,
) -> Result<MfaAppInitResponse, ErrorResponse> {
    // check expired
    if chrono::Local::now().naive_local().timestamp() > mfa_req.exp.timestamp() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("The registration request has expired"),
        ));
    }

    // check email
    if email != mfa_req.email {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("The E-Mail does not match the request"),
        ));
    }

    // compute the challenge verifier
    let s = req.challenge + &mfa_req.code;
    let s256 = digest::digest(&digest::SHA256, s.as_bytes());
    let verifier = hex::encode(s256);

    let res = MfaAppInitResponse {
        challenge: mfa_req.challenge,
        verifier,
    };
    Ok(res)
}

async fn handle_verify(
    app_id: String,
    email: String,
    data: web::Data<AppState>,
    mfa_req: MfaAppReg,
    req: VerifyRequest,
) -> Result<(), ErrorResponse> {
    // check expired
    if chrono::Local::now().naive_local().timestamp() > mfa_req.exp.timestamp() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("The registration request has expired"),
        ));
    }

    // check secret length
    if req.secret.len() < 24 {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("The secret must be at least 24 characters long"),
        ));
    }

    // compute the challenge verifier
    let s = mfa_req.challenge + &mfa_req.code;
    let s256 = digest::digest(&digest::SHA256, s.as_bytes());
    let verifier_res = hex::decode(req.verifier);
    if verifier_res.is_err() {
        let err = String::from("Error decoding verifier from MFA App");
        error!("{}", err);
        return Err(ErrorResponse::new(ErrorResponseType::BadRequest, err));
    }
    let verifier = verifier_res.as_ref().unwrap().as_slice();

    // check challenge verifier
    if s256.as_ref() != verifier {
        let err = String::from("Could not verify the MFA App request challenge");
        error!("{}", err);
        debug!("Local s256 hash: {:?}", s256);
        debug!("MFA App verifier: {:?}", verifier_res);
        return Err(ErrorResponse::new(ErrorResponseType::BadRequest, err));
    }

    // all good
    let mfa_app = MfaApp {
        app_id,
        email,
        secret: req.secret,
    };
    MfaApp::create(&data, mfa_app).await?;

    Ok(())
}
