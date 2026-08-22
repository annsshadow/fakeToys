## Linux 中的红黑树（rbtree

:Date: January 18, 2007
:Author: Rob Landley <rob@landley.net>

### 什么是红黑树，它们有什么用途？


红黑树是一种自平衡的二叉搜索树，用于存储可排序的键/值数据对。它与基数树（用于高效存储稀疏数组，因而使用长整型索引来插访问/删除节点）和哈希表（不保持有序以便按顺序遍历，且必须针对特定大小和哈希函数调优，rbtree 在存储任意键时可优雅地扩展）不同
红黑树与 AVL 树类似，但在插入和删除时提供更快速、实时有界的 worst case 性能（分别最多两次和三次旋转来平衡树），查找时间稍慢（但仍是 O(log n)）
引用 Linux Weekly News 的说法：

    内核中使用了若干红黑树。deadline CFQ I/O 调度器利rbtree 来跟踪请求；数据CD/DVD 驱动也这样做。高精度定时器代码使rbtree 来组织待处理的定时器请求。ext3 文件系统用红黑树跟踪目录项。虚拟内存区域（VMA）用红黑树跟踪，epoll 文件描述符、加密密钥以及“分层令牌桶（hierarchical token bucket）”调度器中的网络数据包也是如此
本文档介Linux rbtree 实现的使用方法。有关红黑树本质与实现的更多信息，请参阅
  Linux Weekly News 上关于红黑树的文    https://lwn.net/Articles/184495/

  维基百科上关于红黑树的条    https://en.wikipedia.org/wiki/Red-black_tree

### 红黑树的 Linux 实现


Linux rbtree 实现位于 "lib/rbtree.c" 文件中。使用它需"#include <linux/rbtree.h>"
Linux rbtree 实现针对速度进行了优化，因此比更传统的树实现少了一层间接（并具有更好的缓存局部性）。它不使用指向独立的 rb_node 与数据结构体的指针，而是将每struct rb_node 实例嵌入到它所组织的数据结构体中。而且，它不使用比较回调函数指针，而是要求用户自行编写调用所提供 rbtree 函数的树查找与插入函数。加锁也rbtree 代码的使用者负责
### 创建一个新rbtree


```

  struct mytype {
  	struct rb_node node;
  	char *keystring;
  };

```
当处理指向内struct rb_node 的指针时，可以使用标准的 container_of() 宏访问其所在的数据结构体。此外，也可以通过 rb_entry(node, type, member) 直接访问各个成员
每个 rbtree 的根部都是一rb_root 结构体，可通过以下方式初始化为空：

  struct rb_root mytree = RB_ROOT;

### rbtree 中查找

为你的树编写查找函数相当直接：从根开始，比较每个值，并按需沿左分支或右分支向下
```

  struct mytype *my_search(struct rb_root *root, char *string)
  {
  	struct rb_node *node = root->rb_node;

  	while (node) {
  		struct mytype *data = container_of(node, struct mytype, node);
		int result;

		result = strcmp(string, data->keystring);

		if (result < 0)
  			node = node->rb_left;
		else if (result > 0)
  			node = node->rb_right;
		else
  			return data;
	}
	return NULL;
  }

```
### rbtree 中插入数

向树中插入数据，首先需查找新节点的插入位置，然后插入该节点并对树重新平衡（“重新着色”）
插入时的查找与前述查找的不同之处在于：要找到用于嫁接新节点的指针所在位置。新节点还需要一个指向其父节点的链接，以便进行重新平衡
```

  int my_insert(struct rb_root *root, struct mytype *data)
  {
  	struct rb_node **new = &(root->rb_node), *parent = NULL;

  	/* Figure out where to put new node */
  	while (*new) {
  		struct mytype *this = container_of(*new, struct mytype, node);
  		int result = strcmp(data->keystring, this->keystring);

		parent = *new;
  		if (result < 0)
  			new = &((*new)->rb_left);
  		else if (result > 0)
  			new = &((*new)->rb_right);
  		else
  			return FALSE;
  	}

  	/* Add new node and rebalance tree. */
  	rb_link_node(&data->node, parent, new);
  	rb_insert_color(&data->node, root);

	return TRUE;
  }

```
### rbtree 中删除或替换已有数据


```

  void rb_erase(struct rb_node *victim, struct rb_root *tree);

```
```

  struct mytype *data = mysearch(&mytree, "walrus");

  if (data) {
  	rb_erase(&data->node, &mytree);
  	myfree(data);
  }

```
```

  void rb_replace_node(struct rb_node *old, struct rb_node *new,
  			struct rb_root *tree);

```
以这种方式替换节点不会对树重新排序：如果新节点的键与旧节点不同，rbtree 很可能被破坏
### 遍历 rbtree 中存储的元素（按排序顺序

提供了四个函数，用于按排序顺序遍rbtree 的内容。它们适用于任意树，通常不需```

  struct rb_node *rb_first(struct rb_root *tree);
  struct rb_node *rb_last(struct rb_root *tree);
  struct rb_node *rb_next(struct rb_node *node);
  struct rb_node *rb_prev(struct rb_node *node);

```
要开始遍历，使用指向树根的指针调rb_first() rb_last()，它们会返回指向树中第一个或最后一个元素所包含节点结构体的指针。要继续遍历，可在当前节点上调用 rb_next() rb_prev() 获取下一个或上一个节点。当没有更多节点时，将返NULL
这些迭代函数返回指向内嵌 struct rb_node 的指针，可借助 container_of() 宏访问其所在的数据结构体，也可通过 rb_entry(node, type, member) 直接访问各个成员
```

  struct rb_node *node;
  for (node = rb_first(&mytree); node; node = rb_next(node))
	printk("key=%s\n", rb_entry(node, struct mytype, node)->keystring);

```
### 带缓存的 rbtree


计算最左（最小）节点是二叉搜索树中相当常见的任务，例如用于遍历，或用于依赖特定顺序的自身逻辑。为此，用户可以使用 'struct rb_root_cached' O(logN) rb_first() 调用优化为一次简单的指针获取，从而避免可能代价高昂的树遍历。这样做带来的维护运行时开销可忽略不计，但会占用更大的内存
rb_root 结构体类似，带缓存的 rbtree 通过以下方式初始化为空：
```

  struct rb_root_cached mytree = RB_ROOT_CACHED;

```
带缓存的 rbtree 只是一个普通的 rb_root，额外带有一个用于缓存最左节点的指针。这使得 rb_root_cached 可以出现rb_root 能出现的任何地方，从而既支持增强型树，也只需少量额外
```

  struct rb_node *rb_first_cached(struct rb_root_cached *tree);
  void rb_insert_color_cached(struct rb_node *, struct rb_root_cached *, bool);
  void rb_erase_cached(struct rb_node *node, struct rb_root_cached *);

```
插入和删除调用都有各自对应的增强```

  void rb_insert_augmented_cached(struct rb_node *node, struct rb_root_cached *,
				  bool, struct rb_augment_callbacks *);
  void rb_erase_augmented_cached(struct rb_node *, struct rb_root_cached *,
				 struct rb_augment_callbacks *);


```
### 对增强型 rbtree 的支

增强rbtree 是在每个节点中存储“一些”额外数据的 rbtree，其中节N 的额外数据必须是 N 为根的子树中所有节点内容的函数。这些数据可用于rbtree 增强一些新功能。增强型 rbtree 是构建在基础 rbtree 基础设施之上的可选特性。想要使用该特性的 rbtree 使用者，必须在插入和删除节点时，配合用户提供增强回调来调用增强函数
实现增强rbtree 操作C 文件必须包含 <linux/rbtree_augmented.h> 而不<linux/rbtree.h>。请注意，linux/rbtree_augmented.h 暴露了一些你不应依赖rbtree 实现细节；请只使用其中已文档化的 API，并且也不要在头文件中包<linux/rbtree_augmented.h>，以尽量降低你的使用者意外依赖这些实现细节的可能性
插入时，使用者必须更新通向被插入节点的路径上的增强信息，然后像往常一样调rb_link_node()，并使用 rb_augment_inserted() 代替通常rb_insert_color() 调用。如rb_augment_inserted() 重新平衡rbtree，它会回调用户提供的函数来更新受影响子树上的增强信息
删除节点时，使用者必须调rb_erase_augmented() 而不rb_erase()。rb_erase_augmented() 会回调用户提供的函数，以更新受影响子树上的增强信息
在这两种情况下，回调都通过 struct rb_augment_callbacks 提供。必须定3 个回调：

- 一个传播（propagation）回调，用于更新给定节点及其祖先的增强值，直到给定的停止点（或 NULL 表示一路更新到根）
- 一个复制（copy）回调，用于将给定子树的增强值复制到新指定的子树根
- 一个树旋转（tree rotation）回调，用于将给定子树的增强值复制到新指定的子树根，并重新计算原子树根的增强信息
rb_erase_augmented() 的编译代码可能会内联传播和复制回调，从而产生一个较大的函数，因此每个增强型 rbtree 使用者应当只有一rb_erase_augmented() 调用点，以限制编译后的代码大小

##### 使用示例


区间树是增强rb 树的一个例子。参考——Cormen、Leiserson、Rivest Stein 所著的《算法导论》。有关区间树的更多细节：

经典rbtree 只有一个键，无法直接用于存[lo:hi] 这样的区间范围，也无法快速查找是否与新的 lo:hi 发生重叠，或判断是否存在与新 lo:hi 完全匹配的项
不过，rbtree 可以被增强，以结构化的方式存储此类区间范围，从而能够实现高效的查找与精确匹配
存储在每个节点中的这种“额外信息”，是其所有后代节点中的最hi（max_hi）值。只需查看节点及其直接子节点，即可在每个节点上维护该信息。它将被用于 O(log n) 的查找中，以找到最低匹配（所有匹配项中最低的起始地址```

  struct interval_tree_node *
  interval_tree_first_match(struct rb_root *root,
			    unsigned long start, unsigned long last)
  {
	struct interval_tree_node *node;

	if (!root->rb_node)
		return NULL;
	node = rb_entry(root->rb_node, struct interval_tree_node, rb);

	while (true) {
		if (node->rb.rb_left) {
			struct interval_tree_node *left =
				rb_entry(node->rb.rb_left,
					 struct interval_tree_node, rb);
			if (left->__subtree_last >= start) {
				/*
				 * Some nodes in left subtree satisfy Cond2.
				 * Iterate to find the leftmost such node N.
				 * If it also satisfies Cond1, that's the match
				 * we are looking for. Otherwise, there is no
				 * matching interval as nodes to the right of N
				 * can't satisfy Cond1 either.
				 */
				node = left;
				continue;
			}
		}
		if (node->start <= last) {		/* Cond1 */
			if (node->last >= start)	/* Cond2 */
				return node;	/* node is leftmost match */
			if (node->rb.rb_right) {
				node = rb_entry(node->rb.rb_right,
					struct interval_tree_node, rb);
				if (node->__subtree_last >= start)
					continue;
			}
		}
		return NULL;	/* No match */
	}
  }

```
```

  static inline unsigned long
  compute_subtree_last(struct interval_tree_node *node)
  {
	unsigned long max = node->last, subtree_last;
	if (node->rb.rb_left) {
		subtree_last = rb_entry(node->rb.rb_left,
			struct interval_tree_node, rb)->__subtree_last;
		if (max < subtree_last)
			max = subtree_last;
	}
	if (node->rb.rb_right) {
		subtree_last = rb_entry(node->rb.rb_right,
			struct interval_tree_node, rb)->__subtree_last;
		if (max < subtree_last)
			max = subtree_last;
	}
	return max;
  }

  static void augment_propagate(struct rb_node *rb, struct rb_node *stop)
  {
	while (rb != stop) {
		struct interval_tree_node *node =
			rb_entry(rb, struct interval_tree_node, rb);
		unsigned long subtree_last = compute_subtree_last(node);
		if (node->__subtree_last == subtree_last)
			break;
		node->__subtree_last = subtree_last;
		rb = rb_parent(&node->rb);
	}
  }

  static void augment_copy(struct rb_node *rb_old, struct rb_node *rb_new)
  {
	struct interval_tree_node *old =
		rb_entry(rb_old, struct interval_tree_node, rb);
	struct interval_tree_node *new =
		rb_entry(rb_new, struct interval_tree_node, rb);

	new->__subtree_last = old->__subtree_last;
  }

  static void augment_rotate(struct rb_node *rb_old, struct rb_node *rb_new)
  {
	struct interval_tree_node *old =
		rb_entry(rb_old, struct interval_tree_node, rb);
	struct interval_tree_node *new =
		rb_entry(rb_new, struct interval_tree_node, rb);

	new->__subtree_last = old->__subtree_last;
	old->__subtree_last = compute_subtree_last(old);
  }

  static const struct rb_augment_callbacks augment_callbacks = {
	augment_propagate, augment_copy, augment_rotate
  };

  void interval_tree_insert(struct interval_tree_node *node,
			    struct rb_root *root)
  {
	struct rb_node **link = &root->rb_node, *rb_parent = NULL;
	unsigned long start = node->start, last = node->last;
	struct interval_tree_node *parent;

	while (*link) {
		rb_parent = *link;
		parent = rb_entry(rb_parent, struct interval_tree_node, rb);
		if (parent->__subtree_last < last)
			parent->__subtree_last = last;
		if (start < parent->start)
			link = &parent->rb.rb_left;
		else
			link = &parent->rb.rb_right;
	}

	node->__subtree_last = last;
	rb_link_node(&node->rb, rb_parent, link);
	rb_insert_augmented(&node->rb, root, &augment_callbacks);
  }

  void interval_tree_remove(struct interval_tree_node *node,
			    struct rb_root *root)
  {
	rb_erase_augmented(&node->rb, root, &augment_callbacks);
  }

```
