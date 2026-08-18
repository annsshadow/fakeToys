
## 閫氱敤铏氭嫙鏈猴紙vm锛夋帴鍙?

铏氭嫙鏈衡€滆澶団€濆悓鏍锋帴鍙?ioctl `KVM_SET_DEVICE_ATTR`銆?`KVM_GET_DEVICE_ATTR` 鍜?`KVM_HAS_DEVICE_ATTR`銆傝鎺ュ彛浣跨敤涓庡叾浠栬澶?鐩稿悓鐨?`struct kvm_device_attr`锛屼絾閽堝鐨勬槸铏氭嫙鏈哄叏灞€鐨勮缃笌鎺у埗銆?
姣忓彴铏氭嫙鏈虹殑鍒嗙粍涓庡睘鎬э紙濡傛灉鏈夛級鏄灦鏋勭浉鍏崇殑銆?
## 1. 鍒嗙粍锛欿VM_S390_VM_MEM_CTRL


:Architectures: s390

### 1.1. 灞炴€э細KVM_S390_VM_MEM_ENABLE_CMMA


:Parameters: none
:Returns: -EBUSY if a vcpu is already defined, otherwise 0

涓鸿櫄鎷熸満鍚敤鍗忎綔寮忓唴瀛樼鐞嗚緟鍔╋紙Collaborative Memory Management
Assist锛孋MMA锛夈€?
### 1.2. 灞炴€э細KVM_S390_VM_MEM_CLR_CMMA


:Parameters: none
:Returns: -EINVAL if CMMA was not enabled;
	  0 otherwise

娓呴櫎鎵€鏈夊鎴锋満椤电殑 CMMA 鐘舵€侊紝浣垮鎴锋満鏍囪涓烘湭浣跨敤鐨勯〉閲嶆柊鍙樹负
宸蹭娇鐢紝浠庤€屽彲鑳戒笉浼氳瀹夸富鏈哄洖鏀躲€?
### 1.3. 灞炴€?KVM_S390_VM_MEM_LIMIT_SIZE


:Parameters: in attr->addr the address for the new limit of guest memory
:Returns: -EFAULT if the given address is not accessible;
	  -EINVAL if the virtual machine is of type UCONTROL;
	  -E2BIG if the given guest memory is to big for that machine;
	  -EBUSY if a vcpu is already defined;
	  -ENOMEM if not enough memory is available for a new shadow guest mapping;
	  0 otherwise.

鍏佽鐢ㄦ埛绌洪棿鏌ヨ瀹為檯闄愬埗锛屽苟涓哄鎴锋満鍐呭瓨鐨勬渶澶уぇ灏忚缃竴涓柊鐨勯檺鍒躲€?璇ラ檺鍒跺皢鍒嗗埆鍚戜笂鍙栨暣鍒?2048 MB銆?096 GB銆?192 TB锛屽洜涓烘闄愬埗鐢遍〉琛?灞傜骇鏁板喅瀹氥€傚湪娌℃湁闄愬埗鐨勬儏鍐典笅锛屾垜浠細灏嗛檺鍒惰涓?`KVM_S390_NO_MEM_LIMIT`锛坄U64_MAX`锛夈€?
## 2. 鍒嗙粍锛欿VM_S390_VM_CPU_MODEL


:Architectures: s390

### 2.1. 灞炴€э細KVM_S390_VM_CPU_MACHINE (r/o)


```
  struct kvm_s390_vm_cpu_machine {
       __u64 cpuid;           # CPUID of host
       __u32 ibc;             # IBC level range offered by host
       __u8  pad[4];
       __u64 fac_mask[256];   # set of cpu facilities enabled by KVM
       __u64 fac_list[256];   # set of cpu facilities offered by host
  }
```

:Parameters: address of buffer to store the machine related cpu data
	     of type struct kvm_s390_vm_cpu_machine*
:Returns:   -EFAULT if the given address is not accessible from kernel space;
	    -ENOMEM if not enough memory is available to process the ioctl;
	    0 in case of success.

## 2.2. 灞炴€э細KVM_S390_VM_CPU_PROCESSOR (r/w)


```
  struct kvm_s390_vm_cpu_processor {
       __u64 cpuid;           # CPUID currently (to be) used by this vcpu
       __u16 ibc;             # IBC level currently (to be) used by this vcpu
       __u8  pad[6];
       __u64 fac_list[256];   # set of cpu facilities currently (to be) used
			      # by this vcpu
  }
```

KVM 涓嶄互浠讳綍褰㈠紡寮哄埗鎴栭檺鍒?cpu 妯″瀷鏁版嵁銆傝鎶婇€氳繃
`KVM_S390_VM_CPU_MACHINE` 鑾峰彇鐨勪俊鎭綔涓哄悎鐞嗛厤缃缃殑鍙傝€冦€傜敱棰濆璁剧疆鐨?facility 浣嶈Е鍙戙€佷絾 KVM 鏈鐞嗙殑鎸囦护鎷︽埅锛岄渶瑕佸湪 VM 椹卞姩浠ｇ爜涓疄鐜般€?
:Parameters: address of buffer to store/set the processor related cpu
	     data of type struct kvm_s390_vm_cpu_processor*.
:Returns:  -EBUSY in case 1 or more vcpus are already activated (only in write case);
	   -EFAULT if the given address is not accessible from kernel space;
	   -ENOMEM if not enough memory is available to process the ioctl;
	   0 in case of success.


### 2.3. 灞炴€э細KVM_S390_VM_CPU_MACHINE_FEAT (r/o)


鍏佽鐢ㄦ埛绌洪棿鑾峰彇鍙敤鐨?cpu 鐗规€с€傝嫢纭欢鎻愪緵涓旇鐗规€ц kvm 鏀寔锛屽垯瑙嗕负
鍙敤銆傜悊璁轰笂锛宑pu 鐗规€х敋鑷冲彲浠ュ畬鍏ㄧ敱 kvm 妯℃嫙銆?
```
  struct kvm_s390_vm_cpu_feat {
	__u64 feat[16]; # Bitmap (1 = feature available), MSB 0 bit numbering
  };
```

:Parameters: address of a buffer to load the feature list from.
:Returns:  -EFAULT if the given address is not accessible from kernel space;
	   0 in case of success.

### 2.4. 灞炴€э細KVM_S390_VM_CPU_PROCESSOR_FEAT (r/w)


鍏佽鐢ㄦ埛绌洪棿鑾峰彇鎴栨洿鏀规煇涓?VM 鎵€鏈?VCPU 宸插惎鐢ㄧ殑 cpu 鐗规€с€備笉鍙敤鐨勭壒鎬?鏃犳硶琚惎鐢ㄣ€?
璇﹁ `KVM_S390_VM_CPU_MACHINE_FEAT` 涓璇ュ弬鏁扮粨鏋勪綋鐨勬弿杩般€?
:Parameters: address of a buffer to store/load the feature list from.
:Returns:   -EFAULT if the given address is not accessible from kernel space;
	    -EINVAL if a cpu feature that is not available is to be enabled;
	    -EBUSY if at least one VCPU has already been defined;
	    0 in case of success.


### 2.5. 灞炴€э細KVM_S390_VM_CPU_MACHINE_SUBFUNC (r/o)


鍏佽鐢ㄦ埛绌洪棿鑾峰彇鍙敤鐨?cpu 瀛愬嚱鏁帮紝涓嶅仛浠讳綍鐢辫瀹?IBC 甯︽潵鐨勮繃婊ゃ€傝繖浜?瀛愬嚱鏁伴€氳繃鈥滄煡璇⑩€濇垨鈥滄祴璇曚綅锛坱est bit锛夆€濆瓙鍑芥暟鎸囩ず缁欏鎴锋満 VCPU锛屽苟琚?cpacf 鍑芥暟銆乸lo 鍜?ptff 绛変娇鐢ㄣ€?
鍙湁褰?`KVM_S390_VM_CPU_MACHINE` 鍖呭惈寮曞叆鐩稿叧鎸囦护鐨?STFL(E) 浣嶆椂锛屽瓙鍑芥暟鍧?鎵嶆湁鏁堛€傝嫢鐩稿叧鎸囦护閫氳繃鈥滄煡璇㈠瓙鍑芥暟鈥濇寚绀哄瓙鍑芥暟锛屽垯鍝嶅簲鍧楀寘鍚湪杩斿洖鐨?缁撴瀯浣撲腑锛涜嫢鐩稿叧鎸囦护閫氳繃鈥滄祴璇曚綅鈥濇満鍒舵寚绀哄瓙鍑芥暟锛屽垯瀛愬嚱鏁颁唬鐮佷互 MSB 0
浣嶇紪鍙锋柟寮忓寘鍚湪杩斿洖鐨勭粨鏋勪綋涓€?
```
  struct kvm_s390_vm_cpu_subfunc {
       u8 plo[32];           # always valid (ESA/390 feature)
       u8 ptff[16];          # valid with TOD-clock steering
       u8 kmac[16];          # valid with Message-Security-Assist
       u8 kmc[16];           # valid with Message-Security-Assist
       u8 km[16];            # valid with Message-Security-Assist
       u8 kimd[16];          # valid with Message-Security-Assist
       u8 klmd[16];          # valid with Message-Security-Assist
       u8 pckmo[16];         # valid with Message-Security-Assist-Extension 3
       u8 kmctr[16];         # valid with Message-Security-Assist-Extension 4
       u8 kmf[16];           # valid with Message-Security-Assist-Extension 4
       u8 kmo[16];           # valid with Message-Security-Assist-Extension 4
       u8 pcc[16];           # valid with Message-Security-Assist-Extension 4
       u8 ppno[16];          # valid with Message-Security-Assist-Extension 5
       u8 kma[16];           # valid with Message-Security-Assist-Extension 8
       u8 kdsa[16];          # valid with Message-Security-Assist-Extension 9
       u8 reserved[1792];    # reserved for future instructions
  };
```

:Parameters: address of a buffer to load the subfunction blocks from.
:Returns:   -EFAULT if the given address is not accessible from kernel space;
	    0 in case of success.

### 2.6. 灞炴€э細KVM_S390_VM_CPU_PROCESSOR_SUBFUNC (r/w)


鍏佽鐢ㄦ埛绌洪棿鑾峰彇鎴栨洿鏀硅鎸囩ず缁欐煇涓?VM 鎵€鏈?VCPU 鐨?cpu 瀛愬嚱鏁般€備粎褰撳唴鏍镐笌
纭欢鏀寔灏辩华鏃讹紝璇ュ睘鎬ф墠鍙敤銆?
鍐呮牳浣跨敤閰嶇疆濂界殑瀛愬嚱鏁板潡鏉ュ悜瀹㈡埛鏈烘寚绀恒€備粎褰撳叧鑱旂殑 STFL(E) 浣嶆湭琚敤鎴风┖闂?绂佺敤鏃讹紙鍗宠鏌ヨ鐨勬寚浠ゅ瀹㈡埛鏈哄疄闄呭彲鐢級锛岃瀛愬嚱鏁板潡鎵嶄細琚娇鐢ㄣ€?
鍙灏氭湭鍐欏叆浠讳綍鏁版嵁锛岃鍙栧氨浼氬け璐ャ€傛绉嶆儏鍐典笅灏嗕娇鐢?IBC 鏉ュ喅瀹氬彲鐢ㄧ殑
瀛愬嚱鏁帮紝浠ヤ繚璇佸悜鍚庡吋瀹规€с€?
璇﹁ `KVM_S390_VM_CPU_MACHINE_SUBFUNC` 涓璇ュ弬鏁扮粨鏋勪綋鐨勬弿杩般€?
:Parameters: address of a buffer to store/load the subfunction blocks from.
:Returns:   -EFAULT if the given address is not accessible from kernel space;
	    -EINVAL when reading, if there was no write yet;
	    -EBUSY if at least one VCPU has already been defined;
	    0 in case of success.

## 3. 鍒嗙粍锛欿VM_S390_VM_TOD


:Architectures: s390

### 3.1. 灞炴€э細KVM_S390_VM_TOD_HIGH


鍏佽鐢ㄦ埛绌洪棿璁剧疆/鑾峰彇 TOD 鏃堕挓鎵╁睍锛坲8锛夛紙宸茶 `KVM_S390_VM_TOD_EXT` 鍙栦唬锛夈€?
:Parameters: address of a buffer in user space to store the data (u8) to
:Returns:   -EFAULT if the given address is not accessible from kernel space;
	    -EINVAL if setting the TOD clock extension to != 0 is not supported
	    -EOPNOTSUPP for a PV guest (TOD managed by the ultravisor)

### 3.2. 灞炴€э細KVM_S390_VM_TOD_LOW


鍏佽鐢ㄦ埛绌洪棿璁剧疆/鑾峰彇 POP 涓畾涔夌殑 TOD 鏃堕挓瀵勫瓨鍣ㄧ 0-63 浣嶏紙u64锛夈€?
:Parameters: address of a buffer in user space to store the data (u64) to
:Returns:    -EFAULT if the given address is not accessible from kernel space
	     -EOPNOTSUPP for a PV guest (TOD managed by the ultravisor)

### 3.3. 灞炴€э細KVM_S390_VM_TOD_EXT


鍏佽鐢ㄦ埛绌洪棿璁剧疆/鑾峰彇 POP 涓畾涔夌殑 TOD 鏃堕挓瀵勫瓨鍣ㄧ 0-63 浣嶏紙u64锛夈€傝嫢瀹㈡埛鏈?CPU 妯″瀷鏀寔 TOD 鏃堕挓鎵╁睍锛坲8锛夛紝瀹冭繕鍏佽鐢ㄦ埛绌洪棿鑾峰彇/璁剧疆璇ユ墿灞曪紱鑻ュ鎴锋満
CPU 妯″瀷涓嶆敮鎸侊紝鍒欏皢鍏跺瓨涓?0 涓斾笉鍏佽琚涓?!= 0 鐨勫€笺€?
:Parameters: address of a buffer in user space to store the data
	     (kvm_s390_vm_tod_clock) to
:Returns:   -EFAULT if the given address is not accessible from kernel space;
	    -EINVAL if setting the TOD clock extension to != 0 is not supported
	    -EOPNOTSUPP for a PV guest (TOD managed by the ultravisor)

## 4. 鍒嗙粍锛欿VM_S390_VM_CRYPTO


:Architectures: s390

### 4.1. 灞炴€э細KVM_S390_VM_CRYPTO_ENABLE_AES_KW (w/o)


鍏佽鐢ㄦ埛绌洪棿鍚敤 aes 瀵嗛挜鍖呰锛坘ey wrapping锛夛紝鍖呮嫭鐢熸垚涓€涓柊鐨勫寘瑁呭瘑閽ャ€?
:Parameters: none
:Returns:    0

### 4.2. 灞炴€э細KVM_S390_VM_CRYPTO_ENABLE_DEA_KW (w/o)


鍏佽鐢ㄦ埛绌洪棿鍚敤 dea 瀵嗛挜鍖呰锛屽寘鎷敓鎴愪竴涓柊鐨勫寘瑁呭瘑閽ャ€?
:Parameters: none
:Returns:    0

### 4.3. 灞炴€э細KVM_S390_VM_CRYPTO_DISABLE_AES_KW (w/o)


鍏佽鐢ㄦ埛绌洪棿绂佺敤 aes 瀵嗛挜鍖呰锛屾竻闄ゅ寘瑁呭瘑閽ャ€?
:Parameters: none
:Returns:    0

### 4.4. 灞炴€э細KVM_S390_VM_CRYPTO_DISABLE_DEA_KW (w/o)


鍏佽鐢ㄦ埛绌洪棿绂佺敤 dea 瀵嗛挜鍖呰锛屾竻闄ゅ寘瑁呭瘑閽ャ€?
:Parameters: none
:Returns:    0

## 5. 鍒嗙粍锛欿VM_S390_VM_MIGRATION


:Architectures: s390

### 5.1. 灞炴€э細KVM_S390_VM_MIGRATION_STOP (w/o)


鍏佽鐢ㄦ埛绌洪棿鍋滄杩佺Щ妯″紡锛孭GSTE 杩佺Щ闇€瑕佹妯″紡銆傚湪杩佺Щ妯″紡鏈縺娲绘椂璁剧疆
璇ュ睘鎬т笉浼氭湁浠讳綍鏁堟灉銆?
:Parameters: none
:Returns:    0

### 5.2. 灞炴€э細KVM_S390_VM_MIGRATION_START (w/o)


鍏佽鐢ㄦ埛绌洪棿鍚姩杩佺Щ妯″紡锛孭GSTE 杩佺Щ闇€瑕佹妯″紡銆傚湪杩佺Щ妯″紡宸茬粡婵€娲绘椂璁剧疆
璇ュ睘鎬т笉浼氭湁浠讳綍鏁堟灉銆?
鎵€鏈夊唴瀛樻Ы锛坢emslot锛変笂蹇呴』鍚敤鑴忛〉璺熻釜锛屽惁鍒欒繑鍥?`-EINVAL`銆傚綋浠讳竴
鍐呭瓨妲戒笂鐨勮剰椤佃窡韪绂佺敤鏃讹紝杩佺Щ妯″紡浼氳嚜鍔ㄥ仠姝€?
:Parameters: none
:Returns:   -ENOMEM if there is not enough free memory to start migration mode;
	    -EINVAL if the state of the VM is invalid (e.g. no memory defined);
	    0 in case of success.

### 5.3. 灞炴€э細KVM_S390_VM_MIGRATION_STATUS (r/o)


鍏佽鐢ㄦ埛绌洪棿鏌ヨ杩佺Щ妯″紡鐨勭姸鎬併€?
:Parameters: address of a buffer in user space to store the data (u64) to;
	     the data itself is either 0 if migration mode is disabled or 1
	     if it is enabled
:Returns:   -EFAULT if the given address is not accessible from kernel space;
	    0 in case of success.

## 6. 鍒嗙粍锛欿VM_ARM_VM_SMCCC_CTRL


:Architectures: arm64

### 6.1. 灞炴€э細KVM_ARM_VM_SMCCC_FILTER (w/o)


:Parameters: Pointer to a `struct kvm_smccc_filter`

:Returns:

        ======  ===========================================
        EEXIST  Range intersects with a previously inserted
                or reserved range
        EBUSY   A vCPU in the VM has already run
        EINVAL  Invalid filter configuration
        ENOMEM  Failed to allocate memory for the in-kernel
                representation of the SMCCC filter
        ======  ===========================================

```
    enum kvm_smccc_filter_action {
            KVM_SMCCC_FILTER_HANDLE = 0,
            KVM_SMCCC_FILTER_DENY,
            KVM_SMCCC_FILTER_FWD_TO_USER,
    };

    struct kvm_smccc_filter {
            __u32 base;
            __u32 nr_functions;
            __u8 action;
            __u8 pad[15];
    };
```

杩囨护鍣ㄥ畾涔変负涓€缁勪簰涓嶉噸鍙犵殑鑼冨洿銆傛瘡涓寖鍥村畾涔変竴涓鏂藉姞浜庤寖鍥村唴 SMCCC
璋冪敤鐨勫姩浣溿€傜敤鎴风┖闂村彲浠ラ€氳繃瀵硅灞炴€ц繛缁娆¤皟鐢ㄦ潵鍚戣繃婊ゅ櫒涓彃鍏ュ涓寖鍥淬€?
KVM 鐨勯粯璁ら厤缃厑璁告墍鏈夊凡瀹炵幇鐨?SMCCC 璋冪敤銆傚洜姝わ紝鐢ㄦ埛绌洪棿鍙互绋€鐤忓湴瀹氫箟
SMCCC 杩囨护鍣紝浠呴渶鎻忚堪閭ｄ簺淇敼榛樿琛屼负鐨勮寖鍥淬€?
`struct kvm_smccc_filter` 琛ㄨ揪鐨勮寖鍥翠负
[`base`, `base + nr_functions`)銆傝鑼冨洿涓嶅厑璁稿洖缁曪紝鍗崇敤鎴风┖闂翠笉鑳戒緷璧?`base + nr_functions` 婧㈠嚭銆?
SMCCC 杩囨护鍣ㄥ悓鏃堕€傜敤浜庡鎴锋満鍙戣捣鐨?SMC 涓?HVC 璋冪敤銆係MCCC 杩囨护鍣ㄤ細鎷︽埅瀵?SMCCC 璋冪敤鐨勫唴鏍稿唴妯℃嫙锛屽洜姝ゅ叾浣滅敤鏃╀簬鍏朵粬涓?SMCCC 璋冪敤浜や簰鐨勬帴鍙?锛堜緥濡?hypercall 浣嶅浘瀵勫瓨鍣級銆?
鍔ㄤ綔锛?
 - `KVM_SMCCC_FILTER_HANDLE`锛氬厑璁歌瀹㈡埛鏈?SMCCC 璋冪敤鍦ㄥ唴鏍稿唴琚鐞嗐€傚己鐑?   寤鸿鐢ㄦ埛绌洪棿 **涓嶈** 鏄惧紡鎻忚堪鍏佽鐨?SMCCC 璋冪敤鑼冨洿銆?
 - `KVM_SMCCC_FILTER_DENY`锛氬湪鍐呮牳鍐呮嫆缁濊瀹㈡埛鏈?SMCCC 璋冪敤骞惰繑鍥炵粰瀹㈡埛鏈恒€?
 - `KVM_SMCCC_FILTER_FWD_TO_USER`锛氳瀹㈡埛鏈?SMCCC 璋冪敤琚浆鍙戝埌鐢ㄦ埛绌洪棿锛?   閫€鍑哄師鍥犱负 `KVM_EXIT_HYPERCALL`銆?
`pad` 瀛楁淇濈暀渚涘皢鏉ヤ娇鐢紝蹇呴』涓?0銆傝嫢璇ュ瓧娈甸潪闆讹紝KVM 鍙兘杩斿洖 `-EINVAL`銆?
KVM 淇濈暀浜嗏€淎rm 鏋舵瀯璋冪敤鈥濈殑鍑芥暟 ID 鑼冨洿锛屽苟灏嗘嫆缁濅负杩欎簺鑼冨洿鐨勪换浣曢儴鍒嗗畾涔?杩囨护鍣細

        =========== ===============
        Start       End (inclusive)
        =========== ===============
        0x8000_0000 0x8000_FFFF
        0xC000_0000 0xC000_FFFF
        =========== ===============
