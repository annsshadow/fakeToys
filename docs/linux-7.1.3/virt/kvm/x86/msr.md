
## KVM 涓撶敤鐨?MSR


:Author: Glauber Costa <glommer@redhat.com>, Red Hat Inc, 2010

KVM 浣跨敤涓€浜涜嚜瀹氫箟鐨?MSR 鏉ュ鐞嗘煇浜涜姹傘€?

鑷畾涔?MSR 鏈変竴涓负鍏朵繚鐣欑殑鍖洪棿锛岃寖鍥翠粠 0x4b564d00 鍒?0x4b564dff銆傚湪姝ゅ尯闂翠箣澶栦篃瀛樺湪涓€浜?MSR锛屼絾瀹冧滑宸茶寮冪敤锛屼笉寤鸿浣跨敤銆?

### 鑷畾涔?MSR 鍒楄〃


褰撳墠鏀寔鐨勮嚜瀹氫箟 MSR 鍒楄〃濡備笅锛?

MSR_KVM_WALL_CLOCK_NEW:
	0x4b564d00

data:
	涓€涓唴瀛樺尯鍩熺殑 4 瀛楄妭瀵归綈鐗╃悊鍦板潃锛岃鍖哄煙蹇呴』浣嶄簬
	瀹㈡埛鏈?RAM 涓€傝鍐呭瓨棰勬湡鐢ㄤ簬淇濆瓨濡備笅鍐呭鐨勫壇鏈?
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
	涓€涓唴瀛樺尯鍩熺殑 4 瀛楄妭瀵归綈鐗╃悊鍦板潃锛岃鍖哄煙蹇呴』浣嶄簬瀹㈡埛鏈?RAM 涓紝
	澶栧姞 bit 0 涓殑涓€涓娇鑳戒綅銆傝鍐呭瓨棰勬湡鐢ㄤ簬淇濆瓨
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
	涓?MSR_KVM_WALL_CLOCK_NEW 鐩稿悓銆傝鏀圭敤鍚庤€呫€?

	璇?MSR 涓嶅湪淇濈暀鐨?KVM 鍖洪棿鍐咃紝鏈潵鍙兘浼氳绉婚櫎銆?
	瀹冪殑浣跨敤宸茶寮冪敤銆?

	浣跨敤鍓嶅繀椤婚€氳繃 0x4000001 cpuid 鍙跺瓙涓殑 bit 0 妫€鏌ヨ MSR 鏄惁鍙敤銆?

MSR_KVM_SYSTEM_TIME:
	0x12

data and functioning:
	涓?MSR_KVM_SYSTEM_TIME_NEW 鐩稿悓銆傝鏀圭敤鍚庤€呫€?

	璇?MSR 涓嶅湪淇濈暀鐨?KVM 鍖洪棿鍐咃紝鏈潵鍙兘浼氳绉婚櫎銆?
	瀹冪殑浣跨敤宸茶寮冪敤銆?

	浣跨敤鍓嶅繀椤婚€氳繃 0x4000001 cpuid 鍙跺瓙涓殑 bit 0 妫€鏌ヨ MSR 鏄惁鍙敤銆?

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
	寮傛椤甸敊璇紙APF锛夋帶鍒?MSR銆?

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
	涓€涓唴瀛樺尯鍩熺殑 64 瀛楄妭瀵归綈鐗╃悊鍦板潃锛岃鍖哄煙蹇呴』
	浣嶄簬瀹㈡埛鏈?RAM 涓紝澶栧姞 bit 0 涓殑浣胯兘浣嶃€傝鍐呭瓨棰勬湡鐢ㄤ簬
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
	褰?vCPU 涓婂惎鐢ㄤ簡 PV锛堝崐铏氭嫙鍖栵級涓柇缁撴潫鏃讹紝bit 0 涓?1锛涚鐢ㄦ椂涓?0銆俠it 1 涓轰繚鐣欎綅锛屽繀椤讳负 0銆傚綋 PV 涓柇缁撴潫琚惎鐢紙bit 0 缃綅锛夋椂锛宐it 63-2 淇濆瓨涓€涓?4 瀛楄妭瀵归綈鐨勭墿鐞嗗湴鍧€锛屾寚鍚戜竴涓?4 瀛楄妭鍐呭瓨鍖哄煙锛岃鍖哄煙蹇呴』浣嶄簬瀹㈡埛鏈?RAM 涓笖蹇呴』琚竻闆躲€?

	璇?4 瀛楄妭鍐呭瓨鍖哄煙鐨勬渶浣庢湁鏁堜綅锛堢涓€浣嶏級灏嗙敱 hypervisor 鍐欏叆锛岄€氬父鏄湪娉ㄥ叆涓柇鏃躲€傚€间负 1 琛ㄧず瀹㈡埛鏈哄彲浠ヨ烦杩囧悜 APIC 鍐欏叆 EOI锛堥€氳繃 MSR 鎴?MMIO 鍐欙級锛涚浉鍙嶏紝鍙渶閫氳繃娓呴櫎瀹㈡埛鏈哄唴瀛樹腑鐨勮浣嶆潵鍙戝嚭 EOI 淇″彿鈥斺€旇浣嶇疆绋嶅悗浼氳 hypervisor 杞銆傚€间负 0 琛ㄧず闇€瑕佽繘琛?EOI 鍐欐搷浣溿€?

	瀹㈡埛鏈哄拷鐣ヨ浼樺寲骞剁洿鎺ユ墽琛?APIC EOI 鍐欐搷浣滃缁堟槸瀹夊叏鐨勩€?

	Hypervisor 淇濊瘉鍙細鍦ㄥ綋鍓?VCPU 涓婁笅鏂囧唴淇敼璇ユ渶浣庢湁鏁堜綅锛岃繖鎰忓懗鐫€瀹㈡埛鏈烘棤闇€浣跨敤 lock 鍓嶇紑鎴栧唴瀛樻帓搴忓師璇潵涓?hypervisor 鍚屾銆?

	鐒惰€岋紝hypervisor 鍙互闅忔椂缃綅鎴栨竻闄よ鍐呭瓨浣嶏細鍥犳锛屼负浜嗙‘淇?hypervisor 涓嶄細鍦ㄥ鎴锋満妫€娴嬫槸鍚﹀彲浠ヨ烦杩?EOI APIC 鍐欍€佷笌娓呴櫎璇ヤ綅浠ュ悜 hypervisor 鍙戝嚭 EOI 淇″彿涔嬮棿鐨勭獥鍙ｆ湡鍐呮墦鏂鎴锋満骞舵竻闄よ鍐呭瓨鍖哄煙鐨勬渶浣庢湁鏁堜綅锛屽鎴锋満蹇呴』浣跨敤鍗曟潯 CPU 鎸囦护锛堝 test-and-clear 鎴?compare-and-exchange锛夊悓鏃惰鍙栧苟娓呴櫎璇ュ唴瀛樺尯鍩熺殑鏈€浣庢湁鏁堜綅銆?

MSR_KVM_POLL_CONTROL:
	0x4b564d05

	鎺у埗瀹夸富鏈轰晶鐨勮疆璇€?

data:
	Bit 0 鐢ㄤ簬鍚敤锛?锛夋垨绂佺敤锛?锛夊涓绘満渚х殑 HLT 杞閫昏緫銆?

	KVM 瀹㈡埛鏈哄彲浠ヨ姹傚涓绘満涓嶈鍦?HLT 鏃惰疆璇紝渚嬪褰撳畠浠嚜韬鍦ㄨ繘琛岃疆璇㈡椂銆?

MSR_KVM_ASYNC_PF_INT:
	0x4b564d06

data:
	绗簩涓紓姝ラ〉閿欒锛圓PF锛夋帶鍒?MSR銆?

	Bit 0-7锛氱敤浜庢姇閫?'page ready'锛堥〉闈㈠氨缁級APF 浜嬩欢鐨?APIC 鍚戦噺銆?
	Bit 8-63锛氫繚鐣?

	鐢ㄤ簬寮傛 'page ready' 閫氱煡鎶曢€掔殑涓柇鍚戦噺銆?
	璇ュ悜閲忓繀椤诲湪寮傛椤甸敊璇満鍒朵簬 MSR_KVM_ASYNC_PF_EN 涓惎鐢ㄤ箣鍓嶈缃ソ銆備粎褰?CPUID 涓瓨鍦?KVM_FEATURE_ASYNC_PF_INT 鏃惰 MSR 鎵嶅彲鐢ㄣ€?

MSR_KVM_ASYNC_PF_ACK:
	0x4b564d07

data:
	寮傛椤甸敊璇紙APF锛夌‘璁ゃ€?

	褰撳鎴锋満澶勭悊瀹?'page ready' APF 浜嬩欢锛屼笖 'struct kvm_vcpu_pv_apf_data' 涓殑 'token' 瀛楁琚竻闄ゅ悗锛屽簲鍚戣 MSR 鐨?bit 0 鍐欏叆 '1'锛岃繖浼氫績浣垮涓绘満閲嶆柊鎵弿鍏堕槦鍒楀苟妫€鏌ユ槸鍚﹁繕鏈夋洿澶氬緟澶勭悊閫氱煡銆備粎褰?CPUID 涓瓨鍦?KVM_FEATURE_ASYNC_PF_INT 鏃惰 MSR 鎵嶅彲鐢ㄣ€?

MSR_KVM_MIGRATION_CONTROL:
        0x4b564d08

data:
        浠呭綋 CPUID 涓瓨鍦?KVM_FEATURE_MIGRATION_CONTROL 鏃惰 MSR 鎵嶅彲鐢ㄣ€侭it 0 琛ㄧず鏄惁鍏佽瀵瑰鎴锋満杩涜瀹炴椂杩佺Щ銆?

        褰撳鎴锋満鍚姩鏃讹紝鑻ュ鎴锋満浣跨敤浜嗗姞瀵嗗唴瀛橈紝bit 0 涓?0锛涜嫢瀹㈡埛鏈烘湭浣跨敤鍔犲瘑鍐呭瓨锛宐it 0 涓?1銆傚鏋滃鎴锋満閫氳繃 `KVM_HC_MAP_GPA_RANGE` hypercall 鍚戝涓绘満閫氭姤椤靛姞瀵嗙姸鎬侊紝鍒欏畠鍙互灏嗚 MSR 鐨?bit 0 缃綅锛屼互鍏佽瀵瑰鎴锋満杩涜瀹炴椂杩佺Щ銆?
