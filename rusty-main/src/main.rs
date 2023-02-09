use std::io::{self, Write};
use std::path::Path;

#[derive(Debug)]
struct File {
    path: String,
    is_directory: bool,
}

impl File {
    fn new(path: &str) -> io::Result<File> {
        let metadata = fs::metadata(path)?;
        Ok(File {
            path: path.to_owned(),
            is_directory: metadata.is_dir(),
        })
    }
}

fn list_directory(dir: &str) -> io::Result<Vec<File>> {
    let mut files = vec![];
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file = File::new(path.to_str().unwrap())?;
        files.push(file);
    }
    Ok(files)
}

fn print_files(files: &[File], prefix: &str) {
    for file in files {
        println!("{}{}", prefix, file.path);
    }
}

fn run() -> Result<(), io::Error> {
    let mut path = String::new();

    loop {
        print!("{} $ ", path);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input = input.trim().to_owned();

        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let mut args = parts;

        match command {
            "ls" => {
                let path = if args.clone().count() > 0 {
                    args.next().unwrap()
                } else {
                    &path
                };

                let files = list_directory(path)?;
                print_files(&files, "");
            }
            "cd" => {
                let new_path = args.next().unwrap();

                let new_path = if new_path.starts_with("/") {
                    new_path.to_owned()
                } else {
                    let current_path = Path::new(&path);
                    let new_path = Path::new(new_path);
                    current_path.join(new_path).to_str().unwrap().to_owned()
                };

                let file = File::new(&new_path)?;

                if file.is_directory {
                    path = new_path;
                } else {
                    eprintln!("cd: not a directory: {}", new_path);
                }
            }
            "pwd" => {
                println!("{}", path);
            }
            "exit" => {
                break;
            }
            _ => {
                eprintln!("Unknown command: {}", command);
            }
        }
    }
    Ok(())
}

fn main() {
    run().unwrap();
}
