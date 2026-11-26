use std::fmt::Display;
use std::sync::OnceLock;
use tracing::error;

static TX_SHUTDOWN: OnceLock<flume::Sender<()>> = OnceLock::new();

/// Registers the global shutdown handler for a started Hiqlite node. Must be called right after
/// the node was started, before anything else can fail.
///
/// Once the shutdown is triggered, the node is shut down cleanly and the process exits with
/// code 1. Without the clean shutdown, the next start finds an unclean shutdown and has to rebuild
/// its state machine from the Raft log.
pub(crate) fn init(client: hiqlite::Client) {
    let (tx, rx) = flume::bounded(1);
    if TX_SHUTDOWN.set(tx).is_err() {
        panic!("shutdown::init() must only be called once at startup");
    }

    tokio::spawn(async move {
        let _ = rx.recv_async().await;
        error!("Shutting down the database / cache layer after a fatal error");
        if let Err(err) = client.shutdown().await {
            error!("Error shutting down the database / cache layer: {err}");
        }
        std::process::exit(1);
    });
}

/// Logs `reason` as a fatal error and triggers the global shutdown. Use this instead of a
/// `panic!()` anywhere after `DB::init()`, also from sync code or a separate thread.
///
/// If the database / cache layer was not started yet, there is nothing to shut down, and the
/// process exits right away.
pub fn trigger(reason: impl Display) {
    if let Some(tx) = TX_SHUTDOWN.get() {
        error!("FATAL: {reason}");
        // `try_send()` because only the first fatal error needs to start the shutdown
        let _ = tx.try_send(());
    } else {
        // logging may not be set up yet at this point
        eprintln!("FATAL: {reason}");
        std::process::exit(1);
    }
}

/// Same as [`trigger()`], but waits for the shutdown to end the process, so the caller never
/// continues.
pub async fn fatal(reason: impl Display) -> ! {
    trigger(reason);
    loop {
        std::future::pending::<()>().await;
    }
}
