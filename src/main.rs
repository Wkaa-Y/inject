use std::{env::args, ffi::OsStr, path::{Path, PathBuf}};


const ALLOWED_EXTENSIONS: [&str; 4] = ["js", "jsx", "ts", "tsx"];
const PATTERNS: [&str; 4] = ["from \"", "from '", "import \"", "import '"];

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

    for path in &root_entries {
        read_dir_recursively(path, args[1], &root_entries);
    }
 }

#[allow(unused_must_use)]
fn read_dir_recursively<P, T>(path: P, alias: String, root_entries: &Vec<T>) -> Result<(), std::io::Error>
where
    P: AsRef<Path>,
    T: AsRef<Path>
{
   let directories = std::fs::read_dir(path)?.filter_map(|d| d.ok()).collect::<Vec<_>>();
     for d in directories {
         let dir_metadata = d.metadata().unwrap();
            if dir_metadata.is_dir() { read_dir_recursively(d.path(), alias, &root_entries); } 
            else if dir_metadata.is_file() {
                let file_name = d.file_name();
                let extension = Path::new(&file_name).extension().and_then(OsStr::to_str).unwrap(); 
                if ALLOWED_EXTENSIONS.contains(&extension) { inject(d.path(), alias, &root_entries);  }
            }
   }

    Ok(())
}

#[allow(unused_must_use)]
fn inject<P, T>(path: P, alias: String, root_entries: &Vec<T>) -> ()
where 
    P: AsRef<Path>,
    T: AsRef<Path>,
{   
    for entry in root_entries {
        let mut content = std::fs::read_to_string(path).unwrap();
        for (index, pattern) in PATTERNS.iter().enumerate() {
            let matcher = vec![pattern.to_string(), entry].join(""); // TODO : expect type error
            let destination = vec![pattern.to_string(), alias, String::from("/"), entry].join(""); // TODO : expect type error
            content = content.replace(&matcher, &destination);
        }
        std::fs::write(path, content);
    }
}