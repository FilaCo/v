use crate::domain::vo::SensorId;
use ddd_derive::Entity;

#[derive(Entity)]
pub(crate) struct Sensor {
    #[entity(id)]
    id: SensorId,
}
