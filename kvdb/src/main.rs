use std::io;

use kvdb::{KVDBType, apply_command_to_kvdb, cli_parser::Commands};

fn main() -> io::Result<()> {
    let mut kvdb = KVDBType::create_kvdb(KVDBType::InMemory);
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        match Commands::try_from(buffer.trim()) {
            Ok(Commands::Exit) => break,
            Ok(command) => println!("{}\n", apply_command_to_kvdb(&mut kvdb, command)),
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}
