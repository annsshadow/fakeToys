## 数字签名验证 API


:Author: Dmitry Kasatkin
:Date: 06.10.2011


   1. 简介
   2. API
   3. 用户空间工具


## 简介


数字签名验证 API 提供了一种验证数字签名的方法。目前数字签名由 IMA/EVM
完整性保护子系统使用。

数字签名验证通过使用 GnuPG 多精度整数（MPI）库的精简内核移植版实现。该内核
移植版提供了内存分配错误处理，已按照内核编码风格进行了重构，并修复了
checkpatch.pl 报告的错误与警告。

```

	struct pubkey_hdr {
		uint8_t		version;	/* key format version */
		time_t		timestamp;	/* key made, always 0 for now */
		uint8_t		algo;
		uint8_t		nmpi;
		char		mpi[0];
	} __packed;

	struct signature_hdr {
		uint8_t		version;	/* signature format version */
		time_t		timestamp;	/* signature made */
		uint8_t		algo;
		uint8_t		hash;
		uint8_t		keyid[8];
		uint8_t		nmpi;
		char		mpi[0];
	} __packed;

```
keyid 等于对密钥整体内容计算 SHA1[12-19] 的结果。
签名头被用作生成签名的输入。
这种方式确保了密钥或签名头无法被更改。
它保护时间戳不被修改，可用于回滚保护。

## API


```

	digsig_verify() - 使用公钥进行数字签名验证


	/**
	* digsig_verify() - 使用公钥进行数字签名验证
	* @keyring:	在其中搜索密钥的 keyring
	* @sig:	数字签名
	* @sigen:	签名长度
	* @data:	数据
	* @datalen:	数据长度
	* @return:	成功返回 0，否则返回 -EINVAL
	*
	* 针对数字签名验证数据完整性。
	* 目前仅支持 RSA。
	* 通常将内容的哈希作为该函数的数据使用。
	*
	*/
	int digsig_verify(struct key *keyring, const char *sig, int siglen,
			  const char *data, int datalen);

```
## 用户空间工具


用于签名与密钥管理的工具 evm-utils 提供了生成签名、将密钥加载到内核 keyring
的功能。密钥可以是 PEM 格式，也可以转换为内核格式。当密钥被加入内核 keyring
时，keyid 定义了密钥的名称：如下例中的 5D2B05FC633EE3E8。

```

	$ keyctl show
	Session Keyring
	-3 --alswrv      0     0  keyring: _ses
	603976250 --alswrv      0    -1   \_ keyring: _uid.0
	817777377 --alswrv      0     0       \_ user: kmk
	891974900 --alswrv      0     0       \_ encrypted: evm-key
	170323636 --alswrv      0     0       \_ keyring: _module
	548221616 --alswrv      0     0       \_ keyring: _ima
	128198054 --alswrv      0     0       \_ keyring: _evm

	$ keyctl list 128198054
	1 key in keyring:
	620789745: --alswrv     0     0 user: 5D2B05FC633EE3E8

```
