use super::*;
use crate::{Context, EventGate, EventType, InputState, Key, KeyEventType};
use div::PaneHandle;

pub fn register_frame<F: Frame + Activity>(
    frame: F,
    state: F::State,
    pos: (u32, u32),
    size: (u32, u32),
) -> FrameHandle<F> {
    nuts::store_to_domain(&Domain::Frame, state);
    register_frame_no_state(frame, pos, size)
}

/// Use this if the state has already been registered previously.
pub fn register_frame_no_state<F: Frame + Activity>(
    frame: F,
    pos: (u32, u32),
    size: (u32, u32),
) -> FrameHandle<F> {
    let div = div::new_pane(pos.0, pos.1, size.0, size.1, "").expect("Div failure");
    let activity = nuts::new_domained_activity(frame, &Domain::Frame);
    init_frame_activity(activity, &div);
    let handle =     FrameHandle::new(activity, Some(div), Rectangle::new(pos, size));
    FrameManipulator::init_frame(&handle);
    handle
}

// Helper for checking if default has been overwritten
struct Nop<STATE> {
    _phantom: std::marker::PhantomData<STATE>,
}
impl<STATE> Frame for Nop<STATE> {
    type State = STATE;
}

fn init_frame_activity<F>(activity: ActivityId<F>, div: &PaneHandle)
where
    F: Frame + Activity,
{
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
            let key: Key = input.last_key().expect("Key not prepared");
            let global_state: &mut F::State = d.try_get_mut().expect("Activity State missing");
            a.key(global_state, key)
        })
    }
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
