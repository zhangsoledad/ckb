use crate::config::NotifierConfig;
use ckb_logger::{debug, error, info, warn};
use ckb_types::{packed, prelude::*};
use fnv::FnvHashMap;
use lru_cache::LruCache;
use std::process::Command;
use std::sync::Arc;

const CANCEL_FILTER_SIZE: usize = 128;

pub struct Notifier {
    /// cancelled alerts
    cancel_filter: LruCache<u32, ()>,
    /// alerts we received
    received_alerts: FnvHashMap<u32, Arc<packed::Alert>>,
    /// alerts that self node should notice
    noticed_alerts: Vec<Arc<packed::Alert>>,
    client_version: String,
    config: NotifierConfig,
}

impl Notifier {
    pub fn new(client_version: String, config: NotifierConfig) -> Self {
        Notifier {
            cancel_filter: LruCache::new(CANCEL_FILTER_SIZE),
            received_alerts: Default::default(),
            noticed_alerts: Vec::new(),
            client_version,
            config,
        }
    }

    fn is_version_effective(&self, alert: &Arc<packed::Alert>) -> bool {
        use semver::Version;
        if let Ok(client_version) = Version::parse(&self.client_version) {
            if alert
                .as_reader()
                .raw()
                .min_version()
                .to_opt()
                .and_then(|v| {
                    v.as_utf8()
                        .ok()
                        .and_then(|v| Version::parse(v).map(|min_v| client_version < min_v).ok())
                        .or(Some(true))
                })
                .unwrap_or(false)
            {
                return false;
            }
            if alert
                .as_reader()
                .raw()
                .max_version()
                .to_opt()
                .and_then(|v| {
                    v.as_utf8()
                        .ok()
                        .and_then(|v| Version::parse(v).map(|max_v| client_version > max_v).ok())
                        .or(Some(true))
                })
                .unwrap_or(false)
            {
                return false;
            }
        }
        true
    }

    pub fn add(&mut self, alert: Arc<packed::Alert>) {
        let alert_id = alert.raw().id().unpack();
        let alert_cancel = alert.raw().cancel().unpack();
        if self.has_received(alert_id) {
            return;
        }
        // checkout cancel_id
        if alert_cancel > 0 {
            self.cancel(alert_cancel);
        }
        // add to received alerts
        self.received_alerts.insert(alert_id, Arc::clone(&alert));

        // check conditions, figure out do we need to notice this alert
        if !self.is_version_effective(&alert) {
            debug!("received a version ineffective alert {:?}", alert);
            return;
        }

        if self.noticed_alerts.contains(&alert) {
            return;
        }
        self.notify(&alert);
        self.noticed_alerts.push(alert);
        // sort by priority
        self.noticed_alerts.sort_by_key(|a| {
            let priority: u32 = a.raw().priority().unpack();
            std::u32::MAX - priority
        });
    }

    fn notify(&self, alert: &packed::Alert) {
        let message = unsafe { alert.as_reader().raw().message().as_utf8_unchecked() }.to_owned();
        warn!("receive a new alert: {}", message);
        if let Some(notify_script) = self.config.notify_script.as_ref() {
            match Command::new(notify_script).args(&[message]).status() {
                Ok(exit_status) => {
                    info!("send alert to notify script. {}", exit_status);
                }
                Err(err) => {
                    error!("failed to run notify script: {}", err);
                }
            }
        }
    }

    pub fn cancel(&mut self, cancel_id: u32) {
        self.cancel_filter.insert(cancel_id, ());
        self.received_alerts.remove(&cancel_id);
        self.noticed_alerts.retain(|a| {
            let id: u32 = a.raw().id().unpack();
            id != cancel_id
        });
    }

    pub fn clear_expired_alerts(&mut self, now: u64) {
        self.received_alerts.retain(|_id, alert| {
            let notice_until: u64 = alert.raw().notice_until().unpack();
            notice_until > now
        });
        self.noticed_alerts.retain(|a| {
            let notice_until: u64 = a.raw().notice_until().unpack();
            notice_until > now
        });
    }

    pub fn has_received(&self, id: u32) -> bool {
        self.received_alerts.contains_key(&id) || self.cancel_filter.contains_key(&id)
    }

    // all unexpired alerts
    pub fn received_alerts(&self) -> Vec<Arc<packed::Alert>> {
        self.received_alerts.values().cloned().collect()
    }

    // alerts that self node should noticed
    pub fn noticed_alerts(&self) -> Vec<Arc<packed::Alert>> {
        self.noticed_alerts.clone()
    }
}
