
## 控制组 v2


:Date: October, 2015
:Author: Tejun Heo <tj@kernel.org>

本文件是关于 cgroup v2 的设计、接口与约定的权威文档。它描述了 cgroup 所有对用户空间可见的方面，包括核心与各具体控制器的行为。今后所有的变更都必须反映到本文件中。关于 v1 的文档可在 Documentation/admin-guide/cgroup-v1/index.rst <cgroup-v1> 中找到。


   [无论何时向本文件新增任何章节，请同时在此处添加对应条目。]

   1. 简介
     1-1. 术语
     1-2. 什么是 cgroup？
   2. 基本操作
     2-1. 挂载
     2-2. 组织进程与线程
       2-2-1. 进程
       2-2-2. 线程
     2-3. [解除]填充通知
     2-4. 控制控制器
       2-4-1. 可用性
       2-4-2. 启用与禁用
       2-4-3. 自上而下约束
       2-4-4. 无内部进程约束
     2-5. 委托
       2-5-1. 委托模型
       2-5-2. 委托 containment（隔离）
     2-6. 准则
       2-6-1. 组织一次并控制
       2-6-2. 避免名称冲突
   3. 资源分配模型
     3-1. 权重
     3-2. 限额
     3-3. 保护
     3-4. 分配
   4. 接口文件
     4-1. 格式
     4-2. 约定
     4-3. 核心接口文件
   5. 控制器
     5-1. CPU
       5-1-1. CPU 接口文件
     5-2. 内存
       5-2-1. 内存接口文件
       5-2-2. 使用准则
       5-2-3. 回收保护
       5-2-4. 内存所有权
     5-3. IO
       5-3-1. IO 接口文件
       5-3-2. 回写
       5-3-3. IO 延迟
         5-3-3-1. IO 延迟节流如何工作
         5-3-3-2. IO 延迟接口文件
       5-3-4. IO 优先级
     5-4. PID
       5-4-1. PID 接口文件
     5-5. Cpuset
       5.5-1. Cpuset 接口文件
     5-6. 设备控制器
     5-7. RDMA
       5-7-1. RDMA 接口文件
     5-8. DMEM
       5-8-1. DMEM 接口文件
     5-9. HugeTLB
       5.9-1. HugeTLB 接口文件
     5-10. Misc
       5.10-1 Misc 接口文件
       5.10-2 迁移与所有权
     5-11. 其他
       5-11-1. perf_event
     5-N. 非规范性信息
       5-N-1. CPU 控制器根 cgroup 进程行为
       5-N-2. IO 控制器根 cgroup 进程行为
   6. 命名空间
     6-1. 基础
     6-2. 根与视图
     6-3. 迁移与 setns(2)
     6-4. 与其他命名空间的交互
   P. 内核编程相关信息
     P-1. 回写相关的文件系统支持
   D. 已弃用的 v1 核心特性
   R. v1 存在的问题及 v2 的设计理由
     R-1. 多重层级
     R-2. 线程粒度
     R-3. 内部节点与线程之间的竞争
     R-4. 其他接口问题
     R-5. 控制器问题及对策
       R-5-1. 内存


## 简介


### 术语


“cgroup” 是 “control group（控制组）”的缩写，永远不大写。单数形式既用来指整个特性，也作为限定语使用，如 “cgroup controllers（cgroup 控制器）”。当明确指代多个独立的控制组时，使用复数形式 “cgroups”。


### 什么是 cgroup？


cgroup 是一种将进程按层次组织、并以可控且可配置的方式沿该层次分配系统资源的机制。

cgroup 大体上由两部分组成——核心（core）与控制器（controllers）。cgroup 核心主要负责按层次组织进程。cgroup 控制器通常负责沿层次分配某一特定类型的系统资源，不过也存在一些用于资源分配之外用途的实用型控制器（utility controllers）。

cgroups 构成树形结构，系统中的每个进程都且仅属于一个 cgroup。一个进程的所有线程都属于同一个 cgroup。进程在创建时会被放入其父进程当时所属的 cgroup。进程可以被迁移到另一个 cgroup。迁移一个进程不会影响已经存在的后代进程。

遵循特定的结构性约束，控制器可以在某个 cgroup 上有选择地启用或禁用。所有控制器的行为都是分层的——如果某个控制器在某个 cgroup 上被启用，它会影响属于构成该 cgroup 包含性子层次（inclusive sub-hierarchy）的所有 cgroup 的进程。当某个控制器在嵌套的 cgroup 上被启用时，它总是进一步限制资源分配。在层次中更靠近根节点所设置的约束，无法被更远的节点覆盖。


## 基本操作


### 挂载


与 v1 不同，cgroup v2 只有单一的层级。cgroup v2
```

  # mount -t cgroup2 none $MOUNT_POINT

```
cgroup2 文件系统的魔数为 0x63677270（“cgrp”）。所有支持 v2、且未绑定到某个 v1 层级的控制器都会自动绑定到 v2 层级并出现在根节点。未在 v2 层级中处于活动使用的控制器可以绑定到其他层级。这使得以完全向后兼容的方式将 v2 层级与遗留的 v1 多重层级混合使用成为可能。

只有当某个控制器在其当前层级中不再被引用后，它才能跨层级移动。由于每个 cgroup 的控制器状态是异步销毁的，且控制器可能持有滞留引用，因此在上一层级最终 umount 之后，该控制器可能不会立即出现在 v2 层级上。类似地，一个控制器必须先被完全禁用才能移出统一层级，而它可能需要一些时间才能对其它层级变为可用；此外，由于控制器之间的相互依赖关系，可能还需要禁用其他控制器。

虽然在开发和手动配置时很有用，但在生产环境中强烈不建议在 v2 与其它层级之间动态移动控制器。建议在系统启动后、开始使用控制器之前就决定好层级结构与控制器关联。

在向 v2 过渡期间，系统管理软件可能仍会自动挂载 v1 的 cgroup 文件系统，从而在手动干预成为可能之前于启动阶段劫持所有控制器。为了让测试和试验更方便，内核参数 cgroup_no_v1= 允许在 v1 中禁用控制器，并使它们在 v2 中始终可用。

cgroup v2 目前支持以下挂载选项。

  nsdelegate
	将 cgroup 命名空间视为委托边界。此选项是系统级的，只能在挂载时设置，或通过从 init 命名空间进行重新挂载来修改。该挂载选项在非 init 命名空间的挂载上会被忽略。详情请参阅“委托”一节。

  favordynmods
       降低诸如任务迁移和控制器开关等动态 cgroup 修改的延迟，代价是使 fork 和 exit 等热路径操作的代价更高。以“创建 cgroup、启用控制器、然后用 CLONE_INTO_CGROUP 填充它”为特征的静态使用模式不受此选项影响。

  memory_localevents
       仅用当前 cgroup 的数据（而非任何子树）填充 memory.events。这是遗留行为，不带此选项的默认行为是包含子树计数。此选项是系统级的，只能在挂载时设置，或通过从 init 命名空间进行重新挂载来修改。该挂载选项在非 init 命名空间的挂载上会被忽略。

  memory_recursiveprot
        将 memory.min 和 memory.low 的保护以递归方式应用到整个子树，而无需显式向下传播到叶子 cgroup。这允许将整个子树相互隔离保护，同时在子树内部保留自由竞争。这本应是默认行为，但为了避免让依赖原始语义（例如在树的更高层级指定虚高的“bypass”保护值）的配置发生回归，它被做成了一个挂载选项。

  memory_hugetlb_accounting
        将 HugeTLB 内存使用计入 cgroup 针对内存控制器的整体内存使用（用于统计上报与内存保护）。这是一种可能影响现有配置的新行为，因此必须通过此挂载选项显式选择加入。

        需要牢记以下几点注意事项：

        - 内存控制器不涉及任何 HugeTLB 池管理。预分配的池不属于任何人。具体而言，当一个新的 HugeTLB folio 被分配进池时，从内存控制器的角度看它不被计入。只有当它真正被使用（例如在处理页错误时）时，才会向某个 cgroup 记账。主机内存过量使用管理在配置硬限额时必须考虑到这一点。一般而言，HugeTLB 池管理应通过其他机制（例如 HugeTLB 控制器）完成。
        - 向内存控制器记账某个 HugeTLB folio 失败会导致 SIGBUS。即使 HugeTLB 池仍有可用页（但 cgroup 限额已满且回收尝试失败），也可能发生这种情况。
        - 将 HugeTLB 内存计入内存控制器会影响内存保护与回收的动态行为。任何用户空间的调优（例如对 low、min 限额的调优）都需要将此考虑在内。
        - 在未选择此选项时使用的 HugeTLB 页不会被内存控制器跟踪（即使之后再重新挂载 cgroup v2 也不会）。

  pids_localevents
        此选项恢复 pids.events:max 类似 v1 的行为，即只统计本地的（cgroup 内部的）fork 失败。不带此选项时，pids.events.max 表示 cgroup 子树上任何 pids.max 的强制执行情况。


### 组织进程与线程


#### 进程


最初，只存在根 cgroup，所有进程都属于它。
```

  # mkdir $CGROUP_NAME

```
一个给定的 cgroup 可以有多个子 cgroup，形成树形结构。每个 cgroup 都有一个可读写的接口文件 “cgroup.procs”。读取时，它逐行列出属于该 cgroup 的所有进程的 PID。PID 没有特定顺序，如果某个进程被移动到另一个 cgroup 后又移回，或者在读取过程中 PID 被回收，同一个 PID 可能会出现多次。

通过将某个进程的 PID 写入目标 cgroup 的 “cgroup.procs” 文件，可以把该进程迁移进该 cgroup。单次 write(2) 调用只能迁移一个进程。如果一个进程由多个线程组成，写入其中任意一个线程的 PID 都会迁移该进程的所有线程。

当一个进程 fork 出子进程时，新进程诞生于执行 fork 操作的进程当时所属的 cgroup。进程退出后，会一直关联到它退出时所属的 cgroup，直到被回收（reaped）；不过，僵尸进程不会出现在 “cgroup.procs” 中，因此无法被移动到另一个 cgroup。

一个没有任何子 cgroup 或活动进程的 cgroup 可以通过删除目录来销毁。注意，一个没有子 cgroup、且只与僵尸进程关联的 cgroup 是
```

  # rmdir $CGROUP_NAME

```
“/proc/$PID/cgroup” 列出了进程的 cgroup 归属。如果系统中正在使用遗留 cgroup，这个文件可能包含多行，每个层级一行。cgroup v2 的条目始终位于
```

  # cat /proc/842/cgroup
  ...
  0::/test-cgroup/test-cgroup-nested

```
如果进程变成了僵尸，且它关联的 cgroup 已被
```

  # cat /proc/842/cgroup
  ...
  0::/test-cgroup/test-cgroup-nested (deleted)


```
#### 线程


cgroup v2 对一部分控制器支持线程粒度，以满足需要在进程组各线程之间进行分层资源分配的使用场景。默认情况下，一个进程的所有线程都属于同一个 cgroup，该 cgroup 同时也作为资源域（resource domain），承载不属于某个特定进程或线程的资源消耗。线程模式允许线程散布在子树上，同时仍然为它们维护共同的资源域。

支持线程模式的控制器称为线程化控制器（threaded controllers）。不支持的称为域控制器（domain controllers）。

将一个 cgroup 标记为线程化会使它作为线程化 cgroup 加入其父节点的资源域。父节点可以是另一个线程化 cgroup，其资源域在层次中更靠上。线程化子树的根——即最近的、非线程化的祖先——可互换地称为线程化域（threaded domain）或线程根（thread root），并作为整个子树的资源域。

在线程化子树内部，一个进程的线程可以被放入不同的 cgroup，且不受“无内部进程”约束的限制——线程化控制器可以在非叶子 cgroup 上启用，无论其中是否有线程。

由于线程化域 cgroup 承载了整棵子树的所有域资源消耗，无论其中是否有进程，都被认为具有内部资源消耗，因此不能拥有非线程化的已填充子 cgroup。由于根 cgroup 不受“无内部进程”约束限制，它可以同时充当线程化域和域 cgroup 的父节点。

cgroup 的当前操作模式或类型显示在 “cgroup.type” 文件中，它表明该 cgroup 是普通域、作为某线程化子树之域的域，还是一个线程化 cgroup。

在创建时，cgroup 始终是域 cgroup，可以通过向 “cgroup.type” 文件写入 “threaded” 变为线程化。
```

  # echo threaded > cgroup.type

```
一旦线程化，该 cgroup 就无法再变回域。要启用线程模式，必须满足以下条件。

- 由于该 cgroup 将加入父节点的资源域。父节点必须是有效的（线程化）域或线程化 cgroup。

- 当父节点是未线程化的域时，它不能启用任何域控制器，也不能有已填充的域子节点。根节点不受此要求限制。

从拓扑角度看，一个 cgroup 可能处于无效状态。请看下例
```

  A (threaded domain) - B (threaded) - C (domain, just created)

```
C 被创建为域，但并未连接到一个能够承载子域的父节点。在把 C 变为线程化 cgroup 之前，它无法被使用。在这些情况下，“cgroup.type” 文件会报告 “domain (invalid)”。因无效拓扑而失败的操作使用 EOPNOTSUPP 作为 errno。

当一个 cgroup 的某个子 cgroup 变为线程化，或者在 cgroup 中仍有进程时于 “cgroup.subtree_control” 文件中启用线程化控制器，该域 cgroup 会变为线程化域。当这些条件清除后，线程化域会恢复为普通域。

读取时，“cgroup.threads” 包含该 cgroup 中所有线程的线程 ID 列表。除了操作是每线程而非每进程之外，“cgroup.threads” 与 “cgroup.procs” 具有相同的格式和行为。虽然 “cgroup.threads” 可以在任何 cgroup 中写入，但由于它只能在同一线程化域内移动线程，其操作被限制在每个线程化子树之内。

线程化域 cgroup 作为整棵子树的资源域，虽然线程可以散布在子树中，但所有进程都被认为位于线程化域 cgroup 中。线程化域 cgroup 中的 “cgroup.procs” 包含子树中所有进程的 PID，且不能在子树内部被读取。不过，“cgroup.procs” 可以从子树中的任何位置写入，以将匹配进程的所有线程迁移到该 cgroup。

只有线程化控制器才能在线程化子树中启用。当某个线程化控制器在线程化子树内部被启用时，它只记账并控制与该 cgroup 及其后代中的线程相关的资源消耗。所有不绑定到特定线程的消耗都属于线程化域 cgroup。

由于线程化子树不受“无内部进程”约束限制，线程化控制器必须能够处理非叶子 cgroup 中线程与其子 cgroup 之间的竞争。每个线程化控制器各自定义了这种竞争如何处理。

目前，以下控制器是线程化的，可以在其中启用
```

```
- cpu
- cpuset
- perf_event
- pids

### [解除]填充通知


每个非根 cgroup 都有一个 “cgroup.events” 文件，其中包含 “populated” 字段，指示该 cgroup 的子层次中是否有活动进程。如果 cgroup 及其后代中没有活动进程，其值为 0；否则为 1。当值发生变化时会触发 poll 和 [id]notify 事件。例如，这可用于在某个子层次的所有进程退出后启动清理操作。填充状态的更新与通知是递归的。考虑以下子层次，其中括号内的数字表示进程数
```

  A(4) - B(0) - C(1)
              \ D(0)

```
A、B 和 C 的 “populated” 字段为 1，而 D 的为 0。C 中的那一个进程退出后，B 和 C 的 “populated” 字段会翻转为 “0”，并在这两个 cgroup 的 “cgroup.events” 文件上生成文件修改事件。


### 控制控制器


#### 可用性


当某个控制器被内核支持（即已编译进内核、未被禁用、也未挂接到 v1 层级），并且列在 “cgroup.controllers” 文件中时，它在该 cgroup 中就是可用的。可用性意味着该控制器的接口文件会被暴露在 cgroup 的目录中，从而能够在该 cgroup 内观察或控制目标资源的分配。


#### 启用与禁用


每个 cgroup 都有一个 “cgroup.controllers” 文件，它列出了所有
```

  # cat cgroup.controllers
  cpu io memory

```
默认不启用任何控制器。控制器可以通过如下方式启用和禁用
```

  # echo "+cpu +memory -io" > cgroup.subtree_control

```
只有列在 “cgroup.controllers” 中的控制器才能被启用。当像上面那样指定多个操作时，它们要么全部成功，要么全部失败。如果对同一个控制器指定了多个操作，最后一个生效。

在某个 cgroup 上启用控制器，意味着其直接子节点之间的目标资源分配将受到控制。考虑以下子层次。已启用的控制器为
```

  A(cpu,memory) - B(memory) - C()
                            \ D()

```
由于 A 启用了 “cpu” 和 “memory”，A 将控制对其子节点（本例中即 B）的 CPU 周期与内存分配。由于 B 启用了 “memory” 但未启用 “CPU”，C 和 D 将在 CPU 周期上自由竞争，但它们从 B 可获得的内存划分将受控。

由于控制器调节目标资源向其 cgroup 子节点的分配，启用它会在子 cgroup 中创建该控制器的接口文件。在上例中，在 B 上启用 “cpu” 会在 C 和 D 中创建以 “cpu.” 为前缀的控制器接口文件。同样地，从 B 禁用 “memory” 会从 C 和 D 中移除以 “memory.” 为前缀的控制器接口文件。这意味着控制器接口文件——任何不以 “cgroup.” 开头的文件——由父节点而非 cgroup 自身拥有。


#### 自上而下约束


资源是自上而下分配的，一个 cgroup 只有在父节点已向它分配了某资源之后，才能进一步分配该资源。这意味着所有非根的 “cgroup.subtree_control” 文件只能包含在其父节点的 “cgroup.subtree_control” 文件中启用的控制器。只有当父节点启用了某个控制器时，该控制器才能被启用；而如果有一个或多个子节点启用了某控制器，则该控制器不能被禁用。


#### 无内部进程约束


非根 cgroup 只有在自身没有任何进程时，才能向子节点分配域资源。换言之，只有不包含任何进程的域 cgroup 才能在其 “cgroup.subtree_control” 文件中启用域控制器。

这保证了：当某个域控制器观察已启用它的那部分层次时，进程永远只位于叶子节点。这就排除了子 cgroup 与父节点内部进程相互竞争的情况。

根 cgroup 不受此限制。根节点包含进程以及无法与任何其他 cgroup 关联的匿名资源消耗，需要大多数控制器的特殊处理。根 cgroup 中的资源消耗如何治理取决于各个控制器（有关此主题的更多信息，请参阅“控制器”一章中的“非规范性信息”一节）。

注意，如果 cgroup 的 “cgroup.subtree_control” 中没有启用任何控制器，该限制并不会造成阻碍。这一点很重要，否则将无法创建已填充 cgroup 的子节点。要控制某个 cgroup 的资源分配，该 cgroup 必须先创建子节点，并将自身的全部进程转移到这些子节点，然后才能在自己的 “cgroup.subtree_control” 文件中启用控制器。


### 委托


#### 委托模型


cgroup 可以通过两种方式被委托。第一种，通过授予某低特权用户对目录及其 “cgroup.procs”、“cgroup.threads” 和 “cgroup.subtree_control” 文件的写权限，委托给该用户。第二种，如果设置了 “nsdelegate” 挂载选项，则在创建命名空间时自动委托给某个 cgroup 命名空间。

由于给定目录中的资源控制接口文件控制的是父节点资源的分配，不应允许被委托方写入这些文件。对于第一种方式，这通过不授予对这些文件的访问权限来实现。对于第二种方式，命名空间之外的文件应通过至少挂载命名空间化的手段对委托方隐藏，并且内核会拒绝从 cgroup 命名空间内部对命名空间根上的所有文件进行写入，但 “/sys/kernel/cgroup/delegate” 中列出的文件（包括 “cgroup.procs”、“cgroup.threads”、“cgroup.subtree_control” 等）除外。

两种委托类型的最终结果是等价的。一旦被委托，用户就可以在该目录下构建子层次、按照自己的需要组织其中的进程，并进一步分配从父节点获得的资源。所有资源控制器的限额及其他设置都是分层的，无论被委托的子层次中发生什么，都没有任何东西能够逃脱父节点施加的资源限制。

目前，cgroup 并未对委托子层次中的 cgroup 数量或其嵌套深度施加任何限制；不过将来可能会显式地加以限制。


#### 委托 containment（隔离）


被委托的子层次是受 containment 约束的，即进程不能被委托方移入或移出该子层次。

对于委托给低特权用户的情况，这通过要求以下条件来实现：一个具有非根 euid 的进程，若要通过向 “cgroup.procs” 文件写入 PID 来将目标进程迁移进某个 cgroup，则必须满足：

- 写入者必须对 “cgroup.procs” 文件具有写权限。

- 写入者必须对源 cgroup 与目的 cgroup 的共同祖先的 “cgroup.procs” 文件具有写权限。

上述两个约束确保：虽然委托方可以在被委托的子层次中自由迁移进程，但它无法从子层次之外拉入进程，也无法将进程推到子层次之外。

举例来说，假设 cgroup C0 和 C1 已被委托给用户 U0，U0 在 C0 下创建了 C00、C01，在 C1 下创建了 C10，如下所示
```

  ~~~~~~~~~~~~~ - C0 - C00
  ~ cgroup    ~      \ C01
  ~ hierarchy ~
  ~~~~~~~~~~~~~ - C1 - C10

```
再假设 U0 想把当前位于 C10 的某个进程的 PID 写入 “C00/cgroup.procs”。U0 对该文件有写权限；然而，源 cgroup C10 与目的 cgroup C00 的共同祖先位于委托点之上，U0 对其 “cgroup.procs” 文件没有写权限，因此该写入将以 -EACCES 被拒绝。

对于委托给命名空间的情况，containment 通过要求源 cgroup 和目的 cgroup 都能从尝试迁移的进程所在的命名空间到达来实现。如果其中任何一个不可达，则该迁移以 -ENOENT 被拒绝。


### 准则


#### 组织一次并控制


跨 cgroup 迁移进程是一个相对昂贵的操作，而内存等有状态的资源不会随进程一起移动。这是一个显式的设计决策，因为在同步代价方面，迁移与各种热路径之间往往存在固有的权衡。

因此，不鼓励将频繁跨 cgroup 迁移进程作为一种施加不同资源限制的手段。工作负载应在启动时根据系统的逻辑与资源结构一次性分配到某个 cgroup。可以通过接口文件更改控制器配置来对资源分配进行动态调整。


#### 避免名称冲突


一个 cgroup 与其子 cgroup 的接口文件占据同一目录，因此有可能创建出与接口文件冲突的子 cgroup。

所有 cgroup 核心接口文件都以 “cgroup.” 为前缀，每个控制器的接口文件都以控制器名加一个点作为前缀。控制器的名称由小写字母和 “_” 组成，但绝不以 “_” 开头，因此它可以作为前缀字符用于避免冲突。此外，接口文件名不会以常用于对工作负载分类的术语开头或结尾，例如 job、service、slice、unit 或 workload。

cgroup 不做任何事情来防止名称冲突，避免冲突是用户的责任。


## 资源分配模型


cgroup 控制器根据资源类型与预期使用场景实现了若干种资源分配方案。本节描述正在使用的主要方案及其预期行为。


### 权重


父节点的资源通过把所有活动子节点的权重相加、并按各自权重占权重之和的比例进行分配。由于只有当前能利用该资源的子节点才参与分配，这是一种工作保持（work-conserving）的方式。由于这种动态特性，该模型通常用于无状态的资源。

所有权重都在 [1, 10000] 范围内，默认值为 100。这允许在两个方向上以足够精细的粒度进行对称的乘法偏置，同时保持在直观的范围内。

只要权重在范围内，所有配置组合都是有效的，没有理由拒绝配置变更或进程迁移。

“cpu.weight” 按比例将 CPU 周期分配给活动子节点，就是这类的一个例子。


### 限额


子节点最多只能消费配置量的资源。限额可以被过度承诺（over-committed）——子节点限额之和可以超过父节点可用的资源量。

限额范围是 [0, max]，默认值为 “max”，即空操作（noop）。

由于限额可以被过度承诺，所有配置组合都是有效的，没有理由拒绝配置变更或进程迁移。

“io.max” 限制一个 cgroup 在某一 IO 设备上可消费的最大 BPS 和/或 IOPS，就是这类的一个例子。


### 保护


只要某 cgroup 的所有祖先的使用量都低于其受保护级别，该 cgroup 就会得到配置量资源的保护。保护可以是硬保证，也可以是尽力而为的软边界。保护也可以被过度承诺，这种情况下子节点之间只有父节点可用量范围内的一部分受到保护。

保护范围是 [0, max]，默认值为 0，即空操作（noop）。

由于保护可以被过度承诺，所有配置组合都是有效的，没有理由拒绝配置变更或进程迁移。

“memory.low” 实现了尽力而为的内存保护，就是这类的一个例子。


### 分配


一个 cgroup 被独占地分配某有限资源的一定数量。分配不能被过度承诺——子节点分配之和不能超过父节点可用的资源量。

分配范围是 [0, max]，默认值为 0，即无资源。

由于分配不能被过度承诺，某些配置组合是无效的，应被拒绝。此外，如果该资源是进程执行所必需的，进程迁移可能会被拒绝。


## 接口文件


### 格式


所有接口文件在可能的情况下都应采用以下格式之一
```

  New-line separated values
  (when only one value can be written at once)

	VAL0\n
	VAL1\n
	...

  Space separated values
  (when read-only or multiple values can be written at once)

	VAL0 VAL1 ...\n

  Flat keyed

	KEY0 VAL0\n
	KEY1 VAL1\n
	...

  Nested keyed

	KEY0 SUB_KEY0=VAL00 SUB_KEY1=VAL01...
	KEY1 SUB_KEY0=VAL10 SUB_KEY1=VAL11...
	...

```
对于可写文件，写入的格式通常应与读取格式匹配；不过，控制器可能允许省略后面的字段，或针对最常见的使用场景实现受限的快捷方式。

对于扁平键值（flat keyed）和嵌套键值（nested keyed）文件，每次只能写入单个键对应的值。对于嵌套键值文件，子键对可以以任意顺序指定，且不必指定所有键值对。


### 约定


- 单一特性的设置应当包含在一个单一文件中。

- 根 cgroup 应不受资源控制约束，因此不应有资源控制接口文件。

- 默认时间单位是微秒。如果使用了不同的单位，必须带显式的单位后缀。

- 以“每单位中所占比例（parts-per）”表示的量应使用带有至少两位小数部分的百分比小数——例如 13.40。

- 如果某个控制器实现了基于权重的资源分配，其接口文件应命名为 “weight”，范围 [1, 10000]，默认值 100。这些取值的选择是为了在两个方向上都能提供足够且对称的偏置，同时保持直观（默认是 100%）。

- 如果某个控制器实现了绝对的资源保证和/或限制，接口文件应分别命名为 “min” 和 “max”。如果某个控制器实现了尽力而为的资源保证和/或限制，接口文件应分别命名为 “low” 和 “high”。

  在上述四个控制文件中，特殊标记 “max” 应用来表示读取和写入时的向上无穷大。

- 如果一个设置具有可配置的默认值以及按键的特定覆盖值，默认条目应以 “default” 为键，并作为文件中的第一个条目出现。

  默认值可以通过写入 “default $VAL” 或 “$VAL” 来更新。

  写入以更新某个特定覆盖值时，可以使用 “default” 作为值，表示移除该覆盖。值为 “default” 的覆盖条目在读取时不得出现。

  例如，一个以主:次设备号（major:minor）为键的设置
```

    # cat cgroup-example-interface-file
    default 150
    8:0 300

  The default value can be updated by::

    # echo 125 > cgroup-example-interface-file

  or::

    # echo "default 125" > cgroup-example-interface-file

  An override can be set by::

    # echo "8:16 170" > cgroup-example-interface-file

  and cleared by::

    # echo "8:0 default" > cgroup-example-interface-file
    # cat cgroup-example-interface-file
    default 125
    8:16 170

```
- 对于频率不太高的事件，应创建一个接口文件 “events”，其中列出事件的键值对。每当发生可通知的事件时，应在该文件上生成文件修改事件。


### 核心接口文件


所有 cgroup 核心文件都以 “cgroup.” 为前缀。

  cgroup.type
	A read-write single value file which exists on non-root
	cgroups.

	When read, it indicates the current type of the cgroup, which
	can be one of the following values.

 - "domain" : 一个普通且有效的域 cgroup。

 - "domain threaded" : 作为一个线程化子树之根的线程化域 cgroup。

 - "domain invalid" : 处于无效状态的 cgroup。
	  它不能被填充，也不能启用控制器。它可能
	  被允许成为线程化 cgroup。

 - "threaded" : 作为线程化子树成员的线程化 cgroup。

	A cgroup can be turned into a threaded cgroup by writing
	"threaded" to this file.

  cgroup.procs
	A read-write new-line separated values file which exists on
	all cgroups.

	When read, it lists the PIDs of all processes which belong to
	the cgroup one-per-line.  The PIDs are not ordered and the
	same PID may show up more than once if the process got moved
	to another cgroup and then back or the PID got recycled while
	reading.

	A PID can be written to migrate the process associated with
	the PID to the cgroup.  The writer should match all of the
	following conditions.

 - It must have write access to the "cgroup.procs" file.

 - It must have write access to the "cgroup.procs" file of the
	  common ancestor of the source and destination cgroups.

	When delegating a sub-hierarchy, write access to this file
	should be granted along with the containing directory.

	In a threaded cgroup, reading this file fails with EOPNOTSUPP
	as all the processes belong to the thread root.  Writing is
	supported and moves every thread of the process to the cgroup.

  cgroup.threads
	A read-write new-line separated values file which exists on
	all cgroups.

	When read, it lists the TIDs of all threads which belong to
	the cgroup one-per-line.  The TIDs are not ordered and the
	same TID may show up more than once if the thread got moved to
	another cgroup and then back or the TID got recycled while
	reading.

	A TID can be written to migrate the thread associated with the
	TID to the cgroup.  The writer should match all of the
	following conditions.

 - It must have write access to the "cgroup.threads" file.

 - The cgroup that the thread is currently in must be in the
          same resource domain as the destination cgroup.

 - It must have write access to the "cgroup.procs" file of the
	  common ancestor of the source and destination cgroups.

	When delegating a sub-hierarchy, write access to this file
	should be granted along with the containing directory.

  cgroup.controllers
	A read-only space separated values file which exists on all
	cgroups.

	It shows space separated list of all controllers available to
	the cgroup.  The controllers are not ordered.

  cgroup.subtree_control
	A read-write space separated values file which exists on all
	cgroups.  Starts out empty.

	When read, it shows space separated list of the controllers
	which are enabled to control resource distribution from the
	cgroup to its children.

	Space separated list of controllers prefixed with '+' or '-'
	can be written to enable or disable controllers.  A controller
	name prefixed with '+' enables the controller and '-'
	disables.  If a controller appears more than once on the list,
	the last one is effective.  When multiple enable and disable
	operations are specified, either all succeed or all fail.

  cgroup.events
	A read-only flat-keyed file which exists on non-root cgroups.
	The following entries are defined.  Unless specified
	otherwise, a value change in this file generates a file
	modified event.

	  populated
		如果该 cgroup 或其后代中包含任何活动进程则为 1；否则为 0。
	  frozen
		如果该 cgroup 被冻结则为 1；否则为 0。

  cgroup.max.descendants
	A read-write single value files.  The default is "max".

	允许的最大后代 cgroup 数量。
	如果实际的后代数量等于或大于此值，
	在该层次中创建新 cgroup 的尝试将失败。

  cgroup.max.depth
	A read-write single value files.  The default is "max".

	当前 cgroup 之下允许的最大后代深度。
	如果实际的后代深度等于或大于此值，
	创建新子 cgroup 的尝试将失败。

  cgroup.stat
	A read-only flat-keyed file with the following entries:

	  nr_descendants
		可见后代 cgroup 的总数。

	  nr_dying_descendants
		处于消亡（dying）状态的 descendant cgroup 总数。一个 cgroup 在被用户删除后进入消亡状态。该 cgroup 在完全被销毁之前，会在消亡状态保持一段未定义的时间（可能取决于系统负载）。

		A process can't enter a dying cgroup under any circumstances,
		a dying cgroup can't revive.

		A dying cgroup can consume system resources not exceeding
		limits, which were active at the moment of cgroup deletion.

	  nr_subsys_<cgroup_subsys>
		当前 cgroup 及其之下处于活动状态的 cgroup 子系统（例如 memory cgroup）总数。

	  nr_dying_subsys_<cgroup_subsys>
		当前 cgroup 及其之下处于消亡状态的 cgroup 子系统（例如 memory cgroup）总数。

  cgroup.stat.local
	A read-only flat-keyed file which exists in non-root cgroups.
	The following entry is defined:

	  frozen_usec
		Cumulative time that this cgroup has spent between freezing and
		thawing, regardless of whether by self or ancestor groups.
		NB: (not) reaching "frozen" state is not accounted here.

		Using the following ASCII representation of a cgroup's freezer
```

			       1    _____
			frozen 0 __/     \__
			          ab    cd

		the duration being measured is the span between a and c.

  cgroup.freeze
	A read-write single value file which exists on non-root cgroups.
	Allowed values are "0" and "1". The default is "0".

	Writing "1" to the file causes freezing of the cgroup and all
	descendant cgroups. This means that all belonging processes will
	be stopped and will not run until the cgroup will be explicitly
	unfrozen. Freezing of the cgroup may take some time; when this action
	is completed, the "frozen" value in the cgroup.events control file
	will be updated to "1" and the corresponding notification will be
	issued.

	A cgroup can be frozen either by its own settings, or by settings
	of any ancestor cgroups. If any of ancestor cgroups is frozen, the
	cgroup will remain frozen.

	Processes in the frozen cgroup can be killed by a fatal signal.
	They also can enter and leave a frozen cgroup: either by an explicit
	move by a user, or if freezing of the cgroup races with fork().
	If a process is moved to a frozen cgroup, it stops. If a process is
	moved out of a frozen cgroup, it becomes running.

	Frozen status of a cgroup doesn't affect any cgroup tree operations:
	it's possible to delete a frozen (and empty) cgroup, as well as
	create new sub-cgroups.

  cgroup.kill
	A write-only single value file which exists in non-root cgroups.
	The only allowed value is "1".

	Writing "1" to the file causes the cgroup and all descendant cgroups to
	be killed. This means that all processes located in the affected cgroup
	tree will be killed via SIGKILL.

	Killing a cgroup tree will deal with concurrent forks appropriately and
	is protected against migrations.

	In a threaded cgroup, writing this file fails with EOPNOTSUPP as
	killing cgroups is a process directed operation, i.e. it affects
	the whole thread-group.

  cgroup.pressure
	A read-write single value file that allowed values are "0" and "1".
	The default is "1".

	Writing "0" to the file will disable the cgroup PSI accounting.
	Writing "1" to the file will re-enable the cgroup PSI accounting.

	This control attribute is not hierarchical, so disable or enable PSI
	accounting in a cgroup does not affect PSI accounting in descendants
	and doesn't need pass enablement via ancestors from root.

	The reason this control attribute exists is that PSI accounts stalls for
	each cgroup separately and aggregates it at each level of the hierarchy.
	This may cause non-negligible overhead for some workloads when under
	deep level of the hierarchy, in which case this control attribute can
	be used to disable PSI accounting in the non-leaf cgroups.

  irq.pressure
	A read-write nested-keyed file.

	Shows pressure stall information for IRQ/SOFTIRQ. See
	:ref:`Documentation/accounting/psi.rst <psi>` for details.

```
## 控制器



### CPU


“cpu” 控制器调节 CPU 周期的分配。该控制器对普通调度策略实现了权重与绝对带宽限制模型，对实时调度策略实现了绝对带宽分配模型。

在上述所有模型中，周期分配仅基于时间定义，并不考虑任务被执行的频率。可选的利用率钳制（utilization clamping）支持允许向 schedutil cpufreq 调速器暗示某个 CPU 应始终提供的最小期望频率，以及不应超过的最大期望频率。

警告：cgroup2 的 cpu 控制器尚不支持对实时进程的（带宽）控制。对于编译时启用了 CONFIG_RT_GROUP_SCHED 选项以支持实时进程分组调度的内核，只有当所有 RT 进程都位于根 cgroup 时，才能启用 cpu 控制器。请注意，系统管理软件可能在系统启动过程中已经将 RT 进程放入了非根 cgroup，在启用 CONFIG_RT_GROUP_SCHED 的内核上启用 cpu 控制器之前，可能需要先将这些进程移动到根 cgroup。

在禁用 CONFIG_RT_GROUP_SCHED 的情况下，此限制不适用，部分接口文件要么影响实时进程，要么对它们记账。详见下一节。只有 cpu 控制器受 CONFIG_RT_GROUP_SCHED 影响。其他控制器无论 CONFIG_RT_GROUP_SCHED 如何，都可用于实时进程的资源控制。


#### CPU 接口文件


进程与 cpu 控制器的交互取决于其调度策略与底层调度器。从 cpu 控制器的角度看，进程可分类如下：

- 处于公平类（fair-class）调度器下的进程
- 使用带有 `cgroup_set_weight` 回调的 BPF 调度器下的进程
- 其他一切：`SCHED_{FIFO,RR,DEADLINE}` 以及使用不带 `cgroup_set_weight` 回调的 BPF 调度器下的进程

关于进程何时处于公平类调度器或 BPF 调度器之下，请参阅 Documentation/scheduler/sched-ext.rst <sched-ext>。

对于以下每个接口文件，都会引用上述分类。所有时间时长均以微秒为单位。

  cpu.stat
	A read-only flat-keyed file.
	This file exists whether the controller is enabled or not.

	It always reports the following three stats, which account for all the
	processes in the cgroup:

 - usage_usec
 - user_usec
 - system_usec

	and the following five when the controller is enabled, which account for
	only the processes under the fair-class scheduler:

 - nr_periods
 - nr_throttled
 - throttled_usec
 - nr_bursts
 - burst_usec

  cpu.weight
	A read-write single value file which exists on non-root
	cgroups.  The default is "100".

	For non idle groups (cpu.idle = 0), the weight is in the
	range [1, 10000].

	If the cgroup has been configured to be SCHED_IDLE (cpu.idle = 1),
	then the weight will show as a 0.

	This file affects only processes under the fair-class scheduler and a BPF
	scheduler with the `cgroup_set_weight` callback depending on what the
	callback actually does.

  cpu.weight.nice
	A read-write single value file which exists on non-root
	cgroups.  The default is "0".

	The nice value is in the range [-20, 19].

	This interface file is an alternative interface for
	"cpu.weight" and allows reading and setting weight using the
	same values used by nice(2).  Because the range is smaller and
	granularity is coarser for the nice values, the read value is
	the closest approximation of the current weight.

	This file affects only processes under the fair-class scheduler and a BPF
	scheduler with the `cgroup_set_weight` callback depending on what the
	callback actually does.

  cpu.max
	A read-write two value file which exists on non-root cgroups.
	The default is "max 100000".

```

	  $MAX $PERIOD

	which indicates that the group may consume up to $MAX in each
	$PERIOD duration.  "max" for $MAX indicates no limit.  If only
	one number is written, $MAX is updated.

	This file affects only processes under the fair-class scheduler.

  cpu.max.burst
	A read-write single value file which exists on non-root
	cgroups.  The default is "0".

	The burst in the range [0, $MAX].

	This file affects only processes under the fair-class scheduler.

  cpu.pressure
	A read-write nested-keyed file.

	Shows pressure stall information for CPU. See
	:ref:`Documentation/accounting/psi.rst <psi>` for details.

	This file accounts for all the processes in the cgroup.

  cpu.uclamp.min
	A read-write single value file which exists on non-root cgroups.
	The default is "0", i.e. no utilization boosting.

	The requested minimum utilization (protection) as a percentage
	rational number, e.g. 12.34 for 12.34%.

	This interface allows reading and setting minimum utilization clamp
	values similar to the sched_setattr(2). This minimum utilization
	value is used to clamp the task specific minimum utilization clamp,
	including those of realtime processes.

	The requested minimum utilization (protection) is always capped by
	the current value for the maximum utilization (limit), i.e.
	`cpu.uclamp.max`.

	This file affects all the processes in the cgroup.

  cpu.uclamp.max
	A read-write single value file which exists on non-root cgroups.
	The default is "max". i.e. no utilization capping

	The requested maximum utilization (limit) as a percentage rational
	number, e.g. 98.76 for 98.76%.

	This interface allows reading and setting maximum utilization clamp
	values similar to the sched_setattr(2). This maximum utilization
	value is used to clamp the task specific maximum utilization clamp,
	including those of realtime processes.

	This file affects all the processes in the cgroup.

  cpu.idle
	A read-write single value file which exists on non-root cgroups.
	The default is 0.

	This is the cgroup analog of the per-task SCHED_IDLE sched policy.
	Setting this value to a 1 will make the scheduling policy of the
	cgroup SCHED_IDLE. The threads inside the cgroup will retain their
	own relative priorities, but the cgroup itself will be treated as
	very low priority relative to its peers.

	This file affects only processes under the fair-class scheduler.

```
### 内存


“memory” 控制器调节内存的分配。内存是有状态的，同时实现了限制与保护模型。由于内存使用与回收压力之间错综复杂的关系，以及内存的有状态特性，其分配模型相对复杂。

虽然并非完全滴水不漏，但一个给定 cgroup 的所有主要内存用量都受到跟踪，从而可以在合理程度上对总内存消耗进行记账与控制。目前，跟踪以下几类内存用量。

- 用户空间内存——页缓存与匿名内存。

- 内核数据结构，例如 dentries 与 inodes。

- TCP 套接字缓冲区。

上述列表未来可能会扩展以获得更好的覆盖度。


#### 内存接口文件


所有内存量都以字节为单位。如果写入的值未对齐到 PAGE_SIZE，读回时该值可能会被向上取整到最接近的 PAGE_SIZE 倍数。

  memory.current
	A read-only single value file which exists on non-root
	cgroups.

	该 cgroup 及其后代当前正在使用的总内存量。

  memory.min
	A read-write single value file which exists on non-root
	cgroups.  The default is "0".

	Hard memory protection.  If the memory usage of a cgroup
	is within its effective min boundary, the cgroup's memory
	won't be reclaimed under any conditions. If there is no
	unprotected reclaimable memory available, OOM killer
	is invoked. Above the effective min boundary (or
	effective low boundary if it is higher), pages are reclaimed
	proportionally to the overage, reducing reclaim pressure for
	smaller overages.

	Effective min boundary is limited by memory.min values of
	ancestor cgroups. If there is memory.min overcommitment
	(child cgroup or cgroups are requiring more protected memory
	than parent will allow), then each child cgroup will get
	the part of parent's protection proportional to its
	actual memory usage below memory.min.

	Putting more memory than generally available under this
	protection is discouraged and may lead to constant OOMs.

  memory.low
	A read-write single value file which exists on non-root
	cgroups.  The default is "0".

	Best-effort memory protection.  If the memory usage of a
	cgroup is within its effective low boundary, the cgroup's
	memory won't be reclaimed unless there is no reclaimable
	memory available in unprotected cgroups.
	Above the effective low	boundary (or 
	effective min boundary if it is higher), pages are reclaimed
	proportionally to the overage, reducing reclaim pressure for
	smaller overages.

	Effective low boundary is limited by memory.low values of
	ancestor cgroups. If there is memory.low overcommitment
	(child cgroup or cgroups are requiring more protected memory
	than parent will allow), then each child cgroup will get
	the part of parent's protection proportional to its
	actual memory usage below memory.low.

	Putting more memory than generally available under this
	protection is discouraged.

  memory.high
	A read-write single value file which exists on non-root
	cgroups.  The default is "max".

	Memory usage throttle limit.  If a cgroup's usage goes
	over the high boundary, the processes of the cgroup are
	throttled and put under heavy reclaim pressure.

	Going over the high limit never invokes the OOM killer and
	under extreme conditions the limit may be breached. The high
	limit should be used in scenarios where an external process
	monitors the limited cgroup to alleviate heavy reclaim
	pressure.

	If memory.high is opened with O_NONBLOCK then the synchronous
	reclaim is bypassed. This is useful for admin processes that
	need to dynamically adjust the job's memory limits without
	expending their own CPU resources on memory reclamation. The
	job will trigger the reclaim and/or get throttled on its
	next charge request.

	Please note that with O_NONBLOCK, there is a chance that the
	target memory cgroup may take indefinite amount of time to
	reduce usage below the limit due to delayed charge request or
	busy-hitting its memory to slow down reclaim.

  memory.max
	A read-write single value file which exists on non-root
	cgroups.  The default is "max".

	Memory usage hard limit.  This is the main mechanism to limit
	memory usage of a cgroup.  If a cgroup's memory usage reaches
	this limit and can't be reduced, the OOM killer is invoked in
	the cgroup. Under certain circumstances, the usage may go
	over the limit temporarily.

	In default configuration regular 0-order allocations always
	succeed unless OOM killer chooses current task as a victim.

	Some kinds of allocations don't invoke the OOM killer.
	Caller could retry them differently, return into userspace
	as -ENOMEM or silently ignore in cases like disk readahead.

	If memory.max is opened with O_NONBLOCK, then the synchronous
	reclaim and oom-kill are bypassed. This is useful for admin
	processes that need to dynamically adjust the job's memory limits
	without expending their own CPU resources on memory reclamation.
	The job will trigger the reclaim and/or oom-kill on its next
	charge request.

	Please note that with O_NONBLOCK, there is a chance that the
	target memory cgroup may take indefinite amount of time to
	reduce usage below the limit due to delayed charge request or
	busy-hitting its memory to slow down reclaim.

  memory.reclaim
	A write-only nested-keyed file which exists for all cgroups.

	This is a simple interface to trigger memory reclaim in the
	target cgroup.

```

	  echo "1G" > memory.reclaim

	Please note that the kernel can over or under reclaim from
	the target cgroup. If less bytes are reclaimed than the
	specified amount, -EAGAIN is returned.

	Please note that the proactive reclaim (triggered by this
	interface) is not meant to indicate memory pressure on the
	memory cgroup. Therefore socket memory balancing triggered by
	the memory reclaim normally is not exercised in this case.
	This means that the networking layer will not adapt based on
	reclaim induced by memory.reclaim.

```
The following nested keys are defined.

	  ==========            ================================
	  swappiness            Swappiness value to reclaim with
	  ==========            ================================

	Specifying a swappiness value instructs the kernel to perform
	the reclaim with that swappiness value. Note that this has the
	same semantics as vm.swappiness applied to memcg reclaim with
	all the existing limitations and potential future extensions.

	The valid range for swappiness is [0-200, max], setting
	swappiness=max exclusively reclaims anonymous memory.

  memory.peak
	A read-write single value file which exists on non-root cgroups.

	The max memory usage recorded for the cgroup and its descendants since
	either the creation of the cgroup or the most recent reset for that FD.

	A write of any non-empty string to this file resets it to the
	current memory usage for subsequent reads through the same
	file descriptor.

  memory.oom.group
	A read-write single value file which exists on non-root
	cgroups.  The default value is "0".

	Determines whether the cgroup should be treated as
	an indivisible workload by the OOM killer. If set,
	all tasks belonging to the cgroup or to its descendants
	(if the memory cgroup is not a leaf cgroup) are killed
	together or not at all. This can be used to avoid
	partial kills to guarantee workload integrity.

	Tasks with the OOM protection (oom_score_adj set to -1000)
	are treated as an exception and are never killed.

	If the OOM killer is invoked in a cgroup, it's not going
	to kill any tasks outside of this cgroup, regardless
	memory.oom.group values of ancestor cgroups.

  memory.events
	A read-only flat-keyed file which exists on non-root cgroups.
	The following entries are defined.  Unless specified
	otherwise, a value change in this file generates a file
	modified event.

	Note that all fields in this file are hierarchical and the
	file modified event can be generated due to an event down the
	hierarchy. For the local events at the cgroup level see
	memory.events.local.

	  low
		The number of times the cgroup is reclaimed due to
		high memory pressure even though its usage is under
		the low boundary.  This usually indicates that the low
		boundary is over-committed.

	  high
		The number of times processes of the cgroup are
		throttled and routed to perform direct memory reclaim
		because the high memory boundary was exceeded.  For a
		cgroup whose memory usage is capped by the high limit
		rather than global memory pressure, this event's
		occurrences are expected.

	  max
		The number of times the cgroup's memory usage was
		about to go over the max boundary.  If direct reclaim
		fails to bring it down, the cgroup goes to OOM state.

	  oom
		The number of time the cgroup's memory usage was
		reached the limit and allocation was about to fail.

		This event is not raised if the OOM killer is not
		considered as an option, e.g. for failed high-order
		allocations or if caller asked to not retry attempts.

	  oom_kill
		The number of processes belonging to this cgroup
		killed by any kind of OOM killer.

          oom_group_kill
                The number of times a group OOM has occurred.

          sock_throttled
                The number of times network sockets associated with
                this cgroup are throttled.

  memory.events.local
	Similar to memory.events but the fields in the file are local
	to the cgroup i.e. not hierarchical. The file modified event
	generated on this file reflects only the local events.

  memory.stat
	A read-only flat-keyed file which exists on non-root cgroups.

	This breaks down the cgroup's memory footprint into different
	types of memory, type-specific details, and other information
	on the state and past events of the memory management system.

	All memory amounts are in bytes.

	The entries are ordered to be human readable, and new entries
	can show up in the middle. Don't rely on items remaining in a
	fixed position; use the keys to look up specific values!

	If the entry has no per-node counter (or not show in the
	memory.numa_stat). We use 'npn' (non-per-node) as the tag
	to indicate that it will not show in the memory.numa_stat.

	  anon
		Amount of memory used in anonymous mappings such as
		brk(), sbrk(), and mmap(MAP_ANONYMOUS). Note that
		some kernel configurations might account complete larger
		allocations (e.g., THP) if only some, but not all the
		memory of such an allocation is mapped anymore.

	  file
		用于缓存文件系统数据的内存量，包括 tmpfs 与共享内存。

	  kernel (npn)
		内核内存总量，包括
		(kernel_stack, pagetables, percpu, vmalloc, slab) 以及
		其它内核内存使用场景。

	  kernel_stack
		分配给内核栈的内存量。

	  pagetables
                Amount of memory allocated for page tables.

	  sec_pagetables
		用于二级页表（secondary page tables）的内存量，
		目前包括 x86 与 arm64 上的 KVM mmu 分配，以及 IOMMU 页表。

	  percpu (npn)
		用于存储每 CPU 内核数据结构的内存量。

	  sock (npn)
		用于网络传输缓冲区的的内存量。

	  vmalloc (npn)
		用于 vmap 后备内存的内存量。

	  shmem
		被交换（swap）支持的已缓存文件系统数据量，
		例如 tmpfs、shm 段、共享匿名 mmap()。

	  zswap
		被 zswap 压缩后端消耗的内存量。

	  zswapped
		被交换到 zswap 的应用程序内存量。

	  file_mapped
		通过 mmap() 映射的已缓存文件系统数据量。注意
		某些内核配置可能将整个更大的分配（例如 THP）记账，
		如果此类分配中只有部分（而非全部）内存仍被映射。

	  file_dirty
		被修改但尚未写回磁盘的已缓存文件系统数据量。

	  file_writeback
		已修改且当前正在写回磁盘的已缓存文件系统数据量。

	  swapcached
		缓存在内存中的交换量。swapcache 同时计入内存使用与交换使用。

	  anon_thp
		由透明大页（transparent hugepages）支持的匿名映射所使用的内存量。

	  file_thp
		由透明大页支持的已缓存文件系统数据量。

	  shmem_thp
		由透明大页支持的 shm、tmpfs、共享匿名 mmap() 量。

	  inactive_anon, active_anon, inactive_file, active_file, unevictable
		基于内部内存管理列表（被页回收算法使用）的内存与交换、文件系统支持的交换量。

		As these represent internal list state (eg. shmem pages are on anon
		memory management lists), inactive_foo + active_foo may not be equal to
		the value for the foo counter, since the foo counter is type-based, not
		list-based.

	  slab_reclaimable
		“slab” 中可能被回收的部分，例如 dentries 与 inodes。

	  slab_unreclaimable
		在内存压力下无法被回收的 “slab” 部分。

	  slab (npn)
		用于存储内核内部数据结构的内存量。

	  workingset_refault_anon
		之前被驱逐的匿名页发生再次缺页（refault）的次数。

	  workingset_refault_file
		之前被驱逐的文件页发生再次缺页的次数。

	  workingset_activate_anon
		被立即激活的再次缺页匿名页数量。

	  workingset_activate_file
		被立即激活的再次缺页文件页数量。

	  workingset_restore_anon
		在被回收之前被检测为活动 workingset 的、已恢复的匿名页数量。

	  workingset_restore_file
		在被回收之前被检测为活动 workingset 的、已恢复的文件页数量。

	  workingset_nodereclaim
		影子节点（shadow node）被回收的次数。

	  pswpin (npn)
		换入内存的页数。

	  pswpout (npn)
		换出内存的页数。

	  pgscan (npn)
		已扫描的页数（在非活动 LRU 列表中）。

	  pgsteal (npn)
		已回收的页数。

	  pgscan_kswapd (npn)
		kswapd 扫描的页数（在非活动 LRU 列表中）。

	  pgscan_direct (npn)
		直接扫描的页数（在非活动 LRU 列表中）。

	  pgscan_khugepaged (npn)
		khugepaged 扫描的页数（在非活动 LRU 列表中）。

	  pgscan_proactive (npn)
		主动扫描的页数（在非活动 LRU 列表中）。

	  pgsteal_kswapd (npn)
		kswapd 回收的页数。

	  pgsteal_direct (npn)
		直接回收的页数。

	  pgsteal_khugepaged (npn)
		khugepaged 回收的页数。

	  pgsteal_proactive (npn)
		主动回收的页数。

	  pgfault (npn)
		发生的总缺页次数。

	  pgmajfault (npn)
		发生的主要缺页（major page fault）次数。

	  pgrefill (npn)
		已扫描的页数（在活动 LRU 列表中）。

	  pgactivate (npn)
		移动到活动 LRU 列表的页数。

	  pgdeactivate (npn)
		移动到非活动 LRU 列表的页数。

	  pglazyfree (npn)
		在内存压力下被推迟释放的页数。

	  pglazyfreed (npn)
		已回收的 lazyfree 页数。

	  swpin_zero
		换入内存并填充为零的页数，其中由于交换出时检测到页内容为零而优化了 I/O。

	  swpout_zero
		因内容被检测为零而跳过 I/O 的、被填充为零并换出的页数。

	  zswpin
		从 zswap 移入内存的页数。

	  zswpout
		从内存移出到 zswap 的页数。

	  zswpwb
		从 zswap 写入交换的页数。

	  zswap_incomp
		当前未经压缩存储在 zswap 中的不可压缩页数。
		这些页无法被压缩到小于 PAGE_SIZE 的尺寸，因此按原样存储。

	  thp_fault_alloc (npn)
		为满足一次缺页而分配的透明大页数量。在未设置 CONFIG_TRANSPARENT_HUGEPAGE
                时不出现此计数器。

	  thp_collapse_alloc (npn)
		为允许将一段现有页范围折叠（collapse）而分配的透明大页数量。在未设置
		CONFIG_TRANSPARENT_HUGEPAGE 时不出现此计数器。

	  thp_swpout (npn)
		不经拆分、一次性整体交换出的透明大页数量。

	  thp_swpout_fallback (npn)
		在交换出之前被拆分的透明大页数量。
		通常是因为未能为这个大页分配某些连续的交换空间。

	  numa_pages_migrated (npn)
		NUMA 平衡迁移的页数。

	  numa_pte_updates (npn)
		其页表项被 NUMA 平衡修改以在访问时产生 NUMA 提示缺页（hinting fault）的页数。

	  numa_hint_faults (npn)
		NUMA 提示缺页的次数。

	  pgdemote_kswapd
		kswapd 降级（demote）的页数。

	  pgdemote_direct
		直接降级的页数。

	  pgdemote_khugepaged
		khugepaged 降级的页数。

	  pgdemote_proactive
		主动降级的页数。

	  hugetlb
		由 hugetlb 页使用的内存量。仅当 hugetlb 用量在 memory.current 中被记账时
		（即 cgroup 以 memory_hugetlb_accounting 选项挂载）才会出现此指标。

  memory.numa_stat
	A read-only nested-keyed file which exists on non-root cgroups.

	This breaks down the cgroup's memory footprint into different
	types of memory, type-specific details, and other information
	per node on the state of the memory management system.

	This is useful for providing visibility into the NUMA locality
	information within an memcg since the pages are allowed to be
	allocated from any physical node. One of the use case is evaluating
	application performance by combining this information with the
	application's CPU allocation.

	All memory amounts are in bytes.

```

	  type N0=<bytes in node 0> N1=<bytes in node 1> ...

	The entries are ordered to be human readable, and new entries
	can show up in the middle. Don't rely on items remaining in a
	fixed position; use the keys to look up specific values!

	The entries can refer to the memory.stat.

  memory.swap.current
	A read-only single value file which exists on non-root
	cgroups.

	该 cgroup 及其后代当前正在使用的交换总量。

  memory.swap.high
	A read-write single value file which exists on non-root
	cgroups.  The default is "max".

	Swap usage throttle limit.  If a cgroup's swap usage exceeds
	this limit, all its further allocations will be throttled to
	allow userspace to implement custom out-of-memory procedures.

	This limit marks a point of no return for the cgroup. It is NOT
	designed to manage the amount of swapping a workload does
	during regular operation. Compare to memory.swap.max, which
	prohibits swapping past a set amount, but lets the cgroup
	continue unimpeded as long as other memory can be reclaimed.

	Healthy workloads are not expected to reach this limit.

  memory.swap.peak
	A read-write single value file which exists on non-root cgroups.

	The max swap usage recorded for the cgroup and its descendants since
	the creation of the cgroup or the most recent reset for that FD.

	A write of any non-empty string to this file resets it to the
	current memory usage for subsequent reads through the same
	file descriptor.

  memory.swap.max
	A read-write single value file which exists on non-root
	cgroups.  The default is "max".

	Swap usage hard limit.  If a cgroup's swap usage reaches this
	limit, anonymous memory of the cgroup will not be swapped out.

  memory.swap.events
	A read-only flat-keyed file which exists on non-root cgroups.
	The following entries are defined.  Unless specified
	otherwise, a value change in this file generates a file
	modified event.

	  high
		The number of times the cgroup's swap usage was over
		the high threshold.

	  max
		The number of times the cgroup's swap usage was about
		to go over the max boundary and swap allocation
		failed.

	  fail
		The number of times swap allocation failed either
		because of running out of swap system-wide or max
		limit.

	When reduced under the current usage, the existing swap
	entries are reclaimed gradually and the swap usage may stay
	higher than the limit for an extended period of time.  This
	reduces the impact on the workload and memory management.

  memory.zswap.current
	A read-only single value file which exists on non-root
	cgroups.

	The total amount of memory consumed by the zswap compression
	backend.

  memory.zswap.max
	A read-write single value file which exists on non-root
	cgroups.  The default is "max".

	Zswap usage hard limit. If a cgroup's zswap pool reaches this
	limit, it will refuse to take any more stores before existing
	entries fault back in or are written out to disk.

  memory.zswap.writeback
	A read-write single value file. The default value is "1".
	Note that this setting is hierarchical, i.e. the writeback would be
	implicitly disabled for child cgroups if the upper hierarchy
	does so.

	When this is set to 0, all swapping attempts to swapping devices
	are disabled. This included both zswap writebacks, and swapping due
	to zswap store failures. If the zswap store failures are recurring
	(for e.g if the pages are incompressible), users can observe
	reclaim inefficiency after disabling writeback (because the same
	pages might be rejected again and again).

	Note that this is subtly different from setting memory.swap.max to
	0, as it still allows for pages to be written to the zswap pool.
	This setting has no effect if zswap is disabled, and swapping
	is allowed unless memory.swap.max is set to 0.

  memory.pressure
	A read-only nested-keyed file.

	Shows pressure stall information for memory. See
	:ref:`Documentation/accounting/psi.rst <psi>` for details.


```
#### 使用准则


“memory.high” 是控制内存使用的主要机制。对 high 限额进行过度承诺（high 限额之和 > 可用内存），并让全局内存压力根据用量来分配内存，是一种可行的策略。

由于突破 high 限额不会触发 OOM killer，而只是节流违规的 cgroup，管理代理有充分的机会进行监控并采取适当行动，例如授予更多内存或终止工作负载。

确定一个 cgroup 是否有足够内存并非易事，因为内存用量并不能表明该工作负载是否可以从更多内存中受益。例如，一个将网络接收到的数据写入文件的工作负载可以使用所有可用内存，但也可以在只有少量内存时同样高效地运行。需要一种内存压力度量——即工作负载因缺乏内存而受到多大影响——来确定工作负载是否需要更多内存；遗憾的是，内存压力监控机制尚未实现。


#### 回收保护


通过 “memory.low” 或 “memory.min” 配置的保护，相对地应用于回收目标（即任何内存 cgroup 限额、主动的 memory.reclaim，或显然位于根 cgroup 中的全局回收）。为 B 配置的保护值原样适用于回收
```

		root - ... - A - B - C
		              \    ` D
		               ` E

```
When the reclaim targets ancestors of A, the effective protection of B is
capped by the protection value configured for A (and any other intermediate
ancestors between A and the target).

To express indifference about relative sibling protection, it is suggested to
use memory_recursiveprot. Configuring all descendants of a parent with finite
protection to "max" works but it may unnecessarily skew memory.events:low
field.


#### 内存所有权


一个内存区域会被记账到实例化它的 cgroup，并一直保持记账到该区域被释放为止。将进程迁移到不同的 cgroup 并不会把它在前一个 cgroup 中实例化的内存用量移动到新的 cgroup。

一个内存区域可能被属于不同 cgroup 的进程使用。该区域会被记账到哪个 cgroup 是不确定的；不过，随着时间的推移，该内存区域很可能最终落在某个拥有足够内存配额、以避免高昂回收压力的 cgroup 中。

如果一个 cgroup 清除了大量预期会被其他 cgroup 反复访问的内存，使用 POSIX_FADV_DONTNEED 来 relinquish 属于相关文件的内存区域的所有权，以确保正确的内存所有权，可能是有意义的。


### IO


“io” 控制器调节 IO 资源的分配。该控制器同时实现了基于权重以及绝对带宽或 IOPS 限额的分配；不过，基于权重的分配仅在使用了 cfq-iosched 时可用，且两种方案对 blk-mq 设备都不可用。


#### IO 接口文件


  io.stat
	A read-only nested-keyed file.

	Lines are keyed by $MAJ:$MIN device numbers and not ordered.
	The following nested keys are defined.

	  ======	=====================
	  rbytes	读取的字节数
	  wbytes	写入的字节数
	  rios		读 IO 次数
	  wios		写 IO 次数
	  dbytes	丢弃的字节数
	  dios		丢弃 IO 次数
	  ======	=====================

```

	  8:16 rbytes=1459200 wbytes=314773504 rios=192 wios=353 dbytes=0 dios=0
	  8:0 rbytes=90430464 wbytes=299008000 rios=8950 wios=1252 dbytes=50331648 dios=3021

  io.cost.qos
	A read-write nested-keyed file which exists only on the root
	cgroup.

	This file configures the Quality of Service of the IO cost
	model based controller (CONFIG_BLK_CGROUP_IOCOST) which
	currently implements "io.weight" proportional control.  Lines
	are keyed by $MAJ:$MIN device numbers and not ordered.  The
	line for a given device is populated on the first write for
	the device on "io.cost.qos" or "io.cost.model".  The following
	nested keys are defined.

	  ======	=====================================
	  enable	基于权重的控制开关
	  ctrl		"auto" 或 "user"
	  rpct		读延迟百分位    [0, 100]
	  rlat		读延迟阈值
	  wpct		写延迟百分位   [0, 100]
	  wlat		写延迟阈值
	  min		最小缩放百分比 [1, 10000]
	  max		最大缩放百分比 [1, 10000]
	  ======	=====================================

	The controller is disabled by default and can be enabled by
	setting "enable" to 1.  "rpct" and "wpct" parameters default
	to zero and the controller uses internal device saturation
	state to adjust the overall IO rate between "min" and "max".

	When a better control quality is needed, latency QoS
	parameters can be configured.  For example::

	  8:16 enable=1 ctrl=auto rpct=95.00 rlat=75000 wpct=95.00 wlat=150000 min=50.00 max=150.0

	shows that on sdb, the controller is enabled, will consider
	the device saturated if the 95th percentile of read completion
	latencies is above 75ms or write 150ms, and adjust the overall
	IO issue rate between 50% and 150% accordingly.

	The lower the saturation point, the better the latency QoS at
	the cost of aggregate bandwidth.  The narrower the allowed
	adjustment range between "min" and "max", the more conformant
	to the cost model the IO behavior.  Note that the IO issue
	base rate may be far off from 100% and setting "min" and "max"
	blindly can lead to a significant loss of device capacity or
	control quality.  "min" and "max" are useful for regulating
	devices which show wide temporary behavior changes - e.g. a
	ssd which accepts writes at the line speed for a while and
	then completely stalls for multiple seconds.

	When "ctrl" is "auto", the parameters are controlled by the
	kernel and may change automatically.  Setting "ctrl" to "user"
	or setting any of the percentile and latency parameters puts
	it into "user" mode and disables the automatic changes.  The
	automatic mode can be restored by setting "ctrl" to "auto".

  io.cost.model
	A read-write nested-keyed file which exists only on the root
	cgroup.

	This file configures the cost model of the IO cost model based
	controller (CONFIG_BLK_CGROUP_IOCOST) which currently
	implements "io.weight" proportional control.  Lines are keyed
	by $MAJ:$MIN device numbers and not ordered.  The line for a
	given device is populated on the first write for the device on
	"io.cost.qos" or "io.cost.model".  The following nested keys
	are defined.

	  =====		================================
	  ctrl		"auto" 或 "user"
	  model		正在使用的成本模型 - "linear"
	  =====		================================

	When "ctrl" is "auto", the kernel may change all parameters
	dynamically.  When "ctrl" is set to "user" or any other
	parameters are written to, "ctrl" become "user" and the
	automatic changes are disabled.

	When "model" is "linear", the following model parameters are
	defined.

	  =============	========================================
	  [r|w]bps	最大顺序 IO 吞吐
	  [r|w]seqiops	最大 4k 顺序 IO 每秒次数
	  [r|w]randiops	最大 4k 随机 IO 每秒次数
	  =============	========================================

	From the above, the builtin linear model determines the base
	costs of a sequential and random IO and the cost coefficient
	for the IO size.  While simple, this model can cover most
	common device classes acceptably.

	The IO cost model isn't expected to be accurate in absolute
	sense and is scaled to the device behavior dynamically.

	If needed, tools/cgroup/iocost_coef_gen.py can be used to
	generate device-specific coefficients.

  io.weight
	A read-write flat-keyed file which exists on non-root cgroups.
	The default is "default 100".

	The first line is the default weight applied to devices
	without specific override.  The rest are overrides keyed by
	$MAJ:$MIN device numbers and not ordered.  The weights are in
	the range [1, 10000] and specifies the relative amount IO time
	the cgroup can use in relation to its siblings.

	The default weight can be updated by writing either "default
	$WEIGHT" or simply "$WEIGHT".  Overrides can be set by writing
	"$MAJ:$MIN $WEIGHT" and unset by writing "$MAJ:$MIN default".

	An example read output follows::

	  default 100
	  8:16 200
	  8:0 50

  io.max
	A read-write nested-keyed file which exists on non-root
	cgroups.

	BPS and IOPS based IO limit.  Lines are keyed by $MAJ:$MIN
	device numbers and not ordered.  The following nested keys are
	defined.

	  =====		==================================
	  rbps		每秒最大读取字节数
	  wbps		每秒最大写入字节数
	  riops		每秒最大读 IO 操作数
	  wiops		每秒最大写 IO 操作数
	  =====		==================================

	When writing, any number of nested key-value pairs can be
	specified in any order.  "max" can be specified as the value
	to remove a specific limit.  If the same key is specified
	multiple times, the outcome is undefined.

	BPS and IOPS are measured in each IO direction and IOs are
	delayed if limit is reached.  Temporary bursts are allowed.

	Setting read limit at 2M BPS and write at 120 IOPS for 8:16::

	  echo "8:16 rbps=2097152 wiops=120" > io.max

	Reading returns the following::

	  8:16 rbps=2097152 wbps=max riops=max wiops=120

	Write IOPS limit can be removed by writing the following::

	  echo "8:16 wiops=max" > io.max

	Reading now returns the following::

	  8:16 rbps=2097152 wbps=max riops=max wiops=max

  io.pressure
	A read-only nested-keyed file.

	Shows pressure stall information for IO. See
	:ref:`Documentation/accounting/psi.rst <psi>` for details.


```
#### 回写


页缓存通过缓冲写（buffered writes）与共享 mmap 被弄脏（dirtied），并由回写（writeback）机制异步写入后备文件系统。回写位于内存域与 IO 域之间，通过平衡弄脏与写 IO 来调节脏内存的比例。

io 控制器与内存控制器协同，实现对页缓存回写 IO 的控制。内存控制器定义了计算并维护脏内存比例的内存域，io 控制器定义了为该内存域写出脏页的 io 域。系统级与每 cgroup 的脏内存状态都会被检查，二者中更严格的那个会被强制执行。

cgroup 回写需要底层文件系统的显式支持。目前，cgroup 回写实现于 ext2、ext4、btrfs、f2fs 和 xfs 上。在其它文件系统上，所有回写 IO 都被归算到根 cgroup。

内存与回写管理之间存在固有差异，这影响了 cgroup 所有权的跟踪方式。内存是每页跟踪的，而回写是每 inode 跟踪的。出于回写的目的，一个 inode 被分配给一个 cgroup，所有从该 inode 写出脏页的 IO 请求都被归算到该 cgroup。

由于内存的 cgroup 所有权是每页跟踪的，可能存在一些页面与 inode 所关联的 cgroup 不同。这些被称为外来页（foreign pages）。回写持续跟踪外来页，如果某个特定的外来 cgroup 在一段时间内成为多数，就将 inode 的所有权切换到该 cgroup。

虽然对于大多数使用场景而言，此模型已经足够——即使主写入 cgroup 随时间变化，给定一个 inode 大部分时候由单一 cgroup 弄脏——但多个 cgroup 同时写入单个 inode 的使用场景支持得并不好。在这种情况下，相当大一部分 IO 很可能被错误归算。由于内存控制器在首次使用时分配页所有权，并且在页被释放之前不会更新，即使回写严格遵循页所有权，多个 cgroup 弄脏重叠区域也无法如预期那样工作。建议避免此类使用模式。

影响回写行为的 sysctl 旋钮按如下方式应用于 cgroup 回写。

  vm.dirty_background_ratio, vm.dirty_ratio
	These ratios apply the same to cgroup writeback with the
	amount of available memory capped by limits imposed by the
	memory controller and system-wide clean memory.

  vm.dirty_background_bytes, vm.dirty_bytes
	For cgroup writeback, this is calculated into ratio against
	total available memory and applied the same way as
	vm.dirty[_background]_ratio.


#### IO 延迟


这是一个用于 IO 工作负载保护的 cgroup v2 控制器。你为某个组提供一个延迟目标，如果平均延迟超过了该目标，控制器就会对所有具有比受保护工作负载更低延迟目标的同辈（peer）进行节流。

限制只应用于层次中的同辈层级。这意味着在下面的图中，只有组 A、B 和 C 会相互影响，而
```

			[root]
		/	   |		\
		A	   B		C
	       /  \        |
	      D    F	   G


```
So the ideal way to configure this is to set io.latency in groups A, B, and C.
Generally you do not want to set a value lower than the latency your device
supports.  Experiment to find the value that works best for your workload.
Start at higher than the expected latency for your device and watch the
avg_lat value in io.stat for your workload group to get an idea of the
latency you see during normal operation.  Use the avg_lat value as a basis for
your real setting, setting at 10-15% higher than the value in io.stat.

#### IO 延迟节流如何工作


io.latency 是工作保持（work conserving）的；因此只要每个组都满足其延迟目标，控制器就不做任何事。一旦某个组开始未达到其目标，它就开始对任何具有比自身更高目标的同辈组进行节流。这种节流有两种形式：

- 队列深度（Queue depth）节流。这是一个组允许拥有的未完成 IO 数量。我们会相对快速地收紧它，从无限制开始，一直降到每次只允许 1 个 IO。

- 人为延迟诱导（Artificial delay induction）。某些类型的 IO 无法在不对更高优先级组产生不利影响的情况下被节流。这包括交换（swapping）与元数据 IO。这些类型的 IO 允许正常发生，但它们会被“记账”到发起组。如果发起组正在被节流，你会看到 io.stat 中的 use_delay 和 delay 字段增加。delay 值是被加到在该组中运行的任何进程上的微秒数。由于如果发生了大量交换或元数据 IO，这个数字可能会变得相当大，我们将单个延迟事件限制为每次最多 1 秒。

一旦受害组再次开始满足其延迟目标，它就会开始解除之前被节流的同辈组的节流。如果受害组干脆停止进行 IO，全局计数器会适当地解除节流。


#### IO 延迟接口文件


  io.latency
	This takes a similar format as the other controllers.

		"MAJOR:MINOR target=<target time in microseconds>"

  io.stat
	If the controller is enabled you will see extra stats in io.stat in
	addition to the normal ones.

	  depth
		该组当前的队列深度。

	  avg_lat
		这是一个衰减率为 1/exp、由采样间隔限定的指数移动平均。
		衰减率间隔可以通过将 io.stat 中的 win 值乘以基于 win 值的相应采样数来计算。

	  win
		采样窗口大小，以毫秒为单位。这是两次评估事件之间的最短持续时间。
		窗口只在有 IO 活动时才会流逝。空闲时段会延长最近的窗口。

#### IO 优先级


A single attribute controls the behavior of the I/O priority cgroup policy,
namely the io.prio.class attribute. The following values are accepted for
that attribute:

  no-change
	不修改 I/O 优先级类。

  promote-to-rt
	对于具有非 RT I/O 优先级类的请求，将其改为 RT。同时将这些请求的优先级级别改为 4。不修改具有 RT 优先级类的请求的 I/O 优先级。

  restrict-to-be
	对于没有 I/O 优先级类或具有 I/O 优先级类 RT 的请求，将其改为 BE。同时将这些请求的优先级级别改为 0。不修改具有 IDLE 优先级类的请求的 I/O 优先级类。

  idle
	将所有请求的 I/O 优先级类改为 IDLE（最低的 I/O 优先级类）。

  none-to-rt
	已弃用。只是 promote-to-rt 的别名。

The following numerical values are associated with the I/O priority policies:

+----------------+---+
| no-change      | 0 |
+----------------+---+
| promote-to-rt  | 1 |
+----------------+---+
| restrict-to-be | 2 |
+----------------+---+
| idle           | 3 |
+----------------+---+

The numerical value that corresponds to each I/O priority class is as follows:

+-------------------------------+---+
| IOPRIO_CLASS_NONE             | 0 |
+-------------------------------+---+
| IOPRIO_CLASS_RT (real-time)   | 1 |
+-------------------------------+---+
| IOPRIO_CLASS_BE (best effort) | 2 |
+-------------------------------+---+
| IOPRIO_CLASS_IDLE             | 3 |
+-------------------------------+---+

The algorithm to set the I/O priority class for a request is as follows:

- If I/O priority class policy is promote-to-rt, change the request I/O
  priority class to IOPRIO_CLASS_RT and change the request I/O priority
  level to 4.
- If I/O priority class policy is not promote-to-rt, translate the I/O priority
  class policy into a number, then change the request I/O priority class
  into the maximum of the I/O priority class policy number and the numerical
  I/O priority class.

### PID


进程数量控制器用于允许某个 cgroup 在达到指定限制后，阻止任何新任务被 fork() 或 clone()。

一个 cgroup 中的任务数量可能以其它控制器无法防止的方式被耗尽，因此需要有自己的控制器。例如，fork 炸弹（fork bomb）很可能先耗尽任务数量，然后才触及内存限制。

注意，此控制器中使用的 PID 指的是 TID，即内核所使用的进程 ID。


#### PID 接口文件


  pids.max
	A read-write single value file which exists on non-root
	cgroups.  The default is "max".

	进程数量的硬限制。

  pids.current
	A read-only single value file which exists on non-root cgroups.

	当前位于该 cgroup 及其后代中的进程数量。

  pids.peak
	A read-only single value file which exists on non-root cgroups.

	该 cgroup 及其后代中进程数量曾经达到过的最大值。

  pids.events
	A read-only flat-keyed file which exists on non-root cgroups. Unless
	specified otherwise, a value change in this file generates a file
	modified event. The following entries are defined.

	  max
		The number of times the cgroup's total number of processes hit the pids.max
		limit (see also pids_localevents).

  pids.events.local
	Similar to pids.events but the fields in the file are local
	to the cgroup i.e. not hierarchical. The file modified event
	generated on this file reflects only the local events.

Organisational operations are not blocked by cgroup policies, so it is
possible to have pids.current > pids.max.  This can be done by either
setting the limit to be smaller than pids.current, or attaching enough
processes to the cgroup such that pids.current is larger than
pids.max.  However, it is not possible to violate a cgroup PID policy
through fork() or clone(). These will return -EAGAIN if the creation
of a new process would cause a cgroup policy to be violated.


### Cpuset


“cpuset” 控制器提供了一种机制，用于将任务所放置的 CPU 和内存节点限制为任务当前 cgroup 中 cpuset 接口文件所指定的资源。这在大型 NUMA 系统上尤其有价值，因为将作业放置在经过合理调整大小的资源子集上、并辅以谨慎的处理器和内存放置，以减少跨节点内存访问与争用，可以提升整体系统性能。

“cpuset” 控制器是分层的。这意味着控制器不能使用其父节点不允许的 CPU 或内存节点。


#### Cpuset 接口文件


  cpuset.cpus
	A read-write multiple values file which exists on non-root
	cpuset-enabled cgroups.

	It lists the requested CPUs to be used by tasks within this
	cgroup.  The actual list of CPUs to be granted, however, is
	subjected to constraints imposed by its parent and can differ
	from the requested CPUs.

	The CPU numbers are comma-separated numbers or ranges.
```

	  # cat cpuset.cpus
	  0-4,6,8-10

	An empty value indicates that the cgroup is using the same
	setting as the nearest cgroup ancestor with a non-empty
	"cpuset.cpus" or all the available CPUs if none is found.

	The value of "cpuset.cpus" stays constant until the next update
	and won't be affected by any CPU hotplug events.

  cpuset.cpus.effective
	A read-only multiple values file which exists on all
	cpuset-enabled cgroups.

	It lists the onlined CPUs that are actually granted to this
	cgroup by its parent.  These CPUs are allowed to be used by
	tasks within the current cgroup.

	If "cpuset.cpus" is empty, the "cpuset.cpus.effective" file shows
	all the CPUs from the parent cgroup that can be available to
	be used by this cgroup.  Otherwise, it should be a subset of
	"cpuset.cpus" unless none of the CPUs listed in "cpuset.cpus"
	can be granted.  In this case, it will be treated just like an
	empty "cpuset.cpus".

	Its value will be affected by CPU hotplug events.

  cpuset.mems
	A read-write multiple values file which exists on non-root
	cpuset-enabled cgroups.

	It lists the requested memory nodes to be used by tasks within
	this cgroup.  The actual list of memory nodes granted, however,
	is subjected to constraints imposed by its parent and can differ
	from the requested memory nodes.

	The memory node numbers are comma-separated numbers or ranges.
	For example::

	  # cat cpuset.mems
	  0-1,3

	An empty value indicates that the cgroup is using the same
	setting as the nearest cgroup ancestor with a non-empty
	"cpuset.mems" or all the available memory nodes if none
	is found.

	The value of "cpuset.mems" stays constant until the next update
	and won't be affected by any memory nodes hotplug events.

	Setting a non-empty value to "cpuset.mems" causes memory of
	tasks within the cgroup to be migrated to the designated nodes if
	they are currently using memory outside of the designated nodes.

	There is a cost for this memory migration.  The migration
	may not be complete and some memory pages may be left behind.
	So it is recommended that "cpuset.mems" should be set properly
	before spawning new tasks into the cpuset.  Even if there is
	a need to change "cpuset.mems" with active tasks, it shouldn't
	be done frequently.

  cpuset.mems.effective
	A read-only multiple values file which exists on all
	cpuset-enabled cgroups.

	It lists the onlined memory nodes that are actually granted to
	this cgroup by its parent. These memory nodes are allowed to
	be used by tasks within the current cgroup.

	If "cpuset.mems" is empty, it shows all the memory nodes from the
	parent cgroup that will be available to be used by this cgroup.
	Otherwise, it should be a subset of "cpuset.mems" unless none of
	the memory nodes listed in "cpuset.mems" can be granted.  In this
	case, it will be treated just like an empty "cpuset.mems".

	Its value will be affected by memory nodes hotplug events.

  cpuset.cpus.exclusive
	A read-write multiple values file which exists on non-root
	cpuset-enabled cgroups.

	It lists all the exclusive CPUs that are allowed to be used
	to create a new cpuset partition.  Its value is not used
	unless the cgroup becomes a valid partition root.  See the
	"cpuset.cpus.partition" section below for a description of what
	a cpuset partition is.

	When the cgroup becomes a partition root, the actual exclusive
	CPUs that are allocated to that partition are listed in
	"cpuset.cpus.exclusive.effective" which may be different
	from "cpuset.cpus.exclusive".  If "cpuset.cpus.exclusive"
	has previously been set, "cpuset.cpus.exclusive.effective"
	is always a subset of it.

	Users can manually set it to a value that is different from
	"cpuset.cpus".	One constraint in setting it is that the list of
	CPUs must be exclusive with respect to "cpuset.cpus.exclusive"
	and "cpuset.cpus.exclusive.effective" of its siblings.	Another
	constraint is that it cannot be a superset of "cpuset.cpus"
	of its sibling in order to leave at least one CPU available to
	that sibling when the exclusive CPUs are taken away.

	For a parent cgroup, any one of its exclusive CPUs can only
	be distributed to at most one of its child cgroups.  Having an
	exclusive CPU appearing in two or more of its child cgroups is
	not allowed (the exclusivity rule).  A value that violates the
	exclusivity rule will be rejected with a write error.

	The root cgroup is a partition root and all its available CPUs
	are in its exclusive CPU set.

  cpuset.cpus.exclusive.effective
	A read-only multiple values file which exists on all non-root
	cpuset-enabled cgroups.

	This file shows the effective set of exclusive CPUs that
	can be used to create a partition root.  The content
	of this file will always be a subset of its parent's
	"cpuset.cpus.exclusive.effective" if its parent is not the root
	cgroup.  It will also be a subset of "cpuset.cpus.exclusive"
	if it is set.  This file should only be non-empty if either
	"cpuset.cpus.exclusive" is set or when the current cpuset is
	a valid partition root.

  cpuset.cpus.isolated
	A read-only and root cgroup only multiple values file.

	This file shows the set of all isolated CPUs used in existing
	isolated partitions. It will be empty if no isolated partition
	is created.

  cpuset.cpus.partition
	A read-write single value file which exists on non-root
	cpuset-enabled cgroups.  This flag is owned by the parent cgroup
	and is not delegatable.

	It accepts only the following input values when written to.

	  ==========	=====================================
	  "member"	分区的非根成员
	  "root"		分区根
	  "isolated"	无负载均衡的分区根
	  ==========	=====================================

	A cpuset partition is a collection of cpuset-enabled cgroups with
	a partition root at the top of the hierarchy and its descendants
	except those that are separate partition roots themselves and
	their descendants.  A partition has exclusive access to the
	set of exclusive CPUs allocated to it.	Other cgroups outside
	of that partition cannot use any CPUs in that set.

	There are two types of partitions - local and remote.  A local
	partition is one whose parent cgroup is also a valid partition
	root.  A remote partition is one whose parent cgroup is not a
	valid partition root itself.

	Writing to "cpuset.cpus.exclusive" is optional for the creation
	of a local partition as its "cpuset.cpus.exclusive" file will
	assume an implicit value that is the same as "cpuset.cpus" if it
	is not set.  Writing the proper "cpuset.cpus.exclusive" values
	down the cgroup hierarchy before the target partition root is
	mandatory for the creation of a remote partition.

	Not all the CPUs requested in "cpuset.cpus.exclusive" can be
	used to form a new partition.  Only those that were present
	in its parent's "cpuset.cpus.exclusive.effective" control
	file can be used.  For partitions created without setting
	"cpuset.cpus.exclusive", exclusive CPUs specified in sibling's
	"cpuset.cpus.exclusive" or "cpuset.cpus.exclusive.effective"
	also cannot be used.

	Currently, a remote partition cannot be created under a local
	partition.  All the ancestors of a remote partition root except
	the root cgroup cannot be a partition root.

	The root cgroup is always a partition root and its state cannot
	be changed.  All other non-root cgroups start out as "member".
	Even though the "cpuset.cpus.exclusive*" and "cpuset.cpus"
	control files are not present in the root cgroup, they are
	implicitly the same as the "/sys/devices/system/cpu/possible"
	sysfs file.

	When set to "root", the current cgroup is the root of a new
	partition or scheduling domain.  The set of exclusive CPUs is
	determined by the value of its "cpuset.cpus.exclusive.effective".

	When set to "isolated", the CPUs in that partition will be in
	an isolated state without any load balancing from the scheduler
	and excluded from the unbound workqueues.  Tasks placed in such
	a partition with multiple CPUs should be carefully distributed
	and bound to each of the individual CPUs for optimal performance.

	A partition root ("root" or "isolated") can be in one of the
	two possible states - valid or invalid.  An invalid partition
	root is in a degraded state where some state information may
	be retained, but behaves more like a "member".

	All possible state transitions among "member", "root" and
	"isolated" are allowed.

	On read, the "cpuset.cpus.partition" file can show the following
	values.

	  =============================	=====================================
	  "member"		分区的非根成员
	  "root"		分区根
	  "isolated"		无负载均衡的分区根
	  "root invalid (<reason>)"	无效的分区根
	  "isolated invalid (<reason>)"	无效的隔离分区根
	  =============================	=====================================

	In the case of an invalid partition root, a descriptive string on
	why the partition is invalid is included within parentheses.

	For a local partition root to be valid, the following conditions
	must be met.

	1) The parent cgroup is a valid partition root.
	2) The "cpuset.cpus.exclusive.effective" file cannot be empty,
	   though it may contain offline CPUs.
	3) The "cpuset.cpus.effective" cannot be empty unless there is
	   no task associated with this partition.

	For a remote partition root to be valid, all the above conditions
	except the first one must be met.

	External events like hotplug or changes to "cpuset.cpus" or
	"cpuset.cpus.exclusive" can cause a valid partition root to
	become invalid and vice versa.	Note that a task cannot be
	moved to a cgroup with empty "cpuset.cpus.effective".

	A valid non-root parent partition may distribute out all its CPUs
	to its child local partitions when there is no task associated
	with it.

	Care must be taken to change a valid partition root to "member"
	as all its child local partitions, if present, will become
	invalid causing disruption to tasks running in those child
	partitions. These inactivated partitions could be recovered if
	their parent is switched back to a partition root with a proper
	value in "cpuset.cpus" or "cpuset.cpus.exclusive".

	Poll and inotify events are triggered whenever the state of
	"cpuset.cpus.partition" changes.  That includes changes caused
	by write to "cpuset.cpus.partition", cpu hotplug or other
	changes that modify the validity status of the partition.
	This will allow user space agents to monitor unexpected changes
	to "cpuset.cpus.partition" without the need to do continuous
	polling.

	A user can pre-configure certain CPUs to an isolated state
	with load balancing disabled at boot time with the "isolcpus"
	kernel boot command line option.  If those CPUs are to be put
	into a partition, they have to be used in an isolated partition.


```
### 设备控制器


设备控制器管理对设备文件的访问。它既包括创建新的设备文件（使用 mknod），也包括对现有设备文件的访问。

Cgroup v2 设备控制器没有接口文件，构建于 cgroup BPF 之上。要控制对设备文件的访问，用户可以创建类型为 BPF_PROG_TYPE_CGROUP_DEVICE 的 bpf 程序，并使用 BPF_CGROUP_DEVICE 标志将它们挂接到 cgroup。当尝试访问设备文件时，相应的 BPF 程序会被执行，并根据返回值决定该尝试成功或以 -EPERM 失败。

一个 BPF_PROG_TYPE_CGROUP_DEVICE 程序接受一个指向 bpf_cgroup_dev_ctx 结构的指针，该结构描述了设备访问尝试：访问类型（mknod/read/write）与设备（类型、主设备号和次设备号）。如果程序返回 0，该尝试以 -EPERM 失败，否则成功。

BPF_PROG_TYPE_CGROUP_DEVICE 程序的一个示例可以在内核源码树的 tools/testing/selftests/bpf/progs/dev_cgroup.c 中找到。


### RDMA


“rdma” 控制器调节 RDMA 资源的分配与记账。


#### RDMA 接口文件


  rdma.max
	A readwrite nested-keyed file that exists for all the cgroups
	except root that describes current configured resource limit
	for a RDMA/IB device.

	Lines are keyed by device name and are not ordered.
	Each line contains space separated resource name and its configured
	limit that can be distributed.

	The following nested keys are defined.

	  ==========	=============================
	  hca_handle	HCA 句柄的最大数量
	  hca_object 	HCA 对象的最大数量
	  ==========	=============================

```

	  mlx4_0 hca_handle=2 hca_object=2000
	  ocrdma1 hca_handle=3 hca_object=max

  rdma.current
	A read-only file that describes current resource usage.
	It exists for all the cgroup except root.

	An example for mlx4 and ocrdma device follows::

	  mlx4_0 hca_handle=1 hca_object=20
	  ocrdma1 hca_handle=1 hca_object=23

```
### DMEM


“dmem” 控制器调节设备内存区域的分配与记账。由于每个内存区域可能拥有自己的页大小，且不必等于系统页大小，单位始终为字节。


#### DMEM 接口文件


  dmem.max, dmem.min, dmem.low
	A readwrite nested-keyed file that exists for all the cgroups
	except root that describes current configured resource limit
	for a region.

```

	  drm/0000:03:00.0/vram0 1073741824
	  drm/0000:03:00.0/stolen max

	The semantics are the same as for the memory cgroup controller, and are
	calculated in the same way.

  dmem.capacity
	A read-only file that describes maximum region capacity.
	It only exists on the root cgroup. Not all memory can be
	allocated by cgroups, as the kernel reserves some for
	internal use.

	An example for xe follows::

	  drm/0000:03:00.0/vram0 8514437120
	  drm/0000:03:00.0/stolen 67108864

  dmem.current
	A read-only file that describes current resource usage.
	It exists for all the cgroup except root.

	An example for xe follows::

	  drm/0000:03:00.0/vram0 12550144
	  drm/0000:03:00.0/stolen 8650752

```
### HugeTLB


HugeTLB 控制器允许限制每个控制组的 HugeTLB 用量，并在发生页错误时强制执行控制器限额。


#### HugeTLB 接口文件


  hugetlb.<hugepagesize>.current
	Show current usage for "hugepagesize" hugetlb.  It exists for all
	the cgroup except root.

  hugetlb.<hugepagesize>.max
	Set/show the hard limit of "hugepagesize" hugetlb usage.
	The default value is "max".  It exists for all the cgroup except root.

  hugetlb.<hugepagesize>.events
	A read-only flat-keyed file which exists on non-root cgroups.

	  max
		因 HugeTLB 限额而导致分配失败的次数。

  hugetlb.<hugepagesize>.events.local
	Similar to hugetlb.<hugepagesize>.events but the fields in the file
	are local to the cgroup i.e. not hierarchical. The file modified event
	generated on this file reflects only the local events.

  hugetlb.<hugepagesize>.numa_stat
	Similar to memory.numa_stat, it shows the numa information of the
        hugetlb pages of <hugepagesize> in this cgroup.  Only active in
        use hugetlb pages are included.  The per-node values are in bytes.

### Misc


Miscellaneous cgroup 为那些无法像其它 cgroup 资源那样被抽象的标量资源提供资源限制与跟踪机制。该控制器通过 CONFIG_CGROUP_MISC 配置选项启用。

可以通过 include/linux/misc_cgroup.h 文件中的 enum misc_res_type{} 向控制器添加资源，并通过 kernel/cgroup/misc.c 文件中的 misc_res_name[] 添加相应名称。资源的提供方必须在使用该资源之前调用 misc_cg_set_capacity() 设置其容量。

一旦设置了容量，就可以通过 charge 与 uncharge API 更新资源用量。所有与 misc 控制器交互的 API 都在 include/linux/misc_cgroup.h 中。


#### Misc 接口文件


Miscellaneous controller provides 3 interface files. If two misc resources (res_a and res_b) are registered then:

  misc.capacity
        A read-only flat-keyed file shown only in the root cgroup.  It shows
        miscellaneous scalar resources available on the platform along with
```

	  $ cat misc.capacity
	  res_a 50
	  res_b 10

  misc.current
        A read-only flat-keyed file shown in the all cgroups.  It shows
        the current usage of the resources in the cgroup and its children.::

	  $ cat misc.current
	  res_a 3
	  res_b 0

  misc.peak
        A read-only flat-keyed file shown in all cgroups.  It shows the
        historical maximum usage of the resources in the cgroup and its
        children.::

	  $ cat misc.peak
	  res_a 10
	  res_b 8

  misc.max
        A read-write flat-keyed file shown in the non root cgroups. Allowed
        maximum usage of the resources in the cgroup and its children.::

	  $ cat misc.max
	  res_a max
	  res_b 4

	Limit can be set by::

	  # echo res_a 1 > misc.max

	Limit can be set to max by::

	  # echo res_a max > misc.max

        Limits can be set higher than the capacity value in the misc.capacity
        file.

  misc.events
	A read-only flat-keyed file which exists on non-root cgroups. The
	following entries are defined. Unless specified otherwise, a value
	change in this file generates a file modified event. All fields in
	this file are hierarchical.

	  max
		该 cgroup 的资源用量即将超过 max 边界的次数。

  misc.events.local
        Similar to misc.events but the fields in the file are local to the
        cgroup i.e. not hierarchical. The file modified event generated on
        this file reflects only the local events.

```
#### 迁移与所有权


一个杂项标量资源会被记账到首次使用它的 cgroup，并一直保持记账到该资源被释放为止。将进程迁移到不同的 cgroup 并不会把记账转移到进程所移动到的目的 cgroup。


### 其他


#### perf_event


perf_event 控制器，如果未挂载到遗留层级，会自动在 v2 层级上启用，以便 perf 事件始终能按 cgroup v2 路径进行过滤。在 v2 层级被填充之后，该控制器仍可被移动到遗留层级。


### 非规范性信息


本节包含不被视为稳定内核 API 一部分、因而可能发生变更的信息。


#### CPU 控制器根 cgroup 进程行为


在根 cgroup 中分配 CPU 周期时，该 cgroup 中的每个线程都被当作是由根 cgroup 的一个独立子 cgroup 承载的。这个子 cgroup 的权重取决于其线程的 nice 级别。

关于这种映射的细节，请参阅 kernel/sched/core.c 文件中的 sched_prio_to_weight 数组（该数组中的值应适当缩放，使得中性的——nice 0——值为 100 而非 1024）。


#### IO 控制器根 cgroup 进程行为


根 cgroup 中的进程承载于一个隐式的叶子子节点中。在分配 IO 资源时，会把这个隐式子节点当作是根 cgroup 的一个普通子 cgroup 来考虑，其权重值为 200。


## 命名空间


### 基础


cgroup 命名空间提供了一种机制，用于虚拟化 “/proc/$PID/cgroup” 文件与 cgroup 挂载的视图。CLONE_NEWCGROUP clone 标志可以与 clone(2) 和 unshare(2) 一起使用，以创建一个新的 cgroup 命名空间。运行在 cgroup 命名空间内部的进程，其 “/proc/$PID/cgroup” 输出会被限制为 cgroupns 根。cgroupns 根是创建 cgroup 命名空间时进程的 cgroup。

在没有 cgroup 命名空间的情况下，“/proc/$PID/cgroup” 文件显示进程 cgroup 的完整路径。在容器设置中，一组 cgroup 和命名空间旨在隔离进程，“/proc/$PID/cgroup” 文件可能会泄漏潜在的系统级信息
```

  # cat /proc/self/cgroup
  0::/batchjobs/container_id1

```
路径 ‘/batchjobs/container_id1’ 可被视为系统数据，不希望暴露给被隔离的进程。cgroup 命名空间可用于限制此路径的可见性。例如，在之前
```

  # ls -l /proc/self/ns/cgroup
  lrwxrwxrwx 1 root root 0 2014-07-15 10:37 /proc/self/ns/cgroup -> cgroup:[4026531835]
  # cat /proc/self/cgroup
  0::/batchjobs/container_id1

```
```

  # ls -l /proc/self/ns/cgroup
  lrwxrwxrwx 1 root root 0 2014-07-15 10:35 /proc/self/ns/cgroup -> cgroup:[4026532183]
  # cat /proc/self/cgroup
  0::/

```
When some thread from a multi-threaded process unshares its cgroup
namespace, the new cgroupns gets applied to the entire process (all
the threads).  This is natural for the v2 hierarchy; however, for the
legacy hierarchies, this may be unexpected.

A cgroup namespace is alive as long as there are processes inside or
mounts pinning it.  When the last usage goes away, the cgroup
namespace is destroyed.  The cgroupns root and the actual cgroups
remain.


### 根与视图


cgroup 命名空间的 ‘cgroupns root’ 是调用 unshare(2) 的进程所运行的 cgroup。例如，如果位于 /batchjobs/container_id1 cgroup 中的一个进程调用 unshare，cgroup /batchjobs/container_id1 就成为 cgroupns 根。对于 init_cgroup_ns，这就是真正的根（‘/’）cgroup。

即使命名空间创建者
```

  #~/unshare -c # unshare cgroupns in some cgroup
  # cat /proc/self/cgroup
  0::/
  # mkdir sub_cgrp_1
  # echo 0 > sub_cgrp_1/cgroup.procs
  # cat /proc/self/cgroup
  0::/sub_cgrp_1

```
Each process gets its namespace-specific view of "/proc/$PID/cgroup"

Processes running inside the cgroup namespace will be able to see
cgroup paths (in /proc/self/cgroup) only inside their root cgroup.
```

  # sleep 100000 &
  [1] 7353
  # echo 7353 > sub_cgrp_1/cgroup.procs
  # cat /proc/7353/cgroup
  0::/sub_cgrp_1

```
From the initial cgroup namespace, the real cgroup path will be
```

  $ cat /proc/7353/cgroup
  0::/batchjobs/container_id1/sub_cgrp_1

```
From a sibling cgroup namespace (that is, a namespace rooted at a
different cgroup), the cgroup path relative to its own cgroup
namespace root will be shown.  For instance, if PID 7353's cgroup
```

  # cat /proc/7353/cgroup
  0::/../container_id2/sub_cgrp_1

```
Note that the relative path always starts with '/' to indicate that
its relative to the cgroup namespace root of the caller.


### 迁移与 setns(2)


Processes inside a cgroup namespace can move into and out of the
namespace root if they have proper access to external cgroups.  For
example, from inside a namespace with cgroupns root at
/batchjobs/container_id1, and assuming that the global hierarchy is
```

  # cat /proc/7353/cgroup
  0::/sub_cgrp_1
  # echo 7353 > batchjobs/container_id2/cgroup.procs
  # cat /proc/7353/cgroup
  0::/../container_id2

```
Note that this kind of setup is not encouraged.  A task inside cgroup
namespace should only be exposed to its own cgroupns hierarchy.

setns(2) to another cgroup namespace is allowed when:

(a) the process has CAP_SYS_ADMIN against its current user namespace
(b) the process has CAP_SYS_ADMIN against the target cgroup
    namespace's userns

No implicit cgroup changes happen with attaching to another cgroup
namespace.  It is expected that the someone moves the attaching
process under the target cgroup namespace root.


### 与其他命名空间的交互


Namespace specific cgroup hierarchy can be mounted by a process
```

  # mount -t cgroup2 none $MOUNT_POINT

```
This will mount the unified cgroup hierarchy with cgroupns root as the
filesystem root.  The process needs CAP_SYS_ADMIN against its user and
mount namespaces.

The virtualization of /proc/self/cgroup file combined with restricting
the view of cgroup hierarchy by namespace-private cgroupfs mount
provides a properly isolated cgroup view inside the container.


## 内核编程相关信息


本节包含在与 cgroup 交互不可避免之处的内核编程信息。cgroup 核心与控制器不在覆盖范围内。


### 回写相关的文件系统支持


一个文件系统可以通过更新 address_space_operations->writepages() 来注释 bio，从而支持 cgroup 回写，使用以下两个函数。

  wbc_init_bio(@wbc, @bio)
	Should be called for each bio carrying writeback data and
	associates the bio with the inode's owner cgroup and the
	corresponding request queue.  This must be called after
	a queue (device) has been associated with the bio and
	before submission.

  wbc_account_cgroup_owner(@wbc, @folio, @bytes)
	Should be called for each data segment being written out.
	While this function doesn't care exactly when it's called
	during the writeback session, it's the easiest and most
	natural to call it as data segments are added to a bio.

With writeback bio's annotated, cgroup support can be enabled per
super_block by setting SB_I_CGROUPWB in ->s_iflags.  This allows for
selective disabling of cgroup writeback support which is helpful when
certain filesystem features, e.g. journaled data mode, are
incompatible.

wbc_init_bio() binds the specified bio to its cgroup.  Depending on
the configuration, the bio may be executed at a lower priority and if
the writeback session is holding shared resources, e.g. a journal
entry, may lead to priority inversion.  There is no one easy solution
for the problem.  Filesystems can try to work around specific problem
cases by skipping wbc_init_bio() and using bio_associate_blkg()
directly.


## 已弃用的 v1 核心特性


- 不支持包括命名层级在内的多重层级。

- 不支持所有 v1 挂载选项。

- 移除了 “tasks” 文件，“cgroup.procs” 也不排序。

- 移除了 “cgroup.clone_children”。

- 对于 v2，/proc/cgroups 没有意义。请改用根节点上的 “cgroup.controllers” 或 “cgroup.stat” 文件。


## v1 存在的问题及 v2 的设计理由


### 多重层级


cgroup v1 允许任意数量的层级，且每个层级可以承载任意数量的控制器。虽然这看似提供了高度的灵活性，但在实践中并无用处。

例如，由于每个控制器只有一个实例，像 freezer 这类在所有层级都可能有用的实用型控制器只能用于其中一个。由于控制器一旦层级被填充就无法移动到另一个层级，这个问题更加严重。另一个问题是，绑定到某个层级的所有控制器被迫拥有完全相同的层次视图。不可能根据特定控制器来改变粒度。

在实践中，这些问题严重限制了哪些控制器能被放在同一个层级上，大多数配置最终都选择将每个控制器放在自己的层级上。只有紧密相关的那些，例如 cpu 和 cpuacct 控制器，放在同一层级才有意义。这通常意味着每当需要进行层级管理操作时，用户空间不得不在多个相似的层级上重复相同的步骤。

此外，对多重层级的支持代价高昂。它极大地复杂化了 cgroup 核心实现，但更重要的是，对多重层级的支持限制了 cgroup 的一般用法以及控制器能够做的事情。

层级数量没有上限，这意味着一个线程的 cgroup 归属无法用有限长度来描述。键可能包含任意数量的条目，且长度不限，这使其操作起来非常笨拙，并导致了专门为了标识归属而存在的控制器增加，而这又反过来加剧了层级数量激增的原始问题。

另外，由于一个控制器无法对其它控制器可能所在的层级拓扑有任何预期，每个控制器都不得不假设所有其它控制器都挂接在完全正交的层级上。这使得控制器之间无法协作，或者至少非常笨拙。

在大多数使用场景中，将控制器放在彼此完全正交的层级上并非必要。通常所需要的是根据特定控制器拥有不同粒度级别的能力。换言之，从叶子向根方向看，层级可能被折叠。例如，某个配置可能不关心超过某一层之后内存如何分配，但仍想控制 CPU 周期如何分配。


### 线程粒度


cgroup v1 允许一个进程的线程属于不同的 cgroup。这对某些控制器而言没有意义，这些控制器最终实现了不同的方式来忽略这种情况；但更重要的是，它模糊了暴露给单个应用程序的 API 与系统管理接口之间的界限。

一般而言，进程内部知识只有进程自身可用；因此，与进程的服务级组织不同，对进程线程进行分类需要拥有目标进程的、该应用程序的积极参与。

cgroup v1 有一个定义模糊的委托模型，它与线程粒度结合被滥用。cgroup 被委托给单个应用程序，以便它们能够创建并管理自己的子层次，并控制沿这些子层次的资源分配。这实际上将 cgroup 提升到了类似于系统调用的 API 的地位，暴露给普通程序。

首先，cgroup 作为一个接口，其本质不足以以此种方式暴露。一个进程要访问自己的旋钮，必须从 /proc/self/cgroup 中提取目标层级上的路径，将旋钮名附加到路径后构造出完整路径，打开然后读取和/或写入。这不仅极其笨拙且不常见，而且本质上是存在竞态的。没有常规方法能定义跨所需步骤的事务，也没有任何东西能保证该进程实际上是在操作自己的子层次。

cgroup 控制器实现了大量永远不会被接受为公共 API 的旋钮，因为它们只是向系统管理伪文件系统添加控制旋钮。cgroup 最终有了未被正确抽象或精炼、直接暴露内核内部细节的接口旋钮。这些旋钮通过定义不清的委托机制暴露给单个应用程序，实际上将 cgroup 滥用为实现公共 API 的捷径，而绕过了所需的严格审查。

这对用户空间和内核都是痛苦的。用户空间最终得到了行为异常且抽象不良的接口，而内核暴露并锁定了无意中的构造。


### 内部节点与线程之间的竞争


cgroup v1 允许线程位于任何 cgroup 中，这造成了一个有趣的问题：属于父 cgroup 及其子 cgroup 的线程相互竞争资源。这很糟糕，因为两种不同类型的实体在竞争，而且没有明显的方法来解决它。不同的控制器做法不同。

cpu 控制器将线程和 cgroup 视为等价，并把 nice 级别映射到 cgroup 权重。这在某些情况下行得通，但当子节点想要被分配特定比例的 CPU 周期、而内部线程数量波动时就会失效——比例随着竞争实体数量的波动而不断变化。还有其它问题。从 nice 级别到权重的映射既不明显也不通用，而且有各种其它旋钮对线程根本不可用。

io 控制器为每个 cgroup 隐式创建了一个隐藏的叶子节点来承载线程。这个隐藏叶子拥有自己所有以 `leaf_` 为前缀的旋钮副本。虽然这允许对内部线程进行等价的控制，但带有严重的缺陷。它总是增加了一层原本不必要的嵌套，使接口变得混乱，并显著复杂化了实现。

内存控制器没有办法控制内部任务与子 cgroup 之间发生的情况，其行为也没有被清晰定义。曾有人尝试添加临时行为与旋钮来针对特定工作负载裁剪行为，但这会导致长期极难解决的问题。

多个控制器都在与内部任务作斗争，并想出了不同的应对方法；不幸的是，所有这些方法都有严重缺陷，而且，差异巨大的行为使 cgroup 作为一个整体高度不一致。

这显然是一个需要从 cgroup 核心以统一方式解决的问题。


### 其他接口问题


cgroup v1 在没有监督的情况下发展，产生了大量特殊行为与不一致。cgroup 核心侧的一个问题是空 cgroup 如何被通知——为每个事件 fork 并执行一个用户空间辅助二进制。事件投递既非递归也不可委托。该机制的局限性还导致了内核内的事件投递过滤机制，进一步复杂化了接口。

控制器接口也有问题。一个极端的例子是控制器完全忽略层次组织，把所有 cgroup 都当作直接位于根 cgroup 之下。一些控制器向用户空间暴露了大量不一致的实现细节。

跨控制器之间也缺乏一致性。当创建一个新的 cgroup 时，一些控制器默认不施加额外限制，而另一些则在被显式配置之前禁止任何资源使用。同一类型控制的配置旋钮使用了差异巨大的命名方案与格式。统计与信息旋钮命名随意，即使在同一控制器内也使用了不同格式与单位。

cgroup v2 在适当之处建立了通用约定，并更新控制器，使它们暴露最小且一致的接口。


### 控制器问题及对策


#### 内存


原始的较低边界——软限制（soft limit）——被定义为一个默认未设置的限额。结果，全局回收优先选择的 cgroup 集合是选择加入（opt-in）的，而非选择退出（opt-out）。优化这些大多为负面的查找的代价如此之高，以至于该实现尽管规模庞大，却连基本的理想行为都无法提供。首先，软限制没有层次含义。所有已配置的组被组织在一个全局 rbtree 中，并被当作平等的同伴对待，无论它们位于层次中的何处。这使得子树委托变得不可能。其次，软限制回收过程过于激进，不仅给系统引入了高分配延迟，还因过度回收影响了系统性能，以至于该特性变得适得其反。

另一方面，memory.low 边界是一个自上而下分配的储备。当一个 cgroup 处于其 effective low 之内时，它享有回收保护，这使得子树委托成为可能。当它高于其 effective low 时，它还享有与其超出量成比例的回收压力。

原始的较高边界——硬限制（hard limit）——被定义为一个严格的限制，即使必须调用 OOM killer 也不得退让。但这总体上违背了充分利用可用内存的目标。工作负载的内存消耗在运行期间是变化的，这要求用户进行过度承诺。但使用严格的硬上限进行过度承诺，要么需要相当准确地预测工作集大小，要么需要在限额上留有余量。由于工作集大小估算既困难又容易出错，而估算错误会导致 OOM kill，大多数用户倾向于选择较为宽松的限额，最终浪费了宝贵资源。

另一方面，memory.high 边界可以设置得保守得多。当被触及，它通过强制分配进入直接回收以消解超额部分，但从不调用 OOM killer。因此，一个设置得过于激进的 high 边界不会终止进程，而是导致性能逐渐下降。用户可以监控这一点并做出修正，直到找到仍能提供可接受性能的最小内存占用。

在极端情况下，存在大量并发分配且组内回收进展完全停滞时，high 边界可能被突破。但即便如此，满足分配需求也大多优于从其它组或系统其余部分可用的余量中满足，而非杀死该组。否则，memory.max 就在那里限制这类溢出，并最终 containment 有 bug 甚至恶意的应用程序。

将原始的 memory.limit_in_bytes 设置到低于当前用量会遭遇竞态条件，并发的记账可能导致限额设置失败。而 memory.max 则会先设置限额以阻止新的记账，然后回收并 OOM kill，直到达到新限额——或者写入 memory.max 的任务被杀死。

合并的内存+交换记账与限制，被替换为对交换空间的真正控制。

原始 cgroup 设计中使用合并内存+交换机制的主要论点是：全局或父级压力总能交换出子组的全部匿名内存，无论子组自身的（可能是不可信的）配置如何。然而，不可信的组可以通过其它方式破坏交换——例如在一个紧凑循环中引用其匿名内存——而管理员在过度承诺不可信作业时，不能假定完全的交换可行性。

另一方面，对于可信作业，合并计数器并不是一个直观的用户空间接口，并且它违背了 cgroup 控制器应当记账并限制特定物理资源的理念。交换空间与系统中的其他资源一样，正因如此，统一层级允许单独分配它。
