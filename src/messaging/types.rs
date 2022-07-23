use tokio::sync::mpsc;

pub type Sender<E> = mpsc::UnboundedSender<E>;
pub type Receiver<E> = mpsc::UnboundedReceiver<E>;
