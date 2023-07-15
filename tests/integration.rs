use anyhow::Result;
use std::error::Error;

#[tokio::test]
async fn get_hello() -> Result<(), Box<dyn Error>> {
    let client = httpc_test::new_client("http://localhost:3000")?;

    let actual = client.do_get("/").await?.text_body()?;
    assert_eq!(actual, "Hello World!");

    Ok(())
}
