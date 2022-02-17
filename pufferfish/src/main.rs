use puf::cli::cli::CLIExecutor;
use puf::cli;

// TODO: cached; make a tree of files and their templates ('dependencies' -> remake all files depending on the changed file)
fn main() {
    
    
    // match rutie::VM::eval("puts \"hello\"") {
    //     Ok(e) => println!("{:?}", e),
    //     Err(e) => println!("{}", e)
    // }
    
    CLIExecutor::new(cli::get_matches()).execute();
}


struct MyTestSTruture {
    int: u32,
    other: String
}

fn __main() {
    let config = PufferfishConfig::from_toml("pufferfish.toml");
    let mut listener = puf::file_listener::FileListener::new(config);
    let my_struct = MyTestSTruture { int: 0, other: String::new() };
    // TODO: test with listener
    let lock = Arc::new(RwLock::new(my_struct));
    let lock2 = lock.clone();
    let lock3 = lock.clone();
    
    let t1 = thread::spawn(move || {
        let mut l = lock.write().unwrap();
        l.other = String::from("Hello");
    });
    
    let t2 = thread::spawn(move || {
        let mut l = lock2.write().unwrap();
        l.other = format!("{} world", l.other);
    });
    
    
    t1.join();
    t2.join();
    
    let l = lock3.write().unwrap();
    println!("{}", l.other);
    
}