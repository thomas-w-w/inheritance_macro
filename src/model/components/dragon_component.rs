#[derive(Debug, Clone)]
pub(crate) struct DragonComponent {
    pub(crate) etanol_liters: u32,
}

impl DragonComponent {
    pub(crate) fn fire(&self) {
        println!("DragonArchetype::fire");
    }
}
