use stremio_core::runtime::msg::{Action, Event};

pub enum AndroidEvent {
    CoreAction(Box<Action>),
    CoreEvent(Event),
}
