use crate::{
    frame::{Frame, FrameHandle},
    register_frame_no_state,
};
use nuts::{LifecycleStatus::Active, LifecycleStatus::Inactive, UncheckedActivityId};
use std::collections::HashMap;
use std::hash::Hash;

// Switches between views by activating and deactivating activities
pub struct ViewManager<V> {
    views_to_activities: HashMap<V, Vec<UncheckedActivityId>>,
    current_view: V,
}

impl<V: Hash + Eq + Copy> ViewManager<V> {
    pub fn new(v: V) -> Self {
        Self {
            views_to_activities: HashMap::new(),
            current_view: v,
        }
    }

    pub fn link_activity_to_view(&mut self, aid: impl Into<UncheckedActivityId>, view: V) {
        self.views_to_activities
            .entry(view)
            .or_default()
            .push(aid.into());
    }
    /// Activity with position and associated view(s)
    pub fn add_frame<S, FRAME>(
        &mut self,
        frame: FRAME,
        views: &[V],
        pos: (u32, u32),
    ) -> FrameHandle<FRAME>
    where
        FRAME: Frame<State = S> + nuts::Activity,
    {
        let handle = register_frame_no_state(frame, pos);
        let activity_id = handle.activity();

        let div_copy = handle.div().clone();
        activity_id.on_enter(move |_| div_copy.show().expect("Div failure"));
        let div_copy = handle.div().clone();
        activity_id.on_leave(move |_| div_copy.hide().expect("Div failure"));

        let mut status = Inactive;
        for view in views {
            if view == &self.current_view {
                status = Active;
            };
            self.link_activity_to_view(activity_id, *view);
        }
        activity_id.set_status(status);
        handle
    }
    pub fn set_view(&mut self, view: V) {
        if self.current_view == view {
            return;
        }
        let _before = self
            .views_to_activities
            .entry(self.current_view)
            .or_default();
        let _after: &Vec<_> = self.views_to_activities.entry(view).or_default();
        let after = &self.views_to_activities[&view];
        let before = &self.views_to_activities[&self.current_view];
        // deactivate all in before that are not in after
        for b in before {
            if !after.iter().any(|a| a == b) {
                b.set_status(Inactive);
            }
        }
        // activate all in after (activating when already active does nothing)
        for a in after {
            a.set_status(Active);
        }
        self.current_view = view;
    }
}
