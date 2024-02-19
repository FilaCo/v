use v::prelude::v0::*;

#[tokio::main]
async fn main() -> VResult<()> {
    V::new().run().await
}
