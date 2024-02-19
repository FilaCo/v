use crate::{Entity, VO};

pub trait Aggregate: Entity {
    type Event: VO;
}
