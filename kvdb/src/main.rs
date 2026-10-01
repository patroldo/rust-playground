use std::io::{self, Write};

use kvdb::{KVDBType, apply_command_to_kvdb, cli_parser::Commands};

fn main() -> io::Result<()> {
    let mut kvdb = KVDBType::create_kvdb(KVDBType::InMemory);
    loop {
        print!("kvdb> ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        match Commands::try_from(buffer.trim()) {
            Ok(Commands::Exit) => break,
            Ok(command) => println!("{}", apply_command_to_kvdb(kvdb.as_mut(), command)),
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}
