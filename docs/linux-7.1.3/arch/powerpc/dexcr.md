
## DEXCR（Dynamic Execution Control Register，动态执行控制寄存器）


## 概述


DEXCR 是 PowerPC ISA 3.1B（Power10）引入的一个特权特殊用途寄存器（SPR），允许
对每个 CPU 的若干动态执行行为进行控制。这些行为包括推测执行（例如间接分支目标
预测）以及启用面向返回编程（ROP）的保护指令。

执行控制在硬件中表现为 DEXCR 中最多 32 位（“aspects”，方面）。每个 aspect 控制
某种行为，可以置位或清除以启用/禁用该 aspect。DEXCR 有几个用于不同目的的变体：

DEXCR
    一个特权 SPR，可控制用户空间和内核空间的 aspects
HDEXCR
    一个超管特权（hypervisor-privileged）SPR，可控制超管的 aspects，并对内核和
    用户空间强制某些 aspects。
UDEXCR
    一个可选的 ultravisor 特权 SPR，可控制 ultravisor 的 aspects。

用户空间可以使用一个专用 SPR 来检查当前 DEXCR 状态，该 SPR 提供用户空间 DEXCR
aspects 的非特权只读视图。还有一个 SPR 提供超管强制 aspects 的只读视图，它与
用户空间 DEXCR 视图相“或”，即得到进程的有效 DEXCR 状态。


## 配置


### prctl


一个进程可以使用 `PR_PPC_GET_DEXCR` 与 `PR_PPC_SET_DEXCR` 这对
```

    prctl(PR_PPC_GET_DEXCR, unsigned long which, 0, 0, 0);
    prctl(PR_PPC_SET_DEXCR, unsigned long which, unsigned long ctrl, 0, 0);

```
可能的“which”与“ctrl”值如下。注意“which”值与 DEXCR aspect 的索引之间没有
关系。

   :header-rows: 1
   :widths: 2 7 1

   - - `prctl()` which
     - Aspect name
     - Aspect index

   - - `PR_PPC_DEXCR_SBHE`
     - Speculative Branch Hint Enable (SBHE)
     - 0

   - - `PR_PPC_DEXCR_IBRTPD`
     - Indirect Branch Recurrent Target Prediction Disable (IBRTPD)
     - 3

   - - `PR_PPC_DEXCR_SRAPD`
     - Subroutine Return Address Prediction Disable (SRAPD)
     - 4

   - - `PR_PPC_DEXCR_NPHIE`
     - Non-Privileged Hash Instruction Enable (NPHIE)
     - 5

   :header-rows: 1
   :widths: 2 8

   - - `prctl()` ctrl
     - Meaning

   - - `PR_PPC_DEXCR_CTRL_EDITABLE`
     - 该 aspect 可通过 PR_PPC_SET_DEXCR 配置（仅用于获取）

   - - `PR_PPC_DEXCR_CTRL_SET`
     - 该 aspect 已置位 / 置位该 aspect

   - - `PR_PPC_DEXCR_CTRL_CLEAR`
     - 该 aspect 已清除 / 清除该 aspect

   - - `PR_PPC_DEXCR_CTRL_SET_ONEXEC`
     - 该 aspect 将在 exec 之后置位 / exec 之后置位该 aspect

   - - `PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC`
     - 该 aspect 将在 exec 之后清除 / exec 之后清除该 aspect

注意

- which 是一个普通值，而非位掩码。aspects 必须逐个处理。

- ctrl 是一个位掩码。`PR_PPC_GET_DEXCR` 返回当前配置和 onexec 配置。例如，
  `PR_PPC_GET_DEXCR` 可能返回
  ``PR_PPC_DEXCR_CTRL_EDITABLE | PR_PPC_DEXCR_CTRL_SET |
  PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC``。这表示该 aspect 当前已置位，在运行 exec 时将
  被清除，并且你可以使用 `PR_PPC_SET_DEXCR` prctl 更改它。

- set/clear 术语指的是在 DEXCR 中置位/清除该位。
```

      prctl(PR_PPC_SET_DEXCR, PR_PPC_DEXCR_IBRTPD, PR_PPC_DEXCR_CTRL_SET, 0, 0);

  将置位 DEXCR 中的 IBRTPD aspect 位，从而导致间接分支预测被禁用。

```
- `PR_PPC_GET_DEXCR` 返回的状态表示进程希望应用的值。它不包含任何替代覆盖，例如
  超管正强制该 aspect 置位。要查看真实的 DEXCR 状态，软件应直接读取相应的 SPR。

- 进程启动时的 aspect 状态在 `fork(2)` 时从父进程状态复制。该状态在 `execve(2)`
  时重置为一个固定值。`PR_PPC_SET_DEXCR` prctl() 可以控制这两个值。

- `*_ONEXEC` 控制项不会改变当前进程的 DEXCR。

使用 `PR_PPC_SET_DEXCR` 并配合 `PR_PPC_DEXCR_CTRL_SET` 或
`PR_PPC_DEXCR_CTRL_CLEAR` 之一来编辑某个 aspect。

获取和设置 DEXCR 的常见错误码如下：

   :header-rows: 1
   :widths: 2 8

   - - Error
     - Meaning

   - - `EINVAL`
     - 内核不支持 DEXCR。

   - - `ENODEV`
     - 该 aspect 内核无法识别，或硬件不支持。

`PR_PPC_SET_DEXCR` 还可能报告以下错误码：

   :header-rows: 1
   :widths: 2 8

   - - Error
     - Meaning

   - - `EINVAL`
     - ctrl 值包含无法识别的标志。

   - - `EINVAL`
     - ctrl 值包含相互冲突的标志（例如 `PR_PPC_DEXCR_CTRL_SET | PR_PPC_DEXCR_CTRL_CLEAR`）

   - - `EPERM`
     - 该 aspect 无法通过 prctl() 修改（用 PR_PPC_GET_DEXCR 检查
       PR_PPC_DEXCR_CTRL_EDITABLE 标志）。

   - - `EPERM`
     - 进程没有足够的权限执行该操作。例如，在 exec 时清除 NPHIE 是特权操作（进程
       仍可在无特权的情况下清除自身的 NPHIE aspect）。

该接口允许一个进程控制其自身的 DEXCR aspects，并设置其进程树中任何子进程的初始
DEXCR 值（直到下一个使用 `*_ONEXEC` 控制的子进程）。这允许对 DEXCR 的默认值进行
细粒度控制，例如允许容器以不同的默认值运行。


## coredump 与 ptrace


DEXCR 与 HDEXCR 的用户空间值（按此顺序）通过 `NT_PPC_DEXCR` 暴露。它们各自为 64
位且只读，用于辅助核心转储（core dump）。DEXCR 未来可能变为可写。两个寄存器的
高 32 位（对应于非用户空间位）被屏蔽掉。

如果内核配置 `CONFIG_CHECKPOINT_RESTORE` 被启用，那么 `NT_PPC_HASHKEYR` 可用，
并暴露进程的 HASHKEYR 值供读写。这是在增强安全性与检查点/恢复支持之间的权衡：进程
通常无需知道其密钥，但恢复一个进程需要设置其原始密钥。因此该密钥会出现在核心转储
中，攻击者可能从核心转储中检索到它，并有效绕过任何共享此密钥的线程上的 ROP 保护
（潜在地，所有来自同一父进程、且尚未运行 `exec()` 的线程）。
