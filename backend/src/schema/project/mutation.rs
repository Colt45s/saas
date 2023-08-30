use async_graphql::Object;

#[derive(Default)]
pub struct ProjectMutation {}

#[Object]
impl ProjectMutation {
    async fn hello(&self) -> String {
        "Hello, world!".to_string()
    }
}
