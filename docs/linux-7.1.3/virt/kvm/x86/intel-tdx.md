
## Intel 淇′换鍩熸墿灞曪紙TDX锛?

## 姒傝堪


Intel 鐨勪俊浠诲煙鎵╁睍锛圱DX锛変繚鎶ゆ満瀵嗗鎴锋満 VM 鍏嶅彈涓绘満鍜岀墿鐞嗘敾鍑汇€備竴涓悕涓?鈥淭DX module鈥濈殑銆佺粡鐢?CPU 璇佹槑鐨勮蒋浠舵ā鍧楄繍琛屽湪涓€涓柊鐨?CPU 闅旂鑼冨洿鍐咃紝
鎻愪緵绠＄悊鍜岃繍琛屽彈淇濇姢 VM锛堝嵆 TDX 瀹㈡埛鏈烘垨 TD锛夌殑鍔熻兘銆?
鐧界毊涔︺€佽鑼冨強鍏朵粬璧勬簮璇峰弬鑰?[^1^]銆?
鏈枃妗ｆ弿杩?TDX 鐗规湁鐨?KVM ABI銆俆DX module 闇€瑕佸厛杩涜鍒濆鍖栵紝涔嬪悗鎵嶈兘琚?KVM
鐢ㄤ簬杩愯浠讳綍 TDX 瀹㈡埛鏈恒€傚涓绘牳蹇冨唴鏍告彁渚涘 TDX module 鍒濆鍖栫殑鏀寔锛岀浉鍏?璇存槑瑙?Documentation/arch/x86/tdx.rst銆?
## API 鎻忚堪


### KVM_MEMORY_ENCRYPT_OP

:Type: vm ioctl, vcpu ioctl

瀵逛簬 TDX 鎿嶄綔锛孠VM_MEMORY_ENCRYPT_OP 琚噸鏂扮敤浣滀竴涓€氱敤鐨?ioctl锛屾惡甯?TDX 鐗瑰畾鐨勫瓙 ioctl() 鍛戒护銆?
```

  /* Trust Domain Extensions 瀛?ioctl() 鍛戒护銆?*/
  enum kvm_tdx_cmd_id {
          KVM_TDX_CAPABILITIES = 0,
          KVM_TDX_INIT_VM,
          KVM_TDX_INIT_VCPU,
          KVM_TDX_INIT_MEM_REGION,
          KVM_TDX_FINALIZE_VM,
          KVM_TDX_GET_CPUID,

          KVM_TDX_CMD_NR_MAX,
  };

  struct kvm_tdx_cmd {
        /* enum kvm_tdx_cmd_id */
        __u32 id;
        /* 瀛愬懡浠ょ殑鏍囧織浣嶃€傝嫢瀛愬懡浠や笉浣跨敤锛岀疆闆躲€?*/
        __u32 flags;
        /*
         * 姣忎釜瀛愬懡浠ょ殑鏁版嵁銆傝繘绋嬭櫄鎷熷湴鍧€涓疄闄呮暟鎹殑绔嬪嵆鏁版垨鎸囬拡銆?         * 鑻ュ瓙鍛戒护涓嶄娇鐢紝缃浂銆?         */
        __u64 data;
        /*
         * 杈呭姪閿欒鐮併€傞櫎浜?-Exxx 涔嬪锛屽瓙鍛戒护杩樺彲鑳借繑鍥?TDX SEAMCALL
         * 鐨勭姸鎬佺爜銆?         */
        __u64 hw_error;
  };

```
### KVM_TDX_CAPABILITIES

:Type: vm ioctl
:Returns: 鎴愬姛杩斿洖 0锛岄敊璇繑鍥?<0

杩斿洖褰撳墠 KVM 鍦ㄧ郴缁熶腑鍔犺浇鐗瑰畾 TDX module 鍚庢墍鏀寔鐨?TDX 鑳藉姏銆傚畠鎶ュ憡鍝簺
鐗规€?鑳藉姏琚厑璁搁厤缃粰 TDX 瀹㈡埛鏈恒€?
- id: KVM_TDX_CAPABILITIES
- flags: 蹇呴』涓?0
- data: 鎸囧悜 struct kvm_tdx_capabilities 鐨勬寚閽?- hw_error: 蹇呴』涓?0

```

  struct kvm_tdx_capabilities {
        __u64 supported_attrs;
        __u64 supported_xfam;

        /* 鍒嗗埆鍦ㄥ唴鏍镐腑鎵ц骞惰浆鍙戝埌鐢ㄦ埛绌洪棿鐨?TDG.VP.VMCALL 瓒呯骇璋冪敤 */
        __u64 kernel_tdvmcallinfo_1_r11;
        __u64 user_tdvmcallinfo_1_r11;

        /* 鍒嗗埆鍦ㄥ唴鏍镐腑鎵ц骞惰浆鍙戝埌鐢ㄦ埛绌洪棿鐨?TDG.VP.VMCALL 鎸囦护鎵ц瀛愬姛鑳?*/
        __u64 kernel_tdvmcallinfo_1_r12;
        __u64 user_tdvmcallinfo_1_r12;

        __u64 reserved[250];

        /* 渚涚敤鎴风┖闂撮厤缃殑鍙厤缃?CPUID 浣?*/
        struct kvm_cpuid2 cpuid;
  };


```
### KVM_TDX_INIT_VM

:Type: vm ioctl
:Returns: 鎴愬姛杩斿洖 0锛岄敊璇繑鍥?<0

鎵ц TDX 鐗瑰畾鐨?VM 鍒濆鍖栥€傝繖闇€瑕佸湪 KVM_CREATE_VM 涔嬪悗銆佸垱寤轰换浣?VCPU 涔嬪墠璋冪敤銆?
- id: KVM_TDX_INIT_VM
- flags: 蹇呴』涓?0
- data: 鎸囧悜 struct kvm_tdx_init_vm 鐨勬寚閽?- hw_error: 蹇呴』涓?0

```

  struct kvm_tdx_init_vm {
          __u64 attributes;
          __u64 xfam;
          __u64 mrconfigid[6];          /* sha384 鎽樿 */
          __u64 mrowner[6];             /* sha384 鎽樿 */
          __u64 mrownerconfig[6];       /* sha384 鎽樿 */

          /* TD_PARAMS 涓?CPUID 涔嬪墠鐨勬€荤┖闂翠负 256 瀛楄妭 */
          __u64 reserved[12];

        /*
         * 鍦ㄥ垱寤?vcpu 涔嬪墠銆佸嵆 KVM_SET_CPUID2 涔嬪墠璋冪敤 KVM_TDX_INIT_VM銆?         * 璇ラ厤缃細鍙栦唬 VCPU 鐨?KVM_SET_CPUID2锛屽洜涓?TDX module 鐩存帴
         * 铏氭嫙鍖栭偅浜?CPUID锛岃€屼笉缁忕敱 VMM銆傜敤鎴风┖闂?VMM锛堜緥濡?qemu锛夊簲浣?         * KVM_SET_CPUID2 涓庤繖浜涘€间繚鎸佷竴鑷淬€傚鏋滀笉涓€鑷达紝KVM 鍙兘瀵瑰鎴锋満鐨?         * vCPUID 浜х敓閿欒璁よ瘑锛屽苟鍙兘閿欒鍦版ā鎷?TDX module 鏈櫄鎷熷寲鐨?         * CPUID 鎴?MSR銆?         */
          struct kvm_cpuid2 cpuid;
  };


```
### KVM_TDX_INIT_VCPU

:Type: vcpu ioctl
:Returns: 鎴愬姛杩斿洖 0锛岄敊璇繑鍥?<0

鎵ц TDX 鐗瑰畾鐨?VCPU 鍒濆鍖栥€?
- id: KVM_TDX_INIT_VCPU
- flags: 蹇呴』涓?0
- data: 瀹㈡埛鏈?TD VCPU RCX 鐨勫垵濮嬪€?- hw_error: 蹇呴』涓?0

### KVM_TDX_INIT_MEM_REGION

:Type: vcpu ioctl
:Returns: 鎴愬姛杩斿洖 0锛岄敊璇繑鍥?<0

鐢ㄦ潵鑷?@source_addr 鐨勭敤鎴风┖闂存彁渚涙暟鎹紝鍒濆鍖栦粠 @gpa 寮€濮嬬殑 @nr_pages 涓?TDX 瀹㈡埛鏈虹鏈夊唴瀛橀〉銆侤source_addr 蹇呴』鎸?PAGE_SIZE 瀵归綈銆?
娉ㄦ剰锛屽湪璋冪敤姝ゅ瓙鍛戒护涔嬪墠锛岃寖鍥?[gpa, gpa + nr_pages] 鐨勫唴瀛樺睘鎬ч渶瑕佹槸绉佹湁鐨勩€?鐢ㄦ埛绌洪棿鍙互浣跨敤 KVM_SET_MEMORY_ATTRIBUTES 鏉ヨ缃灞炴€с€?
濡傛灉鎸囧畾浜?KVM_TDX_MEASURE_MEMORY_REGION 鏍囧織锛屽畠杩樹細鎵╁睍搴﹂噺锛坢easurement锛夈€?
- id: KVM_TDX_INIT_MEM_REGION
- flags: 鐩墠浠呭畾涔変簡 KVM_TDX_MEASURE_MEMORY_REGION
- data: 鎸囧悜 struct kvm_tdx_init_mem_region 鐨勬寚閽?- hw_error: 蹇呴』涓?0

```

  #define KVM_TDX_MEASURE_MEMORY_REGION   (1UL << 0)

  struct kvm_tdx_init_mem_region {
          __u64 source_addr;
          __u64 gpa;
          __u64 nr_pages;
  };


```
### KVM_TDX_FINALIZE_VM

:Type: vm ioctl
:Returns: 鎴愬姛杩斿洖 0锛岄敊璇繑鍥?<0

瀹屾垚鍒濆 TD 鍐呭鐨勫害閲忥紝骞跺皢鍏舵爣璁颁负鍙繍琛屻€?
- id: KVM_TDX_FINALIZE_VM
- flags: 蹇呴』涓?0
- data: 蹇呴』涓?0
- hw_error: 蹇呴』涓?0


### KVM_TDX_GET_CPUID

:Type: vcpu ioctl
:Returns: 鎴愬姛杩斿洖 0锛岄敊璇繑鍥?<0

鑾峰彇 TDX module 涓?TD 瀹㈡埛鏈鸿櫄鎷熷寲鐨?CPUID 鍊笺€傚綋瀹冭繑鍥?-E2BIG 鏃讹紝鐢ㄦ埛绌洪棿
搴斿垎閰嶆洿澶х殑缂撳啿骞堕噸璇曘€傛渶灏忕紦鍐插ぇ灏忎細鍦?struct kvm_cpuid2 鐨?nent 瀛楁涓洿鏂般€?
- id: KVM_TDX_GET_CPUID
- flags: 蹇呴』涓?0
- data: 鎸囧悜 struct kvm_cpuid2 鐨勬寚閽堬紙in/out锛?- hw_error: 蹇呴』涓?0锛坥ut锛?
```

  struct kvm_cpuid2 {
	  __u32 nent;
	  __u32 padding;
	  struct kvm_cpuid_entry2 entries[0];
  };

  struct kvm_cpuid_entry2 {
	  __u32 function;
	  __u32 index;
	  __u32 flags;
	  __u32 eax;
	  __u32 ebx;
	  __u32 ecx;
	  __u32 edx;
	  __u32 padding[3];
  };

```
## KVM TDX 鍒涘缓娴佺▼


闄や簡鏍囧噯鐨?KVM 娴佺▼澶栵紝杩橀渶瑕佽皟鐢ㄦ柊鐨?TDX ioctl銆傛帶鍒舵祦濡備笅锛?
#. 妫€鏌ョ郴缁熺骇鑳藉姏

   - KVM_CAP_VM_TYPES锛氭鏌?VM 绫诲瀷鏄惁鍙楁敮鎸侊紝浠ュ強 KVM_X86_TDX_VM 鏄惁鍙楁敮鎸併€?
#. 鍒涘缓 VM

   - KVM_CREATE_VM
   - KVM_TDX_CAPABILITIES锛氭煡璇㈢敤浜庡垱寤?TDX 瀹㈡埛鏈虹殑鑳藉姏銆?   - KVM_CHECK_EXTENSION(KVM_CAP_MAX_VCPUS)锛氭煡璇?TD 鍦?VM 绾у埆鍙敮鎸佺殑鏈€澶?VCPU
     鏁伴噺锛圱DX 瀵规鏈夎嚜韬檺鍒讹級銆?   - KVM_SET_TSC_KHZ锛氬鏋滃笇鏈涗娇鐢ㄤ笌瀹夸富涓嶅悓鐨?TSC 棰戠巼锛屽垯閰嶇疆 TD 鐨?TSC 棰戠巼銆?     杩欐槸鍙€夌殑銆?   - KVM_TDX_INIT_VM锛氫紶鍏?TDX 鐗瑰畾鐨?VM 鍙傛暟銆?
#. 鍒涘缓 VCPU

   - KVM_CREATE_VCPU
   - KVM_TDX_INIT_VCPU锛氫紶鍏?TDX 鐗瑰畾鐨?VCPU 鍙傛暟銆?   - KVM_SET_CPUID2锛氶厤缃?TD 鐨?CPUID銆?   - KVM_SET_MSRS锛氶厤缃?TD 鐨?MSR銆?
#. 鍒濆鍖栧垵濮嬪鎴锋満鍐呭瓨

   - 鍑嗗鍒濆瀹㈡埛鏈哄唴瀛樼殑鍐呭銆?   - KVM_TDX_INIT_MEM_REGION锛氭坊鍔犲垵濮嬪鎴锋満鍐呭瓨銆?   - KVM_TDX_FINALIZE_VM锛氬畬鎴?TDX 瀹㈡埛鏈虹殑搴﹂噺銆?
#. 杩愯 VCPU

## 鍙傝€?

https://www.intel.com/content/www/us/en/developer/tools/trust-domain-extensions/documentation.html
