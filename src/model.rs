
use sqlx::FromRow;


// pub struct Api {
//     // pub addons: Vec<Addon>,
//     pub games: Vec<Game>,
//     // pub categories: Vec<Category>,
// }

// impl Api {
//     pub fn new() -> Self {
//         Self {
//             games: vec![Game {
//                 id: Uuid::new_v4().into(),
//                 name: "WoW".to_string(),
//                 version: "Retail".to_string(),
//                 categories: vec![Category {
//                     id: Uuid::new_v4().into(),
//                     name: "Mining".to_string(),
//                     addons: vec![
//                         Addon {
//                             id: Uuid::new_v4().into(),
//                             name: "Foo".to_string(),
//                         },
//                         Addon {
//                             name: "Bar".to_string(),
//                             id: Uuid::new_v4().into(),
//                         },
//                     ],
//                 }],
//             }],
//         }
//     }
// }

#[derive(Debug, Clone, FromRow)]
pub struct AddonRecord {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct CategoryRecord {
    pub id: i64,
    pub name: String,
    pub game: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct GameRecord {
    pub id: i64,
    pub name: String,
}
