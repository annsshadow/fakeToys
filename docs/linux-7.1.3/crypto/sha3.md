
## SHA-3 算法集合（SHA-3 Algorithm Collection


## 概述

SHA-3 系列算法NIST FIPS-202 [^1^]_ 规范定义，包含基Keccak 海绵（sponge）函数的六种算法。它们之间的差异在于

- "rate"（速率，即每次调用 Keccak 函数时被新数据更新的状态缓冲区大小，类似于"块大）；
- 追加到输入数据之后的域分离后缀（domain separation suffix）；
- 以及从末尾提取的输出数据量

Keccak 海绵函数被设计为可对任意长度的输出进行提取（这正是部分算法所需要的）

提供四种摘要算法

- SHA3-224
- SHA3-256
- SHA3-384
- SHA3-512

此外，还提供两种可扩展输出函数（XOF）：

- SHAKE128
- SHAKE256

SHA-3 API 支持上述六种算法。其中四种摘要算法支`crypto_shash` `crypto_ahash` 两类 API

本文档描SHA-3 API


## Digests（摘要）

```
	void sha3_224(const u8 *in, size_t in_len, u8 out[SHA3_224_DIGEST_SIZE]);
	void sha3_256(const u8 *in, size_t in_len, u8 out[SHA3_256_DIGEST_SIZE]);
	void sha3_384(const u8 *in, size_t in_len, u8 out[SHA3_384_DIGEST_SIZE]);
	void sha3_512(const u8 *in, size_t in_len, u8 out[SHA3_512_DIGEST_SIZE]);
```

如果用户需要以增量（incremental）方式传入数据，可使用增API

```
	struct sha3_ctx { ... };
```

```
	void sha3_224_init(struct sha3_ctx *ctx);
	void sha3_256_init(struct sha3_ctx *ctx);
	void sha3_384_init(struct sha3_ctx *ctx);
	void sha3_512_init(struct sha3_ctx *ctx);
```

```
	void sha3_update(struct sha3_ctx *ctx, const u8 *in, size_t in_len);
```

```
	void sha3_final(struct sha3_ctx *ctx, u8 *out);
```

`sha3_final` 会清零（zeroize）上下文。摘要长度由所调用的初始化函数决定


## Extendable-Output 函数（可扩展输出函数

```
	void shake128(const u8 *in, size_t in_len, u8 *out, size_t out_len);
	void shake256(const u8 *in, size_t in_len, u8 *out, size_t out_len);
```

如果用户需要以增量方式提供输入数据 / 接收输出数据，可使用增量 API

```
	struct shake_ctx { ... };
```

```
	void shake128_init(struct shake_ctx *ctx);
	void shake256_init(struct shake_ctx *ctx);
```

```
	void shake_update(struct shake_ctx *ctx, const u8 *in, size_t in_len);
```

```
	void shake_squeeze(struct shake_ctx *ctx, u8 *out, size_t out_len);
```

`shake_squeeze` 通过告知要提取的数据量来工作。注意：执行多次 squeeze 时，输出会连续地排布在缓冲区中，这与在单个缓冲区上执行一次、提取相同总量的单squeeze 得到的结果完全相同。一旦开squeeze，就不能再追加更多输入数据

```
	void shake_zeroize_ctx(struct shake_ctx *ctx);
```


## Testing（测试）

测试 SHA-3 代码，可使用 `sha3_kunit`（对应配置项 `CONFIG_CRYPTO_LIB_SHA3_KUNIT_TEST`）

由于 SHA-3 算法已获 FIPS 批准，当内核FIPS 模式启动时，SHA-3 库会执行一次简单的自检测（self-test），这纯粹是为了满足 FIPS 合规要求。常规测试则由内核开发者与集成者使用更为全面的 KUnit 测试套件来完成


## References（参考资料）


## API 函数 参
