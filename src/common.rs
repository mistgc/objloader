use std::io::{ Seek, SeekFrom };

pub fn file_open<T: AsRef<str>>(path: T) -> Result<std::fs::File, std::io::Error> {
    let f = std::fs::File::open(path.as_ref())?;

    Ok(f)
}

pub fn file_size(f: &mut std::fs::File) -> Result<u64, std::io::Error> {
    let current = f.seek(SeekFrom::Current(0))?;
    let size = f.seek(SeekFrom::End(0))?;

    // Seek back to current position.
    f.seek(SeekFrom::Start(current))?;

    Ok(size)
}

pub fn file_read<T: AsRef<str>>(path: T) -> Result<Vec<u8>, std::io::Error> {
    let data = std::fs::read(path.as_ref())?;

    Ok(data)
}
