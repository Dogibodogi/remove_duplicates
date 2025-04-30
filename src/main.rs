use std::collections::HashSet;
use std::fs;
use std::fs::remove_file;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

fn main() {
    let mut file_path: String = String::from("");
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read frfom keyboard!");

    let file_path = file_path.trim();
    let path = Path::new(file_path);
    let mut set = HashSet::new();
    for entry in fs::read_dir(&path).expect("Failed to read from file!") {
        let entry = entry.expect("Failed to read from file!");
        let entry_path = entry.path();
        if let Some(file_name_str) = entry_path.to_str() {
            set.insert(file_name_str.to_string());
        }

        for second_entry in fs::read_dir(&path).expect("Failed to open folder!") {
            let second_entry = second_entry.expect("Failed to open folder!");
            let second_entry_path = second_entry.path();
            if second_entry_path.to_str() != entry_path.to_str() {
                let entry_path_buf: PathBuf = entry_path.to_owned();
                let second_entry_path_buf: PathBuf = second_entry_path.to_owned();
                let result: Result<bool, io::Error> =
                    compare_files(&entry_path_buf, &second_entry_path);
                match result {
                    Ok(true) => {
                        if let Some(file_name_str) = second_entry_path_buf.to_str() {
                            set.insert(file_name_str.to_string());
                        }
                    }
                    Ok(false) => (),
                    Err(e) => println!("Error at comparing files!, {:?}", e),
                }
            }
        }
        if set.len() > 1 {
            delete_files(&set);
        }
        set.clear();
    }
}

fn compare_files(path1: &Path, path2: &Path) -> io::Result<bool> {
    let mut file1 = File::open(path1).expect("Failed to open file 1");
    let mut file2 = File::open(path2).expect("Failed to open file 2");
    let mut buffer1 = [0u8; 4096];
    let mut buffer2 = [0u8; 4096];

    loop {
        let n1 = file1.read(&mut buffer1)?;
        let n2 = file2.read(&mut buffer2)?;

        if n1 != n2 {
            return Ok(false);
        }

        if n1 == 0 && n2 == 0 {
            return Ok(true);
        }

        if buffer1[..n1] != buffer2[..n2] {
            return Ok(false);
        }
    }
}

fn delete_files(set: &HashSet<String>) {
    let mut i = 0;
    let mut buf: String = String::from(" ");
    println!("The following files are identical!");
    for item in set {
        i = i + 1;
        let file_name:PathBuf= PathBuf::from(item);

        if let Some(file_name_osstr)=file_name.file_name(){
            let file_name_str=file_name_osstr.to_str();
            println!("{i}.{:?}", file_name_str);   
        }
    }

    loop{
    println!("Which files would you like to delete? Type exit to leave this menu.");

    io::stdin()
        .read_line(&mut buf)
        .expect("Error while reading input!");

    if buf.trim().eq("exit"){
        break;
    }
    i = 0;
    for item in set {
        i = i + 1;
        println!("debug");
        if let Ok(parsed) = buf.trim().parse::<i32>() {
            println!("parsed:{parsed}");
            if i == parsed {
                if let Err(e) = remove_file(item) {
                    eprintln!("Error at deleting the file! {e}");
                } else {
                    println!("Deleted file!");
                }
            }
        }
    }
}
    }
