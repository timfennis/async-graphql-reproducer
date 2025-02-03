use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

const QUERY: &'static str = r#"
{
  __schema {
    queryType {
      fields {
        name
        isDeprecated
        args {
          name
          isDeprecated
        }
      }
    }
  }
}
"#;

#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    // The deprecated field does show up here
    println!("{}", schema.sdl());

    let res = schema.execute(QUERY).await;
    let out = res.data.into_json().unwrap();

    // The deprecated field does not show up here
    println!("{out}");
}

struct Query;

#[Object]
impl Query {
    async fn foo(&self, #[graphql(deprecation = "deprecated")] bar: i32) -> i32 {
        bar
    }
}
