## BPF 迭代

### 概述


BPF 支持两个合称为「BPF iterators」（BPF 迭代器）的独立实体：BPF 迭代*程序类型**（program type）与**开放编*（open-coded）的 BPF 迭代器。前者是一种独立的 BPF 程序类型，当用户将其附加并激活后，会对每个被迭代的实体（task_struct、cgroup 等）调用一次。后者是一组实现迭代器功能BPF API，可在多BPF 程序类型中使用。开放编码迭代器提供BPF 迭代器程序类似的功能，但赋予所有其BPF 程序类型更大的灵活性与控制力。另一方面，BPF 迭代器程序可用于实现匿名或挂载于 BPF FS 的特殊文件，其内容由附加BPF 迭代器程序生成，并以 seq_file 功能为支撑。两者视具体需求都很有用
在新增一BPF 迭代器程序时，期望同时以开放编码迭代器的形式添加类似功能以获得最大的灵活性。同时也期望迭代逻辑与代码在两种迭代API 接口之间得以最大程度地共享与复用
### 开放编码的 BPF 迭代

开放编BPF 迭代器实现为紧密耦合kfunc 三元组（构造函数、下一个元素获取、析构函数）以及描述栈上迭代器状态的迭代器特定类型，BPF 验证器保证该状态不会在相应constructor/destructor/next API 之外被篡改
每种开放编BPF 迭代器都有其关联struct bpf_iter_<type>，其<type> 表示特定的迭代器类型。bpf_iter_<type> 状态需要位BPF 程序栈上，因此请确保它足够小以适配 BPF 栈。出于性能考虑，最好避免为迭代器状态进行动态内存分配，并将状态结构的大小设得足以容纳一切必要内容。但如有必要，动态内存分配是绕过 BPF 栈限制的一种方式。注意，状态结构的大小属于迭代器用户可API 的一部分，因此更改它会破坏向后兼容性，在设计时务必慎重
所kfunc（构造函数、next、析构函数）必须一致地分别命名bpf_iter_<type>_{new,next,destroy}()type> 表示迭代器类型，迭代器状态应表示为匹配的 `struct bpf_iter_<type>` 状态类型。此外，所iter kfunc 都应将指向该 `struct bpf_iter_<type>` 的指针作为第一个参数
此外  - 构造函数，`bpf_iter_<type>_new()`，可以有任意数量的额外参数。返回类型也不作强制要求  - next 方法，即 `bpf_iter_<type>_next()`，必须返回指针类型，且应恰好有一个参数：`struct bpf_iter_<type> *`（const/volatile/restrict typedef 被忽略）  - 析构函数，即 `bpf_iter_<type>_destroy()`，应返回 void，且应恰好有一个参数，next 方法类似  - `struct bpf_iter_<type>` 的大小被强制要求为正值且8 字节的倍数（以正确适配栈槽）
这种严格性与一致性使得可以构建通用辅助函数，将重要但样板化的细节抽象出来，从而能够高效且顺手地使用开放编码迭代器（参libbpf bpf_for_each() 宏）。这一点由内核kfunc 注册点强制执行
构造函next/析构函数的实现契约如下：
  - 构造函`bpf_iter_<type>_new()` 总是在栈上初始化迭代器状态。如果任何输入参数无效，构造函数仍应确保完成初始化，以使后续的 next() 调用返回 NULL。即，出错时**返回错误并构造空迭代*。构造函kfunc 被标KF_ITER_NEW 标志  - next 方法 `bpf_iter_<type>_next()` 接受指向迭代器状态的指针并产出一个元素。next 方法应始终返回一个指针。与 BPF 验证器的契约是：next 方法**保证**在元素耗尽时最终返NULL。一旦返NULL，后next 调用**应持续返NULL**。next 方法被标KF_ITER_NEXT（当然，它还应具KF_RET_NULL 以表示返NULL kfunc）  - 析构函数 `bpf_iter_<type>_destroy()` 总是被调用一次。即使构造函数失败或 next 没有返回任何内容。析构函数释放所有资源，并将 `struct bpf_iter_<type>` 使用的栈空间标记为可用于其他用途。析构函数被标记 KF_ITER_DESTROY 标志
任何开放编BPF 迭代器实现都必须至少实现这三个方法。内核强制要求：对于任意给定的迭代器类型，只有适用constructor/destructor/next 可被调用。即，验证器确保你不能将（例如）number 迭代器状态传cgroup 迭代器的 next 方法
从宏观的 BPF 验证视角来看，next 方法是分叉验证状态的点，在概念上类似于验证器在校验条件跳转时所做的操作。验证器`call bpf_iter_<type>_next` 指令进行分叉，并模拟两种结果：NULL（迭代完成）与非 NULL（返回新元素）。首先模NULL，并且应当在不进入循环的情况下到达退出。之后验证非 NULL 的情况，它要么到达退出（对于没有真正循环的简单示例），要么到达另一`call bpf_iter_<type>_next` 指令，其状态与已经（部分）验证过的状态等价。此时的状态等价意味着，从技术上讲我们将永远循环，而无法「跳出」已建立的「状态包络」（即，后续迭代不会向验证器状态添加任何新知识或约束，因此运行 1 次 次0 次或一百万次都无关紧要）。但考虑到契约规定迭代器 next 方法**必须**最终返NULL，我们可以得出结论：循环体是安全的，且最终会终止。鉴于我们已经验证了循环之外的逻辑（NULL 情况），并得出循环体安全（尽管可能循环多次）的结论，验证器可以判定整个程序逻辑的安全性
### BPF 迭代器的动机


现有几种将内核数据转储到用户空间的方式。最流行的是 `/proc` 系统。例如，`cat /proc/net/tcp6` 转储系统中所有的 tcp6 套接字，`cat /proc/net/netlink` 转储系统中所有的 netlink 套接字。然而，它们的输出格式往往固定，如果用户想要更多关于这些套接字的信息，就必须给内核打补丁，而这通常需要很长时间才能合入上游并发布。对`ss <https://man7.org/linux/man-pages/man8/ss.8.html>`_ 等流行工具也是如此，任何额外信息都需要内核补丁
为解决这个问题，常常使用 `drgn <https://www.kernel.org/doc/html/latest/bpf/drgn.html>`_ 工具在不修改内核的情况下挖掘内核数据。然而，drgn 的主要缺点在于性能，因为它无法在内核内部进行指针追踪。此外，drgn 无法验证指针值，如果指针在内核中变为无效，可能会读取到无效数据
BPF 迭代器通过提供灵活性解决了上述问题——它通过对每个内核数据对象调BPF 程序，来灵活地收集哪些数据（例如 tasks、bpf_maps 等）
### BPF 迭代器的工作原理


BPF 迭代器是一种允许用户遍历特定类型内核对象的 BPF 程序。与允许用户定义在内核中特定执行点被调用的回调的传统 BPF 跟踪程序不同，BPF 迭代器允许用户定义应对多种内核数据结构中的每一项执行的回调
例如，用户可以定义一个遍历系统上每个 task 并转储它们当前各自使用的 CPU 运行时长总量BPF 迭代器。另一BPF task 迭代器则可以转储每个 task cgroup 信息。这种灵活性正BPF 迭代器的核心价值
BPF 程序总是由用户空间进程在需要时加载到内核中。用户空间进程通过按要求打开并初始化程序骨架（skeleton），然后调用系统调用，使 BPF 程序由内核验证并加载
在传统跟踪程序中，程序通过用户空间`bpf_program__attach()` 获取程序`bpf_link` 而被激活。一旦激活，每当主内核中触发 tracepoint 时，程序回调就会被调用。对BPF 迭代器程序，程序`bpf_link` 通过 `bpf_link_create()` 获取，程序回调则由用户空间发出系统调用而触发
接下来，让我们看看如何使用迭代器遍历内核对象并读取数据
### 如何使用 BPF 迭代

BPF selftests（自测）是说明如何使用迭代器的极佳资源。在本节中，我们将走查一个展示如何加载和使用 BPF 迭代器程序的 BPF 自测。首先，我们来看 `bpf_iter.c <https://git.kernel.org/pub/scm/linux/kernel/git/bpf/bpf-next.git/tree/tools/testing/selftests/bpf/prog_tests/bpf_iter.c>`_，它展示了如何在用户空间侧加载并触发 BPF 迭代器。之后，我们将看一个运行在内核空间BPF 程序
从用户空间在内核中加BPF 迭代器通常涉及以下步骤
- 通过 `libbpf` BPF 程序加载到内核中。一旦内核验证并加载了该程序，它会向用户空间返回一个文件描述符（fd）- 通过调用 `bpf_link_create()` 并指定从内核收到BPF 程序文件描述符，获取BPF 程序`link_fd`- 接下来，通过调用以第 2 步收到的 `bpf_link` 为参数的 `bpf_iter_create()`，获BPF 迭代器文件描述符（`bpf_iter_fd`）- 通过调用 `read(bpf_iter_fd)` 触发迭代，直到没有数据可用- 使用 `close(bpf_iter_fd)` 关闭迭代fd- 如果需要重新读取数据，获取一个新`bpf_iter_fd` 并再次读取
以下是几个自BPF 迭代器程序的示例
- `bpf_iter_tcp4.c <https://git.kernel.org/pub/scm/linux/kernel/git/bpf/bpf-next.git/tree/tools/testing/selftests/bpf/progs/bpf_iter_tcp4.c>`_
- `bpf_iter_task_vmas.c <https://git.kernel.org/pub/scm/linux/kernel/git/bpf/bpf-next.git/tree/tools/testing/selftests/bpf/progs/bpf_iter_task_vmas.c>`_
- `bpf_iter_task_file.c <https://git.kernel.org/pub/scm/linux/kernel/git/bpf/bpf-next.git/tree/tools/testing/selftests/bpf/progs/bpf_iter_task_file.c>`_

让我们来看运行在内核空间`bpf_iter_task_file.c`
以下`vmlinux.h <https://facebookmicrosites.github.io/bpf/blog/2020/02/19/bpf-portability-and-co-re.html#btf>`_ `bpf_iter__task_file` 的定义。在 `vmlinux.h` 中，任何格式`bpf_iter__<iter_name>` 的结构体名称都表示一BPF 迭代器。后缀 `<iter_name>` 表示迭代器的类型
```

    struct bpf_iter__task_file {
            union {
                struct bpf_iter_meta *meta;
            };
            union {
                struct task_struct *task;
            };
            u32 fd;
            union {
                struct file *file;
            };
    };

```
在上述代码中，字'meta' 包含元数据，这对所BPF 迭代器程序都是相同的。其余字段则特定于不同的迭代器。例如，对于 task_file 迭代器，内核层提'task'fd' 'file' 字段值task' 'file' 是`引用计数 <https://facebookmicrosites.github.io/bpf/blog/2018/08/31/object-lifetime.html#file-descriptors-and-reference-counters>`_ 的，因此BPF 程序运行时它们不会消失
以下`bpf_iter_task_file.c` 文件的片段：

```

  SEC("iter/task_file")
  int dump_task_file(struct bpf_iter__task_file *ctx)
  {
    struct seq_file *seq = ctx->meta->seq;
    struct task_struct *task = ctx->task;
    struct file *file = ctx->file;
    __u32 fd = ctx->fd;

    if (task == NULL || file == NULL)
      return 0;

    if (ctx->meta->seq_num == 0) {
      count = 0;
      BPF_SEQ_PRINTF(seq, "    tgid      gid       fd      file\n");
    }

    if (tgid == task->tgid && task->tgid != task->pid)
      count++;

    if (last_tgid != task->tgid) {
      last_tgid = task->tgid;
      unique_tgid_count++;
    }

    BPF_SEQ_PRINTF(seq, "%8d %8d %8d %lx\n", task->tgid, task->pid, fd,
            (long)file->f_op);
    return 0;
  }

```
在上述示例中，段`SEC(iter/task_file)` 表明该程序是一个用于遍历所task 的全部文件的 BPF 迭代器程序。该程序的上下文`bpf_iter__task_file` 结构体
用户空间程序通过发出 `read()` 系统调用来调用运行在内核中的 BPF 迭代器程序。一旦被调用，BPF 程序就可以使用各BPF 辅助函数将数据导出到用户空间。根据你是否需要格式化输出或仅仅是二进制数据，可以分别使用 `bpf_seq_printf()`（以BPF_SEQ_PRINTF 辅助宏）`bpf_seq_write()` 函数。对于二进制编码的数据，用户空间应用程序可以按需处理来自 `bpf_seq_write()` 的数据。对于格式化数据，在BPF 迭代器固定（pin）到 bpffs 挂载点后，可以使`cat <path>` 打印结果，类似于 ``cat /proc/net/netlink``。之后使`rm -f <path>` 移除被固定的迭代器
例如，你可以使用以下命令`bpf_iter_ipv6_route.o` 目标文件创建一BPF 迭代器，并将其固定到 `/sys/fs/bpf/my_route` 路径
```

  $ bpftool iter pin ./bpf_iter_ipv6_route.o  /sys/fs/bpf/my_route

```
然后使用以下命令打印结果
```

  $ cat /sys/fs/bpf/my_route


```
### BPF 迭代器程序类型实现内核支

要在内核中实BPF 迭代器，开发者必须对 `bpf.h <https://git.kernel.org/pub/scm/linux/kernel/git/bpf/bpf-next.git/tree/include/linux/bpf.h>`_ 文件中定义的下列关键数据结构做一次性的修改
```

  struct bpf_iter_reg {
            const char *target;
            bpf_iter_attach_target_t attach_target;
            bpf_iter_detach_target_t detach_target;
            bpf_iter_show_fdinfo_t show_fdinfo;
            bpf_iter_fill_link_info_t fill_link_info;
            bpf_iter_get_func_proto_t get_func_proto;
            u32 ctx_arg_info_size;
            u32 feature;
            struct bpf_ctx_arg_aux ctx_arg_info[BPF_ITER_CTX_ARG_MAX];
            const struct bpf_iter_seq_info *seq_info;
  };

```
填写完数据结构字段后，调`bpf_iter_reg_target()` 将该迭代器注册到BPF 迭代器子系统
以下是对 struct `bpf_iter_reg` 各字段的说明
   :widths: 25 50
   :header-rows: 1

   - - Fields
     - Description
   - - target
     - 指定 BPF 迭代器的名称。例如：`bpf_map`、`bpf_map_elem`。该名称应不同于内核中其`bpf_iter` 目标名称   - - attach_target and detach_target
     - 允许目标特定`link_create` 操作，因为某些目标可能需要特殊处理。在用户空间 link_create 阶段被调用   - - show_fdinfo and fill_link_info
     - 当用户试图获取与迭代器关联的 link 信息时，被调用以填充目标特定信息   - - get_func_proto
     - 允许 BPF 迭代器访问特定于该迭代器BPF 辅助函数   - - ctx_arg_info_size and ctx_arg_info
     - 指定bpf 迭代器关联的 BPF 程序参数的验证器状态   - - feature
     - 指定内核 BPF 迭代器基础设施中的某些操作请求。目前仅支持 BPF_ITER_RESCHED。这意味着会调用内核函cond_resched() 以避免其他内核子系统（例rcu）出现异常行为   - - seq_info
     - 指定用于 BPF 迭代器的 seq 操作集合，以及用于初始化/释放相应 `seq_file` 私有数据的辅助函数
`点击此处 <https://lore.kernel.org/bpf/20210212183107.50963-2-songliubraving@fb.com/>`_ 查看内核`task_vma` BPF 迭代器的实现
### BPF Task 迭代器添加参

默认情况下，BPF 迭代器遍历整个系统中所有指定类型（进程、cgroup、map 等）的对象，以读取相关的内核数据。但常常只关心可迭代内核对象中很小的一个子集，例如仅遍历某个特定进程内task。因此，BPF 迭代器程序支持在附加时由用户空间对迭代器程序进行配置，从而将对象从迭代中过滤掉
### BPF Task 迭代器程

以下代码是一个通过迭代器的 `seq_file` 打印文件task 信息BPF 迭代器程序。它是一个标准的 BPF 迭代器程序，会访问迭代器的每个文件。我们稍后将在示例中使用这个 BPF 程序
```

  #include <vmlinux.h>
  #include <bpf/bpf_helpers.h>

  char _license[] SEC("license") = "GPL";

  SEC("iter/task_file")
  int dump_task_file(struct bpf_iter__task_file *ctx)
  {
        struct seq_file *seq = ctx->meta->seq;
        struct task_struct *task = ctx->task;
        struct file *file = ctx->file;
        __u32 fd = ctx->fd;
        if (task == NULL || file == NULL)
                return 0;
        if (ctx->meta->seq_num == 0) {
                BPF_SEQ_PRINTF(seq, "    tgid      pid       fd      file\n");
        }
        BPF_SEQ_PRINTF(seq, "%8d %8d %8d %lx\n", task->tgid, task->pid, fd,
                        (long)file->f_op);
        return 0;
  }

```
### 创建带参数的文件迭代

现在，让我们看看如何创建一个仅包含某个进程文件的迭代器
首先，按如下所示填`bpf_iter_attach_opts` 结构体：

```

  LIBBPF_OPTS(bpf_iter_attach_opts, opts);
  union bpf_iter_link_info linfo;
  memset(&linfo, 0, sizeof(linfo));
  linfo.task.pid = getpid();
  opts.link_info = &linfo;
  opts.link_info_len = sizeof(linfo);

```
`linfo.task.pid` 若非零，则指示内核创建一个仅包含具有指定 `pid` 的进程所打开文件的迭代器。在本例中，我们将只遍历自己进程的文件。如`linfo.task.pid` 为零，迭代器将访问每个进程的每个已打开文件。类似地，`linfo.task.tid` 指示内核创建一个访问某个特定线程（而非进程）已打开文件的迭代器。本例中，`linfo.task.tid` 仅在某个线程拥有独立的文件描述符表时才与 `linfo.task.pid` 不同。在大多数情况下，进程的所有线程共享同一个文件描述符表
现在，在用户空间程序中，将该结构体的指针传给 `bpf_program__attach_iter()`
```

  link = bpf_program__attach_iter(prog, &opts);
  iter_fd = bpf_iter_create(bpf_link__fd(link));

```
如果 **tid** **pid** 都为零，则从`bpf_iter_attach_opts` 结构体创建的迭代器将包含系统中（实际上是命名空间内）每个 task 的每个已打开文件。这等同于向 `bpf_program__attach_iter()` 传入 NULL 作为第二个参数
整个程序如下所示：

```

  #include <stdio.h>
  #include <unistd.h>
  #include <bpf/bpf.h>
  #include <bpf/libbpf.h>
  #include "bpf_iter_task_ex.skel.h"

  static int do_read_opts(struct bpf_program *prog, struct bpf_iter_attach_opts *opts)
  {
        struct bpf_link *link;
        char buf[16] = {};
        int iter_fd = -1, len;
        int ret = 0;

        link = bpf_program__attach_iter(prog, opts);
        if (!link) {
                fprintf(stderr, "bpf_program__attach_iter() fails\n");
                return -1;
        }
        iter_fd = bpf_iter_create(bpf_link__fd(link));
        if (iter_fd < 0) {
                fprintf(stderr, "bpf_iter_create() fails\n");
                ret = -1;
                goto free_link;
        }
        /* not check contents, but ensure read() ends without error */
        while ((len = read(iter_fd, buf, sizeof(buf) - 1)) > 0) {
                buf[len] = 0;
                printf("%s", buf);
        }
        printf("\n");
  free_link:
        if (iter_fd >= 0)
                close(iter_fd);
        bpf_link__destroy(link);
        return 0;
  }

  static void test_task_file(void)
  {
        LIBBPF_OPTS(bpf_iter_attach_opts, opts);
        struct bpf_iter_task_ex *skel;
        union bpf_iter_link_info linfo;
        skel = bpf_iter_task_ex__open_and_load();
        if (skel == NULL)
                return;
        memset(&linfo, 0, sizeof(linfo));
        linfo.task.pid = getpid();
        opts.link_info = &linfo;
        opts.link_info_len = sizeof(linfo);
        printf("PID %d\n", getpid());
        do_read_opts(skel->progs.dump_task_file, &opts);
        bpf_iter_task_ex__destroy(skel);
  }

  int main(int argc, const char * const * argv)
  {
        test_task_file();
        return 0;
  }

```
以下是该程序的输出
```

  PID 1859

     tgid      pid       fd      file
     1859     1859        0 ffffffff82270aa0
     1859     1859        1 ffffffff82270aa0
     1859     1859        2 ffffffff82270aa0
     1859     1859        3 ffffffff82272980
     1859     1859        4 ffffffff8225e120
     1859     1859        5 ffffffff82255120
     1859     1859        6 ffffffff82254f00
     1859     1859        7 ffffffff82254d80
     1859     1859        8 ffffffff8225abe0

```
### 不带参数


让我们看看不带参数的 BPF 迭代器如何跳过系统中其他进程的文件。在这种情况下，BPF 程序必须检task pid tid，否则它将接收到系统中（实际上是当前 **pid** 命名空间内）每个已打开的文件。因此，我们通常会在 BPF 程序中添加一个全局变量，将 **pid** 传递给 BPF 程序
BPF 程序如下所示
```

    ......
    int target_pid = 0;

    SEC("iter/task_file")
    int dump_task_file(struct bpf_iter__task_file *ctx)
    {
          ......
          if (task->tgid != target_pid) /* Check task->pid instead to check thread IDs */
                  return 0;
          BPF_SEQ_PRINTF(seq, "%8d %8d %8d %lx\n", task->tgid, task->pid, fd,
                          (long)file->f_op);
          return 0;
    }

```
用户空间程序如下所示：

```

    ......
    static void test_task_file(void)
    {
          ......
          skel = bpf_iter_task_ex__open_and_load();
          if (skel == NULL)
                  return;
          skel->bss->target_pid = getpid(); /* process ID.  For thread id, use gettid() */
          memset(&linfo, 0, sizeof(linfo));
          linfo.task.pid = getpid();
          opts.link_info = &linfo;
          opts.link_info_len = sizeof(linfo);
          ......
    }

```
`target_pid` BPF 程序中的全局变量。用户空间程序应将该变量初始化为一个进ID，以跳过 BPF 程序中其他进程的已打开文件。当你为 BPF 迭代器添加参数时，迭代器调用 BPF 程序的次数会减少，从而可以节省大量资源
### VMA 迭代器添加参

默认情况下，BPF VMA 迭代器包含每个进程的每个 VMA。不过，你仍然可以指定一个进程或线程，以仅包含其 VMA。与文件不同，线程不能拥有独立的地址空间（自 Linux 2.6.0-test6 起）。在这里，使**tid** 与使**pid** 没有区别
### Task 迭代器添加参

**pid** BPF task 迭代器包含某个进程的所task（线程）。BPF 程序会逐个接收这些 task。你可以指定**tid** 参数BPF task 迭代器，以仅包含与给**tid** 匹配task