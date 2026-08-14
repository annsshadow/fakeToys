
## Linux KVM 瓒呯骇璋冪敤锛圚ypercall锛?


X86锛?
 KVM 瓒呯骇璋冪敤鐢?vmcall 鎴?vmmcall 鎸囦护鐨勪笁瀛楄妭搴忓垪缁勬垚銆傝櫄鎷熸満鐩戞帶鍣紙hypervisor锛夊彲浠ュ皢鍏舵浛鎹负淇濊瘉鍙楁敮鎸佺殑鎸囦护銆?

 鏈€澶氬彲浠ユ湁鍥涗釜鍙傛暟鍒嗗埆閫氳繃 rbx銆乺cx銆乺dx 鍜?rsi 浼犻€掋€傝秴绾ц皟鐢ㄥ彿搴旀斁鍦?rax 涓紝杩斿洖鍊煎皢鏀惧湪 rax 涓€傞櫎闈炵壒瀹氱殑瓒呯骇璋冪敤鏄庣‘璇存槑锛屽惁鍒欎笉浼氱牬鍧忓叾瀹冨瘎瀛樺櫒銆?

S390锛?
  R2-R7 鐢ㄤ簬鍙傛暟 1-6銆傛澶栵紝R1 鐢ㄤ簬瓒呯骇璋冪敤鍙枫€傝繑鍥炲€煎啓鍏?R2銆?

  S390 浣跨敤璇婃柇鎸囦护锛坉iagnose instruction锛変綔涓鸿秴绾ц皟鐢紙0x500锛夛紝瓒呯骇璋冪敤鍙锋斁鍦?R1 涓€?

  鏈夊叧 KVM 鏀寔鐨?S390 璇婃柇璋冪敤鐨勬洿澶氫俊鎭紝璇峰弬闃?Documentation/virt/kvm/s390/s390-diag.rst銆?

PowerPC锛?
  瀹冧娇鐢?R3-R10锛岃秴绾ц皟鐢ㄥ彿鍦?R11 涓€俁4-R11 鐢ㄤ綔杈撳嚭瀵勫瓨鍣ㄣ€傝繑鍥炲€兼斁鍦?R3 涓€?

  KVM 瓒呯骇璋冪敤浣跨敤 4 瀛楄妭鎿嶄綔鐮侊紝杩欎簺鎿嶄綔鐮佷細琚墦琛ヤ竵鏇挎崲涓鸿澶囨爲 /hypervisor 鑺傜偣鍐呯殑 'hypercall-instructions' 灞炴€с€?
  鏇村淇℃伅璇峰弬闃?Documentation/virt/kvm/ppc-pv.rst

MIPS锛?
  KVM 瓒呯骇璋冪敤浣跨敤 HYPCALL 鎸囦护锛屼唬鐮佷负 0锛岃秴绾ц皟鐢ㄥ彿鍦?$2 (v0) 涓€傛渶澶氬洓涓弬鏁板彲浠ユ斁鍦?$4-$7 (a0-a3) 涓紝杩斿洖鍊兼斁鍦?$2 (v0) 涓€?

## KVM 瓒呯骇璋冪敤鏂囨。


姣忎釜瓒呯骇璋冪敤鐨勬ā鏉夸负锛?
1. 瓒呯骇璋冪敤鍚嶇О銆?
2. 鏋舵瀯锛圓rchitecture(s)锛?
3. 鐘舵€侊紙deprecated 宸插純鐢ㄣ€乷bsolete 宸插簾寮冦€乤ctive 鐢熸晥涓級
4. 鐢ㄩ€旓紙Purpose锛?

### 1. KVM_HC_VAPIC_POLL_IRQ


:Architecture: x86
:Status: active锛堢敓鏁堜腑锛?
:Purpose: 瑙﹀彂瀹㈡埛鏈洪€€鍑猴紝浠ヤ究瀹夸富鏈哄彲浠ュ湪閲嶆柊杩涘叆鏃舵鏌ユ槸鍚︽湁鎸傝捣鐨勪腑鏂€?

### 2. KVM_HC_MMU_OP


:Architecture: x86
:Status: deprecated锛堝凡寮冪敤锛夈€?
:Purpose: 鏀寔 MMU 鎿嶄綔锛屼緥濡傚啓鍏?PTE銆佸埛鏂?TLB銆侀噴鏀?PT銆?

### 3. KVM_HC_FEATURES


:Architecture: PPC
:Status: active锛堢敓鏁堜腑锛?
:Purpose: 鍚戝鎴锋満鏆撮湶瓒呯骇璋冪敤鐨勫彲鐢ㄦ€с€傚湪 x86 骞冲彴涓婏紝浣跨敤 cpuid 鏉ユ灇涓惧摢浜涜秴绾ц皟鐢ㄥ彲鐢ㄣ€傚湪 PPC 涓婏紝鏃㈠彲浠ヤ娇鐢ㄥ熀浜庤澶囨爲鐨勬煡鎵撅紙杩欎篃鏄?EPAPR 鎵€瑙勫畾鐨勶級锛屼篃鍙互浣跨敤 KVM 鐗瑰畾鐨勬灇涓炬満鍒讹紙鍗虫湰瓒呯骇璋冪敤锛夈€?

### 4. KVM_HC_PPC_MAP_MAGIC_PAGE


:Architecture: PPC
:Status: active锛堢敓鏁堜腑锛?
:Purpose: 涓轰簡鍦ㄨ櫄鎷熸満鐩戞帶鍣ㄤ笌瀹㈡埛鏈轰箣闂村缓绔嬮€氫俊锛屽瓨鍦ㄤ竴涓叡浜〉锛屽叾涓寘鍚儴鍒嗙鐞嗙▼搴忓彲瑙佺殑瀵勫瓨鍣ㄧ姸鎬併€傚鎴锋満鍙互閫氳繃姝よ秴绾ц皟鐢ㄥ皢璇ュ叡浜〉鏄犲皠锛屼粠鑰岄€氳繃鍐呭瓨璁块棶鍏剁鐞嗙▼搴忓瘎瀛樺櫒銆?

### 5. KVM_HC_KICK_CPU


:Architecture: x86
:Status: active锛堢敓鏁堜腑锛?
:Purpose: 鐢ㄤ簬灏嗗浜?HLT 鐘舵€佺殑 vcpu 鍞ら啋鐨勮秴绾ц皟鐢?
:Usage example:
  涓€涓崐铏氭嫙鍖栧鎴锋満鐨?vcpu 鍦ㄥ鎴锋満鍐呮牳妯″紡涓嬪繖绛夊緟鏌愪釜浜嬩欢鍙戠敓锛堜緥濡傛煇涓嚜鏃嬮攣鍙樹负鍙敤锛夋椂锛屼竴鏃﹀繖绛夊緟瓒呰繃鏌愪釜闃堝€兼椂闂撮棿闅旓紝灏卞彲浠ユ墽琛?HLT 鎸囦护銆傛墽琛?HLT 鎸囦护浼氬鑷磋櫄鎷熸満鐩戞帶鍣ㄥ皢璇?vcpu 缃负鐫＄湢锛岀洿鍒板嚭鐜板悎閫傜殑浜嬩欢銆傚悓涓€瀹㈡埛鏈虹殑鍙︿竴涓?vcpu 鍙互閫氳繃鍙戝嚭 KVM_HC_KICK_CPU 瓒呯骇璋冪敤骞舵寚瀹氳鍞ら啋鐨?vcpu 鐨?APIC ID (a1) 鏉ュ敜閱掕鐫＄湢涓殑 vcpu銆傝秴绾ц皟鐢ㄤ腑杩樻湁涓€涓澶栧弬鏁?(a0) 鐣欎綔灏嗘潵浣跨敤銆?

### 6. KVM_HC_CLOCK_PAIRING


:Architecture: x86
:Status: active锛堢敓鏁堜腑锛?
:Purpose: 鐢ㄤ簬鍚屾瀹夸富鏈哄拰瀹㈡埛鏈烘椂閽熺殑瓒呯骇璋冪敤銆?

鐢ㄦ硶锛?

a0锛氬涓绘満澶嶅埗 "struct kvm_clock_offset" 缁撴瀯鐨勫鎴锋満鐗╃悊鍦板潃銆?

a1锛歝lock_type锛岀洰鍓嶄粎鏀寔 KVM_CLOCK_PAIRING_WALLCLOCK (0)锛堝搴斾簬瀹夸富鏈虹殑 CLOCK_REALTIME 鏃堕挓锛夈€?

```

		struct kvm_clock_pairing {
			__s64 sec;
			__s64 nsec;
			__u64 tsc;
			__u32 flags;
			__u32 pad[9];
		};

       Where:
               * sec: seconds from clock_type clock.
               * nsec: nanoseconds from clock_type clock.
               * tsc: guest TSC value used to calculate sec/nsec pair
               * flags: flags, unused (0) at the moment.

```
璇ヨ秴绾ц皟鐢ㄨ瀹㈡埛鏈鸿兘澶熷湪瀹夸富鏈哄拰瀹㈡埛鏈轰箣闂磋绠楃簿纭殑鏃堕棿鎴炽€傚鎴锋満鍙互浣跨敤杩斿洖鐨?TSC 鍊硷紝鍦ㄥ悓涓€鏃跺埢璁＄畻鍏舵椂閽熺殑 CLOCK_REALTIME銆?

濡傛灉瀹夸富鏈烘湭浣跨敤 TSC 鏃堕挓婧愶紝鎴栬€呮椂閽熺被鍨嬩笉鍚屼簬 KVM_CLOCK_PAIRING_WALLCLOCK锛屽垯杩斿洖 KVM_EOPNOTSUPP銆?

### 7. KVM_HC_SEND_IPI


:Architecture: x86
:Status: active锛堢敓鏁堜腑锛?
:Purpose: 鍚戝涓?vCPU 鍙戦€?IPI銆?

- a0锛氱洰鏍?APIC ID 浣嶅浘鐨勪綆浣嶉儴鍒?
- a1锛氱洰鏍?APIC ID 浣嶅浘鐨勯珮浣嶉儴鍒?
- a2锛氫綅鍥句腑鐨勬渶浣?APIC ID
- a3锛欰PIC ICR

璇ヨ秴绾ц皟鐢ㄨ瀹㈡埛鏈哄彂閫佸鎾?IPI锛屽湪 64 浣嶆ā寮忎笅姣忔瓒呯骇璋冪敤鏈€澶?128 涓洰鏍囷紝鍦?32 浣嶆ā寮忎笅姣忔鏈€澶?64 涓?vCPU銆傜洰鏍囩敱鍓嶄袱涓弬鏁帮紙a0 鍜?a1锛変腑鍖呭惈鐨勪綅鍥捐〃绀恒€俛0 鐨勪綅 0 瀵瑰簲绗笁涓弬鏁帮紙a2锛変腑鐨?APIC ID锛屼綅 1 瀵瑰簲 a2+1锛屼緷姝ょ被鎺ㄣ€?

杩斿洖鎴愬姛鎶曢€?IPI 鐨?CPU 鏁伴噺銆?

### 8. KVM_HC_SCHED_YIELD


:Architecture: x86
:Status: active锛堢敓鏁堜腑锛?
:Purpose: 濡傛灉 IPI 鐩爣 vCPU 琚姠鍗狅紝鍒欑敤浜庤姝ワ紙yield锛夌殑瓒呯骇璋冪敤

a0锛氱洰鏍?APIC ID

:Usage example: 褰撳悜 vCPU 鍙戦€?call-function IPI-many 鏃讹紝濡傛灉浠讳竴 IPI 鐩爣 vCPU 琚姠鍗狅紝鍒欒姝ャ€?

### 9. KVM_HC_MAP_GPA_RANGE


:Architecture: x86
:Status: active锛堢敓鏁堜腑锛?
:Purpose: 璇锋眰 KVM 浠ユ寚瀹氱殑灞炴€ф槧灏勪竴涓?GPA 鑼冨洿銆?

a0锛氳捣濮嬮〉鐨勫鎴锋満鐗╃悊鍦板潃
a1锛氾紙4kb锛夐〉鐨勬暟閲忥紙鍦?GPA 绌洪棿涓繀椤昏繛缁級
a2锛氬睘鎬?

    鍏朵腑 'attributes' 锛?
        - 浣?3:0 - 棣栭€夐〉澶у皬缂栫爜 0 = 4kb锛? = 2mb锛? = 1gb锛岀瓑绛夆€︹€?
        - 浣?4 - plaintext锛堟槑鏂囷級= 0锛宔ncrypted锛堝姞瀵嗭級= 1
        - 浣?63:5 - 淇濈暀锛堝繀椤讳负 0锛?

**瀹炵幇璇存槑**锛氳瓒呯骇璋冪敤鍦ㄧ敤鎴风┖闂撮€氳繃 KVM_CAP_EXIT_HYPERCALL 鑳藉姏瀹炵幇銆傜敤鎴风┖闂村繀椤诲湪瀹㈡埛鏈?CPUID 涓€氬憡 KVM_FEATURE_HC_MAP_GPA_RANGE 涔嬪墠鍚敤璇ヨ兘鍔涖€傛澶栵紝濡傛灉瀹㈡埛鏈烘敮鎸?KVM_FEATURE_MIGRATION_CONTROL锛岀敤鎴风┖闂磋繕蹇呴』璁剧疆涓€涓?MSR 杩囨护鍣ㄦ潵澶勭悊瀵?MSR_KVM_MIGRATION_CONTROL 鐨勫啓鍏ャ€?
