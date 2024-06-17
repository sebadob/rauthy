use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::oneshot;
use tracing::{debug, error};

#[derive(Debug)]
pub enum IpBlacklistReq {
    CheckExp,
    Blacklist(IpBlacklist),
    BlacklistCheck(IpBlacklistCheck),
    BlacklistDelete(String),
    LoginCheck(IpFailedLoginCheck),
    LoginFailedSet(IpLoginFailedSet),
    LoginFailedDelete(String),
    GetBlacklistedIps(oneshot::Sender<HashMap<String, DateTime<Utc>>>),
}

#[derive(Debug)]
pub struct IpBlacklist {
    pub ip: String,
    pub exp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct IpBlacklistCheck {
    pub ip: String,
    pub tx: oneshot::Sender<Option<DateTime<Utc>>>,
}

#[derive(Debug)]
pub struct IpFailedLoginCheck {
    pub ip: String,
    pub increase_counter: bool,
    /// counter for invalid requests from this IP
    pub tx: oneshot::Sender<Option<u32>>,
}

#[derive(Debug)]
pub struct IpLoginFailedSet {
    pub ip: String,
    pub invalid_logins: u32,
}

/// Handles blacklisted IP's and IP's with failed logins
pub async fn run(tx: flume::Sender<IpBlacklistReq>, rx: flume::Receiver<IpBlacklistReq>) {
    let mut data_blacklist: HashMap<String, DateTime<Utc>> = HashMap::with_capacity(2);
    let mut data_failed_logins: HashMap<String, u32> = HashMap::with_capacity(2);

    let mut exp_checker_handle = tokio::spawn(spawn_exp_checker(tx.clone()));

    loop {
        match rx.recv_async().await {
            Ok(req) => match req {
                IpBlacklistReq::CheckExp => {
                    debug!("Running IpBlacklistReq::CheckExp");
                    let now = Utc::now();
                    let mut remove = Vec::default();
                    for (k, v) in data_blacklist.iter() {
                        if &now > v {
                            remove.push(k.clone());
                        }
                    }

                    debug!("Removing {} IPs in IpBlacklistReq::CheckExp", remove.len());
                    for key in remove {
                        data_blacklist.remove(&key);
                    }

                    if data_blacklist.is_empty() && !exp_checker_handle.is_finished() {
                        exp_checker_handle.abort();
                        debug!("IpBlacklist ExpChecker has been stopped");
                    }
                }

                IpBlacklistReq::Blacklist(req) => {
                    data_blacklist.insert(req.ip, req.exp);

                    if exp_checker_handle.is_finished() {
                        exp_checker_handle = tokio::spawn(spawn_exp_checker(tx.clone()));
                    }
                }

                IpBlacklistReq::BlacklistCheck(req) => {
                    req.tx
                        .send(data_blacklist.get(&req.ip).cloned())
                        .expect("oneshot receiver to not be closed");
                }

                IpBlacklistReq::LoginCheck(req) => {
                    let counter = if let Some(counter) = data_failed_logins.get_mut(&req.ip) {
                        if req.increase_counter {
                            *counter += 1;
                            Some(*counter)
                        } else {
                            Some(*counter)
                        }
                    } else if req.increase_counter {
                        data_failed_logins.insert(req.ip, 1);
                        Some(1)
                    } else {
                        None
                    };

                    req.tx
                        .send(counter)
                        .expect("oneshot receiver to not be closed");
                }

                IpBlacklistReq::LoginFailedSet(req) => {
                    if let Some(counter) = data_failed_logins.get_mut(&req.ip) {
                        if req.invalid_logins > *counter {
                            *counter = req.invalid_logins;
                        }
                    } else {
                        data_failed_logins.insert(req.ip, req.invalid_logins);
                    }
                }

                IpBlacklistReq::BlacklistDelete(ip) => {
                    data_blacklist.remove(&ip);
                }

                IpBlacklistReq::LoginFailedDelete(ip) => {
                    data_failed_logins.remove(&ip);
                }

                IpBlacklistReq::GetBlacklistedIps(tx) => {
                    // just clone the whole HashMap and don't do any iterations here
                    // this handler is in a performance-critical spot.
                    tx.send(data_blacklist.clone()).unwrap();
                }
            },

            Err(err) => {
                error!(
                    "ip_blacklist_handler: {:?}\n\nThis should never happen!",
                    err
                );
            }
        }
    }
}

async fn spawn_exp_checker(tx: flume::Sender<IpBlacklistReq>) {
    debug!("IpBlacklist ExpChecker has been started");
    let mut interval = tokio::time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;
        tx.send_async(IpBlacklistReq::CheckExp).await.unwrap();
    }
}
