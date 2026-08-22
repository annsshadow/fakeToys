
## BPF_MAP_TYPE_QUEUE 涓?BPF_MAP_TYPE_STACK


   - `BPF_MAP_TYPE_QUEUE` `BPF_MAP_TYPE_STACK` 在内核版4.20 中引
`BPF_MAP_TYPE_QUEUE` BPF 程序提供 FIFO（先进先出）存储，`BPF_MAP_TYPE_STACK` 提供 LIFO
（后进先出）存储。这些映射支peek、pop push 操作，通过相应的辅助函数暴露给 BPF 程序这些操作通过现有`bpf` 系统调用以下列方式暴露给用户空间应用程序
- `BPF_MAP_LOOKUP_ELEM` -> peek（查看）
- `BPF_MAP_LOOKUP_AND_DELETE_ELEM` -> pop（弹出）
- `BPF_MAP_UPDATE_ELEM` -> push（压入）

`BPF_MAP_TYPE_QUEUE` `BPF_MAP_TYPE_STACK` 不支`BPF_F_NO_PREALLOC`
## 用法


### 内核 BPF


#### bpf_map_push_elem()


   long bpf_map_push_elem(struct bpf_map **map, const void **value, u64 flags)

可使`bpf_map_push_elem` 辅助函数将一个元`value` 添加到队列或栈。必须将 `flags` 参数
设为 `BPF_ANY` `BPF_EXIST`。如果将 `flags` 设为 `BPF_EXIST`，则当队列或栈已满时，将移除
最旧的元素以腾出空间添`value`。成功时返回 `0`，失败时返回负的错误码
#### bpf_map_peek_elem()


   long bpf_map_peek_elem(struct bpf_map **map, void **value)

该辅助函数从队列或栈中获取一个元`value` 而不移除它。成功时返回 `0`，失败时返回负的错误码
#### bpf_map_pop_elem()


   long bpf_map_pop_elem(struct bpf_map **map, void **value)

该辅助函数将一个元素移除到 `value` 中，从队列或栈中弹出。成功时返回 `0`，失败时返回负的错误码

### 用户空间


#### bpf_map_update_elem()


   int bpf_map_update_elem (int fd, const void **key, const void **value, __u64 flags)

用户空间程序可以使用 libbpf `bpf_map_update_elem` 函数`value` 压入队列或栈。`key` 参数
必须设为 `NULL`，且 `flags` 必须设为 `BPF_ANY` `BPF_EXIST`，语义与 `bpf_map_push_elem`
内核辅助函数相同。成功时返回 `0`，失败时返回负的错误码
#### bpf_map_lookup_elem()


   int bpf_map_lookup_elem (int fd, const void **key, void **value)

用户空间程序可以使用 libbpf `bpf_map_lookup_elem` 函数查看队列或栈头部（head）的 `value``key` 参数必须设为 `NULL`。成功时返回 `0`，失败时返回负的错误码
#### bpf_map_lookup_and_delete_elem()


   int bpf_map_lookup_and_delete_elem (int fd, const void **key, void **value)

用户空间程序可以使用 libbpf `bpf_map_lookup_and_delete_elem` 函数从队列或栈头部弹出一`value`。`key` 参数必须设为 `NULL`。成功时返回 `0`，失败时返回负的错误码
## 示例


### 内核 BPF


以下片段展示如何BPF 程序中声明一个队列：


    struct {
            __uint(type, BPF_MAP_TYPE_QUEUE);
            __type(value, __u32);
            __uint(max_entries, 10);
    } queue SEC(".maps");


### 用户空间


以下片段展示如何使用 libbpf 的低API 从用户空间创建一个队列：


    int create_queue()
    {
            return bpf_map_create(BPF_MAP_TYPE_QUEUE,
                                  "sample_queue", /** name **/
                                  0,              /** key size, must be zero **/
                                  sizeof(__u32),  /** value size **/
                                  10,             /** max entries **/
                                  NULL);          /** create options **/
    }


## 参

https://lwn.net/ml/netdev/153986858555.9127.14517764371945179514.stgit@kernel/
