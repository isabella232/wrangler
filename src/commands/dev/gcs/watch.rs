use std::sync::{mpsc, Arc, Mutex};

use crate::commands::dev::gcs::setup::get_preview_id;
use crate::commands::dev::server_config::ServerConfig;

use crate::settings::toml::Target;
use crate::watch::watch_and_build;

pub fn watch_for_changes(
    target: Target,
    server_config: &ServerConfig,
    preview_id: Arc<Mutex<String>>,
    session_id: &str,
    verbose: bool,
) -> Result<(), failure::Error> {
    let (sender, receiver) = mpsc::channel();
    watch_and_build(&target, Some(sender))?;

    while receiver.recv().is_ok() {
        let target = target.clone();

        // acquire the lock so incoming requests are halted
        // until the new script is ready for them
        let mut preview_id = preview_id.lock().unwrap();

        // while holding the lock, assign a new preview id
        //
        // this allows the server to route subsequent requests
        // to the proper script
        *preview_id = get_preview_id(target, None, server_config, session_id, verbose)?;
    }

    Ok(())
}
