#[async_trait::async_trait]
pub trait RunCommand {
    async fn run(&self) -> anyhow::Result<()>;
}
