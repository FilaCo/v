use ddd_derive::VO;
use uuid::Uuid;

#[derive(VO)]
pub(crate) struct EffectorId {
    value: Uuid,
}
