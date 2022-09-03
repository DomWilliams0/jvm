use std::env::var;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

const DIR_ENV_KEY: &str = "JVM_CLASSPATH_DIR";

fn main() {
    // only applicable to miri builds
    if var("CARGO_FEATURE_MIRI").is_err() {
        return;
    }

    let dir = PathBuf::from(
        var(DIR_ENV_KEY).unwrap_or_else(|_| panic!("missing env var {}", DIR_ENV_KEY)),
    );

    if !dir.is_dir() {
        panic!("not a dir")
    }

    let out_file_path = PathBuf::from(var("OUT_DIR").unwrap()).join("classpath.zip");
    let out_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(out_file_path)
        .expect("failed to create out file");

    let mut zip = ZipWriter::new(out_file);
    let opts = FileOptions::default().compression_method(CompressionMethod::Stored);

    for dent in walkdir::WalkDir::new(&dir).into_iter() {
        let dent = match dent {
            Err(err) => {
                println!("cargo:warning=error walking classpath dir: {}", err);
                continue;
            }
            Ok(d) if d.path_is_symlink() => continue,
            Ok(d) => d,
        };

        let path = dent.path();
        match path.extension().and_then(|s| s.to_str()) {
            Some("class" | "so") => {}
            _ => continue,
        }
        let metadata = dent.metadata().expect("failed to get metadata");
        let zip_path = dent
            .path()
            .strip_prefix(&dir)
            .expect("failed to strip path");
        println!("zipping file {}", zip_path.display());
        if metadata.is_dir() {
            zip.add_directory(zip_path.to_string_lossy(), opts)
                .expect("failed to add directory");
        } else if metadata.is_file() {
            let filename = zip_path.extension();
            if filename
                .map(|ext| !(ext == "class" || ext == "so"))
                .unwrap_or(false)
            {
                continue;
            }

            zip.start_file(zip_path.to_string_lossy(), opts)
                .expect("failed to start file");
            zip.write_all(&std::fs::read(dent.path()).expect("failed to read file"))
                .expect("failed to add file to zip");
        } else {
            unreachable!("not a dir or file: {:?}", dent)
        }
    }

    zip.finish().expect("failed to create zip");
}
