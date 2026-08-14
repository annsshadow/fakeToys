## eCryptfs 文件系统的加密密钥


ECryptfs 是一个堆叠式文件系统，它使用随机生成的文件加密密钥（FEK）对每个文件进行透明加密与解密。

每个 FEK 又由一个文件加密密钥加密密钥（FEKEK）加密，可以在内核空间进行，也可以由名为 'ecryptfsd' 的用户空间守护进程进行。前者的情况下，操作由内核 CryptoAPI 直接使用一个由用户输入口令派生出的密钥（FEKEK）执行；后者的情况下，FEK 由 'ecryptfsd' 借助外部库进行加密，以支持公钥密码学、PKCS#11 以及基于 TPM 的其他机制。

eCryptfs 定义的数据结构，用于包含 FEK 解密所需的信息，称为认证令牌（authentication token），目前可以存储在 'user' 类型的内核密钥中，由随 'ecryptfs-utils' 软件包提供的用户空间工具 'mount.ecryptfs' 插入到用户特定会话的密钥环中。

为了与 eCryptfs 文件系统配合使用，'encrypted' 密钥类型通过引入新的 'ecryptfs' 格式进行了扩展。新引入格式的加密密钥在其载荷中存储一个认证令牌，其中的 FEKEK 由内核随机生成，并由父主密钥保护。

为了避免已知明文攻击，通过 'keyctl print' 或 'keyctl pipe' 命令获得的 datablob 不包含整体认证令牌（其内容众所周知），而只包含加密形式的 FEKEK。

eCryptfs 文件系统确实可以从使用加密密钥中获益，因为所需密钥可以由管理员安全地生成，并在解封一个 'trusted' 密钥之后于引导时提供，以便在受控环境中执行挂载。另一个优势是，该密钥不会暴露于恶意软件的威胁之下，因为它仅在内核层以明文形式可用。

```

   keyctl add encrypted name "new ecryptfs key-type:master-key-name keylen" ring
   keyctl add encrypted name "load hex_blob" ring
   keyctl update keyid "update key-type:master-key-name"

```
```

	name:= '<16 个十六进制字符>'
	key-type:= 'trusted' | 'user'
	keylen:= 64


```

使用 eCryptfs 文件系统的加密密钥示例：

创建一个长度为 64 字节、格式为 'ecryptfs' 的加密密钥 "1000100010001000"
```

    $ keyctl add encrypted 1000100010001000 "new ecryptfs user:test 64" @u
    19184530

    $ keyctl print 19184530
    ecryptfs user:test 64 490045d4bfe48c99f0d465fbbbb79e7500da954178e2de0697
    dd85091f5450a0511219e9f7cd70dcd498038181466f78ac8d4c19504fcc72402bfc41c2
    f253a41b7507ccaa4b2b03fff19a69d1cc0b16e71746473f023a95488b6edfd86f7fdd40
    9d292e4bacded1258880122dd553a661

    $ keyctl pipe 19184530 > ecryptfs.blob

```

使用所创建的加密密钥 "1000100010001000" 挂载 eCryptfs 文件系统
```

    $ mount -i -t ecryptfs -oecryptfs_sig=1000100010001000,\
      ecryptfs_cipher=aes,ecryptfs_key_bytes=32 /secret /secret

```
