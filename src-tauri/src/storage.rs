use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use zbox::{init_env, Repo, RepoOpener};
use crate::models::*;
use crate::errors::StorageError;

pub struct Inner {
    app_config_dir: PathBuf,
    vault: Repo,
}

/// ZBox file system mounted to directry. Entire FS journal is stored inside application config
/// directory and is accessible through native API.
///
/// All stored data are encrypted using libsodium.
#[derive(Clone)]
pub struct Storage(Arc<Mutex<Inner>>);

impl Storage {
    /// Open zbox file system repository
    ///
    /// This method will panic and kill entire application if anything goes wrong and it must do so
    /// since FS access is required for application.
    ///
    /// Repository remains open through application lifetime but stored values are accessible only
    /// when read methods are called
    ///
    /// # Examples
    ///
    /// ```
    /// let vault = Storage::new();
    /// ```
    pub fn new() -> Self {
        init_env();

        let user_dirs = directories::BaseDirs::new().expect(
            "Home directory is not configured. Please check your OS Distribution instruction",
        );
        let root = user_dirs.config_dir();
        Self::open_with_root(root.to_owned())
    }

    #[doc(hidden)]
    pub fn open_with_root(root: PathBuf) -> Self {
        let id = std::process::id().to_be_bytes();

        let app_config_dir = root.join("worryless");
        std::fs::create_dir_all(&app_config_dir).expect("Failed to create config directory");

        let zbox_pass = if let Ok(pass) = std::fs::read_to_string(app_config_dir.join(".sec")) {
            pass
        } else {
            use rand::distributions::Alphanumeric;
            use rand::{thread_rng, Rng};

            let mut rng = thread_rng();
            let pass: String = (0..50).map(|_| rng.sample(Alphanumeric) as char).collect();
            std::fs::write(app_config_dir.join(".sec"), &pass).expect("Failed to save vault pass");
            pass
        };

        let uri = format!("file://{}", app_config_dir.display());
        let path = format!("{uri}/secure");
        std::fs::remove_file(&app_config_dir.join("secure").join(".repo_lock")).ok();

        println!("Storage path is: {path}");
        let vault = match RepoOpener::new().create(true).open(&path, &zbox_pass) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Unable to build secret vault: {e}");
                panic!("Unable to build secret vault");
            }
        };
        std::fs::write(&app_config_dir.join("secure").join(".repo_lock"), &id).ok();

        Self(Arc::new(Mutex::new(Inner {
            app_config_dir,
            vault,
        })))
    }

    /// Read stored credentials from encrypted IO
    ///
    /// # Examples
    ///
    /// ```
    /// async fn load_creds(storage: Storage, creds: Vec<ServerCredentials>) {
    ///     let creds = tokio::spawn_blocking(move || storage.credentials(&creds).unwrap());
    /// }
    /// ```
    pub fn credentials(&self) -> Result<Vec<ServerCredentials>, StorageError> {
        let mut inner = self.0.lock().unwrap();

        let f = zbox::OpenOptions::new()
            .create(true)
            .open(&mut inner.vault, "/credentials")?;

        Ok(bincode::deserialize_from(f)?)
    }

    /// Store all credentials in encrypted safe zbox storage
    ///
    /// Be aware this is IO & crypto operation so it will requires considerable processing power.
    /// To prevent chocking tokio runtime you must use `spawn_blocking` or `spawn_local`.
    ///
    /// # Examples
    ///
    /// ```
    /// async fn save_creds(storage: Storage, creds: Vec<ServerCredentials>) {
    ///     tokio::spawn_blocking(move || storage.store_credentials(&creds).unwrap());
    /// }
    /// ```
    pub fn store_credentials(
        &self,
        credentials: &Vec<ServerCredentials>,
    ) -> Result<(), StorageError> {
        use std::io::Write;
        let mut inner = self.0.lock().unwrap();

        let mut file = zbox::OpenOptions::new()
            .create(true)
            .open(&mut inner.vault, "/credentials")
            .unwrap();

        let bin = bincode::serialize(credentials)?;

        file.write_all(bin.as_slice())?;

        Ok(file.finish()?)
    }
}

#[cfg(test)]
mod check {
    use super::*;
    use tempdir::TempDir;
    use url::Url;

    #[test]
    fn rwr() {
        let root = TempDir::new("rwr").unwrap();
        let creds = vec![
            ServerCredentials {
                url: Url::parse("http://me.mm.so").unwrap(),
                access_token: AccessToken::try_from("hs8das8dg8asgd").unwrap(),
            },
            ServerCredentials {
                url: Url::parse("http://me.mm.so").unwrap(),
                access_token: AccessToken::try_from("hs8das8dg8asgd").unwrap(),
            },
        ];

        {
            let storage = Storage::open_with_root(root.path().to_owned());

            let loaded = storage.credentials().unwrap();
            assert_eq!(loaded, vec![]);

            storage.store_credentials(&creds).unwrap();
            let loaded = storage.credentials().unwrap();
            assert_eq!(loaded, creds);
        }
        {
            let storage = Storage::open_with_root(root.path().to_owned());
            let loaded = storage.credentials().unwrap();
            assert_eq!(loaded, creds);
        }
    }
}
