use crate::cli::file_listener;
use crate::pufferfish_core::config::PufferfishConfig;
use crate::dev_server::PufferfishDevServer;

pub fn start_development_server(config: &PufferfishConfig) {
    let (thread1, thread2) = file_listener::start_listeners(config);
    let mut dev_server = PufferfishDevServer::new(config);
    dev_server.start();
    thread1.join().unwrap();
    thread2.join().unwrap();
}