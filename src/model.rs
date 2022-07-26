pub struct Addons {
    pub addons: Vec<AddonEntity>,
}

impl Addons {
    pub fn new() -> Self {
        Self {
            addons: vec![
                AddonEntity {
                    name: "Foo".to_string(),
                },
                AddonEntity {
                    name: "Bar".to_string(),
                },
            ],
        }
    }
}

pub struct AddonEntity {
    pub name: String,
}
