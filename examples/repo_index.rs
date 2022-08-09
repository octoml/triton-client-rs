use triton_client::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // un-auth'd use of Triton
    let client = Client::new("http://localhost:8001/", None).await?;
    let models = client
        .repository_index(triton_client::inference::RepositoryIndexRequest {
            repository_name: "".into(), // This should show us models not referenced by repo name.
            ready: false,               // show all models, not just ready ones.
        })
        .await?;

    println!("Running models:");

    for model in models.models.iter() {
        println!("    {:?}", model);
    }

    Ok(())
}
