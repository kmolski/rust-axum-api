use std::path::{Path, PathBuf};
use std::sync::Arc;

use tokio::fs::File;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct FileManager {
    base_dir: Arc<Mutex<PathBuf>>,
}

impl FileManager {
    pub fn new(base_dir: impl AsRef<Path>) -> Self {
        FileManager {
            base_dir: Arc::new(Mutex::new(base_dir.as_ref().to_path_buf())),
        }
    }

    pub async fn save_file(&self, name: impl AsRef<Path>, content: &[u8]) -> io::Result<()> {
        let mut path = {
            let base_dir = self.base_dir.lock().await;
            base_dir.clone()
        };
        path.push(name);

        let mut file = File::create(path).await?;
        file.write_all(content).await
    }

    pub async fn open_file(&self, name: impl AsRef<Path>) -> io::Result<File> {
        let mut path = {
            let base_dir = self.base_dir.lock().await;
            base_dir.clone()
        };
        path.push(name);

        File::open(path).await
    }
}
