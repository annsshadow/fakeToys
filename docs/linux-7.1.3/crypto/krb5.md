
## Kerberos V 密码学 API


  - 概述（Overview）。
    - 小型缓冲区（Small Buffer）。
  - 编码类型（Encoding Type）。
  - 密钥派生（Key Derivation）。
    - PRF+ 计算。
    - Kc、Ke 和 Ki 派生。
  - 密码函数（Crypto Functions）。
    - 准备函数（Preparation Functions）。
    - 加密模式（Encryption Mode）。
    - 校验和模式（Checksum Mode）。
  - krb5enc AEAD 算法

## 概述


此 API 提供 Kerberos 5 风格的密码学，用于密钥派生、加密和校验和，可供网络文件系统使用，并可用于实现 GSSAPI 所需的底层加密。

```

	KRB5_ENCTYPE_AES128_CTS_HMAC_SHA1_96
	KRB5_ENCTYPE_AES256_CTS_HMAC_SHA1_96
	KRB5_ENCTYPE_AES128_CTS_HMAC_SHA256_128
	KRB5_ENCTYPE_AES256_CTS_HMAC_SHA384_192
	KRB5_ENCTYPE_CAMELLIA128_CTS_CMAC
	KRB5_ENCTYPE_CAMELLIA256_CTS_CMAC

	KRB5_CKSUMTYPE_HMAC_SHA1_96_AES128
	KRB5_CKSUMTYPE_HMAC_SHA1_96_AES256
	KRB5_CKSUMTYPE_CMAC_CAMELLIA128
	KRB5_CKSUMTYPE_CMAC_CAMELLIA256
	KRB5_CKSUMTYPE_HMAC_SHA256_128_AES128
	KRB5_CKSUMTYPE_HMAC_SHA384_192_AES256

```
```

	#include <crypto/krb5.h>

```
### 小型缓冲区（Small Buffer）


为了传递诸如密钥之类的小块数据，使用如下缓冲区结构
```

	struct krb5_buffer {
		unsigned int	len;
		void		*data;
	};

```
## 编码类型（Encoding Type）


```

	struct krb5_enctype {
		int		etype;
		int		ctype;
		const char	*name;
		u16		key_bytes;
		u16		key_len;
		u16		Kc_len;
		u16		Ke_len;
		u16		Ki_len;
		u16		prf_len;
		u16		block_len;
		u16		conf_len;
		u16		cksum_len;
		...
	};

```
API 使用者感兴趣的字段如下：

  - `etype` 和 `ctype` 分别指示该编码类型用于加密和校验和的协议编号。它们保存 `KRB5_ENCTYPE_**` 和 `KRB5_CKSUMTYPE_**` 常量。

  - `name` 是该编码的正式名称。

  - `key_len` 和 `key_bytes` 是输入密钥长度和派生密钥长度。（我认为它们只在 DES 时不同，而这里不支持 DES）。

  - `Kc_len`、`Ke_len` 和 `Ki_len` 是派生的 Kc、Ke 和 Ki 密钥的大小。Kc 用于校验和模式；Ke 和 Ki 用于加密模式。

  - `prf_len` 是 PRF+ 函数计算的结果大小。

  - `block_len`、`conf_len` 和 `cksum_len` 分别是加密块长度、混淆数（confounder）长度和校验和长度。三者都用于加密模式，但只有校验和长度用于校验和模式。

```

	const struct krb5_enctype *crypto_krb5_find_enctype(u32 enctype);

```
## 密钥派生（Key Derivation）


一旦应用程序选定了加密类型，就可以从传输密钥（transport key）派生出用于实际加密的密钥。

### PRF+ 计算


为了辅助密钥派生，提供一个函数来计算 Kerberos GSSAPI 的 PRF+
```

	int crypto_krb5_calc_PRFplus(const struct krb5_enctype *krb5,
				     const struct krb5_buffer *K,
				     unsigned int L,
				     const struct krb5_buffer *S,
				     struct krb5_buffer *result,
				     gfp_t gfp);

```
这可用于从源密钥加上额外的数据派生传输密钥，以限制其用途。

## 密码函数（Crypto Functions）


密钥派生完成后，就可以对数据执行加密操作。调用方在为传输准备消息时，必须在缓冲区中留出用于存放混淆数（如需要）和校验和的空隙。一个枚举
```

	enum krb5_crypto_mode {
		KRB5_CHECKSUM_MODE,
		KRB5_ENCRYPT_MODE,
	};

	size_t crypto_krb5_how_much_buffer(const struct krb5_enctype *krb5,
					   enum krb5_crypto_mode mode,
					   size_t data_size, size_t *_offset);

	size_t crypto_krb5_how_much_data(const struct krb5_enctype *krb5,
					 enum krb5_crypto_mode mode,
					 size_t *_buffer_size, size_t *_offset);

```
所有这些函数都接受编码类型以及加密模式的指示（仅校验和或完整加密）。

第一个函数返回容纳给定数据量所需的缓冲区大小；第二个函数返回特定大小的缓冲区能容纳多少数据，并相应地下调所需缓冲区的大小。在这两种情况下，还会返回数据在缓冲区中的偏移。

当收到一条消息时，数据的位置和大小由
```

	int crypto_krb5_where_is_the_data(const struct krb5_enctype *krb5,
					  enum krb5_crypto_mode mode,
					  size_t *_offset, size_t *_len);

```
调用方向函数提供消息的偏移和长度，函数随后修改这些值以指示包含数据的区域（加上任何填充）。有多少填充由调用方决定。如果长度太小，或者模式为
```

	int crypto_krb5_check_data_len(const struct krb5_enctype *krb5,
				       enum krb5_crypto_mode mode,
				       size_t len, size_t min_content);

```
则提供一个函数来仅做基本检查，确认解密/验证后的消息具有足够的最小有效载荷。

### 准备函数（Preparation Functions）


提供两个函数来分配并准备一个供使用的加密对象
```

	struct crypto_aead *
	crypto_krb5_prepare_encryption(const struct krb5_enctype *krb5,
				       const struct krb5_buffer *TK,
				       u32 usage, gfp_t gfp);
	struct crypto_shash *
	crypto_krb5_prepare_checksum(const struct krb5_enctype *krb5,
				     const struct krb5_buffer *TK,
				     u32 usage, gfp_t gfp);

```
这两个函数都接受编码类型、传输密钥以及用于派生相应子密钥的 usage 值。它们创建一个合适的加密对象——用于加密的 AEAD 模板和用于校验和的同步哈希——在其上设置密钥并进行配置。调用方应将这些句柄传递给下面的动作函数。

### 加密模式（Encryption Mode）


```

	ssize_t crypto_krb5_encrypt(const struct krb5_enctype *krb5,
				    struct crypto_aead *aead,
				    struct scatterlist *sg, unsigned int nr_sg,
				    size_t sg_len,
				    size_t data_offset, size_t data_len,
				    bool preconfounded);
	int crypto_krb5_decrypt(const struct krb5_enctype *krb5,
				struct crypto_aead *aead,
				struct scatterlist *sg, unsigned int nr_sg,
				size_t *_offset, size_t *_len);

```
在这两种情况下，输入和输出缓冲区由同一个 scatterlist 指示。

对于加密函数，输出缓冲区可能比所需更大（返回生成的输出量），并指出数据的位置和大小（必须与编码匹配）。如果未设置混淆数，函数会插入一个。

对于解密函数，提供缓冲区中消息的偏移和长度，这些值会被收缩以适应数据。解密函数会验证消息内的任何校验和，如果不匹配则报错。

### 校验和模式（Checksum Mode）


提供一对函数来生成消息的校验和并
```

	ssize_t crypto_krb5_get_mic(const struct krb5_enctype *krb5,
				    struct crypto_shash *shash,
				    const struct krb5_buffer *metadata,
				    struct scatterlist *sg, unsigned int nr_sg,
				    size_t sg_len,
				    size_t data_offset, size_t data_len);
	int crypto_krb5_verify_mic(const struct krb5_enctype *krb5,
				   struct crypto_shash *shash,
				   const struct krb5_buffer *metadata,
				   struct scatterlist *sg, unsigned int nr_sg,
				   size_t *_offset, size_t *_len);

```
在这两种情况下，输入和输出缓冲区由同一个 scatterlist 指示。可以传入额外的元数据，它会在数据之前被加入哈希。

对于 get_mic 函数，输出缓冲区可能比所需更大（返回生成的输出量），并指出数据的位置和大小（必须与编码匹配）。

对于验证函数，提供缓冲区中消息的偏移和长度，这些值会被收缩以适应数据。如果校验和不匹配，将返回错误。

## krb5enc AEAD 算法


提供了一个名为 “krb5enc” 的模板 AEAD 加密算法，它在加密明文之前先对明文做哈希（与 authenc 相反）。`crypto_krb5_prepare_encryption()` 返回的句柄可能是其中之一，但此 API 的使用者无需直接与之交互。

作为参考，其密钥格式以格式号的 BE32 开头。只提供格式 1，其后跟一个 Ke 密钥长度的 BE32，再跟一个 Ki 密钥长度的 BE32，然后是 Ke 密钥的字节，再是 Ki 密钥的字节。

使用特定顺序的字意味着静态测试数据不需要字节交换（byteswapping）。
