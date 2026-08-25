// ──────────────────────────────────────────────────────────────────────────────
// storage — Blob 存储后端抽象（plan002 U6a/U6b）
//
// 提供 `BlobStorage` trait 与两个实现：
//   - `DbBlobStorage`：no-op 占位。现状（=行为基线）是 FILE_FILE.content 以
//     base64 TEXT 存于 DB 行内，由 file_assemble_control 的 SQL 直接读写，
//     内容与文件元数据行耦合在同一条 INSERT/SELECT 里，无法在不改动该 crate
//     的前提下拆出独立的 put/get/delete（put 需要补齐 name/person 等 NOT NULL
//     元数据列，且 content 为 base64 编码、与 bytes 不对称）。因此 DB 后端下
//     存储层无职责：put/delete 返回 Ok(())（保持兼容、不产生副作用），
//     get 显式返回 Err（fail loud：内容仍在 FILE_FILE.content 行内，
//     由现有代码路径直读，storage 层拿不到也不应伪造数据）。
//     file_assemble_control 迁移到本抽象时再实现真实语义。
//   - `FsBlobStorage`：写 `<root>/<key>` 文件，自动创建父目录；key 中的 `/`
//     作为路径分隔符，拒绝 `..` 等穿越组件。
//
// 工厂 `storage_from_env()`：STORAGE_BACKEND=fs|db（默认 db）；
// fs 时 STORAGE_ROOT 指定根目录（缺失视为启动期配置错误，panic 尽早暴露）。
// ──────────────────────────────────────────────────────────────────────────────

use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;

/// Blob 存储后端：以不透明的 `key` 存取字节内容。
#[async_trait]
pub trait BlobStorage: Send + Sync {
    async fn put(&self, key: &str, bytes: &[u8]) -> Result<(), String>;
    async fn get(&self, key: &str) -> Result<Vec<u8>, String>;
    async fn delete(&self, key: &str) -> Result<(), String>;
}

/// DB 后端占位实现，语义见模块级文档注释。
#[derive(Clone, Copy, Debug, Default)]
pub struct DbBlobStorage;

#[async_trait]
impl BlobStorage for DbBlobStorage {
    async fn put(&self, _key: &str, _bytes: &[u8]) -> Result<(), String> {
        Ok(())
    }

    async fn get(&self, _key: &str) -> Result<Vec<u8>, String> {
        Err("DbBlobStorage is a placeholder: blobs live in FILE_FILE.content rows \
             and are read directly by file_assemble_control; not yet routed through BlobStorage"
            .to_string())
    }

    async fn delete(&self, _key: &str) -> Result<(), String> {
        Ok(())
    }
}

/// 本地文件系统后端：`<root>/<key>`。
#[derive(Clone, Debug)]
pub struct FsBlobStorage {
    root: PathBuf,
}

impl FsBlobStorage {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// 将 key 解析为 root 下的安全路径；拒绝绝对 key 与 `..` 组件以防目录穿越。
    fn resolve(&self, key: &str) -> Result<PathBuf, String> {
        if key.is_empty() {
            return Err("empty blob key".to_string());
        }
        let rel = Path::new(key);
        if rel.is_absolute() || key.starts_with('/') || key.starts_with('\\') || key.contains(':')
        {
            return Err(format!("invalid blob key: {key:?}"));
        }
        for comp in rel.components() {
            if matches!(comp, std::path::Component::ParentDir | std::path::Component::CurDir) {
                return Err(format!("invalid blob key component in {key:?}"));
            }
        }
        Ok(self.root.join(rel))
    }
}

#[async_trait]
impl BlobStorage for FsBlobStorage {
    async fn put(&self, key: &str, bytes: &[u8]) -> Result<(), String> {
        let path = self.resolve(key)?;
        let parent = path
            .parent()
            .ok_or_else(|| format!("invalid blob key: {key:?}"))?
            .to_path_buf();
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("create dir for {key:?}: {e}"))?;
        tokio::fs::write(path, bytes)
            .await
            .map_err(|e| format!("write blob {key:?}: {e}"))
    }

    async fn get(&self, key: &str) -> Result<Vec<u8>, String> {
        let path = self.resolve(key)?;
        tokio::fs::read(path)
            .await
            .map_err(|e| format!("read blob {key:?}: {e}"))
    }

    /// 幂等：删除不存在的 key 视为成功。
    async fn delete(&self, key: &str) -> Result<(), String> {
        let path = self.resolve(key)?;
        match tokio::fs::remove_file(path).await {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(format!("delete blob {key:?}: {e}")),
        }
    }
}

fn storage_from(backend: &str, storage_root: Option<&str>) -> Arc<dyn BlobStorage> {
    match backend.trim().to_ascii_lowercase().as_str() {
        "fs" => {
            let root = match storage_root.map(str::trim).filter(|r| !r.is_empty()) {
                Some(r) => r,
                None => panic!("STORAGE_BACKEND=fs requires STORAGE_ROOT to be set"),
            };
            Arc::new(FsBlobStorage::new(root))
        }
        // 默认（含未设置 / db / 未知值）：保持现状 = DB 行内存储。
        _ => Arc::new(DbBlobStorage),
    }
}

/// 从环境变量构建存储后端：STORAGE_BACKEND=fs|db（默认 db），fs 时读 STORAGE_ROOT。
pub fn storage_from_env() -> Arc<dyn BlobStorage> {
    let backend = std::env::var("STORAGE_BACKEND").unwrap_or_default();
    let root = std::env::var("STORAGE_ROOT").ok();
    storage_from(&backend, root.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_root(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir()
            .join(format!("oa4rust_storage_test_{}_{}", tag, uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[tokio::test]
    async fn fs_put_get_delete_roundtrip() {
        let root = temp_root("roundtrip");
        let fs = FsBlobStorage::new(&root);
        fs.put("plain.txt", b"hello").await.unwrap();
        assert_eq!(fs.get("plain.txt").await.unwrap(), b"hello");
        assert_eq!(std::fs::read(root.join("plain.txt")).unwrap(), b"hello");
        fs.delete("plain.txt").await.unwrap();
        assert!(fs.get("plain.txt").await.is_err());
        std::fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn fs_put_creates_nested_dirs() {
        let root = temp_root("nested");
        let fs = FsBlobStorage::new(&root);
        fs.put("bbs/2026/08/pic.png", &[1, 2, 3]).await.unwrap();
        assert!(root.join("bbs").join("2026").join("08").join("pic.png").is_file());
        assert_eq!(fs.get("bbs/2026/08/pic.png").await.unwrap(), vec![1, 2, 3]);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn fs_get_missing_is_err_and_delete_is_idempotent() {
        let root = temp_root("missing");
        let fs = FsBlobStorage::new(&root);
        assert!(fs.get("nope.bin").await.is_err());
        assert!(fs.delete("nope.bin").await.is_ok());
        assert!(!root.join("nope.bin").exists());
        std::fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn fs_rejects_traversal_keys() {
        let root = temp_root("traversal");
        let fs = FsBlobStorage::new(&root);
        assert!(fs.put("../escape.txt", b"x").await.is_err());
        assert!(fs.put("a/../../escape.txt", b"x").await.is_err());
        assert!(fs.put("/abs.txt", b"x").await.is_err());
        assert!(fs.put("", b"x").await.is_err());
        assert_eq!(std::fs::read_dir(&root).unwrap().count(), 0);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn factory_switches_backend() {
        let root = temp_root("factory");
        let db: Arc<dyn BlobStorage> = storage_from("", None);
        assert!(db.get("k").await.is_err()); // Db 占位 get 必然 Err

        let fs_default: Arc<dyn BlobStorage> =
            storage_from("FS", Some(root.to_str().unwrap())); // 大小写不敏感
        fs_default.put("f.txt", b"z").await.unwrap();
        assert!(root.join("f.txt").is_file());

        let fallback: Arc<dyn BlobStorage> = storage_from("unknown-backend", None);
        assert!(fallback.get("k").await.is_err());
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    #[should_panic(expected = "STORAGE_BACKEND=fs requires STORAGE_ROOT")]
    fn factory_fs_without_root_panics() {
        let _ = storage_from("fs", None);
    }
}
