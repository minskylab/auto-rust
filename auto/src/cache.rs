use serde_derive::{Deserialize, Serialize};
use std::{fs::File, io::Write};

const AUTO_ARTIFACTS_FOLDER: &str = ".auto";

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolvedSourceCode {
    pub prompt_input: String,
    pub implementation: String,
    pub hash: String,
}

impl ResolvedSourceCode {
    pub fn save(&self) {
        std::fs::create_dir_all(AUTO_ARTIFACTS_FOLDER).unwrap();

        let filepath = format!("{}/{}.json", AUTO_ARTIFACTS_FOLDER, self.hash);

        let mut file = File::create(filepath).unwrap();
        let json = serde_json::to_string_pretty(&self).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn load(hash: &str) -> Option<Self> {
        let filepath = format!("{}/{}.json", AUTO_ARTIFACTS_FOLDER, hash);

        if let Ok(file) = File::open(filepath) {
            let reader = std::io::BufReader::new(file);
            let res: Self = serde_json::from_reader(reader).unwrap();
            Some(res)
        } else {
            None
        }
    }
}
