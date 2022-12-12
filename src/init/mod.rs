mod tracing;

use anyhow::Result;

pub fn init() -> Result<()> {
    tracing::init_tracing()?;

    Ok(())
}
