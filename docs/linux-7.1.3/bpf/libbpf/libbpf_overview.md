
## libbpf 概述


libbpf 是一个基于 C 的库，包含一个 BPF 加载器，它接收编译好的 BPF
目标文件，并将其准备并加载到 Linux 内核中。libbpf 接管了加载、验证以及将
BPF 程序附加到各种内核钩子（hook）的重活，使 BPF 应用开发者只需专注于
BPF 程序的正确性与性能。

以下是 libbpf 支持的高层特性：

- 提供高层和低层 API，供用户空间程序与 BPF 程序进行交互。低层 API 封装了
  全部 bpf 系统调用的功能，在用户需要对用户空间与 BPF 程序之间的交互
  进行更细粒度控制时非常有用。
- 为 bpftool 生成的 BPF 目标骨架（skeleton）提供整体支持。
  骨架文件简化了用户空间程序访问全局变量以及与 BPF 程序协作的过程。
- 提供 BPF 端 API，包括 BPF 辅助函数定义、BPF map 支持以及
  tracing 辅助函数，使开发者能够简化 BPF 代码的编写。
- 支持 BPF CO-RE 机制，使 BPF 开发者能够编写可移植的 BPF 程序，
  这些程序可以一次编译并在不同内核版本上运行。

本文档将深入探讨上述概念，帮助您更深入地理解 libbpf 的能力与优势，
以及它如何帮助您高效地开发 BPF 应用。

## BPF 应用生命周期与 libbpf API


一个 BPF 应用由一个或多个 BPF 程序（彼此协作或完全独立）、BPF map 以及
全局变量组成。全局变量在所有 BPF 程序之间共享，使它们能够围绕一组
公共数据进行协作。libbpf 提供了一系列 API，用户空间程序可以通过触发
BPF 应用生命周期的不同阶段来操纵这些 BPF 程序。

以下小节简要概述了 BPF 生命周期的各个阶段：

- **Open 阶段**：在此阶段，libbpf 解析 BPF
  目标文件并发现 BPF map、BPF 程序以及全局变量。在 BPF 应用被打开后，
  用户空间应用可以在所有实体被创建和加载之前进行额外的调整
  （例如，必要时设置 BPF 程序类型；为全局变量预设初始值等）。

- **Load 阶段**：在加载阶段，libbpf 创建 BPF
  map、解析各种重定位，并将 BPF 程序验证并加载到内核中。此时，libbpf
  会校验 BPF 应用的各个部分并将其加载到内核，但还没有任何 BPF 程序
  被执行。在加载阶段之后，可以设置 BPF map 的初始状态，而无需担心与
  BPF 程序代码执行产生竞争。

- **Attachment 阶段**：在此阶段，libbpf
  将 BPF 程序附加到各种 BPF 钩子点（例如 tracepoint、kprobe、
  cgroup 钩子、网络数据包处理流水线等）。在此阶段，BPF 程序执行
  诸如处理数据包或更新 BPF map 与全局变量等有用工作，这些内容
  可以从用户空间读取。

- **Tear down 阶段**：在拆除阶段，
  libbpf 将 BPF 程序从内核上分离并卸载它们。BPF map 被销毁，
  BPF 应用使用的所有资源被释放。

## BPF 目标骨架文件


BPF 骨架是 libbpf API 操作 BPF 目标的另一种接口。骨架代码抽象了通用的
libbpf API，从而大幅简化了从用户空间操纵 BPF 程序的代码。骨架代码包含
BPF 目标文件的字节码表示，简化了分发 BPF 代码的过程。由于内嵌了 BPF
字节码，您的应用二进制文件无需再部署额外的文件。

您可以通过将 BPF 目标文件传给 bpftool 来生成该目标文件对应的骨架头文件
`(.skel.h)`。生成的 BPF 骨架提供了以下与 BPF 生命周期相对应的自定义函数，
每个函数都以具体的目标名作为前缀：

- `<name>__open()` – 创建并打开 BPF 应用（`<name>` 代表
  具体的 bpf 目标名）
- `<name>__load()` – 实例化、加载并验证 BPF 应用各部分
- `<name>__attach()` – 附加所有可自动附加的 BPF 程序（这是
  可选的，您也可以通过直接使用 libbpf API 获得更多控制）
- `<name>__destroy()` – 分离所有 BPF 程序并
  释放所有已使用的资源

使用骨架代码是操作 bpf 程序的推荐方式。请注意，BPF 骨架提供了对底层
BPF 目标的访问，因此即便使用了 BPF 骨架，凡是能用通用 libbpf API 完成的
操作仍然都可以完成。它是一个附加的便利特性，没有系统调用，也没有
繁琐的代码。

### 使用骨架文件的其他优势


- BPF 骨架为用户空间程序提供了操作 BPF 全局变量的接口。骨架代码将全局变量
  以结构体的形式内存映射到用户空间。该结构体接口允许用户空间程序在 BPF
  加载阶段之前初始化 BPF 程序，并在之后从用户空间获取和更新数据。

- `skel.h` 文件列出了可用的 map、程序等内容，反映了目标文件的结构。BPF 骨架
  将所有的 BPF map 和 BPF 程序作为结构体字段直接暴露。这就消除了使用
  `bpf_object_find_map_by_name()` 和
  `bpf_object_find_program_by_name()` API 进行基于字符串查找的需要，
  从而减少了因 BPF 源代码与用户空间代码失步而产生的错误。

- 内嵌的目标文件字节码表示确保了骨架与 BPF 目标文件始终保持同步。

## BPF 辅助函数


libbpf 提供 BPF 端 API，BPF 程序可以使用它们与系统进行交互。BPF 辅助函数
的定义使开发者能够像使用其他普通 C 函数一样在 BPF 代码中使用它们。例如，
有一些辅助函数可用于打印调试信息、获取系统启动以来的时间、与 BPF map
交互、操纵网络数据包等。

有关这些辅助函数的作用、接收的参数以及返回值的完整说明，请参阅
`bpf-helpers
<https://man7.org/linux/man-pages/man7/bpf-helpers.7.html>`_ 手册页。

## BPF CO-RE（一次编译 – 到处运行）


BPF 程序在内核空间运行，能够访问内核内存和数据结构。BPF 应用面临的一个
局限是缺乏跨不同内核版本和配置的可移植性。`BCC
<https://github.com/iovisor/bcc/>`_ 是 BPF 可移植性的解决方案之一。
然而，它带来了运行时开销，并且由于将编译器嵌入应用而产生了较大的二进制体积。

libbpf 通过支持 BPF CO-RE 概念来提升 BPF 程序的可移植性。
BPF CO-RE 将 BTF 类型信息、libbpf 以及编译器结合起来，生成一个可在多个
内核版本和配置上运行的单一可执行二进制文件。

为了使 BPF 程序可移植，libbpf 依赖于运行中内核的 BTF 类型信息。内核也通过
`sysfs` 在 `/sys/kernel/btf/vmlinux` 暴露这一自描述的权威 BTF
信息。

您可以使用以下命令为运行中的内核生成 BTF 信息：

```

  $ bpftool btf dump file /sys/kernel/btf/vmlinux format c > vmlinux.h

```
该命令会生成一个 `vmlinux.h` 头文件，其中包含运行中的内核使用的所有内核类型
（[BTF types <../btf>](BTF types <../btf>)）。在您的 BPF 程序包含
`vmlinux.h` 后，就不再依赖系统范围内的内核头文件。

libbpf 通过查看 BPF 程序记录下来的 BTF 类型与重定位信息，并将其与运行中
内核提供的 BTF 信息（vmlinux）进行匹配来实现 BPF 程序的可移植性。接着，
libbpf 解析并匹配所有类型与字段，更新必要的偏移量以及其他可重定位数据，
以确保 BPF 程序的逻辑在特定宿主机内核上正确运行。因此，BPF CO-RE 概念
消除了与 BPF 开发相关的开销，使开发者无需修改即可编写可移植的 BPF 应用，
也无需在目标机器上进行运行时源代码编译。

以下代码片段展示了如何使用 BPF CO-RE 和 libbf 读取内核
`task_struct` 的 parent 字段。以可重定位方式读取字段的基本辅助函数是
`bpf_core_read(dst, sz, src)`，它会从 `src` 引用的字段读取
`sz` 个字节到 `dst` 指向的内存中。

   :emphasize-lines: 6

    //...
    struct task_struct **task = (void **)bpf_get_current_task();
    struct task_struct *parent_task;
    int err;

    err = bpf_core_read(&parent_task, sizeof(void *), &task->parent);
    if (err) {
      /** handle error **/
    }

    /** parent_task 包含了 task->parent 指针的值 **/

在代码片段中，我们首先使用 `bpf_get_current_task()` 获取指向当前
`task_struct` 的指针。然后使用 `bpf_core_read()` 将 task struct 的
parent 字段读入 `parent_task` 变量。`bpf_core_read()` 很像
`bpf_probe_read_kernel()` BPF 辅助函数，不同之处在于它会记录关于该字段
的信息，以便在目标内核上进行重定位。也就是说，如果 `parent` 字段由于
其前面新增了某个字段而偏移到了 `struct task_struct` 内的不同偏移位置，
libbpf 会自动将实际偏移量调整到正确的值。

## 开始使用 libbpf


请查看 `libbpf-bootstrap <https://github.com/libbpf/libbpf-bootstrap>`_
代码仓库，其中包含使用 libbpf 构建各类 BPF 应用的简单示例。

另请参阅 `libbpf API 文档
<https://libbpf.readthedocs.io/en/latest/api.html>`_。

## libbpf 与 Rust


如果您使用 Rust 构建 BPF 应用，建议使用
`Libbpf-rs <https://github.com/libbpf/libbpf-rs>`_ 库，而不是直接使用
bindgen 生成的 libbpf 绑定。Libbpf-rs 以符合 Rust 习惯的接口封装了 libbpf
功能，并提供 libbpf-cargo 插件来处理 BPF 代码编译和骨架生成。使用 Libbpf-rs
会使 BPF 应用的用户空间部分更易于构建。请注意，BPF 程序本身仍必须用
纯 C 编写。

## libbpf 日志记录


默认情况下，libbpf 将信息和警告消息记录到 stderr。这些消息的详细程度可以
通过设置环境变量 LIBBPF_LOG_LEVEL 为 warn、info 或 debug 来控制。可以使用
`libbpf_set_print()` 设置自定义的日志回调。

## 其他文档


- `Program types and ELF Sections <https://libbpf.readthedocs.io/en/latest/program_types.html>`_
- `API naming convention <https://libbpf.readthedocs.io/en/latest/libbpf_naming_convention.html>`_
- `Building libbpf <https://libbpf.readthedocs.io/en/latest/libbpf_build.html>`_
- `API documentation Convention <https://libbpf.readthedocs.io/en/latest/libbpf_naming_convention.html#api-documentation-convention>`_
