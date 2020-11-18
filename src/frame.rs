use crate::{canvas::WebGLCanvas, Context, EventGate, EventType, InputState, Key, KeyEventType};
use nuts::*;

mod scheduling;
pub use scheduling::*;

/// A frame takes up some area on the screen where it is drawn and reacts to UI events
pub trait Frame {
    type State;
    fn draw(
        &mut self,
        _state: &mut Self::State,
        _canvas: &mut WebGLCanvas, // TODO: This should be some "DrawArea" or whatever, some handle to just the area the frame uses
        _timestamp: f64,
    ) {
    }
    fn update(&mut self, _state: &mut Self::State) {}
    fn leave(&mut self, _state: &mut Self::State) {}
    fn enter(&mut self, _state: &mut Self::State) {}
    fn left_click(&mut self, _state: &mut Self::State, _pos: (i32, i32)) {}
    fn right_click(&mut self, _state: &mut Self::State, _pos: (i32, i32)) {}
    fn key(&mut self, _state: &mut Self::State, _key: Key) {}
    // TODO:
    // fn browser_event(&mut self, _state: &mut Self::State, event_info: SomeBrowserEventInfoStruct) {}
}

/// Handle to frame is returned when adding it to the view manager.
/// Can be used to add listeners. (And potentially other manipulations of the frame are possible in the future)
pub struct FrameHandle<FRAME> {
    activity_id: ActivityId<FRAME>,
    //     #[allow(dead_code)]
    //     pos: (i32, i32),
    //     #[allow(dead_code)]
    //     size: (i32, i32),
}

impl<FRAME> FrameHandle<FRAME> {
    pub fn new(activity_id: ActivityId<FRAME>) -> Self {
        Self { activity_id }
    }
}

#[derive(Clone, Copy)]
pub enum Domain {
    Frame,
    Network,
}
domain_enum!(Domain);

/// Goes to active and inactive frames
struct GlobalEvent<Ev>(pub(crate) Ev);
/// Goes to active frames only
struct ActiveEvent<Ev>(pub(crate) Ev);

/// Share anything with all other activities in background and foreground
pub fn share<MSG: 'static>(msg: MSG) {
    nuts::publish(GlobalEvent(msg));
}

/// Share anything with all foreground activities
pub fn share_foreground<MSG: 'static>(msg: MSG) {
    nuts::publish(ActiveEvent(msg));
}

pub fn register_frame<F: Frame + Activity>(frame: F, state: F::State) -> ActivityId<F> {
    nuts::store_to_domain(&Domain::Frame, state);
    frame_to_activity(frame, &Domain::Frame)
}

// Helper for checking if default has been overwritten
struct Nop<STATE> {
    _phantom: std::marker::PhantomData<STATE>,
}
impl<STATE> Frame for Nop<STATE> {
    type State = STATE;
}

pub fn frame_to_activity<F, D: DomainEnumeration>(frame: F, domain: &D) -> ActivityId<F>
where
    F: Frame + Activity,
{
    let activity = nuts::new_domained_activity(frame, domain);

    if (F::update as usize) != (Nop::<F::State>::update as usize) {
        activity.subscribe_domained(|a, d, _msg: &UpdateWorld| {
            let global_state: &mut F::State = d.try_get_mut().expect("Activity State missing");
            a.update(global_state)
        });
    }
    if (F::draw as usize) != (Nop::<F::State>::draw as usize) {
        activity.subscribe_domained(|a: &mut F, d: &mut DomainState, msg: &DrawWorld| {
            let (global_state, ctx) = d.try_get_2_mut::<F::State, Context>();
            let canvas = ctx.expect("Context missing").canvas_mut();
            a.draw(
                global_state.expect("Activity State missing"),
                canvas,
                msg.time_ms,
            )
        });
    }
    if (F::left_click as usize) != (Nop::<F::State>::left_click as usize) {
        activity.subscribe_domained(|a, d, msg: &LeftClick| {
            let global_state: &mut F::State = d.try_get_mut().expect("Activity State missing");
            a.left_click(global_state, msg.pos)
        });
    }
    if (F::right_click as usize) != (Nop::<F::State>::right_click as usize) {
        activity.subscribe_domained(|a, d, msg: &RightClick| {
            let global_state: &mut F::State = d.try_get_mut().expect("Activity State missing");
            a.right_click(global_state, msg.pos)
        });
    }
    if (F::enter as usize) != (Nop::<F::State>::enter as usize) {
        activity.on_enter_domained(|a, d| {
            let global_state: &mut F::State = d.try_get_mut().expect("Activity State missing");
            a.enter(global_state)
        });
    }
    if (F::leave as usize) != (Nop::<F::State>::leave as usize) {
        activity.on_leave_domained(|a, d| {
            let global_state: &mut F::State = d.try_get_mut().expect("Activity State missing");
            a.leave(global_state)
        });
    }
    if (F::key as usize) != (Nop::<F::State>::key as usize) {
        EventGate::listen(activity, EventType::Key(KeyEventType::KeyDown), |a, d| {
            let input = InputState::from_domain(d);
            let key: Key = input.last_key().expect("Ky not prepared");
            let global_state: &mut F::State = d.try_get_mut().expect("Activity State missing");
            a.key(global_state, key)
        })
    }
    activity
}

impl<STATE: 'static, FRAME: Frame<State = STATE>> FrameHandle<FRAME> {
    pub fn listen<F, MSG>(&self, f: F)
    where
        F: Fn(&mut FRAME, &mut STATE, &MSG) + Copy + 'static,
        MSG: 'static,
        FRAME: 'static,
    {
        self.activity_id.subscribe_domained_masked(
            SubscriptionFilter::no_filter(),
            move |a, d, msg: &GlobalEvent<MSG>| {
                let global_state: &mut FRAME::State =
                    d.try_get_mut().expect("Activity State missing");
                f(a, global_state, &msg.0);
            },
        );

        self.activity_id
            .subscribe_domained(move |a, d, msg: &ActiveEvent<MSG>| {
                let global_state: &mut FRAME::State =
                    d.try_get_mut().expect("Activity State missing");
                f(a, global_state, &msg.0);
            });
    }
}
