## 对称密钥密码

本文档介绍 Linux 内核加密子系统（Crypto API）中对称密钥分组密码（skcipher）的编程接口，涵盖算法对象分配、加解密请求句柄以及单分组密码等核心函数。面向在内核中开发或对接对称加密功能、需要查阅相关 API 原型的驱动与模块作者。



### 分组密码算法定义


   :doc: Block Cipher Algorithm Definitions

   :functions: crypto_alg cipher_alg compress_alg

### 对称密钥密码 API


   :doc: Symmetric Key Cipher API

   :functions: crypto_alloc_skcipher crypto_free_skcipher crypto_has_skcipher crypto_skcipher_ivsize crypto_skcipher_blocksize crypto_skcipher_setkey crypto_skcipher_reqtfm crypto_skcipher_encrypt crypto_skcipher_decrypt

### 对称密钥密码请求句柄


   :doc: Symmetric Key Cipher Request Handle

   :functions: crypto_skcipher_reqsize skcipher_request_set_tfm skcipher_request_alloc skcipher_request_free skcipher_request_set_callback skcipher_request_set_crypt

### 单一分组密码 API


   :doc: Single Block Cipher API

   :functions: crypto_alloc_cipher crypto_free_cipher crypto_has_cipher crypto_cipher_blocksize crypto_cipher_setkey crypto_cipher_encrypt_one crypto_cipher_decrypt_one
