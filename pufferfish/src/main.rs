use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use std::thread::Thread;
use clap::{arg, App, AppSettings, Arg};
use pufferfish::cli::CLIExecutor;
use pufferfish::config::PufferfishConfig;
use pufferfish::file_listener::FileListener;

// TODO: cached; make a tree of files and their templates ('dependencies' -> remake all files depending on the changed file)
fn main() {
    let matches = App::new("Pufferfish")
        // Info
        .about("An html templating engine that builds static html")
        .version("0.3.0")
        .author("Jonas Everaert, info@jonaseveraert.be")
        .setting(AppSettings::SubcommandRequired)
        // Commands
        .subcommand(
            App::new("build")
                .about("Builds the html to the output directory")
                .arg(
                    Arg::new("file")
                        .help("The file to build")
                )
                .arg(
                    arg!(-c --clean "Does a clean build of the project, not using any cached files")
                        .required(false)
                )
        )
        .subcommand(
            App::new("start")
                .about("Starts the development server")
        )
        .subcommand(
            App::new("start-listener")
                .about("Starts the file listener, building files as they get updated")
        )
        .get_matches();
    
    let config = PufferfishConfig::new();
    let mut listener = pufferfish::file_listener::FileListener::new(config);
    
    let listener = Arc::new(Mutex::new(listener));
    
    let (tx_c, _rx) = channel();
    
    let (listener_data, tx) = (Arc::clone(&listener), tx_c.clone());
    let h = thread::spawn(move || {
        println!("Starting html listener");
        FileListener::start_html(listener_data, tx, &PufferfishConfig::new());
    });
    let (listener_data_templates, tx_templates) = (Arc::clone(&listener), tx_c.clone());
    let t = thread::spawn(move || {
        println!("Starting template listener");
        FileListener::start_templates(listener_data_templates, tx_templates, &PufferfishConfig::new());
    });
    
    h.join();
    t.join();
    
    // match rutie::VM::eval("puts \"hello\"") {
    //     Ok(e) => println!("{:?}", e),
    //     Err(e) => println!("{}", e)
    // }
    
    // CLIExecutor::new(matches).execute();
}
