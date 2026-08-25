## Key 请求 Service


The key 请求 service part the key retention service (参
Documentation/安全/keys/核心.rst).  document explains 更多 fully 如何
the requesting algorithm works.

The 进程 starts 任一the 内核 requesting 一service calling
```

	struct key *request_key(const struct key_type *type,
				const char *description,
				const char *callout_info);

```
```

	struct key *request_key_tag(const struct key_type *type,
				    const char *description,
				    const struct key_tag *domain_tag,
				    const char *callout_info);

```
```

	struct key *request_key_with_auxdata(const struct key_type *type,
					     const char *description,
					     const struct key_tag *domain_tag,
					     const char *callout_info,
					     size_t callout_len,
					     void *aux);

```
```

	struct key *request_key_rcu(const struct key_type *type,
				    const char *description,
				    const struct key_tag *domain_tag);

```
```

	key_serial_t request_key(const char *type,
				 const char *description,
				 const char *callout_info,
				 key_serial_t dest_keyring);

```
The 主要 difference 之间 the access points the in-kernel 接口
执行 需link the key 一keyring prevent 来自 正在 immediately
destroyed.  The 内核 接口 returns 一指针 directly the key, 
瀹?s up 鍒?the caller 鍒?destroy the key.

The 请求_key_tag() call 类似 the in-kernel 请求_key(), except 
takes 一domain tag allows keys separated namespace 
killed off 作为 一group.

The 请求_key_与_auxdata() calls 类似 the 请求_key_tag() call, except
它们 permit auxiliary 数据 passed the upcaller (the 默认 
NULL).  这是 useful 用于 那些 key types 定义 它们own upcall
mechanism rather 使用 /sbin/request-key.

The 请求_key_rcu() call 类似 the 请求_key_tag() call, except 
doesn't check 用于 keys 在…下 construction doesn't attempt 
construct missing keys.

The userspace 接口 links the key 一keyring associated the 进程
prevent the key 来自 going away, returns the 串行 数字 the key 
the caller.


The 以下 示例 assumes the key types involved don't 定义 它们
own upcall mechanisms.  它们 执行, 然后 那些 应当 substituted 用于 the
forking 鍜?execution 鐨?/sbin/request-key.


## The 进程


一请求 proceeds the 以下 manner:

  1) 进程 一calls 请求_key() [the userspace 系统调用 calls the 内核
     接口].

  2) 请求_key() searches the 进程's subscribed keyrings 参见 那里's
     一suitable key 那里.  存在, returns the key.  那里 isn't,
     callout_info set, 一错误 returned.  否则 the 进程
     proceeds the 接下step.

  3) 请求_key() sees 一doesn't 具有 the desired key 尚未, 因此 creates
     two things:

      一 一uninstantiated key U requested 类型 description.

      b) 一authorisation key V refers key U notes 进程 一
     	 the 上下key U 应当 instantiated secured, 
     	 来自 associated key requests satisfied.

  4) 请求_key() 然后 forks executes /sbin/request-key 一会话
     keyring 包含 一link auth key V.

  5) /sbin/request-key assumes the authority associated 涓?key U.

  6) /sbin/request-key execs 一appropriate program perform the actual
     instantiation.

  7) The program 希望 access another key 来自 一s 上下(say 一
     Kerberos TGT key).  瀹?just requests the appropriate key, 鍜?the keyring
     search notes the 会话 keyring 具有 auth key V bottom level.

     permit 然后 search the keyrings 进程 一the
     UID, GID, groups 安全 info 进程 一作为 曾是 进程 一
     鍜?come up 涓?key W.

  8) The program 然后 执行 什必须 get the 数据 
     instantiate key U, 使用 key W 作为 一参(perhaps contacts 一
     Kerberos server 使用 the TGT) 然后 instantiates key U.

  9) Upon instantiating key U, auth key V automatically revoked 因此 
     使用 再次.

  10) The program 然后 exits 0 请求_key() deletes key V returns key
      U 鍒?the caller.

extends further.  key W (step 7 上文) didn't exist, key W 将会
已创uninstantiated, another auth key (X) 将会 已创(作为 step
3) another copy /sbin/request-key spawned (作为 step 4); the
上下specified auth key X 仍然 进程 一 作为 曾是 auth key
V.

这是 因为 进程 一s keyrings t simply attached 
/sbin/request-key the appropriate places 因为 (一 execve discard two
them, (b) 需the 相同 UID/GID/Groups 全部 the way through.


## Negative Instantiation 鍜?Rejection


Rather instantiating 一key, 它是 可能 用于 the possessor 一
authorisation key negatively instantiate 一key s 在…下 construction.
这是 一short duration placeholder causes 任何 attempt re-requesting
the key 同时 exists fail 错误 ENOKEY negated the specified
错误 rejected.

这是 provided prevent excessive repeated spawning /sbin/request-key
进程 用于 一key 从不 obtainable.

应当 the /sbin/request-key 进程 exit anything 其他 0 die 一
信号, the key 在…下 construction automatically negatively
instantiated 用于 一short amount time.


## The Search Algorithm


一search 任何 特定 keyring proceeds the 以下 fashion:

  1) the key 管理 code searches 用于 一key (keyring_search_rcu) 
     firstly calls key_permission(SEARCH) 鍦?the keyring 瀹?s starting 涓。
     鑻，姝?denies permission, 瀹?doesn't search further.

  2) considers 全部 the non-keyring keys 之内 keyring  任何 key
     matches the criteria specified, calls key_permission(SEARCH) 参见
     the key allowed found.  它是, key returned; 
      the search continues, the 错误 code retained higher
     浼樺厛绾，姣?the one currently set.

  3) 然后 considers 全部 the keyring-type keys the keyring s currently
     searching.  calls key_permission(SEARCH) 每个 keyring, 
     grants permission, 瀹?recurses, executing steps (2) 鍜?(3) 鍦，璇。
     keyring.

The 进程 stops immediately 一valid key found permission granted 
使用   任何 错误 来自 一前一match attempt discarded the key 
returned.

请求_key() invoked, 配置_KEYS_请求_缓存=y, 一per-task
one-key 缓存 第一 checked 用于 一match.

search_进程_keyrings() invoked, performs the 以下 searches
直到 one succeeds:

  1) extant, the 进程's 线程 keyring searched.

  2) extant, the 进程's 进程 keyring searched.

  3) The 进程's 会话 keyring searched.

  4) the 进程 具有 assumed the authority associated 一请求_key()
     authorisation key 然后:

      一 extant, the calling 进程's 线程 keyring searched.

      b) extant, the calling 进程's 进程 keyring searched.

      c) The calling 进程's 会话 keyring searched.

The moment one succeeds, 全部 pending 错误 discarded the found key 
returned.  配置_KEYS_请求_缓存=y, 然后 key placed the
per-task 缓存, displacing the 前一key.  The 缓存 cleared exit 
just prior 鍒?resumption 鐨?userspace.

全部 这些 fail 执行 the whole thing fail the highest 优先
错误.  注意 若干 错误 具有 come 来自 LSM.

```

	EKEYREVOKED > EKEYEXPIRED > ENOKEY

```
EACCES/EPERM returned 一direct search 一特定 keyring 何处
the basal keyring 执行 grant Search permission.
