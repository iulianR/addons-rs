use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use serde::{Deserialize, Serialize};

use crate::model::Addons;

pub type AddonsSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(Serialize, Deserialize)]
struct Addon {
    name: String,
}

#[Object]
impl Addon {
    async fn name(&self) -> &String {
        &self.name
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn addons(&self, ctx: &Context<'_>) -> Vec<Addon> {
        let addons: &Addons = ctx.data().unwrap();
        addons
            .addons
            .iter()
            .map(|a| Addon {
                name: a.name.clone(),
            })
            .collect()
    }
}
