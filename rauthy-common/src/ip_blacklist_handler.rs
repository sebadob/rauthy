use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tokio::sync::oneshot;
use tracing::error;

#[derive(Debug, PartialEq)]
pub enum IpBlacklistReq {
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
pub async fn run(rx: flume::Receiver<IpBlacklistReq>) {
    let mut data_blacklist: HashMap<String, DateTime<Utc>> = HashMap::with_capacity(2);
    let mut data_failed_logins: HashMap<String, u32> = HashMap::with_capacity(5);

    loop {
        match rx.recv_async().await {
            Ok(req) => match req {
                IpBlacklistReq::Blacklist(req) => {
                    data_blacklist.insert(req.ip, req.exp);
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
