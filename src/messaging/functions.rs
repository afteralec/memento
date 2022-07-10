use super::MatcherMut;
use crate::Result;
use std::fmt;
use tokio::sync::mpsc;

pub async fn match_receiver<T, M>(
    mut receiver: mpsc::UnboundedReceiver<T>,
    mut matcher: M,
) -> Result<()>
where
    T: 'static + Send + Sync + fmt::Debug,
    M: 'static + Send + Sync + fmt::Debug + MatcherMut<T>,
{
    while let Some(event) = receiver.recv().await {
        matcher.match_on(event)?;
    }

    //@TODO: Figure out how to reattach this receiver to the Room struct

    Ok(())
}
