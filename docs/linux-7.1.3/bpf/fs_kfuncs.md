
## BPF 文件系统 kfunc


BPF LSM 程序需要从 LSM 钩子访问文件系统数据。可使用以下 BPF kfunc 来获取这些数据。

 - `bpf_get_file_xattr()`

 - `bpf_get_fsverity_digest()`

为避免递归，这些 kfunc 遵循以下规则：

1. 这些 kfunc 仅允许在 BPF LSM 函数中使用。
2. 这些 kfunc 不应调用其他 LSM 钩子，即 security_*()。例如，`bpf_get_file_xattr()`
   不使用 `vfs_getxattr()`，因为后者会调用 LSM 钩子 `security_inode_getxattr`。
