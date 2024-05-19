use crate::model::traits::obj_trait::ObjTrait;

pub(crate) trait FoodTrait: ObjTrait {
    fn eat(&mut self, chunk: i32) -> bool;
}
