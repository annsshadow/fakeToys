
## Linux 中的并查集（Union-Find

:Date: 2024 骞?6 鏈?21 鏃?:Author: Xavier <xavier_qy@163.com>

### 什么是并查集，它有何用途？


并查集是一种用于处理不相交集合的合并与查询的数据结构。并查集支持的主要操作有
	初始化：将每个元素重置为一个独立的集合，每个集合的初始父节点指向自身
	查找（Find）：确定一个特定元素属于哪个集合，通常返回该集合的一个“代表元素”		该操作用于检查两个元素是否在同一个集合中
	合并（Union）：将两个集合合并为一个
作为一种用于维护集合（组）的数据结构，并查集通常用于解决与离线查询、动态连通性以及图论相关的
问题。它也是 Kruskal 最小生成树算法中的关键组成部分，在网络路由等场景中至关重要。因此，并查被广泛引用。此外，并查集在符号计算、寄存器分配等领域也有应用
空间复杂度：O(n)，其n 为节点数量
时间复杂度：使用路径压缩可以降低 find 操作的时间复杂度，使用按秩合并可以降union 操作时间复杂度。这些优化将每次 find union 操作的平均时间复杂度降为 O(α(n))，其α(n) 为反
阿克曼函数。在实际用途中可大致视为常数时间复杂度
本文档涵Linux 并查集实现的使用。关于并查集的性质与实现的更多信息，请参见
  Wikipedia 上的并查集条    https://en.wikipedia.org/wiki/Disjoint-set_data_structure

### Linux 中的并查集实

Linux 的并查集实现位于文件 "lib/union_find.c" 中。要使用它，需 "#include <linux/union_find.h>"
```

	struct uf_node {
		struct uf_node *parent;
		unsigned int rank;
	};

```
在此结构中，parent 指向当前节点的父节点。rank 字段表示当前树的高度。在合并操作期间，秩较小树会被挂到秩较大的树之下，以维持平衡
### 初始化并查集


你可以使用静态或初始化接口完成初始化。将父指针初始化为指向自身，并将秩设0```

	struct uf_node my_node = UF_INIT_NODE(my_node);

```
鎴?
	uf_node_init(&my_node);

### 查找并查集的根节

该操作主要用于确定两个节点是否属于并查集中的同一个集合。如果它们具有相同的根，则它们位于同一集合中。在 find 操作期间会进行路径压缩，以提高后find 操作的效率```

	int connected;
	struct uf_node *root1 = uf_find(&node_1);
	struct uf_node *root2 = uf_find(&node_2);
	if (root1 == root2)
		connected = 1;
	else
		connected = 0;

```
### 合并并查集中的两个集

要合并并查集中的两个集合，首先找到它们各自的根节点，然后根据根节点的秩将较小的节点链接到较大节点```

	uf_union(&node_1, &node_2);

```
