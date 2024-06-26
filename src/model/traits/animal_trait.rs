use crate::model::traits::obj_trait::ObjTrait;

pub(crate) trait AnimalTrait: ObjTrait {
    type Offspring;
    fn get_given_name(&self) -> String;
    fn set_given_name(&mut self, given_name: String);
    fn eat(&mut self, calories: u32) -> bool;
    fn try_reproduce(&mut self) -> Option<Self::Offspring>;
}
