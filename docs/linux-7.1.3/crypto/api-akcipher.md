## 非对称密码（Asymmetric Cipher

本页面介绍内核非对称密码（akcipher）子系统的对API，涵盖算法定义、密钥设置、加解密操作与请求句柄的分配管理，供内核中需要调用公钥密码算法的模块参考


以下为内核非对称密码（akcipher）子系统对外提供API 文档

### 非对称密码算法定


   :functions: akcipher_alg akcipher_request

### 非对称密API


   :doc: Generic Public Key Cipher API

   :functions: crypto_alloc_akcipher crypto_free_akcipher crypto_akcipher_set_pub_key crypto_akcipher_set_priv_key crypto_akcipher_maxsize crypto_akcipher_encrypt crypto_akcipher_decrypt

### 非对称密码请求句


   :functions: akcipher_request_alloc akcipher_request_free akcipher_request_set_callback akcipher_request_set_crypt
