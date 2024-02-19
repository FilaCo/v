use crate::domain::vo::EffectorId;
use ddd_derive::Entity;

#[derive(Entity)]
pub(crate) struct Effector {
    #[entity(id)]
    id: EffectorId,
}
