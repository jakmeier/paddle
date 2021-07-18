use std::{
    rc::Rc,
    sync::atomic::{AtomicBool, AtomicU16, Ordering},
};

use crate::{ImageDesc, LoadScheduler, NutsCheck};

/// Helper struct to load multiple assets in parallel and track their progress.
pub struct AssetBundle {
    images: Vec<ImageDesc>,
}

#[derive(Clone)]
pub struct AssetLoadingTracker(Rc<AssetLoadingTrackerData>);

struct AssetLoadingTrackerData {
    total: u16,
    loaded: AtomicU16,
    had_error: AtomicBool,
}

impl AssetBundle {
    pub fn new() -> Self {
        Self { images: Vec::new() }
    }
    pub fn add_images(&mut self, images: &[ImageDesc]) {
        self.images.extend(images);
    }
    /// Loads all items in the bundle and insert them into the asset library available to all display frames.
    pub fn load(self) -> AssetLoadingTracker {
        let tracker = AssetLoadingTracker(Rc::new(AssetLoadingTrackerData {
            total: self.images.len() as u16,
            loaded: AtomicU16::new(0),
            had_error: AtomicBool::new(false),
        }));

        let mut image_futures = Vec::new();
        image_futures.reserve(self.images.len());
        for desc in self.images {
            let tracker = tracker.clone();
            let future = async move {
                let result = desc.load().await;
                if result.is_err() {
                    tracker.0.had_error.store(true, Ordering::SeqCst);
                }
                result.nuts_check();
                tracker.0.loaded.fetch_add(1, Ordering::SeqCst);
            };
            image_futures.push(future);
        }

        let mut scheduler = LoadScheduler::new();
        scheduler.register_vec(image_futures, "Loading Images");

        tracker
    }
}

impl AssetLoadingTracker {
    pub fn progress(&self) -> f32 {
        self.0.loaded.load(Ordering::SeqCst) as f32 / self.0.total as f32
    }
    pub fn total(&self) -> u16 {
        self.0.total
    }
    pub fn loaded(&self) -> u16 {
        self.0.loaded.load(Ordering::SeqCst)
    }
    pub fn had_error(&self) -> bool {
        self.0.had_error.load(Ordering::SeqCst)
    }
}
