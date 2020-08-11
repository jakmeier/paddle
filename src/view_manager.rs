use crate::frame::{frame_to_activity, Frame, FrameHandle};
use nuts::{ActivityId, UncheckedActivityId};
use quicksilver::prelude::Window;
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
    // TODO: Does this interface need to be simplified?
    pub fn add_frame<S, E, FRAME>(
        &mut self,
        frame: FRAME,
        views: &[V],
        _pos: (i32, i32),
        _size: (i32, i32),
    ) -> FrameHandle<FRAME>
    where
        FRAME: Frame<State = S, Graphics = Window, Error = E> + nuts::Activity,
    {
        let activity_id: ActivityId<_> = frame_to_activity(frame).into();
        for view in views {
            if view == &self.current_view {
                nuts::set_active(activity_id, view == &self.current_view);
            }
            self.link_activity_to_view(activity_id, *view);
        }
        FrameHandle::new(activity_id)
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
                nuts::set_active(*b, false);
            }
        }
        // activate all in after (activating when already active does nothing)
        for a in after {
            nuts::set_active(*a, true);
        }
        self.current_view = view;
    }
}
