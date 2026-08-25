## 可信密钥与加密密
可信密钥（Trusted Keys）与加密密钥（Encrypted Keys）是添加到现有内核密钥环服务的两种新密钥类型。这两种新类型均为可变长度对称密钥，且两种情况下所有密钥都在内核中创建，用户空间只能看到、存储和加载加密后的 blob。可信密钥需要信任源（Trust Source）的可用以提供更高安全性，而加密密钥可在任何系统上使用。所有用户层 blob 都以十六进制 ASCII 形式显示和加载，以方便使用，并经过完整性校验
## 作为保护密钥（Protected Key）的可信密钥

这是一种将密钥作为可信密钥保存在内核密钥环中的安全方式，即
- Key-blob（密blob），即加密后的密钥数据，由用户空间存储、加载和查看- Key-data（密钥数据），系统中的明文密钥文本，仅供内核空间使用
尽管密钥数据以明文形式对用户空间不可访问，但在内核空间使用时，它以明文形式存在于系统内存中。即便内核空间攻击面较小，但若内核被攻破或通过侧信道攻击访问系统内存，仍可能导致密钥被攻破或泄漏
为了保护内核空间中的密钥，引入了“保护密钥”（protected-keys）的概念，作为额外的一层保护。保护密钥的密钥数据使用密钥加密密钥（KEK，Key-Encryption-Key）加密，并在信任源边界内解密。明文密钥文本绝不会出现在系统内存外部。因此，任何使用保护密钥执行的加密操作，只能由生成该密钥 blob 的信任源完成
因此，即使保护密钥被泄漏或攻破，对攻击者而言也毫无用处
作为保护密钥的可信密钥，其信任源具备生成如下内容的能力：

- Key-Blob（密Blob），由用户空间加载、存储和查看
## 淇′换婧。
信任源为可信密钥提供安全性来源。本节列出当前支持的信任源及其安全考量。某个信任源是否足够安全，取决于其实现的强度与正确性，以及特定使用场景的威胁环境。由于内核并不知道具体的环境，也没有信任度量标准，因此是否采用该信任源需要由可信密钥的使用者自行判断
  - 存储根信
     (1) TPM（可信平台模块：硬件设备
         根植于存储根密钥（SRK），该密钥永远不会离开 TPM，由 TPM 提供加密操作以建立存储根信任
     (2) TEE（可信执行环境：基于 Arm TrustZone OP-TEE
         根植于硬件唯一密钥（HUK），该密钥通常烧录在片上熔丝中，仅 TEE 可访问
     (3) CAAM（加密加速与保证模块：NXP SoC 上的 IP
         当启用高保证引导（HAB）且 CAAM 处于安全模式时，信任根植OTPMK——一个从不公开256 位密钥，在制造时随机生成并熔入每SoC。否则，将使用一个通用的固定测试密钥
     (4) DCP（数据协处理器：多种 i.MX SoC 的加密加速器
         根植于一次性可编程密钥（OTP），该密钥通常烧录在片上熔丝中，仅 DCP 加密引擎可访问。DCP 提供两个可用作信任根的密钥：OTP 密钥UNIQUE 密钥。默认使UNIQUE 密钥，但可通过模块参数（dcp_use_otp_key）选择 OTP 密钥
     (5) PKWM（PowerVM 密钥封装模块：IBM PowerVM + Platform KeyStore
         根植于唯一的、每LPAR 的密钥，该密钥派生自系统范围内随机生成的 LPAR 根密钥。每LPAR 的密钥和 LPAR 根密钥在运行时都存储hypervisor 拥有的安全内存中，且 LPAR 根密钥还持久保存在处理器 SEEPROM 和加NVRAM 等安全位置
  - 执行隔离

     (1) TPM

         在隔离执行环境中运行的一组固定操作
     (2) TEE

         在经安全/可信引导过程验证的隔离执行环境中运行的一组可定制操作
     (3) CAAM

         在隔离执行环境中运行的一组固定操作
     (4) DCP

         在隔离执行环境中运行的一组固定加密操作。那里仅执行基本blob 密钥加密。实际的密钥封印/解封在主处理内核空间完成
     (5) PKWM（PowerVM 密钥封装模块：IBM PowerVM + Platform KeyStore
         在片上硬件加密加速单NX 上完成的一组固定加密操作。封装与解封装所用的密钥PowerVM Platform KeyStore 管理，该 KeyStore 将密钥存储在安全 hypervisor 内存中一个隔离的内存副本里，以及 hypervisor 加密NVRAM 中的持久副本里
  - 与平台完整性状态的可选绑
     (1) TPM

         密钥可以选择性地封印到指定的 PCR（完整性度量）值，并且仅当 PCR blob 完整性校验匹配时，才TPM 解封。已加载的可信密钥可以用新的（未来的）PCR 值更新，因此密钥可以轻松迁移到新PCR 值，例如在更新内核和 initramfs 时。同一个密钥可以在不同PCR 值下拥有多个已保存的 blob，因此轻松支持多次启动
     (2) TEE

         依赖安全/可信引导过程来保证平台完整性。它可以通过基于 TEE 的度量引导过程进行扩展
     (3) CAAM

         依赖 NXP SoC 的高保证引导（HAB）机制来保证平台完整性
     (4) DCP

         依赖安全/可信引导过程（供应商称之HAB）来保证平台完整性
     (5) PKWM（PowerVM 密钥封装模块：IBM PowerVM + Platform KeyStore
         依赖 IBM Power 系统的安全和可信引导过程来保证平台完整性
  - 接口API

     (1) TPM

         TPM 拥有文档完善、标准化的接口和 API
     (2) TEE

         TEE 拥有文档完善、标准化的客户端接口API。更多细节请参阅 `Documentation/driver-api/tee.rst`
     (3) CAAM

         接口特定于芯片厂商
     (4) DCP

         厂商特定API，作DCP 加密驱动的一部分实现`drivers/crypto/mxs-dcp.c`
     (5) PKWM（PowerVM 密钥封装模块：IBM PowerVM + Platform KeyStore
         Platform Keystore PAPR 文档中有完整文档的接口。请参阅 `Documentation/arch/powerpc/papr_hcalls.rst`

  - 威胁模型

     在使用特定信任源来保护与安全相关的数据时，必须评估该信任源针对给定用途的强度与适用性
## 密钥生成

### 可信密钥

新密钥由随机数创建。它们使用存储密钥层级中的子密钥进行加密/解密。子密钥的加密和解密必须由信任源内的强访问控制策略保护。所使用的随机数生成器因所选信任源而异
  - TPM：基于硬件设备的 RNG

     密钥TPM 内部生成。随机数的强度可能因设备制造商而异
  - TEE：基Arm TrustZone OP-TEE RNG

     RNG 可根据平台需求定制。它既可以是平台特定硬件 RNG 的直接输出，也可以是软件实现Fortuna CSPRNG，后者可通过多个熵源进行播种
  - CAAM：内RNG

     使用普通的内核随机数生成器。若要由 CAAM HWRNG 为其播种，请启用 CRYPTO_DEV_FSL_CAAM_RNG_API 并确保该设备已被探测
  - DCP（数据协处理器：多种 i.MX SoC 的加密加速器
     DCP 硬件设备本身不提供专用的 RNG 接口，因此使用内核默认的 RNG。像 i.MX6ULL 这样带有 DCP SoC 确实拥有独立DCP 的专用硬RNG，可以启用它来支撑内RNG
   - PKWM（PowerVM 密钥封装模块：IBM PowerVM + Platform KeyStore
     使用普通的内核随机数生成器来生成密钥
用户可以通过在内核命令行指定 `trusted.rng=kernel` 来覆盖此项，将所使用RNG 替换为内核的随机数池
### 加密密钥

加密密钥不依赖信任源，且由于使用 AES 进行加密/解密，速度更快。新密钥可由内核生成的随机数或用户提供的已解密数据创建，并使用指定的“主”（master）密钥进行加解密。“主”密钥可以是可信密钥或用户密钥类型。加密密钥的主要缺点是：如果它们没有根植于可信密钥，则其安全性仅等同于加密它们的用户密钥。因此，主用户密钥应尽可能以安全的方式加载，最好是在启动早期
## 用法

### 可信密钥用法：TPM

TPM 1.2：默认情况下，可信密钥封印在 SRK 之下，其默认授权值为0 字节0）。这可以takeownership 时通过 TrouSerS 工具设置tpm_takeownership -u -z"
TPM 2.0：用户必须先创建一个存储密钥并使其持久化，以便该密钥在重启后仍可用。可以使用以下命令完成
```

  #> tsscreateprimary -hi o -st
  Handle 80000000
  #> tssevictcontrol -hi o -ho 80000000 -hp 81000001

```
```

  #> tpm2_createprimary --hierarchy o -G rsa2048 -c key.ctxt
  [...]
  #> tpm2_evictcontrol -c key.ctxt 0x81000001
  persistentHandle: 0x81000001

```
```

    keyctl add trusted name "new keylen [options]" ring
    keyctl add trusted name "load hex_blob [pcrlock=pcrnum]" ring
    keyctl update key "update [options]"
    keyctl print keyid

    options:
       keyhandle=    ascii hex value of sealing key
                       TPM 1.2: default 0x40000000 (SRK)
                       TPM 2.0: no default; must be passed every time
       keyauth=	     ascii hex auth for sealing key default 0x00...i
                     (40 ascii zeros)
       blobauth=     ascii hex auth for sealed data default 0x00...
                     (40 ascii zeros)
       pcrinfo=	     ascii hex of PCR_INFO or PCR_INFO_LONG (no default)
       pcrlock=	     pcr number to be extended to "lock" blob
       migratable=   0|1 indicating permission to reseal to new PCR values,
                     default 1 (resealing allowed)
       hash=         hash algorithm name as a string. For TPM 1.x the only
                     allowed value is sha1. For TPM 2.x the allowed values
                     are sha1, sha256, sha384, sha512 and sm3-256.
       policydigest= digest for the authorization policy. must be calculated
                     with the same hash algorithm as specified by the 'hash='
                     option.
       policyhandle= handle to an authorization policy session that defines the
                     same policy and with the same hash algorithm as was used to
                     seal the key.

```
"keyctl print" 返回封印密钥ASCII 十六进制副本，其格式为标TPM_STORED_DATA。新密钥的密钥长度始终以字节为单位。可信密钥可以是 32 - 128 字节56 - 1024 位），上限是为了适配 2048 SRK（RSA）密钥长度，并包含必要的结构/填充
### 可信密钥用法：TEE

```

    keyctl add trusted name "new keylen" ring
    keyctl add trusted name "load hex_blob" ring
    keyctl print keyid

```
"keyctl print" 返回封印密钥ASCII 十六进制副本，其格式特定TEE 设备实现。新密钥的密钥长度始终以字节为单位。可信密钥可以是 32 - 128 字节56 - 1024 位）
### 可信密钥用法：CAAM

```

    keyctl add trusted name "new keylen" ring
    keyctl add trusted name "load hex_blob" ring
    keyctl print keyid

```
"keyctl print" 返回封印密钥ASCII 十六进制副本，其格式CAAM 特有格式。新密钥的密钥长度始终以字节为单位。可信密钥可以是 32 - 128 字节56 - 1024 位）
```

    keyctl add trusted name "new keylen pk [options]" ring
    keyctl add trusted name "load hex_blob [options]" ring
    keyctl print keyid

    where, 'pk' is used to direct trust source to generate protected key.

    options:
       key_enc_algo =      For CAAM, supported enc algo are ECB(2), CCM(1).

```
"keyctl print" 返回封印密钥ASCII 十六进制副本，其格式CAAM 特有格式。新密钥的密钥长度始终以字节为单位。可信密钥可以是 32 - 128 字节56 - 1024 位）
### 可信密钥用法：DCP

```

    keyctl add trusted name "new keylen" ring
    keyctl add trusted name "load hex_blob" ring
    keyctl print keyid

```
"keyctl print" 返回封印密钥ASCII 十六进制副本，其格式特定于此 DCP 密钥 blob 实现。新密钥的密钥长度始终以字节为单位。可信密钥可以是 32 - 128 字节56 - 1024 位）
### 可信密钥用法：PKWM

```

    keyctl add trusted name "new keylen [options]" ring
    keyctl add trusted name "load hex_blob" ring
    keyctl print keyid

    options:
       wrap_flags=   ascii hex value of security policy requirement
                       0x00: no secure boot requirement (default)
                       0x01: require secure boot to be in either audit or
                             enforced mode
                       0x02: require secure boot to be in enforced mode

```
"keyctl print" 返回封印密钥ASCII 十六进制副本，其格式特定PKWM 密钥 blob 实现。新密钥的密钥长度始终以字节为单位。可信密钥可以是 32 - 128 字节56 - 1024 位）
### 加密密钥用法

加密密钥的已解密部分可以包含简单的对称密钥或更复杂的结构。更复杂结构的格式是应用特定的，'format' 标识
```

    keyctl add encrypted name "new [format] key-type:master-key-name keylen"
        ring
    keyctl add encrypted name "new [format] key-type:master-key-name keylen
        decrypted-data" ring
    keyctl add encrypted name "load hex_blob" ring
    keyctl update keyid "update key-type:master-key-name"

```
```

	format:= 'default | ecryptfs | enc32'
	key-type:= 'trusted' | 'user'

```
### 可信密钥与加密密钥用法示
创建并保存一个名"kmk"、长度为 32 字节的可信密钥
注意：当使用具有句柄 0x81000001 持久密钥TPM 2.0 时，应在引号内的语句后附'keyhandle=0x81000001'，例"new 32 keyhandle=0x81000001"
```

    $ keyctl add trusted kmk "new 32" @u
    440502848

    $ keyctl show
    Session Keyring
           -3 --alswrv    500   500  keyring: _ses
     97833714 --alswrv    500    -1   \_ keyring: _uid.500
    440502848 --alswrv    500   500       \_ trusted: kmk

    $ keyctl print 440502848
    0101000000000000000001005d01b7e3f4a6be5709930f3b70a743cbb42e0cc95e18e915
    3f60da455bbf1144ad12e4f92b452f966929f6105fd29ca28e4d4d5a031d068478bacb0b
    27351119f822911b0a11ba3d3498ba6a32e50dac7f32894dd890eb9ad578e4e292c83722
    a52e56a097e6a68b3f56f7a52ece0cdccba1eb62cad7d817f6dc58898b3ac15f36026fec
    d568bd4a706cb60bb37be6d8f1240661199d640b66fb0fe3b079f97f450b9ef9c22c6d5d
    dd379f0facd1cd020281dfa3c70ba21a3fa6fc2471dc6d13ecf8298b946f65345faa5ef0
    f1f8fff03ad0acb083725535636addb08d73dedb9832da198081e5deae84bfaf0409c22b
    e4a8aea2b607ec96931e6f4d4fe563ba

    $ keyctl pipe 440502848 > kmk.blob

```
```

    $ keyctl add trusted kmk "load `cat kmk.blob`" @u
    268728824

    $ keyctl print 268728824
    0101000000000000000001005d01b7e3f4a6be5709930f3b70a743cbb42e0cc95e18e915
    3f60da455bbf1144ad12e4f92b452f966929f6105fd29ca28e4d4d5a031d068478bacb0b
    27351119f822911b0a11ba3d3498ba6a32e50dac7f32894dd890eb9ad578e4e292c83722
    a52e56a097e6a68b3f56f7a52ece0cdccba1eb62cad7d817f6dc58898b3ac15f36026fec
    d568bd4a706cb60bb37be6d8f1240661199d640b66fb0fe3b079f97f450b9ef9c22c6d5d
    dd379f0facd1cd020281dfa3c70ba21a3fa6fc2471dc6d13ecf8298b946f65345faa5ef0
    f1f8fff03ad0acb083725535636addb08d73dedb9832da198081e5deae84bfaf0409c22b
    e4a8aea2b607ec96931e6f4d4fe563ba

```
创建并保存一个名"kmk"、长度为 32 字节、作为保护密钥的可信密钥
```

    $ keyctl add trusted kmk "new 32 pk key_enc_algo=1" @u
    440502848

    $ keyctl show
    Session Keyring
           -3 --alswrv    500   500  keyring: _ses
     97833714 --alswrv    500    -1   \_ keyring: _uid.500
    440502848 --alswrv    500   500       \_ trusted: kmk

    $ keyctl print 440502848
    0101000000000000000001005d01b7e3f4a6be5709930f3b70a743cbb42e0cc95e18e915
    3f60da455bbf1144ad12e4f92b452f966929f6105fd29ca28e4d4d5a031d068478bacb0b
    27351119f822911b0a11ba3d3498ba6a32e50dac7f32894dd890eb9ad578e4e292c83722
    a52e56a097e6a68b3f56f7a52ece0cdccba1eb62cad7d817f6dc58898b3ac15f36026fec
    d568bd4a706cb60bb37be6d8f1240661199d640b66fb0fe3b079f97f450b9ef9c22c6d5d
    dd379f0facd1cd020281dfa3c70ba21a3fa6fc2471dc6d13ecf8298b946f65345faa5ef0
    f1f8fff03ad0acb083725535636addb08d73dedb9832da198081e5deae84bfaf0409c22b
    e4a8aea2b607ec96931e6f4d4fe563ba

    $ keyctl pipe 440502848 > kmk.blob

```
```

    $ keyctl add trusted kmk "load `cat kmk.blob` key_enc_algo=1" @u
    268728824

    $ keyctl print 268728824
    0101000000000000000001005d01b7e3f4a6be5709930f3b70a743cbb42e0cc95e18e915
    3f60da455bbf1144ad12e4f92b452f966929f6105fd29ca28e4d4d5a031d068478bacb0b
    27351119f822911b0a11ba3d3498ba6a32e50dac7f32894dd890eb9ad578e4e292c83722
    a52e56a097e6a68b3f56f7a52ece0cdccba1eb62cad7d817f6dc58898b3ac15f36026fec
    d568bd4a706cb60bb37be6d8f1240661199d640b66fb0fe3b079f97f450b9ef9c22c6d5d
    dd379f0facd1cd020281dfa3c70ba21a3fa6fc2471dc6d13ecf8298b946f65345faa5ef0
    f1f8fff03ad0acb083725535636addb08d73dedb9832da198081e5deae84bfaf0409c22b
    e4a8aea2b607ec96931e6f4d4fe563ba

```
```

    $ keyctl update 268728824 "update pcrinfo=`cat pcr.blob`"
    $ keyctl print 268728824
    010100000000002c0002800093c35a09b70fff26e7a98ae786c641e678ec6ffb6b46d805
    77c8a6377aed9d3219c6dfec4b23ffe3000001005d37d472ac8a44023fbb3d18583a4f73
    d3a076c0858f6f1dcaa39ea0f119911ff03f5406df4f7f27f41da8d7194f45c9f4e00f2e
    df449f266253aa3f52e55c53de147773e00f0f9aca86c64d94c95382265968c354c5eab4
    9638c5ae99c89de1e0997242edfb0b501744e11ff9762dfd951cffd93227cc513384e7e6
    e782c29435c7ec2edafaa2f4c1fe6e7a781b59549ff5296371b42133777dcc5b8b971610
    94bc67ede19e43ddb9dc2baacad374a36feaf0314d700af0a65c164b7082401740e489c9
    7ef6a24defe4846104209bf0c3eced7fa1a672ed5b125fc9d8cd88b476a658a4434644ef
    df8ae9a178e9f83ba9f08d10fa47e4226b98b0702f06b3b8


```
可信密钥的首个使用者是 EVM，它在启动时需要一个高质量的对称密钥来为文件元数据提供 HMAC 保护。使用可信密钥可以有力保EVM 密钥未被用户层问题攻破，并且在封印到平台完整性状态后，可防御启动和离线攻击。使用上述可信密"kmk" 创建并保存一个加密密"evm"
```

    $ keyctl add encrypted evm "new trusted:kmk 32" @u
    159771175

```
```

    $ keyctl add encrypted evm "new default trusted:kmk 32" @u
    159771175

    $ keyctl print 159771175
    default trusted:kmk 32 2375725ad57798846a9bbd240de8906f006e66c03af53b1b3
    82dbbc55be2a44616e4959430436dc4f2a7a9659aa60bb4652aeb2120f149ed197c564e0
    24717c64 5972dcb82ab2dde83376d82b2e3c09ffc

    $ keyctl pipe 159771175 > evm.blob

```
```

    $ keyctl add encrypted evm "load `cat evm.blob`" @u
    831684262

    $ keyctl print 831684262
    default trusted:kmk 32 2375725ad57798846a9bbd240de8906f006e66c03af53b1b3
    82dbbc55be2a44616e4959430436dc4f2a7a9659aa60bb4652aeb2120f149ed197c564e0
    24717c64 5972dcb82ab2dde83376d82b2e3c09ffc

```
```

    $ evmkey=$(dd if=/dev/urandom bs=1 count=32 | xxd -c32 -p)
    $ keyctl add encrypted evm "new default user:kmk 32 $evmkey" @u
    794890253

    $ keyctl print 794890253
    default user:kmk 32 2375725ad57798846a9bbd240de8906f006e66c03af53b1b382d
    bbc55be2a44616e4959430436dc4f2a7a9659aa60bb4652aeb2120f149ed197c564e0247
    17c64 5972dcb82ab2dde83376d82b2e3c09ffc

```
可信密钥和加密密钥的其他用途（例如用于磁盘和文件加密）也在预期之中。特别是已定义新'ecryptfs' 格式，以便使用加密密钥挂eCryptfs 文件系统。关于用法的更多细节可参`Documentation/security/keys/ecryptfs.rst`
还定义了另一个新格式 'enc32'，以支持载荷大小32 字节的加密密钥。它最初将用于 nvdimm 安全，但可能会扩展到其他需32 字节载荷的用途
### TPM 2.0 ASN.1 密钥格式

TPM 2.0 ASN.1 密钥格式旨在即使在二进制形式下也易于识别（修复了 TPM 1.2 ASN.1 格式存在的问题），并且可扩展以支持可导入密钥等新内容，例如：

```

    TPMKey ::= SEQUENCE {
        type		OBJECT IDENTIFIER
        emptyAuth	[0] EXPLICIT BOOLEAN OPTIONAL
        parent		INTEGER
        pubkey		OCTET STRING
        privkey		OCTET STRING
    }

```
type 是即便在二进制形式下也能区分密钥的关键，因为 OID TCG 提供且唯一，因此在密钥偏移3 处形成可识别的二进制模式。当前已定义OID

```

    2.23.133.10.1.3 TPM Loadable key.  This is an asymmetric key (Usually
                    RSA2048 or Elliptic Curve) which can be imported by a
                    TPM2_Load() operation.

    2.23.133.10.1.4 TPM Importable Key.  This is an asymmetric key (Usually
                    RSA2048 or Elliptic Curve) which can be imported by a
                    TPM2_Import() operation.

    2.23.133.10.1.5 TPM Sealed Data.  This is a set of data (up to 128
                    bytes) which is sealed by the TPM.  It usually
                    represents a symmetric key and must be unsealed before
                    use.

```
可信密钥代码仅使TPM Sealed Data OID
emptyAuth true 表示密钥具有众所周知的授权""。如果为 false 或不出现，则密钥需要一个显式的授权短语。大多数用户空间使用者据此决定是否提示输入密码
parent 表示父密钥句柄，位于 0x81 MSO 空间中，例如 RSA 主存储密钥的 0x81000001。用户空间程序也支持0x40 MSO 空间中指定主密钥句柄。如果发生这种情况，则使TCG 定义模板的主密钥椭圆曲线变体将被即时生成到一个易失对象中并用作父密钥。当前内核代码仅支持 0x81 MSO 形式
pubkey TPM2B_PRIVATE 的二进制表示，不包含初始TPM2B 头，该头可由 ASN.1 八位字节字符串长度重建
privkey TPM2B_PUBLIC 的二进制表示，不包含初始TPM2B 头，该头可由 ASN.1 八位字节字符串长度重建
### DCP Blob 格式

   :doc: dcp blob format

   :identifiers: struct dcp_blob_fmt
