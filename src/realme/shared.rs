use std::{
    sync::{
        Arc,
        RwLock,
    },
    time::{
        Duration,
        Instant,
    },
};

use super::SharedRealme;
use crate::{
    Error,
    Result,
    prelude::*,
};

impl RealmeBuilder {
    pub fn shared_build(mut self) -> Result<SharedRealme> {
        self.check_profile()?;
        self.adaptors.sort_by(|a, b| a.priority.cmp(&b.priority));
        let (sender, receiver) = crossbeam::channel::unbounded::<()>();
        let cache = update_cache(&self.adaptors, &sender)?;
        let shared_realme = Arc::new(RwLock::new(Realme {
            cache,
            default: None,
            builder: self.clone(),
        }));

        let shared_realme_clone = shared_realme.clone();
        let builder_clone = std::sync::RwLock::new(self);

        std::thread::spawn(move || -> Result<()> {
            // To avoid too many updates, set a debounce time
            let debounce_duration = Duration::from_millis(1000);
            // Timeout time
            let timeout_duration = Duration::from_millis(500);
            let mut last_update = Instant::now();
            let mut should_update = false;

            loop {
                match receiver.recv_timeout(timeout_duration) {
                    Ok(()) => {
                        should_update = true;
                    }
                    Err(crossbeam::channel::RecvTimeoutError::Timeout) => {
                        // Timeout, continue loop
                    }
                    Err(crossbeam::channel::RecvTimeoutError::Disconnected) => {
                        // Channel closed, exit loop
                        break;
                    }
                }
                let now = Instant::now();
                if should_update &&
                    now.duration_since(last_update) >= debounce_duration
                {
                    // Update shared_realme
                    if let Ok(mut realme) = shared_realme_clone.write() {
                        if let Ok(builder) = builder_clone.read() {
                            let new_cache =
                                update_cache(&builder.adaptors, &sender)?;
                            realme.cache = new_cache;
                        }
                    }
                    last_update = now;
                    should_update = false;
                }
            }

            Ok(())
        });

        Ok(shared_realme)
    }
}

fn update_cache(
    adaptor: &[Adaptor],
    sender: &crossbeam::channel::Sender<()>,
) -> Result<Value> {
    let mut cache = Value::Table(Map::new());
    adaptor.iter().try_for_each(|adaptor| {
        adaptor.watcher(sender.clone()).and_then(|()| {
            adaptor.parse().and_then(|value| match value {
                Value::Table(table) => {
                    cache.merge(&Value::Table(table));
                    Ok(())
                }
                Value::Null => Ok(()),
                _ => Err(Error::new_build_error(
                    "Adaptor parse result is not a table".to_string(),
                )),
            })
        })
    })?;
    Ok(cache)
}

// impl SharedRealme {
//     pub fn get_realme(&self) -> Result<&Realme> {
//         let Ok(realme) = self.0.read() else {
//             return Err(Error::LockError("realme".to_string()));
//         };
//         Ok(realme)
//     }
// }
