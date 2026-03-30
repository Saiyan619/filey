use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
// use clap::Command;
use clap::{Parser, Subcommand};
use chrono::{DateTime, Local};


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands
}
#[derive(Subcommand)]
enum Commands{
    Create{folder_path_and_name: String},
    CreateFile{file_path_and_name: String},
    DeleteFile{path_and_name: String},
    DeleteFolder{path_and_name: String},
    DeleteFolderAndFiles{folder_name_and_path: String},
    FileDetails{file_path_and_name: String},
    CopyFile{from_file_path_and_name: String, to_file_path_and_name: String},
    MoveFile{from_file_path_and_name: String, to_file_path_and_name: String}
}

fn create<T: AsRef<Path>>(folder_path_and_name:T) -> Result<(), io::Error>{
    match fs::create_dir(folder_path_and_name) {
        Ok(_) => Ok(()),
        Err(err) => {Err(err)}
    }
   }

   fn create_file<T:AsRef<Path>>(file_path_and_name: T) -> Result<(), io::Error> {
    match File::create_new(file_path_and_name) {
        Ok(_) => Ok(()),
        Err(err) => {Err(err)}
    } 
      }

    fn delete<T:AsRef<Path>>(path_and_name: T) -> Result<(), io::Error>{
    match fs::remove_file(path_and_name) {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
   }   

   fn delete_folder<T: AsRef<Path>>(path_and_name: T) -> Result<(), io::Error> {
     fs::remove_dir(path_and_name)?;
     Ok(())
}
   // Removes a directory at this path, after removing all its contents. Use carefully!
fn delete_folder_and_files_within<T:AsRef<Path>>(folder_path_and_name:T) -> Result<(), io::Error> {
    match fs::remove_dir_all(folder_path_and_name) {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}

fn file_details<T: AsRef<Path>>(file_path_and_name: T) -> Result<fs::Metadata, io::Error> {
     match fs::metadata(file_path_and_name) {
         Ok(ok) => Ok(ok),
         Err(err) => Err(err)
     }
}

fn copy<T: AsRef<Path>>(from_file_path_and_name: T, to_file_path_and_name: T) -> Result<u64, io::Error> {
    match fs::copy(from_file_path_and_name, to_file_path_and_name) {
        Ok(ok) => Ok(ok),
        Err(err) => Err(err)
    } 
}

fn move_file<T: AsRef<Path>>(from_file_path_and_name: T, to_file_path_and_name: T) -> Result<(), io::Error>{
     match fs::rename(from_file_path_and_name, to_file_path_and_name) {
        Ok(ok) => Ok(ok),
        Err(err) => Err(err)
     }
}

fn main() {

    let args = CLI::parse();
     
   match &args.command {
       Commands::Create { folder_path_and_name } => {
        match create(folder_path_and_name){
            Ok(_) => println!("Folder created successfully"),
            Err(err) => {println!("{err}")}
   }
       },

       Commands::CreateFile { file_path_and_name } => {
        match create_file(file_path_and_name) {
            Ok(_) => println!("File created successfully"),
            Err(err) => {println!("{err}")}
        }
       },

       Commands::DeleteFile { path_and_name } => {
        match delete(path_and_name) {
            Ok(_) => println!("File deleted successfully"),
            Err(err) => println!("{err}")
        }
       },

       Commands::DeleteFolder { path_and_name } => {
        match delete_folder(path_and_name) {
            Ok(_) => println!("folder deleted successfully"),
            Err(err) => println!("{err}")
        }
       }

       Commands::DeleteFolderAndFiles { folder_name_and_path } => {
        match delete_folder_and_files_within(folder_name_and_path) {
            Ok(_) => println!("folders and files deleted successfully"),
            Err(err) => println!("{err}")
        }
       }

       Commands::FileDetails { file_path_and_name } => {
        match file_details(file_path_and_name) {
            Ok(ok) => {
                let created_at = match ok.created() {
                    Ok(ok) => {
                         let dt: DateTime<Local> = ok.into();
                format!("{}", dt.format("%Y-%m-%d %H:%M:%S"))
                    },
                    Err(err) => format!("something went wrong with time retrival, {:?}", err.to_string())
                };

                let modified_at = match  ok.modified() {
                    Ok(ok) => {
                let dt: DateTime<Local> = ok.into();
                format!("{}", dt.format("%Y-%m-%d %H:%M:%S"))
                    }
                    Err(err) => format!("something went wrong with time retrival, {:?}", err.to_string())
                };
                
                let size = ok.len() / 1024;
                let file_type = if ok.is_file() { "File" } else { "Directory" };
                println!("Created: {}\nLast Modified: {}\nSize: {} KB\nType: {}", created_at, modified_at, size, file_type)
            },
        Err(err) => println!("{err}")
    }
       },
       
       Commands::CopyFile { from_file_path_and_name, to_file_path_and_name } => {
        match copy(from_file_path_and_name, to_file_path_and_name) {
            Ok(_) => println!("file copied"),
            Err(err) => println!("{err}")
        }
       },

       Commands::MoveFile { from_file_path_and_name, to_file_path_and_name } => {
        match move_file(from_file_path_and_name, to_file_path_and_name) {
            Ok(_) => println!("file moved successfully"),
            Err(err) => println!("{err}")
        }
       }

   }
}

