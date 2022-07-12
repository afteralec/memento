use super::ResolverMut;
use anyhow::Result;
use std::fmt;
use tokio::sync::mpsc;

pub async fn resolve_receiver<T, R>(
    mut receiver: mpsc::UnboundedReceiver<T>,
    mut resolver: R,
) -> Result<()>
where
    T: 'static + Send + Sync + fmt::Debug,
    R: 'static + Send + Sync + fmt::Debug + ResolverMut<T>,
{
    while let Some(event) = receiver.recv().await {
        resolver.resolve_on(event)?;
    }

    //@TODO: Figure out how to reattach this receiver to the spawning struct

    Ok(())
}
