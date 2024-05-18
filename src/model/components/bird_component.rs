#[derive(Clone, Debug)]
pub(crate) struct BirdComponent {}
impl BirdComponent {
    pub(crate) fn peep(&self) {
        println!("BirdArchetype::peep");
    }
}
