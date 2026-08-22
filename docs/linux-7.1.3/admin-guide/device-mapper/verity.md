## dm-verity


Device-Mapper 的“verity”目标使用内crypto API 提供的密码学摘要，对块设备进透明的完整性检查。该目标是只读的
## 构造参

```

    <version> <dev> <hash_dev>
    <data_block_size> <hash_block_size>
    <num_data_blocks> <hash_start_block>
    <algorithm> <digest> <salt>
    [<#opt_params> <opt_params>]

```
<version>
    这是磁盘上哈希格式的类型
    0 Chromium OS 中使用的原始格式。加盐（salt）在哈希时追加，摘要连续存储      块的其余部分用零填充
    1 是当前格式，应用于新设备。加盐在哈希时前置，每个摘要用零填充2 的幂
<dev>
    这是包含需要检查其完整性的数据的设备。它可以指定为路径，/dev/sdaX，或
    设备<major>:<minor>
<hash_dev>
    这是提供哈希树数据的设备。它可以类似于设备路径的方式指定，也可以是同一个设备    如果使用同一设备，则 hash_start 应在配置dm-verity 设备之外
<data_block_size>
    数据设备上的块大小（字节）。每个块对应哈希设备上的一个摘要
<hash_block_size>
    哈希块的大小（字节）
<num_data_blocks>
    数据设备上的数据块数量。额外的块不可访问。你可以将哈希放在与数据相同的分区上    在这种情况下，哈希放<num_data_blocks> 之后
<hash_start_block>
    这是hash_dev 起始到哈希树根块的偏移，<hash_block_size> 块为单位
<algorithm>
    该设备使用的密码学哈希算法。这应该是算法的名称，如“sha1”
<digest>
    根哈希块和盐的密码学哈希的十六进制编码。这个哈希应当被信任，因为在此之外没    其他真实性保证
<salt>
    盐值的十六进制编码
<#opt_params>
    可选参数的数量。如果没有可选参数，则可以跳过可选参数段，或#opt_params 可以
    为零。否#opt_params 是后面参数的数量
    可选参数段示例        1 ignore_corruption

ignore_corruption
    记录损坏的块，但允许读操作正常进行
restart_on_corruption
    当发现损坏的块时重启系统。此选项ignore_corruption 不兼容，并且需要用户空    支持以避免重启循环
panic_on_corruption
    当发现损坏的块时使设panic。此选项ignore_corruption restart_on_corruption
    不兼容
restart_on_error
    当检测到 I/O 错误时重启系统。此选项可以restart_on_corruption 选项组合
panic_on_error
    当检测到 I/O 错误时使设备 panic。此选项restart_on_error 选项不兼容，但可以与
    panic_on_corruption 选项组合
ignore_zero_blocks
    不验证预期包含零的块，而是始终返回零。如果分区包含不保证包含零的未使用块，这
    可能很有用
use_fec_from_device <fec_dev>
    使用来自指定设备的前向纠错（FEC）奇偶数据，尝试自动从损坏和 I/O 错误中恢复
    如果给出了此选项，则还必须给<fec_roots> <fec_blocks>hash_block_size>
    也必须等<data_block_size>
    <fec_dev> 可以<dev> 相同，此<fec_start> 必须在数据区域之外。它也可以与
    <hash_dev> 相同，此<fec_start> 必须在哈希和可选附加元数据区域之外
    如果数据 <dev> 是加密的，则 <fec_dev> 也应加密
    更多信息，参`Forward error correction`_
fec_roots <num>
    每个 255 字节 Reed-Solomon 码字中的奇偶字节数。使用的 Reed-Solomon 码将RS(255, k)
    码，其中 k = 255 - fec_roots
    支持的值为 2 24（含）。值越高提供越强的纠错能力。然而，由于使用了交织，最小    2 已经提供了很强的纠错能力，因2 是大多数用户的推荐值。fec_roots=2 对应
    RS(255, 253) 码，其空间开销约为 0.8%
fec_blocks <num>
    使用 FEC 进行错误检查的 <data_block_size> 块的总数。它必须至少<num_data_blocks>
    加上哈希树所需块数之和。它可以包含附加元数据块，这些块假定在哈希块之后、可    <hash_dev> 上访问
    注意，这**不是**奇偶块的数量。奇偶块的数量由 <fec_blocks>fec_roots>     <data_block_size> 推断得出
fec_start <offset>
    这是<fec_dev> 起始到奇偶数据开头的偏移，以 <data_block_size> 块为单位
check_at_most_once
    仅在数据块首次从数据设备读出时验证它们，而不是每次都验证。这降低dm-verity     开销，使其可用于内存CPU 受限的系统。然而，它提供的安全级别较低，因为只    检测到对数据设备内容的离线篡改，而非在线篡改
    哈希块在每次从哈希设备读出时仍会被验证，因为哈希块的验证不如数据块对性能关键    而且一个哈希块在其覆盖的所有数据块都被验证后不再会被验证
root_hash_sig_key_desc <key_description>
    这是 USER_KEY 的描述，内核将查找它以获roothash pkcs7 签名。pkcs7 签名用于
    在创建设备映射器块设备期间验证根哈希。roothash 的验证依赖于内核中设置了
    DM_VERITY_VERIFY_ROOTHASH_SIG 配置。默认情况下，签名针对内建可信密钥环进行检查，
    如果设置DM_VERITY_VERIFY_ROOTHASH_SIG_SECONDARY_KEYRING，则针对二级可信密钥    检查。二级可信密钥环默认包含内建可信密钥环，并且如果在运行时由已存在于二级可    密钥环中的证书签名，它也可以获得新证书
try_verify_in_tasklet
    如果 verity 哈希在缓存中IO 大小未超过限制，则在底半部而非工作队列中验证数据块    此选项可以降低 IO 延迟。大小限制可通过
    /sys/module/dm_verity/parameters/use_bh_bytes 配置。这四个参数依次对应    IOPRIO_CLASS_NONE、IOPRIO_CLASS_RT、IOPRIO_CLASS_BE IOPRIO_CLASS_IDLE 的限制    例如    <none>,<rt>,<be>,<idle>
    4096,4096,4096,4096

## 工作原理


dm-verity 旨在作为已验证引导路径的一部分进行设置。这可以是任何范围，从使tboot trustedgrub 引导，到仅从已知良好的设备（USB 驱动器或 CD）引导
当配dm-verity 设备时，期望调用方已通过某种方式（密码学签名等）进行了认证。实例化
之后，所有哈希将在磁盘访问期间按需验证。如果无法一直验证到树的根节点（根哈希），则
I/O 将失败。这应能检测到对设备上任何数据以及哈希数据的篡改
密码学哈希用于在逐块基础上断言设备的完整性。这允许在首次读入页缓存时进行轻量的哈希
计算。块哈希线性存储，对齐到最近的块大小
### 哈希

树中的每个节点都是一个密码学哈希。如果它是叶节点，则计算磁盘上某个数据块的哈希。如它是中间节点，则计算若干子节点的哈希
树中的每个条目是适合放入一个块的相邻节点的集合。数量由 block_size 和所选密码学摘要
算法的大小决定。哈希在此条目中线性排序，任何未对齐的尾部空间被忽略，但在计算父节点时
会包含它
树看起来类似这样
	alg = sha256, num_blocks = 32768, block_size = 4096

```

                                 [   root    ]
                                /    . . .    \
                     [entry_0]                 [entry_1]
                    /  . . .  \                 . . .   \
         [entry_0_0]   . . .  [entry_0_127]    . . . .  [entry_1_127]
           / ... \             /   . . .  \             /           \
     blk_0 ... blk_127  blk_16256   blk_16383      blk_32640 . . . blk_32767

```
### 前向纠错


dm-verity 可选的前向纠错（FEC）支持为 dm-verity 增加了强大的纠错能力。它使那些会错误而无法运行的系统能够继续运行，尽管性能会下降
FEC 使用跨整个设备交织的 Reed-Solomon（RS）码，允许恢复长串损坏或不可读的块
dm-verity 在使用任FEC 纠正的块之前，会根据期望的哈希对其进行验证。因此，FEC 影响 dm-verity 的安全属性
FEC dm-verity 的集成为单独的纠错层提供了显著优势：

- dm-verity 仅在块的哈希与期望哈希不匹配或块根本无法读出时才调用 FEC。因此，在没  发生错误的常见情况下，FEC 不会增加开销
- dm-verity 哈希也用于识RS 解码的擦除位置。这允许纠正多一倍的错误
FEC 使用 RS(255, k) 码，其中 k = 255 - fec_roots。fec_roots 通常2。这意味着每个
k（通常253）个消息字节都添加了 fec_roots（通常2）个字节的奇偶数据，以得到一255 字节的码字。（许多外部资料RS 码字为“块”。由dm-verity 已经使用术语“块”来表示
其他含义，我们将使用更清晰的术语“RS 码字”。）

FEC 总共检fec_blocks 个消息数据块，包括：

1. 来自数据设备的数据块
2. 来自哈希设备的哈希块
3. 在哈希设备上紧跟哈希块的可选附加元数据

dm-verity 假设 FEC 奇偶数据是按如下过程计算的：

1. 拼接来自上述源的消息数据2. 零填充到下一k 块的倍数。令 msg 为得到的字节数组，msglen 为其长度（字节）3. 对于 0 <= i < msglen / k（对于每RS 码字）：
     a. 对于 0 <= j < k，选择 msg[i + j * msglen / k]        将它们视为一RS 码字的“k”个消息字节     b. 计算RS 码字对应的“fec_roots”个奇偶字节，并将它们拼接至 FEC 奇偶数据
步骤 3a 使用交织data_block_size * ceil(fec_blocks / k) 在整个设备上交织 RS 码字这是最大交织，使得消息数据由一个包含所RS 码字0 字节的区域组成，然后是一个包含所RS 码字1 字节的区域，依此类推，直到“k - 1”字节的区域。注意，码字数量被设置为
data_block_size 的倍数；因此，这些区域是块对齐的，并且存在最多“k - 1”块的隐式零填充
这种交织允许纠正长串错误。它在保持低空间开销的同时，提供了比存储设备通常提供的强得多纠错能力
代价是解码缓慢：纠正单个块通常需要在整个设备（们）上均匀读取 254 个额外块。然而，这是
可接受的，因dm-verity 仅在实际发生错误时才使用 FEC
以下列表包含关于 dm-verity FEC 所使用RS 码的更多细节。生成奇偶数据的用户空间程序
需要使用这些参数，才能使奇偶数据精确匹配：

- 使用的域GF(256)
- 字节以自然方式映射进/GF(256) 元素，其中位 0 7（低位到高位）映射到 x^0 x^7
  的系- 域生成多项式x^8 + x^4 + x^3 + x^2 + 1
- 使用的码是系统码、BCH 视图- 本原alpha 为“x- 码生成多项式的第一个连续根为“x^0
## 磁盘上格

verity 内核代码不读取磁盘上verity 元数据头。它只读取紧跟在头之后的哈希块。预用户空间工具将验verity 头的完整性
或者，可以省略头，并通过内核命令行传dmsetup 参数，在命令行经过验证的信任链中
紧跟在头之后（扇区号填充到下一个哈希块边界）的是哈希块，它们按深度逐层存储（从根开始）按索引递增顺序排序
内核参数和磁盘上元数据格式的全部规范可在 cryptsetup 项目wiki 页面获取

  https://gitlab.com/cryptsetup/cryptsetup/wikis/DMVerity

## 状

1. 如果到目前为止执行的每个检查都有效，则返回 V（Valid，有效）   如果任何检查失败，则返C（Corruption，损坏）2. 前向纠错纠正的块数。如果未启用前向纠错，则为”
## 示例


```

  # dmsetup create vroot --readonly --table \
    "0 2097152 verity 1 /dev/sda1 /dev/sda2 4096 4096 262144 1 sha256 "\
    "4392712ba01368efdf14b05c76f9e4df0d53664630b5d48632ed17a137f39076 "\
    "1234000000000000000000000000000000000000000000000000000000000000"

```
有一个命令行工具 veritysetup 可用于计算或验证哈希树，或激活内核设备。它可从 cryptsetup
上游仓库 https://gitlab.com/cryptsetup/cryptsetup/ 获取（作libcryptsetup 扩展）
```

  # veritysetup format /dev/sda1 /dev/sda2
  ...
  Root hash: 4392712ba01368efdf14b05c76f9e4df0d53664630b5d48632ed17a137f39076

```
```

  # veritysetup create vroot /dev/sda1 /dev/sda2 \
    4392712ba01368efdf14b05c76f9e4df0d53664630b5d48632ed17a137f39076

```
