use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::oneshot;
use tracing::{debug, error};

#[derive(Debug, PartialEq)]
pub enum IpBlacklistReq {
    CheckExp,
    Blacklist(IpBlacklist),
    BlacklistCheck(IpBlacklistCheck),
    BlacklistCleanup(IpBlacklistCleanup),
    LoginCheck(IpFailedLoginCheck),
    LoginCleanup(IpBlacklistCleanup),
}

#[derive(Debug, PartialEq)]
pub struct IpBlacklist {
    pub ip: String,
    pub exp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct IpBlacklistCheck {
    pub ip: String,
    pub tx: oneshot::Sender<Option<DateTime<Utc>>>,
}

impl PartialEq for IpBlacklistCheck {
    fn eq(&self, other: &Self) -> bool {
        self.ip == other.ip
    }
}

#[derive(Debug)]
pub struct IpFailedLoginCheck {
    pub ip: String,
    pub increase_counter: bool,
    /// counter for invalid requests from this IP
    pub tx: oneshot::Sender<Option<u32>>,
}

impl PartialEq for IpFailedLoginCheck {
    fn eq(&self, other: &Self) -> bool {
        self.ip == other.ip && self.increase_counter == other.increase_counter
    }
}

#[derive(Debug, PartialEq)]
pub struct IpBlacklistCleanup {
    pub ip: String,
}

/// Handles blacklisted IP's and IP's with failed logins
pub async fn run(tx: flume::Sender<IpBlacklistReq>, rx: flume::Receiver<IpBlacklistReq>) {
    let mut data_blacklist: HashMap<String, DateTime<Utc>> = HashMap::with_capacity(2);
    let mut data_failed_logins: HashMap<String, u32> = HashMap::with_capacity(5);

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
                        // TODO generate event
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

                    // TODO generate event
                }

                IpBlacklistReq::BlacklistCheck(check) => {
                    check
                        .tx
                        .send(data_blacklist.get(&check.ip).cloned())
                        .expect("oneshot receiver to not be closed");
                }

                IpBlacklistReq::LoginCheck(check) => {
                    let counter = match data_failed_logins.get_mut(&check.ip) {
                        None => {
                            if check.increase_counter {
                                data_failed_logins.insert(check.ip, 1);
                                Some(1)
                                // TODO generate event ?
                            } else {
                                None
                            }
                        }
                        Some(counter) => {
                            *counter += 1;
                            Some(*counter)
                        }
                    };

                    check
                        .tx
                        .send(counter)
                        .expect("oneshot receiver to not be closed");
                }

                IpBlacklistReq::BlacklistCleanup(req) => {
                    data_blacklist.remove(&req.ip);
                    // TODO generate event
                }

                IpBlacklistReq::LoginCleanup(req) => {
                    data_failed_logins.remove(&req.ip);
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
