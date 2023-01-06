use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

fn main() {
    unpack_archive("./archive.tar.gz", "./midjourney").unwrap();
}

fn unpack_archive(archive_filepath: &str, outpath: &str) -> Result<(), std::io::Error> {
    let f = File::open(archive_filepath)?;
    let gz = GzDecoder::new(f);
    let mut archive = Archive::new(gz);

    archive.unpack(outpath)?;
    Ok(())
}
