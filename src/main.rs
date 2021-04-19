use anyhow::Result;

fn main() -> Result<()> {
    serde_edit::start()?;
    Ok(())
}
