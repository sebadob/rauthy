use crate::app_state::AppState;
use crate::entity::mfa_app::MfaApp;
use crate::mfa::{get_mfa_login_req, put_mfa_login_req};
use crate::response::{MfaAckResponse, MfaLoginRequest, MfaLoginRequestState};
use actix::{
    fut, Actor, ActorContext, ActorFutureExt, AsyncContext, ContextFutureSpawner, Message,
    StreamHandler, WrapFuture,
};
use actix_web::web;
use actix_web_actors::ws;
use actix_web_actors::ws::{CloseCode, CloseReason};
use actix_web_lab::sse;
use cached::Cached;
use rauthy_common::constants::MFA_REQ_LIFETIME;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::get_rand;
use ring::digest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, error, info, warn};

// TODO adjust before production
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(60);
// TODO adjust before production
const CLIENT_TIMEOUT: Duration = Duration::from_secs(65);

#[derive(Debug)]
pub enum WsListenRouteReq {
    New {
        app_id: String,
        addr: actix::Addr<MfaListenWs>,
    },
    Req {
        app_id: String,
        msg: WsListenMsg,
    },
    SseTx {
        app_id: String,
        tx: WsListenTx,
    },
    TrySend {
        app_id: String,
        msg: String,
        tx_ack: oneshot::Sender<bool>,
    },
}

#[derive(Debug, Clone, Message, Serialize)]
#[rtype(result = "()")]
pub struct WsListenMsg {
    pub challenge: String,
    pub exp: i64,
    pub req_id: String,
    pub accepted: Option<bool>,
}

impl WsListenMsg {
    pub fn new(challenge: String, exp: i64, req_id: String) -> Self {
        Self {
            challenge,
            exp,
            req_id,
            accepted: None,
        }
    }
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct WsListenTx {
    pub req_id: String,
    pub tx: sse::Sender,
    pub loc: String,
}

pub async fn ws_listen_router(mut rx: mpsc::Receiver<WsListenRouteReq>) {
    let mut clients = HashMap::new();
    let mut sse_clients: cached::TimedCache<String, WsListenTx> =
        cached::TimedCache::with_lifespan(*MFA_REQ_LIFETIME);

    let send_sse_tx = |addr: &actix::Addr<MfaListenWs>, tx: WsListenTx| match addr.try_send(tx) {
        Ok(_) => {
            debug!("WsListenRouteReq::SseTx request sent to actor");
        }
        Err(e) => {
            debug!("Error sending WsListenRouteReq::SseTx to actor: {}", e);
        }
    };

    // handle incoming requests
    loop {
        let res = rx.recv().await;
        if res.is_none() {
            debug!("Received 'None' in handle_reg_ws_brd - exiting");
            break;
        }

        match res.unwrap() {
            // registers a new client
            WsListenRouteReq::New { app_id, addr } => {
                debug!("New client registered in 'ws_listen_router': {}", &app_id);

                // look for an already registered sse client
                if let Some(tx) = sse_clients.cache_get_mut(&app_id) {
                    send_sse_tx(&addr, tx.clone());
                    sse_clients.cache_remove(&app_id);
                }

                clients.insert(app_id, addr);
            }

            // sends out a new mfa verification request to a registered app
            WsListenRouteReq::Req { app_id, msg } => {
                match clients.get(&app_id) {
                    None => {
                        info!("WsListenRouteReq::Req could not be sent - client not connected");
                    }

                    Some(addr) => match addr.try_send(msg) {
                        Ok(_) => {
                            debug!("WsListenRouteReq::Req request sent to actor");
                        }
                        Err(e) => {
                            error!("Error sending WsListenRouteReq::Req to actor: {}", e);
                        }
                    },
                };
            }

            // send the SSE tx to a connected WebSocket client, cache it otherwise
            WsListenRouteReq::SseTx { app_id, tx } => {
                match clients.get(&app_id) {
                    None => {
                        info!("WsListenRouteReq::SseTx could not be sent - client not connected");
                        sse_clients.cache_set(app_id, tx);
                    }

                    Some(addr) => send_sse_tx(addr, tx),
                };
            }

            // try to send a ReqAck message to a cached SSE tx and confirm via given tx
            WsListenRouteReq::TrySend {
                app_id,
                msg,
                tx_ack,
            } => match sse_clients.cache_get(&app_id) {
                Some(client) => match client.tx.send(sse::Data::new(msg.as_str())).await {
                    Ok(_) => {
                        sse_clients.cache_remove(&app_id);
                        let _ = tx_ack.send(true);
                    }
                    Err(err) => {
                        error!(
                            "Error sending ReqAck to SSE client in 'ws_listen_router': {:?}",
                            err
                        );
                        let _ = tx_ack.send(false);
                    }
                },
                None => {
                    let _ = tx_ack.send(false);
                }
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct Challenge<'a> {
    pub challenge: &'a str,
}

#[derive(Debug, Deserialize)]
struct ChallengeVerify {
    pub verifier: String,
    pub challenge: String,
}

#[derive(Debug, Deserialize)]
struct ReqAck {
    pub accepted: bool,
    pub req_id: String,
    pub verifier: String,
}

#[derive(Debug, Serialize)]
struct Verify<'a> {
    pub verifier: &'a str,
}

#[derive(PartialEq)]
enum AuthState {
    Open,
    Challenge,
    ChallengeVerify,
    Authenticated,
}

pub struct MfaListenWs {
    auth_challenge: Option<String>,
    auth_state: AuthState,
    data: web::Data<AppState>,
    heartbeat: Instant,
    mfa_app: MfaApp,
    state: Mutex<MfaListenWsState>,
}

struct MfaListenWsState {
    mfa_reqs: cached::TimedCache<String, WsListenMsg>,
    sse_tx: cached::TimedCache<String, WsListenTx>,
}

impl MfaListenWs {
    pub fn new(data: web::Data<AppState>, mfa_app: MfaApp) -> Self {
        let state = Mutex::new(MfaListenWsState {
            mfa_reqs: cached::TimedCache::with_lifespan(*MFA_REQ_LIFETIME),
            sse_tx: cached::TimedCache::with_lifespan(*MFA_REQ_LIFETIME),
        });

        Self {
            auth_state: AuthState::Open,
            auth_challenge: None,
            data,
            heartbeat: Instant::now(),
            mfa_app,
            state,
        }
    }

    fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.heartbeat) > CLIENT_TIMEOUT {
                info!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                ctx.terminate();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for MfaListenWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Started new WebSocket session");
        self.heartbeat(ctx);

        let addr = ctx.address();
        let app_id = self.mfa_app.app_id.clone();
        let tx = self.data.caches.tx_ws_listen.clone();
        async move {
            tx.send(WsListenRouteReq::New { app_id, addr })
                .await
                .expect("register WS client")
        }
        .into_actor(self)
        .wait(ctx);

        // send the first challenge
        let c = get_rand(48);
        let s = serde_json::to_string(&Challenge { challenge: &c })
            .expect("serializing mfa app challenge");
        ctx.text(format!("SYN {}", s));
        // self.challenges.cache_set("auth".to_string(), c);
        self.auth_challenge = Some(c);
        self.auth_state = AuthState::Challenge;
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!(
            "WebSocket Session with app_id {} was stopped: {:?}",
            self.mfa_app.app_id,
            ctx.state(),
        );
    }
}

impl actix::Handler<WsListenMsg> for MfaListenWs {
    type Result = ();

    fn handle(&mut self, msg: WsListenMsg, ctx: &mut Self::Context) -> Self::Result {
        debug!(
            "in actix::Handler<WsListenMsg> - new MFAReq for req_id '{}'",
            msg.req_id
        );

        if self.auth_state == AuthState::Authenticated {
            let app_msg = format!(
                "REQ {}",
                serde_json::to_string(&msg).expect("serializing WsListenMsg")
            );
            debug!("app_msg in actix::Handler<WsListenMsg>: {}", app_msg);
            ctx.text(app_msg);
        } else {
            info!("Not sending MFA App request - un-authenticated session");
        }

        match self.state.lock() {
            Ok(mut state) => {
                state.mfa_reqs.cache_set(msg.req_id.clone(), msg);
            }
            Err(err) => error!(
                "Mutex Lock Error in 'actix::Handler<WsListenMsg>' : {}",
                err
            ),
        }
    }
}

impl actix::Handler<WsListenTx> for MfaListenWs {
    type Result = ();

    fn handle(&mut self, msg: WsListenTx, ctx: &mut Self::Context) -> Self::Result {
        let req_id = &msg.req_id;
        let mut sse_msg = None;

        debug!("in actix::Handler<WsListenTx>");

        {
            let lock = self.state.lock();
            if lock.is_err() {
                error!("Mutex Lock Error in 'actix::Handler<WsListenTx>'");
                ctx.close(Some(CloseReason::from(CloseCode::Error)));
                return;
            }
            let mut state = lock.unwrap();

            match state.mfa_reqs.cache_get(req_id) {
                Some(mfa_req) => {
                    // TODO should we try fetch the mfa_req from the global cache to avoid problems
                    // with HA deployment?

                    if mfa_req.accepted.is_none() {
                        debug!(
                            "'actix::Handler<WsListenTx>': mfa_req not handled yet, sse_tx cached"
                        );
                        state.sse_tx.cache_set(msg.req_id.clone(), msg.clone());
                        return;
                    }

                    debug!("'actix::Handler<WsListenTx>': mfa_req found - processing");
                    let m = if *mfa_req.accepted.as_ref().unwrap() {
                        format!(
                            "message ACK {}",
                            serde_json::to_string(&MfaAckResponse {
                                req_id: &msg.req_id,
                                loc: &msg.loc,
                            })
                            .unwrap()
                        )
                    } else {
                        "message REJECTED".to_string()
                    };

                    debug!("'actix::Handler<WsListenTx>': mfa_req found, msg: {}", m);

                    sse_msg = Some(m);
                    let data = self.data.clone();
                    let app_id = self.mfa_app.app_id.clone();
                    let email = self.mfa_app.email.clone();
                    set_mfa_req_state(
                        data,
                        app_id,
                        email,
                        mfa_req.to_owned(),
                        MfaLoginRequestState::Sent,
                    )
                    .into_actor(self)
                    .wait(ctx);
                }
                None => {
                    debug!("'actix::Handler<WsListenTx>': mfa_req not found, sse_tx cached");
                    state.sse_tx.cache_set(msg.req_id.clone(), msg.clone());
                }
            }
        }

        if let Some(m) = sse_msg {
            async move {
                if let Err(err) = msg.tx.send(sse::Data::new(m.as_str())).await {
                    error!("SSE Client gone: {:?}", err);
                }
            }
            .into_actor(self)
            .wait(ctx);
        }
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MfaListenWs {
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
                if text.starts_with("SYNACK ") {
                    let (_, payload) = text.split_once(' ').unwrap();
                    // only accept /vc when in state Challenge
                    if self.auth_state != AuthState::Challenge {
                        let mut reason = CloseReason::from(CloseCode::Invalid);
                        reason.description = Some(String::from("Session state != 'Challenge'"));
                        error!("WS close: {:?}", reason);
                        ctx.close(Some(reason));
                    }

                    // deserialize the payload
                    let res = serde_json::from_str::<ChallengeVerify>(payload);
                    if res.is_err() {
                        let mut reason = CloseReason::from(CloseCode::Invalid);
                        reason.description =
                            Some(String::from("Cannot deserialize the ChallengeVerify"));
                        error!("WS close: {:?}", reason);
                        ctx.close(Some(reason));
                    }
                    let challenge_verify = res.unwrap();

                    // handle the request
                    let challenge = self.auth_challenge.as_deref().unwrap_or("");
                    match validate_challenge(
                        challenge,
                        Some(&challenge_verify.challenge),
                        &self.mfa_app.secret,
                        &challenge_verify.verifier,
                    ) {
                        Ok(s) => {
                            self.auth_state = AuthState::ChallengeVerify;
                            // act.challenges.cache_remove(&"auth".to_string());
                            self.auth_challenge = None;
                            let v = serde_json::to_string(&Verify { verifier: &s })
                                .expect("serializing mfa app verify");
                            let resp = format!("ACK {}", v);
                            ctx.text(resp);
                        }
                        Err(e) => {
                            let mut reason = CloseReason::from(CloseCode::Invalid);
                            reason.description = Some(e.message);
                            error!("WS close: {:?}", reason);
                            ctx.close(Some(reason));
                        }
                    }
                } else if text.starts_with("OK") {
                    if self.auth_state != AuthState::ChallengeVerify {
                        let mut reason = CloseReason::from(CloseCode::Invalid);
                        reason.description =
                            Some(String::from("Session state != 'ChallengeVerify'"));
                        error!("WS close: {:?}", reason);
                        ctx.close(Some(reason));
                    }

                    // since this is the last step of the handshake, we can send mfa requests
                    // from now on
                    self.auth_state = AuthState::Authenticated;
                    ctx.text("OK");

                    // look for a pending MfaLoginRequest
                    let data = self.data.clone();
                    let app_id = self.mfa_app.app_id.clone();
                    get_pending_request(data, app_id)
                        .into_actor(self)
                        .then(|res, act, ctx| {
                            if let Some(req) = res {
                                if req.state == MfaLoginRequestState::Sent {
                                    debug!("Found already processed MfaLoginReq for newly connected WebSocket client");
                                    return fut::ready(());
                                }

                                match serde_json::to_string(&req) {
                                    Ok(msg) => {
                                        let mut state = act.state.lock().expect(
                                            "locking after get_pending_request in 'ws_listen'",
                                        );

                                        // check if we have handled this request already
                                        match state.mfa_reqs.cache_get(&req.req_id) {
                                            Some(mfa_req) => {
                                                // send the request again, if it was not handled yet
                                                if mfa_req.accepted.is_none() {
                                                    debug!(
                                                        "Re-Sending MfaReq for req_id '{}'",
                                                        req.req_id
                                                    );
                                                    ctx.text(format!("REQ {}", msg));
                                                }
                                            }
                                            None => {
                                                state.mfa_reqs.cache_set(
                                                    req.req_id.clone(),
                                                    WsListenMsg::new(
                                                        req.challenge,
                                                        req.exp,
                                                        req.req_id,
                                                    ),
                                                );

                                                ctx.text(format!("REQ {}", msg));
                                            }
                                        };
                                    }
                                    Err(err) => {
                                        error!("Serializing MfaLoginRequest: {}", err);
                                    }
                                };
                            }

                            fut::ready(())
                        })
                        .wait(ctx);
                } else if text.starts_with("REQACK ") {
                    // only accept these requests, if the client is authenticated
                    if self.auth_state != AuthState::Authenticated {
                        warn!(
                            "Received REQACK from Unauthenticated WebSocket Client with app_id {}",
                            self.mfa_app.app_id
                        );
                        ctx.close(Some(CloseReason::from(CloseCode::Policy)));
                        return;
                    }

                    let (_, payload) = text.split_once(' ').unwrap();
                    debug!("Payload in REQACK: {}", payload);

                    // deserialize the payload
                    let res = serde_json::from_str::<ReqAck>(payload);
                    if res.is_err() {
                        let mut reason = CloseReason::from(CloseCode::Invalid);
                        reason.description = Some(String::from("Cannot deserialize the REQACK"));
                        error!("WS close: {:?}", reason);
                        ctx.close(Some(reason));
                    }
                    let req_ack = res.unwrap();
                    let req_id = &req_ack.req_id;

                    // do we have the mfa_req?
                    let mut mfa_req = {
                        let mut lock = self.state.lock().expect("Locking state in 'listen_ws'");
                        let mfa_req_opt = lock.mfa_reqs.cache_get(req_id);

                        if mfa_req_opt.is_none() {
                            debug!("mfa_req_opt.is_none() in line 472 for req_id '{}'", req_id);
                            warn!(
                                "Received a 'ReqAck' in 'listen_ws' for expired request with req_id {}",
                                req_id
                            );
                            return;
                        }
                        mfa_req_opt.unwrap().clone()
                    };

                    // check expires of mfa_req
                    if mfa_req.exp < chrono::Local::now().naive_utc().timestamp_millis() {
                        debug!(
                            "mfa_req.exp < chrono::Local::now().timestamp():\n{}\n{}",
                            mfa_req.exp,
                            chrono::Local::now().timestamp()
                        );
                        warn!(
                            "Received a 'ReqAck' in 'listen_ws' for expired request with req_id {}",
                            req_id
                        );
                        return;
                    }

                    // validate the ReqAck payload
                    if let Err(err) = validate_challenge(
                        &mfa_req.challenge,
                        None,
                        &self.mfa_app.secret,
                        &req_ack.verifier,
                    ) {
                        warn!(
                            "Received invalid verifier from ReqAck in 'ws_listen' for req_id {}",
                            req_id
                        );
                        let mut reason = CloseReason::from(CloseCode::Invalid);
                        reason.description = Some(err.message);
                        error!("WS close for invalid Verifier: {:?}", reason);
                        ctx.close(Some(reason));
                        return;
                    }

                    let lock = self.state.lock();
                    if lock.is_err() {
                        error!("Mutex Lock Error in 'StreamHandler<_> for MfaListenWs'");
                        ctx.close(Some(CloseReason::from(CloseCode::Error)));
                        return;
                    }
                    let mut state = lock.unwrap();

                    // check if the SSE client has already subscribed
                    let email = self.mfa_app.email.clone();
                    if let Some(listen_req) = state.sse_tx.cache_get_mut(req_id) {
                        let msg = if req_ack.accepted {
                            format!(
                                "message ACK {}",
                                serde_json::to_string(&MfaAckResponse {
                                    req_id: &listen_req.req_id,
                                    loc: &listen_req.loc,
                                })
                                .unwrap()
                            )
                        } else {
                            "message REJECTED".to_string()
                        };
                        let tx = listen_req.tx.clone();

                        async move {
                            if let Err(err) = tx.send(sse::Data::new(msg.as_str())).await {
                                error!("SSE Client gone: {:?}", err);
                            }
                        }
                        .into_actor(self)
                        .wait(ctx);

                        // state.mfa_reqs.cache_remove(req_id);
                        mfa_req.accepted = Some(req_ack.accepted);
                        state
                            .mfa_reqs
                            .cache_set(req_id.to_string(), mfa_req.clone());
                        state.sse_tx.cache_remove(req_id);

                        let data = self.data.clone();
                        let app_id = self.mfa_app.app_id.clone();
                        set_mfa_req_state(
                            data,
                            app_id,
                            email,
                            mfa_req,
                            MfaLoginRequestState::Accepted,
                        )
                        .into_actor(self)
                        .wait(ctx);
                    } else {
                        // if the SSE client has not subscribed yet, save the ack status for later
                        state.mfa_reqs.cache_set(
                            req_id.to_string(),
                            WsListenMsg {
                                challenge: mfa_req.challenge.to_string(),
                                exp: mfa_req.exp,
                                req_id: req_id.to_string(),
                                accepted: Some(req_ack.accepted),
                            },
                        );

                        let data = self.data.clone();
                        let app_id = self.mfa_app.app_id.clone();
                        let state = if req_ack.accepted {
                            MfaLoginRequestState::Accepted
                        } else {
                            MfaLoginRequestState::Rejected
                        };
                        set_mfa_req_state(data, app_id, email, mfa_req, state)
                            .into_actor(self)
                            .wait(ctx);

                        info!(
                            "No SSE client connected for req_id {} in 'ws_listen'. State cached.",
                            req_id
                        );
                    }
                } else {
                    error!("Invalid WebSocket request");
                    ctx.close(Some(CloseReason::from(CloseCode::Invalid)));
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

fn validate_challenge(
    challenge_local: &str,
    challenge_remote: Option<&str>,
    secret: &str,
    verifier: &str,
) -> Result<String, ErrorResponse> {
    // calculate s256 hash of own challenge
    let c = format!("{}{}", challenge_local, secret);
    let verifier_res = hex::decode(verifier);
    if verifier_res.is_err() {
        let err = String::from("Error decoding verifier from MFA App");
        error!("{}", err);
        return Err(ErrorResponse::new(ErrorResponseType::BadRequest, err));
    }
    let verifier = verifier_res.as_ref().unwrap().as_slice();

    // check challenge verifier
    let s256 = digest::digest(&digest::SHA256, c.as_bytes());
    // let s256 = sha::sha256(c.as_bytes());
    if s256.as_ref() != verifier {
        let err = String::from("Could not verify the MFA App request challenge");
        error!("{}", err);
        debug!("Local s256 hash: {:?}", s256);
        debug!("MFA App verifier: {:?}", verifier_res);
        return Err(ErrorResponse::new(ErrorResponseType::BadRequest, err));
    }

    // calculate verifier for the MFA App
    let mut resp_hex = String::default();
    if let Some(c) = challenge_remote {
        let app_challenge = format!("{}{}", c, secret);
        let s256_resp = digest::digest(&digest::SHA256, app_challenge.as_bytes());
        resp_hex = hex::encode(s256_resp);
    }

    Ok(resp_hex)
}

async fn get_pending_request(data: web::Data<AppState>, app_id: String) -> Option<MfaLoginRequest> {
    if let Ok(login_req) = get_mfa_login_req(&data, &app_id).await {
        Some(login_req)
    } else {
        None
    }
}

async fn set_mfa_req_state(
    data: web::Data<AppState>,
    app_id: String,
    email: String,
    req: WsListenMsg,
    state: MfaLoginRequestState,
) {
    let mfa_req = MfaLoginRequest {
        challenge: req.challenge,
        email,
        exp: req.exp,
        mfa_app_id: app_id,
        req_id: req.req_id,
        state,
    };

    if let Err(err) = put_mfa_login_req(&data, &mfa_req).await {
        error!("Updating MfaLoginRequest after processing: {:?}", err);
    }
}
