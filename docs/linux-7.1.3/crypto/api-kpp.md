## 瀵嗛挜鍗忓晢鍗忚鍘熻锛圞PP锛?
鏈〉鍒楀嚭 Linux 鍐呮牳鍔犲瘑瀛愮郴缁燂紙crypto锛変腑瀵嗛挜鍗忓晢鍗忚鍘熻锛圞PP锛夌浉鍏崇殑绠楁硶瀹氫箟銆丄PI 鎺ュ彛銆佽姹傚彞鏌勶紝浠ュ強 ECDH 涓?DH 鐨勮緟鍔╁嚱鏁般€?
### 瀵嗛挜鍗忓晢鍗忚鍘熻锛圞PP锛夊瘑鐮佺畻娉曞畾涔?
   :functions: kpp_request crypto_kpp kpp_alg kpp_secret

### 瀵嗛挜鍗忓晢鍗忚鍘熻锛圞PP锛夊瘑鐮?API

   :doc: Generic Key-agreement Protocol Primitives API

   :functions: crypto_alloc_kpp crypto_free_kpp crypto_kpp_set_secret crypto_kpp_generate_public_key crypto_kpp_compute_shared_secret crypto_kpp_maxsize

### 瀵嗛挜鍗忓晢鍗忚鍘熻锛圞PP锛夊瘑鐮佽姹傚彞鏌?
   :functions: kpp_request_alloc kpp_request_free kpp_request_set_callback kpp_request_set_input kpp_request_set_output

### ECDH 杈呭姪鍑芥暟

   :doc: ECDH Helper Functions

   :functions: ecdh crypto_ecdh_key_len crypto_ecdh_encode_key crypto_ecdh_decode_key

### DH 杈呭姪鍑芥暟

   :doc: DH Helper Functions

   :functions: dh crypto_dh_key_len crypto_dh_encode_key crypto_dh_decode_key
