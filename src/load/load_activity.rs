use std::collections::HashMap;

use nuts::DomainState;

use crate::{
    AssetLibrary, ComplexShape, Domain, FinishedLoading, FinishedLoadingMsg, Image, ImageDesc,
    LoadScheduler, LoadSchedulerId, LoadedData, LoadingDoneMsg, ShapeDesc, UpdatedProgressMsg,
};

/// Internal activity to keep track of currently loading downloads and reacting to the corresponding events.

pub(crate) struct LoadActivity {
    loading_bundles: HashMap<LoadSchedulerId, LoadScheduler>,
}

// Forwarded from a loading future through LoadActivity into AssetLibrary
pub(crate) struct LoadedImageAsset {
    pub desc: ImageDesc,
    pub img: Image,
}

pub(crate) struct LoadedShapeAsset {
    pub desc: ShapeDesc,
    pub shape: ComplexShape,
}

impl LoadActivity {
    pub(crate) fn init() {
        let activity = LoadActivity {
            loading_bundles: HashMap::new(),
        };
        let aid = nuts::new_domained_activity(activity, &Domain::Frame);
        aid.private_channel(LoadActivity::add_scheduler);
        aid.private_channel(LoadActivity::update_progress);
        aid.private_domained_channel(LoadActivity::image_to_asset_library);
        aid.private_domained_channel(LoadActivity::shape_to_asset_library);
        aid.subscribe(LoadActivity::after_progress);
    }
    fn add_scheduler(&mut self, msg: LoadScheduler) {
        self.loading_bundles.insert(msg.id, msg);
    }
    fn image_to_asset_library(&mut self, domain: &mut DomainState, img_asset: LoadedImageAsset) {
        AssetLibrary::from_domain(domain).add_image(img_asset.desc, img_asset.img);
    }
    fn shape_to_asset_library(&mut self, domain: &mut DomainState, shape_asset: LoadedShapeAsset) {
        AssetLibrary::from_domain(domain).add_shape(shape_asset.desc, shape_asset.shape);
    }
    fn update_progress(&mut self, msg: FinishedLoadingMsg) {
        let mut maybe_lm = self.loading_bundles.get_mut(&msg.id);
        let lm = maybe_lm
            .as_mut()
            .expect("FinishedLoading implies LoadScheduler is around");
        match msg.data {
            FinishedLoading::Item(data) => lm.add_progress_boxed(data),
            FinishedLoading::VecItem(data, index) => lm.add_vec_progress(data, index),
        }
    }
    // After new progress has been reported, perform actions and send notifications as necessary.
    fn after_progress(&mut self, msg: &UpdatedProgressMsg) {
        let maybe_lm = self.loading_bundles.get_mut(&msg.id);
        if let Some(lm) = maybe_lm {
            if lm.done() {
                debug_println!("Loading done");
                let lm = self.loading_bundles.remove(&msg.id).unwrap();
                let loadables = lm.loadables;
                let loaded = LoadedData { loadables };
                if let Some(f) = lm.post_loading {
                    f(loaded);
                };
                nuts::publish(LoadingDoneMsg { id: msg.id });
            } else {
                lm.send_progress_message();
            }
        }
    }
}
