
## KVM VCPU 请求（KVM VCPU Requests

## 概述


KVM 支持一个内API，使线程能够请求某个 VCPU 线程执行某些活动。例如，一个线程可以请求某VCPU 刷新
```

  /* 检VCPU @vcpu 是否有任何待处理请求*/
  bool kvm_request_pending(struct kvm_vcpu *vcpu);

  /* 检VCPU @vcpu 是否有请@req 待处理*/
  bool kvm_test_request(int req, struct kvm_vcpu *vcpu);

  /* 清除 VCPU @vcpu 的请@req*/
  void kvm_clear_request(int req, struct kvm_vcpu *vcpu);

  /*
   * 检VCPU @vcpu 是否有请@req 待处理。当有请求待处理时，
   * 它将被清除，并发出一个内存屏障（memory barrier），该屏障与
   * kvm_make_request() 中的另一个屏障配对   */
  bool kvm_check_request(int req, struct kvm_vcpu *vcpu);

  /*
   * VCPU @vcpu 发出请求 @req。发出一个内存屏障，该屏障与
   * kvm_check_request() 中的另一个屏障配对，然后再设置请求   */
  void kvm_make_request(int req, struct kvm_vcpu *vcpu);

  /* struct kvm @kvm 所表示VM 的所VCPU 发出请求 @req*/
  bool kvm_make_all_cpus_request(struct kvm *kvm, unsigned int req);

```
通常，请求方希望 VCPU 在发出请求后尽快执行该活动。这意味着大多数请求（kvm_make_request() 调用）之后会跟随一次对 kvm_vcpu_kick() 的调用，kvm_make_all_cpus_request() 已经将踢醒（kick）所VCPU 的操作内建其中

### VCPU 踢醒（VCPU Kicks

VCPU 踢醒的目标是使一VCPU 线程退出客户机（guest）模式，以便执行某些 KVM 维护工作。为此，会发送一IPI，强制客户机模式退出。然而，VCPU 线程在踢醒时可能并不处于客户机模式。因此，根据 VCPU 线程的模式和状态，踢醒还可能采取另外两种动作。以下列出全部三种动作：

1) 发送一IPI。这会强制退出客户机模式2) 唤醒一个睡眠中VCPU。睡眠中VCPU 是处于客户机模式之外、在等待队列（waitqueue）上等待VCPU 线程。唤醒它们会将线程从等待队列移除，使线程能够再次运行。此行为可能被抑制，参见下文KVM_REQUEST_NO_WAKEUP3) 什么都不做。当 VCPU 不处于客户机模式VCPU 线程没有睡眠时，则无事可做

### VCPU 模式（VCPU Mode

VCPU 有一个模式状`vcpu->mode`，用于跟踪客户机是否正在客户机模式下运行，以及一些特定的客户机模式之外的状态。架构层可以使用 `vcpu->mode` 来确VCPU 请求VCPU 看到（参确保请求被看），以及避免发送不必要IPI（参IPI 精简"），甚至确保等待 IPI 确认（参等待确认"）。定义了以下模式
OUTSIDE_GUEST_MODE

  VCPU 线程处于客户机模式之外
IN_GUEST_MODE

  VCPU 线程处于客户机模式之中
EXITING_GUEST_MODE

  VCPU 线程正从 IN_GUEST_MODE 过渡OUTSIDE_GUEST_MODE
READING_SHADOW_PAGE_TABLES

  VCPU 线程处于客户机模式之外，但它希望某些 VCPU 请求（即 KVM_REQ_TLB_FLUSH）的发送方等待，直VCPU 线程完成页表读取

## VCPU 请求内部机制


VCPU 请求仅仅`vcpu->requests` 位图中的位索引。这意味着通用的位操作（bitop），例如 [atomic-ops]_ 中记录的那些，可用于
```

  clear_bit(KVM_REQ_UNBLOCK & KVM_REQUEST_MASK, &vcpu->requests);

```
不过，VCPU 请求的使用者应当避免这样做，因为那会破坏抽象。前 8 位保留给与架构无关的请求；所有额外的位可供与架构相关的请求使用

### 与架构无关的请求


KVM_REQ_TLB_FLUSH

  KVM 通用MMU notifier 可能需要刷新客户机所有的 TLB 项，调用 kvm_flush_remote_tlbs() 来完成。选择使用通用 kvm_flush_remote_tlbs() 实现的架构需要处理此 VCPU 请求
KVM_REQ_VM_DEAD

  此请求通知所VCPU VM 已死亡且不可用，例如由于致命错误VM 的状态被有意销毁
KVM_REQ_UNBLOCK

  此请求通知 vCPU 退kvm_vcpu_block。例如，它用于代vCPU 在主机上运行的定时器处理程序，或者用于更新中断路由并确保已分配的（assigned）设备能够唤vCPU
KVM_REQ_OUTSIDE_GUEST_MODE

  请求"确保目标 vCPU 在请求发送方继续执行之前已经退出客户机模式。目标无需采取任何动作，因此实际上不会为目标记录任何请求。此请求类似踢醒（kick，但与踢醒不同的是，它保vCPU 确实已经退出客户机模式。踢醒只保证 vCPU 会在将来的某个时刻退出，例如之前的踢醒可能已经启动了该过程，但无法保证即将被踢醒vCPU 已经完全退出客户机模式

### KVM_REQUEST_MASK


在使用位操作处理 VCPU 请求之前，应当先KVM_REQUEST_MASK 对其掩码。这是因为只有低 8 位用于表示请求编号。高位用作标志。目前只定义了两个标志

### VCPU 请求标志


KVM_REQUEST_NO_WAKEUP

  此标志应用于只需要处于客户机模式VCPU 立即关注的请求。也就是说，睡眠中的 VCPU 不需要为这些请求而被唤醒。睡眠中VCPU 会在稍后由于其他原因被唤醒时处理这些请求
KVM_REQUEST_WAIT

  当带有此标志的请求通过 kvm_make_all_cpus_request() 发出时，调用方将等待每个 VCPU 确认IPI 后再继续。此标志只适用于会收到 IPI VCPU。例如，如果 VCPU 正在睡眠，因此不需IPI，那么请求线程就不会等待。这意味着此标志可以安全地KVM_REQUEST_NO_WAKEUP 组合使用。有关带KVM_REQUEST_WAIT 的请求的更多信息，请参阅"等待确认"

## 带有相关状态的 VCPU 请求


请求方希望接VCPU 处理新状态的话，需要确保在接收 VCPU 线程CPU 观察到该请求时，新写入的状态对其可见。这意味着必须在写入新状态之后、设VCPU 请求位之前插入一个写内存屏障（write memory barrier）。此外，在接VCPU 线程一侧，必须在读取请求位之后、继续读取与之关联的新状态之前，插入一个相应的读屏障（read barrier）。请参阅 [lwn-mb]_ 的场3（消息与标志），以及内核文档 [memory-barriers]_
kvm_check_request() kvm_make_request() 这一对函数提供了内存屏障，使得该要求可由 API 在内部处理

## 确保请求被看

在向 VCPU 发出请求时，我们希望避免接收 VCPU 在客户机模式下执行任意长时间而不处理该请求。只要确VCPU 线程在进入客户机模式之前检kvm_request_pending()，并且在必要时踢醒会发IPI 以强制退出客户机模式，我们就可以确信这种情况不会发生。必须格外小心，以覆VCPU 线程最后一kvm_request_pending() 检查之后、到它进入客户机模式之前的这段时间，因为踢醒 IPI 只会对处于客户机模式VCPU 线程、或至少已经禁用中断以准备进入客户机模式VCPU 线程触发客户机模式退出。这意味着一个优化实现（参见"IPI 精简"）必须确定何时不发IPI 是安全的。一个除 s390 之外的所有架构都采用的解决方案是
- 在禁用中断和最后一kvm_request_pending() 检查之间，`vcpu->mode` 设置IN_GUEST_MODE- 在进入客户机时原子地启用中断
此解决方案还需要在请求线程和接VCPU 中谨慎放置内存屏障。借助内存屏障，我们可以排除这样一种可能性：即一VCPU 线程在最后一次检查中观察!kvm_request_pending()，然后却没有收到针对紧接着该检查之后发出的下一个请求的 IPI。这是通过 Dekker 内存屏障模式（[lwn-mb]_ 的场10）实现的。由Dekker 模式需要两个变量，此方案将 `vcpu->mode` `vcpu->requests` 配对。代```

  CPU1                                    CPU2
  =================                       =================
  local_irq_disable();
  WRITE_ONCE(vcpu->mode, IN_GUEST_MODE);  kvm_make_request(REQ, vcpu);
  smp_mb();                               smp_mb();
  if (kvm_request_pending(vcpu)) {        if (READ_ONCE(vcpu->mode) ==
                                              IN_GUEST_MODE) {
      ...abort guest entry...                 ...send IPI...
  }                                       }

```
如上所述，IPI 只对处于客户机模式或已经禁用中断VCPU 线程有用。这就是为什Dekker 模式的这种特定情形被扩展为在`vcpu->mode` 设置IN_GUEST_MODE 之前先禁用中断。使WRITE_ONCE() READ_ONCE() 是为了严谨地实现内存屏障模式，保证编译器不会干扰 `vcpu->mode` 被精心安排的访问

### IPI 精简（IPI Reduction

由于只需要一IPI 即可VCPU 检查任所有请求，因此这些 IPI 可以被合并。这很容易做到：让第一次发IPI 的踢醒同时将 VCPU 模式改为IN_GUEST_MODE 的某种状态。过渡状EXITING_GUEST_MODE 就是为此目的而使用的

### 等待确认（Waiting for Acknowledgements

某些请求（即带有 KVM_REQUEST_WAIT 标志的请求）需要发IPI，并且需要等待确认，即使目标 VCPU 线程处于 IN_GUEST_MODE 之外的模式。例如，一个情形是目标 VCPU 线程处于 READING_SHADOW_PAGE_TABLES 模式，该模式是在禁用中断后设置的。为了支持这些情形，KVM_REQUEST_WAIT 标志将发IPI 的条件从检VCPU 是否处于 IN_GUEST_MODE 改为检查它是否不处OUTSIDE_GUEST_MODE

### 无请求的 VCPU 踢醒（Request-less VCPU Kicks

由于是否发IPI 取决于双变量 Dekker 内存屏障模式，因此很明显，无请求VCPU 踢醒几乎永远是不正确的。如果没IPI 产生的踢醒仍会导致接VCPU 采取动作"的保证（正如最终的 kvm_request_pending() 检查对于有请求伴随的踢醒所做的那样），那么该踢醒可能根本不会做任何有用的事情。例如，如果对一个刚刚要将自身模式设置为 IN_GUEST_MODE VCPU 发出无请求踢醒（意味着不会发IPI），那么VCPU 线程可能会继续其进入过程，而实际上并未执行该踢醒本应启动的任何操作
一个例外是 x86 posted interrupt 机制。不过，即便在这种情形下，即便是无请求的 VCPU 踢醒，也与上述相同的 local_irq_disable() + smp_mb() 模式相耦合；posted interrupt 描述符中ON 位（Outstanding Notification）扮演了 `vcpu->requests` 的角色。发posted interrupt 时，在读`vcpu->mode` 之前设置 PIR.ON；而在 VCPU 线程中，vmx_sync_pir_to_irr() 在将 `vcpu->mode` 设置IN_GUEST_MODE 之后读取 PIR

## 其他考虑


### 睡眠中的 VCPU


VCPU 线程可能需要在调用可能使其睡眠的函数（例如 kvm_vcpu_block()）之前和/或之后考虑请求。它们是否这样做，以及如果这样做的话需要考虑哪些请求，取决于架构。kvm_vcpu_block() 调用 kvm_arch_vcpu_runnable() 来检查是否应该唤醒。这样做的一个原因是为架构提供一个函数，以便在必要时检查请求

## 参考资
