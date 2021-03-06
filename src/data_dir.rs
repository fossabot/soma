use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use fs2::FileExt;
use question::{Answer, Question};

use crate::error::{Error as SomaError, Result as SomaResult};

const SOMA_DATA_DIR_ENV_NAME: &str = "SOMA_DATA_DIR";
const SOMA_DATA_DIR_NAME: &str = ".soma";
const LOCK_FILE_NAME: &str = "soma.lock";
const CACHE_DIR_NAME: &str = "cache";

pub struct DataDirectory {
    path: PathBuf,
    lock: File,
}

impl DataDirectory {
    pub fn new() -> SomaResult<DataDirectory> {
        let path = {
            if let Some(dir) = std::env::var_os(SOMA_DATA_DIR_ENV_NAME) {
                dir.into()
            } else {
                let mut home = dirs::home_dir().ok_or(SomaError::DataDirectoryAccessError)?;
                home.push(SOMA_DATA_DIR_NAME);
                home
            }
        };

        if !path.exists() {
            if let Answer::NO = Question::new(&format!(
                "It seems that you use soma for the first time. The data directory will be initialized at {:?}. [y/n]\n",
                path.as_os_str(),
            ))
                .confirm()
            {
                Err(SomaError::DataDirectoryAccessError)?;
            }
            fs::create_dir_all(&path)?;
        }

        let lock = File::create(path.join(LOCK_FILE_NAME))?;
        // TODO: use more fine-tuned lock
        lock.try_lock_exclusive()
            .or(Err(SomaError::DataDirectoryLockError))?;

        Ok(DataDirectory { path, lock })
    }

    pub fn root_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn cache_path(&self) -> PathBuf {
        self.path.join(CACHE_DIR_NAME)
    }

    pub fn create_cache(&self, dir_name: impl AsRef<Path>) -> SomaResult<PathBuf> {
        let cache_path = self.cache_path().join(dir_name);
        fs::create_dir(&cache_path)?;
        Ok(cache_path)
    }
}

impl Drop for DataDirectory {
    fn drop(&mut self) {
        self.lock.unlock().expect("failed to unlock the lockfile");
    }
}
