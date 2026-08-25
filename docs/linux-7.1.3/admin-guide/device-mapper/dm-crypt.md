# dm-crypt


Device-Mapper “crypt目标利用内核 crypto API 提供对块设备的透明加密

有关所支持参数的更详细描述，请参见
https://gitlab.com/cryptsetup/cryptsetup/wikis/DMCrypt

```

	      <cipher> <key> <iv_offset> <device path> \
	      <offset> [<#opt_params> <opt_params>]

```
<cipher>
    加密算法（cipher）、加密模式以及初始向量（IV）生成器

```

       cipher[:keycount]-chainmode-ivmode[:ivopts]

    Examples::

       aes-cbc-essiv:sha256
       aes-xts-plain64
       serpent-xts-plain64

    Cipher format also supports direct specification with kernel crypt API
    format (selected by capi: prefix). The IV specification is the same
    as for the first format type.
    This format is mainly used for specification of authenticated modes.

    The crypto API cipher specifications format is::

        capi:cipher_api_spec-ivmode[:ivopts]

    Examples::

        capi:cbc(aes)-essiv:sha256
        capi:xts(aes)-plain64

    Examples of authenticated modes::

        capi:gcm(aes)-random
        capi:authenc(hmac(sha256),xts(aes))-random
        capi:rfc7539(chacha20,poly1305)-random

    The /proc/crypto contains a list of currently loaded crypto modes.

```
<key>
    用于加密的密钥。它既可以编码为十六进制数字
    or it can be passed as <key_string> prefixed with single colon
    character (':') for keys residing in kernel keyring service.
    You can only use key sizes that are valid for the selected cipher
    in combination with the selected iv mode.
    Note that for some iv modes the key string can contain additional
    keys (for example IV seed) so the key contains more parts concatenated
    into a single string.

<key_string>
    内核密钥环（keyring）密钥通过以下格式的字符串标识
    <key_size>:<key_type>:<key_description>銆。

<key_size>
    加密密钥的大小（以字节为单位）。内核密钥的载荷大小必须<key_size> 中传入的值相匹配

<key_type>
    ‘logon’、‘user’、‘encrypted‘trusted内核密钥类型之一

<key_description>
    crypt 目标在加<key_type> 类型密钥时应查找的内核密钥环密钥描述

<keycount>
    Multi-key compatibility mode. You can define <keycount> keys and
    then sectors are encrypted according to their offsets (sector 0 uses key0;
    sector 1 uses key1 etc.).  <keycount> must be a power of two.

<iv_offset>
    IV 偏移是一个扇区计数，在创IV 之前会被加到扇区号上

<device path>
    这是将用作后端并包含加密数据的设备。你可以将其指定为类/dev/xxx 的路径，或指定为设备<major>:<minor>

<offset>
    设备内加密数据开始的起始扇区

<#opt_params>
    可选参数的数量。如果没有可选参数，则可以跳过可选参数部分，或者将 #opt_params 设为零。否#opt_params 为后续参数的数量

    可选参数部分示例：
        3 allow_discards same_cpu_crypt submit_from_crypt_cpus

allow_discards
    块丢弃请求（TRIM）会被透传crypt 设备。默认行为是忽略丢弃请求

    WARNING：在启用此选项之前，请仔细评估特定的安全风险。例如，在加密设备上允许丢弃可能导致密文设备（文件系统类型、已用空间等）的信息泄露，前提是后续可以在设备上轻易定位到被丢弃的块

same_cpu_crypt
    使用提交 IO 时所用的同一CPU 执行加密。默认是使用未绑定的工作队列，从而让加密工作在各可用 CPU 之间自动均衡

high_priority
    dm-crypt 工作队列和写入线程设为高优先级。这会在降低系统整体响应能力的同时，提升 dm-crypt 的吞吐量与延迟

submit_from_crypt_cpus
    禁用加密后将写入操作卸载到单独线程的做法。在某些情况下，将写bio 从加密线程卸载到单个线程会显著降低性能。默认是将写bio 卸载到同一线程，因为使用相同上下文提交写入CFQ 有益

no_read_workqueue
    绕过 dm-crypt 内部工作队列，并同步处理读取请求

no_write_workqueue
    绕过 dm-crypt 内部工作队列，并同步处理写入请求。对于主机管理的规区（zoned）块设备（例如主机管理的 SMR 硬盘），此选项会自动启用

integrity:<bytes>:<type>
    The device requires additional <bytes> metadata per-sector stored
    in per-bio integrity structure. This metadata must by provided
    by underlying dm-integrity target.

    The <type> can be "none" if metadata is used only for persistent IV.

    For Authenticated Encryption with Additional Data (AEAD)
    the <type> is "aead". An AEAD mode additionally calculates and verifies
    integrity for the encrypted device. The additional space is then
    used for storing authentication tag (and persistent IV if needed).

integrity_key_size:<bytes>
    如果与摘要大小不同，可选择性地设置完整性密钥大小。它允许使用封装密钥（wrapped key）算法，其中密钥大小与加密密钥大小无关

sector_size:<bytes>
    Use <bytes> as the encryption unit instead of 512 bytes sectors.
    This option can be in range 512 - 4096 bytes and must be power of two.
    Virtual device will announce this size as a minimal IO and logical sector.

iv_large_sectors
   IV 生成器将使用<sector_size> 为单位计数的扇区号，而不是默认的 512 字节扇区

   例如，如<sector_size> 4096 字节，则第二个扇区的 plain64 IV 在没有该标志时为 8，而在存在 iv_large_sectors 时为 1。如果指定了该标志，<iv_offset> 必须<sector_size> 的倍数（以 512 字节为单位）

integrity_key_size:<bytes>
   使用大小<bytes> 的完整性密钥，而不是使用所HMAC 算法的摘要大小的完整性密钥


```
   max_read_size
      Maximum size of read requests. When a request larger than this size
      is received, dm-crypt will split the request. The splitting improves
      concurrency (the split requests could be encrypted in parallel by multiple
      cores), but it also causes overhead. The user should tune this parameters to
      fit the actual workload.

   max_write_size
      Maximum size of write requests. When a request larger than this size
      is received, dm-crypt will split the request. The splitting improves
      concurrency (the split requests could be encrypted in parallel by multiple
      cores), but it also causes overhead. The user should tune this parameters to
      fit the actual workload.


```
示例脚本

LUKS（Linux Unified Key Setup）现在是使用 'cryptsetup' 工具配合 dm-crypt 设置磁盘加密的首选方式，请参
https://gitlab.com/cryptsetup/cryptsetup


```

	#!/bin/sh
	# Create a crypt device using dmsetup
	dmsetup create crypt1 --table "0 `blockdev --getsz $1` crypt aes-cbc-essiv:sha256 babebabebabebabebabebabebabebabe 0 $1 0"

```
```

	#!/bin/sh
	# Create a crypt device using dmsetup when encryption key is stored in keyring service
	dmsetup create crypt2 --table "0 `blockdev --getsize $1` crypt aes-cbc-essiv:sha256 :32:logon:my_prefix:my_key 0 $1 0"

```
```

	#!/bin/sh
	# Create a crypt device using cryptsetup and LUKS header with default cipher
	cryptsetup luksFormat $1
	cryptsetup luksOpen $1 crypt1

```