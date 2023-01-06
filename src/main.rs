use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    // open file
    File::open("archive.tar.gz").unwrap();
    // create
    Ok(())
}
