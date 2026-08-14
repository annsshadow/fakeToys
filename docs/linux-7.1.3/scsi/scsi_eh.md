
## SCSI EH（SCSI 错误处理）


本文档描述了 SCSI 中间层（midlayer）的错误处理基础设施。
有关 SCSI 中间层的更多信息，请参阅 Documentation/scsi/scsi_mid_low_api.rst。


   [^1^] SCSI 命令如何穿过中间层并进入 EH
       [1-1] struct scsi_cmnd
       [1-2] scmd 是如何被完成的？
   	[1-2-1] 用 scsi_done 完成一个 scmd
   	[1-2-2] 用超时完成一个 scmd
       [1-3] EH 如何接管
   [^2^] SCSI EH 如何工作
       [2-1] 通过细粒度回调的 EH
   	[2-1-1] 概述
   	[2-1-2] scmd 流经 EH 的过程
   	[2-1-3] 控制流
       [2-2] 通过 transportt->eh_strategy_handler() 的 EH
   	[2-2-1] transportt->eh_strategy_handler() 之前的 SCSI 中间层条件
   	[2-2-2] transportt->eh_strategy_handler() 之后的 SCSI 中间层条件
   	[2-2-3] 需要考虑的事项


## 1. SCSI 命令如何穿过中间层并进入 EH


### 1.1 struct scsi_cmnd


每个 SCSI 命令都用 struct scsi_cmnd（即 scmd）表示。一个 scmd 有两个
list_head 将自己链接进链表。这两个分别是 scmd->list 与 scmd->eh_entry。
前者用于空闲链表或每设备分配的 scmd 链表，在本次 EH 讨论中并不重要。后者
用于完成与 EH 链表，除非另有说明，本讨论中 scmd 总是通过 scmd->eh_entry
链接。


### 1.2 scmd 是如何被完成的？


一旦 LLDD 取得一个 scmd，要么由 LLDD 调用在调用 hostt->queuecommand() 时
从中间层传入的 scsi_done 回调来完成命令，要么由块层将其超时。


##### 1.2.1 用 scsi_done 完成一个 scmd


对于所有非 EH 命令，scsi_done() 是完成回调。它只是调用
blk_mq_complete_request() 来删除块层定时器并触发 BLOCK_SOFTIRQ。

BLOCK_SOFTIRQ 间接调用 scsi_complete()，后者调用 scsi_decide_disposition()
来决定如何处理该命令。scsi_decide_disposition() 查看 scmd->result 值与
sense 数据来决定如何处理该命令。

 - SUCCESS（成功）

	为该命令调用 scsi_finish_command()。该函数做一些维护工作，然后调用
	scsi_io_completion() 来完成 I/O。scsi_io_completion() 通过调用
	blk_end_request 及其相关函数来通知块层该请求已完成，或者在出错时
	弄清楚如何处理剩余的数据。

 - NEEDS_RETRY（需要重试）

 - ADD_TO_MLQUEUE（加入中间层队列）

	scmd 被重新入队到 blk 队列。

 - otherwise（其他情况）

	为该命令调用 scsi_eh_scmd_add(scmd)。该函数的细节参见 [1-3]。


##### 1.2.2 用超时完成一个 scmd


超时处理函数是 scsi_timeout()。当发生超时时，该函数

 1. 调用可选的 hostt->eh_timed_out() 回调。返回值可以是下列之一

    - SCSI_EH_RESET_TIMER（重置定时器）
	表示需要更多时间来完成命令。定时器被重新启动。

    - SCSI_EH_NOT_HANDLED（未处理）
        eh_timed_out() 回调没有处理该命令。采取第 2 步。

    - SCSI_EH_DONE（已完成）
        eh_timed_out() 完成了该命令。

 2. 调用 scsi_abort_command() 来调度一个异步中止，它可能会重试
    scmd->allowed + 1 次。对于已经设置了 SCSI_EH_ABORT_SCHEDULED 标志的命令
    （这表明该命令已经被中止过一次，而这是一次失败的重试）、当重试次数
    用尽时、或当 EH 截止时间已过期时，不会调用异步中止。在这些情况下采取
    第 3 步。

 3. 为该命令调用 scsi_eh_scmd_add(scmd)。更多信息参见 [1-4]。

### 1.3 异步命令中止


 超时发生后，会从 scsi_abort_command() 调度一次命令中止。如果中止成功，
 该命令要么被重试（如果重试次数尚未用尽），要么以 DID_TIME_OUT 终止。

 否则为该命令调用 scsi_eh_scmd_add()。更多信息参见 [1-4]。

### 1.4 EH 如何接管


scmd 通过 scsi_eh_scmd_add() 进入 EH，该函数执行以下操作。

 1. 将 scmd->eh_entry 链接到 shost->eh_cmd_q

 2. 设置 shost->shost_state 中的 SHOST_RECOVERY 位

 3. 递增 shost->host_failed

 4. 当 shost->host_busy == shost->host_failed 时唤醒 SCSI EH 线程

如上所见，一旦有任何 scmd 被加入 shost->eh_cmd_q，SHOST_RECOVERY
shost_state 位就会被打开。这会阻止任何新的 scmd 从 blk 队列下发到主机；
最终，主机上的所有 scmd 要么正常完成，要么失败并被加入 eh_cmd_q，要么
超时并被加入 shost->eh_cmd_q。

如果所有 scmd 都完成或失败，在途 scmd 的数量就会等于失败的 scmd 数量——
即 shost->host_busy == shost->host_failed。这会唤醒 SCSI EH 线程。因此，一旦
被唤醒，SCSI EH 线程可以预期所有在途命令都已失败并链接在 shost->eh_cmd_q 上。

注意，这并不表示底层已经静止。如果 LLDD 以一个错误状态完成了一个 scmd，则
假定 LLDD 与底层在那一刻已经遗忘了该 scmd。然而，如果一个 scmd 超时了，除非
hostt->eh_timed_out() 让底层遗忘了该 scmd（目前没有任何 LLDD 这样做），否则
就底层而言该命令仍然是活跃的，并且随时可能完成。当然，由于定时器已经过期，
所有这些完成都会被忽略。

我们稍后讨论 SCSI EH 如何采取行动来中止——让 LLDD 遗忘——超时的 scmd。


## 2. SCSI EH 如何工作


LLDD 可以通过以下两种方式之一来实现 SCSI EH 动作。

 - Fine-grained EH callbacks（细粒度 EH 回调）
	LLDD 可以实现细粒度的 EH 回调，并让 SCSI 中间层驱动错误处理，
	调用适当的回调。这将在 [2-1] 中进一步讨论。

 - eh_strategy_handler() callback（eh_strategy_handler() 回调）
	这是一个大的回调，应当执行整个错误处理。因此，它应当完成 SCSI
	中间层在恢复期间执行的所有杂务。这将在 [2-2] 中讨论。

一旦恢复完成，SCSI EH 通过调用 scsi_restart_operations() 恢复正常运行，该函数

 1. 检查是否需要锁门并锁门。

 2. 清除 SHOST_RECOVERY shost_state 位

 3. 唤醒在 shost->host_wait 上等待的进程。这发生在有人对主机调用
    scsi_block_when_processing_errors() 时。（**疑问** 为什么需要它？在到达
    blk 队列之后，所有操作无论如何都会被阻塞。）

 4. 踢动主机上所有设备中的队列


### 2.1 EH through fine-grained callbacks（通过细粒度回调的 EH）


##### 2.1.1 概述


如果不存在 eh_strategy_handler()，SCSI 中间层负责驱动错误处理。EH 有两个目标——
让 LLDD、主机与设备遗忘超时的 scmd，并让它们准备好接受新命令。当一个 scmd 被
底层遗忘、且底层准备好再次处理或失败该 scmd 时，称该 scmd 已被恢复。

为了实现这些目标，EH 以递增的严重性执行恢复动作。有些动作通过发出 SCSI 命令
来执行，另一些则通过调用下列细粒度 hostt EH 回调之一来执行。回调可以被省略，
被省略的回调被视为总是失败。

```
    int (* eh_abort_handler)(struct scsi_cmnd *);
    int (* eh_device_reset_handler)(struct scsi_cmnd *);
    int (* eh_bus_reset_handler)(struct scsi_cmnd *);
    int (* eh_host_reset_handler)(struct scsi_cmnd *);

```
严重性更高的动作只有在严重性更低的动作无法恢复部分失败的 scmd 时才会采取。
另请注意，最高严重性动作的失败意味着 EH 失败，并导致所有未恢复的设备被下线。

在恢复期间，遵循以下规则

 - 恢复动作在待办列表 eh_work_q 上失败的 scmd 上执行。如果某个恢复动作对
   一个 scmd 成功，已恢复的 scmd 会从 eh_work_q 中移除。

   注意，对单个 scmd 的一个恢复动作可以恢复多个 scmd。例如，重置一个设备
   会恢复该设备上所有失败的 scmd。

 - 只有当低严重性动作完成后 eh_work_q 非空时，才采取更高严重性的动作。

 - EH 复用失败的 scmd 来发出用于恢复的命令。对于超时的 scmd，SCSI EH 确保在
   复用其进行 EH 命令之前，LLDD 已经遗忘了该 scmd。

当一个 scmd 被恢复时，使用 scsi_eh_finish_cmd() 将其从 eh_work_q 移动到 EH
本地的 eh_done_q。在所有 scmd 都被恢复（eh_work_q 为空）后，调用
scsi_eh_flush_done_q() 来重试或错误完成（向上层通知失败）已恢复的 scmd。

当且仅当其 sdev 仍然在线（未在 EH 期间被下线）、未设置 REQ_FAILFAST、且
++scmd->retries 小于 scmd->allowed 时，scmd 才会被重试。


##### 2.1.2 Flow of scmds through EH（scmd 流经 EH 的过程）


 1. 错误完成 / 超时

    :ACTION: 为该 scmd 调用 scsi_eh_scmd_add()

 - 将 scmd 加入 shost->eh_cmd_q
 - 设置 SHOST_RECOVERY
 - shost->host_failed++

    :LOCKING: shost->host_lock

 2. EH 启动

    :ACTION: 将所有 scmd 移动到 EH 本地的 eh_work_q。shost->eh_cmd_q 被清空。

    :LOCKING: shost->host_lock（并非严格必要，仅为一致性）

 3. scmd 已恢复

    :ACTION: 调用 scsi_eh_finish_cmd() 来 EH-完成该 scmd

 - 从本地 eh_work_q 移动到本地 eh_done_q

    :LOCKING: none（无）

    :CONCURRENCY: 每个独立的 eh_work_q 最多一个线程，以保持队列操作的
		  无锁性

 4. EH 完成

    :ACTION: scsi_eh_flush_done_q() 重试 scmd 或向上层通知失败。可以并发
	    调用，但每个独立的 eh_work_q 必须最多只有一个线程，以无锁方式
	    操作队列

      - scmd 从 eh_done_q 中移除，并清除 scmd->eh_entry
      - 如果需要重试，使用 scsi_queue_insert() 重新入队该 scmd
      - 否则，为该 scmd 调用 scsi_finish_command()
      - 将 shost->host_failed 清零

    :LOCKING: 队列或完成函数执行适当的加锁


##### 2.1.3 Flow of control（控制流）


 通过细粒度回调的 EH 从 scsi_unjam_host() 开始。

`scsi_unjam_host`

    1. 锁定 shost->host_lock，将 shost->eh_cmd_q splice_init 到本地
       eh_work_q，并解锁 host_lock。注意，shost->eh_cmd_q 会被此动作清空。

    2. 调用 scsi_eh_get_sense。

    `scsi_eh_get_sense`

	对于每个没有有效 sense 数据的错误完成命令，会采取此动作。大多数
	SCSI 传输层/LLDD 会在命令失败时自动获取 sense 数据（autosense，
	自动感知）。出于性能原因，以及因为 sense 信息可能在 CHECK CONDITION
	发生与此动作之间失去同步，推荐使用 autosense。

	注意，如果不支持 autosense，当用 scsi_done() 错误完成该 scmd 时，
	scmd->sense_buffer 包含无效的 sense 数据。scsi_decide_disposition()
	在这种情况下总是返回 FAILED，从而调用 SCSI EH。当 scmd 到达此处时，
	会获取 sense 数据并再次调用 scsi_decide_disposition()。

 1. 调用 scsi_request_sense()，它发出 REQUEST_SENSE 命令。如果失败，则不
           采取动作。注意，不采取动作会导致对该 scmd 采取更高严重性的恢复。

 2. 对该 scmd 调用 scsi_decide_disposition()

    - SUCCESS（成功）
		scmd->retries 被设为 scmd->allowed，阻止 scsi_eh_flush_done_q()
		重试该 scmd，并调用 scsi_eh_finish_cmd()。

    - NEEDS_RETRY（需要重试）
		scsi_eh_finish_cmd() 被调用

    - otherwise（其他情况）
		不采取动作。

    4. 如果 !list_empty(&eh_work_q)，调用 scsi_eh_ready_devs()

    `scsi_eh_ready_devs`

	该函数采取四种越来越严厉的措施，使失败的 sdev 准备好接受新命令。

 1. 调用 scsi_eh_stu()

	`scsi_eh_stu`

	    对于每个有失败 scmd 且带有有效 sense 数据、且 scsi_check_sense()
	    的判定为 FAILED 的 sdev，发出 start=1 的 START STOP UNIT 命令。
	    注意，由于我们明确选择了错误完成的 scmd，已知底层已经遗忘了该
	    scmd，因此我们可以复用它来进行 STU。

	    如果 STU 成功且 sdev 处于离线或就绪状态，该 sdev 上所有失败的
	    scmd 都会通过 scsi_eh_finish_cmd() 完成 EH。

	    **注意** 如果未实现 hostt->eh_abort_handler() 或它失败，此时我们
	    可能仍有超时的 scmd，而 STU 并不能让底层遗忘那些 scmd。然而，如果
	    STU 成功，该函数会完成该 sdev 上所有 scmd 的 EH，使底层处于不一致
	    的状态。似乎 STU 动作只应在某个 sdev 没有超时 scmd 时才应采取。

 2. 如果 !list_empty(&eh_work_q)，调用 scsi_eh_bus_device_reset()。

	`scsi_eh_bus_device_reset`

	    此动作与 scsi_eh_stu() 非常相似，只是它使用
	    hostt->eh_device_reset_handler() 而不是发出 STU。此外，由于我们不
	    发出 SCSI 命令，且重置会清除该 sdev 上的所有 scmd，因此无需挑选
	    错误完成的 scmd。

 3. 如果 !list_empty(&eh_work_q)，调用 scsi_eh_bus_reset()。

	`scsi_eh_bus_reset`

	    hostt->eh_bus_reset_handler() 对每个有失败 scmd 的通道调用。如果
	    总线重置成功，该通道上所有就绪或离线的 sdev 上失败的 scmd 都会
	    完成 EH。

 4. 如果 !list_empty(&eh_work_q)，调用 scsi_eh_host_reset()。

	`scsi_eh_host_reset`

	    这是最后手段。调用 hostt->eh_host_reset_handler()。如果主机重置
	    成功，该主机上所有就绪或离线的 sdev 上失败的 scmd 都会完成 EH。

 5. 如果 !list_empty(&eh_work_q)，调用 scsi_eh_offline_sdevs()。

	`scsi_eh_offline_sdevs`

	    将所有仍有未恢复 scmd 的 sdev 下线，并完成这些 scmd 的 EH。

    5. 调用 scsi_eh_flush_done_q()。

	`scsi_eh_flush_done_q`

	    此时所有 scmd 都已恢复（或放弃），并由 scsi_eh_finish_cmd() 放到了
	    eh_done_q 上。该函数通过重试或向上层通知 scmd 失败来刷新
	    eh_done_q。


### 2.2 EH through transportt->eh_strategy_handler()（通过 transportt->eh_strategy_handler() 的 EH）


transportt->eh_strategy_handler() 在 scsi_unjam_host() 的位置被调用，它负责
整个恢复过程。在完成后，该处理程序应当已经让底层遗忘了所有失败的 scmd，并且
要么准备好接受新命令，要么已下线。此外，它应当执行 SCSI EH 维护杂务以维护
SCSI 中间层的完整性。换言之，在 [2-1-2] 描述的步骤中，除了第 1 步之外的所有
步骤都必须由 eh_strategy_handler() 实现。


##### 2.2.1 Pre transportt->eh_strategy_handler() SCSI midlayer conditions（transportt->eh_strategy_handler() 之前的 SCSI 中间层条件）


 进入处理程序时，以下条件为真。

 - 每个失败 scmd 的 eh_flags 字段被适当设置。

 - 每个失败的 scmd 通过 scmd->eh_entry 链接在 scmd->eh_cmd_q 上。

 - SHOST_RECOVERY 已设置。

 - shost->host_failed == shost->host_busy


##### 2.2.2 Post transportt->eh_strategy_handler() SCSI midlayer conditions（transportt->eh_strategy_handler() 之后的 SCSI 中间层条件）


 退出处理程序时，以下条件必须为真。

 - shost->host_failed 为零。

 - shost->eh_cmd_q 已清空。

 - 每个 scmd->eh_entry 已清空。

 - 对每个 scmd 都调用了 scsi_queue_insert() 或 scsi_finish_command()。注意，
   处理程序可自由使用 scmd->retries 与 ->allowed 来限制重试次数。


##### 2.2.3 Things to consider（需要考虑的事项）


 - 要知道超时的 scmd 在底层仍然是活跃的。在对那些 scmd 做任何其他事情之前，
   先让底层遗忘它们。

 - 为保持一致，在访问/修改 shost 数据结构时，获取 shost->host_lock。

 - 在完成后，每个失败的 sdev 必须已经遗忘了所有活跃的 scmd。

 - 在完成后，每个失败的 sdev 必须准备好接受新命令或已下线。


Tejun Heo
htejun@gmail.com

2005 年 9 月 11 日
