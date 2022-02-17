use clap::{App, AppSettings, arg, Arg, ArgMatches};

pub fn get_matches() -> ArgMatches {
    App::new("Pufferfish")
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
        .get_matches()
}