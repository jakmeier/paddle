use crate::EventType;
use nuts::{Activity, ActivityId, Capsule, DomainState, UncheckedActivityId};

pub(crate) struct RegisterEventListener {
    capsule: Capsule,
    event_type: EventType,
    aid: UncheckedActivityId,
}

/// Connection to events the browser forwards.
/// A single JS EventListener sits on the JS side of things and will call event_from_js(ID) with event IDs.
pub(crate) struct EventGate {
    listeners: Vec<Capsule>,
}

// TODO
// need something in the JS world, maybe with the help of div.
// 1) Each frame needs an HtmlElement
// 2) An event listener singleton has to be placed that can be used for registering new event listeners.
//  !!! How can this work without creating a new closure? The idea is to send a different Listener index for each new registration. I guess the singleton has to also take over the registration process.

impl EventGate {
    pub(crate) fn init() {
        let gate = EventGate { listeners: vec![] };
        let aid = nuts::new_activity(gate);
        aid.subscribe_owned(Self::register_event_listener);
    }
    pub fn listen<A: Activity, F>(id: ActivityId<A>, event_type: EventType, callback: F)
    where
        F: Fn(&mut A, &mut DomainState) + 'static,
    {
        nuts::publish(RegisterEventListener::new(id, event_type, callback));
    }
    fn register_event_listener(&mut self, msg: RegisterEventListener) {
        self.listeners.push(msg.capsule);
        // TODO: handle incoming registration request
    }
}

impl RegisterEventListener {
    fn new<A: Activity, F>(id: ActivityId<A>, event_type: EventType, callback: F) -> Self
    where
        F: Fn(&mut A, &mut DomainState) + 'static,
    {
        Self {
            capsule: id.encapsulate_domained(callback),
            event_type,
            aid: id.into(),
        }
    }
}
