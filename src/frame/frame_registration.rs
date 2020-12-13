use super::*;
use crate::{
    Context, EventGate, EventListenerType, KeyEventType, LeftClick, MouseEventType, RightClick,
};

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
    let area = Rectangle::new(pos, size);
    let handle = FrameHandle::new(activity, Some(div), area);
    handle.init_frame_activity();
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

impl<STATE: 'static, F: Frame<State = STATE> + Activity> FrameHandle<F> {
    fn init_frame_activity(&self) {
        let activity = self.activity();
        let area = self.region();
        if (F::update as usize) != (Nop::<F::State>::update as usize) {
            activity.subscribe_domained(|a, d, _msg: &UpdateWorld| {
                let global_state: &mut F::State = d.try_get_mut().expect("Activity State missing");
                a.update(global_state)
            });
        }
        if (F::draw as usize) != (Nop::<F::State>::draw as usize) {
            activity.subscribe_domained(move |a: &mut F, d: &mut DomainState, msg: &DrawWorld| {
                let (global_state, ctx) = d.try_get_2_mut::<F::State, Context>();
                let canvas = ctx.expect("Context missing").display_region(area);
                a.draw(
                    global_state.expect("Activity State missing"),
                    canvas,
                    msg.time_ms,
                )
            });
        }
        if (F::left_click as usize) != (Nop::<F::State>::left_click as usize) {
            activity.private_domained_channel(|a, d, msg: LeftClick| {
                let (global_state, ctx) = d.try_get_2_mut::<F::State, Context>();
                let global_state: &mut F::State = global_state.expect("Activity State missing");
                let display = ctx.expect("Context missing").display.full();
                let click = msg.pos / display.browser_to_game_pixel_ratio();
                a.left_click(global_state, (click.x as i32, click.y as i32))
            });
            EventGate::listen(
                self,
                EventListenerType::Mouse(vec![MouseEventType::LeftClick]),
            )
        }
        if (F::right_click as usize) != (Nop::<F::State>::right_click as usize) {
            activity.private_domained_channel(|a, d, msg: RightClick| {
                let (global_state, ctx) = d.try_get_2_mut::<F::State, Context>();
                let global_state: &mut F::State = global_state.expect("Activity State missing");
                let display = ctx.expect("Context missing").display.full();
                let click = msg.pos / display.browser_to_game_pixel_ratio();
                a.right_click(global_state, (click.x as i32, click.y as i32))
            });
            EventGate::listen(
                self,
                EventListenerType::Mouse(vec![MouseEventType::RightClick]),
            )
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
            activity.private_domained_channel(|a, d, msg: KeyEvent| {
                let global_state = d.try_get_mut::<F::State>().expect("Activity State missing");
                a.key(global_state, msg)
            });
            EventGate::listen(
                self,
                EventListenerType::KeyBoard(vec![
                    KeyEventType::KeyPress,
                    KeyEventType::KeyUp,
                    KeyEventType::KeyDown,
                ]),
            )
        }
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
