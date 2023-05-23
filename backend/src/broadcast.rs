use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use actix_web::rt::time::interval;
use actix_web_lab::sse::{self, ChannelStream, Sender, Sse};
use futures::future;
use log::{info, trace};

pub struct Broadcaster {
    inner: Mutex<BroadcasterInner>,
}

struct BroadcasterInner {
    clients: Vec<sse::Sender>,
}

impl Broadcaster {
    pub fn new() -> Arc<Self> {
        let broadcaster = Arc::new(Broadcaster {
            inner: Mutex::new(BroadcasterInner {
                clients: Vec::new(),
            }),
        });

        Broadcaster::start_ping(Arc::clone(&broadcaster));
        broadcaster
    }

    pub async fn register_client(&self) -> Result<Sse<ChannelStream>, sse::SendError> {
        let (tx, rx) = sse::channel(4);

        tx.send(sse::Data::new("connected")).await?;
        self.inner.lock().unwrap().clients.push(tx);
        Ok(rx)
    }

    pub fn start_ping(broadcaster: Arc<Self>) {
        actix_web::rt::spawn(async move {
            info!(
                "Starting ping on worker {}",
                std::thread::current().name().unwrap_or("(unknown worker)")
            );
            let mut interval = interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                broadcaster.remove_stale_clients().await;
                trace!("Ping completed");
            }
        });
    }

    pub async fn broadcast(&self, msg: &str) {
        let clients = self.inner.lock().unwrap().clients.clone();
        let send_futures = clients
            .iter()
            .map(|client| client.send(sse::Data::new(msg)));

        let _ = future::join_all(send_futures).await;
        info!("Broadcasted message: {msg}");
    }

    async fn remove_stale_clients(&self) {
        let ping_futures = {
            let clients = self.inner.lock().unwrap().clients.clone();
            trace!("Pinging {} clients", clients.len());
            clients.into_iter().map(Broadcaster::ping)
        };

        let result = future::join_all(ping_futures).await;
        let fresh_clients = result.into_iter().flatten().collect::<Vec<_>>();

        trace!(
            "Removed stale clients (fresh clients: {})",
            &fresh_clients.len()
        );
        self.inner.lock().unwrap().clients = fresh_clients;
    }

    async fn ping(client: Sender) -> Option<Sender> {
        match client.send(sse::Data::new("ping")).await {
            Ok(_) => Some(client),
            Err(_) => None,
        }
    }
}
