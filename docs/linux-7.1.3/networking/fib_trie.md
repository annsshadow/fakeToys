
## LC-trie 实现说明


### 节点类型


leaf
	带有数据的末端节点。它保存一份相关密钥的副本，以及一个按前缀长度排序的路由表'hlist'。参struct leaf struct leaf_info

trie node 鎴?tnode
	一个内部节点，持有子节点（leaf tnode）指针的数组，通过密钥的一个子集进行索引。参Level Compression

### 几个概念解释


Bits（tnode
	用于索引到子数组中的密钥段中的位数——即“子索引”。参Level Compression

Pos（tnode
	用于索引到子数组中的密钥段在密钥中的位置。参Path Compression

Path Compression / 跳过位（skipped bits
	任何给定tnode 都通过其父节点的子数组链接而来，使用由父节点的 "pos" "bits" 指定的密钥段。在某些情况下，tnode 自身"pos" 不会紧邻父节点（pos+bits），而是密钥中有一些位被跳过，因为它们表示一条没有分叉的单路径。这些“跳过的位”构成了 Path Compression。注意，搜索算法在搜索时将直接跳过这些位，因此有必要在叶子中保存密钥，以验证它们确实与我们正在搜索的密钥相匹配

Level Compression / 子数组（child arrays
	trie 保持层级平衡，在特定条件下将满子节点（见 "full_children"）的子节点上移一层，这样每个内部节点tnode"）可以包含指向多个子节点的任意大小的链接数组，而不是纯粹的二叉树。反之，一个子数组大多为空（见 empty_children）的 tnode 可能会被“减半”，将其部分子节点下移一层，以避免子数组不断增大

empty_children
	给定 tnode 的子数组中为 NULL 的位置数量

full_children
	给定 tnode 中未被路径压缩的子节点数量。（换句话说，它们不NULL 或叶子，且其 "pos" 等于tnode "pos"+"bits"。）

	（这里的“full”一词更多地是在“完整”的意义上使用，而非作为“empty”的反义词，这可能有点令人困惑。）

### 注释


我们尽量让代码的结构尽可能接fib_hash，以便进行验证并有助于审阅

fib_find_node()
	理解这段代码的一个好起点。该函数实现了直接的 trie 查找

fib_insert_node()
	trie 中插入一个新的叶子节点。这fib_find_node() 要复杂一些。插入一个新节点意味着我们可能必须trie 的某部分上运行层级压缩算法

trie_leaf_remove()
	查找一个密钥，将其删除，并运行层级压缩算法

trie_rebalance()
	trie 发生变化后用于动态调trie 的关键函数，它被运行以优化并重新组织。它会从给定 tnode 开始向上朝根方向遍trie，在每一步执resize() 以实现层级压缩

resize()
	分析一tnode，并通过反复扩充或收缩子数组大小来优化，直到满足最优层级压缩的标准。这部分相当紧密地遵循原始论文，这里可能还有一些值得试验的空间

inflate()
	tnode 内子数组的大小加倍。由 resize() 使用

halve()
	tnode 内子数组的大小减半——即 inflate() 的逆操作。由 resize() 使用

fn_trie_insert()、fn_trie_delete()、fn_trie_select_default()
	路由操作函数。应当相当紧密地符合 fib_hash 中相应的函数

fn_trie_flush()
	它遍历整trie（使nextleaf()）并搜索必须被移除的空叶子

fn_trie_dump()
	按前缀长度顺序转储路由表。这fib_hash 中相应的函数稍慢，因为我们必须为每个前缀长度遍历整个 trie。相比之下，fib_hash 被组织为每个前缀长度一个“zone哈希

### 閿。


fib_lock 以与 fib_hash 中相同的方式用作读写锁（RW-lock）。不过，这些函数被稍微分离开，以适应其他可能的加锁场景。理论上有可能通过 RCU 运行 trie_rebalance，以避免fn_trie_lookup() 函数中使read_lock

### 主查找机


fn_trie_lookup() 是主查找函数

查找在其最简单的形式下就如同 fib_find_node()。我们逐段密钥地下trie，直到找到一个叶子。check_leaf() 在叶子按前缀排序hlist 中执fib_semantic_match

如果找到匹配，我们就完成了

如果没有找到匹配，我们进入前缀匹配模式。前缀长度从与密钥长度相同开始，每次减少一步，我们向上回溯遍历 trie 以尝试找到最长匹配前缀。目标始终是到达一个叶子，并从 fib_semantic_match 机制获得肯定结果

在每tnode 内部，对最长匹配前缀的搜索包括遍历子数组，不断砍掉（清零）子索引的最低有效位 "1"，直到找到匹配，或者子索引完全由零组成

此时我们向上回溯（t->stats.backtrack++）遍trie，继续砍掉密钥的一部分以寻找最长匹配前缀

此时我们将反复下降子树以寻找匹配，并且有一些可用的优化可以为我们提供“捷径”以避免下降到死胡同。在代码中寻"HL_OPTIMIZE" 部分

为了消除对路由选择过程正确性的任何疑虑，新增了一netlink 操作。寻NETLINK_FIB_LOOKUP，它向用户空间提供对 fib_lookup() 的访问
