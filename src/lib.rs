pub mod device_client;
pub mod discovery;
pub mod media_renderer;
pub mod media_server;
pub mod parser;
pub mod types;

use std::sync::{mpsc::Sender, LazyLock, Mutex};

use types::Event;

static BROADCAST_EVENT: LazyLock<Mutex<Option<Sender<Event>>>> = LazyLock::new(|| Mutex::new(None));
