
## 分散列表加密 API


## 简介


Scatterlist Crypto API 以页向量（scatterlists）作为参数，并直接作用于
页。在某些情况下（例如 ECB 模式密码），这将允许页就地加密而无需任何
拷贝。

该设计最初的目标之一是便于支持 IPsec，从而可以对分页的 skb 直接施加
处理而无需线性化。


## 细节


最底层是算法，它们会向该 API 动态注册。

“转换”（Transforms）是用户实例化的对象，它们维护状态、处理所有实现
逻辑（例如操作页向量），并为底层算法提供一层抽象。但在用户层面，它们
非常简洁。

```

  [transform api]  (用户接口)
  [transform ops]  (按类型的逻辑粘合，例如 cipher.c、compress.c)
  [algorithm api]  (用于注册算法)

```
其思想是让用户界面和算法注册 API 都尽量简单，同时将核心逻辑对二者
隐藏。来自现有 API（如 Cryptoapi 和 Nettle）的许多优秀设计思想已被
借鉴于此。

该 API 目前支持五种主要的转换类型：AEAD（带关联数据的认证加密）、
分组密码（Block Ciphers）、密码（Ciphers）、压缩器（Compressors）和
哈希（Hashes）。

请注意，“分组密码”多少有些用词不当。它实际上旨在支持包括流密码在内
的所有密码。分组密码与密码的区别在于：后者恰好操作一个块，而前者可以
操作任意数量的数据，但需满足块大小要求（即非流密码只能处理块大小的整数
倍）。

```

	#include <crypto/hash.h>
	#include <linux/err.h>
	#include <linux/scatterlist.h>

	struct scatterlist sg[2];
	char result[128];
	struct crypto_ahash *tfm;
	struct ahash_request *req;

	tfm = crypto_alloc_ahash("md5", 0, CRYPTO_ALG_ASYNC);
	if (IS_ERR(tfm))
		fail();

	/* ... 设置 scatterlists ... */

	req = ahash_request_alloc(tfm, GFP_ATOMIC);
	if (!req)
		fail();

	ahash_request_set_callback(req, 0, NULL, NULL);
	ahash_request_set_crypt(req, sg, result, 2);

	if (crypto_ahash_digest(req))
		fail();

	ahash_request_free(req);
	crypto_free_ahash(tfm);


```
许多真实示例可在回归测试模块（tcrypt.c）中找到。


## 开发者须知


转换只能在用户上下文（user context）中分配，加密方法只能从 softirq 和
用户上下文中调用。对于带有 setkey 方法的转换，setkey 也应只在用户上下文
中调用。

使用该 API 处理密码时，若每个 scatterlist 包含的数据量是密码块大小
（通常为 8 字节）的整数倍，则性能最佳。这可以避免在非对齐的页片段边界
上进行任何拷贝。


## 新增算法


提交新算法以供合入时，一项强制性要求是至少包含来自已知来源（最好是
标准）的几个测试向量。

优先采用转换现有知名代码的方式，因为它更可能已被审阅并经广泛测试。若
提交来自 LGPL 来源的代码，请考虑将许可证改为 GPL（见 LGPL 第 3 节）。

提交的算法还必须大体上无专利问题（例如 IDEA 在 2011 年前后之前不会被
合入主线），并且应基于公认的标准和/或经过适当的同行评审。

同时请查阅可能相关的 RFC，以及通用的应用说明，例如 RFC2451
（“The ESP CBC-Mode Cipher Algorithms”）。

最好避免使用大量宏，改用内联函数，因为 gcc 对内联处理得很好，而过度
使用宏可能会在某些平台上导致编译问题。

也请查看下方网站上的 TODO 列表，了解别人可能已经在做的工作。


## Bug 报告


请将 bug 报告发送至：
    linux-crypto@vger.kernel.org

抄送：
    Herbert Xu <herbert@gondor.apana.org.au>,
    David S. Miller <davem@redhat.com>


## 更多信息


有关后续补丁和各种更新（包括当前的 TODO 列表），请见：
http://gondor.apana.org.au/~herbert/crypto/


## 作者


- James Morris
- David S. Miller
- Herbert Xu


## 致谢


以下人员在 API 的开发过程中提供了宝贵的反馈：

  - Alexey Kuznetzov
  - Rusty Russell
  - Herbert Valerio Riedel
  - Jeff Garzik
  - Michael Richardson
  - Andrew Morton
  - Ingo Oeser
  - Christoph Hellwig

本 API 的部分内容源自以下项目：

  Kerneli Cryptoapi (http://www.kerneli.org/)
   - Alexander Kjeldaas
   - Herbert Valerio Riedel
   - Kyle McMartin
   - Jean-Luc Cooke
   - David Bryson
   - Clemens Fruhwirth
   - Tobias Ringstrom
   - Harald Welte

以及：

  Nettle (https://www.lysator.liu.se/~nisse/nettle/)
   - Niels Möller

加密算法的原始开发者：

  - Dana L. How (DES)
  - Andrew Tridgell and Steve French (MD4)
  - Colin Plumb (MD5)
  - Steve Reid (SHA1)
  - Jean-Luc Cooke (SHA256, SHA384, SHA512)
  - Kazunori Miyazawa / USAGI (HMAC)
  - Matthew Skala (Twofish)
  - Dag Arne Osvik (Serpent)
  - Brian Gladman (AES)
  - Kartikey Mahendra Bhatt (CAST6)
  - Jon Oberheide (ARC4)
  - Jouni Malinen (Michael MIC)
  - NTT(Nippon Telegraph and Telephone Corporation) (Camellia)

SHA1 算法贡献者：
  - Jean-Francois Dive

DES 算法贡献者：
  - Raimar Falke
  - Gisle Sælensminde
  - Niels Möller

Blowfish 算法贡献者：
  - Herbert Valerio Riedel
  - Kyle McMartin

Twofish 算法贡献者：
  - Werner Koch
  - Marc Mutz

SHA256/384/512 算法贡献者：
  - Andrew McDonald
  - Kyle McMartin
  - Herbert Valerio Riedel

AES 算法贡献者：
  - Alexander Kjeldaas
  - Herbert Valerio Riedel
  - Kyle McMartin
  - Adam J. Richter
  - Fruhwirth Clemens (i586)
  - Linus Torvalds (i586)

CAST5 算法贡献者：
  - Kartikey Mahendra Bhatt (原始开发者未知，FSF 版权)。

TEA/XTEA 算法贡献者：
  - Aaron Grothe
  - Michael Ringe

Khazad 算法贡献者：
  - Aaron Grothe

Whirlpool 算法贡献者：
  - Aaron Grothe
  - Jean-Luc Cooke

Anubis 算法贡献者：
  - Aaron Grothe

Tiger 算法贡献者：
  - Aaron Grothe

VIA PadLock 贡献者：
  - Michal Ludvig

Camellia 算法贡献者：
  - NTT(Nippon Telegraph and Telephone Corporation) (Camellia)

通用 scatterwalk 代码由 Adam J. Richter <adam@yggdrasil.com> 编写

请将任何致谢更新或更正发送至：
Herbert Xu <herbert@gondor.apana.org.au>
