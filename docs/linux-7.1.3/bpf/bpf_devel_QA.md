## 如何BPF 子系统交

本文档为 BPF 子系统提供与报告缺陷、提交补丁以及为稳定版内核排队补丁相关的各种工作流信息
关于提交补丁的一般信息，请参Documentation/process/submitting-patches.rst。本文档仅描述与 BPF 相关的额外细节
    :local:
    :depth: 2

## 报告缺陷


### 问：如何报告 BPF 内核代码的缺陷？


答：由于所BPF 内核开发以bpftool iproute2 BPF 加载器的开发都通过 bpf 内核邮件列表进行，请将发现的任何 BPF 相关问题报告到以下邮件列表：

 bpf@vger.kernel.org

这也可能包括XDP、BPF 跟踪等相关的议题
鉴于 netdev 流量很高，请同时也把 BPF 维护者加Cc（来自内`MAINTAINERS` 文件）：

- Alexei Starovoitov <ast@kernel.org>
- Daniel Borkmann <daniel@iogearbox.net>

如果已经定位到有问题的提交，请确保把实际的提交作者也保留在报告的 Cc 中。通常可以通过内核git 树来识别他们
**请不要将 BPF 问题报告bugzilla.kernel.org，因为那几乎可以保证所报告的问题被忽略*

## 提交补丁


### 问：在送出审查之前，如何在我的改动上运BPF CI

答：BPF CI 基于 GitHub，托管在 https://github.com/kernel-patches/bpf。虽GitHub 也提供可以达到相同效果的 CLI，但这里我们关注基于 UI 的工作流
以下步骤说明了如何为你的补丁启动一CI 运行
- 在你自己的账户中创建上述仓库fork（一次性操作）

- 在本地克隆该 fork，检出一个跟bpf-next bpf 分支的新分支，并把你待测试的补丁应用到它之上

- 将本地分支推送到你的 fork，并分别针对 kernel-patches/bpf bpf-next_base bpf_base 分支创建 pull request

pull request 创建后不久，CI 工作流就会运行。注意计算容量与正在被检查的上游提交的补丁是共享的，因此根据利用率，运行可能需要一段时间才能完成
另请注意，两个基础分支（bpf-next_base bpf_base）会随着补丁被推送到它们所跟踪的相应上游分支而更新。因此，你的补丁集也会自动（尝试）被变基。这种行为可能导致一CI 运行被中止并以新的基线重新启动
### 问：我需要把 BPF 补丁提交到哪个邮件列表？


答：请将你的 BPF 补丁提交bpf 内核邮件列表
 bpf@vger.kernel.org

如果你的补丁涉及各种不同的子系统（例如网络、跟踪、安全等），请确保也把相关的内核邮件列表和那里的维护者加Cc，以便他们能够审查这些更改并给出他们Acked-by
### 问：在哪里可以找BPF 子系统当前正在讨论的补丁

答：所有抄送（Cc）到 netdev 的补丁都netdev patchwork 项目下排队等待审查：

  https://patchwork.kernel.org/project/netdevbpf/list/

那些BPF 为目标的补丁会被分配给一'bpf' 代理（delegate），BPF 维护者进一步处理。当前正在审查的补丁队列可以在以下位置找到：

  https://patchwork.kernel.org/project/netdevbpf/list/?delegate=121173

一旦补丁由整个 BPF 社区审查并由 BPF 维护者批准，它们patchwork 中的状态会被改'Accepted'，提交者会通过邮件收到通知。这意味着BPF 的角度看这些补丁没问题，并且已经被应用到两个 BPF 内核树之一
如果来自社区的反馈要求重新提交（respin）补丁，它们patchwork 中的状态会被设'Changes Requested'，并从当前审查队列中清除。对于补丁被拒绝或不适用BPF 树（但分配给'bpf' 代理）的情况也同样如此
### 问：这些更改是如何进Linux 的？


答：有两BPF 内核树（git 仓库）。一旦补丁被 BPF 维护者接受，它们就会被应用到两个 BPF 树之一
 - https://git.kernel.org/pub/scm/linux/kernel/git/bpf/bpf.git/
 - https://git.kernel.org/pub/scm/linux/kernel/git/bpf/bpf-next.git/

bpf 树本身仅用于修复，bpf-next 用于特性、清理或其他类型的改进（“类next 的内容”）。这类似于网络子系统net net-next 树。bpf bpf-next 都只会有 master 分支，以简化补丁应当变基到哪个分支的问题
bpf 树中累积BPF 补丁会定期被拉入 net 内核树。同样地，被接受进入 bpf-next 树的累积 BPF 补丁会进net-next 树。net net-next 都由 David S. Miller 维护。从那里，它们会进入Linus Torvalds 维护的内核主线树。要了解 net net-next 合并到主线树的过程，请参netdev 子系统的文档 Documentation/process/maintainer-netdev.rst
偶尔，为了防止合并冲突，我们可能会向其他树（例如 tracing）发送包含一小部分补丁的 pull request，但 net net-next 始终是要集成的目标主树
pull request 会包含累积补丁的高层摘要，并可以通过以下主题行在 netdev 内核邮件列表上搜索（`yyyy-mm-dd` pull 的日期）
```

  pull-request: bpf yyyy-mm-dd
  pull-request: bpf-next yyyy-mm-dd

```
### 问：如何指明我的补丁应该应用到哪个树（bpf 还是 bpf-next）？


答：过程netdev 子系统文Documentation/process/maintainer-netdev.rst 中描述的完全相同，因此请阅读了解。主题行必须指明该补丁是一个修复还是“类next”的内容，以便让维护者知道它是以 bpf 还是 bpf-next 为目标
对于最终进bpf -> net 树的修复，主题必
```

  git format-patch --subject-prefix='PATCH bpf' start..finish

```
对于最终应进入

```

  git format-patch --subject-prefix='PATCH bpf-next' start..finish

```
如果你不确定补丁或补丁系列是否应该直接进bpf net，或者直接进bpf-next net-next，那么主题行net net-next 为目标也没问题。最终由维护者来决定补丁的委派
如果明确补丁应当进入 bpf bpf-next 树，请确保针对那些树对补丁进行变基，以减少潜在的冲突
如果补丁或补丁系列需要返工并在第二版或更晚的修订中再次发出，则还需要添
```

  git format-patch --subject-prefix='PATCH bpf-next v2' start..finish

```
当补丁系列被要求修改时，请始终将整个补丁系列连同反馈一起再次发送（绝不要在原系列之上单独发diff）
### 问：当一个补丁被应用bpf bpf-next 树时意味着什么？


答：这意味着BPF 的角度看，该补丁看起来适合进入主线
但请注意，这并不等于补丁最终会自动net net-next 树接受的定论
bpf 内核邮件列表上，审查可以随时到来。如果围绕某个补丁的讨论得出结论认为它不能按原样被接受，我们要么会应用一个后续的修复，要么会将其从树中完全丢弃。因此，我们也保留在认为有必要时对树进行变基的权利。毕竟，该树的目的是
i) 累积并暂BPF 补丁，以便集成到诸如 net net-next 之类的树中，以及

ii) 在补丁进一步前进之前，对其运行广泛BPF 测试套件和工作负载
一BPF pull request David S. Miller 接受，补丁就会分别进net net-next 树，并从那里进一步进入主线。同样，关于它们多久合并到主线的更多信息，请参阅 netdev 子系统的文档 Documentation/process/maintainer-netdev.rst
### 问：我需要等待多长时间才能收到关BPF 补丁的反馈？


答：我们尽量保持较低的延迟。通常给出反馈的时间约2 3 个工作日。它可能会根据更改的复杂性和当前的补丁负载而变化
### 问：你们多久net net-next 之类的主要内核树发送一pull request

答：为了不让 bpf bpf-next 中累积过多补丁，会相当频繁地发pull request
作为经验法则，预计每个树都会定期在周末发pull request。在某些情况下，根据当前的补丁负载或紧急程度，pull request 也可能在周中额外发出
### 问：在合并窗口开启时，补丁会被应用到 bpf-next 吗？


答：在合并窗口开启期间，bpf-next 不会被处理。这大致类似net-next 补丁的处理方式，因此请随意阅netdev 文档 Documentation/process/maintainer-netdev.rst 以了解进一步的细节
在那两周的合并窗口期间，我们可能会要求你bpf-next 再次开启后重新发送你的补丁系列。一Linus 在合并窗口之后发布了 `v*-rc1`，我们就继续处理 bpf-next
对于没有订阅内核邮件列表的人，David S. Miller 还维护了一个关net-next 的状态页面提供指导：

  http://vger.kernel.org/~davem/net-next.html

### 问：验证器更改与测试用例


问：我做BPF 验证器的更改，需要为 BPF 内核 selftests_ 添加测试用例吗？

答：如果补丁改变了验证器的行为，那么是的，绝对有必要BPF 内核 selftests_ 套件添加测试用例。如果它们不存在而我们认为需要，我们可能会在接受任何更改之前要求提供它们
特别是，test_verifier.c 跟踪着大量BPF 测试用例，包LLVM BPF 后端可能从受C 代码生成的许多边界情况。因此，添加测试用例对于确保未来的更改不会意外影响先前的用例绝对至关重要。因此，请这样看待这些测试用例：未由 test_verifier.c 跟踪的验证器行为有可能发生变化
### 问：samples/bpf selftests 的取舍？


问：我应该何时向 `samples/bpf/` 添加代码，又何时BPF 内核 selftests_ 添加代码
答：一般来说，我们更倾向于向 BPF 内核 selftests_ 添加内容，而不`samples/bpf/`。理由很简单：内核 selftests 会被各种机器人定期运行，以测试内核回归
我们BPF selftests 添加的测试用例越多，覆盖率就越好，它们被意外破坏的可能性就越小。并不是BPF 内核 selftests 不能演示某个特定特性如何使用
话虽如此，`samples/bpf/` 可能是人们入门的好地方，因此把简单的特性演示放`samples/bpf/`，而把高级功能性和边界情况测试放入内核 selftests 可能是恰当的做法
如果你的示例看起来像一个测试用例，那就改用 BPF 内核 selftests
### 问：我应该何时向 bpftool 添加代码

答：bpftool（位tools/bpf/bpftool/ 下）的主要目的是提供一个集中的用户空间工具，用于调试和自省内核中活跃的 BPF 程序和映射。如果与 BPF 相关UAPI 更改使得可以 dump 程序或映射的附加信息，那bpftool 也应被扩展以支持 dump 它们
### 问：我应该何时向 iproute2 BPF 加载器添加代码？


答：对于XDP tc 层（例如 `cls_bpf`）相关的 UAPI 更改，约定是这些控制路径相关的更改也要从用户空间一侧添加到 iproute2 BPF 加载器中。这不仅有助于让 UAPI 更改被正确设计为可用，也能让这些更改对主要下游发行版的更广泛用户群可用
### 问：你们也接受针iproute2 BPF 加载器的补丁吗？


答：针对 iproute2 BPF 加载器的补丁必须发送到
  netdev@vger.kernel.org

虽然这些补丁不由 BPF 内核维护者处理，但请把他们也保留Cc 中，以便能够审查
iproute2 的官git 仓库Stephen Hemminger 维护，可以在以下位置找到
  https://git.kernel.org/pub/scm/linux/kernel/git/shemminger/iproute2.git/

补丁需要带'``[PATCH iproute2 master]`' '`[PATCH iproute2 net-next]`' 的主题前缀`master``' '`net-next`' 描述补丁应当被应用到的目标分支。也就是说，如果内核更改进入net-next 内核树，那么相关iproute2 更改需要进iproute2 net-next 分支，否则可以将目标定为 master 分支。iproute2 net-next 分支会在当前来自 master iproute2 版本发布后合并到 master 分支
BPF 一样，这些补丁最终会出现patchwork netdev 项目下，并被委派'shemminger' 进行进一步处理：

  http://patchwork.ozlabs.org/project/netdev/list/?delegate=389

### 问：提交 BPF 补丁之前的最低要求是什么？


答：提交补丁时，务必花时间并在提*之前** properly 测试你的补丁。千万不要匆忙提交！如果维护者发现你的补丁没有经proper 测试，这很容易让他们不悦。测试补丁提交是硬性要求！

请注意，进入 bpf 树的修复**必须**包含 `Fixes:` 标签。针bpf-next 的修复同样如此，其中受影响的提交位于 net-next（或某些情况下的 bpf-next）中。`Fixes:` 标签对于识别后续提交至关重要，并且对需要做向后移植的人帮助极大，因此它是必备的
我们也不接受带有空提交信息的补丁。花时间 proper 地撰写高质量的提交信息，这至关重要！

不妨这样想：一个月后查看你代码的其他开发者需要理*为什*某项更改以那种方式完成，以及原作者在分析或假设中是否存在缺陷。因此，提供 proper 的理由并描述更改的用例是必须的
包含多于 1 个补丁的提交必须有一封封面信，其中包含该系列的高层描述。这个高层摘要随后会BPF 维护者放入合并提交中，以便将来也能从 git 日志中查阅
### 问：改变 BPF JIT LLVM 的特

问：当新增一条需BPF JIT LLVM 集成的指令或特性时，我需要考虑什么？

答：我们努力让所BPF JIT 保持最新，以便在不同架构上运行 BPF 程序时能够保证相同的用户体验，而不会在启用内核BPF JIT 时让程序退回到效率较低的解释器
如果你无法实现或测试某些架构所需JIT 更改，请与相BPF JIT 开发者合作，以便及时实现该特性。请参git 日志（`arch/*/net/`）来定位可以提供帮助的相关人员
同时始终确保为新指令添加 BPF 测试用例（例test_bpf.c test_verifier.c），以便它们能获得广泛的测试覆盖，并帮助对各BPF JIT 进行运行时测试
对于新的 BPF 指令，一旦更改被接受进入 Linux 内核，请LLVM BPF 后端中实现支持。更多信息请参阅下面LLVM_ 一节
### 问：“BPF_INTERNAL符号命名空间是做什么用的？


答：BPF_INTERNAL 导出的符号只能被 BPF 基础设施使用，例如带light skeleton 的预加载内核模块。BPF_INTERNAL 之外的大多数符号也不期望BPF 之外的代码使用。符号可能缺少该标识，因为它们早于命名空间的存在，或者由于疏忽
## 稳定版提

### 问：我需要在稳定版内核中使用某个特定BPF 提交。我该怎么做？


答：如果你需要在稳定版内核中使用某个特定的修复，请先检查该提交是否已经应用在相关的 `linux-*.y` 分支中：

  https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux-stable.git/

如果没有，则BPF 维护者发一封邮件，并抄送（Cc）netdev 内核邮件列表，请求将该修复排队：

  netdev@vger.kernel.org

这个过程总体上与 netdev 本身相同，另请参阅网络子系统的文Documentation/process/maintainer-netdev.rst
### 问：你们也会向后移植到当前未被作为稳定版维护的内核吗

答：不会。如果你需要在当前未被稳定版维护者维护的内核中使用某个特定的 BPF 提交，那就只能靠你自己了
当前的稳定版和长期稳定版内核都列在这里：

  https://www.kernel.org/

### 问：我即将提交的 BPF 补丁也需要进入稳定版


我该怎么做？

答：规则netdev 补丁提交的一般规则相同，请参netdev 文档 Documentation/process/maintainer-netdev.rst
绝不要将 "`Cc: stable@vger.kernel.org`" 添加到补丁描述中，而是BPF 维护者排队这些补丁。这可以用一个注释来完成，例如放在补丁的 `---` 部分之下（该部分不会进入 git 日志）。或者，也可以通过邮件做一个简单的请求来代替
### 问：排队稳定版补

问：我在哪里可以找到当前已排队、将被提交到稳定版的 BPF 补丁
答：一旦修复严重缺陷的补丁被应用到 bpf 树，它们就会在以下位置排队等待提交到稳定版：

  http://patchwork.ozlabs.org/bundle/bpf/stable/?state=*

它们至少会一直在那里搁置，直到相关的提交进入主线内核树
在经历了更广泛的曝光之后，排队的补丁会由 BPF 维护者提交给稳定版维护者
## 测试补丁


### 问：如何运行 BPF selftests


答：在你启动进入新编译的内核之后，进BPF selftests_ 套件以测BPF 功能（当
```

  $ cd tools/testing/selftests/bpf/
  $ make

```
```

  $ sudo ./test_verifier

```
验证器测试会打印出当前正在执行的所有检查。运行所有测试结束时的摘要会 dump

```

  Summary: 418 PASSED, 0 FAILED

```
为了运行全部 BPF selftests，以下命令为

```

  $ sudo make run_tests

```
有关详细信息，请参阅 [kernel selftest documentation </dev-tools/kselftest>](kernel selftest documentation </dev-tools/kselftest>)
为了使通过的测试数量最大化，被测内核的 .config 应尽可能tools/testing/selftests/bpf 中的配置文件片段匹配
最后，为确保支持最新的 BPF Type Format 特性（Documentation/bpf/btf.rst 中讨论），对于以 CONFIG_DEBUG_INFO_BTF=y 构建的内核，需pahole 1.16 版本。pahole dwarves 包提供，也可以从以下位置从源码构建：

https://github.com/acmel/dwarves

pahole v1.13 起、在提交 21507cd3e97b（“pahole: add libbpf as submodule under lib/bpf”）之后开始使libbpf 的定义和 API。它git 仓库配合良好，因libbpf 子模块会使用 “git submodule update --init --recursive来更新
不幸的是，github 默认的发布源代码不包libbpf 子模块源代码，这会导致构建问题；来自 https://git.kernel.org/pub/scm/devel/pahole/pahole.git/ tarball github 相同，你可以从以下位置获取带有相libbpf 子模块代码的源码 tarball
https://fedorapeople.org/~acme/dwarves

某些发行版已经打包了 pahole 1.16 版本，例Fedora、Gentoo
### 问：我应该针对哪BPF 内核 selftests 版本来运行我的内核？


答：如果你运行的是内`xyz`，那么也始终运行来自该内`xyz` BPF 内核 selftests。不要指望来自最新主线树BPF selftest 会一直全部通过
特别是，test_bpf.c test_verifier.c 有大量测试用例，并且会随新的 BPF 测试序列不断更新，或者现有用例会适应性地修改以配合验证器的更改（例如由于验证器变得更智能、能够更好地跟踪某些东西）
## LLVM


### 问：我在哪里可以找到支持 BPF LLVM

答：LLVM BPF 后端自版3.7.1 起就LLVM 的上游代码
如今所有主要的发行版都发布了启用了 BPF 后端LLVM，因此对于绝大多数用例，不再需要手工编LLVM，只需安装发行版提供的包即可
LLVM 的静态编译器通过以下方式列出受支持的目标

```

     $ llc --version
     LLVM (http://llvm.org/):
       LLVM version 10.0.0
       Optimized build.
       Default target: x86_64-unknown-linux-gnu
       Host CPU: skylake

       Registered Targets:
         aarch64    - AArch64 (little endian)
         bpf        - BPF (host endian)
         bpfeb      - BPF (big endian)
         bpfel      - BPF (little endian)
         x86        - 32-bit X86: Pentium-Pro and above
         x86-64     - 64-bit X86: EM64T and AMD64

```
为了让开发者能够利用添加到 LLVM BPF 后端的最新特性，建议运行最新的 LLVM 版本。对BPF 内核特性（例如BPF 指令集的增补）的支持通常是一同开发的
所LLVM 版本都可以在以下位置找到：http://releases.llvm.org/

### 问：明白了，那我到底该如何手动构LLVM

答：我们建议希望获得最快增量构建的开发者使Ninja 构建系统，你可以在系统的包管理器中找到它，通常包名ninja ninja-build
你需ninja、cmake gcc-c++ 作为 LLVM 的构建先决条件。一旦设置好，就着手构建最新的 LLVM clang 版本

```

     $ git clone https://github.com/llvm/llvm-project.git
     $ mkdir -p llvm-project/llvm/build
     $ cd llvm-project/llvm/build
     $ cmake .. -G "Ninja" -DLLVM_TARGETS_TO_BUILD="BPF;X86" \
                -DLLVM_ENABLE_PROJECTS="clang"    \
                -DCMAKE_BUILD_TYPE=Release        \
                -DLLVM_BUILD_RUNTIME=OFF
     $ ninja

```
构建好的二进制文件随后可以在 build/bin/ 目录中找到，你可以将 PATH 变量指向那里
`-DLLVM_TARGETS_TO_BUILD` 设置为你希望构建的目标，你可以在 llvm-project/llvm/lib/Target 目录中找到完整的目标列表
### 问：报告 LLVM BPF 问题


问：我是否应该就 LLVM BPF 代码生成后端的问题，或者关于验证器拒绝接受LLVM 生成代码，通知 BPF 内核维护者？

答：是的，请务必通知
LLVM BPF 后端是整BPF 基础设施的关键部分，并且与来自内核一侧的程序验证深度绑定。因此，任何一侧的问题都需要在必要时进行调查和修复
因此，请确保netdev 内核邮件列表上提出这些问题，并把负责 LLVM 和内核部分的 BPF 维护者加Cc
- Yonghong Song <yhs@fb.com>
- Alexei Starovoitov <ast@kernel.org>
- Daniel Borkmann <daniel@iogearbox.net>

LLVM 也有一issue 跟踪器，可以在其中找BPF 相关的缺陷：

  https://bugs.llvm.org/buglist.cgi?quicksearch=bpf

不过，最好还是通过邮件列表联系，并把维护者加Cc 中
### 问：内核LLVM 的新 BPF 指令


问：我向内核添加了一条新BPF 指令，如何将其集成到 LLVM 中？

答：LLVM BPF 后端提供了一`-mcpu` 选择器，以便允许选择 BPF 指令集扩展。在 llvm 20 版本之前，使`generic` 处理器目标，BPF 的基础指令集（v1）。从 llvm 20 起，默认处理器目标已更改为指令集 v3
LLVM 有一个选项可以选择 `-mcpu=probe`，它会探测宿主机内核以获得受支持BPF 指令集扩展，并自动选择最优集合
```

     $ llc -march bpf -mcpu=help
     Available CPUs for this target:

       generic - Select the generic processor.
       probe   - Select the probe processor.
       v1      - Select the v1 processor.
       v2      - Select the v2 processor.
     [...]

```
Linux 内核新添加的 BPF 指令需要遵循相同的方案，提升指令集版本并为这些扩展实现探测，以`-mcpu=probe` 用户能在升级内核时透明地受益于该优化
如果你无法实现对新添加的 BPF 指令的支持，请向 BPF 开发者寻求帮助
顺便一提，BPF 内核 selftests `-mcpu=probe` 运行以获得更好的测试覆盖
### 问：针对 bpf 目标clang 标志

问：某些情况下使clang 标志 `--target=bpf`，而在其他情况下使用与底层架构匹配的默clang 目标。区别是什么，我应该何时使用哪一个？

答：尽管 LLVM IR 生成和优化尽量保持架构无关，`--target=<arch>` 仍然对生成的代码有一定影响：

- BPF 程序可能会递归包含带有文件作用域内联汇编代码的头文件。默认目标可以很好地处理这一点，`bpf` 目标可能会失败，如果 bpf 后端汇编器不理解这些汇编代码（大多数情况下确实如此）
- 当不使用 `-g` 编译时，默认目标的对象文件中可能会存在额外的 elf 节，例如 .eh_frame .rela.eh_frame，`bpf` 目标则不会
- 默认目标可能会把 C switch 语句转换switch 表查找和跳转操作。由switch 表被放在全局只读节中，bpf 程序将无法加载。`bpf` 目标不支switch 表优化。可以使clang 选项 `-fno-jump-tables` 来禁switch 表的生成
- 对于 clang `--target=bpf`，无论底clang 二进制或默认目标（或内核）是否为 32 位，都保证指针或 long / unsigned long 类型始终具有 64 位宽度。然而，当使用原clang 目标时，它会根据这些类型基于底层架构的约定进行编译，也就是说32 位架构的情况下，指针long / unsigned long 类型（例如在 BPF 上下文结构中）将具有 32 位宽度，BPF LLVM 后端仍以 64 位运行。原生目标主要在跟踪中需要遍`pt_regs` 或其CPU 寄存器宽度相关的内核结构时使用。否则，通常推荐使用 `clang --target=bpf`
在以下情况下，你应该使用默认目标
- 你的程序包含某个头文件（例如 ptrace.h），它最终引入了某些包含文件作用域主机汇编代码的头文件
- 你可以添`-fno-jump-tables` 来解switch 表问题
否则，你可以使用 `bpf` 目标。此外，在以下情况下*必须**使用 bpf 目标
- 你的程序使用了带有指针或 long / unsigned long 类型、并BPF 辅助函数或上下文数据结构交互的数据结构。对这些结构的访问由 BPF 验证器验证，如果原生架构BPF 架构（例64 位）不一致，可能会导致验证失败。这方面的一个例子是 BPF_PROG_TYPE_SK_MSG 需`--target=bpf`

   https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/tools/testing/selftests/bpf/

BPF 开发愉快！
