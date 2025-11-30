use std::path::Path;
use tokio::fs;

pub async fn delete_recursively(path: &Path) -> std::io::Result<()> {
    if path.is_file() {
        fs::remove_file(path).await
    } else if path.is_dir() {
        fs::remove_dir_all(path).await
    } else {
        Ok(())
    }
}

use std::future::Future;
use std::pin::Pin;

pub fn compute_size(
    path: &Path,
) -> Pin<Box<dyn Future<Output = std::io::Result<u64>> + Send + '_>> {
    Box::pin(async move {
        let mut total = 0;

        if path.is_file() {
            total += fs::metadata(path).await?.len();
        } else if path.is_dir() {
            let mut entries = fs::read_dir(path).await?;
            while let Some(entry) = entries.next_entry().await? {
                total += compute_size(&entry.path()).await?;
            }
        }

        Ok(total)
    })
}
