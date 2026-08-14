
## 非对称 / 公钥加密密钥类型


  - 概述。
  - 密钥标识。
  - 访问非对称密钥。
    - 签名验证。
  - 非对称密钥子类型。
  - 实例化数据解析器。
  - 密钥环链接限制。


## 概述


“asymmetric”（非对称）密钥类型被设计为公钥加密所用密钥的容器，而不对加密的形式或机制、
或密钥的形式施加任何特定限制。

非对称密钥会被赋予一个子类型，该子类型定义了与密钥关联的数据种类，并提供用于描述和销毁
它的操作。不过，并不要求密钥数据实际存储在密钥中。

可以定义一个完全位于内核内部的密钥保留与操作子类型，但也可以提供对加密硬件（例如 TPM）的
访问，该硬件既可用于保留相关密钥，也可使用该密钥执行操作。在这种情况下，非对称密钥仅仅是
TPM 驱动的一个接口。

同时还提供了数据解析器（data parser）的概念。数据解析器负责从传递给实例化函数的数据块
（blob）中提取信息。第一个识别出该数据块的解析器负责设置密钥的子类型，并定义可在该密钥上
执行的操作。

数据解析器可以将数据块解释为包含代表密钥的比特，也可以将其解释为对系统其他地方（例如 TPM）
所保存密钥的引用。


## 密钥标识


如果以空名称添加一个密钥，则实例化数据解析器会有机会预先解析密钥，并根据密钥内容确定应为
该密钥赋予的描述。

随后便可用于引用该密钥，既可以通过完全匹配，也可以通过部分匹配。密钥类型也可能使用其他
判据来引用密钥。

非对称密钥类型的 match（匹配）函数可以执行比直接将描述与判据字符串比较更广泛的一系列比较：

  1) 如果判据字符串形如 “id:<hexdigits>”，则 match 函数会检查密钥的指纹，看给定的十六进制
     数字是否匹配

```
	keyctl search @s asymmetric id:5acc2142

     will match a key with fingerprint::

	1A00 2040 7601 7889 DE11  882C 3823 04AD 5ACC 2142

  2) 如果判据字符串形如 “<subtype>:<hexdigits>”，则匹配方式与 (1) 相同，但附加限制为只匹配
     指定子类型（例如 tpm）的密钥。例如::

	keyctl search @s asymmetric tpm:5acc2142

```
在 /proc/keys 中，密钥指纹的最后 8 个十六进制数字为

```
	1a39e171 I-----     1 perm 3f010000     0     0 asymmetric modsign.0: DSA 5acc2142 []

```


## 访问非对称密钥


要从内核内部对非对称密钥进行一般访问，可使用以下

```

	#include <crypto/public_key.h>

```
借此可以访问用于处理非对称 / 公钥的函数。其中定义了三个用于表示公钥加密的枚举

```

	enum pkey_algo

```

```

	enum pkey_hash_algo

```

```

	enum pkey_id_type

```
请注意，之所以需要密钥类型表示类型，是因为来自不同标准的密钥标识符并不一定是兼容的。例如，
PGP 通过对密钥数据加上一些 PGP 特有的元数据做哈希来生成密钥标识符，而 X.509 则使用任意的
证书标识符。

在密钥上定义的操作有：

  1) 签名验证。

其他操作（例如加密）使用与验证相同的密钥数据也是可能的，但目前不受支持；而另外一些操作
（例如解密和签名生成）则需要额外的密钥数据。


### 签名验证


提供了一个用于执行加密签名验证的操作，使用

```

	int verify_signature(const struct key *key,
			     const struct public_key_signature *sig);

```
调用者必须已经从某个来源获取了密钥，然后可以用它来检查签名。调用者必须已经解析了签名

```

	struct public_key_signature {
		u8 *digest;
		u8 digest_size;
		enum pkey_hash_algo pkey_hash_algo : 8;
		u8 nr_mpi;
		union {
			MPI mpi[2];
			...
		};
	};

```
所使用的算法必须记录于 sig->pkey_hash_algo，组成实际签名的所有 MPI 必须存储在 sig->mpi[] 中，
MPI 的数量置于 sig->nr_mpi。

此外，调用者必须已经对数据做了摘要（digest），得到的哈希必须由 sig->digest 指向，哈希的大小
置于 sig->digest_size。

函数成功时返回 0，若签名不匹配则返回 -EKEYREJECTED。

函数也可能返回 -ENOTSUPP（如果指定了不支持的公钥算法或公钥/哈希算法组合，或者密钥不支持该
操作）；-EBADMSG 或 -ERANGE（如果某些参数含有异常数据）；或者 -ENOMEM（如果无法完成分配）。
如果 key 参数类型错误或未完整初始化，可能返回 -EINVAL。


## 非对称密钥子类型


非对称密钥拥有一个子类型，该子类型定义了可在该密钥上执行的操作集合，并决定了作为密钥载荷
附带的数据。载荷的格式完全由子类型决定。

子类型由密钥数据解析器选择，解析器必须初始化它所需的数据。非对称密钥持有对子类型模块的
引用。

```

	#include <keys/asymmetric-subtype.h>

```

```

	struct asymmetric_key_subtype {
		struct module		*owner;
		const char		*name;

		void (*describe)(const struct key *key, struct seq_file *m);
		void (*destroy)(void *payload);
		int (*query)(const struct kernel_pkey_params *params,
			     struct kernel_pkey_query *info);
		int (*eds_op)(struct kernel_pkey_params *params,
			      const void *in, void *out);
		int (*verify_signature)(const struct key *key,
					const struct public_key_signature *sig);
	};

```
非对称密钥通过它们的 payload[asym_subtype] 成员指向此结构。

owner 和 name 字段应分别设置为所属模块和子类型的名称。目前，name 仅用于打印语句。

子类型定义了一系列操作：

  1) describe()。

     必需。这允许子类型在 /proc/keys 中针对该密钥显示一些内容。例如可以显示公钥算法类型的
     名称。此后，密钥类型会显示密钥标识字符串的尾部。

  2) destroy()。

     必需。这应释放与密钥关联的内存。非对称密钥会负责释放指纹并释放对子类型模块的引用。

  3) query()。

     必需。这是一个用于查询密钥能力的函数。

  4) eds_op()。

     可选。这是加密、解密和签名创建操作的入口点（这些操作由参数结构体中的操作 ID 区分）。
     子类型可以实现任何它喜欢的方式来执行操作，包括卸载到硬件。

  5) verify_signature()。

     可选。这是签名验证的入口点。子类型可以实现任何它喜欢的方式来执行操作，包括卸载到硬件。

## 实例化数据解析器


非对称密钥类型通常不想存储或处理持有密钥数据的原始数据块。否则它每次想使用时都必须解析并
做错误检查。此外，数据块的内容可能包含可对其执行的各种检查（例如自签名、有效期），并可能
包含关于密钥的有用数据（标识符、能力）。

另外，数据块也可能表示一个指向包含密钥的硬件的指针，而非密钥本身。

可以为以下数据块格式实现解析器：

 - OpenPGP 包流 [RFC 4880]。
 - X.509 ASN.1 流。
 - 指向 TPM 密钥的指针。
 - 指向 UEFI 密钥的指针。
 - PKCS#8 私钥 [RFC 5208]。
 - PKCS#5 加密私钥 [RFC 2898]。

在密钥实例化期间，会依次尝试列表中的每个解析器，直到有一个不返回 -EBADMSG 为止。

```

	#include <keys/asymmetric-parser.h>

```

```

	struct asymmetric_key_parser {
		struct module	*owner;
		const char	*name;

		int (*parse)(struct key_preparsed_payload *prep);
	};

```
owner 和 name 字段应分别设置为所属模块和解析器的名称。

解析器目前只定义了一个操作，且它是必需的：

  1) parse()。

     在密钥创建和更新路径中会调用它来预解析密钥。特别地，它会在分配密钥_之前_于密钥创建期间
     被调用，因此，在调用者不愿提供时，它也可以提供密钥的描述。

     调用者传入一个指向如下结构体的指针，其中除 data、datalen 和 quotalen 之外的所有字段均
     已被清零 [见

```

	struct key_preparsed_payload {
		char		*description;
		void		*payload[4];
		const void	*data;
		size_t		datalen;
		size_t		quotalen;
	};

     实例化数据位于由 data 指向、大小为 datalen 的一个数据块中。parse() 函数绝对不允许改变
     这两个值，也不应改变任何其他值，_除非_它们识别了该数据块格式且不会返回 -EBADMSG 以表明
     这不是它们的格式。

     如果解析器对该数据块满意，它应为密钥建议一个描述并附加到 ->description；->payload[asym_subtype]
     应设置为指向要使用的子类型；->payload[asym_crypto] 应设置为指向该子类型的已初始化数据；
     ->payload[asym_key_ids] 应指向一个或多个十六进制指纹；quotalen 应更新以表明该密钥应占用的
     配额。

     在清理时，附加到 ->payload[asym_key_ids] 和 ->description 的数据会被 kfree()，附加到
     ->payload[asm_crypto] 的数据会被传递给子类型的 ->destroy() 方法以进行释放。指向
     ->payload[asym_subtype] 的子类型的模块引用会被 put。


     如果数据格式无法识别，应返回 -EBADMSG。如果可以识别，但密钥由于某种原因无法建立，则应返回
     其他某个负的错误码。成功时应返回 0。

     可以对密钥的指纹字符串进行部分匹配。对于 RSA 和 DSA 这类公钥算法，这通常将是密钥指纹的
     可打印十六进制形式。

```

```

	int register_asymmetric_key_parser(struct asymmetric_key_parser *parser);
	void unregister_asymmetric_key_parser(struct asymmetric_key_parser *subtype);

```
解析器不得拥有相同的名称。名称在其他情况下仅用于在调试消息中显示。


## 密钥环链接限制


由用户空间使用 add_key 创建的密钥环可以被配置为检查被链接密钥的签名。没有有效签名的密钥
不允许被链接。

有几种可用的限制方法：

  1) 使用内核内建可信密钥环进行限制

     - 与 KEYCTL_RESTRICT_KEYRING 一起使用的选项字符串：
       - "builtin_trusted"

     将在内核内建可信密钥环中搜索签名密钥。如果未配置内建可信密钥环，则所有链接都将被拒绝。
     ca_keys 内核参数也会影响用于签名验证的密钥。

  2) 使用内核内建与次要可信密钥环进行限制

     - 与 KEYCTL_RESTRICT_KEYRING 一起使用的选项字符串：
       - "builtin_and_secondary_trusted"

     将在内核内建与次要可信密钥环中搜索签名密钥。如果未配置次要可信密钥环，此限制的行为将
     类似于 "builtin_trusted" 选项。ca_keys 内核参数也会影响用于签名验证的密钥。

  3) 使用单独的密钥或密钥环进行限制

     - 与 KEYCTL_RESTRICT_KEYRING 一起使用的选项字符串：
       - "key_or_keyring:<密钥或密钥环的序列号>[:chain]"

     每当请求链接一个密钥时，仅当被链接的密钥由某个指定密钥签名时，链接才会成功。该密钥可以
     通过提供一个非对称密钥的序列号直接指定，也可以通过提供某个密钥环的序列号来在其中搜索
     签名密钥。

     当在字符串末尾提供 "chain" 选项时，目标密钥环内的密钥也将被搜索作为签名密钥。这样就可以通过
     按序（从最靠近根的证书开始）将每个证书添加到一个密钥环来验证证书链。例如，可以先用指向
     一组根证书的链接填充一个密钥环，并为每个

```

	# 为根证书创建并填充一个密钥环
	root_id=`keyctl add keyring root-certs "" @s`
	keyctl padd asymmetric "" $root_id < root1.cert
	keyctl padd asymmetric "" $root_id < root2.cert

	# 为证书链创建并限制一个密钥环
	chain_id=`keyctl add keyring chain "" @s`
	keyctl restrict_keyring $chain_id asymmetric key_or_keyring:$root_id:chain

	# 尝试添加链中的每个证书，从最靠近根的证书开始
	keyctl padd asymmetric "" $chain_id < intermediateA.cert
	keyctl padd asymmetric "" $chain_id < intermediateB.cert
	keyctl padd asymmetric "" $chain_id < end-entity.cert

     如果最终的终端实体（end-entity）证书被成功添加到 "chain" 密钥环，我们就可以确定它存在一条
     回溯到某个根证书的有效签名链。

     也可以使用单个密钥环，在链接根证书之后对密钥环进行限制来验证签名链::

	# 为证书链创建密钥环并添加根证书
	chain2_id=`keyctl add keyring chain2 "" @s`
	keyctl padd asymmetric "" $chain2_id < root1.cert

	# 对已链接 root1.cert 的密钥环进行限制。该证书将保留在密钥环的链接中。
	keyctl restrict_keyring $chain2_id asymmetric key_or_keyring:0:chain

	# 尝试添加链中的每个证书，从最靠近根的证书开始
	keyctl padd asymmetric "" $chain2_id < intermediateA.cert
	keyctl padd asymmetric "" $chain2_id < intermediateB.cert
	keyctl padd asymmetric "" $chain2_id < end-entity.cert

     如果最终的终端实体证书被成功添加到 "chain2" 密钥环，我们就可以确定存在一条回溯到在密钥环
     被限制之前添加的根证书的有效签名链。

```

在所有这些情况下，如果找到了签名密钥，则会使用该签名密钥验证待链接密钥的签名。仅当签名被
成功验证时，请求的密钥才会被添加到密钥环。如果找不到父证书，返回 -ENOKEY；如果签名检查失败
或密钥被列入黑名单，返回 -EKEYREJECTED。如果无法执行签名检查，可能返回其他错误。
