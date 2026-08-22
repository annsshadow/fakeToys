
## KVM 专用MSR


:Author: Glauber Costa <glommer@redhat.com>, Red Hat Inc, 2010

KVM 使用一些自定义MSR 来处理某些请求

自定MSR 有一个为其保留的区间，范围从 0x4b564d00 0x4b564dff。在此区间之外也存在一MSR，但它们已被弃用，不建议使用

### 自定MSR 列表


当前支持的自定义 MSR 列表如下

MSR_KVM_WALL_CLOCK_NEW:
	0x4b564d00

data:
	一个内存区域的 4 字节对齐物理地址，该区域必须位于
	客户RAM 中。该内存预期用于保存如下内容的副
```

	 struct pvclock_wall_clock {
		u32   version;
		u32   sec;
		u32   nsec;
	  } __attribute__((__packed__));

	whose data will be filled in by the hypervisor. The hypervisor is only
	guaranteed to update this data at the moment of MSR write.
	Users that want to reliably query this information more than once have
	to write more than once to this MSR. Fields have the following meanings:

	version:
		guest has to check version before and after grabbing
		time information and check that they are both equal and even.
		An odd version indicates an in-progress update.

	sec:
		 number of seconds for wallclock at time of boot.

	nsec:
		 number of nanoseconds for wallclock at time of boot.

	In order to get the current wallclock time, the system_time from
	MSR_KVM_SYSTEM_TIME_NEW needs to be added.

	Note that although MSRs are per-CPU entities, the effect of this
	particular MSR is global.

	Availability of this MSR must be checked via bit 3 in 0x4000001 cpuid
	leaf prior to usage.

```
MSR_KVM_SYSTEM_TIME_NEW:
	0x4b564d01

data:
	一个内存区域的 4 字节对齐物理地址，该区域必须位于客户RAM 中，
	外加 bit 0 中的一个使能位。该内存预期用于保存
```

	  struct pvclock_vcpu_time_info {
		u32   version;
		u32   pad0;
		u64   tsc_timestamp;
		u64   system_time;
		u32   tsc_to_system_mul;
		s8    tsc_shift;
		u8    flags;
		u8    pad[2];
	  } __attribute__((__packed__)); /* 32 bytes */

	whose data will be filled in by the hypervisor periodically. Only one
	write, or registration, is needed for each VCPU. The interval between
	updates of this structure is arbitrary and implementation-dependent.
	The hypervisor may update this structure at any time it sees fit until
	anything with bit0 == 0 is written to it.

	Fields have the following meanings:

	version:
		guest has to check version before and after grabbing
		time information and check that they are both equal and even.
		An odd version indicates an in-progress update.

	tsc_timestamp:
		the tsc value at the current VCPU at the time
		of the update of this structure. Guests can subtract this value
		from current tsc to derive a notion of elapsed time since the
		structure update.

	system_time:
		a host notion of monotonic time, including sleep
		time at the time this structure was last updated. Unit is
		nanoseconds.

	tsc_to_system_mul:
		multiplier to be used when converting
		tsc-related quantity to nanoseconds

	tsc_shift:
		shift to be used when converting tsc-related
		quantity to nanoseconds. This shift will ensure that
		multiplication with tsc_to_system_mul does not overflow.
		A positive value denotes a left shift, a negative value
		a right shift.

		The conversion from tsc to nanoseconds involves an additional
		right shift by 32 bits. With this information, guests can
		derive per-CPU time by doing::

			time = (current_tsc - tsc_timestamp)
			if (tsc_shift >= 0)
				time <<= tsc_shift;
			else
				time >>= -tsc_shift;
			time = (time * tsc_to_system_mul) >> 32
			time = time + system_time

	flags:
		bits in this field indicate extended capabilities
		coordinated between the guest and the hypervisor. Availability
		of specific flags has to be checked in 0x40000001 cpuid leaf.
		Current flags are:


		+-----------+--------------+----------------------------------+
		| flag bit  | cpuid bit    | meaning			      |
		+-----------+--------------+----------------------------------+
		|	    |		   | time measures taken across       |
		|    0      |	   24      | multiple cpus are guaranteed to  |
		|	    |		   | be monotonic		      |
		+-----------+--------------+----------------------------------+
		|	    |		   | guest vcpu has been paused by    |
		|    1	    |	  N/A	   | the host			      |
		|	    |		   | See 4.70 in api.txt	      |
		+-----------+--------------+----------------------------------+

	Availability of this MSR must be checked via bit 3 in 0x4000001 cpuid
	leaf prior to usage.


```
MSR_KVM_WALL_CLOCK:
	0x11

data and functioning:
	MSR_KVM_WALL_CLOCK_NEW 相同。请改用后者

	MSR 不在保留KVM 区间内，未来可能会被移除
	它的使用已被弃用

	使用前必须通过 0x4000001 cpuid 叶子中的 bit 0 检查该 MSR 是否可用

MSR_KVM_SYSTEM_TIME:
	0x12

data and functioning:
	MSR_KVM_SYSTEM_TIME_NEW 相同。请改用后者

	MSR 不在保留KVM 区间内，未来可能会被移除
	它的使用已被弃用

	使用前必须通过 0x4000001 cpuid 叶子中的 bit 0 检查该 MSR 是否可用

```

		if (!kvm_para_available())    /* refer to cpuid.txt */
			return NON_PRESENT;

		flags = cpuid_eax(0x40000001);
		if (flags & 3) {
			msr_kvm_system_time = MSR_KVM_SYSTEM_TIME_NEW;
			msr_kvm_wall_clock = MSR_KVM_WALL_CLOCK_NEW;
			return PRESENT;
		} else if (flags & 0) {
			msr_kvm_system_time = MSR_KVM_SYSTEM_TIME;
			msr_kvm_wall_clock = MSR_KVM_WALL_CLOCK;
			return PRESENT;
		} else
			return NON_PRESENT;

```
MSR_KVM_ASYNC_PF_EN:
	0x4b564d02

data:
	异步页错误（APF）控MSR

	Bits 63-6 hold 64-byte aligned physical address of a 64 byte memory area
	which must be in guest RAM. This memory is expected to hold the
```

	  struct kvm_vcpu_pv_apf_data {
		/* Used for 'page not present' events delivered via #PF */
		__u32 flags;

		/* Used for 'page ready' events delivered via interrupt notification */
		__u32 token;

		__u8 pad[56];
	  };

	Bits 5-4 of the MSR are reserved and should be zero. Bit 0 is set to 1
	when asynchronous page faults are enabled on the vcpu, 0 when disabled.
	Bit 1 is 1 if asynchronous page faults can be injected when vcpu is in
	cpl == 0. Bit 2 is 1 if asynchronous page faults are delivered to L1 as
	#PF vmexits.  Bit 2 can be set only if KVM_FEATURE_ASYNC_PF_VMEXIT is
	present in CPUID. Bit 3 enables interrupt based delivery of 'page ready'
	events. Bit 3 can only be set if KVM_FEATURE_ASYNC_PF_INT is present in
	CPUID.

	'Page not present' events are currently always delivered as synthetic
	#PF exception. During delivery of these events APF CR2 register contains
	a token that will be used to notify the guest when missing page becomes
	available. Also, to make it possible to distinguish between real #PF and
	APF, first 4 bytes of 64 byte memory location ('flags') will be written
	to by the hypervisor at the time of injection. Only first bit of 'flags'
	is currently supported, when set, it indicates that the guest is dealing
	with asynchronous 'page not present' event. If during a page fault APF
	'flags' is '0' it means that this is regular page fault. Guest is
	supposed to clear 'flags' when it is done handling #PF exception so the
	next event can be delivered.

	Note, since APF 'page not present' events use the same exception vector
	as regular page fault, guest must reset 'flags' to '0' before it does
	something that can generate normal page fault.

	Bytes 4-7 of 64 byte memory location ('token') will be written to by the
	hypervisor at the time of APF 'page ready' event injection. The content
	of these bytes is a token which was previously delivered in CR2 as
	'page not present' event. The event indicates the page is now available.
	Guest is supposed to write '0' to 'token' when it is done handling
	'page ready' event and to write '1' to MSR_KVM_ASYNC_PF_ACK after
	clearing the location; writing to the MSR forces KVM to re-scan its
	queue and deliver the next pending notification.

	Note, MSR_KVM_ASYNC_PF_INT MSR specifying the interrupt vector for 'page
	ready' APF delivery needs to be written to before enabling APF mechanism
	in MSR_KVM_ASYNC_PF_EN or interrupt #0 can get injected. The MSR is
	available if KVM_FEATURE_ASYNC_PF_INT is present in CPUID.

	Note, previously, 'page ready' events were delivered via the same #PF
	exception as 'page not present' events but this is now deprecated. If
	bit 3 (interrupt based delivery) is not set APF events are not delivered.

	If APF is disabled while there are outstanding APFs, they will
	not be delivered.

	Currently 'page ready' APF events will be always delivered on the
	same vcpu as 'page not present' event was, but guest should not rely on
	that.

```
MSR_KVM_STEAL_TIME:
	0x4b564d03

data:
	一个内存区域的 64 字节对齐物理地址，该区域必须
	位于客户RAM 中，外加 bit 0 中的使能位。该内存预期用于
```

	  struct kvm_steal_time {
		__u64 steal;
		__u32 version;
		__u32 flags;
		__u8  preempted;
		__u8  u8_pad[3];
		__u32 pad[11];
	  }

	whose data will be filled in by the hypervisor periodically. Only one
	write, or registration, is needed for each VCPU. The interval between
	updates of this structure is arbitrary and implementation-dependent.
	The hypervisor may update this structure at any time it sees fit until
	anything with bit0 == 0 is written to it. Guest is required to make sure
	this structure is initialized to zero.

	Fields have the following meanings:

	version:
		a sequence counter. In other words, guest has to check
		this field before and after grabbing time information and make
		sure they are both equal and even. An odd version indicates an
		in-progress update.

	flags:
		At this point, always zero. May be used to indicate
		changes in this structure in the future.

	steal:
		the amount of time in which this vCPU did not run, in
		nanoseconds. Time during which the vcpu is idle, will not be
		reported as steal time.

	preempted:
		indicate the vCPU who owns this struct is running or
		not. Non-zero values mean the vCPU has been preempted. Zero
		means the vCPU is not preempted. NOTE, it is always zero if the
		the hypervisor doesn't support this field.

```
MSR_KVM_EOI_EN:
	0x4b564d04

data:
	vCPU 上启用了 PV（半虚拟化）中断结束时，bit 0 1；禁用时0。bit 1 为保留位，必须为 0。当 PV 中断结束被启用（bit 0 置位）时，bit 63-2 保存一4 字节对齐的物理地址，指向一4 字节内存区域，该区域必须位于客户RAM 中且必须被清零

	4 字节内存区域的最低有效位（第一位）将由 hypervisor 写入，通常是在注入中断时。值为 1 表示客户机可以跳过向 APIC 写入 EOI（通过 MSR MMIO 写）；相反，只需通过清除客户机内存中的该位来发出 EOI 信号——该位置稍后会被 hypervisor 轮询。值为 0 表示需要进EOI 写操作

	客户机忽略该优化并直接执APIC EOI 写操作始终是安全的

	Hypervisor 保证只会在当VCPU 上下文内修改该最低有效位，这意味着客户机无需使用 lock 前缀或内存排序原语来hypervisor 同步

	然而，hypervisor 可以随时置位或清除该内存位：因此，为了确hypervisor 不会在客户机检测是否可以跳EOI APIC 写、与清除该位以向 hypervisor 发出 EOI 信号之间的窗口期内打断客户机并清除该内存区域的最低有效位，客户机必须使用单条 CPU 指令（如 test-and-clear compare-and-exchange）同时读取并清除该内存区域的最低有效位

MSR_KVM_POLL_CONTROL:
	0x4b564d05

	控制宿主机侧的轮询

data:
	Bit 0 用于启用）或禁用）宿主机侧的 HLT 轮询逻辑

	KVM 客户机可以请求宿主机不要HLT 时轮询，例如当它们自身正在进行轮询时

MSR_KVM_ASYNC_PF_INT:
	0x4b564d06

data:
	第二个异步页错误（APF）控MSR

	Bit 0-7：用于投'page ready'（页面就绪）APF 事件APIC 向量
	Bit 8-63：保

	用于异步 'page ready' 通知投递的中断向量
	该向量必须在异步页错误机制于 MSR_KVM_ASYNC_PF_EN 中启用之前设置好。仅CPUID 中存KVM_FEATURE_ASYNC_PF_INT 时该 MSR 才可用

MSR_KVM_ASYNC_PF_ACK:
	0x4b564d07

data:
	异步页错误（APF）确认

	当客户机处理'page ready' APF 事件，且 'struct kvm_vcpu_pv_apf_data' 中的 'token' 字段被清除后，应向该 MSR bit 0 写入 '1'，这会促使宿主机重新扫描其队列并检查是否还有更多待处理通知。仅CPUID 中存KVM_FEATURE_ASYNC_PF_INT 时该 MSR 才可用

MSR_KVM_MIGRATION_CONTROL:
        0x4b564d08

data:
        仅当 CPUID 中存KVM_FEATURE_MIGRATION_CONTROL 时该 MSR 才可用。Bit 0 表示是否允许对客户机进行实时迁移

        当客户机启动时，若客户机使用了加密内存，bit 0 0；若客户机未使用加密内存，bit 0 1。如果客户机通过 `KVM_HC_MAP_GPA_RANGE` hypercall 向宿主机通报页加密状态，则它可以将该 MSR bit 0 置位，以允许对客户机进行实时迁移
