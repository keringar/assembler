// This module contains stuff relating to ecs resources
// DeltaTime, OS events, texture loading... etc.
// Everything here should be presented as a resource for other
// systems to consume.

pub mod events;
pub use self::events::{EventTypes, Events};

mod time;
pub use self::time::Time;

