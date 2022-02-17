use puf::cli::CLIExecutor;
use puf::cli;

// TODO: cached; make a tree of files and their templates ('dependencies' -> remake all files depending on the changed file)
fn main() {
    CLIExecutor::new(cli::get_matches()).execute();
}
