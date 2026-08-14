
## Linux 中的链表


:Author: Nicolas Frattaroli <nicolas.frattaroli@collabora.com>


## 简介


链表是许多程序中最基本的数据结构之一。Linux 内核实现了几种不同风格的链表。本文档
的目的并非泛泛地解释链表，而是向新的内核开发者展示如何使用 Linux 内核的链表实现。

请注意，尽管链表确实无处不在，但在简单的数组已经无法满足需求的场合，链表很少是
最佳的数据结构选择。特别是，由于其数据局部性（data locality）较差，在需要考虑性能的
情形下，链表是个糟糕的选择。强烈建议去熟悉内核中的其他通用数据结构，尤其是在并发
访问方面。

## Linux 双向链表的实现


Linux 的链表实现可以通过包含头文件 `<linux/list.h>` 来使用。

双向链表对许多读者来说可能是最熟悉的。它可以被高效地正向和反向遍历。

Linux 内核的双向链表本质上是环形的（circular）。这意味着，要从头节点到达尾节点，
我们只需向后走一条边；类似地，要从尾节点到达头节点，我们只需向前“越过”尾部，即可
回到头节点。

### 声明一个节点


双向链表中的节点，是通过在你想放入链表的数据结构中添加一个 struct list_head 成员来
声明的：


  struct clown {
          unsigned long long shoe_size;
          const char *name;
          struct list_head node;  /** the aforementioned member **/
  };

对一些人来说，这可能是一种陌生的做法，因为经典的对链表的解释是：链表节点数据结构
带有指向前一个和后一个链表节点的指针，以及负载（payload）数据。Linux 选择这种做法，
是因为它允许编写通用的链表修改代码，而无需关心链表中包含的是什么数据结构。由于
struct list_head 成员不是指针，而是数据结构本身的组成部分，链表实现可以使用
container_of() 模式来访问负载数据，而不管其类型是什么，同时对所言之类型究竟是什么
一无所知。

### 声明并初始化一个链表


然后，双向链表可以被声明为又一个 struct list_head，并在初始赋值时用 LIST_HEAD_INIT()
宏初始化，或者稍后用 INIT_LIST_HEAD() 函数初始化：


  struct clown_car {
          int tyre_pressure[^4^];
          struct list_head clowns;        /** Looks like a node! **/
  };

  /** ... Somewhere later in our driver ... **/

  static int circus_init(struct circus_priv *circus)
  {
          struct clown_car other_car = {
                .tyre_pressure = {10, 12, 11, 9},
                .clowns = LIST_HEAD_INIT(other_car.clowns)
          };

          INIT_LIST_HEAD(&circus->car.clowns);

          return 0;
  }

另一个可能让一些人困惑的点是，链表本身并没有自己的类型。整个链表的这一概念，与指向
链表中其他条目的 struct list_head 成员，二者是同一回事。

### 向链表中添加节点


向链表中添加节点是通过 list_add() 宏完成的。

我们将回到小丑车的例子，来说明节点是如何被添加到链表中的：


  static int circus_fill_car(struct circus_priv *circus)
  {
          struct clown_car *car = &circus->car;
          struct clown *grock;
          struct clown *dimitri;

          /** State 1 **/

          grock = kzalloc(sizeof(*grock), GFP_KERNEL);
          if (!grock)
                  return -ENOMEM;
          grock->name = "Grock";
          grock->shoe_size = 1000;

          /** Note that we're adding the "node" member **/
          list_add(&grock->node, &car->clowns);

          /** State 2 **/

          dimitri = kzalloc(sizeof(*dimitri), GFP_KERNEL);
          if (!dimitri)
                  return -ENOMEM;
          dimitri->name = "Dimitri";
          dimitri->shoe_size = 50;

          list_add(&dimitri->node, &car->clowns);

          /** State 3 **/

          return 0;
  }

```

         .------.
         v      |
    .--------.  |
    | clowns |--'
    '--------'

```
该图显示了单独的 "clowns" 节点指向自身。在本图以及之后所有图中，为了清晰起见，只画
出了正向的边。

```

         .--------------------.
         v                    |
    .--------.     .-------.  |
    | clowns |---->| Grock |--'
    '--------'     '-------'

```
该图显示了 "clowns" 节点指向一个标记为 "Grock" 的新节点。Grock 节点指回了 "clowns"
节点。

```

         .------------------------------------.
         v                                    |
    .--------.     .---------.     .-------.  |
    | clowns |---->| Dimitri |---->| Grock |--'
    '--------'     '---------'     '-------'

```
该图显示了 "clowns" 节点指向一个标记为 "Dimitri" 的新节点，而该节点又指向标记为
"Grock" 的节点。"Grock" 节点仍指回 "clowns" 节点。

如果我们想让 Dimitri 插到链表末尾，就应使用 list_add_tail()。我们的代码将如下所示：


  static int circus_fill_car(struct circus_priv *circus)
  {
          /** ... **/

          list_add_tail(&dimitri->node, &car->clowns);

          /** State 3b **/

          return 0;
  }

```

         .------------------------------------.
         v                                    |
    .--------.     .-------.     .---------.  |
    | clowns |---->| Grock |---->| Dimitri |--'
    '--------'     '-------'     '---------'

```
该图显示了 "clowns" 节点指向标记为 "Grock" 的节点，而后者又指向标记为 "Dimitri" 的新
节点。"Dimitri" 节点指回 "clowns" 节点。

### 遍历链表


要遍历链表，我们可以用 list_for_each() 循环经过链表中的所有节点。

在我们的小丑例子中，这会得出如下有些笨拙的代码：


  static unsigned long long circus_get_max_shoe_size(struct circus_priv *circus)
  {
          unsigned long long res = 0;
          struct clown *e;
          struct list_head *cur;

          list_for_each(cur, &circus->car.clowns) {
                  e = list_entry(cur, struct clown, node);
                  if (e->shoe_size > res)
                          res = e->shoe_size;
          }

          return res;
  }

list_entry() 宏在内部使用了前面提到的 container_of() 来取回 `node` 作为其成员的那个
数据结构实例。

注意这里额外的 list_entry() 调用有点笨拙。它之所以存在，是因为我们是在遍历 `node`
成员，但我们真正想要遍历的是负载，即包含每个节点的 struct list_head 的那个
`struct clown`。为此，提供了第二个宏：list_for_each_entry()

使用它会把我们的代码改成类似这样：


  static unsigned long long circus_get_max_shoe_size(struct circus_priv *circus)
  {
          unsigned long long res = 0;
          struct clown *e;

          list_for_each_entry(e, &circus->car.clowns, node) {
                  if (e->shoe_size > res)
                          res = e->shoe_size;
          }

          return res;
  }

这省去了 list_entry() 这一步，我们的循环游标现在是我们负载的类型。该宏被给定了在
clown 数据结构中对应于链表的 struct list_head 的那个成员名，这样它仍然能够遍历链表。

### 从链表中移除节点


list_del() 函数可用于从链表中移除条目。它不仅从链表中移除给定的条目，还会毒化
（poison）该条目的 `prev` 和 `next` 指针，使得移除后对该条目的无意使用不会被忽视。

我们可以扩展之前的例子来移除其中一个条目：


  static int circus_fill_car(struct circus_priv *circus)
  {
          /** ... **/

          list_add(&dimitri->node, &car->clowns);

          /** State 3 **/

          list_del(&dimitri->node);

          /** State 4 **/

          return 0;
  }

```

         .--------------------.
         v                    |
    .--------.     .-------.  |      .---------.
    | clowns |---->| Grock |--'      | Dimitri |
    '--------'     '-------'         '---------'

```
该图显示了 "clowns" 节点指向标记为 "Grock" 的节点，而后者又指回 "clowns" 节点。在一旁
是一个孤立的、标记为 "Dimitri" 的节点，没有任何箭头指向任何地方。

注意 Dimitri 节点是如何不指向自身的；它的指针被故意设为一个“毒化”值，链表代码拒绝
遍历它。

如果我们想改为重新初始化被移除的节点，使其再次像空的链表头一样指向自身，我们可以
改用 list_del_init()：


  static int circus_fill_car(struct circus_priv *circus)
  {
          /** ... **/

          list_add(&dimitri->node, &car->clowns);

          /** State 3 **/

          list_del_init(&dimitri->node);

          /** State 4b **/

          return 0;
  }

```

         .--------------------.           .-------.
         v                    |           v       |
    .--------.     .-------.  |      .---------.  |
    | clowns |---->| Grock |--'      | Dimitri |--'
    '--------'     '-------'         '---------'

```
该图显示了 "clowns" 节点指向标记为 "Grock" 的节点，而后者又指回 "clowns" 节点。在一旁
是一个孤立的、标记为 "Dimitri" 的节点，它指向自身。

### 在遍历的同时移除节点


如果我们使用 list_for_each() 和 list_for_each_entry()，在遍历链表的同时删除条目会
引发问题，因为删除当前条目会修改它的 `next` 指针，这意味着遍历无法正确地前进到
下一个链表条目。

不过有一个解决方案：list_for_each_safe() 和 list_for_each_entry_safe()。它们额外
接受一个指向 struct list_head 的指针作为参数，用作遍历期间下一个条目的临时存储，从而
解决该问题。

如何使用它的示例：


  static void circus_eject_insufficient_clowns(struct circus_priv *circus)
  {
          struct clown *e;
          struct clown **n;      /** temporary storage for safe iteration */

          list_for_each_entry_safe(e, n, &circus->car.clowns, node) {
                if (e->shoe_size < 500)
                        list_del(&e->node);
          }
  }

这种情况下，恰当的内存管理（即释放被删除的节点，同时确保没有任何东西仍在引用它）留作
练习给读者。

### 切割链表


有两个辅助函数可用于切割链表。二者都从链表 `head` 中取出元素，并替换链表 `list`
的内容。

第一个这样的函数是 list_cut_position()。它将 `head` 中直到并包括 `entry` 的所有链表
条目移除，转而从 `list` 中放置它们。

```

         .----------------------------------------------------------------.
         v                                                                |
    .--------.     .-------.     .---------.     .-----.     .---------.  |
    | clowns |---->| Grock |---->| Dimitri |---->| Pic |---->| Alfredo |--'
    '--------'     '-------'     '---------'     '-----'     '---------'

```
通过下面的代码，从 "clowns" 直到并包括 "Pic" 的每个小丑都被从 "clowns" 链表头移动到一个
单独的 struct list_head，该结构在局部栈变量 `retirement` 处初始化：


  static void circus_retire_clowns(struct circus_priv *circus)
  {
          struct list_head retirement = LIST_HEAD_INIT(retirement);
          struct clown **grock, **dimitri, **pic, **alfredo;
          struct clown_car *car = &circus->car;

          /** ... clown initialization, list adding ... **/

          list_cut_position(&retirement, &car->clowns, &pic->node);

          /** State 1 **/
  }

```

         .----------------------.
         v                      |
    .--------.     .---------.  |
    | clowns |---->| Alfredo |--'
    '--------'     '---------'

```
```

           .--------------------------------------------------.
           v                                                  |
    .------------.     .-------.     .---------.     .-----.  |
    | retirement |---->| Grock |---->| Dimitri |---->| Pic |--'
    '------------'     '-------'     '---------'     '-----'

```
第二个函数 list_cut_before() 基本一样，只是它在 `entry` 节点之前切割，即它移除 `head`
中直到但不包含 `entry` 的所有链表条目，转而从 `list` 中放置它们。此例假定与前面例子
相同的初始起始链表：


  static void circus_retire_clowns(struct circus_priv *circus)
  {
          struct list_head retirement = LIST_HEAD_INIT(retirement);
          struct clown **grock, **dimitri, **pic, **alfredo;
          struct clown_car *car = &circus->car;

          /** ... clown initialization, list adding ... **/

          list_cut_before(&retirement, &car->clowns, &pic->node);

          /** State 1b **/
  }

```

         .----------------------------------.
         v                                  |
    .--------.     .-----.     .---------.  |
    | clowns |---->| Pic |---->| Alfredo |--'
    '--------'     '-----'     '---------'

```
```

           .--------------------------------------.
           v                                      |
    .------------.     .-------.     .---------.  |
    | retirement |---->| Grock |---->| Dimitri |--'
    '------------'     '-------'     '---------'

```
应当注意，这两个函数都会销毁到目标 `struct list_head *list` 中任何现存节点的链接。

### 移动条目与部分链表


list_move() 和 list_move_tail() 函数可用于将一个条目从一个链表移动到另一个链表，
分别移动到起始或末尾。

在下面的例子中，我们假设从两个链表开始（“clowns” 与
```

         .----------------------------------------------------------------.
         v                                                                |
    .--------.     .-------.     .---------.     .-----.     .---------.  |
    | clowns |---->| Grock |---->| Dimitri |---->| Pic |---->| Alfredo |--'
    '--------'     '-------'     '---------'     '-----'     '---------'

          .-------------------.
          v                   |
    .----------.     .-----.  |
    | sidewalk |---->| Pio |--'
    '----------'     '-----'

```
我们将下面的示例代码应用于这两个链表：


  static void circus_clowns_exit_car(struct circus_priv *circus)
  {
          struct list_head sidewalk = LIST_HEAD_INIT(sidewalk);
          struct clown **grock, **dimitri, **pic, **alfredo, *pio;
          struct clown_car *car = &circus->car;

          /** ... clown initialization, list adding ... **/

          /** State 0 **/

          list_move(&pic->node, &sidewalk);

          /** State 1 **/

          list_move_tail(&dimitri->node, &sidewalk);

          /** State 2 **/
  }

```

        .-----------------------------------------------------.
        |                                                     |
        v                                                     |
    .--------.     .-------.     .---------.     .---------.  |
    | clowns |---->| Grock |---->| Dimitri |---->| Alfredo |--'
    '--------'     '-------'     '---------'     '---------'

          .-------------------------------.
          v                               |
    .----------.     .-----.     .-----.  |
    | sidewalk |---->| Pic |---->| Pio |--'
    '----------'     '-----'     '-----'

```
在 State 2 中，在我们将 Dimitri 移动到 sidewalk 的末尾之后，情形变为
```

        .-------------------------------------.
        |                                     |
        v                                     |
    .--------.     .-------.     .---------.  |
    | clowns |---->| Grock |---->| Alfredo |--'
    '--------'     '-------'     '---------'

          .-----------------------------------------------.
          v                                               |
    .----------.     .-----.     .-----.     .---------.  |
    | sidewalk |---->| Pic |---->| Pio |---->| Dimitri |--'
    '----------'     '-----'     '-----'     '---------'

```
只要源链表头与目标链表头属于同一个链表，我们还可以高效地将链表的一段批量移动到链表的
尾端。我们在前一个例子的基础上，在 State 2 之后添加一次 list_bulk_move_tail()，将 Pic
和 Pio 移动到 sidewalk 链表的尾端。


  static void circus_clowns_exit_car(struct circus_priv *circus)
  {
          struct list_head sidewalk = LIST_HEAD_INIT(sidewalk);
          struct clown **grock, **dimitri, **pic, **alfredo, *pio;
          struct clown_car *car = &circus->car;

          /** ... clown initialization, list adding ... **/

          /** State 0 **/

          list_move(&pic->node, &sidewalk);

          /** State 1 **/

          list_move_tail(&dimitri->node, &sidewalk);

          /** State 2 **/

          list_bulk_move_tail(&sidewalk, &pic->node, &pio->node);

          /** State 3 **/
  }

为简洁起见，只描绘了 State 3 中发生变化的 "sidewalk" 链表
```

          .-----------------------------------------------.
          v                                               |
    .----------.     .---------.     .-----.     .-----.  |
    | sidewalk |---->| Dimitri |---->| Pic |---->| Pio |--'
    '----------'     '---------'     '-----'     '-----'

```
请注意，list_bulk_move_tail() 不会检查所给出的三个 `struct list_head *` 参数是否真的
都属于同一个链表。如果你在文档给出的约束之外使用它，那么结果就是你与实现之间的事情了。

### 旋转条目


链表上一种常见的写操作，尤其是在将其用作队列时，是将其旋转（rotate）。链表旋转意味着
前端的条目被送到后端。

对于旋转，Linux 为我们提供了两个函数：list_rotate_left() 和 list_rotate_to_front()。
前者可以被想象成自行车链条，取所给出的 `struct list_head *` 之后的那个条目并将其移动到
尾部，由于链表的环形本质，这实质上意味着整个链表旋转了一个位置。

后者 list_rotate_to_front() 将同一概念推进一步：它不是让链表前进一个条目，而是前进
**直到**指定的条目成为新的前端。

```

         .-----------------------------------------------------------------.
         v                                                                 |
    .--------.   .-------.   .---------.   .-----.   .---------.   .-----. |
    | clowns |-->| Grock |-->| Dimitri |-->| Pic |-->| Alfredo |-->| Pio |-'
    '--------'   '-------'   '---------'   '-----'   '---------'   '-----'

```
用于演示链表旋转的示例代码如下：


  static void circus_clowns_rotate(struct circus_priv *circus)
  {
          struct clown **grock, **dimitri, **pic, **alfredo, *pio;
          struct clown_car *car = &circus->car;

          /** ... clown initialization, list adding ... **/

          /** State 0 **/

          list_rotate_left(&car->clowns);

          /** State 1 **/

          list_rotate_to_front(&alfredo->node, &car->clowns);

          /** State 2 **/

  }

```

         .-----------------------------------------------------------------.
         v                                                                 |
    .--------.   .---------.   .-----.   .---------.   .-----.   .-------. |
    | clowns |-->| Dimitri |-->| Pic |-->| Alfredo |-->| Pio |-->| Grock |-'
    '--------'   '---------'   '-----'   '---------'   '-----'   '-------'

```
接下来，在 list_rotate_to_front() 调用之后，我们到达如下状态
```

         .-----------------------------------------------------------------.
         v                                                                 |
    .--------.   .---------.   .-----.   .-------.   .---------.   .-----. |
    | clowns |-->| Alfredo |-->| Pio |-->| Grock |-->| Dimitri |-->| Pic |-'
    '--------'   '---------'   '-----'   '-------'   '---------'   '-----'

```
希望从图中可以明显看出，"Alfredo" 之前的条目被循环到了链表的尾端。

### 交换条目


另一个常见操作是两个条目需要彼此交换。

为此，Linux 为我们提供了 list_swap()。

在下面的例子中，我们有一个包含三个条目的链表，并交换其中两个
```

         .-----------------------------------------.
         v                                         |
    .--------.   .-------.   .---------.   .-----. |
    | clowns |-->| Grock |-->| Dimitri |-->| Pic |-'
    '--------'   '-------'   '---------'   '-----'

```


  static void circus_clowns_swap(struct circus_priv *circus)
  {
          struct clown **grock, **dimitri, *pic;
          struct clown_car *car = &circus->car;

          /** ... clown initialization, list adding ... **/

          /** State 0 **/

          list_swap(&dimitri->node, &pic->node);

          /** State 1 **/
  }

```

         .-----------------------------------------.
         v                                         |
    .--------.   .-------.   .-----.   .---------. |
    | clowns |-->| Grock |-->| Pic |-->| Dimitri |-'
    '--------'   '-------'   '-----'   '---------'

```
对比两图可以明显看出，"Pic" 和 "Dimitri" 节点交换了位置。

### 将两个链表拼接在一起


假设我们有两个链表，在下面的例子中，一个由我们称为 "knie" 的链表头表示，另一个由我们
称为 "stey" 的链表头表示。在一次假想的马戏团收购中，这两组小丑应当被拼接在一起。我们
的
```

        .-----------------------------------------.
        |                                         |
        v                                         |
    .------.   .-------.   .---------.   .-----.  |
    | knie |-->| Grock |-->| Dimitri |-->| Pic |--'
    '------'   '-------'   '---------'   '-----'

        .-----------------------------.
        v                             |
    .------.   .---------.   .-----.  |
    | stey |-->| Alfredo |-->| Pio |--'
    '------'   '---------'   '-----'

```
将这两个链表拼接在一起的函数是 list_splice()。我们的示例代码如下：


  static void circus_clowns_splice(void)
  {
          struct clown **grock, **dimitri, **pic, **alfredo, *pio;
          struct list_head knie = LIST_HEAD_INIT(knie);
          struct list_head stey = LIST_HEAD_INIT(stey);

          /** ... Clown allocation and initialization here ... **/

          list_add_tail(&grock->node, &knie);
          list_add_tail(&dimitri->node, &knie);
          list_add_tail(&pic->node, &knie);
          list_add_tail(&alfredo->node, &stey);
          list_add_tail(&pio->node, &stey);

          /** State 0 **/

          list_splice(&stey, &dimitri->node);

          /** State 1 **/
  }

这里的 list_splice() 调用将 `stey` 中的所有条目添加到 `dimitri` 的 `node` 链表头所在
的链表中，位于 `dimitri` 的 `node` 之后。一个
```

        .-----------------------------------------------------------------.
        |                                                                 |
        v                                                                 |
    .------.   .-------.   .---------.   .---------.   .-----.   .-----.  |
    | knie |-->| Grock |-->| Dimitri |-->| Alfredo |-->| Pio |-->| Pic |--'
    '------'   '-------'   '---------'   '---------'   '-----'   '-----'
                                              ^
              .-------------------------------'
              |
    .------.  |
    | stey |--'
    '------'

```
遍历 `stey` 链表不再产生正确的行为。在 `stey` 上调用 list_for_each() 会导致无限循环，
因为它永远不会回到 `stey` 链表头。

这是因为 list_splice() 没有重新初始化它取走条目的那个链表头，使其指针指向了现在已是
不同的链表的位置。

如果我们想避免这种情况，可以使用 list_splice_init()。它做与 list_splice() 相同的事，
只是会在移植之后重新初始化那个供体链表头。

### 并发考量


在大多数情况下，对链表的并发访问与修改需要用锁来保护。另一种（也是更可取的）方式是，
在“读多写少”的使用场景下，对链表使用 RCU 原语，即读取链表很常见、但修改链表较少的情形。
更多细节参见 Documentation/RCU/listRCU.rst。

### 延伸阅读


- `How does the kernel implements Linked Lists? - KernelNewbies <https://kernelnewbies.org/FAQ/LinkedLists>`_

## 完整链表 API


   :internal:

## 私有链表 API


   :doc: Private List Primitives

   :internal:
