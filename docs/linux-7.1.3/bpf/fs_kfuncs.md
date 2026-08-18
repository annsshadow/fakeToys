
## BPF 鏂囦欢绯荤粺 kfunc


BPF LSM 绋嬪簭闇€瑕佷粠 LSM 閽╁瓙璁块棶鏂囦欢绯荤粺鏁版嵁銆傚彲浣跨敤浠ヤ笅 BPF kfunc 鏉ヨ幏鍙栬繖浜涙暟鎹€?

 - `bpf_get_file_xattr()`

 - `bpf_get_fsverity_digest()`

涓洪伩鍏嶉€掑綊锛岃繖浜?kfunc 閬靛惊浠ヤ笅瑙勫垯锛?

1. 杩欎簺 kfunc 浠呭厑璁稿湪 BPF LSM 鍑芥暟涓娇鐢ㄣ€?
2. 杩欎簺 kfunc 涓嶅簲璋冪敤鍏朵粬 LSM 閽╁瓙锛屽嵆 security_*()銆備緥濡傦紝`bpf_get_file_xattr()`
   涓嶄娇鐢?`vfs_getxattr()`锛屽洜涓哄悗鑰呬細璋冪敤 LSM 閽╁瓙 `security_inode_getxattr`銆?
