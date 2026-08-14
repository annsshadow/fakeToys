## BPF 图（graph）数据结构


本文档描述了新型“graph”数据结构（linked_list、rbtree）的实现细节，特别关注
验证器（verifier）对这些数据结构特有的语义的实现。

尽管本文档没有提及任何具体的验证器代码，但假定读者具备 BPF 验证器内部机制、BPF
映射（map）以及 BPF 程序编写的一般知识。

请注意，本文档的目的在于描述这些图数据结构的当前状态。这里**不**作出或暗示对
语义或 API 稳定性的任何保证。

    :local:
    :depth: 2

### 简介


BPF map API 历来是在 BPF 程序中使用各种类型数据结构的主要方式。有些数据结构与 map
API 天然契合（HASH、ARRAY），有些则不然。因此，对于没有 BPF 经验的 kernel 程序员
来说，与后一组数据结构交互的程序可能难以理解。

幸运的是，一些曾经使得必须采用 BPF map 语义的限制已经不再相关。随着 kfuncs、kptrs
以及 any-context BPF 分配器的引入，现在可以实现 API 和语义更贴近内核其他部分所暴露
接口的 BPF 数据结构。

linked_list 和 rbtree 这两个数据结构有许多共同的验证细节。因为两者都有“root”（
linked_list 为 “head”）和“node”，验证器代码和本文档把公共功能称为 “graph_api”、
“graph_root”、“graph_node” 等。

除非另有说明，下面的示例和语义适用于两个图数据结构。

### 不稳定的 API


历史上使用 BPF map API 实现的数据结构使用 BPF 辅助函数（helper）——要么是标准的 map
API 辅助函数如 `bpf_map_update_elem`，要么是 map 特定的辅助函数。新型图数据结构则
改用 kfuncs 来定义它们的操作辅助函数。由于 kfuncs 没有稳定性保证，这些数据结构
的 API 和语义可以在必要时以破坏向后兼容的方式演进。

新数据结构的 root 和 node 类型在 `uapi/linux/bpf.h` 头文件中不透明地定义。

### 加锁（Locking）


新型数据结构是侵入式的（intrusive），定义方式与它们在内核中的普通对应物类似：


        struct node_data {
          long key;
          long data;
          struct bpf_rb_node node;
        };

        struct bpf_spin_lock glock;
        struct bpf_rb_root groot __contains(node_data, node);

linked_list 和 rbtree 的“root”类型都期望位于一个 map_value 中，该 map_value 同时
还包含一个 `bpf_spin_lock` —— 在上例中，两个全局变量都放在一个单值 arraymap 中。
验证器认为这个 spin_lock 与 `bpf_rb_root` 相关联，因为它们都在同一个 map_value 中，
并且会在验证操作该树的 BPF 程序时强制要求持有正确的锁。由于这种锁检查发生在验证
阶段，因此没有运行时开销。

### 非拥有引用（Non-owning references）


**动机**

考虑以下 BPF 代码：


        struct node_data **n = bpf_obj_new(typeof(**n)); /** ACQUIRED **/

        bpf_spin_lock(&lock);

        bpf_rbtree_add(&tree, n); /** PASSED **/

        bpf_spin_unlock(&lock);

从验证器的角度看，从 `bpf_obj_new` 返回的指针 `n` 具有类型 `PTR_TO_BTF_ID |
MEM_ALLOC`，其 `btf_id` 为 `struct node_data`，并且有一个非零的 `ref_obj_id`。因为它
持有 `n`，该程序拥有被指对象（即 `n` 所指向的对象）的生命周期。BPF 程序必须在退出
前交出所有权 —— 要么通过 `bpf_obj_drop`（它会 `free` 该对象），要么通过 `bpf_rbtree_add`
把它加入 `tree`。

（示例中的 `ACQUIRED` 和 `PASSED` 注释分别表示“获得所有权”和“交出所有权”的语句。）

在交出所有权之后，验证器应当如何对待 `n`？如果对象是经由 `bpf_obj_drop` 被 `free` 的，
答案很明显：验证器应当拒绝那些在 `bpf_obj_drop` 之后仍试图访问 `n` 的程序，因为该
对象已不再有效。其底层内存可能已被用于其他分配、被取消映射等等。

当所有权经由 `bpf_rbtree_add` 交给 `tree` 时，答案就不那么明显了。验证器本可以强制
要求与 `bpf_obj_drop` 相同的语义，但那会导致具有有用且常见编码模式的程序被拒绝，例如：


        int x;
        struct node_data **n = bpf_obj_new(typeof(**n)); /** ACQUIRED **/

        bpf_spin_lock(&lock);

        bpf_rbtree_add(&tree, n); /** PASSED **/
        x = n->data;
        n->data = 42;

        bpf_spin_unlock(&lock);

对 `n->data` 的读和写都会被拒绝。不过，验证器可以做得更好，它利用了两个细节：

  - 图数据结构 API 只能在持有与图 root 关联的 `bpf_spin_lock` 时使用

  - 两个图数据结构都具有指针稳定性（pointer stability）

     - 因为图节点是用 `bpf_obj_new` 分配的，而加入/移出 root 涉及摆弄节点结构体的
       `bpf_{list,rb}_node` 字段，所以图节点在任一操作之后都会保持在相同的地址。

由于任何加入或移出该 root 的程序都必须持有关联的 `bpf_spin_lock`，如果我们处于由该
锁界定的临界区中，就知道在该临界区结束之前，没有其他程序能够加入或移出。这一点加上
指针稳定性意味着，直到临界区结束，我们都可以安全地通过 `n` 访问该图节点，即便它已经被
用来交出所有权。

验证器把这样的引用视为**非拥有引用（non-owning reference）**。相应地，`bpf_obj_new`
返回的引用被视为**拥有引用（owning reference）**。这两个术语目前只在图节点和图 API
的语境中有意义。

**细节**

让我们列举两类引用的属性。

**拥有引用（owning reference）**

  - 该引用控制被指对象的生命周期

  - 被指对象的所有权必须被“释放”，方法是把它传给某个图 API kfunc，或者经由
    `bpf_obj_drop`（它会 `free` 被指对象）

    - 如果程序结束前没有释放，验证器认为该程序无效

  - 对被指对象内存的访问不会发生页错误（page fault）

**非拥有引用（non-owning reference）**

  - 该引用不拥有被指对象

     - 它不能用来把图节点加入图 root，也不能经由 `bpf_obj_drop` 被 `free`

  - 没有对生命周期的显式控制，但可以根据非拥有引用的存在推断有效的生命周期（见下
    文说明）

  - 对被指对象内存的访问不会发生页错误

从验证器的角度看，非拥有引用只能存在于 spin_lock 和 spin_unlock 之间。为什么？在
spin_unlock 之后，另一个程序可以对该数据结构做任意操作，例如经由 bpf_obj_drop 移出
并 `free`。一个指向某块内存的非拥有引用，若那块内存被 remove、`free`，又经由
bpf_obj_new 被复用，就会指向一个完全不同的东西。或者那块内存可能消失。

为了防止这种逻辑违规，验证器会在临界区结束后使所有非拥有引用失效。这对于确保非拥有
引用“不会发生页错误”的属性是必要的。因此，如果验证器没有使某个非拥有引用失效，访问
它就不会发生页错误。

目前 `bpf_obj_drop` 不允许在临界区内使用，所以如果存在有效的非拥有引用，我们就必然
处于一个临界区中，从而可以断定该引用的内存没有被 drop 并 `free`，或 drop 并复用。

任何对位于 rbtree 中节点的引用_必须_是非拥有的，因为树控制了被指对象的生命周期。
类似地，任何对不在 rbtree 中节点的引用_必须_是拥有的。这带来了一个不错的性质：图
API 的加入/移出实现不需要检查一个节点是否已经被加入（或已经被移出），因为所有权模型
允许验证器仅通过检查类型就能防止这样一种状态成为有效状态。

然而，指针别名（pointer aliasing）对上面的“不错性质”构成了问题。考虑以下示例：


        struct node_data **n, **m, **o, **p;
        n = bpf_obj_new(typeof(**n));     /** 1 */

        bpf_spin_lock(&lock);

        bpf_rbtree_add(&tree, n);        /** 2 **/
        m = bpf_rbtree_first(&tree);     /** 3 **/

        o = bpf_rbtree_remove(&tree, n); /** 4 **/
        p = bpf_rbtree_remove(&tree, m); /** 5 **/

        bpf_spin_unlock(&lock);

        bpf_obj_drop(o);
        bpf_obj_drop(p); /** 6 **/

假设在该程序运行前树是空的。如果我们用上面注释中的数字来跟踪验证器状态的变化：

  1) n 是一个拥有引用

  2) n 是一个非拥有引用，它已被加入树中

  3) n 和 m 都是非拥有引用，它们都指向同一个节点

  4) o 是一个拥有引用，n 和 m 是非拥有引用，三者都指向同一个节点

  5) o 和 p 是拥有的，n 和 m 是非拥有的，三者都指向同一个节点

  6) 发生了一次双重释放（double-free），因为 o 和 p 指向同一个节点，而 o 在上一语句中
     已被 `free`

状态 4 和 5 违反了我们的“不错性质”，因为存在指向一个不在 rbtree 中的节点的非拥有引用。
语句 5 会试图移出一个已经因这一违规而被移出的节点。状态 6 是危险的双重释放。

至少我们应当防止状态 6 成为可能。如果我们也不能防止状态 5，那就必须放弃我们的“不错性质”，
并在运行时检查一个节点是否已经被移出。

我们通过把 `bpf_spin_unlock` 的“使非拥有引用失效”行为一般化，并在 `bpf_rbtree_remove`
之后做类似的失效处理，来同时防止这两者。这里的逻辑是：任何满足以下条件的图 API kfunc：

  - 接受一个任意的节点参数

  - 把它从数据结构中移出

  - 返回一个指向被移出节点的拥有引用

都可能导致这样一种状态：某个其他非拥有引用指向同一个节点。因此 `remove` 类型的 kfunc
也必须被视为一个非拥有引用的失效点。
