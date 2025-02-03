use async_graphql::{EmptyMutation, EmptySubscription, InputObject, Object, Schema};

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
    let res = schema.execute(QUERY).await;
    let out = res.data.into_json().unwrap();
    println!("{out}");
}

struct Query;

#[derive(InputObject)]
struct Bar {
    num: i32,
}

#[Object]
impl Query {
    async fn foo(&self, #[graphql(deprecation = "deprecated")] bar: Bar) -> i32 {
        420
    }
}
