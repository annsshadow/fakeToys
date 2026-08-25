
## SCSI EH（SCSI 错误处理

本文档描述了 SCSI 中间层（midlayer）的错误处理基础设施有关 SCSI 中间层的更多信息，请参阅 Documentation/scsi/scsi_mid_low_api.rst

   [^1^] SCSI 命令如何穿过中间层并进入 EH
       [1-1] struct scsi_cmnd
       [1-2] scmd 是如何被完成的？
   	[1-2-1] scsi_done 完成一scmd
   	[1-2-2] 用超时完成一scmd
       [1-3] EH 如何接管
   [^2^] SCSI EH 如何工作
       [2-1] 通过细粒度回调的 EH
   	[2-1-1] 概述
   	[2-1-2] scmd 流经 EH 的过   	[2-1-3] 控制       [2-2] 通过 transportt->eh_strategy_handler() EH
   	[2-2-1] transportt->eh_strategy_handler() 之前SCSI 中间层条   	[2-2-2] transportt->eh_strategy_handler() 之后SCSI 中间层条   	[2-2-3] 需要考虑的事

## 1. SCSI 命令如何穿过中间层并进入 EH


### 1.1 struct scsi_cmnd


每个 SCSI 命令都用 struct scsi_cmnd（即 scmd）表示。一scmd 有两list_head 将自己链接进链表。这两个分别scmd->list scmd->eh_entry前者用于空闲链表或每设备分配的 scmd 链表，在本次 EH 讨论中并不重要。后用于完成EH 链表，除非另有说明，本讨论中 scmd 总是通过 scmd->eh_entry
链接

### 1.2 scmd 是如何被完成的？


一LLDD 取得一scmd，要么由 LLDD 调用在调hostt->queuecommand() 从中间层传入scsi_done 回调来完成命令，要么由块层将其超时

##### 1.2.1 scsi_done 完成一scmd


对于所有非 EH 命令，scsi_done() 是完成回调。它只是调用
blk_mq_complete_request() 来删除块层定时器并触BLOCK_SOFTIRQ
BLOCK_SOFTIRQ 间接调用 scsi_complete()，后者调scsi_decide_disposition()
来决定如何处理该命令。scsi_decide_disposition() 查看 scmd->result 值与
sense 数据来决定如何处理该命令
 - SUCCESS（成功）

	为该命令调用 scsi_finish_command()。该函数做一些维护工作，然后调用
	scsi_io_completion() 来完I/O。scsi_io_completion() 通过调用
	blk_end_request 及其相关函数来通知块层该请求已完成，或者在出错	弄清楚如何处理剩余的数据
 - NEEDS_RETRY（需要重试）

 - ADD_TO_MLQUEUE（加入中间层队列
	scmd 被重新入队到 blk 队列
 - otherwise（其他情况）

	为该命令调用 scsi_eh_scmd_add(scmd)。该函数的细节参[1-3]

##### 1.2.2 用超时完成一scmd


超时处理函数scsi_timeout()。当发生超时时，该函
 1. 调用可选的 hostt->eh_timed_out() 回调。返回值可以是下列之一

    - SCSI_EH_RESET_TIMER（重置定时器	表示需要更多时间来完成命令。定时器被重新启动
    - SCSI_EH_NOT_HANDLED（未处理        eh_timed_out() 回调没有处理该命令。采取第 2 步
    - SCSI_EH_DONE（已完成        eh_timed_out() 完成了该命令
 2. 调用 scsi_abort_command() 来调度一个异步中止，它可能会重试
    scmd->allowed + 1 次。对于已经设置了 SCSI_EH_ABORT_SCHEDULED 标志的命    （这表明该命令已经被中止过一次，而这是一次失败的重试）、当重试次数
    用尽时、或EH 截止时间已过期时，不会调用异步中止。在这些情况下采    3 步
 3. 为该命令调用 scsi_eh_scmd_add(scmd)。更多信息参[1-4]
### 1.3 异步命令中止


 超时发生后，会从 scsi_abort_command() 调度一次命令中止。如果中止成功，
 该命令要么被重试（如果重试次数尚未用尽），要么以 DID_TIME_OUT 终止
 否则为该命令调用 scsi_eh_scmd_add()。更多信息参[1-4]
### 1.4 EH 如何接管


scmd 通过 scsi_eh_scmd_add() 进入 EH，该函数执行以下操作
 1. scmd->eh_entry 链接shost->eh_cmd_q

 2. 设置 shost->shost_state 中的 SHOST_RECOVERY 
 3. 递增 shost->host_failed

 4. 褰?shost->host_busy == shost->host_failed 鏃跺敜閱?SCSI EH 绾跨▼

如上所见，一旦有任何 scmd 被加shost->eh_cmd_q，SHOST_RECOVERY
shost_state 位就会被打开。这会阻止任何新scmd blk 队列下发到主机；
最终，主机上的所scmd 要么正常完成，要么失败并被加eh_cmd_q，要超时并被加入 shost->eh_cmd_q
如果所scmd 都完成或失败，在scmd 的数量就会等于失败的 scmd 数量—shost->host_busy == shost->host_failed。这会唤SCSI EH 线程。因此，一被唤醒，SCSI EH 线程可以预期所有在途命令都已失败并链接shost->eh_cmd_q 上
注意，这并不表示底层已经静止。如LLDD 以一个错误状态完成了一scmd，则
假定 LLDD 与底层在那一刻已经遗忘了scmd。然而，如果一scmd 超时了，除非
hostt->eh_timed_out() 让底层遗忘了scmd（目前没有任LLDD 这样做），否就底层而言该命令仍然是活跃的，并且随时可能完成。当然，由于定时器已经过期，
所有这些完成都会被忽略
我们稍后讨论 SCSI EH 如何采取行动来中止——让 LLDD 遗忘——超时的 scmd

## 2. SCSI EH 如何工作


LLDD 可以通过以下两种方式之一来实SCSI EH 动作
 - Fine-grained EH callbacks（细粒度 EH 回调	LLDD 可以实现细粒度的 EH 回调，并SCSI 中间层驱动错误处理，
	调用适当的回调。这将在 [2-1] 中进一步讨论
 - eh_strategy_handler() callback（eh_strategy_handler() 回调	这是一个大的回调，应当执行整个错误处理。因此，它应当完SCSI
	中间层在恢复期间执行的所有杂务。这将在 [2-2] 中讨论
一旦恢复完成，SCSI EH 通过调用 scsi_restart_operations() 恢复正常运行，该函数

 1. 检查是否需要锁门并锁门
 2. 清除 SHOST_RECOVERY shost_state 
 3. 唤醒shost->host_wait 上等待的进程。这发生在有人对主机调用
    scsi_block_when_processing_errors() 时。（**疑问** 为什么需要它？在到达
    blk 队列之后，所有操作无论如何都会被阻塞。）

 4. 踢动主机上所有设备中的队

### 2.1 EH through fine-grained callbacks（通过细粒度回调的 EH

##### 2.1.1 概述


如果不存eh_strategy_handler()，SCSI 中间层负责驱动错误处理。EH 有两个目标—LLDD、主机与设备遗忘超时scmd，并让它们准备好接受新命令。当一scmd 底层遗忘、且底层准备好再次处理或失败scmd 时，称该 scmd 已被恢复
为了实现这些目标，EH 以递增的严重性执行恢复动作。有些动作通过发出 SCSI 命令
来执行，另一些则通过调用下列细粒hostt EH 回调之一来执行。回调可以被省略被省略的回调被视为总是失败
```
    int (* eh_abort_handler)(struct scsi_cmnd *);
    int (* eh_device_reset_handler)(struct scsi_cmnd *);
    int (* eh_bus_reset_handler)(struct scsi_cmnd *);
    int (* eh_host_reset_handler)(struct scsi_cmnd *);

```
严重性更高的动作只有在严重性更低的动作无法恢复部分失败scmd 时才会采取另请注意，最高严重性动作的失败意味着 EH 失败，并导致所有未恢复的设备被下线
在恢复期间，遵循以下规则

 - 恢复动作在待办列eh_work_q 上失败的 scmd 上执行。如果某个恢复动作对
   一scmd 成功，已恢复scmd 会从 eh_work_q 中移除
   注意，对单个 scmd 的一个恢复动作可以恢复多scmd。例如，重置一个设   会恢复该设备上所有失败的 scmd
 - 只有当低严重性动作完成后 eh_work_q 非空时，才采取更高严重性的动作
 - EH 复用失败scmd 来发出用于恢复的命令。对于超时的 scmd，SCSI EH 确保   复用其进EH 命令之前，LLDD 已经遗忘了该 scmd
当一scmd 被恢复时，使scsi_eh_finish_cmd() 将其eh_work_q 移动EH
本地eh_done_q。在所scmd 都被恢复（eh_work_q 为空）后，调scsi_eh_flush_done_q() 来重试或错误完成（向上层通知失败）已恢复scmd
当且仅当sdev 仍然在线（未EH 期间被下线）、未设置 REQ_FAILFAST、且
++scmd->retries 小于 scmd->allowed 时，scmd 才会被重试

##### 2.1.2 Flow of scmds through EH（scmd 流经 EH 的过程）


 1. 错误完成 / 超时

    :ACTION: 为该 scmd 调用 scsi_eh_scmd_add()

 - 灏?scmd 鍔犲叆 shost->eh_cmd_q
 - 设置 SHOST_RECOVERY
 - shost->host_failed++

    :LOCKING: shost->host_lock

 2. EH 启动

    :ACTION: 将所scmd 移动EH 本地eh_work_q。shost->eh_cmd_q 被清空
    :LOCKING: shost->host_lock（并非严格必要，仅为一致性）

 3. scmd 宸叉仮澶。
    :ACTION: 调用 scsi_eh_finish_cmd() EH-完成scmd

 - 从本eh_work_q 移动到本eh_done_q

    :LOCKING: none（无
    :CONCURRENCY: 每个独立eh_work_q 最多一个线程，以保持队列操作的
		  鏃犻攣鎬。
 4. EH 完成

    :ACTION: scsi_eh_flush_done_q() 重试 scmd 或向上层通知失败。可以并	    调用，但每个独立eh_work_q 必须最多只有一个线程，以无锁方	    操作队列

      - scmd eh_done_q 中移除，并清scmd->eh_entry
      - 如果需要重试，使用 scsi_queue_insert() 重新入队scmd
      - 否则，为scmd 调用 scsi_finish_command()
      - shost->host_failed 清零

    :LOCKING: 队列或完成函数执行适当的加

##### 2.1.3 Flow of control（控制流

 通过细粒度回调的 EH scsi_unjam_host() 开始
`scsi_unjam_host`

    1. 锁定 shost->host_lock，将 shost->eh_cmd_q splice_init 到本       eh_work_q，并解锁 host_lock。注意，shost->eh_cmd_q 会被此动作清空
    2. 调用 scsi_eh_get_sense
    `scsi_eh_get_sense`

	对于每个没有有效 sense 数据的错误完成命令，会采取此动作。大多数
	SCSI 传输LLDD 会在命令失败时自动获sense 数据（autosense	自动感知）。出于性能原因，以及因sense 信息可能CHECK CONDITION
	发生与此动作之间失去同步，推荐使autosense
	注意，如果不支持 autosense，当scsi_done() 错误完成scmd 时，
	scmd->sense_buffer 包含无效sense 数据。scsi_decide_disposition()
	在这种情况下总是返回 FAILED，从而调SCSI EH。当 scmd 到达此处时，
	会获sense 数据并再次调scsi_decide_disposition()
 1. 调用 scsi_request_sense()，它发出 REQUEST_SENSE 命令。如果失败，则不
           采取动作。注意，不采取动作会导致对该 scmd 采取更高严重性的恢复
 2. 对该 scmd 调用 scsi_decide_disposition()

    - SUCCESS（成功）
		scmd->retries 被设scmd->allowed，阻scsi_eh_flush_done_q()
		重试scmd，并调用 scsi_eh_finish_cmd()
    - NEEDS_RETRY（需要重试）
		scsi_eh_finish_cmd() 被调
    - otherwise（其他情况）
		不采取动作
    4. 如果 !list_empty(&eh_work_q)，调scsi_eh_ready_devs()

    `scsi_eh_ready_devs`

	该函数采取四种越来越严厉的措施，使失败的 sdev 准备好接受新命令
 1. 调用 scsi_eh_stu()

	`scsi_eh_stu`

	    对于每个有失scmd 且带有有sense 数据、且 scsi_check_sense()
	    的判定为 FAILED sdev，发start=1 START STOP UNIT 命令	    注意，由于我们明确选择了错误完成的 scmd，已知底层已经遗忘了	    scmd，因此我们可以复用它来进STU
	    如果 STU 成功sdev 处于离线或就绪状态，sdev 上所有失败的
	    scmd 都会通过 scsi_eh_finish_cmd() 完成 EH
	    **注意** 如果未实hostt->eh_abort_handler() 或它失败，此时我	    可能仍有超时scmd，STU 并不能让底层遗忘那些 scmd。然而，如果
	    STU 成功，该函数会完成该 sdev 上所scmd EH，使底层处于不一	    的状态。似STU 动作只应在某sdev 没有超时 scmd 时才应采取
 2. 如果 !list_empty(&eh_work_q)，调scsi_eh_bus_device_reset()
	`scsi_eh_bus_device_reset`

	    此动作与 scsi_eh_stu() 非常相似，只是它使用
	    hostt->eh_device_reset_handler() 而不是发STU。此外，由于我们	    发出 SCSI 命令，且重置会清除该 sdev 上的所scmd，因此无需挑	    错误完成scmd
 3. 如果 !list_empty(&eh_work_q)，调scsi_eh_bus_reset()
	`scsi_eh_bus_reset`

	    hostt->eh_bus_reset_handler() 对每个有失败 scmd 的通道调用。如	    总线重置成功，该通道上所有就绪或离线sdev 上失败的 scmd 都会
	    完成 EH
 4. 如果 !list_empty(&eh_work_q)，调scsi_eh_host_reset()
	`scsi_eh_host_reset`

	    这是最后手段。调hostt->eh_host_reset_handler()。如果主机重	    成功，该主机上所有就绪或离线sdev 上失败的 scmd 都会完成 EH
 5. 如果 !list_empty(&eh_work_q)，调scsi_eh_offline_sdevs()
	`scsi_eh_offline_sdevs`

	    将所有仍有未恢复 scmd sdev 下线，并完成这些 scmd EH
    5. 调用 scsi_eh_flush_done_q()
	`scsi_eh_flush_done_q`

	    此时所scmd 都已恢复（或放弃），并由 scsi_eh_finish_cmd() 放到	    eh_done_q 上。该函数通过重试或向上层通知 scmd 失败来刷	    eh_done_q

### 2.2 EH through transportt->eh_strategy_handler()（通过 transportt->eh_strategy_handler() EH

transportt->eh_strategy_handler() scsi_unjam_host() 的位置被调用，它负责
整个恢复过程。在完成后，该处理程序应当已经让底层遗忘了所有失败的 scmd，并要么准备好接受新命令，要么已下线。此外，它应当执SCSI EH 维护杂务以维SCSI 中间层的完整性。换言之，[2-1-2] 描述的步骤中，除了第 1 步之外的所步骤都必须由 eh_strategy_handler() 实现

##### 2.2.1 Pre transportt->eh_strategy_handler() SCSI midlayer conditions（transportt->eh_strategy_handler() 之前SCSI 中间层条件）


 进入处理程序时，以下条件为真
 - 每个失败 scmd eh_flags 字段被适当设置
 - 每个失败scmd 通过 scmd->eh_entry 链接scmd->eh_cmd_q 上
 - SHOST_RECOVERY 已设置
 - shost->host_failed == shost->host_busy


##### 2.2.2 Post transportt->eh_strategy_handler() SCSI midlayer conditions（transportt->eh_strategy_handler() 之后SCSI 中间层条件）


 退出处理程序时，以下条件必须为真
 - shost->host_failed 为零
 - shost->eh_cmd_q 已清空
 - 每个 scmd->eh_entry 已清空
 - 对每scmd 都调用了 scsi_queue_insert() scsi_finish_command()。注意，
   处理程序可自由使scmd->retries ->allowed 来限制重试次数

##### 2.2.3 Things to consider（需要考虑的事项）


 - 要知道超时的 scmd 在底层仍然是活跃的。在对那scmd 做任何其他事情之前，
   先让底层遗忘它们
 - 为保持一致，在访修改 shost 数据结构时，获取 shost->host_lock
 - 在完成后，每个失败的 sdev 必须已经遗忘了所有活跃的 scmd
 - 在完成后，每个失败的 sdev 必须准备好接受新命令或已下线

Tejun Heo
htejun@gmail.com

2005 骞?9 鏈?11 鏃。