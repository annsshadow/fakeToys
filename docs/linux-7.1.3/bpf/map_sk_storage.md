
## BPF_MAP_TYPE_SK_STORAGE


   - `BPF_MAP_TYPE_SK_STORAGE` 在内核版本 5.2 中引入

`BPF_MAP_TYPE_SK_STORAGE` 用于为 BPF 程序提供套接字本地（socket-local）
存储。类型为 `BPF_MAP_TYPE_SK_STORAGE` 的映射声明要提供的存储类型，并作为
访问套接字本地存储的句柄。类型为 `BPF_MAP_TYPE_SK_STORAGE` 的映射的值与
映射本身一起存储在每个套接字本地，而不是与映射一起存储。内核负责在请求时
为套接字分配存储，并在映射或套接字被删除时释放存储。

  - 键类型必须为 `int`，并且 `max_entries` 必须设为 `0`。
  - 创建用于套接字本地存储的映射时必须使用 `BPF_F_NO_PREALLOC` 标志。

## 用法


### 内核 BPF


#### bpf_sk_storage_get()


   void **bpf_sk_storage_get(struct bpf_map **map, void **sk, void **value, u64 flags)

可以使用 `bpf_sk_storage_get()` 辅助函数从套接字 `sk` 获取 `map` 的
套接字本地存储。如果使用了 `BPF_LOCAL_STORAGE_GET_F_CREATE` 标志，那么
`bpf_sk_storage_get()` 将在 `sk` 尚不存在时为其创建存储。可以结合
`BPF_LOCAL_STORAGE_GET_F_CREATE` 使用 `value` 来初始化存储值，否则它将被
零初始化。成功时返回指向该存储的指针，失败时返回 `NULL`。

   - 对于 LSM 或跟踪（tracing）程序，`sk` 是一个内核 `struct sock` 指针。
   - 对于其它程序类型，`sk` 是一个 `struct bpf_sock` 指针。

#### bpf_sk_storage_delete()


   long bpf_sk_storage_delete(struct bpf_map **map, void **sk)

可以使用 `bpf_sk_storage_delete()` 辅助函数从套接字 `sk` 删除 `map` 的
套接字本地存储。成功时返回 `0`，失败时返回负的错误。

### 用户空间


#### bpf_map_update_elem()


   int bpf_map_update_elem(int map_fd, const void **key, const void **value, __u64 flags)

可以使用 `bpf_map_update_elem()` libbpf 函数将 `map` 映射的套接字本地存储
添加或更新到某个套接字本地。套接字由存储在指针 `key` 中的 `socket` `fd`
标识。指针 `value` 包含要添加或更新到该套接字 `fd` 的数据。`value` 的类型
和大小应与映射定义中的值类型相同。

`flags` 参数可用于控制更新行为：

- `BPF_ANY` 将为 `socket` `fd` 创建存储或更新现有存储。
- `BPF_NOEXIST` 将仅在 `socket` `fd` 尚不存在时才为其创建存储，否则调用
  将以 `-EEXIST` 失败。
- `BPF_EXIST` 将仅在 `socket` `fd` 已存在时才更新其现有存储，否则调用将
  以 `-ENOENT` 失败。

成功时返回 `0`，失败时返回负的错误。

#### bpf_map_lookup_elem()


   int bpf_map_lookup_elem(int map_fd, const void *key, void **value)

可以使用 `bpf_map_lookup_elem()` libbpf 函数从某个套接字获取 `map` 映射的
套接字本地存储。存储是从由指针 `key` 中存储的 `socket` `fd` 所标识的套接字
获取的。成功时返回 `0`，失败时返回负的错误。

#### bpf_map_delete_elem()


   int bpf_map_delete_elem(int map_fd, const void *key)

可以使用 `bpf_map_delete_elem()` libbpf 函数从某个套接字删除 `map` 映射的
套接字本地存储。存储从由指针 `key` 中存储的 `socket` `fd` 所标识的套接字
删除。成功时返回 `0`，失败时返回负的错误。

## 示例


### 内核 BPF


以下片段展示了如何在 BPF 程序中声明套接字本地存储：


    struct {
            __uint(type, BPF_MAP_TYPE_SK_STORAGE);
            __uint(map_flags, BPF_F_NO_PREALLOC);
            __type(key, int);
            __type(value, struct my_storage);
    } socket_storage SEC(".maps");

以下片段展示了如何在 BPF 程序中获取套接字本地存储：


    SEC("sockops")
    int _sockops(struct bpf_sock_ops *ctx)
    {
            struct my_storage *storage;
            struct bpf_sock *sk;

            sk = ctx->sk;
            if (!sk)
                    return 1;

            storage = bpf_sk_storage_get(&socket_storage, sk, 0,
                                         BPF_LOCAL_STORAGE_GET_F_CREATE);
            if (!storage)
                    return 1;

            /** 在此使用 'storage' **/

            return 1;
    }


有关功能示例，请参阅 `tools/testing/selftests/bpf` 目录。

## 参考资料


https://lwn.net/ml/netdev/20190426171103.61892-1-kafai@fb.com/
