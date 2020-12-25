// use crate::Display;
// use nuts::DomainState;

// use crate::{quicksilver_compat::Rectangle};

// struct FrameInitialization {
//     div: div::PaneHandle,
//     region: Rectangle,
// }

pub(crate) struct FrameManipulator {
    _private: (),
}

impl FrameManipulator {
    pub(crate) fn init() {
        // let fm = FrameManipulator { _private: () };
        // let domain = Domain::Frame;
        // let aid = nuts::new_domained_activity(fm, &domain);

        // This is useless right now. It was here because I thought frames need to resize themselves, however, this is already managed i div.
        // Keeping it for now, it is very likely that frame repositioning will be a hing in the near future. (Resizing otoh is unlikely)
    }
}
