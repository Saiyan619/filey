use figlet_rs::FIGlet;
use std::fs::File;
use std::fs::{self};
use std::{io};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        folder_path_and_name: String,
    },
    CreateFile {
        file_path_and_name: String,
    },
    DeleteFile {
        path_and_name: String,
    },
    DeleteFolder {
        path_and_name: String,
    },
    DeleteFolderAndFiles {
        folder_name_and_path: String,
    },
    FileDetails {
        file_path_and_name: String,
    },
    CopyFile {
        from_file_path_and_name: String,
        to_file_path_and_name: String,
    },
    MoveFile {
        from_file_path_and_name: String,
        to_file_path_and_name: String,
    },
    List {
        path: String
    },
    SearchFile {
        path_name: String,
        search_arg: String
    }
}

// Helper function to reduce repetitive match results
fn handle_results(result:Result<(), io::Error>, success_msg:&str) {
    match result{
       Ok(_) => println!("{}", success_msg),
       Err(err) => {println!("{err}")}
    }
}

fn create<T: AsRef<Path>>(folder_path_and_name: T) -> Result<(), io::Error> {
    fs::create_dir(folder_path_and_name)?;
    Ok(())
}

fn create_file<T: AsRef<Path>>(file_path_and_name: T) -> Result<(), io::Error> {
    File::create_new(file_path_and_name)?;
    Ok(())
}

fn delete<T: AsRef<Path>>(path_and_name: T) -> Result<(), io::Error> {
    fs::remove_file(path_and_name)?;
    Ok(())
}

fn delete_folder<T: AsRef<Path>>(path_and_name: T) -> Result<(), io::Error> {
    fs::remove_dir(path_and_name)?;
    Ok(())
}
// Removes a directory at this path, after removing all its contents. Use carefully!
fn delete_folder_and_files_within<T: AsRef<Path>>(
    folder_path_and_name: T,
) -> Result<(), io::Error> {
    fs::remove_dir_all(folder_path_and_name)?;
    Ok(())
}

fn file_details<T: AsRef<Path>>(file_path_and_name: T) -> Result<fs::Metadata, io::Error> {
    let meta = fs::metadata(file_path_and_name)?;
    Ok(meta)
}

fn copy<T: AsRef<Path>>(
    from_file_path_and_name: T,
    to_file_path_and_name: T,
) -> Result<u64, io::Error> {
    let result = fs::copy(from_file_path_and_name, to_file_path_and_name)?;
    Ok(result)
}

fn move_file<T: AsRef<Path>>(
    from_file_path_and_name: T,
    to_file_path_and_name: T,
) -> Result<(), io::Error> {
    let move_result = fs::rename(from_file_path_and_name, to_file_path_and_name)?;
    Ok(move_result)
}


fn list<T: AsRef<Path>>(path:T) -> Result<Vec<PathBuf>, io::Error> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    // The entries have now been sorted by their path.

    Ok(entries)
}

fn search_file<T:AsRef<Path>>(path_name:T, search_arg: &str) -> Result<(), io::Error>{
   for entry in WalkDir::new(path_name) {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy();
        if name.contains(&search_arg){
            println!("found:{}, path - {}", name, entry.path().display());
        }
};

Ok(())
}



fn main() {
    let args = CLI::parse();

    let standard_font = FIGlet::standard().unwrap();
    println!("{}", standard_font.convert("FILEY").unwrap());

    match &args.command {
        Commands::Create {
            folder_path_and_name,
        } => {
            handle_results(create(folder_path_and_name), "folder successfully created!");
        },

        Commands::CreateFile { file_path_and_name } =>  {
            handle_results(create_file(file_path_and_name), "File created successfully");
        },

        Commands::DeleteFile { path_and_name } => {
            handle_results(delete(path_and_name), "File deleted successfully");
        },

        Commands::DeleteFolder { path_and_name } => {
            handle_results(delete_folder(path_and_name), "folder deleted successfully");
        },

        Commands::DeleteFolderAndFiles {
            folder_name_and_path,
        } => {
          handle_results(delete_folder_and_files_within(folder_name_and_path), "folders and files deleted successfully");  
        },

        Commands::FileDetails { file_path_and_name } => match file_details(file_path_and_name) {
            Ok(ok) => {
                let created_at = match ok.created() {
                    Ok(ok) => {
                        let dt: DateTime<Local> = ok.into();
                        format!("{}", dt.format("%Y-%m-%d %H:%M:%S"))
                    }
                    Err(err) => format!(
                        "something went wrong with time retrival, {:?}",
                        err.to_string()
                    ),
                };

                let modified_at = match ok.modified() {
                    Ok(ok) => {
                        let dt: DateTime<Local> = ok.into();
                        format!("{}", dt.format("%Y-%m-%d %H:%M:%S"))
                    }
                    Err(err) => format!(
                        "something went wrong with time retrival, {:?}",
                        err.to_string()
                    ),
                };

                let size = ok.len() / 1024;
                let file_type = if ok.is_file() { "File" } else { "Directory" };
                println!(
                    "Created: {}\nLast Modified: {}\nSize: {} KB\nType: {}",
                    created_at, modified_at, size, file_type
                )
            }
            Err(err) => println!("{err}"),
        },

        Commands::CopyFile {
            from_file_path_and_name,
            to_file_path_and_name,
        } => match copy(from_file_path_and_name, to_file_path_and_name) {
            Ok(_) => println!("file copied"),
            Err(err) => println!("{err}"),
        },

        Commands::MoveFile {
            from_file_path_and_name,
            to_file_path_and_name,
        } => {
            handle_results(move_file(from_file_path_and_name, to_file_path_and_name), "file moved successfully");
        },

        Commands::List { path } => {
            match list(path) {
                Ok(ok) => {
                    for item in ok {
                        println!("{}", item.display())
                    }
                },
                Err(err) => println!("{err}")
            }
        },
        Commands::SearchFile { path_name, search_arg } => {
            match search_file(path_name, search_arg) {
                Ok(ok) => ok,
                Err(err) => println!("{err}")
            }
        }
    }
}
