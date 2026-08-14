
## 最小堆 API（Min Heap API）


:Author: Kuan-Wei Chiu <visitorckw@gmail.com>

## 简介


最小堆（Min Heap）API 提供了一组函数和宏，用于在 Linux 内核中管理最小堆。最小堆是一种
二叉树结构，其中每个节点的值都小于或等于其子节点的值，从而保证最小的元素始终位于根节点。

本文档提供了最小堆 API 的使用指南，详述如何定义和使用最小堆。用户不应直接调用带有
**__min_heap_*()** 前缀的函数，而应使用所提供的宏封装（macro wrappers）。

除了这些函数的标准版本外，该 API 还包含一组 inline 版本，用于性能敏感的场合。这些 inline
函数的名称与其非 inline 对应版本相同，但带有 **_inline** 后缀。例如
**__min_heap_init_inline** 及其对应的宏封装 **min_heap_init_inline**。inline 版本允许
直接调用自定义的比较和交换函数，而不经过间接函数调用。这可以显著减少开销，尤其是在启用
CONFIG_MITIGATION_RETPOLINE 时，因为间接函数调用会变得更加昂贵。与非 inline 版本一样，
重要的是对 inline 函数也要使用宏封装，而不是直接调用函数本身。

## 数据结构


### 最小堆的定义


表示最小堆的核心数据结构使用 **MIN_HEAP_PREALLOCATED** 和 **DEFINE_MIN_HEAP** 宏来定义。
这些宏允许你定义一个带有预分配缓冲区或动态分配内存的最小堆。

示例：


    #define MIN_HEAP_PREALLOCATED(_type, _name, _nr)
    struct _name {
        size_t nr;         /** 堆中元素的数量 **/
        size_t size;       /** 可容纳的最大元素数量 **/
        _type **data;    /** 指向堆数据的指针 */
        _type preallocated[_nr];  /** 静态预分配数组 **/
    }

    #define DEFINE_MIN_HEAP(_type, _name) MIN_HEAP_PREALLOCATED(_type, _name, 0)

一个典型的堆结构会包含一个元素计数（`nr`）、堆的最大容量（`size`），以及一个指向元素数组
的指针（`data`）。可选地，你可以使用 **MIN_HEAP_PREALLOCATED** 指定一个静态数组用于堆的
预分配存储。

### 最小堆回调


**struct min_heap_callbacks** 提供了用于堆中元素排序与交换的自定义选项。它包含两个函数
指针：


    struct min_heap_callbacks {
        bool (**less)(const void **lhs, const void **rhs, void **args);
        void (**swp)(void **lhs, void **rhs, void **args);
    };

- **less** 是用于确定元素顺序的比较函数。
- **swp** 是用于交换堆中元素的函数。如果 swp 设为 NULL，则将使用默认的交换函数，该函数
  根据元素大小进行交换。

## 宏封装


为了以用户友好的方式与堆交互，提供了以下宏封装。每个宏对应一个操作堆的函数，它们屏蔽了
对内部函数的直接调用。

每个宏接受若干参数，详情如下。

### 堆初始化



    min_heap_init(heap, data, size);

- **heap**：指向待初始化的堆结构的指针。
- **data**：指向用于存储堆元素的缓冲区的指针。如果为 `NULL`，则使用堆结构内的预分配
  缓冲区。
- **size**：堆可容纳的最大元素数量。

该宏初始化堆，设置其初始状态。如果 `data` 为 `NULL`，则使用堆结构内的预分配内存进行
存储；否则使用用户提供的缓冲区。该操作复杂度为 **O(1)**。

**Inline 版本：** min_heap_init_inline(heap, data, size)

### 访问堆顶元素



    element = min_heap_peek(heap);

- **heap**：指向从中获取最小元素的堆的指针。

该宏返回指向堆中最小元素（根节点）的指针，如果堆为空则返回 `NULL`。该操作复杂度为 **O(1)**。

**Inline 版本：** min_heap_peek_inline(heap)

### 堆插入



    success = min_heap_push(heap, element, callbacks, args);

- **heap**：指向要插入元素的堆的指针。
- **element**：指向要插入堆中的元素的指针。
- **callbacks**：指向 `struct min_heap_callbacks` 的指针，提供 `less` 和 `swp` 函数。
- **args**：传递给 `less` 和 `swp` 函数的可选参数。

该宏将一个元素插入堆中。如果插入成功返回 `true`，如果堆已满则返回 `false`。该操作复杂度为
**O(log n)**。

**Inline 版本：** min_heap_push_inline(heap, element, callbacks, args)

### 堆删除



    success = min_heap_pop(heap, callbacks, args);

- **heap**：指向要从中删除最小元素的堆的指针。
- **callbacks**：指向 `struct min_heap_callbacks` 的指针，提供 `less` 和 `swp` 函数。
- **args**：传递给 `less` 和 `swp` 函数的可选参数。

该宏从堆中删除最小元素（根节点）。如果元素被成功删除返回 `true`，如果堆为空则返回 `false`。
该操作复杂度为 **O(log n)**。

**Inline 版本：** min_heap_pop_inline(heap, callbacks, args)

### 堆维护


你可以使用以下宏来维护堆的结构：


    min_heap_sift_down(heap, pos, callbacks, args);

- **heap**：指向堆的指针。
- **pos**：开始向下筛选（sift down）的索引。
- **callbacks**：指向 `struct min_heap_callbacks` 的指针，提供 `less` 和 `swp` 函数。
- **args**：传递给 `less` 和 `swp` 函数的可选参数。

该宏通过将指定索引（`pos`）处的元素沿堆向下移动，直到它处于正确位置，从而恢复堆性质。
该操作复杂度为 **O(log n)**。

**Inline 版本：** min_heap_sift_down_inline(heap, pos, callbacks, args)


    min_heap_sift_up(heap, idx, callbacks, args);

- **heap**：指向堆的指针。
- **idx**：要向上筛选的元素的索引。
- **callbacks**：指向 `struct min_heap_callbacks` 的指针，提供 `less` 和 `swp` 函数。
- **args**：传递给 `less` 和 `swp` 函数的可选参数。

该宏通过将指定索引（`idx`）处的元素沿堆向上移动，从而恢复堆性质。该操作复杂度为 **O(log n)**。

**Inline 版本：** min_heap_sift_up_inline(heap, idx, callbacks, args)


    min_heapify_all(heap, callbacks, args);

- **heap**：指向堆的指针。
- **callbacks**：指向 `struct min_heap_callbacks` 的指针，提供 `less` 和 `swp` 函数。
- **args**：传递给 `less` 和 `swp` 函数的可选参数。

该宏确保整个堆满足堆性质。它在堆从头构建或经过多次修改后被调用。该操作复杂度为 **O(n)**。

**Inline 版本：** min_heapify_all_inline(heap, callbacks, args)

### 删除特定元素



    success = min_heap_del(heap, idx, callbacks, args);

- **heap**：指向堆的指针。
- **idx**：要删除的元素的索引。
- **callbacks**：指向 `struct min_heap_callbacks` 的指针，提供 `less` 和 `swp` 函数。
- **args**：传递给 `less` 和 `swp` 函数的可选参数。

该宏从堆中删除指定索引（`idx`）处的元素并恢复堆性质。该操作复杂度为 **O(log n)**。

**Inline 版本：** min_heap_del_inline(heap, idx, callbacks, args)

## 其他工具


- **min_heap_full(heap)**：检查堆是否已满。复杂度：**O(1)**。


    bool full = min_heap_full(heap);

- `heap`：指向要检查的堆的指针。

该宏在堆已满时返回 `true`，否则返回 `false`。

**Inline 版本：** min_heap_full_inline(heap)

- **min_heap_empty(heap)**：检查堆是否为空。复杂度：**O(1)**。


    bool empty = min_heap_empty(heap);

- `heap`：指向要检查的堆的指针。

该宏在堆为空时返回 `true`，否则返回 `false`。

**Inline 版本：** min_heap_empty_inline(heap)

## 示例用法


最小堆 API 的典型用法包括定义堆结构、初始化它，以及按需插入和删除元素。


    #include <linux/min_heap.h>

    int my_less_function(const void **lhs, const void **rhs, void *args) {
        return (**(int **)lhs < **(int **)rhs);
    }

    struct min_heap_callbacks heap_cb = {
        .less = my_less_function,    /** 用于堆顺序的比较函数 **/
        .swp  = NULL,                /** 使用默认交换函数 **/
    };

    void example_usage(void) {
        /** 用元素预填充缓冲区 **/
        int buffer[^5^] = {5, 2, 8, 1, 3};
        /** 声明一个最小堆 **/
        DEFINE_MIN_HEAP(int, my_heap);

        /** 用预分配缓冲区和大小初始化堆 **/
        min_heap_init(&my_heap, buffer, 5);

        /** 使用 min_heapify_all 构建堆 **/
        my_heap.nr = 5;  /** 设置堆中元素的数量 **/
        min_heapify_all(&my_heap, &heap_cb, NULL);

        /** 查看堆顶元素（本例中应为 1） **/
        int *top = min_heap_peek(&my_heap);
        pr_info("Top element: %d\n", *top);

        /** 弹出堆顶元素（1）并获取新的堆顶（2） **/
        min_heap_pop(&my_heap, &heap_cb, NULL);
        top = min_heap_peek(&my_heap);
        pr_info("New top element: %d\n", *top);

        /** 插入一个新元素（0）并重新检查堆顶 **/
        int new_element = 0;
        min_heap_push(&my_heap, &new_element, &heap_cb, NULL);
        top = min_heap_peek(&my_heap);
        pr_info("Top element after insertion: %d\n", *top);
    }
