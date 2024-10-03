use stremio_core::runtime::msg::Event;

pub enum AppleEvent {
    CoreEvent(Event),
}
