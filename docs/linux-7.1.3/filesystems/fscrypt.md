## 文件系统级加密（fscrypt


## 简


fscrypt 是一个库，文件系统可以接入它以支
文件和目录的透明加密

注意：本文档中的“fscrypt”指的是内核级部分，
fs/crypto/ 实现，而非用户空间工具
fscrypt <https://github.com/google/fscrypt>_。本文档
涵盖内核级部分。关于如何使用加密的命令行示例，
参阅用户空间工具 fscrypt <https://github.com/google/fscrypt>_ 的文档。此外，建议优先使用
fscrypt 用户空间工具，或其它已有的用户空间工具，例如
fscryptctl <https://github.com/google/fscryptctl>_ Android's key management system <https://source.android.com/security/encryption/file-based>_，
不要直接使用内核 API。使用现有工具可以降
引入自身安全漏洞的风险。（不过，出
完整性考虑，本文档仍然涵盖了内核的 API。）

dm-crypt 不同，fscrypt 运行在文件系统层级，而非
块设备层级。这使得它可以对不同的文
使用不同的密钥进行加密，并且允许在同一
文件系统上保留未加密的文件。这对于多用户系统很有用，因为每个用户的
静态数据需要与其它用户进行加密隔离
然而，除文件名外，fscrypt 不加密文件系统的
元数据

与作为堆叠文件系统的 eCryptfs 不同，fscrypt 被直接集
到受支持的文件系统中——目前包ext4、F2FS、UBIFS
CephFS。这使得加密文件可以被读取和写入
而无需
页缓存中同时缓存解密页和加密页，从而将近一半地减少所用内存，并使
与未加密文件一致。同样，所需dentries 
inodes 也减少一半。eCryptfs 还将加密文件名限制为 143
字节，导致应用程序兼容性问题；fscrypt 允许
完整255 字节（NAME_MAX）。最后，eCryptfs 不同，fscrypt API
可供非特权用户使用，无需挂载任何东西

fscrypt 不支持就地加密文件。相反，
支持将一个空目录标记为已加密。然后，
用户空间提供密钥后，在该目录树中创建的所有普通文件、目录和
符号链接都会被透明
加密

## 威胁模型


### 离线攻击


只要用户空间选择了强加密密钥，fscrypt
就能在块设备内容发生单点、永久性离线泄露的
情况下，保护文件内容和文件名
机密性。fscrypt 不保
非文件名元数据的机密性，例如文件大小、文件权限、文
时间戳和扩展属性。此外，文件
空洞（逻辑上全为零的未分配块）的存在与位置
也不受保护

如果攻击者能够在授权用户之后访问文件系统之前
离线操纵文件系统，则 fscrypt 无法保证
保护机密性或真实性

### 在线攻击


fscrypt（以及存储加密总体而言）只能提供有限的
针对在线攻击的保护。具体而言

#### 渚т俊閬撴敾鍑。


fscrypt 仅在底层 Linux 加密 API 算法或内联加密硬
所具备的程度上，能够抵抗侧信道攻击，例如时序或
电磁攻击。如果使用了易受攻击的算法，
例如基于查表
AES 实现，攻击者就可能对该在线系统发起侧信道攻击
侧信道攻击也可能针对
消费解密数据的应用程序发起

#### 未授权文件访


在添加加密密钥后，fscrypt 不会
同一系统上的其他用户隐藏
明文文件内容或文件名。相反，应使用现有的访问控制机制，例如文件模式位
POSIX ACL、LSM 或命名空间来实现此目的

（要理解其背后的原因，需认识到在密钥
被添加期间，从系统自身角度看，数据的机密
**并非**由加密的数学特性保护，而仅仅由
内核的正确性保护
因此，任何加密特有的访问控制检查都只是
由内*代码**强制执行，从而与
已有的各种访问控制机制大体上冗余。）

#### 只读内核内存泄露


除非使用hardware-wrapped keys_，否则获
读取任意内核内存能力的攻击者（例如通过发动物理攻击
或利用内核安全漏洞），可
泄露当前所有在用的 fscrypt 密钥。这
延伸到冷启动攻击；如果系统突然断电，
系统正在使用的密钥可能会在内存中保留一小段时间

然而，如果使用了硬件封装密钥（hardware-wrapped keys），fscrypt 的主
密钥和文件内容加密密钥（但不包括其它类型fscrypt
子密钥，如文件名加密密钥）会受到
任意内核内存泄露的保护

此外，fscrypt 允许从内核中
移除加密密钥，这可能使它们在日后免受泄露

更详细地说，FS_IOC_REMOVE_ENCRYPTION_KEY ioctl（或
FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS ioctl）可以从内核内存中清除主
加密密钥。如果它这样做了，还会尝
驱逐所有曾用该密钥“解锁”的缓存 inode
从而清除它们的每文件密钥，并使它们再次
呈现“锁定”状态，即以密文或加密形式出现

然而，这些 ioctl 有一些限制：

- 在用文件的每文件密钥**不会**被移除或清除
  因此，为获得最大效果，用户空间应在移除主密钥前
  关闭相关的加密文件和目录，并
  杀死任何工作目录位于受影响
  加密目录中的进程

- 内核无法神奇地清除用户空间可能持有的
  的主密钥副本。因此，用户空间也必须清除它制作的所
  主密钥副本；通常这应
  FS_IOC_ADD_ENCRYPTION_KEY 之后立即进行，而无需等待
  FS_IOC_REMOVE_ENCRYPTION_KEY。自然地，这同样适用
  密钥层级中的所有更高层。用户空间还
  遵循其它安全预防措施，例如用 mlock() 锁定
  含有密钥的内存，防止其被换出

- 一般而言，内VFS 缓存中的
  解密内容和文件名会被释放但不会被清除。因此，其中的部分内容可
  从已释放的内存中恢复，即使在相应的密
  被清除之后。为部分解决此问题，可以
  内核命令行中添加 init_on_free=1。但这会带来性能开销

- 密钥仍可能存在于 CPU 寄存器或其它
  此处未明确考虑的地方

#### 完全系统沦陷


获得“root”访问权限和/或执
任意内核代码能力的攻击者，可以自由地窃取受任何
在用 fscrypt 密钥保护的数据。因此，通常 fscrypt 在此场景
无法提供有意义的保护。（受在整个攻击过程
始终缺席的密钥保护的数据仍然受保护，但需考虑
上文提到的密钥移除的局限性——即在密
于攻击之前被移除的情况下。）

然而，如果使用hardware-wrapped keys_，此类攻击者将
无法以在系统断电后仍有用的形
窃取主密钥或文件内容密钥。如果攻击
在时间上或带宽上受到显著限制
因而只能窃取部分数据并需要依
后续的离线攻击来窃取其余数据，这可能会有用

#### v1 策略的局限


v1 加密策略在在线攻击方面存在一些弱点：
攻击

- 不会验证所提供master key 是否正确
  因此，恶意用户可以将其它用户拥有只读访问权限
  加密文件临时关联到错误的密钥
  由于文件系统缓存，错误的密钥随后会被
  另一用户对这些文件的访问所使用，即使另一用户
  在自己的密钥环中持有正确的密钥。这违背
  “只读访问”的含义

- 单个每文件密钥的泄露也会泄露推导出它
  主密钥

- root 用户无法安全地移除加密密钥

上述问题都在 v2 加密策略中得到修复。因此，
除其它原因外，建议在所有的
新加密目录上使用 v2 加密策略

## 密钥层级


注意：本节假设使用的是原始密钥（raw keys），而非
硬件封装密钥（hardware-wrapped keys）。使用硬件封装密钥会略微
改变密钥层级。详情请Hardware-wrapped keys_

### 主密


每个加密目录树都由一**master key**（主密钥）保护。主
密钥最长可64 字节，且长度必须至少达到
内容和文件名加密模式
两者中较大的安全强度。例如，如果使用了任一 AES-256 模式
主密钥必须至256 位，32 字节。如
密钥v1 加密策略使用且采AES-256-XTS
则适用更严格的要求：此类密钥必须为 64 字节

要“解锁”一个加密目录树，用户空间必须提
相应的主密钥。可以存在任意数量的主密钥，每一
都可以保护任意数量的文件系统上的
任意数量的目录树

主密钥必须是真正的加密密钥，即与
相同长度的随机字节串无法区分。这意味着用户
**不得**直接将口令用作主密钥、对较短的密钥进行零填充
或重复较短的密钥。如果用户犯了此类错误，
则无法保证安全性，因为相关的加密证明与分析
将不再适用

相反，用户应通过以下方式之一生成主密钥：使用
密码学安全的随机数生成器，或使用 KDF
（密钥派生函数）。内核不进行任何密钥拉伸
因此，如果用户空间从一个低熵密钥（例如口令）派生密钥，
则必须使用为此目的设计的 KDF
例如 scrypt、PBKDF2 Argon2

### 密钥派生函数


除一个例外，fscrypt 从不将主密钥直接用于
加密。相反，它们仅作KDF
（密钥派生函数）的输入，以派生出实际的密钥

特定主密钥所使用KDF 取决于该密钥是用
v1 加密策略还是 v2 加密
策略。用*不得**v1 v2
加密策略使用同一把密钥。（目前尚未知悉针对这种
密钥重用具体情形的现实攻击，但其安全性无法保证，
因为相关的加密证明与分析将不再适用。）

对于 v1 加密策略，KDF 仅支持派生每文件
加密密钥。其工作方式是用
AES-128-ECB 加密主密钥，使用文件16 字节随机数作AES 密钥
所得的密文被用作派生密钥。如果密
长于所需长度，则会被截断至所需长度

对于 v2 加密策略，KDF HKDF-SHA512。主密钥
作为“input keying material”传入，不使用盐值，并且对每一个要派生
不同密钥使用一个不同的
“application-specific information string”。例如，当派生每文件加密密钥时，
application-specific information string 是文件的随机数，
前面加上“fscrypt\\0”和一个上下文字节。不同的
上下文字节用于其它类型的派生密钥

HKDF-SHA512 优于原先基于 AES-128-ECB KDF，因
HKDF 更灵活、不可逆，并且均匀
分配来自主密钥的熵。HKDF 也是标准化的，并
其它软件广泛采用，而基AES-128-ECB KDF 是临时设计的

### 每文件加密密


由于每把主密钥可以保护许多文件，有必
对每文件的加密进行“tweak”，使得两个文件中的
相同明文不会映射到相同密文，反之亦然。在大多
情况下，fscrypt 通过派生每文件密钥来实现。当创建新的
加密 inode（普通文件、目录或符号链接）时
fscrypt 随机生成一16 字节随机数并将其存储
inode 的加xattr 中。然后，它使KDF（如 Key derivation function_ 所述）从主密钥
和随机数派生出文件的密钥

之所以选择密钥派生而非密钥包装（key wrapping），是因为包装密钥需
更大xattr，从而不太可能内联放
文件系统inode 表中，而且似乎
没有密钥包装的显著优势。特别是，目
不需要支持用多把备
主密钥解锁文件，也不需要支持轮换主密钥。相反，
主密钥可以在用户空间中被包装，例
fscrypt <https://github.com/google/fscrypt>_ 工具所做的那样

### DIRECT_KEY 策略


Adiantum 加密模式（见 Encryption modes and usage_
适用于内容和文件名加密，并且它接
IV——足以容纳一8 字节的数据单元索引和一
16 字节的每文件随机数。此外，每个 Adiantum 密钥的开销
大于 AES-256-XTS 密钥

因此，为提升性能并节省内存，对于 Adiantum 支持一
“direct key”配置。当用户通过
fscrypt 策略中设FSCRYPT_POLICY_FLAG_DIRECT_KEY 来启用它时，
不使用每文件加密密钥。相反，每当任何数据
（内容或文件名）被加密时，文件的 16 字节随机数会
包含IV 中。此外：

- 对于 v1 加密策略，加密直接使
  主密钥完成。因此，用户**不得**将该主密钥用
  任何其它目的，即使是其它 v1 策略

- 对于 v2 加密策略，加密使用通过 KDF 派生
  每模式密钥完成。用户可以将同一把主密钥用于
  其它 v2 加密策略

### IV_INO_LBLK_64 策略


当在 fscrypt 策略中设置了 FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64 时，
加密密钥由主密钥、加密模式编号和
文件系统 UUID 派生而来。这通常导致所有由同一
主密钥保护的文件共享一把内容加
密钥和一把文件名加密密钥。为了仍然对不同
文件的数据进行不同加密，inode 号被包含IV 中
因此，可能不允许收缩文件系统

这种格式针对符合 UFS 标准
内联加密硬件进行了优化，该标准每I/O 请求仅支64 IV
并且可能只有少量密钥槽

### IV_INO_LBLK_32 策略


IV_INO_LBLK_32 策略的工作方式与 IV_INO_LBLK_64 类似，只不过对于
IV_INO_LBLK_32，inode 号使SipHash-2-4 进行哈希（其
SipHash 密钥从主密钥派生），并加到文件数据单元索
mod 2^32 上，从而产生一32 IV

这种格式针对符合 eMMC v5.2 标准
内联加密硬件进行了优化，该标准每I/O 请求仅支32 IV
并且可能只有少量密钥槽。这
格式会导致一定程度的 IV 重用，因此只有当
因硬件限制而必须使用时才应采用

### 瀵嗛挜鏍囪瘑绗。


对于用于 v2 加密策略的主密钥，还会使KDF 派生一个唯一16 字节“key
identifier”（密钥标识符）。该值以明文存储
因为需要它来可靠地标识密钥本身

### 目录哈希密钥


对于使用基于密钥dirhash（在明文文件名之上）
建立索引的目录，KDF 也用于派生一128 位的
SipHash-2-4 密钥，以便对每个目录的文件名进行哈希。其工作
方式与派生每文件加密密钥相同，只是使用的
KDF 上下文不同。目前，只有大小写折叠（"case-insensitive"）的
加密目录使用这种哈希方式

## 加密模式与用


fscrypt 允许为文件内容指定一种加密模式，
并为文件名指定一种加密模式。不同的
目录树允许使用不同的加密模式

### 支持的模


目前，支持以下加密模式配对：

- AES-256-XTS 用于内容，AES-256-CBC-CTS 用于文件
- AES-256-XTS 用于内容，AES-256-HCTR2 用于文件
- Adiantum 用于内容和文件名
- AES-128-CBC-ESSIV 用于内容，AES-128-CBC-CTS 用于文件
- SM4-XTS 用于内容，SM4-CBC-CTS 用于文件

注意：在 API 中，“CBC”指 CBC-ESSIV，“CTS”指 CBC-CTS
例如，FSCRYPT_MODE_AES_256_CTS 表示 AES-256-CBC-CTS

目前不支持带认证的加密模式，因为
处理密文扩展较为困难。因此，
内容加密使用 XTS mode <https://en.wikipedia.org/wiki/Disk_encryption_theory#XTS>_ 中的分组密码，或
CBC-ESSIV mode <https://en.wikipedia.org/wiki/Disk_encryption_theory#Encrypted_salt-sector_initialization_vector_(ESSIV)>_,
使用宽分组密码。文件名加密使用
CBC-CTS mode <https://en.wikipedia.org/wiki/Ciphertext_stealing>_ 中的分组密码，或宽分
密码

（AES-256-XTS，AES-256-CBC-CTS）配对是推荐的默认选择
它也是唯一**保证**在只要内核支fscrypt 时就
始终受支持的选项；见 Kernel config options_

（AES-256-XTS，AES-256-HCTR2）配对也是一个不错的选择，它
将文件名加密升级为使用宽分组密码。（宽分
密码**，也称为可调超伪随机
置换，具有改变一个比特就会打
整个结果的特性。）Filenames encryption_ 所述，宽分
密码是该问题领域的理想模式，尽管 CBC-CTS 
备选方案中“最不差”的选择。关
HCTR2 的更多信息，the HCTR2 paper <https://eprint.iacr.org/2021/1441.pdf>_

在因缺乏 AES 硬件加速而导AES 过慢的系统上，推荐使Adiantum
Adiantum 是一种宽分组密码
使用 XChaCha12 AES-256 作为其底层组件。其大部
工作XChaCha12 完成，当 AES 加速不可用时，它比 AES 快得多
关于 Adiantum 的更多信息，请见
the Adiantum paper <https://eprint.iacr.org/2018/720.pdf>_.

（AES-128-CBC-ESSIV，AES-128-CBC-CTS）配对被加入，是为了尝试
为缺AES 指令的系统提供一种更高效的选项
CPU 中，但确实有非内联的加密引擎，例CAAM CESA
支持 AES-CBC（而非 AES-XTS）。这已废弃。它
已被证明仅在 CPU 上执AES 实际上更快
此外，Adiantum 更快，并且推荐在此类系统上使用

其余的模式配对是“national pride ciphers”（民族自豪密码）：

- (SM4-XTS, SM4-CBC-CTS)

一般来说，这些密码本身并不“差”，但它
AES ChaCha 等通常的选择相比，受到的
安全审查有限。它们也没有带来太多新东西。建
仅在使用被强制要求时才使用这些密码

### 内核配置选项


启用 fscrypt 支持（CONFIG_FS_ENCRYPTION）会自动引入
使用 AES-256-XTS 
AES-256-CBC-CTS 加密所需crypto API 基本支持。为获得最佳性能
强烈建议同时启用任何可用的、为你要使用
算法提供加速的平台特定
kconfig 选项。对任何“非默认”加密模式的支持通常
也需要额外的 kconfig 选项

下面按加密模式列出一些相关选项。注意，
你的平台可能还有未列出的加速选项
请参kconfig 菜单。文件内容加密可
配置为使用内联加密硬件，而非
内核 crypto API（见 Inline encryption support_）；在这种情况下
文件内容模式无需在内crypto API 中受支持
但文件名模式仍然需要

- AES-256-XTS 鍜?AES-256-CBC-CTS
    - 推荐
        - arm64：CONFIG_CRYPTO_AES_ARM64_CE_BLK
        - x86：CONFIG_CRYPTO_AES_NI_INTEL

- AES-256-HCTR2
    - 必须
        - CONFIG_CRYPTO_HCTR2
    - 推荐
        - arm64：CONFIG_CRYPTO_AES_ARM64_CE_BLK
        - x86：CONFIG_CRYPTO_AES_NI_INTEL

- Adiantum
    - 必须
        - CONFIG_CRYPTO_ADIANTUM

- AES-128-CBC-ESSIV 鍜?AES-128-CBC-CTS锛。
    - 必须
        - CONFIG_CRYPTO_ESSIV
        - CONFIG_CRYPTO_SHA256 或另一SHA-256 实现
    - 推荐
        - AES-CBC 加

### 内容加密


对于内容加密，每个文件的内容被划分为“data
units”（数据单元）。每个数据单元被独立加密。每
数据单元IV 包含了该数据单元在文件中
从零开始的索引。这确保每个文件中的每个数据单元都以
不同的方式加密，这对于防止信息泄露至关重要

注意：加密依赖于文件内的偏移量，这意味着
诸如“collapse range”和“insert range”这类重新排
文件 extent 映射的操作在加密文件上不受支持

数据单元的大小有两种情况

- 固定大小的数据单元。除 UBIFS 之外的所有文件系统都
  采用这种方式。一个文件的所有数据单元大小相同；最后一个数据单
  在需要时会进行零填充。默认情况下，数据单元大小等
  文件系统块大小。在某些文件系统上，用户可以通过
  加密策略log2_data_unit_size 字段选择
  亚块（sub-block）数据单元大小；FS_IOC_SET_ENCRYPTION_POLICY_

- 可变大小的数据单元。这UBIFS 的做法。每个“UBIFS
  data node”被视为一个加密数据单元。每个包含可
  长度、可能经过压缩的数据，零填充到下一16 字节
  边界。用户在 UBIFS 上无法选择亚块数据单元大小

在压加密的情况下，被加密的是
压缩后的数据。UBIFS 压缩如上所述。f2fs
压缩的工作方式略有不同；它将若干
文件系统块压缩为更少数量的文件系统块
因此，f2fs 压缩文件仍然使用固定大小的数据单元，并且
其加密方式与包含空洞的文件类似

Key hierarchy_ 所述，默认加密设置使用
每文件密钥。在这种情况下，每个数据单元IV 仅仅
该数据单元在文件中的索引。然而，用户可以选择一
不使用每文件密钥的加密设置。对于这些设置，某些
类型的文件标识符按如下方式被纳入 IV

- 使用 DIRECT_KEY policies_ 时，数据单元索引被放IV 
  0-63 位，文件的随机数被放64-191 位

- 使用 IV_INO_LBLK_64 policies_ 时，数据单元索引被放
  IV 0-31 位，文件inode 号被放在
  32-63 位。此设置仅在数据单元索引
  inode 号都适合 32 位时才被允许

- 使用 IV_INO_LBLK_32 policies_ 时，文件inode 号被哈希
  并加到数据单元索引上。所得值被截断
  32 位，并放IV 0-31 位。此设置
  在数据单元索引和 inode 号都适合 32 位时才被允许

IV 的字节序始终为小端（little endian）

如果用户为内容模式选择FSCRYPT_MODE_AES_128_CBC，则会自
包含 ESSIV 层。在这种情况下，在将 IV
传给 AES-128-CBC 之前，会先用 AES-256 对其加密，其AES-256
密钥是文件内容加密密钥的 SHA-256 哈希值

### 文件名加


对于文件名，每个完整的文件名被一次性加密。由
需要保留对高效目录查找和最
255 字节文件名的支持，目录中每个文件名都使用
相同IV

然而，每个加密目录仍然使用唯一的密钥，或
（对DIRECT_KEY policies_）将文件的随机数，或
（对IV_INO_LBLK_64 policies_）将文件inode 号纳IV
因此，IV 重用被限制在单个目录内

对于 CBC-CTS，IV 重用意味着当明文文件名共享一
至少与密码块大小（AES 16 字节）一样长的前缀时，
对应的加密文件名也会共享一个相同的前缀。这
不理想的。Adiantum HCTR2 没有这个弱点，因为它们是
宽分组加密模式

所有支持的文件名加密模式都接受任何明文长度
>= 16 字节；不需要密码块对齐。然而，
短于 16 字节的文件名在加密前会被 NUL 填充16 字节
然后才被加密。此外，为减少文件名长度通过
密文泄露，所有文件名都会NUL 填充到下一4
16 32 字节边界（可配置）。建议使32，因为这
能提供最佳的机密性，代价是让目录
项占用略多的空间。注意，由于 NUL（\0
在文件名中不是其它有效字符，填充永远不会
产生重复的明文

符号链接目标被视为一种文件名，并
以与目录项中的文件名相同的方式加密，只是
IV 重用不是问题，因为每个符号链接都有自己的 inode

## 用户 API


### 设置加密策略


#### FS_IOC_SET_ENCRYPTION_POLICY


FS_IOC_SET_ENCRYPTION_POLICY ioctl 在一个空目录上设置加密策略，
或验证一个目录或普通文件已
具有指定的加密策略。它接受一个指
struct fscrypt_policy_v1 struct fscrypt_policy_v2 的指针，其定义如
```

    #define FSCRYPT_POLICY_V1               0
    #define FSCRYPT_KEY_DESCRIPTOR_SIZE     8
    struct fscrypt_policy_v1 {
            __u8 version;
            __u8 contents_encryption_mode;
            __u8 filenames_encryption_mode;
            __u8 flags;
            __u8 master_key_descriptor[FSCRYPT_KEY_DESCRIPTOR_SIZE];
    };
    #define fscrypt_policy  fscrypt_policy_v1

    #define FSCRYPT_POLICY_V2               2
    #define FSCRYPT_KEY_IDENTIFIER_SIZE     16
    struct fscrypt_policy_v2 {
            __u8 version;
            __u8 contents_encryption_mode;
            __u8 filenames_encryption_mode;
            __u8 flags;
            __u8 log2_data_unit_size;
            __u8 __reserved[3];
            __u8 master_key_identifier[FSCRYPT_KEY_IDENTIFIER_SIZE];
    };

```
该结构体必须按如下方式初始化

- 如果使用 struct fscrypt_policy_v1，则 version 必须FSCRYPT_POLICY_V1 (0)
  如果使用 struct fscrypt_policy_v2，则FSCRYPT_POLICY_V2 (2)
  （注意：我们将最初的
  策略版本称为“v1”，尽管其版本码实际上是 0。）
  对于新的加密目录，请使用 v2 策略

- contents_encryption_mode 鍜?filenames_encryption_mode 蹇呴』
  被设置为来自 <linux/fscrypt.h> 的常量，这些常量标识要使用的
  加密模式。如果不确定的话，对
  contents_encryption_mode 使用 FSCRYPT_MODE_AES_256_XTS (1)，对
  filenames_encryption_mode 使用 FSCRYPT_MODE_AES_256_CTS (4)。详情见 Encryption   modes and usage_

  v1 加密策略仅支持三种模式组合：
  (FSCRYPT_MODE_AES_256_XTS, FSCRYPT_MODE_AES_256_CTS),
  (FSCRYPT_MODE_AES_128_CBC, FSCRYPT_MODE_AES_128_CTS), 以及
  (FSCRYPT_MODE_ADIANTUM, FSCRYPT_MODE_ADIANTUM)。v2 策略支持
  Supported modes_ 中记录的所有组合

- flags 包含来自 <linux/fscrypt.h> 的可选标志：

  - FSCRYPT_POLICY_FLAGS_PAD_*：加密文件名时要使用NUL 填充量
    如果不确定的话，使用 FSCRYPT_POLICY_FLAGS_PAD_32
    (0x3)銆。
  - FSCRYPT_POLICY_FLAG_DIRECT_KEY：见 DIRECT_KEY policies_
  - FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64：见 IV_INO_LBLK_64     policies_
  - FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32：见 IV_INO_LBLK_32     policies_

  v1 加密策略仅支PAD_* DIRECT_KEY 标志
  其它标志仅由 v2 加密策略支持

  DIRECT_KEY、IV_INO_LBLK_64 IV_INO_LBLK_32 标志
  互斥的

- log2_data_unit_size 是以字节为单位的数据单元大小log2
  或设0 以选择默认数据单元大小。数据单元大小是
  文件内容加密的粒度。例如，
  log2_data_unit_size 设为 12 会使文件内容4096 字节
  的数据单元传递给底层加密算法（例AES-256-XTS），
  每个数据单元都有自己IV

  并非所有文件系统都支持设置 log2_data_unit_size。ext4
  f2fs Linux v6.7 起支持。在支持它的文件系统上，
  支持的非零取值为 9 到文件系统块大小
  log2（含端点）。默认0 选择
  文件系统块大小

  log2_data_unit_size 的主要用例是选择
  小于文件系统块大小的数据单元大小，以
  兼容只支
  更小数据单元大小的内联加密硬件sys/block/$disk/queue/crypto/ 可用
  检查某特定系统的内联加密硬件支持哪
  数据单元大小

  除非你确定需要它，否则将此字段置零。使
  不必要的较小数据单元大小会降低性能

- 对于 v2 加密策略，__reserved 必须置零

- 对于 v1 加密策略，master_key_descriptor 指定如何
  在密钥环中找到主密钥；见 Adding keys_。由
  master_key_descriptor。e4crypt fscrypt 工具使用
  主密钥。e4crypt fscrypt 工具使用其前 8 个字节的
  SHA-512(SHA-512(master_key)) 的前 8 个字节，但这种特定方案并
  必须。此外，在执FS_IOC_SET_ENCRYPTION_POLICY 时，
  主密钥无需已经在密钥环中。然而，它必须在
  加密目录中创建任何文件之前被添加

  对于 v2 加密策略，master_key_descriptor 已被
  替换为更长的 master_key_identifier，并且不
  被任意选择。相反，必须首先使用
  FS_IOC_ADD_ENCRYPTION_KEY_。然后，key_spec.u.identifier
  内核struct fscrypt_add_key_arg 中返回的
  struct fscrypt_policy_v2 中的 master_key_identifier
  struct fscrypt_policy_v2銆。

如果文件尚未加密，则 FS_IOC_SET_ENCRYPTION_POLICY
会验证该文件是否是一个空目录。如果是，则将指定的
加密策略分配给该目录，使其成
加密目录。此后，并且在提供了
Adding keys_ 所述提供相应的主密钥后，该目录中创建的所有普
文件、目录（递归地）和符号链接都将被加密
并继承相同的加密策略
该目录项中的文件名也会被加密

或者，如果文件已经加密，则
FS_IOC_SET_ENCRYPTION_POLICY 会验证指定的加密
策略与实际策略完全一致。如果一致，则该 ioctl
返回 0。否则，它会EEXIST 失败。这适用
普通文件和目录，包括非空目录

当将 v2 加密策略分配给一个目录时，还
要求指定的密钥已由当
用户添加，或者调用者在初始用户命名空间中具CAP_FOWNER
（这是为了防止用户用另一个用户的密钥加密其数据。）
执行 FS_IOC_SET_ENCRYPTION_POLICY 期间，密钥必须保持已添加状态
然而，如果新的
加密目录不需要立即被访问，那么之后可
立即移除密钥

注意，ext4 文件系统不允许对根目录进
加密，即使它是空的。想要用一把密钥加密整
文件系统的用户应考虑改用 dm-crypt

FS_IOC_SET_ENCRYPTION_POLICY 可能因以下错误而失败：

- EACCES：该文件不属于进程的 uid，进程也不在
  文件属主 uid 被映射的命名空间内具CAP_FOWNER 能力
  銆。
- EEXIST：该文件已用与指定策
  不同的加密策略加
- EINVAL：指定了无效的加密策略（无效
  版本、模式或标志；或设置了保留位）；或者指定的
  v1 加密策略，但目录启用casefold
  标志（大小写折叠v1 策略不兼容）
- ENOKEY：指定了 v2 加密策略，但具有指定
  master_key_identifier 的密钥尚未被添加，进程也
  在初始用户命名空间中具有 CAP_FOWNER 能力
  銆。
- ENOTDIR：该文件未加密且是普通文件，而非
  目录
- ENOTEMPTY：该文件未加密且是非空目
- ENOTTY：此类文件系统未实现加密
- EOPNOTSUPP：内核未配置对文件系统的加密
  支持，或者文件系统超级块尚未
  启用加密。（例如，要ext4 文件系统上使用加密，
  必须在内核配置中启用 CONFIG_FS_ENCRYPTION，并
  超级块必须通过
  tune2fs -O encrypt 鎴?mkfs.ext4 -O
  encrypt 启用“encrypt”特性标志。）
- EPERM：此目录可能不可加密，例如因为它
  ext4 文件系统的根目录
- EROFS：文件系统为只读

### 获取加密策略


有两ioctl 可用于获取文件的加密策略

- FS_IOC_GET_ENCRYPTION_POLICY_EX_
- FS_IOC_GET_ENCRYPTION_POLICY_

ioctl 的扩展（_EX）版本更通用，并
建议在可能时优先使用。然而，在较旧的内核上只
原始 ioctl 可用。应用程序应尝试扩展
版本，如果它ENOTTY 失败，则回退到原
版本

#### FS_IOC_GET_ENCRYPTION_POLICY_EX


FS_IOC_GET_ENCRYPTION_POLICY_EX ioctl 获取目录或普通文件的
加密策略（如果有）。除打开文件的能力外
不需要额外的权限。它
接受一个指struct fscrypt_get_policy_ex_arg 的指针，
```

    struct fscrypt_get_policy_ex_arg {
            __u64 policy_size; /* input/output */
            union {
                    __u8 version;
                    struct fscrypt_policy_v1 v1;
                    struct fscrypt_policy_v2 v2;
            } policy; /* output */
    };

```
调用者必须将 policy_size 初始化为可用
策略结构体的大小，即 sizeof(arg.policy)

成功时，策略结构体通过 policy 返回，其
实际大小policy_size 中返回。policy.version 应当
被检查以确定返回的策略版本。注意，
"v1" 策略的版本码实际上是 0（FSCRYPT_POLICY_V1）

FS_IOC_GET_ENCRYPTION_POLICY_EX 可能以下列错误失败：

- EINVAL：该文件已加密，但它使用了无法识别的
  加密策略版本
- ENODATA：该文件未加
- ENOTTY：此类文件系统未实现加密
  或此内核太旧，不支持 FS_IOC_GET_ENCRYPTION_POLICY_EX
  （请改为尝试 FS_IOC_GET_ENCRYPTION_POLICY
- EOPNOTSUPP：内核未配置
  此文件系统的加密支持，或者文件系统超级块尚未
  启用加密
- EOVERFLOW：该文件已加密，并且使用可识别的
  加密策略版本，但策略结构体无法放
  所提供的缓冲区

注意：如果你只需要知道一个文件是否加密，在大多数文件系统
也可以使FS_IOC_GETFLAGS ioctl
并检FS_ENCRYPT_FL，或者使statx() 系统调用
检stx_attributes 中的 STATX_ATTR_ENCRYPTED

#### FS_IOC_GET_ENCRYPTION_POLICY


FS_IOC_GET_ENCRYPTION_POLICY ioctl 也可以获
目录或普通文件的加密策略（如果有）。然而，
FS_IOC_GET_ENCRYPTION_POLICY_EX_ 不同
FS_IOC_GET_ENCRYPTION_POLICY 仅支持原始策
版本。它直接接受一个指struct fscrypt_policy_v1 的指针，
而非 struct fscrypt_get_policy_ex_arg

FS_IOC_GET_ENCRYPTION_POLICY 的错误码
FS_IOC_GET_ENCRYPTION_POLICY_EX 相同，只
如果文件使用更新的加密策略版本加密，FS_IOC_GET_ENCRYPTION_POLICY 还会返回 EINVAL
加密策略版本

### 获取每个文件系统的盐


某些文件系统，例ext4 F2FS，还支持已废弃的
ioctl FS_IOC_GET_ENCRYPTION_PWSALT。该 ioctl 获取一个随
生成的、存储在文件系统超级块中16 字节值。该
值旨在作为从口令或其它低熵用户凭
派生加密密钥时的盐值

FS_IOC_GET_ENCRYPTION_PWSALT 已废弃。相反，建议
用户空间生成并管理任何所需的盐值

### 获取文件的加密随机数


Linux v5.7 起，支持 ioctl FS_IOC_GET_ENCRYPTION_NONCE
在加密的文件和目录上，它获取 inode 16 字节随机数
在未加密的文件和目录上，它以 ENODATA 失败

ioctl 对验证加密是否被正确执行
自动化测试很有用。正常使fscrypt
时不需要它

### 添加密钥


#### FS_IOC_ADD_ENCRYPTION_KEY


FS_IOC_ADD_ENCRYPTION_KEY ioctl 将一个主加密密钥添加
到文件系统，使文件系统中所有使用该密钥加密
文件呈现“解锁”状态，即以明文形式出现
它可以在目标文件系统中的任何文件或目录上执行
但建议使用文件系统的根目录。它接受一
```

    struct fscrypt_add_key_arg {
            struct fscrypt_key_specifier key_spec;
            __u32 raw_size;
            __u32 key_id;
    #define FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED 0x00000001
            __u32 flags;
            __u32 __reserved[7];
            __u8 raw[];
    };

    #define FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR        1
    #define FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER        2

    struct fscrypt_key_specifier {
            __u32 type;     /* one of FSCRYPT_KEY_SPEC_TYPE_* */
            __u32 __reserved;
            union {
                    __u8 __reserved[32]; /* reserve some extra space */
                    __u8 descriptor[FSCRYPT_KEY_DESCRIPTOR_SIZE];
                    __u8 identifier[FSCRYPT_KEY_IDENTIFIER_SIZE];
            } u;
    };

    struct fscrypt_provisioning_key_payload {
            __u32 type;
            __u32 flags;
            __u8 raw[];
    };

```
struct fscrypt_add_key_arg，必须先将其置零，然
按如下方式初始化

- 如果要添加的密钥v1 加密策略使用，则
  key_spec.type 必须包含 FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR，并
  key_spec.u.descriptor 必须包含所要添加的密钥的描述符
  瀵瑰簲浜。
  struct fscrypt_policy_v1 master_key_descriptor 字段中的值
  要添加此类密钥，调用进程必须具有
  初始用户命名空间中的 CAP_SYS_ADMIN 能力

  或者，如果要添加的密钥v2 加密
  策略使用，则 key_spec.type 必须包含
  FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER，并key_spec.u.identifier 
  一*输出**字段，内核会用密钥的密码
  哈希值填充它。要添加此类密钥，调用进
  不需要任何特权。然而，可以添加的密钥数
  受用户对密钥环服务的配额限制（见
  Documentation/security/keys/core.rst).

- raw_size 必须是所提供raw 密钥的大小（以字节为单位）
  或者，如果 key_id 非零，则此字段必须为 0，因
  在这种情况下，大小由指定Linux 密钥环密钥隐含给出

- 如果密钥直接raw 字段中给出，key_id 0
  否则 key_id 是一个类型为
  “fscrypt-provisioning”的 Linux 密钥环密钥的 ID，其载荷是结构体
  fscrypt_provisioning_key_payload，其 raw 字段包含
  密钥，其 type 字段key_spec.type 匹配，并且其
  flags 字段flags 匹配。由raw 
  可变长度的，此密钥载荷的总大小必须为
  sizeof(struct fscrypt_provisioning_key_payload) 加上
  密钥字节数。进程必须对该密钥具Search 权限

  大多数用户应将此设为 0 并直接指定密钥。该
  支持指定 Linux 密钥环密钥主要是为了
  在文件系统卸载并重新挂载后重新添加密钥，
  而无需将密钥存储在用户空间内存中

- flags 包含来自 <linux/fscrypt.h> 的可选标志：

  - FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED：这表示该密钥是一
    硬件封装密钥。见 Hardware-wrapped keys_。该标志
    不能FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR 一起使用

- raw 是一个可变长度字段，必须包含实际
  密钥，长度为 raw_size 字节。或者，如果 key_id
  非零，则此字段未使用。注意，尽管名为
  raw，如果指定了 FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED，则
  将包含一个封装密钥，而非原始密钥

对于 v2 策略密钥，内核会记录是哪个用户（
有效用户 ID 标识）添加了该密钥，并且只允许该用户
移除它——或者由“root”移除，前提是他们使
FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS_.

然而，如果另一个用户已经添加了该密钥，可能希望
防止那个其他用户意外地移除它。因此，
FS_IOC_ADD_ENCRYPTION_KEY 也可用于**再次**添加一v2 策略密钥
即使它已被其他用户添加。在这种情况下，
FS_IOC_ADD_ENCRYPTION_KEY 只会为当前用户安装对该密钥的
一个声明，而不是真正再次添加该密钥（但必须
  仍然提供密钥，作为知情的证明）

如果密钥或对该密钥的声明被添加或已存在，
FS_IOC_ADD_ENCRYPTION_KEY 返回 0

FS_IOC_ADD_ENCRYPTION_KEY 可能因以下错误而失败：

- EACCES：指定了 FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR，但
  调用者不具有初始用户命名空间中的
  CAP_SYS_ADMIN 能力；或者密钥是通过 Linux 密钥 ID 指定的，
  进程对该密钥缺少 Search 权限
- EBADMSG：无效的硬件封装密钥
- EDQUOT：添加该密钥会超出该用户的密钥配
  
- EINVAL：无效的密钥大小或密钥说明符类型，或设置了保留位
  
- EKEYREJECTED：密钥是通过 Linux 密钥 ID 指定的，但该密钥
  类型错误
- ENOKEY：密钥是通过 Linux 密钥 ID 指定的，但不存在具有ID 的密
  
- ENOTTY：此类文件系统未实现加密
- EOPNOTSUPP：内核未配置
  支持该文件系统，或者该文件系统的超级块尚未
  启用加密；或者指定了硬件封装密钥，但
  文件系统不支持内联加密，或者硬
  不支持硬件封装密

#### 传统方法


对于 v1 加密策略，主加密密钥也可
通过将其添加到进程订阅的密钥环来提供，例如添加到
会话密钥环，或者在用户密钥环已链接
会话密钥环的情况下，添加到用户密钥环

此方法已废弃（并且不支持 v2 加密
策略），原因如下。首先，它不能用
  Removing keys_），因此要移除密钥，必须使用诸如
因此，要移除密钥，可采用诸如 keyctl_unlink() 之类的变通方法，
  sync; echo 2 > /proc/sys/vm/drop_caches 结合 keyctl_unlink() 之类的变通方法。其次，它与
必须使用。其次，它与以下事实不符
加密文件的锁解锁状态（即它们是否以
明文形式或密文形式出现）是全局性的。这种不匹配
在进
在不UID 下运行（例如 sudo 命令）的进程需
访问加密文件时，已造成了许多困惑以及实际问题

尽管如此，要向某个进程订阅的密钥环添加密钥，
可以使用 add_key() 系统调用（参见：
Documentation/security/keys/core.rst）。密钥类型必须为
“logon”；此类密钥保存在内核内存中，用户空间无
读回。密钥描述必须为“fscrypt:
，后跟加密策略中设置
master_key_descriptor 16 位小写十六进制表示。该
```

    #define FSCRYPT_MAX_KEY_SIZE            64

    struct fscrypt_key {
            __u32 mode;
            __u8 raw[FSCRYPT_MAX_KEY_SIZE];
            __u32 size;
    };

```
mode 被忽略；直接将其设为 0。实际的密钥
raw 中提供，size 指示其大小（字节）。即
raw[0..size-1]（含）字节就是实际的密钥

密钥描述前缀“fscrypt:”也可以替换
特定于文件系统的前缀（例如“ext4:”）。然而，
特定于文件系统的前缀已废弃，不应
新程序中使用

### 移除密钥


有两ioctl 可用于移除由以下方式添加的密钥：
FS_IOC_ADD_ENCRYPTION_KEY_:

- FS_IOC_REMOVE_ENCRYPTION_KEY_
- FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS_

这两ioctl 仅在root 用户添加或移v2 策略密钥
有所区别

这些 ioctl 对通过传统
进程订阅密钥环机制添加的密钥不起作用

在使用这ioctl 之前，请阅读 Online attacks_ 一节，以了
这些 ioctl 的安全目标与局限性

#### FS_IOC_REMOVE_ENCRYPTION_KEY


FS_IOC_REMOVE_ENCRYPTION_KEY ioctl 从文件系统中移除对主加密
密钥的声明，并可能移除该密钥
本身。它可以在目标文件系统中的任何文件或目录上执行，
但建议使用文件系统的根目录
它接受一个指struct fscrypt_remove_key_arg 的指针，该结构体定义
```

    struct fscrypt_remove_key_arg {
            struct fscrypt_key_specifier key_spec;
    #define FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY      0x00000001
    #define FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS     0x00000002
            __u32 removal_status_flags;     /* output */
            __u32 __reserved[5];
    };

```
该结构体必须先置零，然后按如下方式初始化

- 要移除的密钥key_spec 指定

    - 要移v1 加密策略使用的密钥，
      key_spec.type 设为 FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR 并填
      key_spec.u.descriptor。要移除此类密钥
      调用进程必须具有初始用户命名空间中的
      CAP_SYS_ADMIN 能力

    - 要移v2 加密策略使用的密钥，
      key_spec.type 设为 FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER 并填
      key_spec.u.identifier銆。

对于 v2 策略密钥，此 ioctl 可供root 用户使用。然而，
为了实现这一点，它实际上只是移除当前用户
对该密钥的声明，撤销一次对 FS_IOC_ADD_ENCRYPTION_KEY 的调用
只有在所有声明都被移除后，密钥才会真正被删除

例如，如FS_IOC_ADD_ENCRYPTION_KEY 是以 uid 1000 调用的，
那么该密钥将uid 1000“声明”，并且
FS_IOC_REMOVE_ENCRYPTION_KEY 仅能uid 1000 成功。或者，如果
uid 1000 2000 都添加了该密钥，那么对于每个 uid
FS_IOC_REMOVE_ENCRYPTION_KEY 将只移除它们各自的声明。只
*两*都被移除后，密钥才会真正被删除。（可以将其理解
取消链接一个可能具有硬链接的文件。）

如果 FS_IOC_REMOVE_ENCRYPTION_KEY 真正移除了密钥，它还
尝试“锁定”所有曾用该密钥解锁的文件。它不会
锁定仍在使用中的文件，因此此 ioctl 预期会与
用户空间配合使用的，以确保没有文件仍处于
打开状态。不过，如有必要，可以再次执行此 ioctl
以重试锁定任何剩余的文件

如果密钥被移除
（但可能仍有文件等待锁定），或者用户的密钥声明
移除，或者密钥已被移除但仍有文件
等待锁定，因ioctl 会重试锁定它们，则在任何
任何这些情况下，removal_status_flags 都会被填
  以下信息性状态标志：

- FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY：如果某些文
  仍在使用中，则设置。在仅移除了用户的声明的情况下，
  不保证会设置
- FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS：如果仅
  移除了用户的声明而非密钥本身，则设置

FS_IOC_REMOVE_ENCRYPTION_KEY 可能因以下错误而失败：

- EACCES：指定了 FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR 密钥说明符类型，
  但调用者不具有初始用户命名空间中的
  CAP_SYS_ADMIN 能力
- EINVAL：无效的密钥说明符类型，或设置了保留
- ENOKEY：完全找不到密钥对象，即它从
  被添加过，或者已经被完全移除（包括所
  文件已锁定）；或者，用户对该密钥没有声明（但
  其他人可能有）
- ENOTTY：此类文件系统未实现加密
- EOPNOTSUPP：内核未配置
  此文件系统的加密支持，或者文件系统超级块尚未
  启用加密

#### FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS


FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS 涓。
FS_IOC_REMOVE_ENCRYPTION_KEY_ 完全相同，只不过对于 v2 策略密钥
ioctl ALL_USERS 版本会移除所有用户对该密钥的
声明，而不仅仅是当前用户的。即，无论有多少用户添加了该密钥
密钥本身总是会被移除。这一差异
仅在使用root 用户添加和移除密钥时才有意义

因此，FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS 也要
“root”，即初始用户命名空间中CAP_SYS_ADMIN 能力
否则它会EACCES 失败

### 获取密钥状


#### FS_IOC_GET_ENCRYPTION_KEY_STATUS


FS_IOC_GET_ENCRYPTION_KEY_STATUS ioctl 获取主加
密钥的状态。它可以在目标文件系统中的任何文件或目录上执行，
但建议使用文件系统的根目录
它接受一个指
```

    struct fscrypt_get_key_status_arg {
            /* input */
            struct fscrypt_key_specifier key_spec;
            __u32 __reserved[6];

            /* output */
    #define FSCRYPT_KEY_STATUS_ABSENT               1
    #define FSCRYPT_KEY_STATUS_PRESENT              2
    #define FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED 3
            __u32 status;
    #define FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF   0x00000001
            __u32 status_flags;
            __u32 user_count;
            __u32 __out_reserved[13];
    };

```
的指针。调用者必须将所有制零输入字段，然后填写 key_spec

    - 要获v1 加密策略密钥的状态，
      key_spec.type 设为 FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR 并填
      key_spec.u.descriptor銆。

    - 要获v2 加密策略密钥的状态，
      key_spec.type 设为 FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER 并填
      key_spec.u.identifier銆。

成功时返0，内核会填入输出字段

- status 指示密钥是缺失、存在，还是
  被不完全移除。不完全移除意味着移除
  启动，但某些文件仍在使用中；
  FS_IOC_REMOVE_ENCRYPTION_KEY_ 返回0，但设置了信息
  状态标FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY

- status_flags 可包含以下标志：

    - FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF 指示该密
      已被当前用户添加。这仅为通过
      identifier 标识的密钥设置，而非通过 descriptor 标识的

- user_count 指定已添加该密钥的用户数量
  这仅为通过 identifier 标识的密钥设置，而非
  通过 descriptor 标识的

FS_IOC_GET_ENCRYPTION_KEY_STATUS 可能因以下错误而失败：

- EINVAL：无效的密钥说明符类型，或设置了保留
- ENOTTY：此类文件系统未实现加密
- EOPNOTSUPP：内核未配置
  此文件系统的加密支持，或者文件系统超级块尚未
  启用加密

除其它用例外，FS_IOC_GET_ENCRYPTION_KEY_STATUS 可用
确定给定加密目录的密钥是
需要在提示用户输入用于派生密钥
口令之前被添加

FS_IOC_GET_ENCRYPTION_KEY_STATUS 只能获取
文件系统级密钥环中密钥的状态，即由
FS_IOC_ADD_ENCRYPTION_KEY_ FS_IOC_REMOVE_ENCRYPTION_KEY_。它
无法获取仅被添加以供 v1
加密策略使用的、涉
进程订阅密钥环的传统机制

## 访问语义


### 鎸佹湁瀵嗛挜鏃。


持有加密密钥时，加密的常规文件、目录和
符号链接的行为与它们未加密的对应物非常相---
毕竟，加密本意是透明的。然而，
敏锐的用户可能会注意到行为上的一些差异：

- 未加密的文件，或者采用不同加
  策略（即不同的密钥、模式或标志）加密的文件，无法被重命名或
  链接到加密目录中；见 Encryption policy   enforcement_。此类尝试会EXDEV 失败。然而，
  加密文件可以在加密目录内重命名，
  移入未加密目录

  注意：将未加密文件“移动”到加密目录中（例如
  使用 mv 程序）是用户空间通过复制
  再删除来实现的。要注意原始的未加密数据
  可能仍可从磁盘上的空闲空间恢复；最好从一开始就
  将所有文件加密。shred 程序
  可用于覆盖源文件，但不能保证
  对所有文件系统和存储设备都有效

- 加密文件仅在部分条件下支Direct I/O
  情况下支持加密文件。详情见 Direct I/O support_

- fallocate 鎿嶄綔 FALLOC_FL_COLLAPSE_RANGE 鍜。
  FALLOC_FL_INSERT_RANGE 在加密文件上不受支持，并会以
  EOPNOTSUPP 失败

- 加密文件的在线碎片整理不受支持
  EXT4_IOC_MOVE_EXT 鍜?F2FS_IOC_MOVE_RANGE ioctl 浼氫互
  EOPNOTSUPP 失败

- ext4 文件系统不支持对加密
  普通文件进行数据日志（data journaling）。它会退回到 ordered data 模式

- DAX（Direct Access）在加密文件上不受支持

- 加密符号链接的最大长度比
  未加密符号链接的最大长度短 2 个字节。例如，
  块大小为 4K EXT4 文件系统上，未加密符号链接最长可
  4095 字节，而加密符号链接最长只能达4093
  字节（两个长度都不包括结尾的 null）

注意，mmap ***受支持的。这是因为加密文件的页缓
包含的是明文，而非密文

### 娌℃湁瀵嗛挜鏃。


某些文件系统操作可以在加密的普
文件、目录和符号链接上执行，即使在它们的加密密钥
尚未被添加，或已被移除之后：

- 可以读取文件元数据，例如使用 stat()

- 可以列出目录，此时文件名会以
  从其密文派生的编码形式列出。当前的
  编码算法Filename hashing and   encoding_ 中描述。该算法可能会变化，但保
  所呈现的文件名长度不会超过
  NAME_MAX 字节，不会包/ \0 字符，并
  能唯一标识目录项

  . .. 目录项是特殊的。它们始
  存在，并且不被加密或编码

- 可以删除文件。即，普通文件可以像往常一样用
  unlink() 删除，空目录可以像往常一样用
  rmdir() 删除。因此，rm rm -r 会按
  预期

- 符号链接目标可以被读取和跟随，但它们将以
  加密形式呈现，类似于目录中的文件名。因此，它们
  不太可能指向任何有用的位置

在没有密钥的情况下，常规文件无法被打开或截断
尝试这样做会失败并返ENOKEY。这意味着任何
需要文件描述符的常规文件操作，例如
read()、write()、mmap()、fallocate() ioctl()，也被禁止

同样在没有密钥的情况下，任何类型的文件（包括目录）都无法
被创建或链接到加密目录中，加密目录中的名称也不能
作为重命名的源或目标，也不能
加密目录中创O_TMPFILE 临时文件。所
此类操作都会失败并返ENOKEY

目前无法
没有加密密钥的情况下备份和恢复加密文件。这会需要特殊的 API
而这API 尚未实现

## 加密策略强制执行


在目录上设置加密策略后，在该目录中创建的所有常
文件、目录和符号链接（递归地）
都将继承该加密策略。特殊文---
即命名管道、设备节点和 UNIX 域套接字 --- 
不会被加密

除这些特殊文件外，禁止在
加密目录树中存在未加密的文件，或采用不同加密策略加密的文件。尝
将此类文件链接或重命名到
加密目录中将失败并返EXDEV。这也会
->lookup() 期间被强制执行，以提供针对离
攻击的有限保护，这类攻击试图在应用程序稍后可能写
敏感数据的已知位置禁用或降级加密。建
实现“验证启动”（verified boot）这类机制的系统利用
这一点，在访问前验证所有顶层加密策略

## 内联加密支持


许多较新的系统（尤其是移SoC）拥*内联加密
硬件*，能够在数据进出存储设备的过程中对其进行加密/解密
Linux 通过一
对块层的扩展（称**blk-crypto**）支持内联加密*blk-crypto** 允许
文件系统将加密上下文附加bios（I/O 请求）上，以
指定数据将如何被在线加密或解密。有
blk-crypto 的更多信息，请参
Documentation/block/inline-encryption.rst <inline_encryption>銆。

在受支持的文件系统（目前ext4 f2fs）上，fscrypt 可以使用
blk-crypto 代替内核 crypto API 来加解密文件
内容。要启用此功能，需在内核配置中设置 CONFIG_FS_ENCRYPTION_INLINE_CRYPT=y
并在挂载文件系统时指"inlinecrypt" 挂载选项
銆。

注意inlinecrypt" 挂载选项只是表示在可能时使用内联
加密；它并不强制使用。fscrypt 
仍然会回退到使用内crypto API，对于那
内联加密硬件不具备所需加密能力
例如，不支持所需的加密算法和数据单元大小
blk-crypto-fallback 不可用的文件。（要使 blk-crypto-fallback
可用，必须在内核配置中通过
CONFIG_BLK_INLINE_ENCRYPTION_FALLBACK=y 启用它，并且该文件必
由原始密钥而非硬件封装密钥保护。）

目前，fscrypt 始终使用文件系统块大小（通常
4096 字节）作为数据单元大小。因此，它只能使
支持该数据单元大小的内联加密硬件

内联加密不会影响密文
磁盘上格式的其它方面，因此用户可以自由地
使用 "inlinecrypt" 和不使用 "inlinecrypt" 之间切换。一个例外是
由硬件封装密钥保护的文件只能
内联加密硬件加密/解密，因
仅能在使"inlinecrypt" 挂载选项时被访问。有
硬件封装密钥的更多信息，请参阅下文

### 硬件封装密钥


褰撳唴鑱。
加密硬件支持时，fscrypt 支持使用 **硬件封装密钥**。这类密钥在 kernel 内存
仅以封装（加密）形式存在；它们只能被
内联加密硬件解封（解密），并且与
当前启动临时绑定。这可以防止在内核内存泄漏时密钥被泄露
这是在不限制
可用密钥数量的情况下实现的，同时仍允许执
那些绑定到同一密钥但无法使用内
加密硬件的加密任务，例如文件名加密

注意，硬件封装密钥并fscrypt 所特有；它们是
一个块层特性（**blk-crypto** 的一部分）。有
硬件封装密钥的更多细节，请参阅块层文
:ref:Documentation/block/inline-encryption.rst <hardware_wrapped_keys>。本节的其余部分只关
fscrypt 如何使用硬件封装密钥的细节

fscrypt 支持硬件封装密钥的方式是：允许将 fscrypt 
密钥作为原始密钥的替代，设为硬件封装密钥。要
使用 FS_IOC_ADD_ENCRYPTION_KEY_ 添加硬件封装密钥
用户空间必须
flags 的字段中（struct fscrypt_add_key_arg），以及
flags 的字段中（struct fscrypt_provisioning_key_payload，在适用时）
指定 FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED。密钥必须采用临时封装形式，而非
长期封装形式

存在一些限制。首先，由硬件封装密钥保护的
文件与系统的内联加密硬件绑定。因
它们只能在使"inlinecrypt" 挂载选项时被访问
并且不能被包含在可移植的文件系统映像中。其次，
目前硬件封装密钥支持仅与
IV_INO_LBLK_64 policies_ IV_INO_LBLK_32 policies_ 兼容，因
它假设每fscrypt 主密钥只有一个文件内容加密密钥，
而非每个文件一个。未来的工作可能会通过
将每文件 nonce 向下传递到存储栈，
让硬件派生每文件密钥，从而解决此限制

在实现层面，要对由硬件封装密钥保护的
文件内容进行加密/解密，fscrypt 使用 blk-crypto
将硬件封装密钥附加到 bio 加密上下文。与
原始密钥的情况一样，块层会将该密钥编程到
keyslot 中（当它尚未在其中时）。然而，在编
硬件封装密钥时，硬件不会将给定密钥直
编程keyslot 中，而是将其解封（使用硬件的
临时封装密钥）并从中派生出内联加密密钥
内联加密密钥是实际被编程
keyslot 中的密钥，并且它永远不会暴露给软件

然而，fscrypt 不只是做文件内容加密；它
使用其主密钥派生文件名加密密钥、密
标识符，有时还会派生一些更晦涩类型的子密钥，例
dirhash 密钥。因此，即使文件内容加密
不在考虑范围内，fscrypt 仍然需要一个原始密钥来工作。要获得这样
密钥，fscrypt 会请求内联加
硬件从硬件封装密钥派生出一个加密隔离的 "software secret"（软件密钥）
fscrypt 使用"software secret" 作为密钥
驱动KDF，以派生除文件内容密钥之外的所有子密钥

注意，这意味着硬件封装密钥特性仅
保护文件内容加密密钥。它不保护其
fscrypt 子密钥，例如文件名加密密钥

## Direct I/O 支持


要使加密文件上的 direct I/O 正常工作，必须满足以
条件（此外还需满足
未加密文件上 direct I/O 的条件）

- 该文件必须使用内联加密。通常这意味着
  文件系统必须使用 -o inlinecrypt 挂载，并
  必须存在内联加密硬件。不过，也存在一种软件回退
  方案。有关细节，请参Inline encryption support_

- I/O 请求必须与文件系统块大小完全对齐
  这意味着 I/O 所针对的文件位置
  所I/O 段的长度，以及所I/O 缓冲区的
  内存地址，都必须是该值的整数倍。注意，文件系统
  大小可能大于块设备的逻辑块大小

如果上述任一条件未满足，则加密文件上
direct I/O 会回退到缓I/O

## 实现细节


### 加密上下


加密策略在磁盘上
struct fscrypt_context_v1 struct fscrypt_context_v2 表示。由
各个文件系统自行决定存储位置，但通常
会存储在一个隐藏的扩展属性中。它**不应**
xattr 相关的系统调用（getxattr() 
setxattr()）暴露，因为加密 xattr 具有特殊语义
（特别是，如果将加密策略
添加或移除到除一个空对象之外的任何对象上
```

    #define FSCRYPT_FILE_NONCE_SIZE 16

    #define FSCRYPT_KEY_DESCRIPTOR_SIZE  8
    struct fscrypt_context_v1 {
            u8 version;
            u8 contents_encryption_mode;
            u8 filenames_encryption_mode;
            u8 flags;
            u8 master_key_descriptor[FSCRYPT_KEY_DESCRIPTOR_SIZE];
            u8 nonce[FSCRYPT_FILE_NONCE_SIZE];
    };

    #define FSCRYPT_KEY_IDENTIFIER_SIZE  16
    struct fscrypt_context_v2 {
            u8 version;
            u8 contents_encryption_mode;
            u8 filenames_encryption_mode;
            u8 flags;
            u8 log2_data_unit_size;
            u8 __reserved[3];
            u8 master_key_identifier[FSCRYPT_KEY_IDENTIFIER_SIZE];
            u8 nonce[FSCRYPT_FILE_NONCE_SIZE];
    };

```
都会造成很大的混乱。）上下文结构体包含与相
策略结构体相同的信息（见 Setting an encryption policy_），不同之处在于
上下文结构体还包含一nonce。该 nonce 由内
随机生成，并用作 KDF 输入或作tweak，以
不同文件被不同地加密；见 Per-file encryption keys_ DIRECT_KEY policies_

### 数据路径改动


当使用内联加密时，文件系统只需
加密上下文与 bios 关联，以指定块层
内联加密硬件将如何加解密文件内容

当不使用内联加密时，文件系统必须自行
加密/解密文件内容，如下所述：

对于常规文件的读取路径（->read_folio()），文件系统可以
将密文读入页缓存并就地解密。在
解密完成之前必须持有 folio 锁，以防
folio 过早对用户空间可见

对于常规文件的写入路径（->writepages()），文件系统
无法在页缓存中就地加密数据，因为缓存
明文必须保留。相反，文件系统必须加密到一
临时缓冲区或 "bounce page"（反弹页）中，然后写出该临时
缓冲区。某些文件系统（UBIFS）无论是否加
都已使用临时缓冲区。其它文件系统（ext4 
F2FS）必须为加密专门分配 bounce page

### 文件名哈希与编码


现代文件系统通过使用索引
目录来加速目录查找。索引目录被组织成一棵以
文件名哈希为键的树。当请求 ->lookup() 时，文件系统
通常会对所查找的文件名进行哈希，以便快
找到对应的目录项（如果存在）

在加密情况下，查找必须同时支持带密钥和不带密
两种情形且都高效。显然，
明文文件名进行哈希是行不通的，因为在没有密钥
明文文件名不可用。（对明文文件名进行哈希还会
文件系统fsck 工具无法优化加密
目录。）相反，文件系统对密文文件名进行哈希，
即目录项中实际存储在磁盘上的字节。当
使用密钥执行 ->lookup() 时，文件系统只需加密
用户提供的名称即可得到密文

不带密钥的查找更为复杂。原始密文可
包含 \0 / 字符，这些字符在
文件名中非法的。因此，readdir() 必须对密文进base64url 编码
以供呈现。对于大多数文件名，这没有问题；->lookup() 时，
文件系统只需对用户提供的名称进行 base64url 解码，以得到
原始密文

然而，对于非常长的文件名，base64url 编码会导
文件名长度超NAME_MAX。为防止此情况，readdir()
实际上以缩写形式呈现长文件名，该形式编码
密文文件名的"hash"，以及进行目录查找所需
可选的、文件系统特定的哈希。这
使文件系统仍能以很高的置信度，将
->lookup() 中给出的文件名映射回之前readdir() 列出
特定目录项。参
源码中的 struct fscrypt_nokey_name 以了解更多细节

注意，在没有密钥时向用户空间呈现文件名的确切方式
在未来可能会发生变化。它只是作为一种临时方式，
用于呈现有效的文件名，以便诸
rm -r 之类的命令在加密目录上按预期工作

## 测试


要测fscrypt，请使用 xfstests，它Linux 事实上的标准
文件系统测试套件。首先，在相关文件系统上运行 "encrypt"
组中的所有测试。也可以使用
  使用 "inlinecrypt" 挂载选项运行测试，以测试
内联加密支持的实现。例如，要测ext4 
f2fs 加密，使`kvm-xfstests
```

    kvm-xfstests -c ext4,f2fs -g encrypt
    kvm-xfstests -c ext4,f2fs -g encrypt -m inlinecrypt

```
UBIFS 加密也可以用这种方式测试，但应在
单独的命令中进行，并kvm-xfstests 需要一些时间来设置
```

    kvm-xfstests -c ubifs -g encrypt

```
不应有测试失败。然而，使用非默认加
模式的测试（例如 generic/549 generic/550）如果所需
算法未内置到内核crypto API 中，则会被跳过。此外，
访问原始块设备的测试（例generic/399、generic/548
generic/549、generic/550）在 UBIFS 上会被跳过

除了运行 "encrypt" 组测试外，对ext4 f2fs，还可以
使用 "test_dummy_encryption" 挂载选项运行大多xfstests
该选项会使所有新文件自动
使用虚拟密钥进行加密，而无需进行任何 API 调用
这会更彻底地测试加密I/O 路径。要使用
```

    kvm-xfstests -c ext4/encrypt,f2fs/encrypt -g auto
    kvm-xfstests -c ext4/encrypt,f2fs/encrypt -g auto -m inlinecrypt

```
来执行此操作，由于它运行的测试比 "-g encrypt" 多得多，
运行时间也长得多；因此也可考虑使用 gce-xfstests <https://github.com/tytso/xfstests-bld/blob/master/Documentation/gce-xfstests.md>_
```

    gce-xfstests -c ext4/encrypt,f2fs/encrypt -g auto
    gce-xfstests -c ext4/encrypt,f2fs/encrypt -g auto -m inlinecrypt

```