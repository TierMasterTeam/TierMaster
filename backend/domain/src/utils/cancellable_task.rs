use tokio::{task, time};
use tokio_util::sync::CancellationToken;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct CancellableTask {
    cancel_token: CancellationToken,
}

impl CancellableTask {
    pub fn new() -> Self {
        Self {
            cancel_token: CancellationToken::new(),
        }
    }

    /// Lance un timer annulable de `duration` puis exécute `task_fn`
    /// Retourne un JoinHandle<Option<T>> pour récupérer la valeur ou None si annulé
    pub fn start<Fut, T>(
        &self,
        duration: Duration,
        task_fn: impl FnOnce() -> Fut + Send + 'static,
    ) -> tokio::task::JoinHandle<Option<T>>
    where
        Fut: std::future::Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let token = self.cancel_token.clone();

        task::spawn(async move {
            let sleep = time::sleep(duration);
            tokio::pin!(sleep);

            tokio::select! {
                _ = &mut sleep => {
                    let result = task_fn().await;
                    Some(result)
                }
                _ = token.cancelled() => {
                    None
                }
            }
        })
    }


    pub fn cancel(&self) {
        self.cancel_token.cancel();
    }
}