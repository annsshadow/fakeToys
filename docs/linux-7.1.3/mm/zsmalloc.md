## zsmalloc

该分配器设计用于zram 配合使用。因此，该分配器被期望在内存紧张的条件下也能良好工作。特别是，它从不尝试高阶页分配，而在内存压力下这种分配极有可能失败。另一方面，如果我们只使用单个 阶）页，它将遭受非常高的碎片——任何大小为 PAGE_SIZE/2 或更大的对象都会占据整个页。这是其前身（xvmalloc）的主要问题之一
为了克服这些问题，zsmalloc 分配一0 阶页，并使用各种 `struct page` 字段将它们链接在一起。这些被链接起来的页充当单个高阶页，也就是说，一个对象可以跨0 阶页的边界。代码将这些被链接的页作为一个称zspage 的单一实体来引用
为简单起见，zsmalloc 只能分配大小不超PAGE_SIZE 的对象，因为这满足了其所有当前用户的需求（在最坏情况下，页不可压缩，因而以“原样”（即未压缩形式）存储）。对于大于此大小的分配请求，返回失败（参zs_malloc）
此外，`zs_malloc()` 不返回可解引用的指针。相反，它返回一个不透明的句柄（unsigned long），其中编码了所分配对象的真实位置。这种间接性的原因zsmalloc 不会永久映射 zspage，因为那样会在内核空间映射的虚拟地址（VA）区域非常小32 位系统上引发问题。因此，使用所分配的内存应当通过合适的基于句柄API 进行
## 统计信息（stat
启用 `CONFIG_ZSMALLOC_STAT` 后，我们可以通过以下方式查看 zsmalloc 内部信息
```
 # cat /sys/kernel/debug/zsmalloc/zram0/classes

 class  size       10%       20%       30%       40%       50%       60%       70%       80%       90%       99%      100% obj_allocated   obj_used pages_used pages_per_zspage freeable
    ...
    ...
    30   512         0        12         4         1         0         1         0         0         1         0       414          3464       3346        433                1       14
    31   528         2         7         2         2         1         0         1         0         0         2       117          4154       3793        536                4       44
    32   544         6         3         4         1         2         1         0         0         0         1       260          4170       3965        556                2       26
    ...
    ...

```
class
	索引
size
	zspage 存储的对象大10%
	使用率低10% zspage 数量（见下文20%
	使用率在 10% 20% 之间zspage 数量
30%
	使用率在 20% 30% 之间zspage 数量
40%
	使用率在 30% 40% 之间zspage 数量
50%
	使用率在 40% 50% 之间zspage 数量
60%
	使用率在 50% 60% 之间zspage 数量
70%
	使用率在 60% 70% 之间zspage 数量
80%
	使用率在 70% 80% 之间zspage 数量
90%
	使用率在 80% 90% 之间zspage 数量
99%
	使用率在 90% 99% 之间zspage 数量
100%
	使用率为 100% zspage 数量
obj_allocated
	已分配的对象数量
obj_used
	已分配给用户的对象数pages_used
	为该 size class 分配的页pages_per_zspage
	组成一zspage 所需0 阶页freeable
	size class 压缩后可释放的大致页
每个 zspage 维护一inuse 计数器，用于追踪zspage 中存储的对象数量。inuse 计数器决定了 zspage 的“填充度分组”（fullness group），其计算方式为 “inuse对象数量zspage 可容纳的对象总数（objs_per_zspage）之比。inuse 计数器越接近 objs_per_zspage 越好
## 内部实现

zsmalloc 255 size class，每size class 可以容纳若干zspage。每zspage 最多可包含 ZSMALLOC_CHAIN_SIZE 个物理（0 阶）页。每size class 的最zspage 链大小在创建 zsmalloc 池时计算（参`calculate_zspage_chain_size()`）
作为一种优化，zsmalloc 会合并在“每 zspage 页数”和“每zspage 可存储对象数”方面具有相似特征的 size class
```
  class  size       10%   ....    100% obj_allocated   obj_used pages_used pages_per_zspage freeable
  ...
     94  1536        0    ....       0             0          0          0                3        0
    100  1632        0    ....       0             0          0          0                2        0
  ...

```
size class #95-99 被合并到 size class #100。这意味着当我们需要存储一个大小为（例如）1568 字节的对象时，最终会用到 size class #100 而非 size class #96。size class #100 面向大小1632 字节的对象，因此每个大小1568 字节的对象会浪费 1632-1568=64 字节
size class #100 由每个含 2 个物理页zspage 组成，总共可容5 个对象。如果我们需要存13 个大小为 1568 的对象，最终会分配三个 zspage，即 6 个物理页
然而，如果我们仔细查看 size class #96（面向大小为 1568 字节的对象）并追`calculate_zspage_chain_size()`，会发现class 最优的 zspage 配置是一个链
```
    pages per zspage      wasted bytes     used%
           1                  960           76
           2                  352           95
           3                 1312           89
           4                  704           95
           5                   96           99

```
这意味着，由 5 个物理页组成class #96 配置可以在单zspage 中存13 个大小为 1568 的对象，总共使用 5 个物理页。这class #100 的配置更高效，后者会6 个物理页来存储相同数量的对象
随着 class #96 zspage 链大小增加，其关键特征（如每 zspage 页数和每 zspage 对象数）也随之改变。这导致更少class 合并，从而形成更紧凑class 分组，减少了内存浪费
```
  class  size       10%   ....    100% obj_allocated   obj_used pages_used pages_per_zspage freeable

  ...
    202  3264         0   ..         0             0          0          0                4        0
    254  4096         0   ..         0             0          0          0                1        0
  ...

```
size class #202 存储大小3264 字节的对象，每个 zspage 最4 页。任何大3264 字节的对象被视为巨大（huge）对象，属于 size class #254，该 class 将每个对象存储在其自己的物理页中（巨class 中的对象不共享页）
增大 zspage 链的大小也会导致巨大 size class watermark 更高，总体上巨class 更少。这允许更高效地存储大对象
```
  class  size       10%   ....    100% obj_allocated   obj_used pages_used pages_per_zspage freeable

  ...
    202  3264         0   ..         0             0          0          0                4        0
    211  3408         0   ..         0             0          0          0                5        0
    217  3504         0   ..         0             0          0          0                6        0
    222  3584         0   ..         0             0          0          0                7        0
    225  3632         0   ..         0             0          0          0                8        0
    254  4096         0   ..         0             0          0          0                1        0
  ...

```
```
  class  size       10%   ....    100% obj_allocated   obj_used pages_used pages_per_zspage freeable

  ...
    202  3264         0   ..         0             0          0          0                4        0
    206  3328         0   ..         0             0          0          0               13        0
    207  3344         0   ..         0             0          0          0                9        0
    208  3360         0   ..         0             0          0          0               14        0
    211  3408         0   ..         0             0          0          0                5        0
    212  3424         0   ..         0             0          0          0               16        0
    214  3456         0   ..         0             0          0          0               11        0
    217  3504         0   ..         0             0          0          0                6        0
    219  3536         0   ..         0             0          0          0               13        0
    222  3584         0   ..         0             0          0          0                7        0
    223  3600         0   ..         0             0          0          0               15        0
    225  3632         0   ..         0             0          0          0                8        0
    228  3680         0   ..         0             0          0          0                9        0
    230  3712         0   ..         0             0          0          0               10        0
    232  3744         0   ..         0             0          0          0               11        0
    234  3776         0   ..         0             0          0          0               12        0
    235  3792         0   ..         0             0          0          0               13        0
    236  3808         0   ..         0             0          0          0               14        0
    238  3840         0   ..         0             0          0          0               15        0
    254  4096         0   ..         0             0          0          0                1        0
  ...

```
```
  pages per zspage   number of size classes (clusters)   huge size class watermark
         4                        69                               3264
         5                        86                               3408
         6                        93                               3504
         7                       112                               3584
         8                       123                               3632
         9                       140                               3680
        10                       143                               3712
        11                       159                               3744
        12                       164                               3776
        13                       180                               3792
        14                       183                               3808
        15                       188                               3840
        16                       191                               3840


```
### 一个合成测
zram 用作构建产物存储（Linux 内核编译）
- `CONFIG_ZSMALLOC_CHAIN_SIZE=4`

```
    class  size       10%   ....    100% obj_allocated   obj_used pages_used pages_per_zspage freeable

    ...
    Total              13   ..        51        413836     412973     159955                         3

  zram mm_stat:::

   1691783168 628083717 655175680        0 655175680       60        0    34048    34049


```
- `CONFIG_ZSMALLOC_CHAIN_SIZE=8`

```
    class  size       10%   ....    100% obj_allocated   obj_used pages_used pages_per_zspage freeable

    ...
    Total              18   ..        87        414852     412978     156666                         0

  zram mm_stat:::

    1691803648 627793930 641703936        0 641703936       60        0    33591    33591

```
使用更大zspage 链可能会减少物理页的使用，如示例所示——使用的物理页数159955 下降156666，同zsmalloc 池的最大内存使用量655175680 下降641703936 字节
然而，在内部碎片严重且 zspool 压缩无法重定位对象并释放 zspage 的情况下，这一优势可能被系统内存压力的潜在增加所抵消。在这些情况下，建议减小 zspage 链大小的上限（由 `CONFIG_ZSMALLOC_CHAIN_SIZE` 选项指定）
## 函数

