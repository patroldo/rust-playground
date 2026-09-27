use std::io;

use kvdb::Commands;

fn main() -> io::Result<()> {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        match Commands::try_from(buffer.trim()) {
            Ok(Commands::Exit) => break,
            Ok(val) => println!("{:?}", val),
            Err(_) => println!("Coudn't unparse the command"),
        }
    }
    Ok(())
}
