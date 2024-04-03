use std::{
    fmt::Debug,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use bincode::{
    config,
    serde::{decode_from_std_read, encode_into_std_write},
};
use serde::{de::DeserializeOwned, Serialize};

pub struct Binary;

impl Binary {
    pub fn import<T, P>(path: P) -> color_eyre::Result<T>
    where
        T: DeserializeOwned + Debug,
        P: AsRef<Path>,
    {
        let config = config::standard();

        let file = File::open(path)?;

        let mut reader = BufReader::new(file);

        let decoded = decode_from_std_read(&mut reader, config)?;

        Ok(decoded)
    }

    pub fn export<T, P>(path: P, value: &T) -> color_eyre::Result<()>
    where
        T: Serialize,
        P: AsRef<Path>,
    {
        let config = config::standard();

        let file = File::create(path)?;

        let mut writer = BufWriter::new(file);

        encode_into_std_write(value, &mut writer, config)?;

        Ok(())
    }
}
