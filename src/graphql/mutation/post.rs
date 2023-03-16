use async_graphql::{Context, InputObject, Object, Result};
use serde::Deserialize;

use crate::{
    graphql::types::Post,
    prisma::{user, PrismaClient},
};

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(Deserialize, InputObject)]
pub struct CreatePostInput {
    pub content: String,
    // this really would be grabbed from session or something but just for demo
    pub user_id: String,
}

#[derive(Default)]
pub struct PostMutation;

#[Object]
impl PostMutation {
    pub async fn create_post(&self, ctx: &Context<'_>, input: CreatePostInput) -> Result<Post> {
        let db = ctx.data::<PrismaClient>().unwrap();

        let created = db
            .post()
            .create(
                input.content.to_string(),
                user::id::equals(input.user_id.to_string()),
                vec![],
            )
            .exec()
            .await?;

        Ok(created.into())
    }
}