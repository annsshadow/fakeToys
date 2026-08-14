## Key 请求 Service


The key 请求 service 是 part 的 the key retention service (参考 到
Documentation/安全/keys/核心.rst).  此 document explains 更多 fully 如何
the requesting algorithm works.

The 进程 starts 由 任一个 the 内核 requesting 一个 service 由 calling
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
The 主要 difference 之间 the access points 是 该 the in-kernel 接口
执行 不 需要 到 link the key 到 一个 keyring 到 prevent 它 来自 正在 immediately
destroyed.  The 内核 接口 returns 一个 指针 directly 到 the key, 和
它's up 到 the caller 到 destroy the key.

The 请求_key_tag() call 是 类似 the in-kernel 请求_key(), except 该 它
也 takes 一个 domain tag 该 allows keys 到 为 separated 由 namespace 和
killed off 作为 一个 group.

The 请求_key_与_auxdata() calls 是 类似 the 请求_key_tag() call, except
该 它们 permit auxiliary 数据 到 为 passed 到 the upcaller (the 默认 是
NULL).  这是 仅 useful 用于 那些 key types 该 定义 它们的 own upcall
mechanism rather 比 使用 /sbin/request-key.

The 请求_key_rcu() call 是 类似 the 请求_key_tag() call, except 该 它
doesn't check 用于 keys 该 是 在…下 construction 和 doesn't attempt 到
construct missing keys.

The userspace 接口 links the key 到 一个 keyring associated 与 the 进程
到 prevent the key 来自 going away, 和 returns the 串行 数字 的 the key 到
the caller.


The 以下 示例 assumes 该 the key types involved don't 定义 它们的
own upcall mechanisms.  若 它们 执行, 然后 那些 应当 为 substituted 用于 the
forking 和 execution 的 /sbin/request-key.


## The 进程


一个 请求 proceeds 在 the 以下 manner:

  1) 进程 一个 calls 请求_key() [the userspace 系统调用 calls the 内核
     接口].

  2) 请求_key() searches the 进程's subscribed keyrings 到 参见 若 那里's
     一个 suitable key 那里.  若 存在, 它 returns the key.  若 那里 isn't,
     和 callout_info 是 不 set, 一个 错误 是 returned.  否则 the 进程
     proceeds 到 the 接下来 step.

  3) 请求_key() sees 该 一个 doesn't 具有 the desired key 尚未, 因此 它 creates
     two things:

      一个) 一个 uninstantiated key U 的 requested 类型 和 description.

      b) 一个 authorisation key V 该 refers 到 key U 和 notes 该 进程 一个
     	 是 the 上下文 在 其 key U 应当 为 instantiated 和 secured, 和
     	 来自 其 associated key requests 可 为 satisfied.

  4) 请求_key() 然后 forks 和 executes /sbin/request-key 与 一个 新 会话
     keyring 该 包含 一个 link 到 auth key V.

  5) /sbin/request-key assumes the authority associated 与 key U.

  6) /sbin/request-key execs 一个 appropriate program 到 perform the actual
     instantiation.

  7) The program 可 希望 到 access another key 来自 一个's 上下文 (say 一个
     Kerberos TGT key).  它 just requests the appropriate key, 和 the keyring
     search notes 该 the 会话 keyring 具有 auth key V 在 其 bottom level.

     此 将 permit 它 到 然后 search the keyrings 的 进程 一个 与 the
     UID, GID, groups 和 安全 info 的 进程 一个 作为 若 它 曾是 进程 一个,
     和 come up 与 key W.

  8) The program 然后 执行 什么 它 必须 到 get the 数据 与 其 到
     instantiate key U, 使用 key W 作为 一个 参考 (perhaps 它 contacts 一个
     Kerberos server 使用 the TGT) 和 然后 instantiates key U.

  9) Upon instantiating key U, auth key V 是 automatically revoked 因此 该 它
     可 不 为 使用 再次.

  10) The program 然后 exits 0 和 请求_key() deletes key V 和 returns key
      U 到 the caller.

此 也 extends further.  若 key W (step 7 上文) didn't exist, key W 将会
为 已创建 uninstantiated, another auth key (X) 将会 为 已创建 (作为 每 step
3) 和 another copy 的 /sbin/request-key spawned (作为 每 step 4); 但 the
上下文 specified 由 auth key X 将 仍然 为 进程 一个, 作为 它 曾是 在 auth key
V.

这是 因为 进程 一个's keyrings 可't simply 为 attached 到
/sbin/request-key 在 the appropriate places 因为 (一个) execve 将 discard two
的 them, 和 (b) 它 需要 the 相同 UID/GID/Groups 全部 the way through.


## Negative Instantiation 和 Rejection


Rather 比 instantiating 一个 key, 它是 可能 用于 the possessor 的 一个
authorisation key 到 negatively instantiate 一个 key 该's 在…下 construction.
这是 一个 short duration placeholder 该 causes 任何 attempt 在 re-requesting
the key 同时 它 exists 到 fail 与 错误 ENOKEY 若 negated 或 the specified
错误 若 rejected.

这是 provided 到 prevent excessive repeated spawning 的 /sbin/request-key
进程 用于 一个 key 该 将 从不 为 obtainable.

应当 the /sbin/request-key 进程 exit anything 其他 比 0 或 die 在 一个
信号, the key 在…下 construction 将 为 automatically negatively
instantiated 用于 一个 short amount 的 time.


## The Search Algorithm


一个 search 的 任何 特定 keyring proceeds 在 the 以下 fashion:

  1) 当 the key 管理 code searches 用于 一个 key (keyring_search_rcu) 它
     firstly calls key_permission(SEARCH) 在 the keyring 它's starting 与,
     若 此 denies permission, 它 doesn't search further.

  2) 它 considers 全部 the non-keyring keys 之内 该 keyring 和, 若 任何 key
     matches the criteria specified, calls key_permission(SEARCH) 在 它 到 参见
     若 the key 是 allowed 到 为 found.  若 它是, 该 key 是 returned; 若
     不, the search continues, 和 the 错误 code 是 retained 若 的 higher
     优先级 比 the one currently set.

  3) 它 然后 considers 全部 the keyring-type keys 在 the keyring 它's currently
     searching.  它 calls key_permission(SEARCH) 在 每个 keyring, 和 若 此
     grants permission, 它 recurses, executing steps (2) 和 (3) 在 该
     keyring.

The 进程 stops immediately 一个 valid key 是 found 与 permission granted 到
使用 它.  任何 错误 来自 一个 前一个 match attempt 是 discarded 和 the key 是
returned.

当 请求_key() 是 invoked, 若 配置_KEYS_请求_缓存=y, 一个 per-task
one-key 缓存 是 第一 checked 用于 一个 match.

当 search_进程_keyrings() 是 invoked, 它 performs the 以下 searches
直到 one succeeds:

  1) 若 extant, the 进程's 线程 keyring 是 searched.

  2) 若 extant, the 进程's 进程 keyring 是 searched.

  3) The 进程's 会话 keyring 是 searched.

  4) 若 the 进程 具有 assumed the authority associated 与 一个 请求_key()
     authorisation key 然后:

      一个) 若 extant, the calling 进程's 线程 keyring 是 searched.

      b) 若 extant, the calling 进程's 进程 keyring 是 searched.

      c) The calling 进程's 会话 keyring 是 searched.

The moment one succeeds, 全部 pending 错误 是 discarded 和 the found key 是
returned.  若 配置_KEYS_请求_缓存=y, 然后 该 key 是 placed 在 the
per-task 缓存, displacing the 前一个 key.  The 缓存 是 cleared 在 exit 或
just prior 到 resumption 的 userspace.

仅 若 全部 这些 fail 执行 the whole thing fail 与 the highest 优先级
错误.  注意 该 若干 错误 可 具有 come 来自 LSM.

```

	EKEYREVOKED > EKEYEXPIRED > ENOKEY

```
EACCES/EPERM 是 仅 returned 在 一个 direct search 的 一个 特定 keyring 何处
the basal keyring 执行 不 grant Search permission.
