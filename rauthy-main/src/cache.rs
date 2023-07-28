// use cached::Cached;
// use tracing::{error, warn};
// use tokio::sync::mpsc;
// use tokio::sync::oneshot;
//
// // The struct which is sent over the channel to the cache thread when data is requested.
// pub struct CacheReq<T> {
//     // TODO can this be optimized and borrowed?
//     pub entry: String,
//     pub method: CacheMethod,
//     pub value: Option<T>,
//     pub resp: Option<oneshot::Sender<Option<T>>>,
// }
//
// pub enum CacheMethod {
//     Get,
//     Put,
//     Del,
// }
//
// // GET a value out of the cache
// pub async fn cache_get_local<T>(entry: &str, tx: &mpsc::Sender<CacheReq<T>>) -> Option<T>
// where
//     T: Clone,
// {
//     let (tx_one, rx_one) = oneshot::channel::<Option<T>>();
//
//     let req = CacheReq {
//         entry: String::from(entry),
//         method: CacheMethod::Get,
//         value: None,
//         resp: Some(tx_one),
//     };
//     let send = tx.send(req).await;
//     if send.is_err() {
//         error!("Error sending request into cache channel");
//     }
//
//     let res = rx_one.await;
//     if let Err(e) = res {
//         error!("Error receiving cache answer on oneshot channel: {:?}", e);
//         return None;
//     }
//     res.unwrap()
// }
//
// // PUT a value into the cache
// pub async fn cache_put_local<T>(entry: &str, value: &T, tx: &mpsc::Sender<CacheReq<T>>)
// where
//     T: Clone,
// {
//     let req = CacheReq {
//         entry: String::from(entry),
//         method: CacheMethod::Put,
//         value: Some(value.clone()),
//         resp: None,
//     };
//     let send = tx.send(req).await;
//     if send.is_err() {
//         error!("Error sending request into cache channel");
//     }
// }
//
// // DELETE a value from the cache
// pub async fn cache_del_local<T>(entry: &str, tx: &mpsc::Sender<CacheReq<T>>)
// where
//     T: Clone,
// {
//     let req = CacheReq {
//         entry: String::from(entry),
//         method: CacheMethod::Del,
//         value: None,
//         resp: None,
//     };
//     let send = tx.send(req).await;
//     if send.is_err() {
//         error!("Error sending request into cache channel");
//     }
// }
//
// /**
// This is the Caches main loop.<br>
// Its one and only job is to wait for an incoming message on the channel and then execute the
// requested operation.
// */
// pub async fn cache_recv<C, T>(mut cache: C, mut rx: mpsc::Receiver<CacheReq<T>>)
// where
//     C: Cached<String, T>,
//     T: Clone,
// {
//     loop {
//         let req_opt = rx.recv().await;
//         if req_opt.is_some() {
//             let req = req_opt.unwrap();
//             match req.method {
//                 CacheMethod::Get => {
//                     if req.resp.is_none() {
//                         error!("Cannot get a value from cache without a return channel");
//                         continue;
//                     }
//                     let res_opt = cache.cache_get(&req.entry);
//                     let res = if res_opt.is_some() {
//                         // debug!("Cache hit for {}", &req.entry);
//                         Some(res_opt.as_ref().unwrap().to_owned().clone())
//                     } else {
//                         // debug!("Cache miss for {}", &req.entry);
//                         None
//                     };
//                     req.resp
//                         .unwrap()
//                         .send(res)
//                         .map_err(|_| error!("Error sending oneshot response"))
//                         .expect("Error sending oneshot response");
//                 }
//                 CacheMethod::Put => {
//                     if req.value.is_none() {
//                         continue;
//                     }
//                     cache.cache_set(req.entry, req.value.unwrap());
//                 }
//                 CacheMethod::Del => {
//                     cache.cache_remove(&req.entry);
//                 }
//             }
//         } else {
//             warn!("Received 'None' in 'cache_recv' - exiting");
//             break;
//         }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use pretty_assertions::assert_eq;
//     use std::time::Duration;
//
//     macro_rules! aw {
//         ($e:expr) => {
//             tokio_test::block_on($e)
//         };
//     }
//
//     #[test]
//     fn test_cache_recv_sized() -> Result<(), Box<dyn std::error::Error>> {
//         let rt = tokio::runtime::Runtime::new()?;
//
//         let (tx, rx) = mpsc::channel::<CacheReq<i64>>(10);
//         rt.spawn(cache_recv(cached::SizedCache::with_size(2), rx));
//
//         aw!(cache_put_local("1", &1, &tx));
//         aw!(cache_put_local("2", &2, &tx));
//         aw!(cache_put_local("3", &3, &tx));
//
//         let one = aw!(cache_get_local("1", &tx));
//         let two = aw!(cache_get_local("2", &tx)).unwrap();
//         let three = aw!(cache_get_local("3", &tx)).unwrap();
//
//         assert!(one.is_none());
//         assert_eq!(two, 2);
//         assert_eq!(three, 3);
//
//         aw!(cache_del_local("2", &tx));
//         let two = aw!(cache_get_local("2", &tx));
//         assert!(two.is_none());
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_cache_recv_sized_timed() -> Result<(), Box<dyn std::error::Error>> {
//         let rt = tokio::runtime::Runtime::new()?;
//
//         let (tx, rx) = mpsc::channel::<CacheReq<i64>>(10);
//         rt.spawn(cache_recv(
//             cached::TimedSizedCache::with_size_and_lifespan(2, 1),
//             rx,
//         ));
//
//         aw!(cache_put_local("1", &1, &tx));
//         aw!(cache_put_local("2", &2, &tx));
//         aw!(cache_put_local("3", &3, &tx));
//
//         let one = aw!(cache_get_local("1", &tx));
//         let two = aw!(cache_get_local("2", &tx)).unwrap();
//         let three = aw!(cache_get_local("3", &tx)).unwrap();
//         assert!(one.is_none());
//         assert_eq!(two, 2);
//         assert_eq!(three, 3);
//
//         // after 1 second, the values should be gone
//         std::thread::sleep(Duration::from_secs(1));
//         let one = aw!(cache_get_local("1", &tx));
//         let two = aw!(cache_get_local("2", &tx));
//         let three = aw!(cache_get_local("3", &tx));
//         assert!(one.is_none());
//         assert!(two.is_none());
//         assert!(three.is_none());
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_cache_recv_timed() -> Result<(), Box<dyn std::error::Error>> {
//         let rt = tokio::runtime::Runtime::new()?;
//
//         let (tx, rx) = mpsc::channel::<CacheReq<i64>>(10);
//         rt.spawn(cache_recv(cached::TimedCache::with_lifespan(1), rx));
//
//         aw!(cache_put_local("1", &1, &tx));
//         aw!(cache_put_local("2", &2, &tx));
//         aw!(cache_put_local("3", &3, &tx));
//
//         let one = aw!(cache_get_local("1", &tx)).unwrap();
//         let two = aw!(cache_get_local("2", &tx)).unwrap();
//         let three = aw!(cache_get_local("3", &tx)).unwrap();
//         assert_eq!(one, 1);
//         assert_eq!(two, 2);
//         assert_eq!(three, 3);
//
//         // after 1 second, the values should be gone
//         std::thread::sleep(Duration::from_secs(1));
//         let one = aw!(cache_get_local("1", &tx));
//         let two = aw!(cache_get_local("2", &tx));
//         let three = aw!(cache_get_local("3", &tx));
//         assert!(one.is_none());
//         assert!(two.is_none());
//         assert!(three.is_none());
//
//         Ok(())
//     }
// }
