## SipHash —— 一种短输入 PRF


:Author: Written by Jason A. Donenfeld <jason@zx2c4.com>

SipHash 是一种加密安全的 PRF（带密钥的哈希函数），针对短输入表现极佳，
由此得名。它由密码学家 Daniel J. Bernstein 和 Jean-Philippe Aumasson 设计，
旨在替代某些场景下对 `jhash`、`md5_transform`、`sha1_transform` 等函数的使用。

SipHash 接受一个由随机生成的数字填充而成的密钥，以及输入缓冲区或若干输入整数，
并输出一个与随机数无法区分的整数。你可以将该整数用作安全序列号、安全
cookie 的一部分，或经掩码处理后用于哈希表。

## 生成密钥


密钥应当始终由加密安全的来源生成
```

	siphash_key_t key;
	get_random_bytes(&key, sizeof(key));

```
如果你不是从这里派生密钥，那就做错了。

## 使用这些函数


该函数的变体有两种：一种接受整数列表，另一种
```

	u64 siphash(const void *data, size_t len, const siphash_key_t *key);

```
```

	u64 siphash_1u64(u64, const siphash_key_t *key);
	u64 siphash_2u64(u64, u64, const siphash_key_t *key);
	u64 siphash_3u64(u64, u64, u64, const siphash_key_t *key);
	u64 siphash_4u64(u64, u64, u64, u64, const siphash_key_t *key);
	u64 siphash_1u32(u32, const siphash_key_t *key);
	u64 siphash_2u32(u32, u32, const siphash_key_t *key);
	u64 siphash_3u32(u32, u32, u32, const siphash_key_t *key);
	u64 siphash_4u32(u32, u32, u32, u32, const siphash_key_t *key);

```
如果你向通用 siphash 函数传入长度恒定的内容，编译器会在编译期进行常量折叠，
并自动选择其中一个经过优化的函数。


```

	struct some_hashtable {
		DECLARE_HASHTABLE(hashtable, 8);
		siphash_key_t key;
	};

	void init_hashtable(struct some_hashtable *table)
	{
		get_random_bytes(&table->key, sizeof(table->key));
	}

	static inline hlist_head *some_hashtable_bucket(struct some_hashtable *table, struct interesting_input *input)
	{
		return &table->hashtable[siphash(input, sizeof(*input), &table->key) & (HASH_SIZE(table->hashtable) - 1)];
	}

```
然后你可以像往常一样遍历返回的哈希桶。

## 安全性


SipHash 具有极高的安全裕度，其密钥为 128 位。只要密钥保持机密，攻击者就不可能
猜出函数的输出，即便能够观察到大量输出，因为 2^128 种输出是相当可观的。

Linux 实现了 SipHash 的 "2-4" 变体。

## 结构体传递陷阱


很多时候 XuY 系列函数容量不足，此时你会希望向 siphash 传入一个预先填充好的
结构体。这样做时，务必确保结构体中没有填充空洞。最简单的方法是：按大小
降序排列结构体的成员，并在获取大小时使用 offsetofend() 而非 sizeof()。出于
性能考虑，如果可能的话，将结构体进行对齐是个不错的做法
```

	const struct {
		struct in6_addr saddr;
		u32 counter;
		u16 dport;
	} __aligned(SIPHASH_ALIGNMENT) combined = {
		.saddr = *(struct in6_addr *)saddr,
		.counter = counter,
		.dport = dport
	};
	u64 h = siphash(&combined, offsetofend(typeof(combined), dport), &secret);

```
## 资源


如果你有兴趣深入了解，请阅读 SipHash 论文：
https://131002.net/siphash/siphash.pdf

### 

## HalfSipHash —— SipHash 不安全的小表弟


:Author: Written by Jason A. Donenfeld <jason@zx2c4.com>

万一 SipHash 的速度无法满足你的需求，你可能能够找到使用 HalfSipHash 的理由——
一种令人不安但或许有用的可能。HalfSipHash 将 SipHash 的轮数从 "2-4" 削减到
"1-3"，更可怕的是，它使用容易被暴力破解的 64 位密钥（输出为 32 位），而非
SipHash 的 128 位密钥。不过，这可能对某些高性能的 `jhash` 用户有吸引力。

HalfSipHash 的支持通过 "hsiphash" 系列函数提供。

   切勿将 hsiphash 函数用于除哈希表键函数以外的任何用途，且只有在你能够绝对
   确定其输出永远不会传出内核时才可使用。相比 `jhash`，它仅作为缓解哈希表
   泛洪拒绝服务攻击的一种手段才具有有限的价值。

在 64 位内核中，hsiphash 函数实际上实现的是 SipHash-1-3（SipHash 的缩减轮数
变体），而非 HalfSipHash-1-3。这是因为在 64 位代码中，SipHash-1-3 并不比
HalfSipHash-1-3 慢，甚至可能更快。注意，这**并不**意味着在 64 位内核中
hsiphash 函数与 siphash 函数相同，或意味着它们是安全的；hsiphash 函数仍使用
安全性较低的缩减轮数算法，并将其输出截断为 32 位。

## 生成 hsiphash 密钥


密钥应当始终由加密安全的来源生成
```

	hsiphash_key_t key;
	get_random_bytes(&key, sizeof(key));

```
如果你不是从这里派生密钥，那就做错了。

## 使用 hsiphash 函数


该函数的变体有两种：一种接受整数列表，另一种
```

	u32 hsiphash(const void *data, size_t len, const hsiphash_key_t *key);

```
```

	u32 hsiphash_1u32(u32, const hsiphash_key_t *key);
	u32 hsiphash_2u32(u32, u32, const hsiphash_key_t *key);
	u32 hsiphash_3u32(u32, u32, u32, const hsiphash_key_t *key);
	u32 hsiphash_4u32(u32, u32, u32, u32, const hsiphash_key_t *key);

```
如果你向通用 hsiphash 函数传入长度恒定的内容，编译器会在编译期进行常量折叠，
并自动选择其中一个经过优化的函数。

## 哈希表键函数用法



```

	struct some_hashtable {
		DECLARE_HASHTABLE(hashtable, 8);
		hsiphash_key_t key;
	};

	void init_hashtable(struct some_hashtable *table)
	{
		get_random_bytes(&table->key, sizeof(table->key));
	}

	static inline hlist_head *some_hashtable_bucket(struct some_hashtable *table, struct interesting_input *input)
	{
		return &table->hashtable[hsiphash(input, sizeof(*input), &table->key) & (HASH_SIZE(table->hashtable) - 1)];
	}

```
然后你可以像往常一样遍历返回的哈希桶。

## 性能


hsiphash() 大约比 jhash() 慢 3 倍。对于许多替代场景而言，这不会成为问题，因为
哈希表查找并非瓶颈。总体而言，为了 hsiphash() 的安全性和抗 DoS 能力而做出这种
牺牲或许是值得的。

