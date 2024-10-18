use clap::{Parser, ValueEnum};
use regex::Regex;
use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::path::Path;

#[derive(ValueEnum, Clone, Debug)]
enum FilterType {
    // Show any file type
    Any,
    // Show text files
    Text,
    // Show symbolic links
    Symlink,
    // Show Char or Block device
    Device,
    // Show directories
    Folder,
}

/// Search files inside a directory, easily
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Search directory
    #[arg(required = false, default_value = ".")]
    dir: String,

    /// Regex pattern used to filter file names
    #[arg(short, long)]
    pattern: String,

    /// Filter type
    #[arg(short, long)]
    #[arg(required = false, default_value = "any")]
    filter: FilterType,

    /// Color output if defined
    #[arg(short, long)]
    #[arg(required = false, default_value_t = false)]
    no_colors: bool,
}

fn print_path(parent: &str, fname: &str, subs: &str, colors: bool) {
    let start = fname
        .find(subs)
        .expect("Can't find subtring inside file name");
    let end = start + subs.len();
    let mut col_a = "";
    let mut col_b = "";

    if colors {
        col_a = "\x1b[91m";
        col_b = "\x1b[0m";
    }

    println!(
        "{}/{}{}{}{}{}",
        parent,
        &fname[..start],
        col_a,
        &fname[start..end],
        col_b,
        &fname[end..]
    );
}

fn find(path: &Path, regex: &Regex, ftype: &FilterType, colors: bool) {
    let Some(os_fname) = path.file_name() else {
        eprintln!("Can't read file name for path '{path:#?}'");
        return;
    };
    let Some(fname) = os_fname.to_str() else {
        eprintln!("Can't read UTF-8 string file name for '{os_fname:#?}'");
        return;
    };

    if let Some(mat) = regex.find(fname) {
        if match ftype {
            FilterType::Any => true,
            FilterType::Text => path.is_file(),
            FilterType::Folder => path.is_dir(),
            FilterType::Symlink => path.is_symlink(),
            FilterType::Device => match fs::metadata(path) {
                Ok(md) => md.file_type().is_char_device() || md.file_type().is_block_device(),
                Err(e) => {
                    eprintln!("Error reading '{path:#?}': {e}");
                    false
                }
            },
        } {
            let parent = path.parent().expect("Path must have parent");
            let sparent = parent
                .to_str()
                .expect("Can't convert parent to UTF-8 string");

            print_path(sparent, fname, mat.as_str(), colors);
        }
    }

    if !path.is_dir() || path.is_symlink() {
        return;
    }

    let iter = fs::read_dir(path).expect("Path must be a directory");

    for entry in iter {
        if let Ok(item) = entry {
            find(&item.path(), regex, ftype, colors);
        } else {
            eprintln!("Can't iterate over '{entry:#?}'");
        }
    }
}

fn main() {
    let args = Args::parse();
    let dir = args.dir;
    let regex = Regex::new(&args.pattern);

    if regex.is_err() {
        println!("Please provide a valid regex");
        return;
    }

    let fspath = Path::new(&dir);

    match fspath.try_exists() {
        Ok(exists) => {
            if exists {
                find(fspath, &regex.unwrap(), &args.filter, !args.no_colors);
            } else {
                println!("{dir} directory doesn't exist");
            }
        }
        Err(e) => {
            eprintln!("Can't open {dir}: {e}");
        }
    }
}
