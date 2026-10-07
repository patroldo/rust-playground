mod file_service;

use std::{
    io::{self, Write},
    path::PathBuf,
};

use kvdb::{KVDB, KVDBErrors, KVDBType, apply_command_to_kvdb, cli_parser::Commands};

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = ".")]
    root: PathBuf,

    #[command(subcommand)]
    command: DbType,
}

#[derive(Subcommand)]
enum DbType {
    InMemory,
    FileBased {
        db_filename: PathBuf,
        #[arg(short, long, default_value_t = false)]
        create: bool,
    },
}

fn create_db(db_type: DbType) -> std::io::Result<Result<Box<dyn KVDB>, KVDBErrors>> {
    match db_type {
        DbType::InMemory => Ok(KVDBType::create_kvdb(KVDBType::InMemory)),
        DbType::FileBased {
            create,
            db_filename,
        } => {
            if create {
                file_service::create_file_if_not_exists(&db_filename)?;
            }
            Ok(KVDBType::create_kvdb(KVDBType::FileBased(
                db_filename.to_str().unwrap(),
            )))
        }
    }
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let mut kvdb = create_db(args.command)?.unwrap();
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
