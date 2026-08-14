## 控制组（Control Groups）


由 Paul Menage <menage@google.com> 基于
Documentation/admin-guide/cgroup-v1/cpusets.rst 撰写

cpusets.txt 的原始版权声明：

Portions Copyright (C) 2004 BULL SA.

Portions Copyright (c) 2004-2006 Silicon Graphics, Inc.

Modified by Paul Jackson <pj@sgi.com>

Modified by Christoph Lameter <cl@gentwo.org>


 1. 控制组（Control Groups）
	1.1 什么是 cgroups？
	1.2 为何需要 cgroups？
	1.3 cgroups 是如何实现的？
	1.4 notify_on_release 的作用是什么？
	1.5 clone_children 的作用是什么？
	1.6 如何使用 cgroups？
 2. 使用示例与语法
	2.1 基本用法
	2.2 附加进程
	2.3 按名称挂载层级
 3. 内核 API
	3.1 概述
	3.2 同步
	3.3 子系统 API
 4. 扩展属性的使用
 5. 问题

## 1. 控制组（Control Groups）


### 1.1 什么是 cgroups？


控制组（Control Groups）提供了一种机制，用于将一组任务及其所有未来子任务聚合/划分到具有特定行为的层级化组中。

定义：

**cgroup** 将一组任务与一个或多个子系统的参数集合关联起来。

**subsystem（子系统）** 是一个模块，它利用 cgroups 提供的任务分组机制，以特定方式处理任务组。子系统通常是一个"资源控制器"，它调度某种资源或施加每 cgroup 限制，但它也可以是任何希望对一组进程施加作用的东西，例如虚拟化子系统。

**hierarchy（层级）** 是一组以树状排列的 cgroup，使得系统中的每个任务都恰好位于该层级中某一个 cgroup 内，同时还有一组子系统；每个子系统都有附着于该层级中每个 cgroup 的、系统特定的状态。每个层级都关联着一个 cgroup 虚拟文件系统的实例。

任何时刻都可能存在多个处于活动状态的任务 cgroup 层级。每个层级都是对系统中所有任务的一种划分。

用户态代码可以在 cgroup 虚拟文件系统的一个实例中按名称创建和销毁 cgroup、指定并查询某任务被分配到哪个 cgroup，以及列出分配给某 cgroup 的任务 PID。这些创建与分配只会影响与该 cgroup 文件系统实例相关联的层级。

就自身而言，cgroups 的唯一用途是进行简单的作业跟踪。其意图在于让其他子系统挂接到通用的 cgroup 支持上，为 cgroups 提供新属性，例如对 cgroup 中进程可访问的资源进行统计/限制。例如，cpusets（见 Documentation/admin-guide/cgroup-v1/cpusets.rst）允许你将一组 CPU 和一组内存节点与每个 cgroup 中的任务关联起来。


### 1.2 为何需要 cgroups？


Linux 内核中有多种为进程聚合（主要出于资源跟踪目的）所做的努力。这些努力包括 cpusets、CKRM/ResGroups、UserBeanCounters 以及虚拟服务器命名空间。它们都需要"对进程进行分组/划分"这一基本概念，使得新 fork 出的进程最终与其父进程处于同一组（cgroup）中。

内核 cgroup 补丁提供了高效实现此类分组所需的最小必要内核机制。它对系统快速路径的影响极小，并为诸如 cpusets 之类的特定子系统提供钩子，以便按需提供额外行为。

提供多层级支持，是为了应对不同子系统对任务划分到 cgroup 的方式截然不同的情况——拥有并行层级使得每个层级都可以成为任务的一种自然划分，而不必处理当几个互不相关的子系统被迫塞进同一棵 cgroup 树时会出现的复杂任务组合。

在一个极端，每个资源控制器或子系统可以处于独立的层级中；在另一个极端，所有子系统都附加到同一个层级。

作为可以从多层级受益的场景示例（最初由 vatsa@in.ibm.com 提出），考虑一台拥有各类用户（学生、教授、系统任务等）的大型大学服务器。该服务器的资源规划可以沿以下方向进行
```

       CPU :          "Top cpuset"
                       /       \
               CPUSet1         CPUSet2
                  |               |
               (Professors)    (Students)

               In addition (system tasks) are attached to topcpuset (so
               that they can run anywhere) with a limit of 20%

       Memory : Professors (50%), Students (30%), system (20%)

       Disk : Professors (50%), Students (30%), system (20%)

       Network : WWW browsing (20%), Network File System (60%), others (20%)
                               / \
               Professors (15%)  students (5%)

```
像 Firefox/Lynx 这样的浏览器归入 WWW 网络类，而 (k)nfsd 归入 NFS 网络类。

同时，Firefox/Lynx 会根据启动者（教授/学生）共享相应的 CPU/内存类。

由于能够针对不同资源对任务进行不同分类（通过将这些资源子系统放入不同层级），管理员可以轻松设置一个接收 exec 通知的脚本
```

    # echo browser_pid > /sys/fs/cgroup/<restype>/<userclass>/tasks

```
如果只有单一层级，他现在可能需要为启动的每个浏览器创建一个独立的 cgroup，并将其与相应的网络及其他资源类关联。这可能导致此类 cgroup 大量增殖。

再假设管理员想临时给予某学生的浏览器更高的网络访问权限（因为已是深夜，该用户想进行在线游戏 :))，或者给予该学生的某个仿真程序更高的 CPU 算力。

借助直接将 PID 写入资源类的能力，只需
```

       # echo pid > /sys/fs/cgroup/network/<new_class>/tasks
       (after some time)
       # echo pid > /sys/fs/cgroup/network/<orig_class>/tasks

```
没有这种能力，管理员就不得不把这个 cgroup 拆分成多个独立的 cgroup，然后将新的 cgroup 与新的资源类关联起来。



### 1.3 cgroups 是如何实现的？


控制组对内核的扩展如下：

 - 系统中的每个任务都有一个指向 css_set 的引用计数指针。

 - 一个 css_set 包含一组指向 cgroup_subsys_state 对象的引用计数指针，系统中注册的每个 cgroup 子系统对应一个。任务与其所属的每个层级中的 cgroup 之间没有直接的链接，但可以通过 cgroup_subsys_state 对象中的指针来确定。这是因为访问子系统状态是预期会频繁发生在性能关键代码中的操作，而需要任务实际 cgroup 分配（尤其是 cgroup 间迁移）的操作则较少见。一条链表通过 css_set 贯穿每个 task_struct 的 cg_list 字段，锚定于 css_set->tasks。

 - 可以挂载一个 cgroup 层级文件系统，以便从用户空间进行浏览和操作。

 - 你可以列出附加到任何 cgroup 的所有任务（按 PID）。

cgroups 的实现需要在内核其余部分加入少量简单钩子，且都不在性能关键路径上：

 - 在 init/main.c 中，用于在系统启动时初始化根 cgroups 和初始 css_set。

 - 在 fork 和 exit 中，用于将任务附加到其 css_set 或从其中分离。

此外，可以挂载一个 "cgroup" 类型的新文件系统，以便浏览和修改内核当前已知的 cgroups。挂载 cgroup 层级时，你可以指定一个逗号分隔的子系统列表作为文件系统挂载选项。默认情况下，挂载 cgroup 文件系统会尝试挂载一个包含所有已注册子系统的层级。

如果已经存在一个由完全相同子系统集合构成的活动层级，它将被新挂载复用。如果没有匹配的现有层级，且所请求的任一子系统已在某个现有层级中使用，则挂载将以 -EBUSY 失败。否则，将激活一个新的层级，并与所请求的子系统关联。

目前无法将新子系统绑定到一个活动的 cgroup 层级，也无法将子系统从活动的 cgroup 层级解绑。未来或许可以实现，但这充满了棘手的错误处理恢复问题。

当某个 cgroup 文件系统被卸载时，如果在顶层 cgroup 之下创建了任何子 cgroup，该层级即使被卸载仍会保持活动；如果没有子 cgroup，则该层级将被停用。

没有为 cgroups 新增任何系统调用——所有用于查询和修改 cgroups 的支持都通过这个 cgroup 文件系统实现。

/proc 下每个任务都多了一个名为 'cgroup' 的文件，它针对每个活动层级显示子系统名称以及 cgroup 名称（作为相对于 cgroup 文件系统根的路径）。

每个 cgroup 由 cgroup 文件系统中的目录表示，其中包含如下描述该 cgroup 的文件：

 - tasks：附加到该 cgroup 的任务列表（按 PID）。此列表不保证有序。向该文件写入一个线程 ID 会将该线程移入此 cgroup。
 - cgroup.procs：该 cgroup 中的线程组 ID 列表。此列表不保证有序，也不保证不含重复的 TGID，如果确实需要，用户态应对其排序/去重。向该文件写入线程组 ID 会将该组中所有线程移入此 cgroup。
 - notify_on_release 标志：退出时是否运行 release agent？
 - release_agent：用于释放通知的路径（该文件仅存在于顶层 cgroup）。

其他子系统（如 cpusets）可能会在每个 cgroup 目录中添加额外的文件。

新 cgroups 通过 mkdir 系统调用或 shell 命令创建。cgroup 的属性（例如其标志）通过写入该 cgroup 目录中的相应文件来修改，如上所列。

嵌套 cgroups 的具名层级结构允许将大型系统划分为嵌套的、可动态变更的"软分区"。

每个任务对 cgroup 的附加（在 fork 时由其任意子任务自动继承）使得可以将系统上的工作负载组织成相关的任务集合。如果相应 cgroup 文件系统目录的权限允许，任务可以被重新附加到任何其他 cgroup。

当任务从一个 cgroup 移到另一个时，它会获得一个新的 css_set 指针——如果已经存在一个包含所需 cgroup 集合的 css_set，则复用该组，否则分配一个新的 css_set。通过查找哈希表来定位合适的现有 css_set。

为了允许从某个 cgroup 访问构成它的 css_set（进而访问任务），一组 cg_cgroup_link 对象构成了一个格；每个 cg_cgroup_link 通过其 cgrp_link_list 字段链接进单个 cgroup 的 cg_cgroup_links 列表，并通过其 cg_link_list 字段链接进单个 css_set 的 cg_cgroup_links 列表。

因此，可以通过遍历引用该 cgroup 的每个 css_set、并进一步遍历每个 css_set 的任务集，来列出该 cgroup 中的任务集合。

使用 Linux 虚拟文件系统（vfs）来表示 cgroup 层级，为 cgroups 提供了一个熟悉的权限和命名空间，同时只需最少的额外内核代码。

### 1.4 notify_on_release 的作用是什么？


如果某 cgroup 中的 notify_on_release 标志被启用（1），那么每当该 cgroup 中的最后一个任务离开（退出或附加到其他 cgroup），并且该 cgroup 的最后一个子 cgroup 被移除时，内核就会运行该层级根目录中 "release_agent" 文件内容所指定的命令，并提供被遗弃 cgroup 的路径名（相对于 cgroup 文件系统的挂载点）。这样就实现了被遗弃 cgroups 的自动移除。系统启动时根 cgroup 中 notify_on_release 的默认值为禁用（0）。其他 cgroup 在创建时的默认值，是其父级 notify_on_release 设置的当前值。cgroup 层级 release_agent 路径的默认值为空。

### 1.5 clone_children 的作用是什么？


该标志只影响 cpuset 控制器。如果某 cgroup 中启用了 clone_children 标志（1），新的 cpuset cgroup 将在初始化时从父级复制其配置。

### 1.6 如何使用 cgroups？


要启动一个将被包含在 cgroup 中的新作业，使用
```

 1) mount -t tmpfs cgroup_root /sys/fs/cgroup
 2) mkdir /sys/fs/cgroup/cpuset
 3) mount -t cgroup -ocpuset cpuset /sys/fs/cgroup/cpuset
 4) Create the new cgroup by doing mkdir's and write's (or echo's) in
    the /sys/fs/cgroup/cpuset virtual file system.
 5) Start a task that will be the "founding father" of the new job.
 6) Attach that task to the new cgroup by writing its PID to the
    /sys/fs/cgroup/cpuset tasks file for that cgroup.
 7) fork, exec or clone the job tasks from this founding father task.

```
例如，下面这组命令将建立一个名为 "Charlie" 的 cgroup，其中只包含 CPU 2 和 3，以及内存节点 1，
```

  mount -t tmpfs cgroup_root /sys/fs/cgroup
  mkdir /sys/fs/cgroup/cpuset
  mount -t cgroup cpuset -ocpuset /sys/fs/cgroup/cpuset
  cd /sys/fs/cgroup/cpuset
  mkdir Charlie
  cd Charlie
  /bin/echo 2-3 > cpuset.cpus
  /bin/echo 1 > cpuset.mems
  /bin/echo $$ > tasks
  sh
  # The subshell 'sh' is now running in cgroup Charlie
  # The next line should display '/Charlie'
  cat /proc/self/cgroup

```
## 2. 使用示例与语法


### 2.1 基本用法


创建、修改、使用 cgroups 都可以通过 cgroup 虚拟文件系统完成。

```

  # mount -t cgroup xxx /sys/fs/cgroup

```
"xxx" 不会被 cgroup 代码解释，但它会出现在 /proc/mounts 中，因此可以是你喜欢的任何有用的标识字符串。

注意：某些子系统在没有用户先提供一些输入时无法工作。例如，如果启用了 cpusets，用户必须先为每个新建的 cgroup 填充 cpus 和 mems 文件，该组才能被使用。

如 `1.2 Why are cgroups needed?` 一节所述，你应该为想要控制的每种单一资源或资源组创建不同的 cgroups 层级。因此，你应该在 /sys/fs/cgroup 上挂载一个 tmpfs，并为每个 cgroup 资源或资源
```

  # mount -t tmpfs cgroup_root /sys/fs/cgroup
  # mkdir /sys/fs/cgroup/rg1

```
要挂载一个仅包含 cpuset 和 memory 的 cgroup 层级
```

  # mount -t cgroup -o cpuset,memory hier1 /sys/fs/cgroup/rg1

```
虽然目前支持重新挂载 cgroups，但不推荐使用。重新挂载允许更改绑定的子系统和 release_agent。重新绑定几乎没什么用处，因为它只在层级为空时有效，而且 release_agent 本身应被常规的 fsnotify 取代。对重新挂载的支持将在未来被移除。

```

  # mount -t cgroup -o cpuset,release_agent="/sbin/cpuset_release_agent" \
    xxx /sys/fs/cgroup/rg1

```
注意，多次指定 'release_agent' 将返回失败。

注意，更改子系统集合目前仅在层级由单一（根）cgroup 构成时才受支持。支持从现有 cgroup 层级任意绑定/解绑子系统的能力，计划在将来实现。

然后在 /sys/fs/cgroup/rg1 下，你可以找到一个与系统中 cgroups 树相对应的树。例如，/sys/fs/cgroup/rg1 是持有整个系统的 cgroup。

```

  # echo "/sbin/new_release_agent" > /sys/fs/cgroup/rg1/release_agent

```
它也可以通过重新挂载来更改。

```

  # cd /sys/fs/cgroup/rg1
  # mkdir my_cgroup

```
现在你想用这个 cgroup 做点什么：

  # cd my_cgroup

```

  # ls
  cgroup.procs notify_on_release tasks
  (plus whatever files added by the attached subsystems)

```

```

  # /bin/echo $$ > tasks

```
你也可以在你的 cgroup 内部使用 mkdir 创建 cgroups
```

  # mkdir my_sub_cs

```

```

  # rmdir my_sub_cs

```
如果该 cgroup 正在使用中（内部有 cgroup、或已附加进程、或被其他子系统特定的引用保持存活），这将失败。

### 2.2 附加进程


```

  # /bin/echo PID > tasks

```
注意，是 PID 而不是 PIDs。你一次只能附加一个任务。
```

  # /bin/echo PID1 > tasks
  # /bin/echo PID2 > tasks
	  ...
  # /bin/echo PIDn > tasks

```

```

  # echo 0 > tasks

```
你可以使用 cgroup.procs 文件代替 tasks 文件，一次性移动一个线程组中的所有线程。将线程组中任意任务的 PID 回显到 cgroup.procs 会使该线程组中的所有任务都附加到该 cgroup。向 cgroup.procs 写入 0 会移动写入任务所在线程组中的所有任务。

注意：由于每个任务在每个已挂载层级中始终恰好是一个 cgroup 的成员，要将任务从其当前 cgroup 移除，你必须通过写入新 cgroup 的 tasks 文件将它移入一个新的 cgroup（可能是根 cgroup）。

注意：由于某些 cgroup 子系统施加的限制，将进程移动到另一个 cgroup 可能会失败。

### 2.3 按名称挂载层级


在挂载 cgroups 层级时传入 name=<x> 选项，会将给定名称与该层级关联。这可以在挂载一个已存在的层级时使用，以便按名称而不是按其活动子系统集合来引用它。每个层级要么无名，要么具有一个唯一名称。

名称应当匹配 [\w.-]+

当为新层级传入 name=<x> 选项时，你需要手动指定子系统；当你为子系统指定名称时，不支持"未显式指定任何子系统时挂载所有子系统"的传统行为。

子系统的名称会作为层级描述的一部分出现在 /proc/mounts 和 /proc/<pid>/cgroups 中。


## 3. 内核 API


### 3.1 概述


每个想要挂接到通用 cgroup 系统的内核子系统都需要创建一个 cgroup_subsys 对象。它包含各种方法（即来自 cgroup 系统的回调），以及一个将由 cgroup 系统分配的子系统 ID。

cgroup_subsys 对象中的其他字段包括：

- subsys_id：子系统的唯一数组索引，指示该子系统应管理的 cgroup->subsys[] 中的条目。

- name：应初始化为一个唯一的子系统名称。长度不应超过 MAX_CGROUP_TYPE_NAMELEN。

- early_init：指示该子系统是否需要在系统启动时提前初始化。

系统创建的每个 cgroup 对象都有一个按子系统 ID 索引的指针数组；该指针完全由子系统管理；通用的 cgroup 代码永远不会触及这个指针。

### 3.2 同步


cgroup 系统使用一个全局互斥体 cgroup_mutex。任何想要修改 cgroup 的代码都应获取它。它也可以被获取以阻止 cgroups 被修改，但在那种情况下使用更具体的锁可能更合适。

更多细节见 kernel/cgroup.c。

子系统可以通过 cgroup_lock()/cgroup_unlock() 函数获取/释放 cgroup_mutex。

可以通过以下方式访问任务的 cgroup 指针：
- 持有 cgroup_mutex 时
- 持有任务的 alloc_lock 时（通过 task_lock()）
- 在 rcu_read_lock() 临界区内通过 rcu_dereference()

### 3.3 子系统 API


每个子系统应当：

- 在 linux/cgroup_subsys.h 中添加一个条目
- 定义一个名为 <name>_cgrp_subsys 的 cgroup_subsys 对象

每个子系统可以导出以下方法。唯一必需的方法是 css_alloc/free。其他为 null 的方法被假定为成功的空操作。

`struct cgroup_subsys_state **css_alloc(struct cgroup **cgrp)`
(cgroup_mutex held by caller)

调用以为 cgroup 分配一个子系统状态对象。子系统应当为传入的 cgroup 分配其子系统状态对象，成功时返回指向新对象的指针，否则返回 ERR_PTR() 值。成功后，子系统指针应指向一个 cgroup_subsys_state 类型的结构（通常内嵌于更大的、子系统特定的对象中），该结构将由 cgroup 系统初始化。注意，在初始化时会调用本函数以创建该子系统的根子系统状态；这种情况可以通过传入的 cgroup 对象具有 NULL 父级（因为它是层级的根）来识别，这里也适合放置初始化代码。

`int css_online(struct cgroup *cgrp)`
(cgroup_mutex held by caller)

在 @cgrp 成功完成所有分配并对 cgroup_for_each_child/descendant_*() 迭代器可见之后调用。子系统可以通过返回 -errno 来选择使创建失败。该回调可用于实现可靠的状态共享与沿层级的传播。详见 cgroup_for_each_live_descendant_pre() 上的注释。

`void css_offline(struct cgroup *cgrp);`
(cgroup_mutex held by caller)

这是 css_online() 的反向操作，当且仅当 css_online() 已在 @cgrp 上成功时才会被调用。它标志着 @cgrp 终结过程的开始。@cgrp 正在被移除，子系统应开始丢弃其持有的对 @cgrp 的所有引用。当所有引用都被丢弃后，cgroup 移除将进入下一步——css_free()。在此回调之后，子系统应将 @cgrp 视为已死亡。

`void css_free(struct cgroup *cgrp)`
(cgroup_mutex held by caller)

cgroup 系统即将释放 @cgrp；子系统应释放其子系统状态对象。调用此方法时，@cgrp 已完全不再被使用；@cgrp->parent 仍然有效。（注意——如果本子系统的 create() 方法已为新 cgroup 调用之后发生错误，也可能针对新建的 cgroup 调用。）

`int can_attach(struct cgroup **cgrp, struct cgroup_taskset **tset)`
(cgroup_mutex held by caller)

在将一个或多个任务移入 cgroup 之前调用；如果子系统返回错误，这将中止附加操作。@tset 包含待附加的任务，且保证其中至少有一个任务。

如果 taskset 中有多个任务，则：
  - 保证都来自同一个线程组
  - @tset 包含该线程组中的所有任务，无论它们是否正在切换 cgroup
  - 第一个任务是组长（leader）

每个 @tset 条目还包含任务的旧 cgroup，而并未切换 cgroup 的任务可以使用 cgroup_taskset_for_each() 迭代器轻松跳过。注意，在 fork 时不会调用本方法。如果本方法返回 0（成功），那么当调用者持有 cgroup_mutex 时此有效性应保持不变，并且保证将来会调用 attach() 或 cancel_attach() 之一。

`void css_reset(struct cgroup_subsys_state *css)`
(cgroup_mutex held by caller)

一个可选操作，应将 @css 的配置恢复到初始状态。目前它仅用于统一层级（unified hierarchy），即当某个子系统通过 "cgroup.subtree_control" 在某 cgroup 上被禁用、但因为其他子系统依赖它而应保持启用时。cgroup 核心会通过移除关联的接口文件使这样的 css 不可见，并调用此回调，以便被隐藏的子系统可以回到初始的中性状态。这可以阻止来自隐藏 css 的意外资源控制，并确保配置在日后再次可见时处于初始状态。

`void cancel_attach(struct cgroup **cgrp, struct cgroup_taskset **tset)`
(cgroup_mutex held by caller)

在 can_attach() 已成功、但任务附加操作失败时被调用。如果某个子系统的 can_attach() 有副作用，应提供此函数，以便该子系统能够实现回滚。若没有副作用则无需提供。本函数只会针对 can_attach() 操作已成功的子系统调用。参数与 can_attach() 相同。

`void attach(struct cgroup **cgrp, struct cgroup_taskset **tset)`
(cgroup_mutex held by caller)

在任务已附加到 cgroup 之后调用，以允许任何需要内存分配或阻塞的附加后活动。参数与 can_attach() 相同。

`void fork(struct task_struct *task)`

当任务被 fork 进一个 cgroup 时调用。

`void exit(struct task_struct *task)`

在任务退出期间调用。

`void free(struct task_struct *task)`

在 task_struct 被释放时调用。

`void bind(struct cgroup *root)`
(cgroup_mutex held by caller)

当某个 cgroup 子系统被重新绑定到不同的层级和根 cgroup 时调用。目前这只涉及默认层级（从不包含子 cgroup）与被创建/销毁的层级（因此也没有子 cgroup）之间的移动。

## 4. 扩展属性的使用


cgroup 文件系统在其目录和文件中支持某些类型的扩展属性。当前支持的类型有：

 - 受信任的（XATTR_TRUSTED）
 - 安全的（XATTR_SECURITY）

二者都需要 CAP_SYS_ADMIN 能力才能设置。

与 tmpfs 中一样，cgroup 文件系统中的扩展属性使用内核内存存储，建议将其使用保持在最低限度。这正是为什么不支持用户自定义扩展属性的原因，因为任何用户都可以设置它们，且值的大小没有限制。

当前已知的该功能使用者包括：SELinux（用于限制容器中 cgroup 的使用）和 systemd（用于各类元数据，例如 cgroup 中的主 PID（systemd 为每个服务创建一个 cgroup））。

## 5. 问题


```

  Q: what's up with this '/bin/echo' ?
  A: bash's builtin 'echo' command does not check calls to write() against
     errors. If you use it in the cgroup file system, you won't be
     able to tell whether a command succeeded or failed.

  Q: When I attach processes, only the first of the line gets really attached !
  A: We can only return one error code per call to write(). So you should also
     put only ONE PID.

```
