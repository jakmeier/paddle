use std::collections::HashMap;

use nuts::DomainState;

use crate::{
    AssetLibrary, Domain, FinishedLoading, FinishedLoadingMsg, Image, ImageDesc, LoadScheduler,
    LoadSchedulerId, LoadedData, LoadingDoneMsg, UpdatedProgressMsg,
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

impl LoadActivity {
    pub(crate) fn init() {
        let activity = LoadActivity {
            loading_bundles: HashMap::new(),
        };
        let aid = nuts::new_domained_activity(activity, &Domain::Frame);
        aid.private_channel(LoadActivity::add_scheduler);
        aid.private_channel(LoadActivity::update_progress);
        aid.private_domained_channel(LoadActivity::image_to_asset_library);
        aid.subscribe(LoadActivity::finish_if_done);
    }
    fn add_scheduler(&mut self, msg: LoadScheduler) {
        self.loading_bundles.insert(msg.id, msg);
    }
    fn image_to_asset_library(&mut self, domain: &mut DomainState, img_asset: LoadedImageAsset) {
        AssetLibrary::from_domain(domain).add_image(img_asset.desc, img_asset.img);
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
    fn finish_if_done(&mut self, msg: &UpdatedProgressMsg) {
        let maybe_lm = self.loading_bundles.get_mut(&msg.id);
        let mut done = false;
        if let Some(lm) = maybe_lm {
            done = lm.done();
        }
        if done {
            debug_println!("Loading done");
            let lm = self.loading_bundles.remove(&msg.id).unwrap();
            let loadables = lm.loadables;
            let loaded = LoadedData { loadables };
            if let Some(f) = lm.post_loading {
                f(loaded);
            };
            nuts::publish(LoadingDoneMsg { id: msg.id });
        }
    }
}
