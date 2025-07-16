use std::sync::Arc;
use tracing::debug;
use std::sync::atomic::{ AtomicUsize, Ordering };

#[derive(Debug, Clone)]
pub struct ProxyRotator {
    proxies: Arc<Vec<String>>,
    current_index: Arc<AtomicUsize>,
}

impl ProxyRotator {
    pub fn new(proxies: Vec<String>) -> Self {
        let filtered_proxies: Vec<String> = proxies
            .into_iter()
            .filter(|p| !p.trim().is_empty())
            .collect();

        if !filtered_proxies.is_empty() {
            debug!("Initialized proxy rotator with {} proxies", filtered_proxies.len());
        }

        Self {
            proxies: Arc::new(filtered_proxies),
            current_index: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn get_next_proxy(&self) -> Option<String> {
        if self.proxies.is_empty() {
            return None;
        }

        let index = self.current_index.fetch_add(1, Ordering::SeqCst) % self.proxies.len();
        let proxy = self.proxies[index].clone();
        debug!("Selected proxy [{}]: {}", index, proxy);
        Some(proxy)
    }

    pub fn get_current_proxy(&self) -> Option<String> {
        if self.proxies.is_empty() {
            return None;
        }

        let index = self.current_index.load(Ordering::SeqCst) % self.proxies.len();
        Some(self.proxies[index].clone())
    }

    pub fn has_proxies(&self) -> bool {
        !self.proxies.is_empty()
    }

    pub fn proxy_count(&self) -> usize {
        self.proxies.len()
    }

    pub fn get_all_proxies(&self) -> Vec<String> {
        self.proxies.as_ref().clone()
    }

    pub fn reset_rotation(&self) {
        self.current_index.store(0, Ordering::SeqCst);
        debug!("Proxy rotation reset to start");
    }
}
