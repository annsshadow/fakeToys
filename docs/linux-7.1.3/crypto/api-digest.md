## 消息摘要

本页Linux 内核 Crypto API 中消息摘要（哈希）子系统的参考文档，按同步（shash）与异步（ahash）两类接口列出算法定义、分配释放、初始化、更新与最终摘要等函数，供内核密码学模块的开发与调用查阅



### 消息摘要算法定义


 :doc: 消息摘要算法定义

 :functions: hash_alg_common ahash_alg shash_alg

### 异步消息摘要API


 :doc: 异步消息摘要API

 :functions: crypto_alloc_ahash crypto_free_ahash crypto_ahash_init crypto_ahash_digestsize crypto_ahash_reqtfm crypto_ahash_reqsize crypto_ahash_statesize crypto_ahash_setkey crypto_ahash_finup crypto_ahash_final crypto_ahash_digest crypto_ahash_export crypto_ahash_import

### 异步哈希请求句柄


 :doc: 异步哈希请求句柄

 :functions: ahash_request_set_tfm ahash_request_alloc ahash_request_free ahash_request_set_callback ahash_request_set_crypt

### 同步消息摘要 API


 :doc: 同步消息摘要 API

 :functions: crypto_alloc_shash crypto_free_shash crypto_shash_blocksize crypto_shash_digestsize crypto_shash_descsize crypto_shash_setkey crypto_shash_digest crypto_shash_export crypto_shash_import crypto_shash_init crypto_shash_update crypto_shash_final crypto_shash_finup
