use super::ResolverMut;
use anyhow::Result;
use std::fmt;
use tokio::sync::mpsc;

pub async fn resolve_receiver<T, M>(
    mut receiver: mpsc::UnboundedReceiver<T>,
    mut matcher: M,
) -> Result<()>
where
    T: 'static + Send + Sync + fmt::Debug,
    M: 'static + Send + Sync + fmt::Debug + ResolverMut<T>,
{
    while let Some(event) = receiver.recv().await {
        matcher.resolve_on(event)?;
    }

    //@TODO: Figure out how to reattach this receiver to the Room struct

    Ok(())
}
