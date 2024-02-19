use v::prelude::v1::*;

#[tokio::main]
async fn main() -> VResult<()> {
    V::new().run().await
}
