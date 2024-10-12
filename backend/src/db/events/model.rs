use uuid::Uuid;
use yaaytos_common::{Event, EventType};

#[derive(Clone)]
pub struct EventDb {
    pub uuid: Uuid,
    pub season_uuid: Uuid,
    pub order: i32,
}

impl From<EventDb> for Event {
    fn from(value: EventDb) -> Self {
        Event {
            uuid: value.uuid.to_string(),
            season_uuid: value.season_uuid.to_string(),
            order: value.order,
            r#type: EventType::Start,
        }
    }
}
