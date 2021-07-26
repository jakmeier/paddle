use core::any::TypeId;
use std::{
    any::Any,
    collections::HashMap,
    future::Future,
    sync::atomic::{AtomicU64, Ordering},
};

use crate::{ErrorMessage, LoadActivity, PaddleResult};

/// Helper object to manage resource loading. This is a more low-level approach than using `AssetBundle` and should rarely be necessary.
///
/// ## Usage
///     1. Register a set of futures that will load the resources
///     2. (optional) Place a closure with `set_after_loading` that receives the data once the scheduler has loaded everything.
///     3. Call `track_loading`, which will consume the `LoadScheduler` and return a tracker ID.
///     4. (optional)  Subscribe to `LoadingProgressMsg` event to receive updates on each loaded item. Use the tracker ID to check if the loaded item was part of
///     5. (optional) Subscribe to `LoadingDone` event and extract resources from the `LoadedData` object in the domain
///
/// ## Nuts Events published
/// **LoadingProgressMsg**: Published on every resource that has been loaded, including the relative progress and the message of a currently loaded resource
///
/// **LoadingDone**: Published once when the last resource finished loading
///
pub struct LoadScheduler {
    pub(crate) id: LoadSchedulerId,
    pub(crate) total_items: usize,
    pub(crate) loaded: usize,
    pub(crate) loadables: HashMap<TypeId, Loadable>,
    pub(crate) post_loading: Option<Box<dyn FnOnce(LoadedData)>>,
}
#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub struct LoadSchedulerId(u64);

#[derive(Default)]
pub struct LoadedData {
    pub(crate) loadables: HashMap<TypeId, Loadable>,
}
#[derive(Copy, Clone)]
pub struct LoadingProgressMsg {
    pub id: LoadSchedulerId,
    pub progress: f32,
    pub description: &'static str,
}
#[derive(Copy, Clone)]
pub struct LoadingDoneMsg {
    pub id: LoadSchedulerId,
}
#[derive(Copy, Clone)]
pub(crate) struct UpdatedProgressMsg {
    pub(crate) id: LoadSchedulerId,
}

pub(crate) struct FinishedLoadingMsg {
    pub(crate) id: LoadSchedulerId,
    pub(crate) data: FinishedLoading,
}
pub(crate) enum FinishedLoading {
    Item(Box<dyn Any>),
    VecItem(Box<dyn Any>, usize),
}

impl LoadScheduler {
    pub fn new() -> Self {
        static ID_COUNTER: AtomicU64 = AtomicU64::new(0);
        LoadScheduler {
            id: LoadSchedulerId(ID_COUNTER.fetch_add(1, Ordering::SeqCst)),
            total_items: 0,
            loaded: 0,
            loadables: HashMap::new(),
            post_loading: None,
        }
    }
    pub fn track_loading(self) -> LoadSchedulerId {
        let id = self.id;
        nuts::send_to::<LoadActivity, _>(self);
        id
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
        nuts::publish(UpdatedProgressMsg { id: self.id });
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

    /// Set the closure to be executed after all loading is done.
    /// The closure has access to `LoadedData`, from which all the data can be extracted.
    pub fn set_after_loading<F>(&mut self, f: F)
    where
        F: FnOnce(LoadedData) + 'static,
    {
        self.post_loading = Some(Box::new(f));
    }
}

pub(crate) struct Loadable {
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
