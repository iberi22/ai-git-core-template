use async_trait::async_trait;
use gc_core::ports::{FileSystemPort, Result, CoreError};
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub struct TokioFileSystem;

#[async_trait]
impl FileSystemPort for TokioFileSystem {
    async fn create_dir(&self, path: &str) -> Result<()> {
        if !Path::new(path).exists() {
            fs::create_dir_all(path).await.map_err(CoreError::Io)?;
        }
        Ok(())
    }

    async fn write_file(&self, path: &str, content: &str) -> Result<()> {
        let mut file = fs::File::create(path).await.map_err(CoreError::Io)?;
        file.write_all(content.as_bytes()).await.map_err(CoreError::Io)?;
        Ok(())
    }

    async fn read_file(&self, path: &str) -> Result<String> {
        let content = fs::read_to_string(path).await.map_err(CoreError::Io)?;
        Ok(content)
    }

    async fn exists(&self, path: &str) -> Result<bool> {
        Ok(Path::new(path).exists())
    }

    async fn move_file(&self, source: &str, dest: &str) -> Result<()> {
        fs::rename(source, dest).await.map_err(CoreError::Io)?;
        Ok(())
    }

    async fn list_files(&self, dir: &str, pattern: Option<&str>) -> Result<Vec<String>> {
        let mut entries = fs::read_dir(dir).await.map_err(CoreError::Io)?;
        let mut files = Vec::new();

        while let Some(entry) = entries.next_entry().await.map_err(CoreError::Io)? {
            let path = entry.path();
            if path.is_file() {
                let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                if let Some(pat) = pattern {
                    if name.contains(pat) || (pat.starts_with("*.") && name.ends_with(&pat[1..])) {
                        files.push(name);
                    }
                } else {
                    files.push(name);
                }
            }
        }
        Ok(files)
    }
}
