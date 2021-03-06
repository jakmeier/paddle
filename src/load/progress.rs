use core::any::TypeId;
use std::{any::Any, collections::HashMap, future::Future};

use nuts::LifecycleStatus;

use crate::{Domain, ErrorMessage, PaddleResult};

/// Helper object to manage resource loading.
///
/// ## Usage
///     1. Register a set of futures that will load the resources
///     2. Call `attach_to_domain`, which will consume the `LoadScheduler`
///     3. (optional)  Subscribe to `LoadingProgress` event
///     4. Subscribe to `LoadingDone` event and extract resources from the `LoadedData` object in the domain
///
/// ## Nuts Events published
/// **LoadingProgress**: Published on every resource that has been loaded, including the relative progress and the message of a currently loaded resource
///
/// **LoadingDone**: Published once when the last resource finished loading
///
#[derive(Default)]
pub struct LoadScheduler {
    total_items: usize,
    loaded: usize,
    loadables: HashMap<TypeId, Loadable>,
    load_activity: Option<nuts::ActivityId<LoadActivity>>,
}

#[derive(Default)]
pub struct LoadedData {
    loadables: HashMap<TypeId, Loadable>,
}
#[derive(Copy, Clone)]
pub struct LoadingProgress(f32, &'static str);
#[derive(Copy, Clone)]
pub struct LoadingDone;
#[derive(Copy, Clone)]
struct UpdatedProgress;

enum FinishedLoading {
    Item(Box<dyn Any>),
    VecItem(Box<dyn Any>, usize),
}
struct LoadActivity;

impl LoadActivity {
    fn update_progress(&mut self, domain: &mut nuts::DomainState, msg: FinishedLoading) {
        let maybe_lm: &mut Option<LoadScheduler> = domain.get_mut();
        let lm = maybe_lm
            .as_mut()
            .expect("FinishedLoading implies LoadScheduler is around");
        match msg {
            FinishedLoading::Item(data) => lm.add_progress_boxed(data),
            FinishedLoading::VecItem(data, index) => lm.add_vec_progress(data, index),
        }
    }
    pub fn finish_if_done(&mut self, domain: &mut nuts::DomainState, _msg: &UpdatedProgress) {
        let maybe_lm: &mut Option<LoadScheduler> = domain.get_mut();
        if let Some(lm) = maybe_lm {
            if lm.done() {
                debug_println!("Loading done");
                let data = lm.finish();
                domain.store(data);
                nuts::publish(LoadingDone);
            }
        }
    }
}

impl LoadScheduler {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn attach_to_domain(mut self) {
        let aid = nuts::new_domained_activity(LoadActivity, &Domain::Frame);
        self.load_activity = Some(aid);
        nuts::store_to_domain(&Domain::Frame, Some(self));
        aid.private_domained_channel(LoadActivity::update_progress);
        aid.subscribe_domained(LoadActivity::finish_if_done);
    }

    /// Register a future to be loaded.
    ///
    /// The future will be spawned and automatically reporting to the load scheduler is set up for afterwards.
    pub fn register<T: Sized + 'static, F: Future<Output = T> + 'static>(
        &mut self,
        future: F,
        msg: &'static str,
    ) {
        let key = TypeId::of::<T>();
        self.loadables.insert(key, Loadable::new::<T>(msg));
        self.total_items += 1;

        let outer_future = async {
            let resource = future.await;
            let data = Box::new(resource);
            let msg = FinishedLoading::Item(data);
            nuts::send_to::<LoadActivity, _>(msg);
        };
        wasm_bindgen_futures::spawn_local(outer_future);
    }

    /// Same as `register` but for a vector of futures.
    ///
    /// The `LoadedData` will contain a vector with the values loaded in the exact same order.
    pub fn register_vec<T: Sized + 'static, F: Future<Output = T> + 'static>(
        &mut self,
        future_vec: Vec<F>,
        msg: &'static str,
    ) {
        let n = future_vec.len();
        let key = TypeId::of::<T>();
        self.loadables.insert(key, Loadable::new_vec::<T>(n, msg));
        self.total_items += n;

        for (i, future) in future_vec.into_iter().enumerate() {
            let outer_future = async move {
                let resource = future.await;
                let data = Box::new(resource);
                let msg = FinishedLoading::VecItem(data, i);
                nuts::send_to::<LoadActivity, _>(msg);
            };
            wasm_bindgen_futures::spawn_local(outer_future);
        }
    }

    /// Register a data type that needs to be loaded.
    ///
    /// When this function is used, the data has to be manually added using `add_progress`.
    /// It can then be later extracted from LoadedData just like data registers as futures.
    ///
    /// This is useful when some of data to load does not use a future.
    pub fn register_manually_reported<T: Any>(&mut self, msg: &'static str) {
        let key = TypeId::of::<T>();
        self.loadables.insert(key, Loadable::new::<T>(msg));
        self.total_items += 1;
    }

    /// Builder pattern for `register`
    pub fn with<T: Any, F: Future<Output = T> + 'static>(
        mut self,
        future: F,
        msg: &'static str,
    ) -> Self {
        self.register::<T, F>(future, msg);
        self
    }
    /// Builder pattern for `register_vec`
    pub fn with_vec<T: Any, F: Future<Output = T> + 'static>(
        mut self,
        futures: Vec<F>,
        msg: &'static str,
    ) -> Self {
        self.register_vec::<T, F>(futures, msg);
        self
    }
    /// Builder pattern for `register_manually_reported`
    pub fn with_manually_reported<T: Any>(mut self, msg: &'static str) -> Self {
        self.register_manually_reported::<T>(msg);
        self
    }

    /// Update the progress tracking with a new resource
    pub fn add_progress_boxed(&mut self, loaded: Box<dyn Any>) {
        self.add_vec_progress(loaded, 0)
    }

    /// Update the progress tracking with a new resource.
    /// The resource will be put in a box, so if you already have one, just
    /// use add_progress_boxed to avoid an extra reboxing.
    pub fn add_progress<T: Any>(&mut self, loaded: T) {
        self.add_vec_progress(Box::new(loaded), 0)
    }
    /// Update the progress tracking with a new resource
    pub fn add_vec_progress(&mut self, loaded: Box<dyn Any>, index: usize) {
        let key = (*loaded).type_id();
        let loadable = self.loadables.get_mut(&key).expect("Unknown loadable"); // TODO: error handling
        if loadable.data[index].is_none() {
            loadable.data[index] = Some(loaded); // TODO: index checks
            self.loaded += 1;
        } else if loadable.data.len() == 1 {
            #[cfg(debug_assertions)]
            panic!("Already loaded {}", loadable.name);
            #[cfg(not(debug_assertions))]
            panic!("Already loaded {:?}", key);
        } else {
            #[cfg(debug_assertions)]
            panic!("Already loaded Vec<{}> index [{}]", loadable.name, index);
            #[cfg(not(debug_assertions))]
            panic!("Already loaded Vec<{:?}> index [{}]", key, index);
        }
        nuts::publish(UpdatedProgress);
    }
    /// Reports relative loading progress between 0.0 and 1.0
    pub fn progress(&self) -> f32 {
        self.loaded as f32 / self.total_items as f32
    }
    /// Returns true iff all registered resources are fully loaded.
    pub fn done(&self) -> bool {
        self.loaded >= self.total_items
    }
    /// If there are resources left to be loaded, returns the loading message of one of them.
    pub fn waiting_for(&self) -> Option<&'static str> {
        for loadable in self.loadables.values() {
            if loadable.capacity > loadable.loaded {
                return Some(loadable.msg);
            }
        }
        None
    }

    pub fn finish(&mut self) -> LoadedData {
        if let Some(aid) = self.load_activity {
            aid.set_status(LifecycleStatus::Inactive);
        }

        let loadables = std::mem::take(&mut self.loadables);
        LoadedData { loadables }
    }
}

struct Loadable {
    msg: &'static str,
    capacity: usize,
    loaded: usize,
    data: Vec<Option<Box<dyn Any>>>,
    #[cfg(debug_assertions)]
    name: &'static str,
}

impl Loadable {
    fn new<T: Any>(msg: &'static str) -> Self {
        Loadable::new_vec::<T>(1, msg)
    }
    fn new_vec<T: Any>(max: usize, msg: &'static str) -> Self {
        let mut data = Vec::<Option<Box<dyn Any>>>::with_capacity(max);
        for _ in 0..max {
            data.push(None);
        }
        Loadable {
            msg,
            capacity: max,
            loaded: 0,
            data,
            #[cfg(debug_assertions)]
            name: std::any::type_name::<T>(),
        }
    }
}

impl LoadedData {
    /// Take ownership of a loaded resource
    pub fn extract<T: Any>(&mut self) -> PaddleResult<Box<T>> {
        let key = TypeId::of::<T>();
        let mut loadable = self.loadables.remove(&key).ok_or_else(|| {
            ErrorMessage::technical(format!(
                "Tried to extract data type that has not been registered for loading: {}",
                std::any::type_name::<T>()
            ))
        })?;
        let resource = loadable.data.pop().flatten().ok_or_else(|| {
            ErrorMessage::technical(format!(
                "Data not available. Either not loaded or already extracted: {}",
                std::any::type_name::<T>()
            ))
        })?;
        Ok(resource.downcast().unwrap())
    }
    /// Take ownership of a loaded resource vector
    pub fn extract_vec<T: Any>(&mut self) -> PaddleResult<Vec<T>> {
        let key = TypeId::of::<T>();
        let loadable = self.loadables.remove(&key).ok_or_else(|| {
            ErrorMessage::technical(format!(
                "Tried to extract data type that has not been registered for loading: Vec<{}>",
                std::any::type_name::<T>()
            ))
        })?;

        let mut out = vec![];
        for resource in loadable.data {
            let r = resource.ok_or_else(|| {
                ErrorMessage::technical(
                    "Data not available. Either not loaded or already extracted.".to_owned(),
                )
            })?;
            out.push(*r.downcast().unwrap());
        }
        Ok(out)
    }
}
