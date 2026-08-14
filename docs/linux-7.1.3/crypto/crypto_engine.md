
## 加密引擎（Crypto Engine）


### 概述


加密引擎（CE）API 是一个加密请求队列管理器。

### 要求


你必须在你的转换上下文 your_tfm_ctx 的起始处放置结构体
crypto_engine：

```

	struct your_tfm_ctx {
		struct crypto_engine engine;
		...
	};

```
加密引擎只以 crypto_async_request 的形式管理异步请求。它无法知晓
底层请求类型，因此只能访问转换结构体。无法使用 container_of 访问
上下文。此外，引擎对你的结构体 "`struct your_tfm_ctx`" 一无所知。
引擎假定（要求）将已知的成员 `struct crypto_engine` 放在起始位置。

### 操作顺序


你需要通过 `crypto_engine_alloc_init()` 获取一个 struct crypto_engine。
通过 `crypto_engine_start()` 启动它。完成工作后，使用 `crypto_engine_stop()`
关闭引擎，并使用 `crypto_engine_exit()` 销毁引擎。

在传输任何请求之前，你必须通过提供以下函数来填充上下文 enginectx：

- `prepare_cipher_request`/`prepare_hash_request`：在每次对应的
  请求执行前被调用。如果需要某些处理或其它准备工作，在此处完成。

- `unprepare_cipher_request`/`unprepare_hash_request`：在每次
  请求处理后被调用。清理 / 撤销在 prepare 函数中完成的工作。

- `cipher_one_request`/`hash_one_request`：通过执行操作来处理当前请求。

注意，这些函数访问与收到的请求相关联的 crypto_async_request 结构体。
你可以通过如下方式取回原始请求：

```

	container_of(areq, struct yourrequesttype_request, base);

```
当你的驱动收到一个 crypto_request 时，你必须通过以下之一将其
传输给加密引擎：

- crypto_transfer_aead_request_to_engine()

- crypto_transfer_akcipher_request_to_engine()

- crypto_transfer_hash_request_to_engine()

- crypto_transfer_kpp_request_to_engine()

- crypto_transfer_skcipher_request_to_engine()

在请求处理结束时，需要调用以下函数之一：

- crypto_finalize_aead_request()

- crypto_finalize_akcipher_request()

- crypto_finalize_hash_request()

- crypto_finalize_kpp_request()

- crypto_finalize_skcipher_request()
