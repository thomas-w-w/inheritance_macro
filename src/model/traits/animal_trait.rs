use crate::model::traits::obj_trait::ObjTrait;

pub(crate) trait AnimalTrait: ObjTrait {
    type Offspring;
    fn eat(&mut self, calories: u32);
    fn try_reproduce(&mut self) -> Option<Self::Offspring>;
}
