use nuts::*;
use quicksilver::lifecycle::Event;
use quicksilver::prelude::Window;

/// A frame takes up some area on the screen where it is drawn and reacts to UI events
pub trait Frame {
    type Error;
    type State;
    type Graphics;
    fn draw(
        &mut self,
        _state: &mut Self::State,
        _graphics: &mut Self::Graphics,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn update(&mut self, _state: &mut Self::State) -> Result<(), Self::Error> {
        Ok(())
    }
    fn left_click(
        &mut self,
        _state: &mut Self::State,
        _pos: (i32, i32),
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn right_click(
        &mut self,
        _state: &mut Self::State,
        _pos: (i32, i32),
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn leave(&mut self, _state: &mut Self::State) -> Result<(), Self::Error> {
        Ok(())
    }
    fn enter(&mut self, _state: &mut Self::State) -> Result<(), Self::Error> {
        Ok(())
    }
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
    Main, //  TODO: rename to Paddle
}
domain_enum!(Domain);

pub struct LeftClick {
    pub pos: (i32, i32),
}
pub struct RightClick {
    pub pos: (i32, i32),
}

pub struct UpdateWorld {
    window: *mut Window,
}
pub struct DrawWorld {
    window: *mut Window,
}
pub struct WorldEvent {
    window: *mut Window,
    event: Event,
}
impl UpdateWorld {
    pub fn new(window: &mut Window) -> Self {
        Self {
            window: window as *mut Window,
        }
    }
    pub fn window(&mut self) -> &mut Window {
        unsafe { self.window.as_mut().unwrap() }
    }
}
impl DrawWorld {
    pub fn new(window: &mut Window) -> Self {
        Self {
            window: window as *mut Window,
        }
    }
    pub fn window(&mut self) -> &mut Window {
        unsafe { self.window.as_mut().unwrap() }
    }
}
impl WorldEvent {
    pub fn new(window: &mut Window, event: &Event) -> Self {
        Self {
            window: window as *mut Window,
            event: event.clone(),
        }
    }
    pub fn window(&mut self) -> &mut Window {
        unsafe { self.window.as_mut().unwrap() }
    }
    pub fn event(&self) -> Event {
        self.event.clone()
    }
}

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

pub fn frame_to_activity<F>(frame: F) -> ActivityId<F>
where
    F: Frame<Graphics = Window> + Activity,
{
    let activity = nuts::new_domained_activity(frame, Domain::Main, false);

    activity.subscribe_domained(|a, d, _msg: &UpdateWorld| {
        let global_state: &mut F::State = d.try_get_mut().expect("Global state missing");
        if let Err(e) = a.update(global_state) {
            nuts::publish(e);
        }
    });

    activity.subscribe_domained_mut(|a: &mut F, d: &mut DomainState, msg: &mut DrawWorld| {
        let global_state: &mut F::State = d.try_get_mut().expect("Global state missing");
        let window = msg.window();
        if let Err(e) = a.draw(global_state, window) {
            nuts::publish(e);
        }
    });

    activity.subscribe_domained(|a, d, msg: &LeftClick| {
        let global_state: &mut F::State = d.try_get_mut().expect("Global state missing");
        if let Err(e) = a.left_click(global_state, msg.pos) {
            nuts::publish(e);
        }
    });

    activity.subscribe_domained(|a, d, msg: &RightClick| {
        let global_state: &mut F::State = d.try_get_mut().expect("Global state missing");
        if let Err(e) = a.right_click(global_state, msg.pos) {
            nuts::publish(e);
        }
    });

    activity.on_enter_domained(|a, d| {
        let global_state: &mut F::State = d.try_get_mut().expect("Global state missing");
        if let Err(e) = a.enter(global_state) {
            nuts::publish(e);
        }
    });

    activity.on_leave_domained(|a, d| {
        let global_state: &mut F::State = d.try_get_mut().expect("Global state missing");
        if let Err(e) = a.leave(global_state) {
            nuts::publish(e);
        }
    });

    activity
}

impl<STATE, FRAME: Frame<State = STATE>> FrameHandle<FRAME> {
    // FIXME: Declaring Error and State (redundantly) on each event handler is annoying and can lead to complex error messages.
    pub fn listen<F, MSG>(&self, f: F)
    where
        F: Fn(&mut FRAME, &mut STATE, &MSG) -> Result<(), FRAME::Error> + Copy + 'static,
        MSG: 'static,
        FRAME: 'static,
        STATE: 'static,
    {
        self.activity_id.subscribe_domained_masked(
            SubscriptionFilter::no_filter(),
            move |a, d, msg: &GlobalEvent<MSG>| {
                let global_state: &mut FRAME::State =
                    d.try_get_mut().expect("Global state missing");
                let err = f(a, global_state, &msg.0);
                if let Err(e) = err {
                    nuts::publish(e);
                }
            },
        );

        self.activity_id
            .subscribe_domained(move |a, d, msg: &ActiveEvent<MSG>| {
                let global_state: &mut FRAME::State =
                    d.try_get_mut().expect("Global state missing");
                let err = f(a, global_state, &msg.0);
                if let Err(e) = err {
                    nuts::publish(e);
                }
            });
    }
}