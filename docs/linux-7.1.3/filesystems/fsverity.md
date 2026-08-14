

## fs-verity：基于文件的只读真实性保护


## 简介


fs-verity（`fs/verity/`）是一个支撑层，文件系统可以挂接（hook）到它上面，
以支持对只读文件进行透明的完整性与真实性保护。目前，ext4、f2fs 和
btrfs 文件系统支持它。与 fscrypt 类似，支持 fs-verity 所需的
文件系统特定代码并不多。

fs-verity 类似于 `dm-verity
<https://www.kernel.org/doc/Documentation/admin-guide/device-mapper/verity.rst>`_，
但它作用于文件而非块设备。在支持 fs-verity 的文件系统上的普通文件中，
用户空间可以执行一个 ioctl，让文件系统为该文件构建一棵 Merkle 树，并将其
持久化到与该文件相关联的文件系统特定位置。

此后，该文件变为只读，对该文件的所有读取都会根据文件的 Merkle 树自动
进行验证。任何损坏数据的读取（包括 mmap 读取）都会失败。

用户空间可以使用另一个 ioctl 来检索 fs-verity 正在为该文件强制执行的
根哈希（实际上是“fs-verity 文件摘要”，这是一个包含了 Merkle 树根哈希的
哈希值）。无论文件大小如何，该 ioctl 都在常数时间内执行。

fs-verity 本质上是一种在常数时间内对文件进行哈希的方法，前提条件是
违反该哈希的读取会在运行时失败。

## 使用场景


fs-verity 本身仅提供完整性保护，即检测意外（非恶意）的损坏。

然而，由于 fs-verity 让检索文件哈希变得极其高效，它主要被用作一个工具
来支持认证（检测恶意修改）或审计（在使用前记录文件哈希）。

可以使用标准的文件哈希来替代 fs-verity。但是，如果文件很大且只有一小部分
可能被访问，这样做效率很低。例如 Android 应用程序包（APK）文件通常就是
这种情况。它们通常包含许多翻译、类以及其他资源，在特定设备上很少甚至
从未被访问。在启动应用程序之前读取并哈希整个文件既缓慢又浪费。

与提前（ahead-of-time）计算的哈希不同，fs-verity 还会在每次数据被分页读入
时重新验证。这确保了恶意的磁盘固件无法在运行时悄悄更改文件内容。

fs-verity 不会取代或过时 dm-verity。在只读文件系统上仍应使用 dm-verity。
fs-verity 适用于那些必须位于可读写文件系统上的文件，因为它们是独立更新
且可能由用户安装的，因此无法使用 dm-verity。

fs-verity 不强制要求任何特定的方案来对它的文件哈希进行认证。（类似地，
dm-verity 也不强制要求任何特定的方案来认证其块设备根哈希。）用于认证
fs-verity 文件哈希的方案包括：

- 受信任的用户空间代码。通常，访问文件的用户空间代码可以被信任来认证
  它们。例如，考虑一个想要在使用数据文件之前先认证它们的应用程序，或者
  一个作为操作系统一部分的应用程序加载器（它已经通过其他方式被认证，例如
  从使用 dm-verity 的只读分区加载），并且想要在加载应用程序之前先认证它们。
  在这些情况下，这些受信任的用户空间代码可以通过使用 `FS_IOC_MEASURE_VERITY`_
  检索文件的 fs-verity 摘要，然后使用任何支持数字签名的用户空间加密库
  对其签名进行验证，从而认证文件内容。

- 完整性度量架构（IMA）。IMA 支持将 fs-verity 文件摘要作为其传统全文件
  摘要的替代方案。“IMA 评估（appraisal）”会根据 IMA 策略强制要求文件在其
  “security.ima”扩展属性中包含有效的、匹配的签名。更多信息请参阅 IMA 文档。

- 完整性策略执行（IPE）。IPE 支持基于文件的不可变安全属性来实施访问控制
  决策，包括那些受 fs-verity 内建签名保护的文件。“IPE 策略”特别允许使用
  属性 `fsverity_digest` 通过 verity 摘要来识别文件，并使用 `fsverity_signature`
  来授权带有经过验证的 fs-verity 内建签名的文件。关于配置 IPE 策略及其
  运行模式的详细信息，请参阅 [IPE admin guide </admin-guide/LSM/ipe>](IPE admin guide </admin-guide/LSM/ipe>)。

- 受信任的用户空间代码与 `内建签名验证`_ 结合使用。这种方法应极其谨慎地
  使用。

## 用户 API


### FS_IOC_ENABLE_VERITY


`FS_IOC_ENABLE_VERITY` ioctl 在文件上启用 fs-verity。它接受一个指向
struct fsverity_enable_arg 的指针，其定义如下：

```

    struct fsverity_enable_arg {
            __u32 version;
            __u32 hash_algorithm;
            __u32 block_size;
            __u32 salt_size;
            __u64 salt_ptr;
            __u32 sig_size;
            __u32 __reserved1;
            __u64 sig_ptr;
            __u64 __reserved2[11];
    };

```
该结构包含要为文件构建的 Merkle 树的参数。必须按如下方式初始化：

- `version` 必须为 1。
- `hash_algorithm` 必须是用于 Merkle 树的哈希算法的标识符，例如
  FS_VERITY_HASH_ALG_SHA256。可能的值列表请参见
  `include/uapi/linux/fsverity.h`。
- `block_size` 是 Merkle 树的块大小（以字节为单位）。在 Linux v6.3 及之后
  版本中，它可以是（含边界）1024 到系统页大小与文件系统块大小二者最小值
  之间的任意 2 的幂。在更早的版本中，页大小是唯一允许的值。
- `salt_size` 是盐值的大小（以字节为单位），如果不提供盐值则为 0。盐值
  是一个被前置到每个被哈希块之前的值；它可用于针对特定文件或设备对
  哈希进行个性化处理。目前最大盐值大小为 32 字节。
- `salt_ptr` 是指向盐值的指针，如果不提供盐值则为 NULL。
- `sig_size` 是内建签名的大小（以字节为单位），如果不提供内建签名则为 0。
  目前内建签名（有些随意地）被限制为 16128 字节。
- `sig_ptr` 是指向内建签名的指针，如果不提供内建签名则为 NULL。只有在
  使用 `内建签名验证`_ 特性时才需要内建签名。IMA 评估不需要它，如果
  文件签名完全在用户空间处理，也不需要它。
- 所有保留字段必须清零。

FS_IOC_ENABLE_VERITY 使文件系统为文件构建 Merkle 树，并将其持久化到与文件
关联的文件系统特定位置，然后将文件标记为 verity 文件。该 ioctl 在处理大文件
时可能需要很长时间，并且可以被致命信号中断。

FS_IOC_ENABLE_VERITY 会检查对 inode 的写访问权限。但是，它必须在 O_RDONLY
文件描述符上执行，并且不能有任何进程将文件以写方式打开。在该 ioctl 执行期间
尝试以写方式打开文件将失败并返回 ETXTBSY。（这对于保证在 verity 启用后不会
存在可写文件描述符，并保证在构建 Merkle 树期间文件内容保持稳定是必要的。）

成功后，FS_IOC_ENABLE_VERITY 返回 0，文件成为 verity 文件。失败时（包括
被致命信号中断的情况），不会对文件做任何更改。

FS_IOC_ENABLE_VERITY 可能因以下错误而失败：

- `EACCES`：进程对文件没有写访问权限
- `EBADMSG`：内建签名格式错误
- `EBUSY`：该 ioctl 已在文件上运行
- `EEXIST`：文件已经启用了 verity
- `EFAULT`：调用者提供了不可访问的内存
- `EFBIG`：文件太大，无法在其上启用 verity
- `EINTR`：操作被致命信号中断
- `EINVAL`：不支持的版本、哈希算法或块大小；或者设置了保留位；或者文件
  描述符既不指向普通文件也不指向目录。
- `EISDIR`：文件描述符指向一个目录
- `EKEYREJECTED`：内建签名与文件不匹配
- `EMSGSIZE`：盐值或内建签名过长
- `ENOKEY`：“.fs-verity”密钥环不包含验证内建签名所需的证书
- `ENOPKG`：fs-verity 识别该哈希算法，但在当前内核配置中不可用
- `ENOTTY`：此类文件系统没有实现 fs-verity
- `EOPNOTSUPP`：内核未配置 fs-verity 支持；或者文件系统超级块未启用
  'verity' 特性；或者文件系统不支持在该文件上使用 fs-verity。
  （参见 `文件系统支持`_。）
- `EPERM`：文件是仅追加（append-only）的；或者需要内建签名但未提供。
- `EROFS`：文件系统是只读的
- `ETXTBSY`：有人将文件以写方式打开。这可能是调用者的文件描述符、另一个
  打开的文件描述符，或者由可写内存映射持有的文件引用。

### FS_IOC_MEASURE_VERITY


`FS_IOC_MEASURE_VERITY` ioctl 检索一个 verity 文件的摘要。fs-verity 文件
摘要是一个加密摘要，用于标识在读取时强制执行的文件内容；它通过 Merkle 树
计算，不同于传统的全文件摘要。

```

    struct fsverity_digest {
            __u16 digest_algorithm;
            __u16 digest_size; /* input/output */
            __u8 digest[];
    };

```
`digest_size` 是一个输入/输出字段。在输入时，它必须初始化为为变长
`digest` 字段分配的内存的字节数。

成功后，返回 0，内核按如下方式填充结构：

- `digest_algorithm` 将是文件摘要使用的哈希算法。它将与
  `fsverity_enable_arg::hash_algorithm` 相匹配。
- `digest_size` 将是摘要的大小（以字节为单位），例如 SHA-256 为 32。
  （这可能和 `digest_algorithm` 冗余。）
- `digest` 将是摘要的实际字节。

FS_IOC_MEASURE_VERITY 保证在常数时间内执行，与文件大小无关。

FS_IOC_MEASURE_VERITY 可能因以下错误而失败：

- `EFAULT`：调用者提供了不可访问的内存
- `ENODATA`：文件不是 verity 文件
- `ENOTTY`：此类文件系统没有实现 fs-verity
- `EOPNOTSUPP`：内核未配置 fs-verity 支持，或者文件系统超级块未启用
  'verity' 特性。（参见 `文件系统支持`_。）
- `EOVERFLOW`：摘要比指定的 `digest_size` 字节更长。尝试提供更大的缓冲区。

### FS_IOC_READ_VERITY_METADATA


`FS_IOC_READ_VERITY_METADATA` ioctl 从一个 verity 文件读取 verity 元数据。
该 ioctl 自 Linux v5.12 起可用。

当 verity 验证应在当前运行的内核之外的地方执行时，该 ioctl 很有用。

一个例子是，一个服务器程序接收到一个 verity 文件并将其提供给客户端程序，
以便客户端可以自行对文件进行 fs-verity 兼容的验证。这只有在客户端不信任
服务器、且服务器需要为客户端提供存储时才有意义。

另一个例子是在用户空间创建文件系统映像时（例如使用 `mkfs.ext4 -d`）复制
verity 元数据。

这是一个相当专门的用例，大多数 fs-verity 用户并不需要这个 ioctl。

```

   #define FS_VERITY_METADATA_TYPE_MERKLE_TREE     1
   #define FS_VERITY_METADATA_TYPE_DESCRIPTOR      2
   #define FS_VERITY_METADATA_TYPE_SIGNATURE       3

   struct fsverity_read_metadata_arg {
           __u64 metadata_type;
           __u64 offset;
           __u64 length;
           __u64 buf_ptr;
           __u64 __reserved;
   };

```
`metadata_type` 指定要读取的元数据类型：

- `FS_VERITY_METADATA_TYPE_MERKLE_TREE` 读取 Merkle 树的块。这些块按从根层
  到叶层的顺序返回。在每一层内，块按照其哈希本身被哈希的顺序返回。
  更多信息参见 `Merkle 树`_。

- `FS_VERITY_METADATA_TYPE_DESCRIPTOR` 读取 fs-verity 描述符。
  参见 `fs-verity 描述符`_。

- `FS_VERITY_METADATA_TYPE_SIGNATURE` 读取传递给 FS_IOC_ENABLE_VERITY 的
  内建签名（如果有的话）。参见 `内建签名验证`_。

其语义与 `pread()` 类似。`offset` 指定要从元数据项中读取的字节偏移量，
`length` 指定要从元数据项中读取的最大字节数。`buf_ptr` 是指向要读入的
缓冲区的指针，被转换为 64 位整数。`__reserved` 必须为 0。成功时，返回
读取的字节数。在元数据项末尾返回 0。返回的字节数可能小于 `length`，
例如，如果 ioctl 被中断。

FS_IOC_READ_VERITY_METADATA 返回的元数据不保证针对 `FS_IOC_MEASURE_VERITY`_
所返回的文件摘要进行认证，因为该元数据本来就会用于实现 fs-verity 兼容的
验证（尽管在没有恶意磁盘的情况下，元数据确实会匹配）。例如，为了实现
这个 ioctl，文件系统被允许只是从磁盘读取 Merkle 树块，而不会真正验证到
根节点的路径。

FS_IOC_READ_VERITY_METADATA 可能因以下错误而失败：

- `EFAULT`：调用者提供了不可访问的内存
- `EINTR`：在读取任何数据之前 ioctl 被中断
- `EINVAL`：设置了保留字段，或者 `offset + length` 溢出
- `ENODATA`：文件不是 verity 文件，或者请求了
  FS_VERITY_METADATA_TYPE_SIGNATURE 但文件没有内建签名
- `ENOTTY`：此类文件系统没有实现 fs-verity，或者尚未在其上实现该 ioctl
- `EOPNOTSUPP`：内核未配置 fs-verity 支持，或者文件系统超级块未启用
  'verity' 特性。（参见 `文件系统支持`_。）

### FS_IOC_GETFLAGS


已有的 ioctl FS_IOC_GETFLAGS（它并非 fs-verity 特有）也可以用来检查文件
是否启用了 fs-verity。要做到这一点，检查返回的标志中的 FS_VERITY_FL
（0x00100000）。

verity 标志不能通过 FS_IOC_SETFLAGS 设置。你必须改用 FS_IOC_ENABLE_VERITY，
因为需要提供参数。

### statx


自 Linux v5.5 起，如果文件启用了 fs-verity，statx() 系统调用会设置
STATX_ATTR_VERITY。这可能比 FS_IOC_GETFLAGS 和 FS_IOC_MEASURE_VERITY
性能更好，因为它不需要打开文件，而打开 verity 文件可能开销较大。

### FS_IOC_FSGETXATTR


自 Linux v7.0 起，当文件启用了 verity 时，FS_IOC_FSGETXATTR ioctl 会在
返回的 flags 中设置 FS_XFLAG_VERITY（0x00020000）。注意，由于启用 verity
需要输入参数，该属性不能通过 FS_IOC_FSSETXATTR 设置。参见 FS_IOC_ENABLE_VERITY。

### file_getattr


自 Linux v7.0 起，当文件启用了 verity 时，file_getattr() 系统调用会在
返回的 flags 中设置 FS_XFLAG_VERITY（0x00020000）。注意，由于启用 verity
需要输入参数，该属性不能通过 file_setattr() 设置。参见 FS_IOC_ENABLE_VERITY。


## 访问 verity 文件


应用程序可以像访问非 verity 文件一样透明地访问 verity 文件，但有以下
例外：

- verity 文件是只读的。即使文件模式位允许，它们也不能以写方式打开或进行
  truncate()，尝试做这些事情之一将失败并返回 EPERM。但是，对元数据（如
  所有者、模式、时间戳和 xattr）的更改仍然被允许，因为它们不被 fs-verity
  度量。verity 文件也仍然可以被重命名、删除和建立硬链接。

- verity 文件不支持直接 I/O。在此类文件上使用直接 I/O 会回退到缓冲 I/O。

- DAX（直接访问）在 verity 文件上不受支持，因为这会绕过数据验证。

- 与 verity Merkle 树不匹配的数据读取将失败，返回 EIO（对于 read()）或
  SIGBUS（对于 mmap() 读取）。

- 如果 sysctl “fs.verity.require_signatures” 被设置为 1，且文件未被
  “.fs-verity”密钥环中的密钥签名，那么打开该文件将失败。参见
  `内建签名验证`_。

不支持直接访问 Merkle 树。因此，如果复制了一个 verity 文件，或者对其进行
了备份和恢复，那么它将失去其“verity”特性。fs-verity 主要适用于像由
包管理器管理的可执行文件这样的文件。

## 文件摘要计算


本节描述 fs-verity 如何使用 Merkle 树对文件内容进行哈希，以生成从密码学上
标识文件内容的摘要。对于所有支持 fs-verity 的文件系统，该算法都是相同的。

只有当用户空间需要自己计算 fs-verity 文件摘要（例如为了对文件进行签名）
时，才需要了解该算法。


### Merkle 树


文件内容被划分为块，块大小可配置，但通常是 4096 字节。如有需要，最后一个
块的末尾会用零填充。然后对每个块进行哈希，生成第一层哈希。接着，将这一层
中的哈希分组到 'blocksize' 字节的块中（按需对末尾用零填充），并对这些块
进行哈希，生成第二层哈希。这样沿着树向上进行，直到只剩下一个块。这个块的
哈希就是“Merkle 树根哈希”。

如果文件能放入一个块且非空，那么“Merkle 树根哈希”就是单个数据块的哈希。
如果文件为空，那么“Merkle 树根哈希”全为零。

这里的“块”不一定与“文件系统块”相同。

如果指定了盐值，则将其用零填充到哈希算法压缩函数输入大小的最接近倍数，
例如 SHA-256 为 64 字节，SHA-512 为 128 字节。填充后的盐值被前置到每个
被哈希的数据或 Merkle 树块之前。

块填充的目的是使每次哈希都在相同数量的数据上进行，这简化了实现并为硬件
加速保留了更多可能性。盐值填充的目的是，当带盐哈希状态被预计算、然后为
每次哈希导入时，使加盐变得“免费”。

示例：在 SHA-256 和 4K 块的推荐配置下，每个块中可以容纳 128 个哈希值。
因此，Merkle 树的每一层大约比上一层小 128 倍，对于大文件，Merkle 树的大小
收敛到原始文件大小的大约 1/127。但是，对于小文件，填充占比显著，使得空间
开销成比例地更大。


### fs-verity 描述符


单凭 Merkle 树根哈希是含糊的。例如，它无法区分一个大文件与一个数据恰好是
第一个文件顶层哈希块的较小的第二个文件。填充到下一个块边界的约定也会产生
歧义。

为了解决这个问题，fs-verity 文件摘要实际上是作为以下结构的哈希来计算的，
该结构包含了 Merkle 树

```

    struct fsverity_descriptor {
            __u8 version;           /* must be 1 */
            __u8 hash_algorithm;    /* Merkle tree hash algorithm */
            __u8 log_blocksize;     /* log2 of size of data and tree blocks */
            __u8 salt_size;         /* size of salt in bytes; 0 if none */
            __le32 __reserved_0x04; /* must be 0 */
            __le64 data_size;       /* size of file the Merkle tree is built over */
            __u8 root_hash[64];     /* Merkle tree root hash */
            __u8 salt[32];          /* salt prepended to each hashed block */
            __u8 __reserved[144];   /* must be 0's */
    };

```
## 内建签名验证


CONFIG_FS_VERITY_BUILTIN_SIGNATURES=y 增加了对 fs-verity 内建签名进行
内核内验证的支持。

**重要**！在使用此特性之前请务必极其谨慎。它不是使用 fs-verity 进行签名的
唯一方式，而且替代方案（例如用户空间签名验证，以及 IMA 评估）可能好得多。
也很容易落入这样的陷阱：认为此特性解决的问题比它实际能解决的更多。

启用此选项会增加以下内容：

1. 在启动时，内核创建一个名为 “.fs-verity”的密钥环。root 用户可以使用
   add_key() 系统调用将受信任的 X.509 证书添加到这个密钥环中。

2. `FS_IOC_ENABLE_VERITY`_ 接受指向文件 fs-verity 摘要的、DER 格式的
   PKCS#7 分离签名（detached signature）的指针。成功后，该 ioctl 将签名
   与 Merkle 树一起持久化。然后，无论何时打开文件，内核都会使用 “.fs-verity”
   密钥环中的证书，根据该签名验证文件的实际摘要。只要文件的签名存在，无论
   下一项中描述的 sysctl 变量 “fs.verity.require_signatures” 的状态如何，
   都会进行此验证。IPE LSM 依赖此行为来识别和标记包含经过验证的内建
   fsverity 签名的 fsverity 文件。

3. 提供了一个新的 sysctl “fs.verity.require_signatures”。当设置为 1 时，
   内核要求所有 verity 文件都具有如（2）中所述正确签名的摘要。

（2）中描述的签名所签名的数据必须是以下结构的签名：

```

    struct fsverity_formatted_digest {
            char magic[8];                  /* must be "FSVerity" */
            __le16 digest_algorithm;
            __le16 digest_size;
            __u8 digest[];
    };

```
仅此而已。应该再次强调，fs-verity 内建签名不是使用 fs-verity 进行签名的
唯一方式。有关 fs-verity 可用方式的概述，请参见 `使用场景`_。
fs-verity 内建签名有一些在使用前应仔细考虑的重大限制：

- 内建签名验证**不**会让内核强制要求任何文件实际启用了 fs-verity。因此，
  它不是一个完整的认证策略。目前，如果使用它，完成认证策略的一种方式是
  让受信任的用户空间代码在访问文件之前显式检查文件是否具有带签名的
  fs-verity 启用。（在 fs.verity.require_signatures=1 的情况下，只需检查
  是否启用了 fs-verity 就足够了。）但是，在这种情况下，受信任的用户空间
  代码本就可以将签名与文件一起存储，并使用加密库自行验证，而不必使用此
  特性。

- 另一种方法是利用 fs-verity 内建签名验证与 IPE LSM 结合，IPE LSM 支持
  定义一种内核强制的、系统级的认证策略，该策略只允许带有经过验证的
  fs-verity 内建签名的文件执行某些操作，例如执行。注意 IPE 不要求
  fs.verity.require_signatures=1。更多详情请参阅
  [IPE admin guide </admin-guide/LSM/ipe>](IPE admin guide </admin-guide/LSM/ipe>)。

- 文件的内建签名只能在为文件启用 fs-verity 的同时设置。之后更改或删除
  内建签名需要重新创建文件。

- 内建签名验证对系统上所有启用了 fs-verity 的文件使用同一组公钥。不能为
  不同文件信任不同的密钥；每个密钥都是全有或全无。

- sysctl fs.verity.require_signatures 是系统级的。将其设置为 1 只有在
  系统上所有 fs-verity 用户都同意应设置为 1 时才能生效。此限制可能会阻止
  fs-verity 在有帮助的情况下被使用。

- 内建签名验证只能使用内核支持的签名算法。例如，内核尚不支持 Ed25519，
  尽管这通常是推荐用于新加密设计的签名算法。

- fs-verity 内建签名采用 PKCS#7 格式，公钥采用 X.509 格式。这些格式被广泛
  使用，包括被一些其他内核特性使用（这就是 fs-verity 内建签名使用它们的
  原因），并且功能非常丰富。遗憾的是，历史表明，解析和处理这些格式（它们
  来自 1990 年代，基于 ASN.1）的代码常常因其复杂性而产生漏洞。这种复杂性
  并非密码学本身所固有。

  fs-verity 用户如果不需要 X.509 和 PKCS#7 的高级特性，应强烈考虑使用更
  简单的格式，例如纯 Ed25519 密钥和签名，并在用户空间验证签名。

  fs-verity 用户如果无论如何选择使用 X.509 和 PKCS#7，仍应考虑在用户空间
  验证这些签名更灵活（由于本文档前面提到的其他原因），并且消除了启用
  CONFIG_FS_VERITY_BUILTIN_SIGNATURES 及其带来的内核攻击面增加的必要性。
  在某些情况下这甚至是必要的，因为高级的 X.509 和 PKCS#7 特性并不总是能
  按预期与内核一起工作。例如，内核不检查 X.509 证书的有效期时间。

  注意：支持 fs-verity 的 IMA 评估不为其签名使用 PKCS#7，因此它部分避免了
  这里讨论的问题。IMA 评估确实使用 X.509。

## 文件系统支持


fs-verity 受到多个文件系统的支持，如下所述。要在这其中任何文件系统上使用
fs-verity，必须启用 CONFIG_FS_VERITY kconfig 选项。

`include/linux/fsverity.h` 声明了 `fs/verity/` 支撑层与文件系统之间的
接口。简而言之，文件系统必须提供一个 `fsverity_operations` 结构，该结构
提供将 verity 元数据读取和写入文件系统特定位置（包括 Merkle 树块和
`fsverity_descriptor`）的方法。文件系统还必须在特定时机调用 `fs/verity/`
中的函数，例如在打开文件时，或当页已被读入页缓存（pagecache）时。
（参见 `验证数据`_。）

### ext4


ext4 自 Linux v5.4 和 e2fsprogs v1.45.2 起支持 fs-verity。

要在 ext4 文件系统上创建 verity 文件，该文件系统必须用 `-O verity` 格式化，
或对其运行过 `tune2fs -O verity`。“verity”是一个 RO_COMPAT 文件系统特性，
因此一旦设置，旧内核将只能以只读方式挂载该文件系统，旧版本的 e2fsck 将
无法检查该文件系统。

最初，带有 “verity” 特性的 ext4 文件系统只能在其块大小等于系统页大小
（通常为 4096 字节）时挂载。在 Linux v6.3 中，移除了此限制。

ext4 在 verity 文件上设置磁盘 inode 标志 EXT4_VERITY_FL。它只能由
`FS_IOC_ENABLE_VERITY`_ 设置，且不能被清除。

ext4 也支持加密，它可以与 fs-verity 同时使用。在这种情况下，被验证的是
明文数据而非密文。为了使 fs-verity 文件摘要有意义，这是必要的，因为每个
文件都以不同方式加密。

ext4 将 verity 元数据（Merkle 树和 fsverity_descriptor）存储在文件末尾之外，
从超出 i_size 的第一个 64K 边界处开始。这种方法可行是因为 (a) verity 文件
是只读的，且 (b) 完全超出 i_size 的页对用户空间不可见，但 ext4 可以仅通过
对 ext4 做一些相对较小的改动在内部对其进行读/写。这种方法避免依赖 EA_INODE
特性，也避免重新架构 ext4 的 xattr 支持以支持将多 GB 的 xattr 分页读入内存
并支持加密 xattr。注意，当文件被加密时，verity 元数据**必须**被加密，因为
它包含明文数据的哈希。

ext4 只允许在基于 extent 的文件上启用 verity。

### f2fs


f2fs 自 Linux v5.4 和 f2fs-tools v1.11.0 起支持 fs-verity。

要在 f2fs 文件系统上创建 verity 文件，该文件系统必须用 `-O verity` 格式化。

f2fs 在 verity 文件上设置磁盘 inode 标志 FADVISE_VERITY_BIT。它只能由
`FS_IOC_ENABLE_VERITY`_ 设置，且不能被清除。

与 ext4 类似，f2fs 将 verity 元数据（Merkle 树和 fsverity_descriptor）存储
在文件末尾之外，从超出 i_size 的第一个 64K 边界处开始。参见上面 ext4 的说明。
此外，f2fs 每个 inode 最多支持 4096 字节的 xattr 条目，这通常甚至不够一个
Merkle 树块使用。

f2fs 不支持在当前有待处理的原子写入或易失写入的文件上启用 verity。

### btrfs


btrfs 自 Linux v5.15 起支持 fs-verity。启用了 verity 的 inode 被标记一个
RO_COMPAT inode 标志，verity 元数据存储在独立的 btree 条目中。

## 实现细节


### 验证数据


fs-verity 确保对 verity 文件数据的所有读取都经过验证，无论使用哪个系统调用
进行读取（例如 mmap()、read()、pread()），也无论这是第一次读取还是后续读取
（除非后续读取可以返回已被缓存且已经验证的数据）。下面我们描述文件系统如何
实现这一点。

#### 页缓存（Pagecache）


对于使用 Linux 页缓存的文件系统，`->read_folio()` 和 `->readahead()` 方法
必须被修改为在 folio 被标记为 Uptodate 之前验证它们。仅仅挂钩 `->read_iter()`
是不够的，因为 `->read_iter()` 不用于内存映射。

因此，fs/verity/ 提供了函数 fsverity_verify_blocks()，它验证已被读入 verity
inode 页缓存的数据。包含的 folio 必须仍然被锁定且不是 Uptodate，因此它还
不能被用户空间读取。为了进行验证，fsverity_verify_blocks() 会回调文件系统，
通过 fsverity_operations 中的 **read_merkle_tree_page()** 读取哈希块。

fsverity_verify_blocks() 在验证失败时返回 false；在这种情况下，文件系统
不得将该 folio 设置为 Uptodate。此后，按照 Linux 页缓存的通常行为，用户空间
尝试从该 folio 所在文件部分进行 read() 将失败并返回 EIO，而在内存映射中访问
该 folio 将引发 SIGBUS。

原则上，验证一个数据块需要验证从数据块到根哈希的 Merkle 树中的整条路径。
但是，为了效率，文件系统可以缓存哈希块。因此，fsverity_verify_blocks() 只
向上遍历树、读取哈希块，直到看到一个已经验证过的哈希块。然后它验证到该块的
路径。

这种优化（dm-verity 也使用）带来了出色的顺序读取性能。这是因为通常（例如
对于 4K 块和 SHA-256 是 128 次中的 127 次）树最底层的哈希块已经被缓存并检查过，
这来自读取前一个数据块。但是，随机读取的性能较差。

#### 基于块设备的文件系统


基于块设备的文件系统（例如 ext4 和 f2fs）在 Linux 中也使用页缓存，因此上一
小节同样适用。但是，它们通常也会一次性从文件中读取许多数据块，分组到称为
“bio”的结构中。为了让这些类型的文件系统更容易支持 fs-verity，fs/verity/
还提供了一个函数 fsverity_verify_bio()，它验证一个 bio 中的所有数据块。

ext4 和 f2fs 也支持加密。如果一个 verity 文件同时也被加密，数据必须在验证
之前被解密。为了支持这一点，这些文件系统为

```

    struct bio_post_read_ctx {
           struct bio *bio;
           struct work_struct work;
           unsigned int cur_step;
           unsigned int enabled_steps;
    };

```
分配一个“读后上下文（post-read context）”。`enabled_steps` 是一个位掩码，
指定启用了解密、verity 还是两者。在 bio 完成后，对于每个需要的后处理步骤，
文件系统将该 bio_post_read_ctx 排入一个工作队列，然后工作队列的工作进行
解密或验证。最后，未发生解密或 verity 错误的 folio 被标记为 Uptodate，并
被解锁。

在许多文件系统上，文件可以包含空洞（hole）。通常，`->readahead()` 只是将
空洞块清零，并认为相应的数据是最新的；不会发出任何 bio。为了防止这种情况
绕过 fs-verity，文件系统使用 fsverity_verify_blocks() 来验证空洞块。

文件系统还会在 verity 文件上禁用直接 I/O，否则直接 I/O 会绕过 fs-verity。

## 用户空间工具


本文档侧重于内核，但 fs-verity 的用户空间工具可在此处找到：

	https://git.kernel.org/pub/scm/fs/fsverity/fsverity-utils.git

有关设置受 fs-verity 保护的文件的详细信息，请参阅 fsverity-utils 源代码树中
的 README.md 文件。

## 测试


要测试 fs-verity，请使用 xfstests。例如，使用 `kvm-xfstests

```

    kvm-xfstests -c ext4,f2fs,btrfs -g verity

```
## 常见问题解答（FAQ）


本节回答关于 fs-verity 的、本文档其他部分没有直接回答的常见问题。

:Q: 为什么 fs-verity 不是 IMA 的一部分？
:A: fs-verity 和 IMA（完整性度量架构）侧重点不同。fs-verity 是一个文件系统级
    的机制，用于使用 Merkle 树对单个文件进行哈希。相比之下，IMA 指定了一个
    系统级策略，说明哪些文件被哈希以及如何处理这些哈希，例如记录它们、认证
    它们，或将它们添加到度量列表。

    IMA 支持将 fs-verity 哈希机制作为全文件哈希的替代方案，供那些想要 Merkle
    树哈希带来的性能和安全好处的人使用。但是，强制所有 fs-verity 的使用都
    通过 IMA 是没有意义的。fs-verity 即使作为一个独立的文件系统特性也已经
    满足了众多用户的需求，并且它可以像其他文件系统特性（例如使用 xfstests）
    一样被测试。

:Q: fs-verity 不是没用吗？因为攻击者可以修改存储在磁盘上的 Merkle 树中的
    哈希。
:A: 要验证一个 fs-verity 文件的真实性，你必须验证“fs-verity 文件摘要”的
    真实性，它包含了 Merkle 树的根哈希。参见 `使用场景`_。

:Q: fs-verity 不是没用吗？因为攻击者可以直接用一个非 verity 文件替换 verity
    文件。
:A: 参见 `使用场景`_。在最初的使用场景中，真正由受信任的用户空间代码来认证
    文件；fs-verity 只是高效、安全地完成这项工作的一个工具。受信任的用户空间
    代码会将非 verity 文件视为不真实。

:Q: 为什么 Merkle 树需要存储在磁盘上？难道不能只存储根哈希吗？
:A: 如果 Merkle 树不存储在磁盘上，那么当文件第一次被访问时，你就必须计算
    整棵树，即使只读取一个字节。这是 Merkle 树哈希工作方式的一个根本性后果。
    要验证一个叶节点，你需要验证到根哈希的整条路径，包括根节点（根哈希就是
    对其的哈希）。但是如果根节点不存储在磁盘上，你就必须通过哈希它的子节点
    来计算它，依此类推，直到你真正哈希了整个文件。

    这违背了进行基于 Merkle 树哈希的大部分意义，因为如果你无论如何都必须提前
    哈希整个文件，那么你干脆做 sha256(file) 就行了。那会简单得多，而且也会
    稍微快一点。

    确实，内存中的 Merkle 树仍然可以提供在每次读取时（而不仅仅第一次读取时）
    验证的好处。但是，它会很低效，因为每当一个哈希页被逐出（你无法将整个
    Merkle 树固定到内存中，因为它可能非常大），为了恢复它你又需要哈希它下面
    树中的所有内容。这再次违背了进行基于 Merkle 树哈希的大部分意义，因为单次
    块读取可能会触发对 GB 级数据的重新哈希。

:Q: 但是，难道不能只存储叶节点，而计算其余部分吗？
:A: 参见上一个回答；这其实只是上移了一层，因为也可以将数据块解释为 Merkle
    树的叶节点。确实，如果存储了叶层而非仅仅数据，树可以计算得更快得多，但这
    仅仅是因为每一层都不到下面一层大小的 1%（假设推荐设置 SHA-256 和 4K 块）。
    出于完全相同的原因，通过存储“仅叶节点”，你已经存储了树的 99% 以上，所以
    你不如直接存储整棵树。

:Q: Merkle 树可以提前构建吗，例如作为分发给许多计算机的软件包的一部分？
:A: 目前不支持。它是原始设计的一部分，但为了简化内核 UAPI 而被移除，而且它
    并非关键用例。文件通常安装一次并使用多次，并且加密哈希在大多数现代处理器
    上还算快。

:Q: 为什么 fs-verity 不支持写入？
:A: 写入支持会非常困难，并且需要完全不同的设计，因此它远远超出了 fs-verity
    的范围。写入支持需要：

    - 一种在数据与哈希（包括所有层级的哈希）之间保持一致性的方法，因为在崩溃
      后（尤其是可能整个文件！）的损坏是不可接受的。解决这个问题的主要选项是
      数据日志（journalling）、写时复制（copy-on-write）和日志结构卷（log-structured
      volume）。但是，给现有文件系统加装新的机制非常困难。数据日志在 ext4 上
      可用，但非常慢。

    - 在每次写入后重建 Merkle 树，这会极其低效。或者，可以使用不同的认证字典
      结构，例如“认证跳跃表（authenticated skiplist）”。但是，这会复杂得多。

    将它对比 dm-verity 与 dm-integrity。dm-verity 非常简单：内核只是根据一个
    只读的 Merkle 树验证只读数据。相比之下，dm-integrity 支持写入但很慢，复杂
    得多，并且实际上并不支持全设备认证，因为它独立认证每个扇区，即没有“根哈希”。
    让同一个 device-mapper 目标支持这两种非常不同的情况并不合理；这同样适用于
    fs-verity。

:Q: 既然 verity 文件是不可变的，为什么不设置 immutable 位？
:A: 现有的 “immutable” 位（FS_IMMUTABLE_FL）已经具有一组特定的语义，它不仅
    使文件内容只读，还防止文件被删除、重命名、建立硬链接，以及更改其所有者或
    模式。fs-verity 不需要这些额外的属性，因此复用 immutable 位并不合适。

:Q: 为什么 API 使用 ioctl 而不是 setxattr() 和 getxattr()？
:A: 将 xattr 接口滥用于基本任意的系统调用，被大多数 Linux 文件系统开发者所
    强烈反对。一个 xattr 确实应该只是磁盘上的一个 xattr，而不是一个例如神奇地
    触发构建 Merkle 树的 API。

:Q: fs-verity 支持远程文件系统吗？
:A: 迄今为止，所有实现了 fs-verity 支持的文件系统都是本地文件系统，但原则上
    任何能够存储每文件 verity 元数据的文件系统都可以支持 fs-verity，无论它是
    本地的还是远程的。一些文件系统存储 verity 元数据的位置选择较少；一种可能
    是将它存储在文件末尾之外，并通过操纵 i_size 将其对用户空间“隐藏”。由
    `fs/verity/` 提供的数据验证函数也假定文件系统使用 Linux 页缓存，但本地和
    远程文件系统通常都这样做。

:Q: 为什么会有任何文件系统特定的东西？fs-verity 不应该完全在 VFS 层实现吗？
:A: 有许多原因说明这是不可能的或会非常困难，包括以下几点：

    - 为了防止绕过验证，在 folio 被验证之前不得将其标记为 Uptodate。目前，每个
      文件系统都负责通过 `->readahead()` 将 folio 标记为 Uptodate。因此，目前
      VFS 不可能独自进行验证。改变这一点需要对 VFS 和所有文件系统进行重大更改。

    - 它需要定义一种与文件系统无关的方式来存储 verity 元数据。扩展属性不适合
      此用途，因为 (a) Merkle 树可能有 GB 级大小，但许多文件系统假定所有 xattr
      都适合单个 4K 文件系统块，且 (b) ext4 和 f2fs 加密不加密 xattr，而当文件
      内容被加密时，Merkle 树**必须**被加密，因为它存储明文文件内容的哈希。

      因此，verity 元数据必须存储在一个实际的文件中。使用单独的文件会非常难看，
      因为元数据从根本上说是受保护文件的一部分，并且会导致用户可能删除真实文件
      而非元数据文件，或反之的问题。另一方面，将它放在同一文件中会破坏应用程序，
      除非文件系统的 i_size 概念与 VFS 的概念分离，这很复杂且需要更改所有文件
      系统。

    - 期望 FS_IOC_ENABLE_VERITY 使用文件系统的事务机制，使得要么文件最终启用了
      verity，要么没有做任何更改。允许在崩溃后出现中间状态可能会导致问题。
