## 通用关联数组实现


## 概述


该关联数组实现是一个对象容器，具有以下特性：

1. 对象是一些不透明指针。该实现并不关心它们指向何处（如果有指向的话）或指向什么（如果有指向内容的话）。

```

      Pointers to objects _must_ be zero in the least significant bit.

```
2. 对象无需包含供数组使用的链接块。这使得一个对象可以同时存在于多个数组中。相反，该数组由指向对象的元数据块组成。

3. 对象需要索引键来在数组内定位。

4. 索引键必须唯一。插入一个与数组中已有对象键相同的对象时，将替换旧对象。

5. 索引键可以是任意长度，也可以各不相同。

6. 索引键应在开头尽早编码其长度，在任何因长度造成的差异出现之前。

7. 索引键可以包含哈希值，以便将对象分散到整个数组中。

8. 该数组可以遍历。对象的输出顺序不一定与键序一致。

9. 只要迭代器持有 RCU 读锁，就可以在数组被修改的同时遍历它。但请注意，在这种情况下，某些对象可能会被看到多次。如果这是个问题，迭代器应当加锁以阻止修改。不过，除非对象被删除，否则不会被遗漏。

10. 数组中的对象可以通过其索引键进行查找。

11. 只要执行查找的线程持有 RCU 读锁，就可以在数组被修改的同时查找对象。

该实现在内部使用一棵由 16 指针节点组成的树，每一层都像基数树（radix tree）那样由索引键中的半字节（nibble）进行索引。为了提高内存效率，可以插入快捷方式（shortcut）以跳过原本会是一系列单占用（single-occupancy）节点的部分。此外，节点会将叶子对象指针打包进节点的空闲空间中，而不是立即额外开分支，直到确实需要向一个已满节点添加对象时为止。


## 公共 API


公共 API 可以在 `<linux/assoc_array.h>` 中找到。该关联

```
      struct assoc_array {
              ...
      };

```
```
      ./script/config -e ASSOCIATIVE_ARRAY


```
### 编辑脚本


插入和删除函数会产生一个“编辑脚本”（edit script），稍后可被应用以实施变更，而不会有 `ENOMEM` 风险。这会保留将安装到内部树中的预分配元数据块，并跟踪在应用脚本时将从树中移除的元数据块。

这也用于在脚本应用之后跟踪失效（dead）块和失效对象，以便它们稍后被释放。释放是在经过一个 RCU 宽限期之后进行的——从而允许访问函数在 RCU 读锁下继续执行。

```

    struct assoc_array_edit;

```
有两个函数用于处理该脚本：

```

    void assoc_array_apply_edit(struct assoc_array_edit *edit);

   This will perform the edit functions, interpolating various write barriers
   to permit accesses under the RCU read lock to continue.  The edit script
   will then be passed to ``call_rcu()`` to free it and any dead stuff it
   points to.

```
```

    void assoc_array_cancel_edit(struct assoc_array_edit *edit);

   This frees the edit script and all preallocated memory immediately. If
   this was for insertion, the new object is *not* released by this function,
   but must rather be released by the caller.

```
这些函数是保证不会失败的。


### 操作表


```

    struct assoc_array_ops {
            ...
    };

```
这指向若干方法，它们都需要被提供：

```

    unsigned long (*get_key_chunk)(const void *index_key, int level);

   This should return a chunk of caller-supplied index key starting at the
   *bit* position given by the level argument.  The level argument will be a
   multiple of ``ASSOC_ARRAY_KEY_CHUNK_SIZE`` and the function should return
   ``ASSOC_ARRAY_KEY_CHUNK_SIZE bits``.  No error is possible.


```
```

    unsigned long (*get_object_key_chunk)(const void *object, int level);

   As the previous function, but gets its data from an object in the array
   rather than from a caller-supplied index key.


```
```

    bool (*compare_object)(const void *object, const void *index_key);

   Compare the object against an index key and return ``true`` if it matches
   and ``false`` if it doesn't.


```
```

    int (*diff_objects)(const void *object, const void *index_key);

   Return the bit position at which the index key of the specified object
   differs from the given index key or -1 if they are the same.


```
```

    void (*free_object)(void *object);

   Free the specified object.  Note that this may be called an RCU grace period
   after ``assoc_array_apply_edit()`` was called, so ``synchronize_rcu()`` may
   be necessary on module unloading.


```
### 操作函数


有许多函数用于操作关联数组：

```

    void assoc_array_init(struct assoc_array *array);

   This initialises the base structure for an associative array.  It can't fail.


```
```

    struct assoc_array_edit *
    assoc_array_insert(struct assoc_array *array,
                       const struct assoc_array_ops *ops,
                       const void *index_key,
                       void *object);

   This inserts the given object into the array.  Note that the least
   significant bit of the pointer must be zero as it's used to type-mark
   pointers internally.

   If an object already exists for that key then it will be replaced with the
   new object and the old one will be freed automatically.

   The ``index_key`` argument should hold index key information and is
   passed to the methods in the ops table when they are called.

   This function makes no alteration to the array itself, but rather returns
   an edit script that must be applied.  ``-ENOMEM`` is returned in the case of
   an out-of-memory error.

   The caller should lock exclusively against other modifiers of the array.


```
```

    struct assoc_array_edit *
    assoc_array_delete(struct assoc_array *array,
                       const struct assoc_array_ops *ops,
                       const void *index_key);

   This deletes an object that matches the specified data from the array.

   The ``index_key`` argument should hold index key information and is
   passed to the methods in the ops table when they are called.

   This function makes no alteration to the array itself, but rather returns
   an edit script that must be applied.  ``-ENOMEM`` is returned in the case of
   an out-of-memory error.  ``NULL`` will be returned if the specified object
   is not found within the array.

```
调用者应当针对数组的其他修改者加排他锁。


```

    struct assoc_array_edit *
    assoc_array_clear(struct assoc_array *array,
                      const struct assoc_array_ops *ops);

   This deletes all the objects from an associative array and leaves it
   completely empty.

   This function makes no alteration to the array itself, but rather returns
   an edit script that must be applied.  ``-ENOMEM`` is returned in the case of
   an out-of-memory error.

   The caller should lock exclusively against other modifiers of the array.


```
```

    void assoc_array_destroy(struct assoc_array *array,
                             const struct assoc_array_ops *ops);

   This destroys the contents of the associative array and leaves it
   completely empty.  It is not permitted for another thread to be traversing
   the array under the RCU read lock at the same time as this function is
   destroying it as no RCU deferral is performed on memory release -
   something that would require memory to be allocated.

   The caller should lock exclusively against other modifiers and accessors
   of the array.


```
```

    int assoc_array_gc(struct assoc_array *array,
                       const struct assoc_array_ops *ops,
                       bool (*iterator)(void *object, void *iterator_data),
                       void *iterator_data);

   This iterates over the objects in an associative array and passes each one
   to ``iterator()``.  If ``iterator()`` returns ``true``, the object is kept.
   If it returns ``false``, the object will be freed.  If the ``iterator()``
   function returns ``true``, it must perform any appropriate refcount
   incrementing on the object before returning.

   The internal tree will be packed down if possible as part of the iteration
   to reduce the number of nodes in it.

   The ``iterator_data`` is passed directly to ``iterator()`` and is otherwise
   ignored by the function.

   The function will return ``0`` if successful and ``-ENOMEM`` if there wasn't
   enough memory.

   It is possible for other threads to iterate over or search the array under
   the RCU read lock while this function is in progress.  The caller should
   lock exclusively against other modifiers of the array.


```
### 访问函数


有两个函数用于访问关联数组：

```

    int assoc_array_iterate(const struct assoc_array *array,
                            int (*iterator)(const void *object,
                                            void *iterator_data),
                            void *iterator_data);

   This passes each object in the array to the iterator callback function.
   ``iterator_data`` is private data for that function.

   This may be used on an array at the same time as the array is being
   modified, provided the RCU read lock is held.  Under such circumstances,
   it is possible for the iteration function to see some objects twice.  If
   this is a problem, then modification should be locked against.  The
   iteration algorithm should not, however, miss any objects.

   The function will return ``0`` if no objects were in the array or else it
   will return the result of the last iterator function called.  Iteration
   stops immediately if any call to the iteration function results in a
   non-zero return.


```
```

    void *assoc_array_find(const struct assoc_array *array,
                           const struct assoc_array_ops *ops,
                           const void *index_key);

   This walks through the array's internal tree directly to the object
   specified by the index key.

   This may be used on an array at the same time as the array is being
   modified, provided the RCU read lock is held.

   The function will return the object if found (and set ``*_type`` to the
   object type) or will return ``NULL`` if the object was not found.


```
### 索引键形式


索引键可以采用任何形式，但由于算法并不知道键有多长，强烈建议索引键在开头尽早包含其长度，在任何因长度造成的差异会对比较产生影响之前。

这会使具有不同长度键的叶子彼此分散开，而具有相同长度键的叶子聚集在一起。

还建议索引键以其余部分的哈希开头，以最大化在整个键空间中的分散程度。

分散越好，内部树就越宽、越矮。

分散不佳也不是太大的问题，因为存在快捷方式，且节点可以包含叶子与元数据指针的混合。

索引键以机器字为单位读取。每个字被细分为每层一个半字节（4 位），因此在 32 位 CPU 上这适用于 8 层，在 64 位 CPU 上适用于 16 层。除非分散确实很差，否则不太可能需要用到某个特定索引键的多于一个字。


## 内部工作机制


关联数组数据结构具有一棵内部树。这棵树由两类元数据块构成：节点（node）和快捷方式（shortcut）。

节点是一个槽位（slot）数组。每个槽位可以包含以下四类内容之一：

- 一个 NULL 指针，表示该槽位为空。
- 一个指向对象（叶子）的指针。
- 一个指向下一层节点的指针。
- 一个指向快捷方式的指针。


### 基本内部树布局


暂时忽略快捷方式，节点构成一棵多层树。索引键空间被树中的节点严格细分，节点出现在

```

 Level: 0               1               2               3
        =============== =============== =============== ===============
                                                        NODE D
                        NODE B          NODE C  +------>+---+
                +------>+---+   +------>+---+   |       | 0 |
        NODE A  |       | 0 |   |       | 0 |   |       +---+
        +---+   |       +---+   |       +---+   |       :   :
        | 0 |   |       :   :   |       :   :   |       +---+
        +---+   |       +---+   |       +---+   |       | f |
        | 1 |---+       | 3 |---+       | 7 |---+       +---+
        +---+           +---+           +---+
        :   :           :   :           | 8 |---+
        +---+           +---+           +---+   |       NODE E
        | e |---+       | f |           :   :   +------>+---+
        +---+   |       +---+           +---+           | 0 |
        | f |   |                       | f |           +---+
        +---+   |                       +---+           :   :
                |       NODE F                          +---+
                +------>+---+                           | f |
                        | 0 |           NODE G          +---+
                        +---+   +------>+---+
                        :   :   |       | 0 |
                        +---+   |       +---+
                        | 6 |---+       :   :
                        +---+           +---+
                        :   :           | f |
                        +---+           +---+
                        | f |
                        +---+

```
在上述示例中，有 7 个节点（A-G），每个有 16 个槽位（0-f）。假设树中没有其他元数据节点，键空间划分如下：

    ===========     ====
    KEY PREFIX      NODE
    ===========     ====
    137*            D
    138*            E
    13[0-69-f]*     C
    1[0-24-f]*      B
    e6*             G
    e[0-57-f]*      F
    [02-df]*        A
    ===========     ====

因此，例如，具有以下示例索引键的键将出现在相应的节点中：

    =============== ======= ====
    INDEX KEY       PREFIX  NODE
    =============== ======= ====
    13694892892489  13      C
    13795289025897  137     D
    13889dde88793   138     E
    138bbb89003093  138     E
    1394879524789   12      C
    1458952489      1       B
    9431809de993ba  \-      A
    b4542910809cd   \-      A
    e5284310def98   e       F
    e68428974237    e6      G
    e7fffcbd443     e       F
    f3842239082     \-      A
    =============== ======= ====

为了节省内存，如果一个节点能够容纳其键空间部分内的所有叶子，那么该节点将包含所有这些叶子，并且不会有任何元数据指针——即使其中某些叶子本应位于同一个槽位中。

一个节点可以包含叶子与元数据指针的异构混合。元数据指针必须位于与其键空间细分相匹配的槽位中。叶子可以位于任何未被元数据指针占用的槽位中。保证节点中没有叶子会与元数据指针占用的槽位匹配。如果元数据指针存在，那么任何键与元数据键前缀匹配的对象叶子都必须位于该元数据指针所指向的子树中。

在上述索引键列表示例中，节点 A 将包含：

    ====    =============== ==================
    SLOT    CONTENT         INDEX KEY (PREFIX)
    ====    =============== ==================
    1       PTR TO NODE B   1*
    any     LEAF            9431809de993ba
    any     LEAF            b4542910809cd
    e       PTR TO NODE F   e*
    any     LEAF            f3842239082
    ====    =============== ==================

以及节点 B：

    ====    =============== ==================
    SLOT    CONTENT         INDEX KEY (PREFIX)
    ====    =============== ==================
    3       PTR TO NODE C   13*
    any     LEAF            1458952489
    ====    =============== ==================


### 快捷方式


快捷方式是跳过一段键空间的元数据记录。快捷方式是一系列顺着层级上升的单占用节点的替代。快捷方式的存在是为了节省内存并加速遍历。

树的根节点有可能是一个快捷方式——例如，假设树中包含至少 17 个键前缀均为 `1111` 的节点。插入算法将插入一个快捷方式，一次性跳过 `1111` 键空间，直达这些节点实际产生差异的第四层。


### 拆分与合并节点


每个节点的最大容量为 16 个叶子和元数据指针。如果插入算法发现它正试图向一个节点中插入第 17 个对象，那么该节点将被拆分，使得至少两个在该层具有共同键段（key segment）的叶子最终进入一个以该共同键段槽位为根的新节点。

如果已满节点中的叶子以及正在插入的叶子足够相似，那么将在树中插入一个快捷方式。

当以某个节点为根的子树中对象数量降至 16 个或更少时，该子树将被合并（collapse）为单个节点——并且如果可能，这会向根节点方向传播。


### 非递归迭代


每个节点和快捷方式都包含一个指向其父节点的反向指针，以及在父节点中指向它的槽位号。非递归迭代利用这些指针自底向上（rootwards）地遍历树，转到父节点、槽位 N + 1，以确保无需栈即可取得进展。

然而，这些反向指针使得同时修改与迭代变得棘手。


### 同时修改与迭代


有若干种情况需要考虑：

1. 简单插入/替换。这只需在屏障（barrier）之后，用一个指向新叶子的指针替换 NULL 或旧的匹配叶子指针。除此之外元数据块不会改变。旧叶子在 RCU 宽限期之后才会被释放。

2. 简单删除。这只涉及清除一个旧的匹配叶子。除此之外元数据块不会改变。旧叶子在 RCU 宽限期之后才会被释放。

3. 插入替换了我们尚未进入的子树的一部分。这可能涉及替换该子树的一部分——但这不会影响迭代，因为我们尚未到达指向它的指针，且祖先块不会被替换（那些块的布局不改变）。

4. 插入替换我们正在主动处理的节点。这不是问题，因为我们已经越过了锚定指针，并且在沿反向指针回溯之前不会切换到新布局——而在那个时候，我们已经检查了被替换节点中的叶子（我们在跟随任何元数据指针之前会先遍历节点中的所有叶子）。

   然而，我们可能会再次看到一些被拆分到新分支中的叶子，该分支位于我们当时所处位置之后的某个槽位中。

5. 插入替换我们正在处理其依赖分支的节点。这在我们沿反向指针回溯之前不会影响我们。与（4）类似。

6. 删除合并我们下方的分支。这不会影响我们，因为反向指针会使我们在看到新节点之前就回到新节点的父节点。整个被合并的子树原样被丢弃——并且仍将以同一个槽位为根，因此我们在回到槽位 + 1 时不应再次处理它。


   在某些情况下，我们需要同时改变节点的父指针和父槽位指针（例如，我们在它之前插入了另一个节点并将其上移了一层）。不加锁地读就无法做到这一点——因此我们也必须替换该节点。

   然而，当我们将一个快捷方式改为节点时，这不是问题，因为快捷方式只有一个槽位，因此在沿其反向遍历时不会用到父槽位号。这意味着可以先改变槽位号——只要使用合适的屏障来确保父槽位号是在读取反向指针之后才被读取的。

失效的块和叶子在经过一个 RCU 宽限期之后才会被释放，因此只要任何执行遍历或迭代的人持有 RCU 读锁，旧的支撑结构就不应当从它们下面消失。
