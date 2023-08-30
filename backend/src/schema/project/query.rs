use async_graphql::Object;

#[derive(Default)]
pub struct ProjectQuery {}

#[Object]
impl ProjectQuery {
    async fn hello(&self) -> String {
        "Hello, world!".to_string()
    }
}
