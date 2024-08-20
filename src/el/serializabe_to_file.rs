use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::genesis_config::GenesisConfig;


pub trait Genesis: SerializableToFile {
    fn create_genesis(genesis: &GenesisConfig) -> Self;

}

pub trait SerializableToFile: Serialize + Sized + DeserializeOwned {
    fn read_from_file<P: AsRef<Path>>(path: P) -> serde_json::Result<Self> {
        let file = File::open(path).unwrap();
        serde_json::from_reader(file)
    }
    fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        let serialized_data = serde_json::to_string(self)?;
        write!(file, "{}", serialized_data)?;
        Ok(())
    }

    fn save_if_not_exists<P: AsRef<Path>>(&self, path: P) {
        if !path.as_ref().exists() {
            self.save_to_file(path).expect("Could not save genesis file");
        } else {
            println!("{} already exists. Skipping generation...", path.as_ref().display());
        }
    }
 
}