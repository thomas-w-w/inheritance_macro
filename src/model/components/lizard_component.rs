#[derive(Clone, Debug)]
pub(crate) struct LizardComponent {}

impl LizardComponent {
    pub(crate) fn crawl(&self) {
        println!("LizardArchetype::crawl");
    }
}
