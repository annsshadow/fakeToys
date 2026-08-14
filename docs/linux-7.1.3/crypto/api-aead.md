## 带关联数据的认证加密（AEAD）


### 带关联数据的认证加密（AEAD）算法定义


   :doc: 带关联数据的认证加密（AEAD）Cipher API

   :functions: aead_request aead_alg

### 带关联数据的认证加密（AEAD）Cipher API


   :functions: crypto_alloc_aead crypto_free_aead crypto_aead_ivsize crypto_aead_authsize crypto_aead_blocksize crypto_aead_setkey crypto_aead_setauthsize crypto_aead_encrypt crypto_aead_decrypt

### 异步 AEAD 请求句柄


   :doc: 异步 AEAD 请求句柄

   :functions: crypto_aead_reqsize aead_request_set_tfm aead_request_alloc aead_request_free aead_request_set_callback aead_request_set_crypt aead_request_set_ad
