use crate::VO;

pub trait Entity: Eq + PartialEq {
    type Id: VO;

    fn id(&self) -> &Self::Id;
}
