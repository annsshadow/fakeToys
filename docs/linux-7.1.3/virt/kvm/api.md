
## KVM锛堝熀浜庡唴鏍哥殑铏氭嫙鏈猴級API 鏉冨▉鏂囨。


## 1. General description


kvm API 鍥寸粫鍙互鍙戦€佺粰鍚勭被鏂囦欢鎻忚堪绗︾殑涓嶅悓绉嶇被鐨?ioctl 鏋勫缓銆傛渶鍒濈殑涓€娆?open("/dev/kvm") 鑾峰彇涓€涓寚鍚?kvm 瀛愮郴缁熺殑鍙ユ焺锛涜鍙ユ焺鍙敤浜庡彂鍑虹郴缁?ioctl銆傚湪姝ゅ彞鏌勪笂鎵ц KVM_CREATE_VM ioctl 灏嗗垱寤轰竴涓?VM 鏂囦欢鎻忚堪绗︼紝鍙敤浜庡彂鍑?VM ioctl銆傚湪 VM fd 涓婃墽琛?KVM_CREATE_VCPU 鎴?KVM_CREATE_DEVICE ioctl 灏嗗垱寤轰竴涓櫄鎷?cpu 鎴栬澶囷紝骞惰繑鍥炴寚鍚戞柊璧勬簮鐨勬枃浠舵弿杩扮銆?

鎹㈠彞璇濊锛宬vm API 鏄竴缁勫彂閫佺粰涓嶅悓绉嶇被鏂囦欢鎻忚堪绗︾殑 ioctl锛岀敤浜庢帶鍒惰櫄鎷熸満鐨勫悇涓柟闈€傛牴鎹帴鍙楀畠浠殑鏂囦欢鎻忚堪绗︼紝ioctl 灞炰簬浠ヤ笅绫诲埆锛?

 - System ioctls: These query and set global attributes which affect the
   whole kvm subsystem.  In addition a system ioctl is used to create
   virtual machines.

 - VM ioctls: These query and set attributes that affect an entire virtual
   machine, for example memory layout.  In addition a VM ioctl is used to
   create virtual cpus (vcpus) and devices.

   VM ioctls must be issued from the same process (address space) that was
   used to create the VM.

 - vcpu ioctls: These query and set attributes that control the operation
   of a single virtual cpu.

   vcpu ioctls should be issued from the same thread that was used to create
   the vcpu, except for asynchronous vcpu ioctl that are marked as such in
   the documentation.  Otherwise, the first ioctl after switching threads
   could see a performance impact.

 - device ioctls: These query and set attributes that control the operation
   of a single device.

   device ioctls must be issued from the same process (address space) that
   was used to create the VM.

铏界劧澶у鏁?ioctl 鏄壒瀹氫簬鏌愪竴绉嶆枃浠舵弿杩扮鐨勶紝浣嗗湪鏌愪簺鎯呭喌涓嬶紝鍚屼竴涓?ioctl 鍙互灞炰簬澶氫釜绫诲埆銆?

KVM API 鏄殢鐫€鏃堕棿鎺ㄧЩ鎴愰暱璧锋潵鐨勩€傚洜姝わ紝KVM 瀹氫箟浜嗚澶氬舰濡?`KVM_CAP_*` 鐨勫父閲忥紝姣忎釜瀵瑰簲鐢变竴涓垨澶氫釜 ioctl 鎻愪緵鐨勪竴缁勫姛鑳姐€傝繖浜?鑳藉姏"锛坈apabilities锛夌殑鍙敤鎬у彲浠ラ€氳繃 KVM_CHECK_EXTENSION <KVM_CHECK_EXTENSION> 鏉ユ鏌ャ€傚浜庡笇鏈涜幏寰楀叾鍔熻兘鐨?VM 鎴?VCPU锛屾煇浜涜兘鍔涜繕闇€瑕佽鍚敤锛堝弬瑙?cap_enable 鍜?cap_enable_vm锛夈€?


## 2. Restrictions


涓€鑸€岃█锛屾枃浠舵弿杩扮鍙互閫氳繃 fork() 鍜?unix 鍩熷鎺ュ瓧鐨?SCM_RIGHTS 璁炬柦鍦ㄨ繘绋嬮棿杩佺Щ銆傝繖绫绘妧宸ф槑纭笉鍙?kvm 鏀寔銆傝櫧鐒跺畠浠笉浼氬瀹夸富鏈洪€犳垚鎹熷锛屼絾鍏跺疄闄呰涓轰笉琚?API 淇濊瘉銆傛湁鍏?KVM 鏀寔鐨?ioctl 浣跨敤妯″瀷璇︽儏锛岃鍙傞槄"General description"銆?

闇€瑕佹敞鎰忕殑鏄紝灏界 VM ioctl 鍙兘浠庡垱寤鸿 VM 鐨勮繘绋嬪彂鍑猴紝浣?VM 鐨勭敓鍛藉懆鏈熶笌鍏舵枃浠舵弿杩扮鐩稿叧鑱旓紝鑰岄潪涓庡叾鍒涘缓鑰咃紙杩涚▼锛夌浉鍏宠仈銆傛崲鍙ヨ瘽璇达紝VM 鍙婂叾璧勬簮锛?*鍖呮嫭鍏宠仈鐨勫湴鍧€绌洪棿**锛夊湪瀵硅 VM 鏂囦欢鎻忚堪绗︾殑鏈€鍚庝竴涓紩鐢ㄨ閲婃斁涔嬪墠涓嶄細琚噴鏀俱€備緥濡傦紝濡傛灉鍦?ioctl(KVM_CREATE_VM) 涔嬪悗鎵ц fork()锛屽垯璇?VM 鍦ㄧ埗锛堝師濮嬶級杩涚▼鍙婂叾瀛愯繘绋嬮兘閲婃斁浜嗗畠浠 VM 鏂囦欢鎻忚堪绗︾殑寮曠敤涔嬪墠涓嶄細琚噴鏀俱€?

鐢变簬 VM 鐨勮祫婧愬湪鍏舵枃浠舵弿杩扮鐨勬渶鍚庝竴涓紩鐢ㄨ閲婃斁涔嬪墠涓嶄細琚噴鏀撅紝鍥犳寮虹儓涓嶅缓璁湪鏈粩缁嗚€冭檻鐨勬儏鍐典笅閫氳繃 fork()銆乨up() 绛夋柟寮忓垱寤哄 VM 鐨勯澶栧紩鐢紝杩欏彲鑳戒細浜х敓涓嶅笇鏈涚殑鍓綔鐢紝渚嬪 VM 鍏抽棴鏃讹紝鐢?VM 杩涚▼鍙婂叾浠ｈ〃鍒嗛厤鐨勫唴瀛樺彲鑳戒笉浼氳閲婃斁/璁拌处銆?


## 3. Extensions


鑷?Linux 2.6.22 璧凤紝KVM ABI 宸茬粡绋冲畾锛氫笉鍏佽浠讳綍涓嶅悜鍚庡吋瀹圭殑鍙樻洿銆傜劧鑰岋紝瀛樺湪涓€涓墿灞曡鏂斤紝鍏佽鏌ヨ鍜屼娇鐢ㄥ API 鐨勫悜鍚庡吋瀹规墿灞曘€?

鎵╁睍鏈哄埗骞堕潪鍩轰簬 Linux 鐗堟湰鍙枫€傜浉鍙嶏紝kvm 瀹氫箟鎵╁睍鏍囪瘑绗︼紝骞舵彁渚涗竴涓鏂芥潵鏌ヨ鏌愪釜鐗瑰畾鐨勬墿灞曟爣璇嗙鏄惁鍙敤銆傚鏋滃彲鐢紝鍒欐湁涓€缁?ioctl 鍙緵搴旂敤绋嬪簭浣跨敤銆?


## 4. API description


鏈妭鎻忚堪鍙敤浜庢帶鍒?kvm 瀹㈡埛鏈虹殑 ioctl銆傚浜庢瘡涓?ioctl锛岄櫎鎻忚堪澶栬繕鎻愪緵浠ヤ笅淇℃伅锛?

  Capability:
      which KVM extension provides this ioctl.  Can be 'basic',
      which means that is will be provided by any kernel that supports
      API version 12 (see KVM_GET_API_VERSION <KVM_GET_API_VERSION>),
      or a KVM_CAP_xyz constant that can be checked with
      KVM_CHECK_EXTENSION <KVM_CHECK_EXTENSION>.

  Architectures:
      which instruction set architectures provide this ioctl.
      x86 includes both i386 and x86_64.

  Type:
      system, vm, or vcpu.

  Parameters:
      what parameters are accepted by the ioctl.

  Returns:
      the return value.  General error numbers (EBADF, ENOMEM, EINVAL)
      are not detailed, but errors with specific meanings are.



### 4.1 KVM_GET_API_VERSION



:Capability: basic
:Architectures: all
:Type: system ioctl
:Parameters: none
:Returns: the constant KVM_API_VERSION (=12)

杩欎細灏?API 鐗堟湰鏍囪瘑涓虹ǔ瀹氱殑 kvm API銆傞璁¤鏁板瓧涓嶄細鍙樺寲銆備笉杩囷紝Linux 2.6.20 鍜?2.6.21 鎶ュ憡鐨勬槸鏇存棭鐨勭増鏈紱杩欎簺鐗堟湰娌℃湁鏂囨。涓斾笉鍙楁敮鎸併€傚鏋?KVM_GET_API_VERSION 杩斿洖鐨勫€间笉鏄?12锛屽簲鐢ㄧ▼搴忓簲褰撴嫆缁濊繍琛屻€傚鏋滄椤规鏌ラ€氳繃锛屾墍鏈夎鎻忚堪涓?'basic' 鐨?ioctl 閮藉皢鍙敤銆?


### 4.2 KVM_CREATE_VM



:Capability: basic
:Architectures: all
:Type: system ioctl
:Parameters: machine type identifier (KVM_VM_*)
:Returns: a VM fd that can be used to control the new virtual machine.

鏂?VM 娌℃湁铏氭嫙 cpu锛屼篃娌℃湁鍐呭瓨銆備綘鍙兘甯屾湜灏?0 鐢ㄤ綔鏈哄櫒绫诲瀷銆?

##### X86:



鍙楁敮鎸佺殑 X86 VM 绫诲瀷鍙互閫氳繃 KVM_CAP_VM_TYPES 鏌ヨ銆?

##### S390:



涓轰簡鍦?S390 涓婂垱寤虹敤鎴锋帶鍒剁殑铏氭嫙鏈猴紝璇锋鏌?KVM_CAP_S390_UCONTROL锛屽苟浠ョ壒鏉冪敤鎴凤紙CAP_SYS_ADMIN锛変娇鐢ㄦ爣蹇?KVM_VM_S390_UCONTROL銆?

##### MIPS:



瑕佸湪 MIPS 涓婁娇鐢ㄧ‖浠惰緟鍔╄櫄鎷熷寲锛圴Z ASE锛夛紝鑰岄潪榛樿鐨勯櫡鍏ュ苟妯℃嫙锛坱rap & emulate锛夊疄鐜帮紙璇ュ疄鐜颁細鏀瑰彉铏氭嫙鍐呭瓨甯冨眬浠ラ€傞厤鐢ㄦ埛妯″紡锛夛紝璇锋鏌?KVM_CAP_MIPS_VZ 骞朵娇鐢ㄦ爣蹇?KVM_VM_MIPS_VZ銆?

##### ARM64:



鍦?arm64 涓婏紝VM 鐨勭墿鐞嗗湴鍧€澶у皬锛圛PA 澶у皬闄愬埗锛夐粯璁ら檺鍒朵负 40 浣嶃€傚鏋滃涓绘満鏀寔 KVM_CAP_ARM_VM_IPA_SIZE 鎵╁睍锛岃闄愬埗鍙厤缃€傚彈鏀寔鏃讹紝浣跨敤 KVM_VM_TYPE_ARM_IPA_SIZE(IPA_Bits) 鍦ㄦ満鍣ㄧ被鍨嬫爣璇嗙涓缃ぇ灏忥紝鍏朵腑 IPA_Bits 鏄?VM 浣跨敤鐨勪换浣曠墿鐞嗗湴鍧€鐨勬渶澶у搴︺€侷PA_Bits 琚紪鐮佸湪鏈哄櫒绫诲瀷鏍囪瘑绗︾殑 bits[7-0] 涓€?

```
    vm_fd = ioctl(dev_fd, KVM_CREATE_VM, KVM_VM_TYPE_ARM_IPA_SIZE(48));
```

鎵€璇锋眰鐨勫ぇ灏忥紙IPA_Bits锛夊繀椤绘弧瓒筹細

 ==   =========================================================
  0   Implies default size, 40bits (for backward compatibility)
  N   Implies N bits, where N is a positive integer such that,
      32 <= N <= Host_IPA_Limit
 ==   =========================================================

Host_IPA_Limit 鏄涓绘満涓?IPA_Bits 鍙兘鐨勬渶澶у€硷紝鍙栧喅浜?CPU 鑳藉姏鍜屽唴鏍搁厤缃€傝闄愬埗鍙互閫氳繃杩愯鏃惰皟鐢?KVM_CHECK_EXTENSION ioctl() 鐨?KVM_CAP_ARM_VM_IPA_SIZE 鑾峰彇銆?

濡傛灉鎵€璇锋眰鐨?IPA 澶у皬锛堟棤璁烘槸闅愬紡杩樻槸鏄惧紡锛夊湪瀹夸富鏈轰笂涓嶅彈鏀寔锛孷M 鐨勫垱寤哄皢澶辫触銆?

璇锋敞鎰忥紝閰嶇疆 IPA 澶у皬涓嶄細褰卞搷瀹㈡埛鏈?CPU 鍦?ID_AA64MMFR0_EL1[PARange] 涓毚闇茬殑鑳藉姏銆傚畠鍙奖鍝嶇敱 stage2 绾у埆锛堝鎴锋満鐗╃悊鍦板潃鍒板涓绘満鐗╃悊鍦板潃杞崲锛夋墍杞崲鐨勫湴鍧€澶у皬銆?


### 4.3 KVM_GET_MSR_INDEX_LIST, KVM_GET_MSR_FEATURE_INDEX_LIST



:Capability: basic, KVM_CAP_GET_MSR_FEATURES for KVM_GET_MSR_FEATURE_INDEX_LIST
:Architectures: x86
:Type: system ioctl
:Parameters: struct kvm_msr_list (in/out)
:Returns: 0 on success; -1 on error

閿欒锛?

  ======     ============================================================
  EFAULT     msr 绱㈠紩鍒楄〃鏃犳硶琚鍙栨垨鍐欏叆
  E2BIG      msr 绱㈠紩鍒楄〃澶ぇ锛屾棤娉曟斁鍏ョ敤鎴锋寚瀹氱殑鏁扮粍涓?
  ======     ============================================================

```
  struct kvm_msr_list {
	__u32 nmsrs; /* number of msrs in entries */
	__u32 indices[0];
  };
```

鐢ㄦ埛鐢?nmsrs 濉叆 indices 鏁扮粍鐨勫ぇ灏忥紝浣滀负鍥炴姤 kvm 璋冩暣 nmsrs 浠ュ弽鏄犲疄闄呯殑 msr 鏁伴噺锛屽苟鐢ㄥ叾缂栧彿濉厖 indices 鏁扮粍銆?

KVM_GET_MSR_INDEX_LIST 杩斿洖鍙楁敮鎸佺殑瀹㈡埛鏈?msr銆傝鍒楄〃闅?kvm 鐗堟湰鍜屽涓绘満澶勭悊鍣ㄨ€屽彉锛岄櫎姝や箣澶栦笉浼氭敼鍙樸€?

娉ㄦ剰锛氬鏋?kvm 琛ㄦ槑鏀寔 MCE锛圞VM_CAP_MCE锛夛紝鍒?MCE bank MSR 涓嶄細鍦?MSR 鍒楄〃涓繑鍥烇紝鍥犱负涓嶅悓鐨?vcpu 鍙兘鎷ユ湁涓嶅悓鏁伴噺鐨?bank锛岃繖閫氳繃 KVM_X86_SETUP_MCE ioctl 璁剧疆銆?

KVM_GET_MSR_FEATURE_INDEX_LIST 杩斿洖鍙互浼犻€掔粰 KVM_GET_MSRS 绯荤粺 ioctl 鐨?MSR 鍒楄〃銆傝繖璁╃敤鎴风┖闂磋兘澶熸帰娴嬮€氳繃 MSR 鏆撮湶鐨勫涓绘満鑳藉姏鍙婂鐞嗗櫒鐗规€э紙渚嬪 VMX 鑳藉姏锛夈€傝鍒楄〃涔熼殢 kvm 鐗堟湰鍜屽涓绘満澶勭悊鍣ㄨ€屽彉锛岄櫎姝や箣澶栦笉浼氭敼鍙樸€?



### 4.4 KVM_CHECK_EXTENSION



:Capability: basic, KVM_CAP_CHECK_EXTENSION_VM for vm ioctl
:Architectures: all
:Type: system ioctl, vm ioctl
:Parameters: extension identifier (KVM_CAP_*)
:Returns: 0 if unsupported; 1 (or some other positive integer) if supported

璇?API 鍏佽搴旂敤绋嬪簭鏌ヨ鏍稿績 kvm API 鐨勬墿灞曘€傜敤鎴风┖闂翠紶閫掍竴涓墿灞曟爣璇嗙锛堟暣鏁帮級骞舵帴鏀朵竴涓弿杩版墿灞曞彲鐢ㄦ€х殑鏁存暟銆傞€氬父 0 琛ㄧず鍚︼紝1 琛ㄧず鏄紝浣嗘煇浜涙墿灞曞彲鑳藉湪鏁存暟杩斿洖鍊间腑鎶ュ憡棰濆淇℃伅銆?

鏍规嵁鍏跺垵濮嬪寲鏂瑰紡锛屼笉鍚岀殑 VM 鍙兘鍏锋湁涓嶅悓鐨勮兘鍔涖€傚洜姝ゅ缓璁娇鐢?vm ioctl 鏉ユ煡璇㈣兘鍔涳紙鍦?vm fd 涓婇€氳繃 KVM_CAP_CHECK_EXTENSION_VM 鍙敤锛夈€?

### 4.5 KVM_GET_VCPU_MMAP_SIZE



:Capability: basic
:Architectures: all
:Type: system ioctl
:Parameters: none
:Returns: size of vcpu mmap area, in bytes

KVM_RUN ioctl锛堝弬瑙佸墠鏂囷級閫氳繃鍏变韩鍐呭瓨鍖哄煙涓庣敤鎴风┖闂撮€氫俊銆傝 ioctl 杩斿洖璇ュ尯鍩熺殑澶у皬銆傝鎯呰鍙傞槄 KVM_RUN 鏂囨。銆?

闄や簡 KVM_RUN 閫氫俊鍖哄煙鐨勫ぇ灏忓锛孷CPU 鏂囦欢鎻忚堪绗︾殑鍏朵粬鍖哄煙涔熷彲浠ヨ mmap锛屽寘鎷細

- if KVM_CAP_COALESCED_MMIO is available, a page at
  KVM_COALESCED_MMIO_PAGE_OFFSET * PAGE_SIZE; for historical reasons,
  this page is included in the result of KVM_GET_VCPU_MMAP_SIZE.
  KVM_CAP_COALESCED_MMIO is not documented yet.

- if KVM_CAP_DIRTY_LOG_RING is available, a number of pages at
  KVM_DIRTY_LOG_PAGE_OFFSET * PAGE_SIZE.  For more information on
  KVM_CAP_DIRTY_LOG_RING, see KVM_CAP_DIRTY_LOG_RING.


### 4.7 KVM_CREATE_VCPU



:Capability: basic
:Architectures: all
:Type: vm ioctl
:Parameters: vcpu id (apic id on x86)
:Returns: vcpu fd on success, -1 on error

璇?API 鍚戣櫄鎷熸満娣诲姞涓€涓?vcpu銆傛坊鍔犳暟閲忎笉寰楄秴杩?max_vcpus銆倂cpu id 鏄寖鍥?[0, max_vcpu_id) 鍐呯殑鏁存暟銆?

寤鸿鐨?max_vcpus 鍊煎彲浠ラ€氳繃杩愯鏃惰皟鐢?KVM_CHECK_EXTENSION ioctl() 鐨?KVM_CAP_NR_VCPUS 鑾峰彇銆俶ax_vcpus 鍙兘鐨勬渶澶у€煎彲浠ラ€氳繃杩愯鏃惰皟鐢?KVM_CHECK_EXTENSION ioctl() 鐨?KVM_CAP_MAX_VCPUS 鑾峰彇銆?

濡傛灉 KVM_CAP_NR_VCPUS 涓嶅瓨鍦紝浣犲簲褰撳亣瀹?max_vcpus 鏈€澶氫负 4 涓?cpu銆傚鏋?KVM_CAP_MAX_VCPUS 涓嶅瓨鍦紝浣犲簲褰撳亣瀹?max_vcpus 涓?KVM_CAP_NR_VCPUS 杩斿洖鐨勫€肩浉鍚屻€?

max_vcpu_id 鍙兘鐨勬渶澶у€煎彲浠ラ€氳繃杩愯鏃惰皟鐢?KVM_CHECK_EXTENSION ioctl() 鐨?KVM_CAP_MAX_VCPU_ID 鑾峰彇銆?

濡傛灉 KVM_CAP_MAX_VCPU_ID 涓嶅瓨鍦紝浣犲簲褰撳亣瀹?max_vcpu_id 涓?KVM_CAP_MAX_VCPUS 杩斿洖鐨勫€肩浉鍚屻€?

鍦ㄤ娇鐢?book3s_hv 妯″紡鐨?powerpc 涓婏紝vcpu 琚槧灏勫埌鐢变竴涓垨澶氫釜铏氭嫙 CPU 鏍哥粍鎴愮殑铏氭嫙绾跨▼涓€傦紙杩欐槸鍥犱负纭欢瑕佹眰涓€涓?CPU 鏍镐腑鐨勬墍鏈夌‖浠剁嚎绋嬮兘澶勪簬鍚屼竴鍒嗗尯涓€傦級KVM_CAP_PPC_SMT 鑳藉姏琛ㄧず姣忎釜铏氭嫙鏍革紙vcore锛夌殑 vcpu 鏁伴噺銆倂core id 鐢?vcpu id 闄や互姣忎釜 vcore 鐨?vcpu 鏁伴噺寰楀埌銆傜粰瀹?vcore 涓殑 vcpu 濮嬬粓褰兼浣嶄簬鍚屼竴鐗╃悊鏍镐腑锛堝敖绠″彲鑳介殢鏃堕棿鍒囨崲鍒颁笉鍚岀殑鐗╃悊鏍革級銆傜敤鎴风┖闂村彲浠ラ€氳繃鍒嗛厤 vcpu id 鏉ユ帶鍒跺鎴锋満鐨勭嚎绋嬶紙SMT锛夋ā寮忋€備緥濡傦紝濡傛灉鐢ㄦ埛绌洪棿甯屾湜瀹㈡埛鏈?vcpu 鏄崟绾跨▼鐨勶紝瀹冨簲褰撲娇鎵€鏈?vcpu id 閮芥槸姣忎釜 vcore 鐨?vcpu 鏁伴噺鐨勫€嶆暟銆?

瀵逛簬浣跨敤 S390 鐢ㄦ埛鎺у埗铏氭嫙鏈哄垱寤虹殑铏氭嫙 cpu锛屽緱鍒扮殑 vcpu fd 鍙互鍦ㄩ〉鍋忕Щ KVM_S390_SIE_PAGE_OFFSET 澶勮繘琛屽唴瀛樻槧灏勶紝浠ヨ幏鍙栬櫄鎷?cpu 纭欢鎺у埗鍧楃殑鍐呭瓨鏄犲皠銆?


### 4.8 KVM_GET_DIRTY_LOG



:Capability: basic
:Architectures: all
:Type: vm ioctl
:Parameters: struct kvm_dirty_log (in/out)
:Returns: 0 on success, -1 on error

```
  /* for KVM_GET_DIRTY_LOG */
  struct kvm_dirty_log {
	__u32 slot;
	__u32 padding;
	union {
		void __user *dirty_bitmap; /* one bit per page */
		__u64 padding;
	};
  };
```

缁欏畾涓€涓唴瀛樻Ы锛岃繑鍥炰竴涓綅鍥撅紝鍖呭惈鑷笂娆¤皟鐢ㄨ ioctl 浠ユ潵琚紕鑴忕殑鎵€鏈夐〉銆傜 0 浣嶅搴斿唴瀛樻Ы涓殑绗竴椤点€傝纭繚鏁翠釜缁撴瀯浣撹娓呴浂锛屼互閬垮厤濉厖闂銆?

濡傛灉 KVM_CAP_MULTI_ADDRESS_SPACE 鍙敤锛宻lot 瀛楁鐨?16-31 浣嶆寚瀹氫簡浣犳兂瑕佽繑鍥炶剰浣嶅浘鐨勫湴鍧€绌洪棿銆傛湁鍏?slot 瀛楁鐢ㄦ硶鐨勮鎯咃紝璇峰弬闃?KVM_SET_USER_MEMORY_REGION銆?

鑴忎綅鍥句腑鐨勪綅浼氬湪 ioctl 杩斿洖涔嬪墠琚竻闆讹紝闄ら潪鍚敤浜?KVM_CAP_MANUAL_DIRTY_LOG_PROTECT2銆傛洿澶氫俊鎭鍙傞槄璇ヨ兘鍔涚殑鎻忚堪銆?

娉ㄦ剰锛孹en shared_info 椤碉紙濡傛灉宸查厤缃級搴斿缁堣瑙嗕负鑴忛〉銆侹VM 涓嶄細鏄惧紡鍦板皢鍏舵爣璁颁负鑴忋€?


### 4.10 KVM_RUN



:Capability: basic
:Architectures: all
:Type: vcpu ioctl
:Parameters: none
:Returns: 0 on success, -1 on error

閿欒锛?

  =======    ==============================================================
  EINTR      an unmasked signal is pending
  ENOEXEC    the vcpu hasn't been initialized or the guest tried to execute
             instructions from device memory (arm64)
  ENOSYS     data abort outside memslots with no syndrome info and
             KVM_CAP_ARM_NISV_TO_USER not enabled (arm64)
  EPERM      SVE feature set but not finalized (arm64)
  =======    ==============================================================

璇?ioctl 鐢ㄤ簬杩愯涓€涓鎴锋満铏氭嫙 cpu銆傝櫧鐒舵病鏈夋樉寮忓弬鏁帮紝浣嗗瓨鍦ㄤ竴涓殣寮忓弬鏁板潡锛屽彲浠ラ€氳繃浠?KVM_GET_VCPU_MMAP_SIZE 缁欏畾澶у皬瀵?vcpu fd 鍦ㄥ亸绉?0 澶勮繘琛?mmap() 鑾峰緱銆傝鍙傛暟鍧楄鏍煎紡鍖栦负 'struct kvm_run'锛堣涓嬫枃锛夈€?


### 4.11 KVM_GET_REGS



:Capability: basic
:Architectures: all except arm64
:Type: vcpu ioctl
:Parameters: struct kvm_regs (out)
:Returns: 0 on success, -1 on error

浠?vcpu 璇诲彇閫氱敤瀵勫瓨鍣ㄣ€?

```
  /* x86 */
  struct kvm_regs {
	/* out (KVM_GET_REGS) / in (KVM_SET_REGS) */
	__u64 rax, rbx, rcx, rdx;
	__u64 rsi, rdi, rsp, rbp;
	__u64 r8,  r9,  r10, r11;
	__u64 r12, r13, r14, r15;
	__u64 rip, rflags;
  };

  /* mips */
  struct kvm_regs {
	/* out (KVM_GET_REGS) / in (KVM_SET_REGS) */
	__u64 gpr[32];
	__u64 hi;
	__u64 lo;
	__u64 pc;
  };

  /* LoongArch */
  struct kvm_regs {
	/* out (KVM_GET_REGS) / in (KVM_SET_REGS) */
	unsigned long gpr[32];
	unsigned long pc;
  };


```
### 4.12 KVM_SET_REGS



:Capability: basic
:Architectures: all except arm64
:Type: vcpu ioctl
:Parameters: struct kvm_regs (in)
:Returns: 0 on success, -1 on error

灏嗛€氱敤瀵勫瓨鍣ㄥ啓鍏?vcpu銆?

See KVM_GET_REGS for the data structure.


### 4.13 KVM_GET_SREGS



:Capability: basic
:Architectures: x86, ppc
:Type: vcpu ioctl
:Parameters: struct kvm_sregs (out)
:Returns: 0 on success, -1 on error

浠?vcpu 璇诲彇鐗规畩瀵勫瓨鍣ㄣ€?

```
  /* x86 */
  struct kvm_sregs {
	struct kvm_segment cs, ds, es, fs, gs, ss;
	struct kvm_segment tr, ldt;
	struct kvm_dtable gdt, idt;
	__u64 cr0, cr2, cr3, cr4, cr8;
	__u64 efer;
	__u64 apic_base;
	__u64 interrupt_bitmap[(KVM_NR_INTERRUPTS + 63) / 64];
  };

  /* ppc -- see arch/powerpc/include/uapi/asm/kvm.h */

```

interrupt_bitmap 鏄寕璧峰閮ㄤ腑鏂殑浣嶅浘銆傛渶澶氬彧鑳借缃竴浣嶃€傝涓柇宸茶 APIC 纭锛屼絾灏氭湭琚敞鍏ュ埌 cpu 鏍镐腑銆?


### 4.14 KVM_SET_SREGS



:Capability: basic
:Architectures: x86, ppc
:Type: vcpu ioctl
:Parameters: struct kvm_sregs (in)
:Returns: 0 on success, -1 on error

灏嗙壒娈婂瘎瀛樺櫒鍐欏叆 vcpu銆係ee KVM_GET_SREGS for the data structures.


### 4.15 KVM_TRANSLATE



:Capability: basic
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_translation (in/out)
:Returns: 0 on success, -1 on error

鏍规嵁 vcpu 褰撳墠鍦板潃杞崲妯″紡缈昏瘧涓€涓櫄鎷熷湴鍧€銆?

```
  struct kvm_translation {
	/* in */
	__u64 linear_address;

	/* out */
	__u64 physical_address;
	__u8  valid;
	__u8  writeable;
	__u8  usermode;
	__u8  pad[5];
  };


```
### 4.16 KVM_INTERRUPT



:Capability: basic
:Architectures: x86, ppc, mips, riscv, loongarch
:Type: vcpu ioctl
:Parameters: struct kvm_interrupt (in)
:Returns: 0 on success, negative on failure.

灏嗗緟娉ㄥ叆鐨勭‖浠朵腑鏂悜閲忔帓鍏ラ槦鍒椼€?

```
  /* for KVM_INTERRUPT */
  struct kvm_interrupt {
	/* in */
	__u32 irq;
  };
```

##### X86:



:Returns:

	========= ===================================
	  0       on success,
	 -EEXIST  if an interrupt is already enqueued
	 -EINVAL  the irq number is invalid
	 -ENXIO   if the PIC is in the kernel
	 -EFAULT  if the pointer is invalid
	========= ===================================

娉ㄦ剰锛?irq' 鏄腑鏂悜閲忥紝鑰岄潪涓柇寮曡剼鎴栫嚎璺€傚鏋滄湭浣跨敤鍐呮牳鎬?PIC锛岃 ioctl 寰堟湁鐢ㄣ€?

##### PPC:



灏嗗緟娉ㄥ叆鐨勫閮ㄤ腑鏂帓鍏ラ槦鍒椼€傝 ioctl 琚噸杞戒负 3 涓笉鍚岀殑 irq 鍊硷細

a) KVM_INTERRUPT_SET

   涓€鏃﹀鎴锋満鍑嗗濂芥帴鏀朵腑鏂紝灏卞皢杈规部鍨嬪閮ㄤ腑鏂敞鍏ュ埌瀹㈡埛鏈轰腑銆傛敞鍏ュ悗锛屼腑鏂嵆瀹屾垚銆?

b) KVM_INTERRUPT_UNSET

   杩欎細鍙栨秷浠讳綍鎸傝捣鐨勪腑鏂€?

   Only available with KVM_CAP_PPC_UNSET_IRQ.

c) KVM_INTERRUPT_SET_LEVEL

   杩欏皢鐢靛钩鍨嬪閮ㄤ腑鏂敞鍏ュ埌瀹㈡埛鏈轰笂涓嬫枃涓€備腑鏂繚鎸佹寕璧凤紝鐩村埌瑙﹀彂甯︽湁 KVM_INTERRUPT_UNSET 鐨勭壒瀹?ioctl銆?

   Only available with KVM_CAP_PPC_IRQ_LEVEL.

娉ㄦ剰锛岄櫎涓婅堪澹版槑鐨勫€间箣澶栫殑浠讳綍 'irq' 鍊奸兘鏄棤鏁堢殑锛屽苟浼氬鑷存剰澶栬涓恒€?

This is an asynchronous vcpu ioctl and can be invoked from any thread.

##### MIPS:



灏嗗緟娉ㄥ叆铏氭嫙 CPU 鐨勫閮ㄤ腑鏂帓鍏ラ槦鍒椼€傝礋鐨?interrupt 鍙蜂細灏嗕腑鏂嚭闃熴€?

This is an asynchronous vcpu ioctl and can be invoked from any thread.

##### RISC-V:



灏嗗緟娉ㄥ叆铏氭嫙 CPU 鐨勫閮ㄤ腑鏂帓鍏ラ槦鍒椼€傝 ioctl 琚噸杞戒负 2 涓笉鍚岀殑 irq 鍊硷細

a) KVM_INTERRUPT_SET

   杩欎负铏氭嫙 CPU 璁剧疆澶栭儴涓柇锛屽畠灏嗗湪灏辩华鍚庢帴鏀躲€?

b) KVM_INTERRUPT_UNSET

   杩欎細娓呴櫎铏氭嫙 CPU 鐨勬寕璧峰閮ㄤ腑鏂€?

This is an asynchronous vcpu ioctl and can be invoked from any thread.

##### LOONGARCH:



灏嗗緟娉ㄥ叆铏氭嫙 CPU 鐨勫閮ㄤ腑鏂帓鍏ラ槦鍒椼€傝礋鐨?interrupt 鍙蜂細灏嗕腑鏂嚭闃熴€?

This is an asynchronous vcpu ioctl and can be invoked from any thread.


### 4.18 KVM_GET_MSRS



:Capability: basic (vcpu), KVM_CAP_GET_MSR_FEATURES (system)
:Architectures: x86
:Type: system ioctl, vcpu ioctl
:Parameters: struct kvm_msrs (in/out)
:Returns: number of msrs successfully returned;
          -1 on error

褰撶敤浣滅郴缁?ioctl 鏃讹細璇诲彇 VM 鍙敤鐨勫熀浜?MSR 鐨勭壒鎬х殑鍊笺€傝繖绫讳技浜?KVM_GET_SUPPORTED_CPUID锛屼絾瀹冭繑鍥?MSR 绱㈠紩鍜屽€笺€傚熀浜?MSR 鐨勭壒鎬у垪琛ㄥ彲浠ラ€氳繃绯荤粺 ioctl 涓殑 KVM_GET_MSR_FEATURE_INDEX_LIST 鑾峰彇銆?

褰撶敤浣?vcpu ioctl 鏃讹細浠?vcpu 璇诲彇妯″瀷鐗瑰畾瀵勫瓨鍣ㄣ€傚彈鏀寔鐨?msr 绱㈠紩鍙互閫氳繃绯荤粺 ioctl 涓殑 KVM_GET_MSR_INDEX_LIST 鑾峰彇銆?

```
  struct kvm_msrs {
	__u32 nmsrs; /* number of msrs in entries */
	__u32 pad;

	struct kvm_msr_entry entries[0];
  };

  struct kvm_msr_entry {
	__u32 index;
	__u32 reserved;
	__u64 data;
  };
```

搴旂敤绋嬪簭浠ｇ爜搴旇缃?'nmsrs' 鎴愬憳锛堣〃绀?entries 鏁扮粍鐨勫ぇ灏忥級浠ュ強姣忎釜鏁扮粍鏉＄洰鐨?'index' 鎴愬憳銆俴vm 灏嗗～鍏?'data' 鎴愬憳銆?


### 4.19 KVM_SET_MSRS



:Capability: basic
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_msrs (in)
:Returns: number of msrs successfully set (see below), -1 on error

灏嗘ā鍨嬬壒瀹氬瘎瀛樺櫒鍐欏叆 vcpu銆傛暟鎹粨鏋勮鍙傞槄 KVM_GET_MSRS銆?

搴旂敤绋嬪簭浠ｇ爜搴旇缃?'nmsrs' 鎴愬憳锛堣〃绀?entries 鏁扮粍鐨勫ぇ灏忥級锛屼互鍙婃瘡涓暟缁勬潯鐩殑 'index' 鍜?'data' 鎴愬憳銆?

瀹冧細灏濊瘯閫愪竴璁剧疆鏁扮粍 entries[] 涓殑 MSR銆傚鏋滆缃煇涓?MSR 澶辫触锛堜緥濡傦紝鐢变簬璁剧疆浜嗕繚鐣欎綅銆並VM 涓嶆敮鎸?涓嶆ā鎷熻 MSR 绛夛級锛屽畠浼氬仠姝㈠鐞?MSR 鍒楄〃锛屽苟杩斿洖宸叉垚鍔熻缃殑 MSR 鏁伴噺銆?


### 4.20 KVM_SET_CPUID



:Capability: basic
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_cpuid (in)
:Returns: 0 on success, -1 on error

瀹氫箟 vcpu 瀵?cpuid 鎸囦护鐨勫搷搴斻€傚鏋滃彲鐢紝搴旂敤绋嬪簭搴斾娇鐢?KVM_SET_CPUID2 ioctl銆?

娉ㄦ剰浜嬮」锛圕aveat emptor锛夛細
  - 濡傛灉璇?IOCTL 澶辫触锛孠VM 涓嶄繚璇佸厛鍓嶇殑鏈夋晥 CPUID 閰嶇疆锛堝鏋滃瓨鍦級鏈鐮村潖銆傜敤鎴风┖闂村彲浠ラ€氳繃 KVM_GET_CPUID2 鑾峰彇缁撴灉 CPUID 閰嶇疆鐨勫壇鏈€?
  - 鍦?KVM_RUN 涔嬪悗浣跨敤 KVM_SET_CPUID{,2}锛屽嵆鍦ㄨ繍琛屽鎴锋満涔嬪悗鏇存敼瀹㈡埛鏈?vCPU 妯″瀷锛屽彲鑳藉鑷村鎴锋満涓嶇ǔ瀹氥€?
  - 浣跨敤寮傛瀯鐨?CPUID 閰嶇疆锛圓PIC ID銆佹嫇鎵戠瓑闄ゅ锛夊彲鑳藉鑷村鎴锋満涓嶇ǔ瀹氥€?

```
  struct kvm_cpuid_entry {
	__u32 function;
	__u32 eax;
	__u32 ebx;
	__u32 ecx;
	__u32 edx;
	__u32 padding;
  };

  /* for KVM_SET_CPUID */
  struct kvm_cpuid {
	__u32 nent;
	__u32 padding;
	struct kvm_cpuid_entry entries[0];
  };


```
### 4.21 KVM_SET_SIGNAL_MASK



:Capability: basic
:Architectures: all
:Type: vcpu ioctl
:Parameters: struct kvm_signal_mask (in)
:Returns: 0 on success, -1 on error

瀹氫箟鍦ㄦ墽琛?KVM_RUN 鏈熼棿琚樆濉炵殑淇″彿銆傝淇″彿鎺╃爜涓存椂瑕嗙洊绾跨▼鐨勪俊鍙锋帺鐮併€傛敹鍒扮殑浠讳綍鏈樆濉炰俊鍙凤紙SIGKILL 鍜?SIGSTOP 闄ゅ锛屽畠浠繚鐣欎紶缁熻涓猴級灏嗗鑷?KVM_RUN 浠?-EINTR 杩斿洖銆?

娉ㄦ剰锛屽彧鏈夊綋璇ヤ俊鍙锋湭琚師濮嬩俊鍙锋帺鐮侀樆濉炴椂鎵嶄細琚姇閫掋€?

```
  /* for KVM_SET_SIGNAL_MASK */
  struct kvm_signal_mask {
	__u32 len;
	__u8  sigset[0];
  };


```
### 4.22 KVM_GET_FPU



:Capability: basic
:Architectures: x86, loongarch
:Type: vcpu ioctl
:Parameters: struct kvm_fpu (out)
:Returns: 0 on success, -1 on error

浠?vcpu 璇诲彇娴偣鐘舵€併€?

```
  /* x86: for KVM_GET_FPU and KVM_SET_FPU */
  struct kvm_fpu {
	__u8  fpr[8][16];
	__u16 fcw;
	__u16 fsw;
	__u8  ftwx;  /* in fxsave format */
	__u8  pad1;
	__u16 last_opcode;
	__u64 last_ip;
	__u64 last_dp;
	__u8  xmm[16][16];
	__u32 mxcsr;
	__u32 pad2;
  };

  /* LoongArch: for KVM_GET_FPU and KVM_SET_FPU */
  struct kvm_fpu {
	__u32 fcsr;
	__u64 fcc;
	struct kvm_fpureg {
		__u64 val64[4];
	}fpr[32];
  };


```
### 4.23 KVM_SET_FPU



:Capability: basic
:Architectures: x86, loongarch
:Type: vcpu ioctl
:Parameters: struct kvm_fpu (in)
:Returns: 0 on success, -1 on error

灏嗘诞鐐圭姸鎬佸啓鍏?vcpu銆?

```
  /* x86: for KVM_GET_FPU and KVM_SET_FPU */
  struct kvm_fpu {
	__u8  fpr[8][16];
	__u16 fcw;
	__u16 fsw;
	__u8  ftwx;  /* in fxsave format */
	__u8  pad1;
	__u16 last_opcode;
	__u64 last_ip;
	__u64 last_dp;
	__u8  xmm[16][16];
	__u32 mxcsr;
	__u32 pad2;
  };

  /* LoongArch: for KVM_GET_FPU and KVM_SET_FPU */
  struct kvm_fpu {
	__u32 fcsr;
	__u64 fcc;
	struct kvm_fpureg {
		__u64 val64[4];
	}fpr[32];
  };


```
### 4.24 KVM_CREATE_IRQCHIP



:Capability: KVM_CAP_IRQCHIP, KVM_CAP_S390_IRQCHIP (s390)
:Architectures: x86, arm64, s390
:Type: vm ioctl
:Parameters: none
:Returns: 0 on success, -1 on error

鍦ㄥ唴鏍镐腑鍒涘缓涓€涓腑鏂帶鍒跺櫒妯″瀷銆傚湪 x86 涓婏紝鍒涘缓涓€涓櫄鎷?ioapic銆佷竴涓櫄鎷?PIC锛堜袱涓祵濂楃殑 PIC锛夛紝骞堕厤缃湭鏉ョ殑 vcpu 鎷ユ湁鏈湴 APIC銆侴SI 0-15 鐨?IRQ 璺敱鍚屾椂鎸囧悜 PIC 鍜?IOAPIC锛汫SI 16-23 浠呮寚鍚?IOAPIC銆傚湪 arm64 涓婏紝鍒涘缓涓€涓?GICv2銆備换浣曞叾浠?GIC 鐗堟湰閮介渶瑕佷娇鐢?KVM_CREATE_DEVICE锛屽畠涔熸敮鎸佸垱寤?GICv2銆傚浜?GICv2锛屾帹鑽愪娇鐢?KVM_CREATE_DEVICE 鑰岄潪 KVM_CREATE_IRQCHIP銆傚湪 s390 涓婏紝鍒涘缓涓€涓櫄鎷熺殑 irq 璺敱琛ㄣ€?

娉ㄦ剰锛屽湪 s390 涓婏紝鍦ㄤ娇鐢?KVM_CREATE_IRQCHIP 涔嬪墠闇€瑕佸厛鍚敤 KVM_CAP_S390_IRQCHIP vm 鑳藉姏銆?


### 4.25 KVM_IRQ_LINE



:Capability: KVM_CAP_IRQCHIP
:Architectures: x86, arm64
:Type: vm ioctl
:Parameters: struct kvm_irq_level
:Returns: 0 on success, -1 on error

璁剧疆鍐呮牳涓柇鎺у埗鍣ㄦā鍨嬩腑 GSI 杈撳叆鐨勭數骞炽€傚湪鏌愪簺鏋舵瀯涓婏紝瑕佹眰宸查鍏堜娇鐢?KVM_CREATE_IRQCHIP 鍒涘缓浜嗕腑鏂帶鍒跺櫒妯″瀷銆傛敞鎰忥紝杈规部瑙﹀彂鐨勪腑鏂姹傜數骞冲厛缃负 1 鍐嶇疆鍥?0銆?

鍦ㄧ湡瀹炵‖浠朵笂锛屼腑鏂紩鑴氬彲浠ユ槸浣庣數骞虫湁鏁堟垨楂樼數骞虫湁鏁堛€傝繖瀵逛簬 struct kvm_irq_level 鐨?level 瀛楁娌℃湁褰卞搷锛? 濮嬬粓琛ㄧず鏈夋晥锛坅sserted锛夛紝0 琛ㄧず鏃犳晥锛坉easserted锛夈€?

x86 鍏佽鎿嶄綔绯荤粺涓虹數骞宠Е鍙戜腑鏂紪绋嬩腑鏂瀬鎬э紙浣庣數骞虫湁鏁?楂樼數骞虫湁鏁堬級锛孠VM 杩囧幓涔熶細鑰冭檻鏋佹€с€傜劧鑰岋紝鐢变簬鍦ㄤ綆鐢靛钩鏈夋晥涓柇澶勭悊涓殑浠ｇ爜鑵愬寲锛坆itrot锛夛紝涓婅堪绾﹀畾鐜板湪鍦?x86 涓婁篃鏈夋晥銆傝繖鐢?KVM_CAP_X86_IOAPIC_POLARITY_IGNORED 鍙戝嚭淇″彿銆傜敤鎴风┖闂翠笉搴斿皢涓柇浠ヤ綆鐢靛钩鏈夋晥鐨勬柟寮忓憟鐜扮粰瀹㈡埛鏈猴紝闄ら潪瀛樺湪璇ヨ兘鍔涳紙鎴栬€呭綋鐒讹紝闄ら潪瀹冩病鏈変娇鐢ㄥ唴鏍告€?irqchip锛夈€?

arm64 鍙互鍦?CPU 绾у埆鎴栧湪鍐呮牳鎬?irqchip锛圙IC锛夊鍙戝嚭涓柇淇″彿锛屽苟涓斿浜庡唴鏍告€?irqchip锛屽彲浠ュ憡鐭?GIC 浣跨敤涓虹壒瀹?cpu 鎸囧畾鐨?PPI銆俰rq 瀛楁鐨勮В閲婂涓嬶細

```
  bits:  |  31 ... 28  | 27 ... 24 | 23  ... 16 | 15 ... 0 |
  field: | vcpu2_index | irq_type  | vcpu_index |  irq_id  |
```

irq_type 瀛楁鍏锋湁浠ヤ笅鍙栧€硷細

- KVM_ARM_IRQ_TYPE_CPU:
	       out-of-kernel GIC: irq_id 0 is IRQ, irq_id 1 is FIQ
- KVM_ARM_IRQ_TYPE_SPI:
	       in-kernel GICv2/GICv3: SPI, irq_id between 32 and 1019 (incl.)
               (the vcpu_index field is ignored)
	       in-kernel GICv5: SPI, irq_id between 0 and 65535 (incl.)
- KVM_ARM_IRQ_TYPE_PPI:
	       in-kernel GICv2/GICv3: PPI, irq_id between 16 and 31 (incl.)
	       in-kernel GICv5: PPI, irq_id between 0 and 127 (incl.)

锛堝洜姝?irq_id 瀛楁鎭板ソ瀵瑰簲浜?ARM GIC 瑙勮寖涓殑 IRQ ID锛?

鍦ㄨ繖涓ょ鎯呭喌涓嬶紝level 閮界敤浜庣疆浣?娓呴櫎璇ョ嚎璺€?

褰撴敮鎸?KVM_CAP_ARM_IRQ_LINE_LAYOUT_2 鏃讹紝鐩爣 vcpu 琚爣璇嗕负 (256 * vcpu2_index + vcpu_index)銆傚惁鍒欙紝vcpu2_index 蹇呴』涓洪浂銆?

娉ㄦ剰锛屽湪 arm64 涓婏紝KVM_CAP_IRQCHIP 鑳藉姏浠呭喅瀹氬唴鏍告€?irqchip 鐨勪腑鏂敞鍏ャ€侹VM_IRQ_LINE 濮嬬粓鍙敤浜庣敤鎴风┖闂翠腑鏂帶鍒跺櫒銆?

```
  struct kvm_irq_level {
	union {
		__u32 irq;     /* GSI */
		__s32 status;  /* not used for KVM_IRQ_LEVEL */
	};
	__u32 level;           /* 0 or 1 */
  };

```
### 4.26 KVM_GET_IRQCHIP



:Capability: KVM_CAP_IRQCHIP
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_irqchip (in/out)
:Returns: 0 on success, -1 on error

灏嗕娇鐢?KVM_CREATE_IRQCHIP 鍒涘缓鐨勫唴鏍镐腑鏂帶鍒跺櫒鐨勭姸鎬佽鍏ヨ皟鐢ㄨ€呮彁渚涚殑缂撳啿鍖恒€?

```
  struct kvm_irqchip {
	__u32 chip_id;  /* 0 = PIC1, 1 = PIC2, 2 = IOAPIC */
	__u32 pad;
        union {
		char dummy[512];  /* reserving space */
		struct kvm_pic_state pic;
		struct kvm_ioapic_state ioapic;
	} chip;
  };

```
### 4.27 KVM_SET_IRQCHIP



:Capability: KVM_CAP_IRQCHIP
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_irqchip (in)
:Returns: 0 on success, -1 on error

浠庤皟鐢ㄨ€呮彁渚涚殑缂撳啿鍖鸿缃娇鐢?KVM_CREATE_IRQCHIP 鍒涘缓鐨勫唴鏍镐腑鏂帶鍒跺櫒鐨勭姸鎬併€?

```
  struct kvm_irqchip {
	__u32 chip_id;  /* 0 = PIC1, 1 = PIC2, 2 = IOAPIC */
	__u32 pad;
        union {
		char dummy[512];  /* reserving space */
		struct kvm_pic_state pic;
		struct kvm_ioapic_state ioapic;
	} chip;
  };

```
### 4.28 KVM_XEN_HVM_CONFIG



:Capability: KVM_CAP_XEN_HVM
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_xen_hvm_config (in)
:Returns: 0 on success, -1 on error

璁剧疆 Xen HVM 瀹㈡埛鏈虹敤浜庡垵濮嬪寲鍏惰秴绾ц皟鐢ㄩ〉鐨?MSR锛屽苟鎻愪緵鐢ㄦ埛绌洪棿涓秴绾ц皟鐢?blob 鐨勮捣濮嬪湴鍧€鍜屽ぇ灏忋€傚綋瀹㈡埛鏈哄啓鍏ヨ MSR 鏃讹紝kvm 浼氬皢涓€涓?blob 椤碉紙32 浣嶆垨 64 浣嶏紝鍙栧喅浜?vcpu 妯″紡锛夊鍒跺埌瀹㈡埛鏈哄唴瀛樹腑銆?

MSR 绱㈠紩蹇呴』浣嶄簬 [0x40000000, 0x4fffffff] 鑼冨洿鍐咃紝鍗冲繀椤讳綅浜庨潪瀹樻柟涓鸿櫄鎷熸満鐩戞帶鍣ㄤ繚鐣欑殑鑼冨洿鍐呫€傛渶灏忓€煎拰鏈€澶у€奸€氳繃 KVM_XEN_MSR_MIN_INDEX 鍜?KVM_XEN_MSR_MAX_INDEX 鏋氫妇銆?

```
  struct kvm_xen_hvm_config {
	__u32 flags;
	__u32 msr;
	__u64 blob_addr_32;
	__u64 blob_addr_64;
	__u8 blob_size_32;
	__u8 blob_size_64;
	__u8 pad2[30];
  };
```

濡傛灉 KVM_CAP_XEN_HVM 妫€鏌ヨ繑鍥炰簡鏌愪簺鏍囧織锛屽垯鍙互灏嗗畠浠缃湪璇?ioctl 鐨?flags 瀛楁涓細

KVM_XEN_HVM_CONFIG_INTERCEPT_HCALL 鏍囧織璇锋眰 KVM 鑷姩鐢熸垚瓒呯骇璋冪敤椤电殑鍐呭锛涜秴绾ц皟鐢ㄥ皢琚嫤鎴苟閫氳繃 KVM_EXIT_XEN 浼犻€掔粰鐢ㄦ埛绌洪棿銆傚湪杩欑鎯呭喌涓嬶紝鎵€鏈?blob 澶у皬鍜屽湴鍧€瀛楁蹇呴』涓洪浂銆?

KVM_XEN_HVM_CONFIG_EVTCHN_SEND 鏍囧織鍚?KVM 琛ㄦ槑锛岀敤鎴风┖闂村皢濮嬬粓浣跨敤 KVM_XEN_HVM_EVTCHN_SEND ioctl 鏉ユ姇閫掍簨浠堕€氶亾涓柇锛岃€屼笉鏄洿鎺ユ搷浣滃鎴锋満鐨?shared_info 缁撴瀯銆傚弽杩囨潵锛岃繖鍙兘鍏佽 KVM 鍚敤璇稿鎷︽埅 SCHEDOP_poll 瓒呯骇璋冪敤浠ュ姞閫熷鎴锋満鐨?PV 鑷棆閿佹搷浣滅瓑鐗规€с€傚嵆浣胯骞垮憡浜嗚鑳藉姏锛岀敤鎴风┖闂翠粛鍙娇鐢ㄨ ioctl 鏉ユ姇閫掍簨浠讹紝鍗充娇鐢ㄦ埛绌洪棿娌℃湁鍙戦€佸畠灏嗗缁堣繖鏍峰仛鐨勬寚绀恒€?

鐩墠锛宻truct kvm_xen_hvm_config 涓病鏈夊叾浠栨湁鏁堟爣蹇椼€?

### 4.29 KVM_GET_CLOCK



:Capability: KVM_CAP_ADJUST_CLOCK
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_clock_data (out)
:Returns: 0 on success, -1 on error

鑾峰彇褰撳墠瀹㈡埛鏈烘墍鐪嬪埌鐨?kvmclock 鐨勫綋鍓嶆椂闂存埑銆傜粨鍚?KVM_SET_CLOCK锛屽畠鐢ㄤ簬鍦ㄨ縼绉荤瓑鍦烘櫙涓‘淇濆崟璋冩€с€?

褰撳皢 KVM_CAP_ADJUST_CLOCK 浼犻€掔粰 KVM_CHECK_EXTENSION 鏃讹紝瀹冭繑鍥?KVM 鍙湪 struct kvm_clock_data 鐨?flag 鎴愬憳涓繑鍥炵殑涓€缁勪綅銆?

瀹氫箟浜嗕互涓嬫爣蹇楋細

KVM_CLOCK_TSC_STABLE
  濡傛灉缃綅锛岃繑鍥炵殑鍊兼槸璋冪敤 KVM_GET_CLOCK 閭ｄ竴鍒绘墍鏈?VCPU 鎵€鐪嬪埌鐨勭簿纭?kvmclock 鍊笺€?
  濡傛灉娓呴浂锛岃繑鍥炵殑鍊煎彧鏄?CLOCK_MONOTONIC 鍔犱笂涓€涓父閲忓亸绉伙紱璇ュ亸绉诲彲浠ラ€氳繃 KVM_SET_CLOCK 淇敼銆侹VM 浼氬皾璇曡鎵€鏈?VCPU 璺熼殢姝ゆ椂閽燂紝浣嗙敱浜庡涓?TSC 涓嶇ǔ瀹氾紝姣忎釜 VCPU 璇诲彇鐨勭簿纭€煎彲鑳戒笉鍚屻€?

KVM_CLOCK_REALTIME
  濡傛灉缃綅锛宬vm_clock_data 缁撴瀯涓殑 `realtime` 瀛楁浼氳濉厖涓鸿皟鐢?KVM_GET_CLOCK 閭ｄ竴鍒诲涓绘満瀹炴椂鏃堕挓婧愮殑鍊笺€傚鏋滄竻闆讹紝鍒?`realtime` 瀛楁涓嶅寘鍚€笺€?

KVM_CLOCK_HOST_TSC
  濡傛灉缃綅锛宬vm_clock_data 缁撴瀯涓殑 `host_tsc` 瀛楁浼氳濉厖涓鸿皟鐢?KVM_GET_CLOCK 閭ｄ竴鍒诲涓绘満鏃堕棿鎴宠鏁板櫒锛圱SC锛夌殑鍊笺€傚鏋滄竻闆讹紝鍒?`host_tsc` 瀛楁涓嶅寘鍚€笺€?

```
  struct kvm_clock_data {
	__u64 clock;  /* kvmclock current value */
	__u32 flags;
	__u32 pad0;
	__u64 realtime;
	__u64 host_tsc;
	__u32 pad[4];
  };

```
### 4.30 KVM_SET_CLOCK



:Capability: KVM_CAP_ADJUST_CLOCK
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_clock_data (in)
:Returns: 0 on success, -1 on error

灏?kvmclock 鐨勫綋鍓嶆椂闂存埑璁剧疆涓哄弬鏁颁腑鎸囧畾鐨勫€笺€傜粨鍚?KVM_GET_CLOCK锛屽畠鐢ㄤ簬鍦ㄨ縼绉荤瓑鍦烘櫙涓‘淇濆崟璋冩€с€?

鍙互浼犻€掍互涓嬫爣蹇楋細

KVM_CLOCK_REALTIME
  濡傛灉缃綅锛孠VM 浼氬皢 `realtime` 瀛楁鐨勫€间笌璋冪敤 KVM_SET_CLOCK 閭ｄ竴鍒诲涓绘満瀹炴椂鏃堕挓婧愮殑鍊艰繘琛屾瘮杈冦€傜粡杩囨椂闂寸殑宸€间細琚姞鍒版渶缁堟彁渚涚粰瀹㈡埛鏈虹殑 kvmclock 鍊间腑銆?

`KVM_GET_CLOCK` 杩斿洖鐨勫叾浠栨爣蹇椾細琚帴鍙椾絾琚拷鐣ャ€?

```
  struct kvm_clock_data {
	__u64 clock;  /* kvmclock current value */
	__u32 flags;
	__u32 pad0;
	__u64 realtime;
	__u64 host_tsc;
	__u32 pad[4];
  };

```
### 4.31 KVM_GET_VCPU_EVENTS



:Capability: KVM_CAP_VCPU_EVENTS
:Extended by: KVM_CAP_INTR_SHADOW
:Architectures: x86, arm64
:Type: vcpu ioctl
:Parameters: struct kvm_vcpu_events (out)
:Returns: 0 on success, -1 on error

##### X86:



鑾峰彇褰撳墠鎸傝捣鐨勫紓甯搞€佷腑鏂拰 NMI 浠ュ強 vcpu 鐨勭浉鍏崇姸鎬併€?

```
  struct kvm_vcpu_events {
	struct {
		__u8 injected;
		__u8 nr;
		__u8 has_error_code;
		__u8 pending;
		__u32 error_code;
	} exception;
	struct {
		__u8 injected;
		__u8 nr;
		__u8 soft;
		__u8 shadow;
	} interrupt;
	struct {
		__u8 injected;
		__u8 pending;
		__u8 masked;
		__u8 pad;
	} nmi;
	__u32 sipi_vector;
	__u32 flags;
	struct {
		__u8 smm;
		__u8 pending;
		__u8 smm_inside_nmi;
		__u8 latched_init;
	} smi;
	__u8 reserved[27];
	__u8 exception_has_payload;
	__u64 exception_payload;
  };
```

鍦?flags 瀛楁涓畾涔変簡浠ヤ笅浣嶏細

- KVM_VCPUEVENT_VALID_SHADOW 鍙缃綅浠ヨ〃鏄?interrupt.shadow 鍖呭惈鏈夋晥鐘舵€併€?

- KVM_VCPUEVENT_VALID_SMM 鍙缃綅浠ヨ〃鏄?smi 鍖呭惈鏈夋晥鐘舵€併€?

- KVM_VCPUEVENT_VALID_PAYLOAD 鍙缃綅浠ヨ〃鏄?exception_has_payload銆乪xception_payload 鍜?exception.pending 瀛楁鍖呭惈鏈夋晥鐘舵€併€傚彧瑕佸惎鐢ㄤ簡 KVM_CAP_EXCEPTION_PAYLOAD锛岃浣嶅氨浼氳缃綅銆?

- KVM_VCPUEVENT_VALID_TRIPLE_FAULT 鍙缃綅浠ヨ〃鏄?triple_fault_pending 瀛楁鍖呭惈鏈夋晥鐘舵€併€傚彧瑕佸惎鐢ㄤ簡 KVM_CAP_X86_TRIPLE_FAULT_EVENT锛岃浣嶅氨浼氳缃綅銆?

##### ARM64:



濡傛灉瀹㈡埛鏈轰互鏌愮鏂瑰紡璁块棶鐢卞涓诲唴鏍告ā鎷熺殑璁惧锛岃€岀湡瀹炶澶囦細鍥犳鐢熸垚鐗╃悊 SError锛孠VM 鍙兘浼氫负璇?VCPU 浣夸竴涓櫄鎷?SError 鎸傝捣銆傝绯荤粺閿欒涓柇淇濇寔鎸傝捣锛岀洿鍒板鎴锋満閫氳繃瑙ｉ櫎 PSTATE.A 灞忚斀鏉ユ帴鍙楄寮傚父銆?

杩愯 VCPU 鍙兘瀵艰嚧瀹冩帴鍙楁寕璧风殑 SError锛屾垨杩涜瀵艰嚧 SError 鎸傝捣鐨勮闂€備簨浠剁殑鎻忚堪浠呭湪 VPCU 鏈繍琛屾椂鏈夋晥銆?

璇?API 鎻愪緵浜嗕竴绉嶈鍐欎负瀹㈡埛鏈轰笉鍙鐨勬寕璧?event"鐘舵€佺殑鏂规硶銆傝淇濆瓨銆佹仮澶嶆垨杩佺Щ VCPU锛屽彲浠ヤ娇鐢ㄦ GET/SET API 璇诲彇鐒跺悗鍐欏叆琛ㄧず璇ョ姸鎬佺殑缁撴瀯浣擄紝浠ュ強涓庡叾瀹冨鎴锋満鍙鐨勫瘎瀛樺櫒涓€璧枫€傛棤娉?鍙栨秷"涓€涓凡鎸傝捣鐨?SError銆?

鍦ㄧ敤鎴风┖闂存ā鎷熺殑璁惧涔熷彲鑳藉笇鏈涚敓鎴?SError銆備负姝わ紝浜嬩欢缁撴瀯浣撳彲浠ョ敱鐢ㄦ埛绌洪棿濉厖銆傚簲棣栧厛璇诲彇褰撳墠鐘舵€侊紝浠ョ‘淇濇病鏈夌幇鏈夌殑 SError 鎸傝捣銆傚鏋滃瓨鍦ㄧ幇鏈夌殑 SError 鎸傝捣锛屽垯搴旈伒寰灦鏋勭殑"Multiple SError interrupts"瑙勫垯銆傦紙DDI0587.a "ARM Reliability, Availability, and Serviceability (RAS) Specification" 鐨?2.5.3 鑺傦級銆?

SError 寮傚父濮嬬粓鏈変竴涓?ESR 鍊笺€傛煇浜?CPU 鑳藉鎸囧畾铏氭嫙 SError 鐨?ESR 鍊煎簲璇ユ槸浠€涔堛€傝繖浜涚郴缁熶細骞垮憡 KVM_CAP_ARM_INJECT_SERROR_ESR銆傚湪杩欑鎯呭喌涓嬶紝璇诲彇鏃?exception.has_esr 濮嬬粓鍏锋湁闈為浂鍊硷紝鑰屼娇 SError 鎸傝捣鐨勪唬鐞嗗簲鎸囧畾 exception.serror_esr 浣?24 浣嶄腑鐨?ISS 瀛楁銆傚鏋滅郴缁熸敮鎸?KVM_CAP_ARM_INJECT_SERROR_ESR锛屼絾鐢ㄦ埛绌洪棿灏嗕簨浠惰缃负 exception.has_esr 涓洪浂锛孠VM 浼氶€夋嫨涓€涓?ESR銆?

鍦ㄤ笉鏀寔璇ヨ兘鍔涚殑绯荤粺涓婃寚瀹?exception.has_esr 灏嗚繑鍥?-EINVAL銆傝缃?exception.serror_esr 浣?24 浣嶄箣澶栫殑浠讳綍鍐呭灏嗚繑鍥?-EINVAL銆?

鏃犳硶璇诲洖鎸傝捣鐨勫閮ㄤ腑姝紙閫氳繃 KVM_SET_VCPU_EVENTS 鎴栧叾浠栨柟寮忔敞鍏ワ級锛屽洜涓烘绫诲紓甯告€绘槸鐩存帴鎶曢€掑埌铏氭嫙 CPU銆?

鍦ㄥ皻鏈垵濮嬪寲鐨?vCPU 涓婅皟鐢ㄦ ioctl 灏嗚繑鍥?-ENOEXEC銆?

```
  struct kvm_vcpu_events {
	struct {
		__u8 serror_pending;
		__u8 serror_has_esr;
		__u8 ext_dabt_pending;
		/* Align it to 8 bytes */
		__u8 pad[5];
		__u64 serror_esr;
	} exception;
	__u32 reserved[12];
  };
```
### 4.32 KVM_SET_VCPU_EVENTS



:Capability: KVM_CAP_VCPU_EVENTS
:Extended by: KVM_CAP_INTR_SHADOW
:Architectures: x86, arm64
:Type: vcpu ioctl
:Parameters: struct kvm_vcpu_events (in)
:Returns: 0 on success, -1 on error

##### X86:



璁剧疆鎸傝捣鐨勫紓甯搞€佷腑鏂€丯MI 浠ュ強 vcpu 鐨勭浉鍏崇姸鎬併€?

See KVM_GET_VCPU_EVENTS for the data structure.

鍙兘琚繍琛岀殑 VCPU 寮傛淇敼鐨勫瓧娈靛彲浠ヤ粠鏇存柊涓帓闄ゃ€傝繖浜涘瓧娈垫槸 nmi.pending銆乻ipi_vector銆乻mi.smm銆乻mi.pending銆備繚鎸?flags 瀛楁涓浉搴旂殑浣嶈娓呴浂锛屼互鎶戝埗瑕嗙洊褰撳墠鍐呮牳鎬佺姸鎬併€傝繖浜涗綅鏄細

===============================  ==================================
KVM_VCPUEVENT_VALID_NMI_PENDING  transfer nmi.pending to the kernel
KVM_VCPUEVENT_VALID_SIPI_VECTOR  transfer sipi_vector
KVM_VCPUEVENT_VALID_SMM          transfer the smi sub-struct.
===============================  ==================================

濡傛灉 KVM_CAP_INTR_SHADOW 鍙敤锛屽垯鍙互鍦?flags 瀛楁涓缃?KVM_VCPUEVENT_VALID_SHADOW锛屼互琛ㄦ槑 interrupt.shadow 鍖呭惈鏈夋晥鐘舵€佸苟搴旇鍐欏叆 VCPU銆?

鍙湁鍦?KVM_CAP_X86_SMM 鍙敤鏃舵墠鑳借缃?KVM_VCPUEVENT_VALID_SMM銆?

濡傛灉鍚敤浜?KVM_CAP_EXCEPTION_PAYLOAD锛屽垯鍙互鍦?flags 瀛楁涓缃?KVM_VCPUEVENT_VALID_PAYLOAD锛屼互琛ㄦ槑 exception_has_payload銆乪xception_payload 鍜?exception.pending 瀛楁鍖呭惈鏈夋晥鐘舵€佸苟搴旇鍐欏叆 VCPU銆?

濡傛灉鍚敤浜?KVM_CAP_X86_TRIPLE_FAULT_EVENT锛屽垯鍙互鍦?flags 瀛楁涓缃?KVM_VCPUEVENT_VALID_TRIPLE_FAULT锛屼互琛ㄦ槑 triple_fault 瀛楁鍖呭惈鏈夋晥鐘舵€佸苟搴旇鍐欏叆 VCPU銆?

##### ARM64:



鐢ㄦ埛绌洪棿鍙兘闇€瑕佸悜瀹㈡埛鏈烘敞鍏ュ绉嶇被鍨嬬殑浜嬩欢銆?

璁剧疆姝?VCPU 鎸傝捣鐨?SError 寮傚父鐘舵€併€傛棤娉?鍙栨秷"涓€涓凡鎸傝捣鐨?SError銆?

濡傛灉瀹㈡埛鏈哄 I/O 鍐呭瓨杩涜浜嗙敤鎴风┖闂存棤娉曞鐞嗙殑璁块棶锛屼緥濡傜敱浜庣己灏戞寚浠ょ患鍚堝緛锛坰yndrome锛夎В鐮佷俊鎭紝鎴栬€呭洜涓哄湪琚闂殑 IPA 澶勬病鏈夋槧灏勮澶囷紝閭ｄ箞鐢ㄦ埛绌洪棿鍙互璇峰唴鏍镐娇鐢ㄦ潵鑷?VCPU 閫€鍑烘晠闅滅殑鍦板潃娉ㄥ叆涓€涓閮ㄤ腑姝€傚湪涓嶆槸 KVM_EXIT_MMIO銆並VM_EXIT_ARM_NISV 鎴?KVM_EXIT_ARM_LDST64B 鐨勯€€鍑轰箣鍚庤缃?ext_dabt_pending 鏄竴绉嶇紪绋嬮敊璇€傛鐗规€т粎鍦ㄧ郴缁熸敮鎸?KVM_CAP_ARM_INJECT_EXT_DABT 鏃跺彲鐢ㄣ€傝繖鏄竴涓緟鍔╄鏂斤紝涓轰笉鍚岀敤鎴风┖闂村疄鐜板湪濡備綍鍚戝鎴锋満鎶ュ憡涓婅堪鎯呭喌鐨勮闂柟闈㈡彁渚涗竴鑷存€с€傚敖绠″姝わ紝鐢ㄦ埛绌洪棿浠嶇劧鍙互閫氳繃浣跨敤 KVM_SET_ONE_REG API 鎿嶄綔鍚勪釜瀵勫瓨鍣ㄦ潵妯℃嫙鎵€鏈?Arm 寮傚父銆?

See KVM_GET_VCPU_EVENTS for the data structure.

鍦ㄥ皻鏈垵濮嬪寲鐨?vCPU 涓婅皟鐢ㄦ ioctl 灏嗚繑鍥?-ENOEXEC銆?

### 4.33 KVM_GET_DEBUGREGS



:Capability: KVM_CAP_DEBUGREGS
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_debugregs (out)
:Returns: 0 on success, -1 on error

浠?vcpu 璇诲彇璋冭瘯瀵勫瓨鍣ㄣ€?

```
  struct kvm_debugregs {
	__u64 db[4];
	__u64 dr6;
	__u64 dr7;
	__u64 flags;
	__u64 reserved[9];
  };


```
### 4.34 KVM_SET_DEBUGREGS



:Capability: KVM_CAP_DEBUGREGS
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_debugregs (in)
:Returns: 0 on success, -1 on error

灏嗚皟璇曞瘎瀛樺櫒鍐欏叆 vcpu銆?

See KVM_GET_DEBUGREGS for the data structure. The flags field is unused yet and must be cleared on entry.### 4.35 KVM_SET_USER_MEMORY_REGION


:Capability: KVM_CAP_USER_MEMORY
:Architectures: all
:Type: vm ioctl
:Parameters: struct kvm_userspace_memory_region (in)
:Returns: 0 on success, -1 on error

```

  struct kvm_userspace_memory_region {
	__u32 slot;
	__u32 flags;
	__u64 guest_phys_addr;
	__u64 memory_size; /* bytes */
	__u64 userspace_addr; /* start of the userspace allocated memory */
  };

  /* for kvm_userspace_memory_region::flags */
  #define KVM_MEM_LOG_DIRTY_PAGES	(1UL << 0)
  #define KVM_MEM_READONLY	(1UL << 1)

```
璇?ioctl 鍏佽鐢ㄦ埛鍒涘缓銆佷慨鏀规垨鍒犻櫎涓€涓鎴锋満鐗╃悊鍐呭瓨妲姐€?slot" 鐨?0-15 浣嶆寚瀹氭Ы id锛岃鍊煎簲灏忎簬姣忎釜
VM 鎵€鏀寔鐨勬渶澶х敤鎴峰唴瀛樻Ы鏁伴噺銆傛渶澶у厑璁哥殑妲芥暟閲忓彲閫氳繃 KVM_CAP_NR_MEMSLOTS 鏌ヨ銆?
妲藉湪瀹㈡埛鏈虹墿鐞嗗湴鍧€绌洪棿涓笉寰楅噸鍙犮€?

濡傛灉 KVM_CAP_MULTI_ADDRESS_SPACE 鍙敤锛?slot" 鐨?16-31 浣嶆寚瀹氳淇敼鐨勫湴鍧€绌洪棿銆傚畠浠繀椤诲皬浜?
KVM_CHECK_EXTENSION 閽堝 KVM_CAP_MULTI_ADDRESS_SPACE 鑳藉姏杩斿洖鐨勫€笺€備笉鍚屽湴鍧€绌洪棿涓殑妲藉郊姝ゆ棤鍏筹紱
鍏充簬妲介噸鍙犵殑闄愬埗浠呴€傜敤浜庡悇鑷殑鍦板潃绌洪棿鍐呴儴銆?

鍒犻櫎妲界殑鏂规硶鏄护 memory_size 涓洪浂銆傚綋淇敼涓€涓凡瀛樺湪鐨勬Ы鏃讹紝瀹冨彲浠ュ湪瀹㈡埛鏈虹墿鐞嗗唴瀛樼┖闂翠腑绉诲姩锛?
鎴栧叾 flags 鍙互琚慨鏀癸紝浣嗗ぇ灏忎笉鍙璋冩暣銆?

璇ュ尯鍩熺殑鍐呭瓨浠?userspace_addr 瀛楁鎵€鎸囧悜鐨勫湴鍧€澶勫紑濮嬭幏鍙栵紝璇ュ湴鍧€蹇呴』鎸囧悜鏁翠釜鍐呭瓨妲藉ぇ灏忚寖鍥村唴
鐢ㄦ埛鍙鍧€鐨勫唴瀛樸€備换浣曞璞￠兘鍙互浣滀负杩欏潡鍐呭瓨鐨勫悗澶囷紝鍖呮嫭鍖垮悕鍐呭瓨銆佹櫘閫氭枃浠朵互鍙?hugetlbfs銆傚唴瀛樺尯鍩?
鍚庡鐨勫彉鍖栦細鑷姩鍙嶆槧鍒板鎴锋満涓€備緥濡傦紝褰卞搷璇ュ尯鍩熺殑 mmap() 浼氱珛鍒诲彉寰楀瀹㈡埛鏈哄彲瑙併€傚彟涓€涓緥瀛愭槸
madvise(MADV_DROP)銆?

鍦ㄦ敮鎸佹煇绉嶅湴鍧€鏍囪锛坅ddress tagging锛夊舰寮忔灦鏋勪笂锛寀serspace_addr 蹇呴』鏄湭鏍囪鐨勶紙untagged锛夊湴鍧€銆?

寤鸿 guest_phys_addr 涓?userspace_addr 鐨勪綆 21 浣嶄繚鎸佷竴鑷淬€傝繖鏍峰彲浠ヨ瀹㈡埛鏈轰腑鐨勫ぇ椤电敱瀹夸富鏈轰腑鐨?
澶ч〉浣滀负鍚庡銆?

flags 瀛楁鏀寔涓や釜鏍囧織锛欿VM_MEM_LOG_DIRTY_PAGES 涓?KVM_MEM_READONLY銆傚墠鑰呭彲琚缃互鎸囩ず KVM 璺熻釜
妲藉唴鍐呭瓨鐨勫啓鍏ユ儏鍐点€傚浣曚娇鐢ㄥ畠鍙弬瑙?KVM_GET_DIRTY_LOG ioctl銆傝嫢 KVM_CAP_READONLY_MEM 鑳藉姏鍏佽锛?
鍚庤€呭彲琚缃互浣挎柊妲藉彉涓哄彧璇汇€傚湪杩欑鎯呭喌涓嬶紝瀵硅鍐呭瓨鐨勫啓鍏ヤ細琚綔涓?KVM_EXIT_MMIO 閫€鍑轰笂鎶ョ粰鐢ㄦ埛绌洪棿銆?

瀵逛簬 TDX 瀹㈡埛鏈猴紝鍒犻櫎/绉诲姩鍐呭瓨鍖哄煙浼氫涪澶卞鎴锋満鍐呭瓨鍐呭銆備笉鏀寔鍙鍖哄煙銆備粎鏀寔 as-id 0銆?

娉ㄦ剰锛氬湪 arm64 涓婏紝褰撴Ы鍏锋湁 KVM_MEM_READONLY 鏍囧織鏃讹紝鐢遍〉琛ㄩ亶鍘嗗櫒锛坧age-table walker锛変骇鐢熺殑鍐欏叆
锛堜緥濡傜敤浜庢洿鏂?Access 鍜?Dirty 鏍囧織锛夋案杩滀笉浼氬鑷?KVM_EXIT_MMIO 閫€鍑恒€傝繖鏄洜涓?KVM 鏃犳硶鎻愪緵椤佃〃閬嶅巻鍣?
灏嗚鍐欏叆鐨勬暟鎹紝浠庤€屾棤娉曟ā鎷熻璁块棶銆傚彇鑰屼唬涔嬶紝浼氬悜瀹㈡埛鏈烘敞鍏ヤ竴涓紓甯革紙濡傛灉椤佃〃鏇存柊鐨勮捣鍥犳槸鍔犺浇鎴?
瀛樺偍锛屽垯涓烘暟鎹紓甯?data abort锛涘鏋滄槸鎸囦护鑾峰彇锛屽垯涓烘寚浠ゅ紓甯?instruction abort锛夈€?

##### S390:


濡傛灉 VM 璁剧疆浜?KVM_VM_S390_UCONTROL 鏍囧織锛屽垯杩斿洖 -EINVAL 鎴?-EEXIST銆?
濡傛灉鏄湪鍙椾繚鎶ょ殑 VM 涓婅皟鐢紝鍒欒繑鍥?-EINVAL銆?

### 4.36 KVM_SET_TSS_ADDR


:Capability: KVM_CAP_SET_TSS_ADDR
:Architectures: x86
:Type: vm ioctl
:Parameters: unsigned long tss_address (in)
:Returns: 0 on success, -1 on error

璇?ioctl 瀹氫箟瀹㈡埛鏈虹墿鐞嗗湴鍧€绌洪棿涓竴涓笁椤靛尯鍩熺殑鐗╃悊鍦板潃銆傝鍖哄煙蹇呴』浣嶄簬瀹㈡埛鏈虹墿鐞嗗湴鍧€绌洪棿鐨勫墠
4GB 涔嬪唴锛屼笖涓嶈兘涓庝换浣曞唴瀛樻Ы鎴栦换浣?mmio 鍦板潃鍐茬獊銆傚鏋滃鎴锋満璁块棶璇ュ唴瀛樺尯鍩燂紝鍙兘浼氬彂鐢熸晠闅溿€?

鍦ㄥ熀浜?Intel 鐨勪富鏈轰笂锛岃 ioctl 鏄繀闇€鐨勩€傚湪 Intel 纭欢涓婇渶瑕佸畠锛屾槸鍥犱负铏氭嫙鍖栧疄鐜颁腑鐨勪竴涓?
鎬紓涔嬪锛堝弬瑙佸皻鏈潰涓栫殑 internals 鏂囨。锛夈€?



### 4.37 KVM_ENABLE_CAP


:Capability: KVM_CAP_ENABLE_CAP
:Architectures: mips, ppc, s390, x86, loongarch
:Type: vcpu ioctl
:Parameters: struct kvm_enable_cap (in)
:Returns: 0 on success; -1 on error

:Capability: KVM_CAP_ENABLE_CAP_VM
:Architectures: all
:Type: vm ioctl
:Parameters: struct kvm_enable_cap (in)
:Returns: 0 on success; -1 on error


   骞堕潪鎵€鏈夋墿灞曢兘榛樿鍚敤銆傞€氳繃姝?ioctl锛屽簲鐢ㄧ▼搴忓彲浠ュ惎鐢ㄤ竴涓墿灞曪紝浣垮叾瀵瑰鎴锋満鍙敤銆?

鍦ㄤ笉鏀寔姝?ioctl 鐨勭郴缁熶笂锛屽畠鎬绘槸澶辫触銆傚湪鏀寔瀹冪殑绯荤粺涓婏紝瀹冨彧瀵归偅浜涙敮鎸佽鍚敤鐨勬墿灞曟湁鏁堛€?

瑕佹鏌ユ煇涓兘鍔涙槸鍚﹀彲浠ヨ鍚敤锛屽簲褰撲娇鐢?KVM_CHECK_EXTENSION ioctl銆?

```

  struct kvm_enable_cap {
       /* in */
       __u32 cap;

```
瑕佽鍚敤鐨勮兘鍔涖€?

```

       __u32 flags;

```
涓€涓寚绀烘湭鏉ュ寮虹殑浣嶅煙銆傜洰鍓嶅繀椤讳负 0銆?

```

       __u64 args[4];

```
鍚敤鏌愪釜鐗规€ф墍闇€鐨勫弬鏁般€傚鏋滀竴涓壒鎬ч渶瑕佸垵濮嬪€兼墠鑳芥甯稿伐浣滐紝杩欓噷灏辨槸鏀剧疆瀹冧滑鐨勫湴鏂广€?

```

       __u8  pad[64];
  };

```
vcpu ioctl 搴旂敤浜?vcpu 鐗瑰畾鐨勮兘鍔涳紝vm ioctl 搴旂敤浜?VM 鑼冨洿鐨勮兘鍔涖€?

### 4.38 KVM_GET_MP_STATE


:Capability: KVM_CAP_MP_STATE
:Architectures: x86, s390, arm64, riscv, loongarch
:Type: vcpu ioctl
:Parameters: struct kvm_mp_state (out)
:Returns: 0 on success; -1 on error

```

  struct kvm_mp_state {
	__u32 mp_state;
  };

```
杩斿洖 vcpu 褰撳墠鐨?澶氬鐞嗗櫒鐘舵€?锛堝敖绠″湪鍗曞鐞嗗櫒瀹㈡埛鏈轰笂涔熸湁鏁堬級銆?

鍙兘鐨勫€煎涓嬶細

   ==========================    ===============================================
   KVM_MP_STATE_RUNNABLE         the vcpu is currently running
                                 [x86,arm64,riscv,loongarch]
   KVM_MP_STATE_UNINITIALIZED    the vcpu is an application processor (AP)
                                 which has not yet received an INIT signal [x86]
   KVM_MP_STATE_INIT_RECEIVED    the vcpu has received an INIT signal, and is
                                 now ready for a SIPI [x86]
   KVM_MP_STATE_HALTED           the vcpu has executed a HLT instruction and
                                 is waiting for an interrupt [x86]
   KVM_MP_STATE_SIPI_RECEIVED    the vcpu has just received a SIPI (vector
                                 accessible via KVM_GET_VCPU_EVENTS) [x86]
   KVM_MP_STATE_STOPPED          the vcpu is stopped [s390,arm64,riscv]
   KVM_MP_STATE_CHECK_STOP       the vcpu is in a special error state [s390]
   KVM_MP_STATE_OPERATING        the vcpu is operating (running or halted)
                                 [s390]
   KVM_MP_STATE_LOAD             the vcpu is in a special load/startup state
                                 [s390]
   KVM_MP_STATE_SUSPENDED        the vcpu is in a suspend state and is waiting
                                 for a wakeup event [arm64]
   ==========================    ===============================================

鍦?x86 涓婏紝姝?ioctl 浠呭湪 KVM_CREATE_IRQCHIP 涔嬪悗鎵嶆湁鐢ㄣ€傚鏋滄病鏈夊唴鏍告€?irqchip锛屽澶勭悊鍣ㄧ姸鎬?
蹇呴』鍦ㄨ繖浜涙灦鏋勪笂鐢辩敤鎴风┖闂寸淮鎶ゃ€?

##### For arm64:


濡傛灉 vCPU 澶勪簬 KVM_MP_STATE_SUSPENDED 鐘舵€侊紝KVM 浼氭ā鎷?WFI 鎸囦护鐨勬灦鏋勫寲鎵ц銆?

濡傛灉璇嗗埆鍒颁竴涓敜閱掍簨浠讹紝KVM 浼氶€€鍑哄埌鐢ㄦ埛绌洪棿锛屼骇鐢熶竴涓?KVM_SYSTEM_EVENT 閫€鍑猴紝鍏朵腑浜嬩欢绫诲瀷涓?
KVM_SYSTEM_EVENT_WAKEUP銆傚鏋滅敤鎴风┖闂村笇鏈涘搷搴旀鍞ら啋锛屽畠蹇呴』灏?vCPU 鐨?MP 鐘舵€佽缃负
KVM_MP_STATE_RUNNABLE銆傚鏋滀笉杩欐牱鍋氾紝KVM 浼氬湪鍚庣画瀵?KVM_RUN 鐨勮皟鐢ㄤ腑缁х画绛夊緟鍞ら啋浜嬩欢銆?


     濡傛灉鐢ㄦ埛绌洪棿鎵撶畻灏?vCPU 淇濇寔鍦?SUSPENDED 鐘舵€侊紝寮虹儓寤鸿鐢ㄦ埛绌洪棿閲囧彇琛屽姩鎶戝埗鍞ら啋浜嬩欢
     锛堜緥濡傚睆钄芥煇涓腑鏂級銆傚惁鍒欙紝鍚庣画瀵?KVM_RUN 鐨勮皟鐢ㄤ細绔嬪嵆浠?KVM_SYSTEM_EVENT_WAKEUP 浜嬩欢閫€鍑猴紝
     骞舵棤鎰忎腑娴垂 CPU 鍛ㄦ湡銆?

     姝ゅ锛屽鏋滅敤鎴风┖闂撮噰鍙栬鍔ㄦ姂鍒朵簡鍞ら啋浜嬩欢锛屽己鐑堝缓璁畠鍦?vCPU 鍐嶆鍙樹负 RUNNABLE 鏃跺皢鍏?
     鎭㈠鍒板師濮嬬姸鎬併€備緥濡傦紝濡傛灉鐢ㄦ埛绌洪棿灞忚斀浜嗕竴涓寕璧风殑涓柇鏉ユ姂鍒跺敜閱掞紝閭ｄ箞鍦ㄥ皢鎺у埗鏉冧氦杩樼粰
     瀹㈡埛鏈轰箣鍓嶏紝搴旇В闄よ涓柇鐨勫睆钄姐€?

##### For riscv:


鍞竴鏈夋晥鐨勭姸鎬佹槸 KVM_MP_STATE_STOPPED 涓?KVM_MP_STATE_RUNNABLE锛屽畠浠弽鏄?vcpu 鏄惁琚殏鍋溿€?

鍦?LoongArch 涓婏紝浠呬娇鐢?KVM_MP_STATE_RUNNABLE 鐘舵€佹潵鍙嶆槧 vcpu 鏄惁鍙繍琛屻€?

### 4.39 KVM_SET_MP_STATE


:Capability: KVM_CAP_MP_STATE
:Architectures: x86, s390, arm64, riscv, loongarch
:Type: vcpu ioctl
:Parameters: struct kvm_mp_state (in)
:Returns: 0 on success; -1 on error

璁剧疆 vcpu 褰撳墠鐨?澶氬鐞嗗櫒鐘舵€?锛涘弬鏁拌鏄庡弬瑙?KVM_GET_MP_STATE銆?

鍦?x86 涓婏紝姝?ioctl 浠呭湪 KVM_CREATE_IRQCHIP 涔嬪悗鎵嶆湁鐢ㄣ€傚鏋滄病鏈夊唴鏍告€?irqchip锛屽澶勭悊鍣ㄧ姸鎬?
蹇呴』鍦ㄨ繖浜涙灦鏋勪笂鐢辩敤鎴风┖闂寸淮鎶ゃ€?

##### For arm64/riscv:


鍞竴鏈夋晥鐨勭姸鎬佹槸 KVM_MP_STATE_STOPPED 涓?KVM_MP_STATE_RUNNABLE锛屽畠浠弽鏄?vcpu 鏄惁搴旇鏆傚仠銆?

鍦?LoongArch 涓婏紝浠呬娇鐢?KVM_MP_STATE_RUNNABLE 鐘舵€佹潵鍙嶆槧 vcpu 鏄惁鍙繍琛屻€?

### 4.40 KVM_SET_IDENTITY_MAP_ADDR


:Capability: KVM_CAP_SET_IDENTITY_MAP_ADDR
:Architectures: x86
:Type: vm ioctl
:Parameters: unsigned long identity (in)
:Returns: 0 on success, -1 on error

璇?ioctl 瀹氫箟瀹㈡埛鏈虹墿鐞嗗湴鍧€绌洪棿涓竴涓崟椤靛尯鍩熺殑鐗╃悊鍦板潃銆傝鍖哄煙蹇呴』浣嶄簬瀹㈡埛鏈虹墿鐞嗗湴鍧€绌洪棿鐨?
鍓?4GB 涔嬪唴锛屼笖涓嶈兘涓庝换浣曞唴瀛樻Ы鎴栦换浣?mmio 鍦板潃鍐茬獊銆傚鏋滃鎴锋満璁块棶璇ュ唴瀛樺尯鍩燂紝鍙兘浼氬彂鐢熸晠闅溿€?

灏嗗湴鍧€璁剧疆涓?0 浼氬鑷磋鍦板潃琚噸缃负榛樿鍊硷紙0xfffbc000锛夈€?

鍦ㄥ熀浜?Intel 鐨勪富鏈轰笂锛岃 ioctl 鏄繀闇€鐨勩€傚湪 Intel 纭欢涓婇渶瑕佸畠锛屾槸鍥犱负铏氭嫙鍖栧疄鐜颁腑鐨勪竴涓?
鎬紓涔嬪锛堝弬瑙佸皻鏈潰涓栫殑 internals 鏂囨。锛夈€?

濡傛灉鏈変换浣?VCPU 宸茬粡琚垱寤猴紝鍒欎細澶辫触銆?

### 4.41 KVM_SET_BOOT_CPU_ID


:Capability: KVM_CAP_SET_BOOT_CPU_ID
:Architectures: x86
:Type: vm ioctl
:Parameters: unsigned long vcpu_id
:Returns: 0 on success, -1 on error

瀹氫箟鍝釜 vcpu 鏄紩瀵煎鐞嗗櫒锛圔ootstrap Processor锛孊SP锛夈€傚彇鍊间笌 KVM_CREATE_VCPU 涓殑 vcpu id 鐩稿悓銆?
濡傛灉鏈皟鐢ㄦ ioctl锛屽垯榛樿鏄?vcpu 0銆傛 ioctl 蹇呴』鍦?vcpu 鍒涘缓涔嬪墠璋冪敤锛屽惁鍒欎細杩斿洖 EBUSY 閿欒銆?


### 4.42 KVM_GET_XSAVE


:Capability: KVM_CAP_XSAVE
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_xsave (out)
:Returns: 0 on success, -1 on error


```

  struct kvm_xsave {
	__u32 region[1024];
	__u32 extra[0];
  };

```
璇?ioctl 浼氬皢褰撳墠 vcpu 鐨?xsave 缁撴瀯浣撳鍒跺埌鐢ㄦ埛绌洪棿銆?


### 4.43 KVM_SET_XSAVE


:Capability: KVM_CAP_XSAVE and KVM_CAP_XSAVE2
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_xsave (in)
:Returns: 0 on success, -1 on error

```


  struct kvm_xsave {
	__u32 region[1024];
	__u32 extra[0];
  };

```
璇?ioctl 浼氬皢鐢ㄦ埛绌洪棿鐨?xsave 缁撴瀯浣撳鍒跺埌鍐呮牳銆傚畠澶嶅埗鐨勫瓧鑺傛暟绛変簬 KVM_CHECK_EXTENSION(KVM_CAP_XSAVE2)
鍦?vm 鏂囦欢鎻忚堪绗︿笂璋冪敤鏃惰繑鍥炵殑鍊笺€侹VM_CHECK_EXTENSION(KVM_CAP_XSAVE2) 杩斿洖鐨勫ぇ灏忓€兼€绘槸鑷冲皯涓?4096銆?
鐩墠锛屽彧鏈夊綋鏌愪釜鍔ㄦ€佺壒鎬у凡閫氳繃 `arch_prctl()` 鍚敤鏃跺畠鎵嶄細澶т簬 4096锛屼絾杩欏湪鏈潵鍙兘浼氭敼鍙樸€?

struct kvm_xsave 涓悇鐘舵€佷繚瀛樺尯鍩熺殑鍋忕Щ閲忛伒寰涓绘満涓?CPUID 鍙跺瓙 0xD 鐨勫唴瀹广€?


### 4.44 KVM_GET_XCRS


:Capability: KVM_CAP_XCRS
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_xcrs (out)
:Returns: 0 on success, -1 on error

```

  struct kvm_xcr {
	__u32 xcr;
	__u32 reserved;
	__u64 value;
  };

  struct kvm_xcrs {
	__u32 nr_xcrs;
	__u32 flags;
	struct kvm_xcr xcrs[KVM_MAX_XCRS];
	__u64 padding[16];
  };

```
璇?ioctl 浼氬皢褰撳墠 vcpu 鐨?xcrs 澶嶅埗鍒扮敤鎴风┖闂淬€?


### 4.45 KVM_SET_XCRS


:Capability: KVM_CAP_XCRS
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_xcrs (in)
:Returns: 0 on success, -1 on error

```

  struct kvm_xcr {
	__u32 xcr;
	__u32 reserved;
	__u64 value;
  };

  struct kvm_xcrs {
	__u32 nr_xcrs;
	__u32 flags;
	struct kvm_xcr xcrs[KVM_MAX_XCRS];
	__u64 padding[16];
  };

```
璇?ioctl 浼氬皢 vcpu 鐨?xcr 璁剧疆涓虹敤鎴风┖闂存寚瀹氱殑鍊笺€?


### 4.46 KVM_GET_SUPPORTED_CPUID


:Capability: KVM_CAP_EXT_CPUID
:Architectures: x86
:Type: system ioctl
:Parameters: struct kvm_cpuid2 (in/out)
:Returns: 0 on success, -1 on error

```

  struct kvm_cpuid2 {
	__u32 nent;
	__u32 padding;
	struct kvm_cpuid_entry2 entries[0];
  };

  #define KVM_CPUID_FLAG_SIGNIFCANT_INDEX		BIT(0)
  #define KVM_CPUID_FLAG_STATEFUL_FUNC		BIT(1) /* deprecated */
  #define KVM_CPUID_FLAG_STATE_READ_NEXT		BIT(2) /* deprecated */

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
璇?ioctl 杩斿洖鍦ㄩ粯璁ら厤缃笅鐢辩‖浠跺拰 kvm 閮芥敮鎸佺殑 x86 cpuid 鐗规€с€傜敤鎴风┖闂村彲浠ヤ娇鐢ㄨ ioctl 杩斿洖鐨?
淇℃伅鏉ユ瀯閫犱笌纭欢銆佸唴鏍镐互鍙婄敤鎴风┖闂磋兘鍔涗竴鑷寸殑 cpuid 淇℃伅锛堢敤浜?KVM_SET_CPUID2锛夛紝骞朵笌鐢ㄦ埛闇€姹備竴鑷?
锛堜緥濡傦紝鐢ㄦ埛鍙兘甯屾湜绾︽潫 cpuid 浠ユā鎷熻緝鏃х殑纭欢锛屾垨涓轰簡鍦ㄩ泦缇や腑淇濇寔涓€鑷寸殑鐗规€э級銆?

鍔ㄦ€佸惎鐢ㄧ殑鐗规€т綅闇€瑕佸湪璋冪敤姝?ioctl 涔嬪墠閫氳繃 `arch_prctl()` 璇锋眰銆傛湭琚姹傜殑鐗规€т綅涓嶄細鍖呭惈鍦ㄧ粨鏋滀腑銆?

娉ㄦ剰锛屾煇浜涜兘鍔涳紙濡?KVM_CAP_X86_DISABLE_EXITS锛夊彲鑳戒細鏆撮湶 kvm 鍦ㄩ粯璁ら厤缃笅涓嶆敮鎸佺殑 cpuid 鐗规€?
锛堜緥濡?MONITOR锛夈€傚鏋滅敤鎴风┖闂村惎鐢ㄤ簡姝ょ被鑳藉姏锛屽畠璐熻矗閫傚綋鍦颁慨鏀规 ioctl 鐨勭粨鏋溿€?

鐢ㄦ埛绌洪棿璋冪敤 KVM_GET_SUPPORTED_CPUID 鏃讹紝闇€浼犲叆涓€涓?kvm_cpuid2 缁撴瀯浣擄紝鍏?'nent' 瀛楁鎸囩ず鍙彉闀?
鏁扮粍 'entries' 涓殑鏉＄洰鏁伴噺銆傚鏋滄潯鐩暟閲忓お灏戣€屾棤娉曟弿杩?cpu 鑳藉姏锛屼細杩斿洖閿欒锛圗2BIG锛夈€傚鏋滄暟閲?
杩囧锛?nent' 瀛楁浼氳璋冩暣骞惰繑鍥炰竴涓敊璇紙ENOMEM锛夈€傚鏋滄暟閲忔伆濂藉悎閫傦紝'nent' 瀛楁浼氳璋冩暣涓?
'entries' 鏁扮粍涓湁鏁堟潯鐩殑鏁伴噺锛屽苟闅忓悗琚～鍏呫€?

杩斿洖鐨勬潯鐩槸 cpuid 鎸囦护杩斿洖鐨勪富鏈?cpuid锛屽叾涓湭鐭ユ垨涓嶆敮鎸佺殑鐗规€ц灞忚斀銆傛煇浜涚壒鎬э紙渚嬪 x2apic锛夊彲鑳?
涓嶅湪涓绘満 cpu 涓紝浣嗗鏋?kvm 鑳藉楂樻晥鍦版ā鎷熷畠浠紝鍒欎細琚?kvm 鏆撮湶鍑烘潵銆傛瘡涓潯鐩腑鐨勫瓧娈靛畾涔夊涓嬶細

  function:
         鐢ㄤ簬鑾峰彇璇ユ潯鐩殑 eax 鍊?

  index:
         鐢ㄤ簬鑾峰彇璇ユ潯鐩殑 ecx 鍊硷紙閽堝鍙?ecx 褰卞搷鐨勬潯鐩級

  flags:
     浠ヤ笅闆朵釜鎴栧涓殑鎸変綅鎴栵細

        KVM_CPUID_FLAG_SIGNIFCANT_INDEX:
           琛ㄧず index 瀛楁鏈夋晥

   eax, ebx, ecx, edx:
         璇?function/index 缁勫悎涓?cpuid 鎸囦护杩斿洖鐨勫€?

x2APIC锛圕PUID 鍙跺瓙 1锛宔cx[21]锛夊拰 TSC deadline 瀹氭椂鍣紙CPUID 鍙跺瓙 1锛宔cx[24]锛夊彲鑳戒綔涓?true 杩斿洖锛?
浣嗗畠浠緷璧栦簬 KVM_CREATE_IRQCHIP 鐨勫唴鏍告€?
```

  ioctl(KVM_CHECK_EXTENSION, KVM_CAP_TSC_DEADLINE_TIMER)

```
鏉ュ疄鐜帮紱濡傛灉瀹冭繑鍥?true 涓斾綘浣跨敤浜?KVM_CREATE_IRQCHIP锛屾垨鑰呬綘鍦ㄧ敤鎴风┖闂存ā鎷熶簡璇ョ壒鎬э紝閭ｄ箞浣犲氨鍙互
涓?KVM_SET_CPUID2 鍚敤璇ョ壒鎬с€?

鍦?KVM_SET_CPUID2 涓惎鐢?x2APIC 闇€瑕?KVM_CREATE_IRQCHIP锛屽洜涓?KVM 涓嶆敮鎸佸皢 x2APIC MSR 璁块棶杞彂鍒?
鐢ㄦ埛绌洪棿锛屽嵆 KVM 涓嶆敮鎸佸湪鐢ㄦ埛绌洪棿妯℃嫙 x2APIC銆?

### 4.47 KVM_PPC_GET_PVINFO


:Capability: KVM_CAP_PPC_GET_PVINFO
:Architectures: ppc
:Type: vm ioctl
:Parameters: struct kvm_ppc_pvinfo (out)
:Returns: 0 on success, !0 on error

```

  struct kvm_ppc_pvinfo {
	__u32 flags;
	__u32 hcall[4];
	__u8  pad[108];
  };

```
璇?ioctl 浠?vm 涓婁笅鏂囦腑鑾峰彇闇€瑕佸€熷姪璁惧鏍戞垨鍏朵粬鏂瑰紡浼犻€掔粰瀹㈡埛鏈虹殑 PV 鐗瑰畾淇℃伅銆?

hcall 鏁扮粍瀹氫箟浜嗘瀯鎴愪竴娆?hypercall 鐨?4 鏉℃寚浠ゃ€?

濡傛灉浠ュ悗璇ョ粨鏋勪綋娣诲姞浜嗕换浣曢檮鍔犲瓧娈碉紝浼氬湪 flags 浣嶅浘涓缃搴斾簬璇ラ檮鍔犱俊鎭殑涓€涓綅銆?

```

   /* the host supports the ePAPR idle hcall
   #define KVM_PPC_PVINFO_FLAGS_EV_IDLE   (1<<0)

```
### 4.52 KVM_SET_GSI_ROUTING


:Capability: KVM_CAP_IRQ_ROUTING
:Architectures: x86 s390 arm64
:Type: vm ioctl
:Parameters: struct kvm_irq_routing (in)
:Returns: 0 on success, -1 on error

璁剧疆 GSI 璺敱琛ㄦ潯鐩紝瑕嗙洊浠讳綍鍏堝墠璁剧疆鐨勬潯鐩€?

鍦?arm64 涓婏紝GSI 璺敱鏈変互涓嬮檺鍒讹細

- GSI 璺敱涓嶉€傜敤浜?KVM_IRQ_LINE锛岃€屽彧閫傜敤浜?KVM_IRQFD銆?

```

  struct kvm_irq_routing {
	__u32 nr;
	__u32 flags;
	struct kvm_irq_routing_entry entries[0];
  };

```
鐩墠鏈寚瀹氫换浣曟爣蹇楋紝鐩稿簲瀛楁蹇呴』璁剧疆涓洪浂銆?

```

  struct kvm_irq_routing_entry {
	__u32 gsi;
	__u32 type;
	__u32 flags;
	__u32 pad;
	union {
		struct kvm_irq_routing_irqchip irqchip;
		struct kvm_irq_routing_msi msi;
		struct kvm_irq_routing_s390_adapter adapter;
		struct kvm_irq_routing_hv_sint hv_sint;
		struct kvm_irq_routing_xen_evtchn xen_evtchn;
		__u32 pad[8];
	} u;
  };

  /* gsi routing entry types */
  #define KVM_IRQ_ROUTING_IRQCHIP 1
  #define KVM_IRQ_ROUTING_MSI 2
  #define KVM_IRQ_ROUTING_S390_ADAPTER 3
  #define KVM_IRQ_ROUTING_HV_SINT 4
  #define KVM_IRQ_ROUTING_XEN_EVTCHN 5

```
鍦?s390 涓婏紝鍚?ucontrol VM 娣诲姞 KVM_IRQ_ROUTING_S390_ADAPTER 浼氫互 -EINVAL 閿欒琚嫆缁濄€?

flags:

- KVM_MSI_VALID_DEVID锛氫笌 KVM_IRQ_ROUTING_MSI 璺敱鏉＄洰绫诲瀷涓€璧蜂娇鐢紝琛ㄧず devid 瀛楁鍖呭惈涓€涓?
  鏈夋晥鍊笺€傛瘡 VM 鐨?KVM_CAP_MSI_DEVID 鑳藉姏鐢ㄤ簬閫氬憡闇€瑕佹彁渚涜澶?ID 鐨勮姹傘€傚鏋滆鑳藉姏涓嶅彲鐢紝
  鐢ㄦ埛绌洪棿缁濅笉搴旇缃?KVM_MSI_VALID_DEVID 鏍囧織锛屽惁鍒?ioctl 鍙兘浼氬け璐ャ€?
- 鍚﹀垯涓洪浂

```

  struct kvm_irq_routing_irqchip {
	__u32 irqchip;
	__u32 pin;
  };

  struct kvm_irq_routing_msi {
	__u32 address_lo;
	__u32 address_hi;
	__u32 data;
	union {
		__u32 pad;
		__u32 devid;
	};
  };

```
濡傛灉璁剧疆浜?KVM_MSI_VALID_DEVID锛屽垯 devid 鍖呭惈鍐欏叆 MSI 娑堟伅鐨勮澶囩殑鍞竴璁惧鏍囪瘑绗︺€傚浜?PCI锛?
杩欓€氬父鏄綆 16 浣嶄腑鐨?BDF 鏍囪瘑绗︺€?

鍦?x86 涓婏紝闄ら潪鍚敤浜?KVM_CAP_X2APIC_API 鑳藉姏鐨?KVM_X2APIC_API_USE_32BIT_IDS 鐗规€э紝鍚﹀垯 address_hi
浼氳蹇界暐銆傚鏋滃惎鐢紝address_hi 鐨?31-8 浣嶆彁渚涚洰鐨?id 鐨?31-8 浣嶃€俛ddress_hi 鐨?7-0 浣嶅繀椤讳负闆躲€?

```

  struct kvm_irq_routing_s390_adapter {
	__u64 ind_addr;
	__u64 summary_addr;
	__u64 ind_offset;
	__u32 summary_offset;
	__u32 adapter_id;
  };

  struct kvm_irq_routing_hv_sint {
	__u32 vcpu;
	__u32 sint;
  };

  struct kvm_irq_routing_xen_evtchn {
	__u32 port;
	__u32 vcpu;
	__u32 priority;
  };


```
褰?KVM_CAP_XEN_HVM 鍦ㄥ叾鏀寔鐗规€ф寚绀轰腑鍖呭惈 KVM_XEN_HVM_CONFIG_EVTCHN_2LEVEL 浣嶆椂锛屾敮鎸佽矾鐢卞埌 Xen
浜嬩欢閫氶亾銆傚敖绠″瓨鍦?priority 瀛楁锛屼絾鐩墠浠呮敮鎸佸€?KVM_XEN_HVM_CONFIG_EVTCHN_2LEVEL锛岃繖鎰忓懗鐫€閫氳繃
涓ょ骇浜嬩欢閫氶亾鎶曢€掋€傛湭鏉ュ彲鑳戒細娣诲姞 FIFO 浜嬩欢閫氶亾鏀寔銆?


### 4.55 KVM_SET_TSC_KHZ


:Capability: KVM_CAP_TSC_CONTROL / KVM_CAP_VM_TSC_CONTROL
:Architectures: x86
:Type: vcpu ioctl / vm ioctl
:Parameters: virtual tsc_khz
:Returns: 0 on success, -1 on error

鎸囧畾铏氭嫙鏈虹殑 tsc 棰戠巼銆傞鐜囩殑鍗曚綅鏄?KHz銆?

濡傛灉閫氬憡浜?KVM_CAP_VM_TSC_CONTROL 鑳藉姏锛屽畠涔熷彲浠ヤ綔涓?vm ioctl 浣跨敤锛屼互璁剧疆闅忓悗鍒涘缓鐨?vCPU 鐨?
鍒濆 tsc 棰戠巼銆傛敞鎰忥紝vm ioctl 浠呭厑璁稿湪鍒涘缓 vCPU 涔嬪墠浣跨敤銆?

瀵逛簬 TSC 鍙椾繚鎶ょ殑鏈哄瘑璁＄畻锛圕oCo锛塚M锛堝叾 TSC 棰戠巼鍦?VM 鑼冨洿閰嶇疆涓€娆″苟鍦?VM 鐢熷懡鍛ㄦ湡鍐呬繚鎸佷笉鍙橈級锛?
搴斾娇鐢?vm ioctl 鏉ラ厤缃?TSC 棰戠巼锛寁cpu ioctl 涓嶈鏀寔銆?

姝ょ被 CoCo VM 鐨勪緥瀛愶細TDX 瀹㈡埛鏈恒€?

### 4.56 KVM_GET_TSC_KHZ


:Capability: KVM_CAP_GET_TSC_KHZ / KVM_CAP_VM_TSC_CONTROL
:Architectures: x86
:Type: vcpu ioctl / vm ioctl
:Parameters: none
:Returns: virtual tsc-khz on success, negative value on error

杩斿洖瀹㈡埛鏈虹殑 tsc 棰戠巼銆傝繑鍥炲€肩殑鍗曚綅鏄?KHz銆傚鏋滃涓绘満鍏锋湁涓嶇ǔ瀹氱殑 tsc锛岃 ioctl 浼氳繑鍥?-EIO
浣滀负閿欒銆?


### 4.57 KVM_GET_LAPIC


:Capability: KVM_CAP_IRQCHIP
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_lapic_state (out)
:Returns: 0 on success, -1 on error

```

  #define KVM_APIC_REG_SIZE 0x400
  struct kvm_lapic_state {
	char regs[KVM_APIC_REG_SIZE];
  };

```
璇诲彇 Local APIC 瀵勫瓨鍣ㄥ苟灏嗗叾澶嶅埗鍒拌緭鍏ュ弬鏁颁腑銆傛暟鎹牸寮忓拰甯冨眬涓庢灦鏋勬墜鍐屼腑璁板綍鐨勪竴鑷淬€?

濡傛灉鍚敤浜?KVM_CAP_X2APIC_API 鐨?KVM_X2APIC_API_USE_32BIT_IDS 鐗规€э紝閭ｄ箞 APIC_ID 瀵勫瓨鍣ㄧ殑鏍煎紡
鍙栧喅浜庡叾 VCPU 鐨?APIC 妯″紡锛堢敱 MSR_IA32_APICBASE 鎶ュ憡锛夈€倄2APIC 灏?APIC ID 瀛樺偍鍦?APIC_ID 瀵勫瓨鍣?
锛堝瓧鑺?32-35锛変腑銆倄APIC 浠呭厑璁镐竴涓?8 浣嶇殑 APIC ID锛屽瓨鍌ㄥ湪 APIC 瀵勫瓨鍣ㄧ殑 31-24 浣嶏紝鎴栫瓑鏁堝湴瀛樺偍鍦?
struct kvm_lapic_state 鐨?regs 瀛楁鐨勫瓧鑺?35 涓€傚洜姝?KVM_GET_LAPIC 蹇呴』鍦?MSR_IA32_APICBASE 宸?
閫氳繃 KVM_SET_MSR 璁剧疆涔嬪悗璋冪敤銆?

濡傛灉绂佺敤浜?KVM_X2APIC_API_USE_32BIT_IDS 鐗规€э紝struct kvm_lapic_state 濮嬬粓浣跨敤 xAPIC 鏍煎紡銆?


### 4.58 KVM_SET_LAPIC


:Capability: KVM_CAP_IRQCHIP
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_lapic_state (in)
:Returns: 0 on success, -1 on error

```

  #define KVM_APIC_REG_SIZE 0x400
  struct kvm_lapic_state {
	char regs[KVM_APIC_REG_SIZE];
  };

```
灏嗚緭鍏ュ弬鏁板鍒跺埌 Local APIC 瀵勫瓨鍣ㄤ腑銆傛暟鎹牸寮忓拰甯冨眬涓庢灦鏋勬墜鍐屼腑璁板綍鐨勪竴鑷淬€?

APIC ID 瀵勫瓨鍣ㄧ殑鏍煎紡锛坰truct kvm_lapic_state 鐨?regs 瀛楁鐨勫瓧鑺?32-35锛夊彇鍐充簬 KVM_CAP_X2APIC_API
鑳藉姏鐨勭姸鎬併€傚弬瑙?KVM_GET_LAPIC 涓殑璇存槑銆?


### 4.59 KVM_IOEVENTFD


:Capability: KVM_CAP_IOEVENTFD
:Architectures: all
:Type: vm ioctl
:Parameters: struct kvm_ioeventfd (in)
:Returns: 0 on success, !0 on error

璇?ioctl 灏?ioeventfd 闄勫姞鎴栧垎绂诲埌瀹㈡埛鏈哄唴涓€涓悎娉曠殑 pio/mmio 鍦板潃銆傚娉ㄥ唽鍦板潃鐨勫鎴锋満鍐欏叆灏?
瑙﹀彂鎵€鎻愪緵鐨勪簨浠讹紝鑰屼笉鏄鑷翠竴娆￠€€鍑恒€?

```

  struct kvm_ioeventfd {
	__u64 datamatch;
	__u64 addr;        /* legal pio/mmio address */
	__u32 len;         /* 0, 1, 2, 4, or 8 bytes    */
	__s32 fd;
	__u32 flags;
	__u8  pad[36];
  };

```
瀵逛簬 s390 涓?virtio-ccw 璁惧鐨勭壒娈婃儏鍐碉紝ioevent 鍖归厤鐨勬槸涓€涓瓙閫氶亾/virtqueue 鍏冪粍锛岃€屼笉鏄湴鍧€銆?

```

  #define KVM_IOEVENTFD_FLAG_DATAMATCH (1 << kvm_ioeventfd_flag_nr_datamatch)
  #define KVM_IOEVENTFD_FLAG_PIO       (1 << kvm_ioeventfd_flag_nr_pio)
  #define KVM_IOEVENTFD_FLAG_DEASSIGN  (1 << kvm_ioeventfd_flag_nr_deassign)
  #define KVM_IOEVENTFD_FLAG_VIRTIO_CCW_NOTIFY \
	(1 << kvm_ioeventfd_flag_nr_virtio_ccw_notify)

```
濡傛灉璁剧疆浜?datamatch 鏍囧織锛屽垯鍙湁褰撳啓鍏ユ敞鍐屽湴鍧€鐨勫€肩瓑浜?struct kvm_ioeventfd 涓殑 datamatch 鏃讹紝
鎵嶄細瑙﹀彂璇ヤ簨浠躲€?

瀵逛簬 virtio-ccw 璁惧锛宎ddr 鍖呭惈瀛愰€氶亾 id锛宒atamatch 鍖呭惈 virtqueue 绱㈠紩銆?

鍊熷姪 KVM_CAP_IOEVENTFD_ANY_LENGTH锛屽厑璁搁暱搴︿负 0 鐨?ioeventfd锛屽唴鏍稿皢蹇界暐瀹㈡埛鏈哄啓鍏ョ殑闀垮害锛屽苟鍙兘
鑾峰緱鏇村揩鐨?vmexit銆傝繖绉嶅姞閫熷彲鑳藉彧閫傜敤浜庣壒瀹氭灦鏋勶紝浣?ioeventfd 鍦ㄤ换浣曟儏鍐典笅閮借兘宸ヤ綔銆?

### 4.60 KVM_DIRTY_TLB


:Capability: KVM_CAP_SW_TLB
:Architectures: ppc
:Type: vcpu ioctl
:Parameters: struct kvm_dirty_tlb (in)
:Returns: 0 on success, -1 on error

```

  struct kvm_dirty_tlb {
	__u64 bitmap;
	__u32 num_dirty;
  };

```
姣忓綋鐢ㄦ埛绌洪棿鏇存敼浜嗗叡浜?TLB 涓殑涓€涓潯鐩椂锛屽繀椤诲湪鍏宠仈鐨?vcpu 涓婅皟鐢?KVM_RUN 涔嬪墠璋冪敤姝?ioctl銆?

"bitmap" 瀛楁鏄竴涓暟缁勭殑鐢ㄦ埛绌洪棿鍦板潃銆傝鏁扮粍鐢辫嫢骞蹭綅缁勬垚锛屼綅鏁扮瓑浜庣敱涓婃鎴愬姛璋冪敤
`KVM_ENABLE_CAP(KVM_CAP_SW_TLB)` 纭畾鐨?TLB 鏉＄洰鎬绘暟锛屽悜涓婅垗鍏ュ埌鏈€鎺ヨ繎鐨?64 鐨勫€嶆暟銆?

姣忎竴浣嶅搴斾竴涓?TLB 鏉＄洰锛岄『搴忎笌鍏变韩 TLB 鏁扮粍涓殑椤哄簭鐩稿悓銆?

璇ユ暟缁勪负灏忕搴忥細浣?0 鏄涓€涓瓧鑺傜殑鏈€浣庢湁鏁堜綅锛屼綅 8 鏄浜屼釜瀛楄妭鐨勬渶浣庢湁鏁堜綅锛屼緷姝ょ被鎺ㄣ€傝繖閬垮厤浜?
鍥犲瓧闀夸笉鍚岃€屽甫鏉ョ殑浠讳綍澶嶆潅鎬с€?

"num_dirty" 瀛楁鏄粰 KVM 鐨勪竴涓€ц兘鎻愮ず锛岀敤浜庡垽鏂畠鏄惁搴旇璺宠繃澶勭悊浣嶅浘鑰岀洿鎺ヤ娇鎵€鏈夊唴瀹瑰け鏁堛€傚畠
蹇呴』璁剧疆涓轰綅鍥句腑琚疆浣嶇殑浣嶆暟銆?


### 4.62 KVM_CREATE_SPAPR_TCE


:Capability: KVM_CAP_SPAPR_TCE
:Architectures: powerpc
:Type: vm ioctl
:Parameters: struct kvm_create_spapr_tce (in)
:Returns: file descriptor for manipulating the created TCE table

杩欏皢鍒涘缓涓€涓櫄鎷?TCE锛堣浆鎹㈡帶鍒舵潯鐩?translation control entry锛夎〃锛屽畠鏄?PAPR 椋庢牸铏氭嫙 I/O 鐨?
IOMMU銆傚畠鐢ㄤ簬灏嗚櫄鎷?I/O 涓娇鐢ㄧ殑閫昏緫鍦板潃杞崲涓哄鎴锋満鐗╃悊鍦板潃锛屽苟涓?PAPR 铏氭嫙 I/O 鎻愪緵鍒嗘暎/鑱氶泦
锛坰catter/gather锛夎兘鍔涖€?

```

  /* for KVM_CAP_SPAPR_TCE */
  struct kvm_create_spapr_tce {
	__u64 liobn;
	__u32 window_size;
  };

```
liobn 瀛楁缁欏嚭浜嗚涓哄叾鍒涘缓 TCE 琛ㄧ殑閫昏緫 IO 鎬荤嚎鍙枫€倃indow_size 瀛楁鎸囧畾浜嗚 TCE 琛ㄥ皢杞崲鐨?DMA
绐楀彛澶у皬鈥斺€旇琛ㄥ皢涓?DMA 绐楀彛鐨勬瘡 4kiB 鍖呭惈涓€涓?64 浣嶇殑 TCE 鏉＄洰銆?

褰撳鎴锋満瀵瑰凡缁忎娇鐢ㄦ ioctl() 鍒涘缓浜?TCE 琛ㄧ殑 liobn 鍙戝嚭 H_PUT_TCE hcall 鏃讹紝鍐呮牳灏嗗湪瀹炴ā寮忎笅澶勭悊
瀹冿紝鏇存柊 TCE 琛ㄣ€傞拡瀵瑰叾浠?liobn 鐨?H_PUT_TCE 璋冪敤浼氬鑷?vm 閫€鍑猴紝蹇呴』鐢辩敤鎴风┖闂村鐞嗐€?

杩斿洖鍊兼槸涓€涓枃浠舵弿杩扮锛屽彲浠ヤ紶閫掔粰 mmap(2) 浠ュ皢鍒涘缓鐨?TCE 琛ㄦ槧灏勫埌鐢ㄦ埛绌洪棿銆傝繖鍏佽鐢ㄦ埛绌洪棿璇诲彇
鐢卞唴鏍稿鐞嗙殑 H_PUT_TCE 璋冪敤鎵€鍐欏叆鐨勬潯鐩紝涔熷厑璁哥敤鎴风┖闂寸洿鎺ユ洿鏂?TCE 琛紝杩欏湪鏌愪簺鎯呭喌涓嬪緢鏈夌敤銆?


### 4.64 KVM_NMI


:Capability: KVM_CAP_USER_NMI
:Architectures: x86
:Type: vcpu ioctl
:Parameters: none
:Returns: 0 on success, -1 on error

鍦ㄧ嚎绋嬬殑 vcpu 涓婃帓闃熶竴涓?NMI銆傛敞鎰忥紝杩欎粎鍦ㄦ湭璋冪敤 KVM_CREATE_IRQCHIP 鏃舵湁鏄庣‘瀹氫箟锛屽洜涓鸿繖鏄櫄鎷?
cpu 鏍稿績涓庤櫄鎷?Local APIC 涔嬮棿鐨勬帴鍙ｃ€傚湪璋冪敤 KVM_CREATE_IRQCHIP 涔嬪悗锛岃鎺ュ彛瀹屽叏鍦ㄥ唴鏍镐腑妯℃嫙銆?

瑕佷娇鐢ㄥ畠鏉ラ厤鍚?KVM_CREATE_IRQCHIP 妯℃嫙 LINT1 杈撳叆锛岃浣跨敤浠ヤ笅绠楁硶锛?

  - 鏆傚仠 vcpu
  - 璇诲彇 Local APIC 鐨勭姸鎬侊紙KVM_GET_LAPIC锛?
  - 妫€鏌ユ洿鏀?LINT1 鏄惁浼氭帓闃熶竴涓?NMI锛堝弬瑙?LINT1 鐨?LVT 鏉＄洰锛?
  - 濡傛灉鏄紝鍙戝嚭 KVM_NMI
  - 鎭㈠ vcpu

鏌愪簺瀹㈡埛鏈哄皢 LINT1 NMI 杈撳叆閰嶇疆涓哄紩鍙?panic锛屼互鍗忓姪璋冭瘯銆?


### 4.65 KVM_S390_UCAS_MAP


:Capability: KVM_CAP_S390_UCONTROL
:Architectures: s390
:Type: vcpu ioctl
:Parameters: struct kvm_s390_ucas_mapping (in)
:Returns: 0 in case of success

```

	struct kvm_s390_ucas_mapping {
		__u64 user_addr;
		__u64 vcpu_addr;
		__u64 length;
	};

```
璇?ioctl 灏嗕粠 "user_addr" 寮€濮嬨€侀暱搴︿负 "length" 鐨勫唴瀛樻槧灏勫埌浠?"vcpu_addr" 寮€濮嬬殑 vcpu 鍦板潃绌洪棿銆?
鎵€鏈夊弬鏁伴兘闇€瑕佹寜 1 鍏嗗瓧鑺傚榻愩€?


### 4.66 KVM_S390_UCAS_UNMAP


:Capability: KVM_CAP_S390_UCONTROL
:Architectures: s390
:Type: vcpu ioctl
:Parameters: struct kvm_s390_ucas_mapping (in)
:Returns: 0 in case of success

```

	struct kvm_s390_ucas_mapping {
		__u64 user_addr;
		__u64 vcpu_addr;
		__u64 length;
	};

```
璇?ioctl 鍙栨秷鏄犲皠浠?"vcpu_addr" 寮€濮嬨€侀暱搴︿负 "length" 鐨?vcpu 鍦板潃绌洪棿涓殑鍐呭瓨銆?user_addr" 瀛楁
琚拷鐣ャ€傛墍鏈夊弬鏁伴兘闇€瑕佹寜 1 鍏嗗瓧鑺傚榻愩€?


### 4.67 KVM_S390_VCPU_FAULT


:Capability: KVM_CAP_S390_UCONTROL
:Architectures: s390
:Type: vcpu ioctl
:Parameters: vcpu absolute address (in)
:Returns: 0 in case of success

璇ヨ皟鐢ㄤ細鍦ㄨ櫄鎷?cpu 鐨勫湴鍧€绌洪棿锛堝浜庣敤鎴锋帶鍒剁殑铏氭嫙鏈猴級鎴栬櫄鎷熸満鐨勫湴鍧€绌洪棿锛堝浜庡父瑙勮櫄鎷熸満锛変笂
鍒涘缓涓€涓〉琛ㄦ潯鐩€傝繖浠呭娆¤缂洪〉锛坢inor fault锛夋湁鏁堬紝鍥犳寤鸿浜嬪厛閫氳繃鐢ㄦ埛椤佃〃璁块棶鐩稿叧鍐呭瓨椤点€?
杩欏浜庡鐞嗙敤鎴锋帶鍒惰櫄鎷熸満鐨勬湁鏁堟€ф嫤鎴紙validity intercept锛夐潪甯告湁鐢紝鍙湪璋冪敤 KVM_RUN ioctl 涔嬪墠
灏嗚櫄鎷?cpu 鐨?lowcore 椤电己椤佃鍏ャ€?
### 4.68 KVM_SET_ONE_REG


:Capability: KVM_CAP_ONE_REG
:Architectures: all
:Type: vcpu ioctl
:Parameters: struct kvm_one_reg (in)
:Returns: 0 on success, negative value on failure

閿欒鐮侊細

  ======   ============================================================
  ENOENT   娌℃湁璇ュ瘎瀛樺櫒
  EINVAL   鏃犳晥鐨勫瘎瀛樺櫒 ID锛屾垨娌℃湁璇ュ瘎瀛樺櫒锛屾垨涓?s390 涓婂彈淇濇姢铏氭嫙鍖?
           妯″紡涓嬬殑 VM 涓€璧蜂娇鐢?
  EPERM    (arm64) 鍦?vcpu 瀹氱锛坒inalization锛変箣鍓嶄笉鍏佽璁块棶璇ュ瘎瀛樺櫒
  EBUSY    (riscv) vcpu 鑷冲皯杩愯杩囦竴娆′箣鍚庝笉鍏佽鏇存敼瀵勫瓨鍣ㄥ€?
  ======   ============================================================

锛堣繖浜涢敊璇爜浠呬緵鍙傝€冿細涓嶈渚濊禆鍦ㄧ壒瀹氭儏鍐典笅杩斿洖鐗瑰畾鐨勯敊璇爜銆傦級

```

  struct kvm_one_reg {
       __u64 id;
       __u64 addr;
 };

```
浣跨敤璇?ioctl锛屽彲浠ラ€氳繃浼犲叆鐨?struct kvm_one_reg 灏嗗崟涓?vcpu 瀵勫瓨鍣ㄨ缃负鐢ㄦ埛绌洪棿鎸囧畾鐨勭壒瀹氬€硷紝
鍏朵腑 id 鎸囦唬濡備笅鎵€杩扮殑瀵勫瓨鍣ㄦ爣璇嗙锛宎ddr 鏄寚鍚戠浉搴斿ぇ灏忓彉閲忕殑鎸囬拡銆傚瘎瀛樺櫒鍙互鏋舵瀯鏃犲叧锛?
涔熷彲浠ユ灦鏋勭浉鍏炽€傛瘡绉嶉兘鏈夊悇鑷殑鎿嶄綔鑼冨洿鍜屽悇鑷殑甯搁噺涓庡搴︺€傝杩借釜宸插疄鐜扮殑瀵勫瓨鍣紝璇峰弬瑙?
浠ヤ笅鍒楄〃锛?

  ======= =============================== ============
  Arch              Register              Width (bits)
  ======= =============================== ============
  PPC     KVM_REG_PPC_HIOR                64
  PPC     KVM_REG_PPC_IAC1                64
  PPC     KVM_REG_PPC_IAC2                64
  PPC     KVM_REG_PPC_IAC3                64
  PPC     KVM_REG_PPC_IAC4                64
  PPC     KVM_REG_PPC_DAC1                64
  PPC     KVM_REG_PPC_DAC2                64
  PPC     KVM_REG_PPC_DABR                64
  PPC     KVM_REG_PPC_DSCR                64
  PPC     KVM_REG_PPC_PURR                64
  PPC     KVM_REG_PPC_SPURR               64
  PPC     KVM_REG_PPC_DAR                 64
  PPC     KVM_REG_PPC_DSISR               32
  PPC     KVM_REG_PPC_AMR                 64
  PPC     KVM_REG_PPC_UAMOR               64
  PPC     KVM_REG_PPC_MMCR0               64
  PPC     KVM_REG_PPC_MMCR1               64
  PPC     KVM_REG_PPC_MMCRA               64
  PPC     KVM_REG_PPC_MMCR2               64
  PPC     KVM_REG_PPC_MMCRS               64
  PPC     KVM_REG_PPC_MMCR3               64
  PPC     KVM_REG_PPC_SIAR                64
  PPC     KVM_REG_PPC_SDAR                64
  PPC     KVM_REG_PPC_SIER                64
  PPC     KVM_REG_PPC_SIER2               64
  PPC     KVM_REG_PPC_SIER3               64
  PPC     KVM_REG_PPC_PMC1                32
  PPC     KVM_REG_PPC_PMC2                32
  PPC     KVM_REG_PPC_PMC3                32
  PPC     KVM_REG_PPC_PMC4                32
  PPC     KVM_REG_PPC_PMC5                32
  PPC     KVM_REG_PPC_PMC6                32
  PPC     KVM_REG_PPC_PMC7                32
  PPC     KVM_REG_PPC_PMC8                32
  PPC     KVM_REG_PPC_FPR0                64
  ...
  PPC     KVM_REG_PPC_FPR31               64
  PPC     KVM_REG_PPC_VR0                 128
  ...
  PPC     KVM_REG_PPC_VR31                128
  PPC     KVM_REG_PPC_VSR0                128
  ...
  PPC     KVM_REG_PPC_VSR31               128
  PPC     KVM_REG_PPC_FPSCR               64
  PPC     KVM_REG_PPC_VSCR                32
  PPC     KVM_REG_PPC_VPA_ADDR            64
  PPC     KVM_REG_PPC_VPA_SLB             128
  PPC     KVM_REG_PPC_VPA_DTL             128
  PPC     KVM_REG_PPC_EPCR                32
  PPC     KVM_REG_PPC_EPR                 32
  PPC     KVM_REG_PPC_TCR                 32
  PPC     KVM_REG_PPC_TSR                 32
  PPC     KVM_REG_PPC_OR_TSR              32
  PPC     KVM_REG_PPC_CLEAR_TSR           32
  PPC     KVM_REG_PPC_MAS0                32
  PPC     KVM_REG_PPC_MAS1                32
  PPC     KVM_REG_PPC_MAS2                64
  PPC     KVM_REG_PPC_MAS7_3              64
  PPC     KVM_REG_PPC_MAS4                32
  PPC     KVM_REG_PPC_MAS6                32
  PPC     KVM_REG_PPC_MMUCFG              32
  PPC     KVM_REG_PPC_TLB0CFG             32
  PPC     KVM_REG_PPC_TLB1CFG             32
  PPC     KVM_REG_PPC_TLB2CFG             32
  PPC     KVM_REG_PPC_TLB3CFG             32
  PPC     KVM_REG_PPC_TLB0PS              32
  PPC     KVM_REG_PPC_TLB1PS              32
  PPC     KVM_REG_PPC_TLB2PS              32
  PPC     KVM_REG_PPC_TLB3PS              32
  PPC     KVM_REG_PPC_EPTCFG              32
  PPC     KVM_REG_PPC_ICP_STATE           64
  PPC     KVM_REG_PPC_VP_STATE            128
  PPC     KVM_REG_PPC_TB_OFFSET           64
  PPC     KVM_REG_PPC_SPMC1               32
  PPC     KVM_REG_PPC_SPMC2               32
  PPC     KVM_REG_PPC_IAMR                64
  PPC     KVM_REG_PPC_TFHAR               64
  PPC     KVM_REG_PPC_TFIAR               64
  PPC     KVM_REG_PPC_TEXASR              64
  PPC     KVM_REG_PPC_FSCR                64
  PPC     KVM_REG_PPC_PSPB                32
  PPC     KVM_REG_PPC_EBBHR               64
  PPC     KVM_REG_PPC_EBBRR               64
  PPC     KVM_REG_PPC_BESCR               64
  PPC     KVM_REG_PPC_TAR                 64
  PPC     KVM_REG_PPC_DPDES               64
  PPC     KVM_REG_PPC_DAWR                64
  PPC     KVM_REG_PPC_DAWRX               64
  PPC     KVM_REG_PPC_CIABR               64
  PPC     KVM_REG_PPC_IC                  64
  PPC     KVM_REG_PPC_VTB                 64
  PPC     KVM_REG_PPC_CSIGR               64
  PPC     KVM_REG_PPC_TACR                64
  PPC     KVM_REG_PPC_TCSCR               64
  PPC     KVM_REG_PPC_PID                 64
  PPC     KVM_REG_PPC_ACOP                64
  PPC     KVM_REG_PPC_VRSAVE              32
  PPC     KVM_REG_PPC_LPCR                32
  PPC     KVM_REG_PPC_LPCR_64             64
  PPC     KVM_REG_PPC_PPR                 64
  PPC     KVM_REG_PPC_ARCH_COMPAT         32
  PPC     KVM_REG_PPC_DABRX               32
  PPC     KVM_REG_PPC_WORT                64
  PPC	  KVM_REG_PPC_SPRG9               64
  PPC	  KVM_REG_PPC_DBSR                32
  PPC     KVM_REG_PPC_TIDR                64
  PPC     KVM_REG_PPC_PSSCR               64
  PPC     KVM_REG_PPC_DEC_EXPIRY          64
  PPC     KVM_REG_PPC_PTCR                64
  PPC     KVM_REG_PPC_HASHKEYR            64
  PPC     KVM_REG_PPC_HASHPKEYR           64
  PPC     KVM_REG_PPC_DAWR1               64
  PPC     KVM_REG_PPC_DAWRX1              64
  PPC     KVM_REG_PPC_DEXCR               64
  PPC     KVM_REG_PPC_TM_GPR0             64
  ...
  PPC     KVM_REG_PPC_TM_GPR31            64
  PPC     KVM_REG_PPC_TM_VSR0             128
  ...
  PPC     KVM_REG_PPC_TM_VSR63            128
  PPC     KVM_REG_PPC_TM_CR               64
  PPC     KVM_REG_PPC_TM_LR               64
  PPC     KVM_REG_PPC_TM_CTR              64
  PPC     KVM_REG_PPC_TM_FPSCR            64
  PPC     KVM_REG_PPC_TM_AMR              64
  PPC     KVM_REG_PPC_TM_PPR              64
  PPC     KVM_REG_PPC_TM_VRSAVE           64
  PPC     KVM_REG_PPC_TM_VSCR             32
  PPC     KVM_REG_PPC_TM_DSCR             64
  PPC     KVM_REG_PPC_TM_TAR              64
  PPC     KVM_REG_PPC_TM_XER              64

  MIPS    KVM_REG_MIPS_R0                 64
  ...
  MIPS    KVM_REG_MIPS_R31                64
  MIPS    KVM_REG_MIPS_HI                 64
  MIPS    KVM_REG_MIPS_LO                 64
  MIPS    KVM_REG_MIPS_PC                 64
  MIPS    KVM_REG_MIPS_CP0_INDEX          32
  MIPS    KVM_REG_MIPS_CP0_ENTRYLO0       64
  MIPS    KVM_REG_MIPS_CP0_ENTRYLO1       64
  MIPS    KVM_REG_MIPS_CP0_CONTEXT        64
  MIPS    KVM_REG_MIPS_CP0_CONTEXTCONFIG  32
  MIPS    KVM_REG_MIPS_CP0_USERLOCAL      64
  MIPS    KVM_REG_MIPS_CP0_XCONTEXTCONFIG 64
  MIPS    KVM_REG_MIPS_CP0_PAGEMASK       32
  MIPS    KVM_REG_MIPS_CP0_PAGEGRAIN      32
  MIPS    KVM_REG_MIPS_CP0_SEGCTL0        64
  MIPS    KVM_REG_MIPS_CP0_SEGCTL1        64
  MIPS    KVM_REG_MIPS_CP0_SEGCTL2        64
  MIPS    KVM_REG_MIPS_CP0_PWBASE         64
  MIPS    KVM_REG_MIPS_CP0_PWFIELD        64
  MIPS    KVM_REG_MIPS_CP0_PWSIZE         64
  MIPS    KVM_REG_MIPS_CP0_WIRED          32
  MIPS    KVM_REG_MIPS_CP0_PWCTL          32
  MIPS    KVM_REG_MIPS_CP0_HWRENA         32
  MIPS    KVM_REG_MIPS_CP0_BADVADDR       64
  MIPS    KVM_REG_MIPS_CP0_BADINSTR       32
  MIPS    KVM_REG_MIPS_CP0_BADINSTRP      32
  MIPS    KVM_REG_MIPS_CP0_COUNT          32
  MIPS    KVM_REG_MIPS_CP0_ENTRYHI        64
  MIPS    KVM_REG_MIPS_CP0_COMPARE        32
  MIPS    KVM_REG_MIPS_CP0_STATUS         32
  MIPS    KVM_REG_MIPS_CP0_INTCTL         32
  MIPS    KVM_REG_MIPS_CP0_CAUSE          32
  MIPS    KVM_REG_MIPS_CP0_EPC            64
  MIPS    KVM_REG_MIPS_CP0_PRID           32
  MIPS    KVM_REG_MIPS_CP0_EBASE          64
  MIPS    KVM_REG_MIPS_CP0_CONFIG         32
  MIPS    KVM_REG_MIPS_CP0_CONFIG1        32
  MIPS    KVM_REG_MIPS_CP0_CONFIG2        32
  MIPS    KVM_REG_MIPS_CP0_CONFIG3        32
  MIPS    KVM_REG_MIPS_CP0_CONFIG4        32
  MIPS    KVM_REG_MIPS_CP0_CONFIG5        32
  MIPS    KVM_REG_MIPS_CP0_CONFIG7        32
  MIPS    KVM_REG_MIPS_CP0_XCONTEXT       64
  MIPS    KVM_REG_MIPS_CP0_ERROREPC       64
  MIPS    KVM_REG_MIPS_CP0_KSCRATCH1      64
  MIPS    KVM_REG_MIPS_CP0_KSCRATCH2      64
  MIPS    KVM_REG_MIPS_CP0_KSCRATCH3      64
  MIPS    KVM_REG_MIPS_CP0_KSCRATCH4      64
  MIPS    KVM_REG_MIPS_CP0_KSCRATCH5      64
  MIPS    KVM_REG_MIPS_CP0_KSCRATCH6      64
  MIPS    KVM_REG_MIPS_CP0_MAAR(0..63)    64
  MIPS    KVM_REG_MIPS_COUNT_CTL          64
  MIPS    KVM_REG_MIPS_COUNT_RESUME       64
  MIPS    KVM_REG_MIPS_COUNT_HZ           64
  MIPS    KVM_REG_MIPS_FPR_32(0..31)      32
  MIPS    KVM_REG_MIPS_FPR_64(0..31)      64
  MIPS    KVM_REG_MIPS_VEC_128(0..31)     128
  MIPS    KVM_REG_MIPS_FCR_IR             32
  MIPS    KVM_REG_MIPS_FCR_CSR            32
  MIPS    KVM_REG_MIPS_MSA_IR             32
  MIPS    KVM_REG_MIPS_MSA_CSR            32
  ======= =============================== ============

ARM 瀵勫瓨鍣ㄦ槧灏勪娇鐢ㄤ綆 32 浣嶃€傚叾涓殑楂?16 浣嶆槸瀵勫瓨鍣ㄧ粍绫诲瀷锛屾垨鍗忓鐞嗗櫒缂栧彿锛?

```

  0x4020 0000 0010 <index into the kvm_regs struct:16>

```
```

  0x4020 0000 000F <zero:1> <crn:4> <crm:4> <opc1:4> <opc2:3>

```
```

  0x4030 0000 000F <zero:1> <zero:4> <crm:4> <opc1:4> <zero:3>

```
```

  0x4020 0000 0011 00 <csselr:8>

```
```

  0x4020 0000 0012 1 <regno:12>

```
```

  0x4030 0000 0012 0 <regno:12>

```
```

  0x4030 0000 0014 <regno:16>


```
arm64 瀵勫瓨鍣ㄦ槧灏勪娇鐢ㄤ綆 32 浣嶃€傚叾涓殑楂?16 浣嶆槸瀵勫瓨鍣ㄧ粍绫诲瀷锛屾垨鍗忓鐞嗗櫒缂栧彿锛?

arm64 鏍稿績/FP-SIMD 瀵勫瓨鍣ㄥ叿鏈変互涓?id 浣嶆ā寮忋€傛敞鎰忥紝璁块棶澶у皬鏄彲鍙樼殑锛屽洜涓?kvm_regs 缁撴瀯浣?
鍖呭惈浠?32 鍒?128 浣嶄笉绛夌殑鍏冪礌銆俰ndex 鏄竴涓?32 浣嶇殑
```

  0x60x0 0000 0010 <index into the kvm_regs struct:16>

```
鍏蜂綋鏉ヨ锛?

======================= ========= ===== =======================================
    Encoding            Register  Bits  kvm_regs member
======================= ========= ===== =======================================
  0x6030 0000 0010 0000 X0          64  regs.regs[^0^]
  0x6030 0000 0010 0002 X1          64  regs.regs[^1^]
  ...
  0x6030 0000 0010 003c X30         64  regs.regs[^30^]
  0x6030 0000 0010 003e SP          64  regs.sp
  0x6030 0000 0010 0040 PC          64  regs.pc
  0x6030 0000 0010 0042 PSTATE      64  regs.pstate
  0x6030 0000 0010 0044 SP_EL1      64  sp_el1
  0x6030 0000 0010 0046 ELR_EL1     64  elr_el1
  0x6030 0000 0010 0048 SPSR_EL1    64  spsr[KVM_SPSR_EL1] (alias SPSR_SVC)
  0x6030 0000 0010 004a SPSR_ABT    64  spsr[KVM_SPSR_ABT]
  0x6030 0000 0010 004c SPSR_UND    64  spsr[KVM_SPSR_UND]
  0x6030 0000 0010 004e SPSR_IRQ    64  spsr[KVM_SPSR_IRQ]
  0x6030 0000 0010 0050 SPSR_FIQ    64  spsr[KVM_SPSR_FIQ]
  0x6040 0000 0010 0054 V0         128  fp_regs.vregs[^0^]    [^1^]_
  0x6040 0000 0010 0058 V1         128  fp_regs.vregs[^1^]    [^1^]_
  ...
  0x6040 0000 0010 00d0 V31        128  fp_regs.vregs[^31^]   [^1^]_
  0x6020 0000 0010 00d4 FPSR        32  fp_regs.fpsr
  0x6020 0000 0010 00d5 FPCR        32  fp_regs.fpcr
======================= ========= ===== =======================================

       KVM_ARM_VCPU_INIT銆?

       瀵逛簬宸插惎鐢?SVE 鐨?vcpu锛堣涓嬫枃锛夛紝鍙互閫氳繃鐩稿簲 SVE Zn 瀵勫瓨鍣ㄧ殑浣?[127:0]
       璁块棶绛変环鐨勫瘎瀛樺櫒鍐呭銆?

```

  0x6020 0000 0011 00 <csselr:8>

```
```

  0x6030 0000 0013 <op0:2> <op1:3> <crn:4> <crm:4> <op2:3>

```

     鏈変袱涓郴缁熷瘎瀛樺櫒 ID 涓嶉伒寰寚瀹氱殑妯″紡銆傚畠浠槸 KVM_REG_ARM_TIMER_CVAL 鍜?
     KVM_REG_ARM_TIMER_CNT锛屽垎鍒槧灏勫埌绯荤粺瀵勫瓨鍣?CNTV_CVAL_EL0 鍜?CNTVCT_EL0銆?
     杩欎袱涓殑鍊艰鎰忓鍦颁氦鎹簡锛岃繖鎰忓懗鐫€ TIMER_CVAL 娲剧敓鑷?CNTVCT_EL0 鐨勫瘎瀛樺櫒缂栫爜锛?
     鑰?TIMER_CNT 娲剧敓鑷?CNTV_CVAL_EL0 鐨勫瘎瀛樺櫒缂栫爜銆傜敱浜庤繖鏄?API锛屽繀椤讳繚鎸佺幇鐘躲€?

```

  0x6030 0000 0014 <regno:16>

```
```

  0x6080 0000 0015 00 <n:5> <slice:5>   Zn bits[2048*slice + 2047 : 2048*slice]
  0x6050 0000 0015 04 <n:4> <slice:5>   Pn bits[256*slice + 255 : 256*slice]
  0x6050 0000 0015 060 <slice:5>        FFR bits[256*slice + 255 : 256*slice]
  0x6060 0000 0015 ffff                 KVM_REG_ARM64_SVE_VLS pseudo-register

```
褰?2048 * slice >= 128 * max_vq 鏃讹紝璁块棶璇ュ瘎瀛樺櫒 ID 浼氬け璐ュ苟杩斿洖 ENOENT銆俶ax_vq 鏄?vcpu 鏀寔鐨?
鏈€澶у悜閲忛暱搴︼紙浠?128 浣嶅洓瀛椾负鍗曚綅锛夛細瑙佷笅鏂囩殑 [^2^]_銆?

杩欎簺瀵勫瓨鍣ㄥ彧鑳藉湪鍚敤浜?SVE 鐨?vcpu 涓婅闂€傝瑙?KVM_ARM_VCPU_INIT銆?

姝ゅ锛岄櫎浜?KVM_REG_ARM64_SVE_VLS 涔嬪锛屽湪 vcpu 鐨?SVE 閰嶇疆閫氳繃
KVM_ARM_VCPU_FINALIZE(KVM_ARM_VCPU_SVE) 瀹氱涔嬪墠锛屾棤娉曡闂繖浜涘瘎瀛樺櫒銆傚叧浜庢杩囩▼鐨勬洿澶氫俊鎭紝
璇峰弬瑙?KVM_ARM_VCPU_INIT 鍜?KVM_ARM_VCPU_FINALIZE銆?

KVM_REG_ARM64_SVE_VLS 鏄竴涓吉瀵勫瓨鍣紝鍏佽鐢ㄦ埛绌洪棿鍙戠幇骞堕厤缃?vcpu 鎵€鏀寔鐨勫悜閲忛暱搴﹂泦鍚堛€?
閫氳繃 KVM_GET_ONE_REG 鎴?KVM_SET_ONE_REG 鍦ㄧ敤鎴峰唴瀛樹箣闂翠紶杈撴椂锛岃瀵勫瓨鍣ㄧ殑鍊间负
__u64[KVM_ARM64_SVE_VLS_WORDS] 绫诲瀷锛屽苟灏嗗悜閲忛暱搴﹂泦鍚堢紪鐮佷负
```

  __u64 vector_lengths[KVM_ARM64_SVE_VLS_WORDS];

  if (vq >= SVE_VQ_MIN && vq <= SVE_VQ_MAX &&
      ((vector_lengths[(vq - KVM_ARM64_SVE_VQ_MIN) / 64] >>
		((vq - KVM_ARM64_SVE_VQ_MIN) % 64)) & 1))
	/* Vector length vq * 16 bytes supported */
  else
	/* Vector length vq * 16 bytes not supported */

```
       max_vq銆傝繖鏄 vcpu 涓婂鎴锋満鍙敤鐨勬渶澶у悜閲忛暱搴︼紝骞跺喅瀹氫簡閫氳繃姝?ioctl 鎺ュ彛鍙鐨?
       瀵勫瓨鍣ㄥ垏鐗囥€?

锛堝叧浜?"vq" 鍛藉悕娉曠殑瑙ｉ噴锛岃鍙傝 Documentation/arch/arm64/sve.rst銆傦級

KVM_REG_ARM64_SVE_VLS 浠呭湪 KVM_ARM_VCPU_INIT 涔嬪悗鍙闂€侹VM_ARM_VCPU_INIT 灏嗗叾鍒濆鍖栦负
瀹夸富鏈烘敮鎸佺殑鏈€浣冲悜閲忛暱搴﹂泦鍚堛€?

鐢ㄦ埛绌洪棿闅忓悗鍙互鏍规嵁闇€瑕佷慨鏀瑰畠锛岀洿鍒?vcpu 鐨?SVE 閰嶇疆閫氳繃
KVM_ARM_VCPU_FINALIZE(KVM_ARM_VCPU_SVE) 瀹氱涓烘銆?

闄や簡绠€鍗曞湴浠庡涓绘満闆嗗悎涓Щ闄ゆ墍鏈夎秴杩囨煇涓€肩殑鍚戦噺闀垮害涔嬪锛屽浠绘剰閫夊畾鍚戦噺闀垮害闆嗗悎鐨勬敮鎸?
渚濊禆浜庣‖浠讹紝鍙兘涓嶅彲鐢ㄣ€傚皾璇曢€氳繃 KVM_SET_ONE_REG 閰嶇疆鏃犳晥鐨勫悜閲忛暱搴﹂泦鍚堜細浠?EINVAL 澶辫触銆?

鍦?vcpu 鐨?SVE 閰嶇疆瀹氱涔嬪悗锛岃繘涓€姝ュ啓鍏ヨ瀵勫瓨鍣ㄧ殑灏濊瘯浼氫互 EPERM 澶辫触銆?

```

  0x6030 0000 0016 <regno:16>

```
浣嶅浘鐗规€у浐浠跺瘎瀛樺櫒鏆撮湶浜嗗彲渚涚敤鎴风┖闂撮厤缃殑 hypercall 鏈嶅姟銆傜疆浣嶇殑浣嶅搴斾簬鍙緵瀹㈡埛鏈鸿闂殑
鏈嶅姟銆傞粯璁ゆ儏鍐典笅锛孠VM 鍦?VM 鍒濆鍖栨湡闂磋缃墍鏈夊彈鏀寔鐨勪綅銆傜敤鎴风┖闂村彲浠ラ€氳繃 KVM_GET_ONE_REG
鍙戠幇鍙敤鐨勬湇鍔★紝骞堕€氳繃 KVM_SET_ONE_REG 鍐欏洖瀹冨笇鏈涘鎴锋満鐪嬪埌鐨勩€佸搴斾簬鐩稿簲鐗规€х殑浣嶅浘銆?

娉ㄦ剰锛氫竴鏃?VM 鐨勪换浣?vCPU 鑷冲皯杩愯杩囦竴娆★紝杩欎簺瀵勫瓨鍣ㄥ氨鍙樹负涓嶅彲鍙樼殑銆傚湪杩欑鎯呭喌涓嬶紝
KVM_SET_ONE_REG 浼氬悜鐢ㄦ埛绌洪棿杩斿洖 -EBUSY銆?

锛堟洿澶氱粏鑺傝鍙傝 Documentation/virt/kvm/arm/hypercalls.rst銆傦級


MIPS 瀵勫瓨鍣ㄦ槧灏勪娇鐢ㄤ綆 32 浣嶃€傚叾涓殑楂?16 浣嶆槸瀵勫瓨鍣ㄧ粍绫诲瀷锛?

```

  0x7030 0000 0000 <reg:16>

```
MIPS CP0 瀵勫瓨鍣紙瑙佷笂鏂?KVM_REG_MIPS_CP0_*锛夊叿鏈変互涓?id 浣?
```

  0x7020 0000 0001 00 <reg:5> <sel:3>   (32-bit)
  0x7030 0000 0001 00 <reg:5> <sel:3>   (64-bit)

```
娉ㄦ剰锛欿VM_REG_MIPS_CP0_ENTRYLO0 鍜?KVM_REG_MIPS_CP0_ENTRYLO1 鏄?EntryLo 瀵勫瓨鍣ㄧ殑 MIPS64 鐗堟湰锛?
鏃犺瀹夸富鏈虹‖浠躲€佸涓绘満鍐呮牳銆佸鎴锋満鐨勫瓧闀垮浣曪紝涔熸棤璁哄鎴锋満涓槸鍚﹀瓨鍦?XPA锛屽嵆 RI 鍜?XI 浣?
锛堝鏋滃瓨鍦級鍒嗗埆浣嶄簬浣?63 鍜屼綅 62锛孭FNX 瀛楁浠庝綅 30 寮€濮嬨€?

MIPS MAAR锛堣涓婃枃 KVM_REG_MIPS_CP0_MAAR(*)锛夊叿鏈変互涓?id 浣?
```

  0x7030 0000 0001 01 <reg:8>

```
```

  0x7030 0000 0002 <reg:16>

```
MIPS FPU 瀵勫瓨鍣紙瑙佷笂鏂?KVM_REG_MIPS_FPR_{32,64}()锛夋牴鎹墍璁块棶瀵勫瓨鍣ㄧ殑澶у皬鍏锋湁涓嶅悓鐨?id 浣嶆ā寮忋€?
瀹冧滑濮嬬粓渚濇嵁褰撳墠瀹㈡埛鏈?FPU 妯″紡锛圫tatus.FR 鍜?Config5.FRE锛夎繘琛岃闂紝鍗冲鎴锋満鎵€瑙佺殑鏂瑰紡锛?
濡傛灉瀹㈡埛鏈?FPU 妯″紡鍙戠敓鏀瑰彉锛屽畠浠細鍙樺緱涓嶅彲棰勬祴銆侻IPS SIMD 鏋舵瀯锛圡SA锛夊悜閲忓瘎瀛樺櫒
锛堣涓婃枃 KVM_REG_MIPS_VEC_128()锛夊叿鏈夌被浼肩殑妯″紡锛屽洜涓哄畠浠?
```

  0x7020 0000 0003 00 <0:3> <reg:5> (32-bit FPU registers)
  0x7030 0000 0003 00 <0:3> <reg:5> (64-bit FPU registers)
  0x7040 0000 0003 00 <0:3> <reg:5> (128-bit MSA vector registers)

```
MIPS FPU 鎺у埗瀵勫瓨鍣紙瑙佷笂鏂?KVM_REG_MIPS_FCR_{IR,CSR}锛夊叿鏈?
```

  0x7020 0000 0003 01 <0:3> <reg:5>

```
MIPS MSA 鎺у埗瀵勫瓨鍣紙瑙佷笂鏂?KVM_REG_MIPS_MSA_{IR,CSR}锛夊叿鏈?
```

  0x7020 0000 0003 02 <0:3> <reg:5>

```
RISC-V 瀵勫瓨鍣ㄦ槧灏勪娇鐢ㄤ綆 32 浣嶃€傚叾涓殑楂?8 浣嶆槸瀵勫瓨鍣ㄧ粍绫诲瀷銆?

RISC-V 閰嶇疆瀵勫瓨鍣ㄧ敤浜庨厤缃鎴锋満 VCPU锛屽畠鍏锋湁
```

  0x8020 0000 01 <index into the kvm_riscv_config struct:24> (32bit Host)
  0x8030 0000 01 <index into the kvm_riscv_config struct:24> (64bit Host)

```
浠ヤ笅鏄?RISC-V 閰嶇疆瀵勫瓨鍣細

======================= ========= =============================================
    Encoding            Register  Description
======================= ========= =============================================
  0x80x0 0000 0100 0000 isa       ISA feature bitmap of Guest VCPU
======================= ========= =============================================

isa 閰嶇疆瀵勫瓨鍣ㄥ彲浠ラ殢鏃惰鍙栵紝浣嗗彧鑳藉湪瀹㈡埛鏈?VCPU 杩愯涔嬪墠鍐欏叆銆傞粯璁ゆ儏鍐典笅锛屽畠鍏锋湁涓庡簳灞傚涓绘満
鍖归厤鐨?ISA 鐗规€т綅銆?

RISC-V 鏍稿績瀵勫瓨鍣ㄨ〃绀哄鎴锋満 VCPU 鐨勪竴鑸墽琛岀姸鎬?
```

  0x8020 0000 02 <index into the kvm_riscv_core struct:24> (32bit Host)
  0x8030 0000 02 <index into the kvm_riscv_core struct:24> (64bit Host)

```
浠ヤ笅鏄?RISC-V 鏍稿績瀵勫瓨鍣細

======================= ========= =============================================
    Encoding            Register  Description
======================= ========= =============================================
  0x80x0 0000 0200 0000 regs.pc   Program counter
  0x80x0 0000 0200 0001 regs.ra   Return address
  0x80x0 0000 0200 0002 regs.sp   Stack pointer
  0x80x0 0000 0200 0003 regs.gp   Global pointer
  0x80x0 0000 0200 0004 regs.tp   Task pointer
  0x80x0 0000 0200 0005 regs.t0   Caller saved register 0
  0x80x0 0000 0200 0006 regs.t1   Caller saved register 1
  0x80x0 0000 0200 0007 regs.t2   Caller saved register 2
  0x80x0 0000 0200 0008 regs.s0   Callee saved register 0
  0x80x0 0000 0200 0009 regs.s1   Callee saved register 1
  0x80x0 0000 0200 000a regs.a0   Function argument (or return value) 0
  0x80x0 0000 0200 000b regs.a1   Function argument (or return value) 1
  0x80x0 0000 0200 000c regs.a2   Function argument 2
  0x80x0 0000 0200 000d regs.a3   Function argument 3
  0x80x0 0000 0200 000e regs.a4   Function argument 4
  0x80x0 0000 0200 000f regs.a5   Function argument 5
  0x80x0 0000 0200 0010 regs.a6   Function argument 6
  0x80x0 0000 0200 0011 regs.a7   Function argument 7
  0x80x0 0000 0200 0012 regs.s2   Callee saved register 2
  0x80x0 0000 0200 0013 regs.s3   Callee saved register 3
  0x80x0 0000 0200 0014 regs.s4   Callee saved register 4
  0x80x0 0000 0200 0015 regs.s5   Callee saved register 5
  0x80x0 0000 0200 0016 regs.s6   Callee saved register 6
  0x80x0 0000 0200 0017 regs.s7   Callee saved register 7
  0x80x0 0000 0200 0018 regs.s8   Callee saved register 8
  0x80x0 0000 0200 0019 regs.s9   Callee saved register 9
  0x80x0 0000 0200 001a regs.s10  Callee saved register 10
  0x80x0 0000 0200 001b regs.s11  Callee saved register 11
  0x80x0 0000 0200 001c regs.t3   Caller saved register 3
  0x80x0 0000 0200 001d regs.t4   Caller saved register 4
  0x80x0 0000 0200 001e regs.t5   Caller saved register 5
  0x80x0 0000 0200 001f regs.t6   Caller saved register 6
  0x80x0 0000 0200 0020 mode      Privilege mode (1 = S-mode or 0 = U-mode)
======================= ========= =============================================

RISC-V csr 瀵勫瓨鍣ㄨ〃绀虹洃鐫ｈ€呮ā寮忕殑鎺у埗/鐘舵€佸瘎瀛樺櫒
```

  0x8020 0000 03 <index into the kvm_riscv_csr struct:24> (32bit Host)
  0x8030 0000 03 <index into the kvm_riscv_csr struct:24> (64bit Host)

```
浠ヤ笅鏄?RISC-V csr 瀵勫瓨鍣細

======================= ========= =============================================
    Encoding            Register  Description
======================= ========= =============================================
  0x80x0 0000 0300 0000 sstatus   Supervisor status
  0x80x0 0000 0300 0001 sie       Supervisor interrupt enable
  0x80x0 0000 0300 0002 stvec     Supervisor trap vector base
  0x80x0 0000 0300 0003 sscratch  Supervisor scratch register
  0x80x0 0000 0300 0004 sepc      Supervisor exception program counter
  0x80x0 0000 0300 0005 scause    Supervisor trap cause
  0x80x0 0000 0300 0006 stval     Supervisor bad address or instruction
  0x80x0 0000 0300 0007 sip       Supervisor interrupt pending
  0x80x0 0000 0300 0008 satp      Supervisor address translation and protection
======================= ========= =============================================

RISC-V 瀹氭椂鍣ㄥ瘎瀛樺櫒琛ㄧず瀹㈡埛鏈?VCPU 鐨勫畾鏃跺櫒鐘舵€侊紝瀹冨叿鏈?
```

  0x8030 0000 04 <index into the kvm_riscv_timer struct:24>

```
浠ヤ笅鏄?RISC-V 瀹氭椂鍣ㄥ瘎瀛樺櫒锛?

======================= ========= =============================================
    Encoding            Register  Description
======================= ========= =============================================
  0x8030 0000 0400 0000 frequency Time base frequency (read-only)
  0x8030 0000 0400 0001 time      Time value visible to Guest
  0x8030 0000 0400 0002 compare   Time compare programmed by Guest
  0x8030 0000 0400 0003 state     Time compare state (1 = ON or 0 = OFF)
======================= ========= =============================================

RISC-V F-extension 瀵勫瓨鍣ㄨ〃绀哄崟绮惧害娴偣
```

  0x8020 0000 05 <index into the __riscv_f_ext_state struct:24>

```
浠ヤ笅鏄?RISC-V F-extension 瀵勫瓨鍣細

======================= ========= =============================================
    Encoding            Register  Description
======================= ========= =============================================
  0x8020 0000 0500 0000 f[^0^]      Floating point register 0
  ...
  0x8020 0000 0500 001f f[^31^]     Floating point register 31
  0x8020 0000 0500 0020 fcsr      Floating point control and status register
======================= ========= =============================================

RISC-V D-extension 瀵勫瓨鍣ㄨ〃绀哄弻绮惧害娴偣
```

  0x8020 0000 06 <index into the __riscv_d_ext_state struct:24> (fcsr)
  0x8030 0000 06 <index into the __riscv_d_ext_state struct:24> (non-fcsr)

```
浠ヤ笅鏄?RISC-V D-extension 瀵勫瓨鍣細

======================= ========= =============================================
    Encoding            Register  Description
======================= ========= =============================================
  0x8030 0000 0600 0000 f[^0^]      Floating point register 0
  ...
  0x8030 0000 0600 001f f[^31^]     Floating point register 31
  0x8020 0000 0600 0020 fcsr      Floating point control and status register
======================= ========= =============================================

LoongArch 瀵勫瓨鍣ㄦ槧灏勪娇鐢ㄤ綆 32 浣嶃€傚叾涓殑楂?16 浣嶆槸瀵勫瓨鍣ㄧ粍绫诲瀷銆?

LoongArch csr 瀵勫瓨鍣ㄧ敤浜庢帶鍒跺鎴锋満 cpu 鎴栬幏鍙栧鎴锋満鐘舵€?
```

  0x9030 0000 0001 00 <reg:5> <sel:3>   (64-bit)

```
LoongArch KVM 鎺у埗瀵勫瓨鍣ㄧ敤浜庡疄鐜颁竴浜涙柊瀹氫箟鐨勫姛鑳?
```

  0x9030 0000 0002 <reg:16>

```
```

  0x2030 0002 <msr number:32>

```
浠ヤ笅鏄?x86 鐨?KVM 瀹氫箟瀵勫瓨鍣細

======================= ========= =============================================
    Encoding            Register  Description
======================= ========= =============================================
  0x2030 0003 0000 0000 SSP       Shadow Stack Pointer
======================= ========= =============================================

### 4.69 KVM_GET_ONE_REG


:Capability: KVM_CAP_ONE_REG
:Architectures: all
:Type: vcpu ioctl
:Parameters: struct kvm_one_reg (in and out)
:Returns: 0 on success, negative value on failure

閿欒鐮佸寘鎷細

  ======== ============================================================
  ENOENT   娌℃湁璇ュ瘎瀛樺櫒
  EINVAL   鏃犳晥鐨勫瘎瀛樺櫒 ID锛屾垨娌℃湁璇ュ瘎瀛樺櫒锛屾垨涓?s390 涓婂彈淇濇姢铏氭嫙鍖?
           妯″紡涓嬬殑 VM 涓€璧蜂娇鐢?
  EPERM    (arm64) 鍦?vcpu 瀹氱锛坒inalization锛変箣鍓嶄笉鍏佽璁块棶璇ュ瘎瀛樺櫒
  ======== ============================================================

锛堣繖浜涢敊璇爜浠呬緵鍙傝€冿細涓嶈渚濊禆鍦ㄧ壒瀹氭儏鍐典笅杩斿洖鐗瑰畾鐨勯敊璇爜銆傦級

璇?ioctl 鍏佽鎺ユ敹 vcpu 涓疄鐜扮殑鍗曚釜瀵勫瓨鍣ㄧ殑鍊笺€傝璇诲彇鐨勫瘎瀛樺櫒鐢变紶鍏ョ殑 kvm_one_reg 缁撴瀯浣撶殑
"id" 瀛楁鎸囩ず銆傛垚鍔熸椂锛屽瘎瀛樺櫒鍊煎彲浠ュ湪 "addr" 鎸囧悜鐨勫唴瀛樹綅缃壘鍒般€?

浣跨敤璇ユ帴鍙ｅ彲璁块棶鐨勫瘎瀛樺櫒鍒楄〃涓?4.68 涓殑鍒楄〃鐩稿悓銆?


### 4.70 KVM_KVMCLOCK_CTRL


:Capability: KVM_CAP_KVMCLOCK_CTRL
:Architectures: Any that implement pvclocks (currently x86 only)
:Type: vcpu ioctl
:Parameters: None
:Returns: 0 on success, -1 on error

璇?ioctl 璁剧疆涓€涓瀹㈡埛鏈哄彲璁块棶鐨勬爣蹇楋紝鎸囩ず鎸囧畾鐨?vCPU 宸茶瀹夸富鏈虹敤鎴风┖闂存殏鍋溿€?

瀹夸富鏈哄皢鍦?pvclock 缁撴瀯浣撲腑璁剧疆涓€涓爣蹇楋紝璇ユ爣蹇楃敱 soft lockup 鐪嬮棬鐙楁鏌ャ€傝鏍囧織鏄鎴锋満涓?
瀹夸富鏈轰箣闂村叡浜殑 pvclock 缁撴瀯浣撶殑涓€閮ㄥ垎锛屽叿浣撴槸 pvclock_vcpu_time_info 缁撴瀯浣撶殑 flags 瀛楁鐨?
绗簩浣嶃€傚畠鐢卞涓绘満鐙崰璁剧疆锛岀敱瀹㈡埛鏈虹嫭鍗犺鍙?娓呴櫎銆傚鎴锋満妫€鏌ュ拰娓呴櫎璇ユ爣蹇楃殑鎿嶄綔蹇呴』鏄師瀛?
鎿嶄綔锛屽洜姝ゅ繀椤讳娇鐢?load-link/store-conditional 鎴栫瓑浠锋寚浠ゃ€傚鎴锋満鍦ㄤ袱绉嶆儏鍐典笅浼氭竻闄よ鏍囧織锛?
褰?soft lockup 鐪嬮棬鐙楀畾鏃跺櫒閲嶇疆鑷韩鏃讹紝鎴栧綋妫€娴嬪埌 soft lockup 鏃躲€傝 ioctl 鍙互鍦ㄦ殏鍋?vcpu 涔嬪悗銆?
浣嗗湪鍏舵仮澶嶄箣鍓嶇殑浠讳綍鏃堕棿璋冪敤銆?


### 4.71 KVM_SIGNAL_MSI


:Capability: KVM_CAP_SIGNAL_MSI
:Architectures: x86 arm64
:Type: vm ioctl
:Parameters: struct kvm_msi (in)
:Returns: >0 on delivery, 0 if guest blocked the MSI, and -1 on error

鐩存帴娉ㄥ叆涓€鏉?MSI 娑堟伅銆備粎鍦ㄨ兘澶勭悊 MSI 娑堟伅鐨勫唴鏍告€?irqchip 涓嬫湁鏁堛€?

```

  struct kvm_msi {
	__u32 address_lo;
	__u32 address_hi;
	__u32 data;
	__u32 flags;
	__u32 devid;
	__u8  pad[12];
  };

```
flags:
  KVM_MSI_VALID_DEVID锛歞evid 鍖呭惈涓€涓湁鏁堝€笺€傛瘡 VM 鐨?KVM_CAP_MSI_DEVID 鑳藉姏鐢ㄤ簬閫氬憡闇€瑕佹彁渚?
  璁惧 ID 鐨勮姹傘€傚鏋滆鑳藉姏涓嶅彲鐢紝鐢ㄦ埛绌洪棿缁濅笉搴旇缃?KVM_MSI_VALID_DEVID 鏍囧織锛屽惁鍒?ioctl
  鍙兘浼氬け璐ャ€?

濡傛灉璁剧疆浜?KVM_MSI_VALID_DEVID锛屽垯 devid 鍖呭惈鍐欏叆 MSI 娑堟伅鐨勮澶囩殑鍞竴璁惧鏍囪瘑绗︺€傚浜?PCI锛?
杩欓€氬父鏄綆 16 浣嶄腑鐨?BDF 鏍囪瘑绗︺€?

鍦?x86 涓婏紝闄ら潪鍚敤浜?KVM_CAP_X2APIC_API 鑳藉姏鐨?KVM_X2APIC_API_USE_32BIT_IDS 鐗规€э紝鍚﹀垯 address_hi
浼氳蹇界暐銆傚鏋滃惎鐢紝address_hi 鐨?31-8 浣嶆彁渚涚洰鐨?id 鐨?31-8 浣嶃€俛ddress_hi 鐨?7-0 浣嶅繀椤讳负闆躲€?


### 4.71 KVM_CREATE_PIT2


:Capability: KVM_CAP_PIT2
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_pit_config (in)
:Returns: 0 on success, -1 on error

涓?i8254 PIT 鍒涘缓涓€涓唴鏍告€佽澶囨ā鍨嬨€傝璋冪敤浠呭湪閫氳繃 KVM_CREATE_IRQCHIP 鍚敤鍐呮牳鎬?irqchip
鏀寔涔嬪悗鎵嶆湁鏁堛€備互涓?
```

  struct kvm_pit_config {
	__u32 flags;
	__u32 pad[15];
  };

```
```

  #define KVM_PIT_SPEAKER_DUMMY     1 /* emulate speaker port stub */

```
PIT 瀹氭椂鍣ㄤ腑鏂彲浠ヤ娇鐢ㄤ竴涓瘡 VM 鐨勫唴鏍哥嚎绋嬫潵娉ㄥ叆銆傚鏋滃畠
```

  kvm-pit/<owner-process-pid>

```
鍦ㄨ繍琛屽叿鏈夐珮浼樺厛绾х殑瀹㈡埛鏈烘椂锛屽彲鑳介渶瑕佺浉搴斿湴璋冩暣璇ョ嚎绋嬬殑璋冨害鍙傛暟銆?

姝?IOCTL 鍙栦唬浜嗗凡杩囨椂鐨?KVM_CREATE_PIT銆?


### 4.72 KVM_GET_PIT2


:Capability: KVM_CAP_PIT_STATE2
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_pit_state2 (out)
:Returns: 0 on success, -1 on error

鑾峰彇鍐呮牳鎬?PIT 妯″瀷鐨勭姸鎬併€備粎鍦?
```

  struct kvm_pit_state2 {
	struct kvm_pit_channel_state channels[3];
	__u32 flags;
	__u32 reserved[9];
  };

```
```

  /* disable PIT in HPET legacy mode */
  #define KVM_PIT_FLAGS_HPET_LEGACY     0x00000001
  /* speaker port data bit enabled */
  #define KVM_PIT_FLAGS_SPEAKER_DATA_ON 0x00000002

```
姝?IOCTL 鍙栦唬浜嗗凡杩囨椂鐨?KVM_GET_PIT銆?


### 4.73 KVM_SET_PIT2


:Capability: KVM_CAP_PIT_STATE2
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_pit_state2 (in)
:Returns: 0 on success, -1 on error

璁剧疆鍐呮牳鎬?PIT 妯″瀷鐨勭姸鎬併€備粎鍦?KVM_CREATE_PIT2 涔嬪悗鏈夋晥銆傚叧浜?struct kvm_pit_state2 鐨勭粏鑺?
璇峰弬瑙?KVM_GET_PIT2銆?

  `KVM_SET_PIT2` 涓ユ牸閬靛畧 Intel 8254 PIT 鐨勮鑼冦€備緥濡傦紝`struct kvm_pit_channel_state` 涓?
  `count` 鍊间负 0 琚В閲婁负 65536锛屽嵆鏈€澶ц鏁板€笺€傚弬鑰?`Intel 8254 programmable interval
  timer <https://www.scs.stanford.edu/10wi-cs140/pintos/specs/8254.pdf>`_銆?

姝?IOCTL 鍙栦唬浜嗗凡杩囨椂鐨?KVM_SET_PIT銆?


### 4.74 KVM_PPC_GET_SMMU_INFO


:Capability: KVM_CAP_PPC_GET_SMMU_INFO
:Architectures: powerpc
:Type: vm ioctl
:Parameters: None
:Returns: 0 on success, -1 on error

璇?ioctl 濉厖骞惰繑鍥炰竴涓弿杩?KVM 鏀寔鐨?鏈嶅姟鍣?绾?MMU 妯℃嫙鐗规€х殑缁撴瀯浣撱€傜敤鎴风┖闂村弽杩囨潵鍙互鐢ㄥ畠
涓哄鏈烘搷浣滅郴缁熺敓鎴愰€傚綋鐨勮澶囨爲灞炴€с€?

璇ョ粨鏋勪綋鍖呭惈涓€浜涘叏灞€淇℃伅锛屽悗闈㈣窡鐫€涓€涓?
```

      struct kvm_ppc_smmu_info {
	     __u64 flags;
	     __u32 slb_size;
	     __u32 pad;
	     struct kvm_ppc_one_seg_page_size sps[KVM_PPC_PAGE_SIZES_MAX_SZ];
      };

```
鏀寔鐨勬爣蹇楀涓嬶細

    - KVM_PPC_PAGE_SIZES_REAL:
        褰撹缃鏍囧織鏃讹紝瀹㈡埛鏈洪〉澶у皬蹇呴』"閫傞厤"鍚庡瀛樺偍鐨勯〉澶у皬銆傚綋鏈缃椂锛屽垪琛ㄤ腑鐨勪换浣曢〉澶у皬
        閮藉彲浠ヤ娇鐢紝鑰屼笉绠″畠浠浣曠敱鐢ㄦ埛绌洪棿浣滀负鍚庡銆?

    - KVM_PPC_1T_SEGMENTS
        闄や簡鏍囧噯鐨?256M 娈典箣澶栵紝妯℃嫙鐨?MMU 杩樻敮鎸?1T 娈点€?

    - KVM_PPC_NO_HASH
	璇ユ爣蹇楄〃绀?KVM 涓嶆敮鎸?HPT 瀹㈡埛鏈猴紝鍥犳鎵€鏈夊鎴锋満蹇呴』浣跨敤 radix MMU 妯″紡銆?

"slb_size" 瀛楁鎸囩ず鏀寔澶氬皯涓?SLB 鏉＄洰銆?

"sps" 鏁扮粍鍖呭惈 8 涓潯鐩紝鎸夐€掑椤哄簭鎸囩ず娈垫敮鎸佺殑鍩洪〉澶у皬銆傛瘡涓潯鐩畾涔変负
```

   struct kvm_ppc_one_seg_page_size {
	__u32 page_shift;	/* Base page shift of segment (or 0) */
	__u32 slb_enc;		/* SLB encoding for BookS */
	struct kvm_ppc_one_page_size enc[KVM_PPC_PAGE_SIZES_MAX_SZ];
   };

```
"page_shift" 涓?0 鐨勬潯鐩湭琚娇鐢ㄣ€傜敱浜庢暟缁勬寜閫掑椤哄簭缁勭粐锛岄亣鍒版绫绘潯鐩椂鏌ユ壘鍗冲彲鍋滄銆?

"slb_enc" 瀛楁鎻愪緵鍦?SLB 涓敤浜庤椤靛ぇ灏忕殑缂栫爜銆傝繖浜涗綅鐨勪綅缃娇寰楄鍊煎彲浠ョ洿鎺ユ寜浣嶆垨鍒?slbmte
鎸囦护鐨?"vsid" 鍙傛暟涓€?

"enc" 鏁扮粍鏄竴涓垪琛紝閽堝姣忎釜娈靛熀椤靛ぇ灏忔彁渚涘彈鏀寔鐨勫疄闄呴〉澶у皬鍒楄〃锛堝彧鑳藉ぇ浜庢垨绛変簬鍩洪〉澶у皬锛夛紝
浠ュ強鍝堝笇 PTE 涓殑鐩稿簲缂栫爜銆傜被浼煎湴锛岃鏁扮粍鏄?8 涓潯鐩紝鎸夐€掑澶у皬鎺掑簭锛岃€?"0" 鍋忕Щ鐨勬潯鐩?
```

   struct kvm_ppc_one_page_size {
	__u32 page_shift;	/* Page shift (or 0) */
	__u32 pte_enc;		/* Encoding in the HPTE (>>12) */
   };

```
"pte_enc" 瀛楁鎻愪緵涓€涓€硷紝鍙互鎸変綅鎴栧埌鍝堝笇 PTE 鐨?RPN 瀛楁涓紙鍗筹紝闇€瑕佸厛宸︾Щ 12 浣嶆墠鑳芥寜浣嶆垨
鍒板搱甯?PTE 鐨勭浜屼釜鍙屽瓧涓級銆?

### 4.75 KVM_IRQFD


:Capability: KVM_CAP_IRQFD
:Architectures: x86 s390 arm64
:Type: vm ioctl
:Parameters: struct kvm_irqfd (in)
:Returns: 0 on success, -1 on error

鍏佽璁剧疆涓€涓?eventfd 浠ョ洿鎺ヨЕ鍙戜竴娆″鎴锋満涓柇銆俴vm_irqfd.fd 鎸囧畾鐢ㄤ綔 eventfd 鐨勬枃浠舵弿杩扮锛?
kvm_irqfd.gsi 鎸囧畾鐢辨浜嬩欢鍒囨崲鐨?irqchip 寮曡剼銆傚綋 eventfd 涓婅Е鍙戜竴涓簨浠舵椂锛屼細浣跨敤鎸囧畾鐨?gsi
寮曡剼鍚戝鎴锋満娉ㄥ叆涓€涓腑鏂€備娇鐢?KVM_IRQFD_FLAG_DEASSIGN 鏍囧織骞跺悓鏃舵寚瀹?kvm_irqfd.fd 鍜?
kvm_irqfd.gsi锛屽彲浠ョЩ闄よ irqfd銆?

鍊熷姪 KVM_CAP_IRQFD_RESAMPLE锛孠VM_IRQFD 鏀寔鍘绘柇瑷€锛坉e-assert锛夊拰閫氱煡鏈哄埗锛屼粠鑰屽厑璁告ā鎷熷熀浜?
irqfd 鐨勭數骞宠Е鍙戜腑鏂€傚綋璁剧疆 KVM_IRQFD_FLAG_RESAMPLE 鏃讹紝鐢ㄦ埛蹇呴』鍦?kvm_irqfd.resamplefd 瀛楁
涓紶鍏ヤ竴涓澶栫殑 eventfd銆傚湪閲嶉噰鏍锋ā寮忎笅锛岄€氳繃 kvm_irq.fd 鎶曢€掍腑鏂細鏂█ irqchip 涓寚瀹氱殑 gsi銆?
褰?irqchip 琚噸閲囨牱鏃讹紙渚嬪鏉ヨ嚜 EOI锛夛紝gsi 琚幓鏂█锛屽苟閫氳繃 kvm_irqfd.resamplefd 閫氱煡鐢ㄦ埛銆傛槸鍚?
閲嶆柊鎺掗槦璇ヤ腑鏂紝鐢辩敤鎴疯礋璐ｏ紝鍓嶆彁鏄娇鐢ㄥ畠鐨勮澶囦粛闇€瑕佹湇鍔°€傛敞鎰忥紝鍏抽棴 resamplefd 涓嶈冻浠ョ鐢?
璇?irqfd銆侹VM_IRQFD_FLAG_RESAMPLE 浠呭湪鍒嗛厤鏃堕渶瑕侊紝鑰屼笉蹇呬笌 KVM_IRQFD_FLAG_DEASSIGN 涓€璧锋寚瀹氥€?

鍦?arm64 涓婏紝鐢变簬鏀寔 gsi 璺敱锛屽彲鑳藉彂鐢熶互涓嬫儏鍐碉細

- 濡傛灉娌℃湁涓庤 gsi 鍏宠仈鐨勮矾鐢辨潯鐩紝娉ㄥ叆澶辫触
- 濡傛灉璇?gsi 鍏宠仈鍒?irqchip 璺敱鏉＄洰锛宨rqchip.pin + 32 瀵瑰簲浜庤娉ㄥ叆鐨?SPI ID
- 濡傛灉璇?gsi 鍏宠仈鍒?MSI 璺敱鏉＄洰锛孧SI 娑堟伅鍜岃澶?ID 琚浆鎹负涓€涓?LPI锛堟敮鎸佷粎闄愪簬 GICv3 ITS
  鐨勫唴鏍告€佹ā鎷燂級

### 4.76 KVM_PPC_ALLOCATE_HTAB


:Capability: KVM_CAP_PPC_ALLOC_HTAB
:Architectures: powerpc
:Type: vm ioctl
:Parameters: Pointer to u32 containing hash table order (in/out)
:Returns: 0 on success, -1 on error

璇?ioctl 璇锋眰瀹夸富鏈哄唴鏍镐娇鐢?PAPR 鍗婅櫄鎷熷寲鎺ュ彛涓哄鎴锋満鍒嗛厤涓€涓?MMU 鍝堝笇琛ㄣ€傝繖浠呭湪鍐呮牳閰嶇疆涓轰娇鐢?
Book 3S HV 椋庢牸鐨勮櫄鎷熷寲鏃舵墠璧蜂綔鐢ㄣ€傚惁鍒欒鑳藉姏涓嶅瓨鍦紝ioctl 杩斿洖 ENOTTY 閿欒銆傛湰璇存槑鐨勫叾浣欓儴鍒?
鍋囪涓?Book 3S HV銆?

璋冪敤姝?ioctl 鏃朵笉鑳芥湁姝ｅ湪杩愯鐨?vcpu锛涘鏋滄湁锛屽畠灏嗕笉鎵ц浠讳綍鎿嶄綔骞惰繑鍥?EBUSY 閿欒銆?

鍙傛暟鏄竴涓寚鍚?32 浣嶆棤绗﹀彿鏁存暟鍙橀噺鐨勬寚閽堬紝璇ュ彉閲忓寘鍚墍闇€鍝堝笇琛ㄥぇ灏忥紙浠?2 涓哄簳鐨勫鏁帮級鐨勯樁锛坥rder锛夛紝
鍏跺彇鍊艰寖鍥村繀椤诲湪 18 鍒?46 涔嬮棿銆傚湪 ioctl 鎴愬姛杩斿洖鏃讹紝璇ュ€间笉浼氳鍐呮牳鏀瑰彉銆?

濡傛灉褰撲换浣?vcpu 琚姹傝繍琛岋紙閫氳繃 KVM_RUN ioctl锛夋椂灏氭湭鍒嗛厤鍝堝笇琛紝瀹夸富鏈哄唴鏍稿皢鍒嗛厤涓€涓粯璁ゅぇ灏?
鐨勫搱甯岃〃锛?6 MB锛夈€?

濡傛灉鍦ㄥ搱甯岃〃宸插垎閰嶇殑鎯呭喌涓嬭皟鐢ㄦ ioctl锛屼笖闃朵笌鐜版湁鍝堝笇琛ㄤ笉鍚岋紝鍒欎細閲婃斁鐜版湁鍝堝笇琛ㄥ苟鍒嗛厤涓€涓柊鐨勩€?
濡傛灉鍦ㄥ搱甯岃〃宸插垎閰嶄笖闃朵笌鎸囧畾鐩稿悓鏃惰皟鐢ㄦ ioctl锛屽唴鏍稿皢娓呯┖鐜版湁鍝堝笇琛紙灏嗘墍鏈?HPTE 缃浂锛夈€傛棤璁?
鍝鎯呭喌锛屽鏋滃鎴锋満浣跨敤浜嗚櫄鎷熷寲瀹炴ā寮忓尯鍩燂紙VRMA锛夎鏂斤紝鍐呮牳灏嗗湪浠讳綍 vcpu 鐨勪笅涓€娆?KVM_RUN 鏃?
閲嶆柊鍒涘缓 VMRA HPTE銆?

### 4.77 KVM_S390_INTERRUPT


:Capability: basic
:Architectures: s390
:Type: vm ioctl, vcpu ioctl
:Parameters: struct kvm_s390_interrupt (in)
:Returns: 0 on success, -1 on error

鍏佽鍚戝鎴锋満娉ㄥ叆涓€涓腑鏂€傛牴鎹腑鏂被鍨嬶紝涓柇鍙互鏄诞鍔ㄧ殑锛坴m ioctl锛夋垨姣?cpu 鐨勶紙vcpu ioctl锛夈€?

```

  struct kvm_s390_interrupt {
	__u32 type;
	__u32 parm;
	__u64 parm64;
  };

```
type 鍙互鏄互涓嬩箣涓€锛?

KVM_S390_SIGP_STOP (vcpu)
    - sigp 鍋滄锛涘彲閫夋爣蹇楀湪 parm 涓?
KVM_S390_PROGRAM_INT (vcpu)
    - 绋嬪簭妫€鏌ワ紱code 鍦?parm 涓?
KVM_S390_SIGP_SET_PREFIX (vcpu)
    - sigp 璁剧疆鍓嶇紑锛涘墠缂€鍦板潃鍦?parm 涓?
KVM_S390_RESTART (vcpu)
    - 閲嶅惎
KVM_S390_INT_CLOCK_COMP (vcpu)
    - 鏃堕挓姣旇緝鍣ㄤ腑鏂?
KVM_S390_INT_CPU_TIMER (vcpu)
    - CPU 瀹氭椂鍣ㄤ腑鏂?
KVM_S390_INT_VIRTIO (vm)
    - virtio 澶栭儴涓柇锛涘閮ㄤ腑鏂弬鏁板湪 parm 鍜?parm64 涓?
KVM_S390_INT_SERVICE (vm)
    - sclp 澶栭儴涓柇锛泂clp 鍙傛暟鍦?parm 涓?
KVM_S390_INT_EMERGENCY (vcpu)
    - sigp 绱ф€ワ紱婧?cpu 鍦?parm 涓?
KVM_S390_INT_EXTERNAL_CALL (vcpu)
    - sigp 澶栭儴璋冪敤锛涙簮 cpu 鍦?parm 涓?
KVM_S390_INT_IO(ai,cssid,ssid,schid) (vm)
    - 澶嶅悎鍊硷紝鎸囩ず涓€涓?I/O 涓柇锛坅i - 閫傞厤鍣ㄤ腑鏂紱cssid,ssid,schid - 瀛愰€氶亾锛夛紱
      I/O 涓柇鍙傛暟鍦?parm锛堝瓙閫氶亾锛夊拰 parm64锛坕ntparm锛屼腑鏂瓙绫伙級涓?
KVM_S390_MCHK (vm, vcpu)
    - 鏈哄櫒妫€鏌ヤ腑鏂紱cr 14 浣嶅湪 parm 涓紝鏈哄櫒妫€鏌ヤ腑鏂爜鍦?parm64 涓紙娉ㄦ剰锛岄渶瑕侀澶栬礋杞界殑
      鏈哄櫒妫€鏌ヤ笉鍙楁 ioctl 鏀寔锛?

杩欐槸涓€涓紓姝ョ殑 vcpu ioctl锛屽彲浠ヤ粠浠讳綍绾跨▼璋冪敤銆?

### 4.78 KVM_PPC_GET_HTAB_FD


:Capability: KVM_CAP_PPC_HTAB_FD
:Architectures: powerpc
:Type: vm ioctl
:Parameters: Pointer to struct kvm_get_htab_fd (in)
:Returns: file descriptor number (>= 0) on success, -1 on error

璇?ioctl 杩斿洖涓€涓枃浠舵弿杩扮锛屽彲鐢ㄤ簬璇诲嚭瀹㈡埛鏈哄搱甯岄〉琛紙HPT锛変腑鐨勬潯鐩紝鎴栧啓鍏ユ潯鐩互鍒濆鍖?HPT銆?
浠呭綋鍙傛暟鐨?flags 瀛楁涓缃簡 KVM_GET_HTAB_WRITE 浣嶆椂锛岃繑鍥炵殑 fd 鎵嶅彲鍐欙紱浠呭綋璇ヤ綅娓呴浂鏃讹紝鎵嶅彲
璇汇€傚弬鏁扮粨鏋勪綋濡備笅
```

  /* For KVM_PPC_GET_HTAB_FD */
  struct kvm_get_htab_fd {
	__u64	flags;
	__u64	start_index;
	__u64	reserved[2];
  };

  /* Values for kvm_get_htab_fd.flags */
  #define KVM_GET_HTAB_BOLTED_ONLY	((__u64)0x1)
  #define KVM_GET_HTAB_WRITE		((__u64)0x2)

```
'start_index' 瀛楁缁欏嚭 HPT 涓紑濮嬭鍙栫殑鏉＄洰鐨勭储寮曘€傚啓鍏ユ椂蹇界暐璇ュ瓧娈点€?

瀵?fd 鐨勮鍙栨渶鍒濅細鎻愪緵鎵€鏈?鏈夎叮"鐨?HPT 鏉＄洰鐨勪俊鎭€傚鏋滆缃簡 KVM_GET_HTAB_BOLTED_ONLY 浣嶏紝
鏈夎叮鐨勬潯鐩槸閭ｄ簺缃綅浜?bolted 浣嶇殑鏉＄洰锛涘惁鍒欐槸鎵€鏈夋潯鐩€傚埌杈?HPT 鏈熬鏃讹紝read() 浼氳繑鍥炪€傚鏋?
鍐嶆瀵?fd 璋冪敤 read()锛屽畠浼氫粠 HPT 寮€澶撮噸鏂板紑濮嬶紝浣嗗彧杩斿洖鑷笂娆¤鍙栦互鏉ュ彂鐢熷彉鍖栫殑 HPT 鏉＄洰銆?

璇诲彇鎴栧啓鍏ョ殑鏁版嵁缁撴瀯涓轰竴涓ご閮紙8 瀛楄妭锛夛紝鍚庤窡涓€绯诲垪鏈夋晥鐨?HPT 鏉＄洰锛堟瘡鏉?16 瀛楄妭锛夈€傚ご閮ㄦ寚绀?
鏈夊灏戜釜鏈夋晥 HPT 鏉＄洰锛屼互鍙婃湁鏁堟潯鐩箣鍚庤窡闅忓灏戜釜鏃犳晥鏉＄洰銆傛棤鏁堟潯鐩笉琚樉寮忚〃绀?
```

  struct kvm_get_htab_header {
	__u32	index;
	__u16	n_valid;
	__u16	n_invalid;
  };

```
瀵?fd 鐨勫啓鍏ヤ粠澶撮儴涓粰鍑虹殑绱㈠紩澶勫垱寤?HPT 鏉＄洰锛涘厛鏄?'n_valid' 涓潵鑷啓鍏ユ暟鎹殑鏈夋晥鏉＄洰锛岀劧鍚庢槸
'n_invalid' 涓棤鏁堟潯鐩紝浣挎壘鍒扮殑浠讳綍鍏堝墠鏈夋晥鏉＄洰澶辨晥銆?

### 4.79 KVM_CREATE_DEVICE


:Capability: KVM_CAP_DEVICE_CTRL
:Architectures: all
:Type: vm ioctl
:Parameters: struct kvm_create_device (in/out)
:Returns: 0 on success, -1 on error

閿欒鐮侊細

  ======  =======================================================
  ENODEV  璁惧绫诲瀷鏈煡鎴栦笉琚敮鎸?
  EEXIST  璁惧宸插垱寤猴紝涓旇绫诲瀷鐨勮澶囧彲鑳戒笉浼氬疄渚嬪寲澶氭
  ======  =======================================================

  鍏朵粬閿欒鏉′欢鍙兘鐢卞悇涓澶囩被鍨嬪畾涔夛紝鎴栧叿鏈夊叾鏍囧噯鍚箟銆?

鍦ㄥ唴鏍镐腑鍒涘缓涓€涓ā鎷熻澶囥€傚湪 fd 涓繑鍥炵殑鏂囦欢鎻忚堪绗﹀彲鐢ㄤ簬 KVM_SET/GET/HAS_DEVICE_ATTR銆?

濡傛灉璁剧疆浜?KVM_CREATE_DEVICE_TEST 鏍囧織锛屽垯鍙祴璇曡澶囩被鍨嬫槸鍚﹀彈鏀寔锛堜笉涓€瀹氭槸瀹冭兘鍚﹀湪褰撳墠 vm 涓?
鍒涘缓锛夈€?

鍚勪釜璁惧涓嶅簲瀹氫箟鏍囧織銆傚睘鎬у簲鐢ㄤ簬鎸囧畾浠讳綍涓嶈璁惧绫诲瀷缂栧彿鎵€鏆楃ず鐨勮涓恒€?

```

  struct kvm_create_device {
	__u32	type;	/* in: KVM_DEV_TYPE_xxx */
	__u32	fd;	/* out: device handle */
	__u32	flags;	/* in: KVM_CREATE_DEVICE_xxx */
  };

```
### 4.80 KVM_SET_DEVICE_ATTR/KVM_GET_DEVICE_ATTR


:Capability: KVM_CAP_DEVICE_CTRL, KVM_CAP_VM_ATTRIBUTES for vm device,
             KVM_CAP_VCPU_ATTRIBUTES for vcpu device
             KVM_CAP_SYS_ATTRIBUTES for system (/dev/kvm) device (no set)
:Architectures: x86, arm64, s390
:Type: device ioctl, vm ioctl, vcpu ioctl
:Parameters: struct kvm_device_attr
:Returns: 0 on success, -1 on error

閿欒鐮侊細

  =====   =============================================================
  ENXIO   璇ョ粍鎴栧睘鎬у姝ゅ璁炬湭鐭?涓嶅彈鏀寔锛屾垨缂哄皯纭欢鏀寔銆?
  EPERM   璇ュ睘鎬э紙褰撳墠锛変笉鑳戒互杩欑鏂瑰紡璁块棶
          锛堜緥濡傚彧璇诲睘鎬э紝鎴栦粎鍦ㄨ澶囧浜庝笉鍚岀姸鎬佹椂鎵嶆湁鎰忎箟鐨勫睘鎬э級
  =====   =============================================================

  鍏朵粬閿欒鏉′欢鍙兘鐢卞悇涓澶囩被鍨嬪畾涔夈€?

鑾峰彇/璁剧疆鎸囧畾鐨勮澶囬厤缃拰/鎴栫姸鎬佺墖娈点€傚叾璇箟鏄澶囩浉鍏崇殑銆傝鍙傝 "devices" 鐩綍涓殑鍚勪釜
璁惧鏂囨。銆備笌 ONE_REG 涓€鏍凤紝浼犺緭鏁版嵁鐨勫ぇ灏忕敱鐗瑰畾灞炴€у畾涔夈€?

```

  struct kvm_device_attr {
	__u32	flags;		/* no flags currently defined */
	__u32	group;		/* device-defined */
	__u64	attr;		/* group-defined */
	__u64	addr;		/* userspace address of attr data */
  };

```
### 4.81 KVM_HAS_DEVICE_ATTR


:Capability: KVM_CAP_DEVICE_CTRL, KVM_CAP_VM_ATTRIBUTES for vm device,
             KVM_CAP_VCPU_ATTRIBUTES for vcpu device
             KVM_CAP_SYS_ATTRIBUTES for system (/dev/kvm) device
:Type: device ioctl, vm ioctl, vcpu ioctl
:Parameters: struct kvm_device_attr
:Returns: 0 on success, -1 on error

閿欒鐮侊細

  =====   =============================================================
  ENXIO   璇ョ粍鎴栧睘鎬у姝ゅ璁炬湭鐭?涓嶅彈鏀寔锛屾垨缂哄皯纭欢鏀寔銆?
  =====   =============================================================

娴嬭瘯涓€涓澶囨槸鍚︽敮鎸佺壒瀹氬睘鎬с€傛垚鍔熻繑鍥炶〃绀哄凡瀹炵幇璇ュ睘鎬с€傚畠骞朵笉涓€瀹氳〃绀鸿灞炴€у彲浠ュ湪璁惧
褰撳墠鐘舵€佷笅琚鍙栨垨鍐欏叆銆?addr" 琚拷鐣ャ€?


### 4.82 KVM_ARM_VCPU_INIT


:Capability: basic
:Architectures: arm64
:Type: vcpu ioctl
:Parameters: struct kvm_vcpu_init (in)
:Returns: 0 on success; -1 on error

閿欒鐮侊細

  ======     =================================================================
  EINVAL    鐩爣鏈煡锛屾垨鐗规€х粍鍚堟棤鏁堛€?
  ENOENT    鎸囧畾鐨勬煇涓壒鎬т綅鏈煡銆?
  ======     =================================================================

璇?ioctl 鍛婅瘔 KVM 瑕佸悜瀹㈡埛鏈哄憟鐜颁粈涔堢被鍨嬬殑 CPU锛屼互鍙婂畠搴斿叿鏈夊摢浜涘彲閫夌壒鎬с€傝繖灏嗕娇 cpu 瀵勫瓨鍣?
閲嶇疆涓哄畠浠殑鍒濆鍊笺€傚鏋滄湭璋冪敤瀹冿紝KVM_RUN 灏嗗璇?vcpu 杩斿洖 ENOEXEC銆?

鍒濆鍊煎畾涔変负锛?
 - 澶勭悊鍣ㄧ姸鎬侊細
  - AArch64锛欵L1h锛孌銆丄銆両 鍜?F 浣嶇疆浣嶃€傛墍鏈夊叾浠栦綅娓呴浂銆?
  - AArch32锛歋VC锛孉銆両 鍜?F 浣嶇疆浣嶃€傛墍鏈夊叾浠栦綅娓呴浂銆?
 - 閫氱敤瀵勫瓨鍣紝鍖呮嫭 PC 鍜?SP锛氱疆涓?0
 - FPSIMD/NEON 瀵勫瓨鍣細缃负 0
 - SVE 瀵勫瓨鍣細缃负 0
 - 绯荤粺瀵勫瓨鍣細閲嶇疆涓烘灦鏋勫畾涔夌殑鍒濆鍊硷紝鍗抽拡瀵?EL1锛堟垨 SVC锛夋垨 EL2锛堝湪鍚敤 EL2 鐨勬儏鍐典笅锛?
   鐨勭儹澶嶄綅鍊笺€?

娉ㄦ剰锛岀敱浜庢煇浜涘瘎瀛樺櫒鍙嶆槧鏈哄櫒鎷撴墤锛屾墍鏈?vcpu 閮藉簲鍦ㄦ ioctl 璋冪敤涔嬪墠鍒涘缓銆?

鐢ㄦ埛绌洪棿鍙互瀵圭粰瀹氱殑 vcpu 澶氭璋冪敤姝ゅ嚱鏁帮紝鍖呮嫭鍦?vcpu 杩愯涔嬪悗銆傝繖灏嗘妸 vcpu 閲嶇疆涓哄叾鍒濆鐘舵€併€?
鍒濆璋冪敤涔嬪悗鐨勬墍鏈夎皟鐢ㄥ繀椤讳娇鐢ㄧ浉鍚岀殑鐩爣浠ュ強鐩稿悓鐨勭壒鎬ф爣蹇楅泦鍚堬紝鍚﹀垯灏嗚繑鍥?EINVAL銆?

鍙兘鐨勭壒鎬э細

 - KVM_ARM_VCPU_POWER_OFF锛氫互鏂數鐘舵€佸惎鍔?CPU銆?
	  渚濊禆浜?KVM_CAP_ARM_PSCI銆傚鏋滄湭璁剧疆锛屽垯鍦ㄨ皟鐢?KVM_RUN 鏃?CPU 灏嗕笂鐢靛苟
	  鎵ц瀹㈡埛鏈轰唬鐮併€?
 - KVM_ARM_VCPU_EL1_32BIT锛氫互 32 浣嶆ā寮忓惎鍔?CPU銆?
	  渚濊禆浜?KVM_CAP_ARM_EL1_32BIT锛堜粎 arm64锛夈€?
 - KVM_ARM_VCPU_PSCI_0_2锛氫负璇?CPU 妯℃嫙 PSCI v0.2锛堟垨涓?v0.2 鍚戝悗鍏煎鐨勬湭鏉ヤ慨璁㈢増锛夈€?
	  渚濊禆浜?KVM_CAP_ARM_PSCI_0_2銆?
 - KVM_ARM_VCPU_PMU_V3锛氫负璇?CPU 妯℃嫙 PMUv3銆?
	  渚濊禆浜?KVM_CAP_ARM_PMU_V3銆?

 - KVM_ARM_VCPU_PTRAUTH_ADDRESS锛氬惎鐢ㄥ湴鍧€鎸囬拡璁よ瘉锛屼粎閫傜敤浜?arm64銆?
	  渚濊禆浜?KVM_CAP_ARM_PTRAUTH_ADDRESS銆?
	  濡傛灉 KVM_CAP_ARM_PTRAUTH_ADDRESS 鍜?KVM_CAP_ARM_PTRAUTH_GENERIC 閮藉瓨鍦紝
	  鍒欏繀椤诲悓鏃惰姹?KVM_ARM_VCPU_PTRAUTH_ADDRESS 鍜?KVM_ARM_VCPU_PTRAUTH_GENERIC锛?
	  鎴栬€呬袱鑰呴兘涓嶈姹傘€?

 - KVM_ARM_VCPU_PTRAUTH_GENERIC锛氬惎鐢ㄩ€氱敤鎸囬拡璁よ瘉锛屼粎閫傜敤浜?arm64銆?
	  渚濊禆浜?KVM_CAP_ARM_PTRAUTH_GENERIC銆?
	  濡傛灉 KVM_CAP_ARM_PTRAUTH_ADDRESS 鍜?KVM_CAP_ARM_PTRAUTH_GENERIC 閮藉瓨鍦紝
	  鍒欏繀椤诲悓鏃惰姹?KVM_ARM_VCPU_PTRAUTH_ADDRESS 鍜?KVM_ARM_VCPU_PTRAUTH_GENERIC锛?
	  鎴栬€呬袱鑰呴兘涓嶈姹傘€?

 - KVM_ARM_VCPU_SVE锛氫负 CPU 鍚敤 SVE锛堜粎 arm64锛夈€?
	  渚濊禆浜?KVM_CAP_ARM_SVE銆?
	  闇€瑕?KVM_ARM_VCPU_FINALIZE(KVM_ARM_VCPU_SVE)锛?

    - 鍦?KVM_ARM_VCPU_INIT 涔嬪悗锛?

       - 鍙互浣跨敤 KVM_GET_ONE_REG 璇诲彇 KVM_REG_ARM64_SVE_VLS锛氳浼瘎瀛樺櫒鐨勫垵濮嬪€兼寚绀?
	      鍦ㄦ瀹夸富鏈轰笂 vcpu 鍙兘鐨勬渶浣冲悜閲忛暱搴﹂泦鍚堛€?

    - 鍦?KVM_ARM_VCPU_FINALIZE(KVM_ARM_VCPU_SVE) 涔嬪墠锛?

       - KVM_RUN 鍜?KVM_GET_REG_LIST 涓嶅彲鐢紱

       - 涓嶈兘浣跨敤 KVM_GET_ONE_REG 鍜?KVM_SET_ONE_REG 璁块棶鍙几缂╃殑鏋舵瀯 SVE 瀵勫瓨鍣?
	        KVM_REG_ARM64_SVE_ZREG()銆並VM_REG_ARM64_SVE_PREG() 鎴?
	        KVM_REG_ARM64_SVE_FFR锛?

       - 鍙互閫夋嫨浣跨敤 KVM_SET_ONE_REG 鍐欏叆 KVM_REG_ARM64_SVE_VLS锛屼互淇敼 vcpu
	       鍙敤鐨勫悜閲忛暱搴﹂泦鍚堛€?

    - 鍦?KVM_ARM_VCPU_FINALIZE(KVM_ARM_VCPU_SVE) 涔嬪悗锛?

       - KVM_REG_ARM64_SVE_VLS 浼瘎瀛樺櫒鍙樹负涓嶅彲鍙橈紝涓嶈兘鍐嶄娇鐢?KVM_SET_ONE_REG 鍐欏叆銆?

 - KVM_ARM_VCPU_HAS_EL2锛氬惎鐢ㄥ祵濂楄櫄鎷熷寲鏀寔锛屼粠 EL2 鑰屼笉鏄?EL1 鍚姩瀹㈡埛鏈恒€?
	  渚濊禆浜?KVM_CAP_ARM_EL2銆?
	  闄ら潪鍚屾椂璁剧疆浜?KVM_ARM_VCPU_HAS_EL2_E2H0锛屽惁鍒?VM 浠?HCR_EL2.E2H 涓?RES1锛圴HE锛?
	  鐨勬柟寮忚繍琛屻€?

 - KVM_ARM_VCPU_HAS_EL2_E2H0锛氬皢宓屽铏氭嫙鍖栨敮鎸侀檺鍒朵负 HCR_EL2.E2H 涓?RES0锛堥潪 VHE锛夈€?
	  渚濊禆浜?KVM_CAP_ARM_EL2_E2H0銆?
	  杩樺繀椤昏缃?KVM_ARM_VCPU_HAS_EL2銆?

### 4.83 KVM_ARM_PREFERRED_TARGET


:Capability: basic
:Architectures: arm64
:Type: vm ioctl
:Parameters: struct kvm_vcpu_init (out)
:Returns: 0 on success; -1 on error

閿欒鐮侊細

  ======     ==========================================
  ENODEV     瀹夸富鏈烘病鏈夊彲鐢ㄧ殑棣栭€夌洰鏍?
  ======     ==========================================

璇?ioctl 鏌ヨ KVM 鍦ㄥ簳灞傚涓绘満涓婂彲妯℃嫙鐨勯閫?CPU 鐩爣绫诲瀷銆?

璇?ioctl 杩斿洖 struct kvm_vcpu_init 瀹炰緥锛屽叾涓寘鍚湁鍏抽閫?CPU 鐩爣绫诲瀷鍙婂叾鎺ㄨ崘鐗规€х殑淇℃伅銆?
濡傛灉棣栭€夌洰鏍囧缓璁缃繖浜涚壒鎬э紝鍒欒繑鍥炵殑 kvm_vcpu_init->features 浣嶅浘浼氱疆涓婄浉搴旂殑鐗规€т綅锛屼絾
杩欏苟闈炲己鍒惰姹傘€?

璇?ioctl 杩斿洖鐨勪俊鎭彲鐢ㄤ簬鍑嗗 struct kvm_vcpu_init 瀹炰緥浠ョ敤浜?KVM_ARM_VCPU_INIT ioctl锛?
浠庤€岀敓鎴愪笌搴曞眰瀹夸富鏈哄尮閰嶇殑 VCPU銆?


### 4.84 KVM_GET_REG_LIST


:Capability: basic
:Architectures: arm64, mips, riscv, x86 (if KVM_CAP_ONE_REG)
:Type: vcpu ioctl
:Parameters: struct kvm_reg_list (in/out)
:Returns: 0 on success; -1 on error

閿欒鐮侊細

  =====      ==============================================================
  E2BIG      reg 绱㈠紩鍒楄〃澶ぇ锛屾棤娉曟斁鍏ョ敤鎴锋寚瀹氱殑鏁扮粍涓紙鎵€闇€鐨勬暟閲忓皢琚啓鍏?n锛夈€?
  =====      ==============================================================

```

  struct kvm_reg_list {
	__u64 n; /* number of registers in reg[] */
	__u64 reg[0];
  };

```
璇?ioctl 杩斿洖鍙?KVM_GET_ONE_REG/KVM_SET_ONE_REG 璋冪敤鏀寔鐨勫鎴锋満瀵勫瓨鍣ㄣ€?

娉ㄦ剰锛岀敱浜庡巻鍙插師鍥狅紙璇寸櫧浜嗗氨鏄病浜哄叧蹇冿級锛宻390 涓嶆敮鎸?KVM_GET_REG_LIST銆傚湪鍐呮牳 4.x 鍙婃洿鏂?
鐗堟湰涓殑瀵勫瓨鍣ㄩ泦鍚堜负锛?

- KVM_REG_S390_TODPR

- KVM_REG_S390_EPOCHDIFF

- KVM_REG_S390_CPU_TIMER

- KVM_REG_S390_CLOCK_COMP

- KVM_REG_S390_PFTOKEN

- KVM_REG_S390_PFCOMPARE

- KVM_REG_S390_PFSELECT

- KVM_REG_S390_PP

- KVM_REG_S390_GBEA

娉ㄦ剰锛屽浜?x86锛岀敱 KVM_GET_MSR_INDEX_LIST 鏋氫妇鐨勬墍鏈?MSR 閮戒綔涓?KVM_X86_REG_TYPE_MSR 绫诲瀷
鍙楁敮鎸侊紝浣嗕笉浼氶€氳繃 KVM_GET_REG_LIST 鏋氫妇銆?

### 4.85 KVM_ARM_SET_DEVICE_ADDR (deprecated)


:Capability: KVM_CAP_ARM_SET_DEVICE_ADDR
:Architectures: arm64
:Type: vm ioctl
:Parameters: struct kvm_arm_device_address (in)
:Returns: 0 on success, -1 on error

閿欒鐮侊細

  ======  ============================================
  ENODEV  璁惧 id 鏈煡
  ENXIO   褰撳墠绯荤粺涓嶆敮鎸佽璁惧
  EEXIST  鍦板潃宸茶缃?
  E2BIG   鍦板潃瓒呭嚭瀹㈡埛鏈虹墿鐞嗗湴鍧€绌洪棿
  EBUSY   鍦板潃涓庡叾浠栬澶囪寖鍥撮噸鍙?
  ======  ============================================

```

  struct kvm_arm_device_addr {
	__u64 id;
	__u64 addr;
  };

```
鍦ㄥ鎴锋満鐗╃悊鍦板潃绌洪棿涓寚瀹氫竴涓澶囧湴鍧€锛屽鎴锋満鍙互鍦ㄨ鍦板潃璁块棶妯℃嫙鎴栫洿閫氱殑銆佸涓绘満鍐呮牳
闇€瑕佺煡鏅撶殑璁惧銆俰d 瀛楁鏄壒瀹氳澶囩殑涓€涓灦鏋勭浉鍏虫爣璇嗙銆?

arm64 灏?id 瀛楁鍒嗕负涓ら儴鍒嗭細涓€涓澶?id 鍜屼竴涓?
```

  bits:  | 63        ...       32 | 31    ...    16 | 15    ...    0 |
  field: |        0x00000000      |     device id   |  addr type id  |

```
arm64 鐩墠浠呭湪浣跨敤鍐呮牳鎬?GIC 鏀寔纭欢 VGIC 鐗规€ф椂鎵嶉渶瑕佸畠锛屼娇鐢?KVM_ARM_DEVICE_VGIC_V2
浣滀负璁惧 id銆傚湪涓哄鎴风殑 VGIC 铏氭嫙 CPU 鍜屽垎鍙戝櫒锛坉istributor锛夋帴鍙ｆ槧灏勮缃熀鍧€鏃讹紝蹇呴』鍦?
璋冪敤 KVM_CREATE_IRQCHIP 涔嬪悗銆佷絾鍦ㄤ换浣?VCPU 涓婅皟鐢?KVM_RUN 涔嬪墠璋冪敤璇?ioctl銆傚浠讳綍鍩哄潃
涓ゆ璋冪敤姝?ioctl 灏嗚繑鍥?-EEXIST銆?

娉ㄦ剰锛屾 IOCTL 宸插簾寮冿紝搴斾娇鐢ㄦ洿鐏垫椿鐨?SET/GET_DEVICE_ATTR API 浠ｆ浛銆?


### 4.86 KVM_PPC_RTAS_DEFINE_TOKEN


:Capability: KVM_CAP_PPC_RTAS
:Architectures: ppc
:Type: vm ioctl
:Parameters: struct kvm_rtas_token_args
:Returns: 0 on success, -1 on error

涓?RTAS锛圧un Time Abstraction Services锛岃繍琛屾椂鎶借薄鏈嶅姟锛夋湇鍔″畾涔変竴涓护鐗屽€硷紝浠ュ厑璁稿畠鍦ㄥ唴鏍镐腑
琚鐞嗐€傚弬鏁扮粨鏋勪綋缁欏嚭鏈嶅姟鐨勫悕绉帮紝璇ュ悕绉板繀椤绘槸鍏锋湁鍐呮牳渚у疄鐜扮殑鏈嶅姟鍚嶇О銆傚鏋滀护鐗屽€奸潪闆讹紝瀹冨皢
涓庤鏈嶅姟鍏宠仈锛屽鎴锋満闅忓悗鎸囧畾璇ヤ护鐗岀殑 RTAS 璋冪敤灏嗙敱鍐呮牳澶勭悊銆傚鏋滀护鐗屽€间负 0锛屽垯涓庤鏈嶅姟鍏宠仈鐨?
浠讳綍浠ょ墝閮藉皢琚仐蹇橈紝瀹㈡埛鏈洪殢鍚庨拡瀵硅鏈嶅姟鐨?RTAS 璋冪敤灏嗚浼犻€掔粰鐢ㄦ埛绌洪棿澶勭悊銆?

### 4.87 KVM_SET_GUEST_DEBUG


:Capability: KVM_CAP_SET_GUEST_DEBUG
:Architectures: x86, s390, ppc, arm64
:Type: vcpu ioctl
:Parameters: struct kvm_guest_debug (in)
:Returns: 0 on success; -1 on error

```

  struct kvm_guest_debug {
       __u32 control;
       __u32 pad;
       struct kvm_guest_debug_arch arch;
  };

```
璁剧疆澶勭悊鍣ㄧ壒瀹氱殑璋冭瘯瀵勫瓨鍣紝骞堕厤缃?vcpu 浠ュ鐞嗗鎴锋満璋冭瘯浜嬩欢銆傜粨鏋勪綋鏈変袱閮ㄥ垎锛岀涓€閮ㄥ垎鏄竴涓?
鎺у埗浣嶅煙锛屾寚绀鸿繍琛屾椂澶勭悊鐨勮皟璇曚簨浠剁被鍨嬨€傞€氱敤鎺у埗浣嶅涓嬶細

  - KVM_GUESTDBG_ENABLE:        鍚敤瀹㈡埛鏈鸿皟璇?
  - KVM_GUESTDBG_SINGLESTEP:    涓嬩竴娆¤繍琛屽簲鍗曟鎵ц

control 瀛楁鐨勯珮 16 浣嶆槸鏋舵瀯鐩稿叧鐨勬帶鍒舵爣蹇楋紝鍙寘鎷互涓嬶細

  - KVM_GUESTDBG_USE_SW_BP:     浣跨敤杞欢鏂偣 [x86, arm64]
  - KVM_GUESTDBG_USE_HW_BP:     浣跨敤纭欢鏂偣 [x86, s390]
  - KVM_GUESTDBG_USE_HW:        浣跨敤纭欢璋冭瘯浜嬩欢 [arm64]
  - KVM_GUESTDBG_INJECT_DB:     娉ㄥ叆 DB 绫诲瀷寮傚父 [x86]
  - KVM_GUESTDBG_INJECT_BP:     娉ㄥ叆 BP 绫诲瀷寮傚父 [x86]
  - KVM_GUESTDBG_EXIT_PENDING:  瑙﹀彂绔嬪嵆鐨勫鎴锋満閫€鍑?[s390]
  - KVM_GUESTDBG_BLOCKIRQ:      閬垮厤娉ㄥ叆涓柇/NMI/SMI [x86]

渚嬪锛孠VM_GUESTDBG_USE_SW_BP 琛ㄧず鍐呭瓨涓惎鐢ㄤ簡杞欢鏂偣锛屽洜姝ゆ垜浠渶瑕佺‘淇濇纭崟鑾锋柇鐐瑰紓甯革紝
骞朵笖 KVM 杩愯寰幆鍦ㄦ柇鐐瑰閫€鍑猴紝鑰屼笉鏄户缁繍琛屽埌姝ｅ父鐨勫鎴锋満鍚戦噺銆傚浜?KVM_GUESTDBG_USE_HW_BP锛?
鎴戜滑闇€瑕佺‘淇濆鎴锋満 vCPU 鐨勬灦鏋勭浉鍏冲瘎瀛樺櫒琚洿鏂颁负姝ｇ‘鐨勶紙鎻愪緵鐨勶級鍊笺€?

缁撴瀯浣撶殑绗簩閮ㄥ垎鏄灦鏋勭浉鍏崇殑锛岄€氬父鍖呭惈涓€缁勮皟璇曞瘎瀛樺櫒銆?

瀵逛簬 arm64锛岃皟璇曞瘎瀛樺櫒鐨勬暟閲忔槸瀹炵幇瀹氫箟鐨勶紝鍙互閫氳繃鏌ヨ KVM_CAP_GUEST_DEBUG_HW_BPS 鍜?
KVM_CAP_GUEST_DEBUG_HW_WPS 鑳藉姏鏉ョ‘瀹氾紝杩欎袱涓兘鍔涜繑鍥炰竴涓鏁帮紝鎸囩ず鍙楁敮鎸佺殑瀵勫瓨鍣ㄦ暟閲忋€?

瀵逛簬 ppc锛孠VM_CAP_PPC_GUEST_DEBUG_SSTEP 鑳藉姏鎸囩ず鏄惁鏀寔鍗曟璋冭瘯浜嬩欢
锛圞VM_GUESTDBG_SINGLESTEP锛夈€?

鍦ㄥ彈鏀寔鐨勬儏鍐典笅锛孠VM_CAP_SET_GUEST_DEBUG2 鑳藉姏鎸囩ず control 瀛楁涓彈鏀寔鐨?KVM_GUESTDBG_* 浣嶃€?

褰撹皟璇曚簨浠朵互 KVM_EXIT_DEBUG 鍘熷洜閫€鍑轰富杩愯寰幆鏃讹紝kvm_run 缁撴瀯浣撶殑 kvm_debug_exit_arch 閮ㄥ垎
鍖呭惈鏋舵瀯鐩稿叧鐨勮皟璇曚俊鎭€?

### 4.88 KVM_GET_EMULATED_CPUID


:Capability: KVM_CAP_EXT_EMUL_CPUID
:Architectures: x86
:Type: system ioctl
:Parameters: struct kvm_cpuid2 (in/out)
:Returns: 0 on success, -1 on error

```

  struct kvm_cpuid2 {
	__u32 nent;
	__u32 flags;
	struct kvm_cpuid_entry2 entries[0];
  };

```
member 'flags' 瀛楁鐢ㄤ簬浠庣敤鎴风┖闂翠紶閫掓爣蹇椼€?

```

  #define KVM_CPUID_FLAG_SIGNIFCANT_INDEX		BIT(0)
  #define KVM_CPUID_FLAG_STATEFUL_FUNC		BIT(1) /* deprecated */
  #define KVM_CPUID_FLAG_STATE_READ_NEXT		BIT(2) /* deprecated */

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
璇?ioctl 杩斿洖鐢?kvm 妯℃嫙鐨?x86 cpuid 鐗规€с€傜敤鎴风┖闂村彲浠ヤ娇鐢ㄨ ioctl 杩斿洖鐨勪俊鎭潵鏌ヨ鍝簺鐗规€?
鏄敱 kvm 妯℃嫙鐨勶紝鑰屼笉鏄師鐢熷瓨鍦ㄧ殑銆?

鐢ㄦ埛绌洪棿閫氳繃浼犲叆涓€涓?kvm_cpuid2 缁撴瀯浣撴潵璋冪敤 KVM_GET_EMULATED_CPUID锛屽叾涓?'nent' 瀛楁鎸囩ず
鍙彉闀挎暟缁?'entries' 涓殑鏉＄洰鏁伴噺銆傚鏋滄潯鐩暟閲忓お灏戣€屾棤娉曟弿杩?cpu 鑳藉姏锛屼細杩斿洖閿欒锛圗2BIG锛夈€?
濡傛灉鏁伴噺杩囧锛?nent' 瀛楁浼氳璋冩暣骞惰繑鍥炰竴涓敊璇紙ENOMEM锛夈€傚鏋滄暟閲忔伆濂藉悎閫傦紝'nent' 瀛楁浼氳
璋冩暣涓?'entries' 鏁扮粍涓湁鏁堟潯鐩殑鏁伴噺锛屽苟闅忓悗琚～鍏呫€?

杩斿洖鐨勬潯鐩槸 kvm 妯℃嫙鐨勫悇涓壒鎬х殑 CPUID 浣嶉泦鍚堬紝鐢?CPUID 鎸囦护杩斿洖锛屽叾涓湭鐭ユ垨涓嶆敮鎸佺殑鐗规€т綅
琚竻闆躲€?

渚嬪锛屽儚 x2apic 杩欐牱鐨勭壒鎬у彲鑳戒笉鍦ㄤ富鏈?cpu 涓紝浣嗗洜涓哄彲浠ヨ楂樻晥妯℃嫙鑰屽湪 KVM_GET_SUPPORTED_CPUID
涓敱 kvm 鏆撮湶锛屽洜姝や笉鍖呭惈鍦ㄦ澶勩€?

姣忎釜鏉＄洰涓殑瀛楁瀹氫箟濡備笅锛?

  function:
	 鐢ㄤ簬鑾峰彇璇ユ潯鐩殑 eax 鍊?
  index:
	 鐢ㄤ簬鑾峰彇璇ユ潯鐩殑 ecx 鍊硷紙閽堝鍙?ecx 褰卞搷鐨勬潯鐩級
  flags:
    浠ヤ笅闆朵釜鎴栧涓殑鎸変綅鎴栵細

        KVM_CPUID_FLAG_SIGNIFCANT_INDEX:
           琛ㄧず index 瀛楁鏈夋晥

   eax, ebx, ecx, edx:

         璇?function/index 缁勫悎涓?cpuid 鎸囦护杩斿洖鐨勫€?

### 4.89 KVM_S390_MEM_OP


:Capability: KVM_CAP_S390_MEM_OP, KVM_CAP_S390_PROTECTED, KVM_CAP_S390_MEM_OP_EXTENSION
:Architectures: s390
:Type: vm ioctl, vcpu ioctl
:Parameters: struct kvm_s390_mem_op (in)
:Returns: = 0 on success,
          < 0 on generic error (e.g. -EFAULT or -ENOMEM),
          16 bit program exception code if the access causes such an exception

浠?鍚?VM 鐨勫唴瀛樿鍙栨垨鍐欏叆鏁版嵁銆侹VM_CAP_S390_MEM_OP_EXTENSION 鑳藉姏鎸囧畾浜嗗彈鏀寔鐨勫姛鑳姐€?

```

  struct kvm_s390_mem_op {
	__u64 gaddr;		/* the guest address */
	__u64 flags;		/* flags */
	__u32 size;		/* amount of bytes */
	__u32 op;		/* type of operation */
	__u64 buf;		/* buffer in userspace */
	union {
		struct {
			__u8 ar;	/* the access register number */
			__u8 key;	/* access key, ignored if flag unset */
			__u8 pad1[6];	/* ignored */
			__u64 old_addr;	/* ignored if flag unset */
		};
		__u32 sida_offset; /* offset into the sida */
		__u8 reserved[32]; /* ignored */
	};
  };

```
鍐呭瓨鍖哄煙鐨勮捣濮嬪湴鍧€蹇呴』鍦?"gaddr" 瀛楁涓寚瀹氾紝鍖哄煙鐨勯暱搴﹀湪 "size" 瀛楁涓紙涓嶈兘涓?0锛夈€?size"
鐨勬渶澶у€煎彲浠ラ€氳繃妫€鏌?KVM_CAP_S390_MEM_OP 鑳藉姏鑾峰緱銆?buf" 鏄敤鎴风┖闂村簲鐢ㄧ▼搴忔彁渚涚殑缂撳啿鍖猴紝瀵逛簬
璇昏闂紝璇诲彇鐨勬暟鎹簲鍐欏叆璇ョ紦鍐插尯锛涘浜庡啓璁块棶锛岃鍐欏叆鐨勬暟鎹瓨鍌ㄥ湪璇ョ紦鍐插尯涓€?reserved" 瀛楁
鐢ㄤ簬鏈潵鐨勬墿灞曘€備繚鐣欏拰鏈娇鐢ㄧ殑鍊间細琚拷鐣ャ€傛坊鍔犳垚鍛樼殑鏈潵鎵╁睍蹇呴』寮曞叆鏂扮殑鏍囧織銆?

鎿嶄綔绫诲瀷鍦?"op" 瀛楁涓寚瀹氥€傚彲淇敼鍏惰涓虹殑鏍囧織鍙互鍦?"flags" 瀛楁涓缃€傛湭瀹氫箟鐨勬爣蹇椾綅蹇呴』
缃负 0銆?

鍙兘鐨勬搷浣滄湁锛?
  - `KVM_S390_MEMOP_LOGICAL_READ`
  - `KVM_S390_MEMOP_LOGICAL_WRITE`
  - `KVM_S390_MEMOP_ABSOLUTE_READ`
  - `KVM_S390_MEMOP_ABSOLUTE_WRITE`
  - `KVM_S390_MEMOP_SIDA_READ`
  - `KVM_S390_MEMOP_SIDA_WRITE`
  - `KVM_S390_MEMOP_ABSOLUTE_CMPXCHG`

##### Logical read/write锛堥€昏緫璇?鍐欙級锛?


璁块棶閫昏緫鍐呭瓨锛屽嵆鏍规嵁 VCPU 鐨勭姸鎬佸皢缁欏畾鐨勫鎴锋満鍦板潃杞崲涓虹粷瀵瑰湴鍧€锛屽苟浣跨敤璇ョ粷瀵瑰湴鍧€浣滀负璁块棶鐨?
鐩爣銆?ar" 鎸囧畾瑕佷娇鐢ㄧ殑璁块棶瀵勫瓨鍣ㄧ紪鍙凤紱鏈夋晥鑼冨洿鏄?0..15銆傞€昏緫璁块棶浠呭厑璁哥敤浜?VCPU ioctl銆傞€昏緫
璁块棶浠呭厑璁哥敤浜庨潪鍙椾繚鎶ょ殑瀹㈡埛鏈恒€?

鍙楁敮鎸佺殑鏍囧織锛?
  - `KVM_S390_MEMOP_F_CHECK_ONLY`
  - `KVM_S390_MEMOP_F_INJECT_EXCEPTION`
  - `KVM_S390_MEMOP_F_SKEY_PROTECTION`

鍙互璁剧疆 KVM_S390_MEMOP_F_CHECK_ONLY 鏍囧織锛屼互妫€鏌ョ浉搴旂殑鍐呭瓨璁块棶鏄惁浼氬鑷磋闂紓甯革紱浣嗘槸锛?
涓嶄細瀵圭洰鏍囧鍐呭瓨涓殑鏁版嵁杩涜瀹為檯璁块棶銆傚湪杩欑鎯呭喌涓嬶紝"buf" 鏈浣跨敤锛屽彲浠ヤ负 NULL銆?

濡傛灉鍦ㄨ闂湡闂村彂鐢熶簡璁块棶寮傚父锛堟垨鍦?KVM_S390_MEMOP_F_CHECK_ONLY 鎯呭喌涓嬪皢浼氬彂鐢燂級锛宨octl 杩斿洖
涓€涓鐨勯敊璇彿锛屾寚绀哄紓甯哥殑绫诲瀷銆傚鏋滆缃簡鏍囧織 KVM_S390_MEMOP_F_INJECT_EXCEPTION锛岃寮傚父涔熶細
鐩存帴鍦ㄧ浉搴旂殑 VCPU 涓婂紩鍙戙€傚湪淇濇姢寮傚父鐨勬儏鍐典笅锛岄櫎闈炲彟鏈夎鏄庯紝娉ㄥ叆鐨勭炕璇戝紓甯告爣璇嗙锛圱EID锛夎〃绀?
鎶戝埗锛坰uppression锛夈€?

濡傛灉璁剧疆浜?KVM_S390_MEMOP_F_SKEY_PROTECTION 鏍囧織锛屽瓨鍌ㄩ敭淇濇姢涔熶細鐢熸晥锛屽苟鍙兘鍦ㄨ闂洜 "key"
鎸囧畾鐨勮闂敭鑰岃绂佹鏃跺鑷村紓甯革紱鏈夋晥鑼冨洿鏄?0..15銆侹VM_S390_MEMOP_F_SKEY_PROTECTION 鍦?
KVM_CAP_S390_MEM_OP_EXTENSION 澶т簬 0 鏃跺彲鐢ㄣ€傜敱浜庤璁块棶鐨勫唴瀛樺彲鑳借法瓒婂涓〉锛岃€岃繖浜涢〉鍙兘鍏锋湁
涓嶅悓鐨勫瓨鍌ㄩ敭锛屽洜姝ゆ湁鍙兘鍦ㄥ唴瀛樺凡琚慨鏀逛箣鍚庢墠鍙戠敓淇濇姢寮傚父銆傚湪杩欑鎯呭喌涓嬶紝濡傛灉娉ㄥ叆浜嗗紓甯革紝TEID
涓嶄細鎸囩ず鎶戝埗銆?

##### Absolute read/write锛堢粷瀵硅/鍐欙級锛?


璁块棶缁濆鍐呭瓨銆傝鎿嶄綔鏃ㄥ湪涓?KVM_S390_MEMOP_F_SKEY_PROTECTION 鏍囧織涓€璧蜂娇鐢紝浠ュ厑璁稿湪涓€涓搷浣滀腑
璁块棶鍐呭瓨骞舵墽琛屽瓨鍌ㄩ敭淇濇姢鎵€闇€鐨勬鏌ワ紙鐩稿浜庣敤鎴风┖闂磋幏鍙栧瓨鍌ㄩ敭銆佹墽琛屾鏌ャ€佺劧鍚庤闂唴瀛橈紝杩欏彲鑳戒細
鍦ㄦ鏌ュ拰璁块棶涔嬮棿浜х敓寤惰繜锛夈€傚鏋?KVM_CAP_S390_MEM_OP_EXTENSION 璁剧疆浜?
KVM_S390_MEMOP_EXTENSION_CAP_BASE 浣嶏紝鍒欑粷瀵硅闂厑璁哥敤浜?VM ioctl銆傜洰鍓嶇粷瀵硅闂笉鍏佽鐢ㄤ簬 VCPU
ioctl銆傜粷瀵硅闂粎鍏佽鐢ㄤ簬闈炲彈淇濇姢鐨勫鎴锋満銆?

鍙楁敮鎸佺殑鏍囧織锛?
  - `KVM_S390_MEMOP_F_CHECK_ONLY`
  - `KVM_S390_MEMOP_F_SKEY_PROTECTION`

涓庨€昏緫璁块棶鍏辨湁鐨勬爣蹇楃殑璇箟涓庨€昏緫璁块棶鐩稿悓銆?

##### Absolute cmpxchg锛堢粷瀵规瘮杈冧氦鎹級锛?


瀵瑰鎴锋満缁濆鍐呭瓨鎵ц cmpxchg銆傛棬鍦ㄤ笌 KVM_S390_MEMOP_F_SKEY_PROTECTION 鏍囧織涓€璧蜂娇鐢ㄣ€備笌鏃犳潯浠?
鍐欏叆涓嶅悓锛屼粎褰撶洰鏍囦綅缃寘鍚?"old_addr" 鎸囧悜鐨勫€兼椂鎵嶄細鍙戠敓璁块棶銆傝繖浣滀负涓€娆″師瀛?cmpxchg 鎵ц锛?
闀垮害鐢?"size" 鍙傛暟鎸囧畾銆?size" 蹇呴』鏄?2 鐨勫箓锛屾渶澶т负 16锛堝惈锛夈€傚鏋滃洜涓虹洰鏍囧€间笌鏂板€间笉鍖归厤鑰?
鏈彂鐢熶氦鎹紝鍒?"old_addr" 鎸囧悜鐨勫€间細琚浛鎹负鐩爣鍊笺€傜敤鎴风┖闂村彲浠ラ€氳繃妫€鏌ユ槸鍚﹀彂鐢熶簡杩欑鏇挎崲鏉?
鍒ゆ柇浜ゆ崲鏄惁鍙戠敓銆傚鏋?KVM_CAP_S390_MEM_OP_EXTENSION 璁剧疆浜?
KVM_S390_MEMOP_EXTENSION_CAP_CMPXCHG 鏍囧織锛屽垯 cmpxchg 鎿嶄綔鍏佽鐢ㄤ簬 VM ioctl銆?

鍙楁敮鎸佺殑鏍囧織锛?
  - `KVM_S390_MEMOP_F_SKEY_PROTECTION`

##### SIDA read/write锛圫IDA 璇?鍐欙級锛?


璁块棶瀹夊叏鎸囦护鏁版嵁鍖猴紙secure instruction data area锛夛紝鍏朵腑鍖呭惈鍙椾繚鎶ゅ鎴锋満杩涜鎸囦护妯℃嫙鎵€闇€鐨?
鍐呭瓨鎿嶄綔鏁般€係IDA 璁块棶鍦?KVM_CAP_S390_PROTECTED 鑳藉姏鍙敤鏃舵彁渚涖€係IDA 璁块棶浠呭厑璁哥敤浜?VCPU
ioctl銆係IDA 璁块棶浠呭厑璁哥敤浜庡彈淇濇姢鐨勫鎴锋満銆?

涓嶆敮鎸佷换浣曟爣蹇椼€?

### 4.90 KVM_S390_GET_SKEYS


:Capability: KVM_CAP_S390_SKEYS
:Architectures: s390
:Type: vm ioctl
:Parameters: struct kvm_s390_skeys
:Returns: 0 on success, KVM_S390_GET_SKEYS_NONE if guest is not using storage
          keys, negative value on error

璇?ioctl 鐢ㄤ簬鍦?s390 涓婅幏鍙栧鎴锋満瀛樺偍閿殑鍊?
```

  struct kvm_s390_skeys {
	__u64 start_gfn;
	__u64 count;
	__u64 skeydata_addr;
	__u32 flags;
	__u32 reserved[9];
  };

```
start_gfn 瀛楁鏄綘瑕佽幏鍙栧叾瀛樺偍閿殑绗竴涓鎴锋満甯х殑缂栧彿銆?

count 瀛楁鏄鑾峰彇鍏跺瓨鍌ㄩ敭鐨勮繛缁抚鐨勬暟閲忥紙浠?start_gfn 寮€濮嬶級銆俢ount 瀛楁蹇呴』鑷冲皯涓?1锛屽厑璁?
鐨勬渶澶у€煎畾涔変负 KVM_S390_SKEYS_MAX銆傝秴鍑烘鑼冨洿鐨勫€煎皢瀵艰嚧 ioctl 杩斿洖 -EINVAL銆?

skeydata_addr 瀛楁鏄冻浠ュ绾?count 瀛楄妭鐨勭紦鍐插尯鐨勫湴鍧€銆傝缂撳啿鍖哄皢琚?ioctl 濉叆瀛樺偍閿暟鎹€?

### 4.91 KVM_S390_SET_SKEYS


:Capability: KVM_CAP_S390_SKEYS
:Architectures: s390
:Type: vm ioctl
:Parameters: struct kvm_s390_skeys
:Returns: 0 on success, negative value on error

璇?ioctl 鐢ㄤ簬鍦?s390 鏋舵瀯涓婅缃鎴锋満瀛樺偍閿殑鍊笺€傝 ioctl 閫氳繃 kvm_s390_skeys 缁撴瀯浣撴帴鏀跺弬鏁般€?
缁撴瀯浣撳畾涔夎鍙傝 KVM_S390_GET_SKEYS 涓€鑺傘€?

start_gfn 瀛楁鏄綘瑕佽缃叾瀛樺偍閿殑绗竴涓鎴锋満甯х殑缂栧彿銆?

count 瀛楁鏄鑾峰彇鍏跺瓨鍌ㄩ敭鐨勮繛缁抚鐨勬暟閲忥紙浠?start_gfn 寮€濮嬶級銆俢ount 瀛楁蹇呴』鑷冲皯涓?1锛屽厑璁?
鐨勬渶澶у€煎畾涔変负 KVM_S390_SKEYS_MAX銆傝秴鍑烘鑼冨洿鐨勫€煎皢瀵艰嚧 ioctl 杩斿洖 -EINVAL銆?

skeydata_addr 瀛楁鏄寘鍚?count 瀛楄妭瀛樺偍閿殑缂撳啿鍖虹殑鍦板潃銆傜紦鍐插尯涓殑姣忎釜瀛楄妭灏嗚璁剧疆涓轰粠
start_gfn 寮€濮嬨€佸叡 count 涓抚涓瘡涓抚鐨勫瓨鍌ㄩ敭銆?

娉ㄦ剰锛氬鏋滃湪缁欏畾鐨勬暟鎹腑鍙戠幇浠讳綍鏋舵瀯鏃犳晥鐨勯敭鍊硷紝ioctl 灏嗚繑鍥?-EINVAL銆?

### 4.92 KVM_S390_IRQ


:Capability: KVM_CAP_S390_INJECT_IRQ
:Architectures: s390
:Type: vcpu ioctl
:Parameters: struct kvm_s390_irq (in)
:Returns: 0 on success, -1 on error

閿欒鐮侊細

  ======  =================================================================
  EINVAL  涓柇绫诲瀷鏃犳晥
          type 涓?KVM_S390_SIGP_STOP 涓?flag 鍙傛暟涓烘棤鏁堝€硷紝
          type 涓?KVM_S390_INT_EXTERNAL_CALL 涓?code 澶т簬
          VCPU 鐨勬渶澶ф暟閲?
  EBUSY   type 涓?KVM_S390_SIGP_SET_PREFIX 涓?vcpu 鏈仠姝紝
          type 涓?KVM_S390_SIGP_STOP 涓斿凡鏈変竴涓?stop 涓柇鎸傝捣锛?
          type 涓?KVM_S390_INT_EXTERNAL_CALL 涓斿凡鏈変竴涓閮ㄨ皟鐢ㄤ腑鏂?
          鎸傝捣
  ======  =================================================================

鍏佽鍚戝鎴锋満娉ㄥ叆涓€涓腑鏂€?

浣跨敤 struct kvm_s390_irq 浣滀负鍙傛暟鍙互娉ㄥ叆鏃犳硶閫氳繃 KVM_S390_INTERRUPT 娉ㄥ叆鐨勯澶栬礋杞姐€?

```

  struct kvm_s390_irq {
	__u64 type;
	union {
		struct kvm_s390_io_info io;
		struct kvm_s390_ext_info ext;
		struct kvm_s390_pgm_info pgm;
		struct kvm_s390_emerg_info emerg;
		struct kvm_s390_extcall_info extcall;
		struct kvm_s390_prefix_info prefix;
		struct kvm_s390_stop_info stop;
		struct kvm_s390_mchk_info mchk;
		char reserved[64];
	} u;
  };

```
type 鍙互鏄互涓嬩箣涓€锛?

- KVM_S390_SIGP_STOP - sigp 鍋滄锛涘弬鏁板湪 .stop 涓?
- KVM_S390_PROGRAM_INT - 绋嬪簭妫€鏌ワ紱鍙傛暟鍦?.pgm 涓?
- KVM_S390_SIGP_SET_PREFIX - sigp 璁剧疆鍓嶇紑锛涘弬鏁板湪 .prefix 涓?
- KVM_S390_RESTART - 閲嶅惎锛涙棤鍙傛暟
- KVM_S390_INT_CLOCK_COMP - 鏃堕挓姣旇緝鍣ㄤ腑鏂紱鏃犲弬鏁?
- KVM_S390_INT_CPU_TIMER - CPU 瀹氭椂鍣ㄤ腑鏂紱鏃犲弬鏁?
- KVM_S390_INT_EMERGENCY - sigp 绱ф€ワ紱鍙傛暟鍦?.emerg 涓?
- KVM_S390_INT_EXTERNAL_CALL - sigp 澶栭儴璋冪敤锛涘弬鏁板湪 .extcall 涓?
- KVM_S390_MCHK - 鏈哄櫒妫€鏌ヤ腑鏂紱鍙傛暟鍦?.mchk 涓?

杩欐槸涓€涓紓姝ョ殑 vcpu ioctl锛屽彲浠ヤ粠浠讳綍绾跨▼璋冪敤銆?

### 4.94 KVM_S390_GET_IRQ_STATE


:Capability: KVM_CAP_S390_IRQ_STATE
:Architectures: s390
:Type: vcpu ioctl
:Parameters: struct kvm_s390_irq_state (out)
:Returns: >= number of bytes copied into buffer,
          -EINVAL if buffer size is 0,
          -ENOBUFS if buffer size is too small to fit all pending interrupts,
          -EFAULT if the buffer address was invalid

璇?ioctl 鍏佽鐢ㄦ埛绌洪棿鍦ㄥ崟涓紦鍐插尯涓绱㈠綋鍓嶆墍鏈夋寕璧蜂腑鏂殑瀹屾暣鐘舵€併€傜敤渚嬪寘鎷縼绉诲拰鑷渷銆傚弬鏁?
缁撴瀯浣撳寘鍚?
```

  struct kvm_s390_irq_state {
	__u64 buf;
	__u32 flags;        /* will stay unused for compatibility reasons */
	__u32 len;
	__u32 reserved[4];  /* will stay unused for compatibility reasons */
  };

```
鐢ㄦ埛绌洪棿浼犲叆涓婅堪缁撴瀯浣擄紝瀵逛簬姣忎釜鎸傝捣鐨勪腑鏂紝涓€涓?struct kvm_s390_irq 浼氳澶嶅埗鍒版彁渚涚殑缂撳啿鍖轰腑銆?

璇ョ粨鏋勪綋鍖呭惈涓€涓?flags 瀛楁鍜屼竴涓?reserved 瀛楁锛岀敤浜庢湭鏉ョ殑鎵╁睍銆傜敱浜庡唴鏍镐粠鏈鏌?flags == 0锛?
鑰?QEMU 涔熶粠鏈娓呴浂 flags 鍜?reserved锛屽洜姝ゆ湭鏉ュ鏋滀笉鐮村潖鍏煎鎬э紝灏辨棤娉曚娇鐢ㄨ繖浜涘瓧娈点€?

濡傛灉杩斿洖 -ENOBUFS锛屽垯鎻愪緵鐨勭紦鍐插尯澶皬锛岀敤鎴风┖闂村彲浠ヤ娇鐢ㄦ洿澶х殑缂撳啿鍖洪噸璇曘€?

### 4.95 KVM_S390_SET_IRQ_STATE


:Capability: KVM_CAP_S390_IRQ_STATE
:Architectures: s390
:Type: vcpu ioctl
:Parameters: struct kvm_s390_irq_state (in)
:Returns: 0 on success,
          -EFAULT if the buffer address was invalid,
          -EINVAL for an invalid buffer length (see below),
          -EBUSY if there were already interrupts pending,
          errors occurring when actually injecting the
          interrupt. See KVM_S390_IRQ.

璇?ioctl 鍏佽鐢ㄦ埛绌洪棿璁剧疆褰撳墠涓鸿 vcpu 鎸傝捣鐨勬墍鏈?cpu 鏈湴涓柇鐨勫畬鏁寸姸鎬併€傚畠鏃ㄥ湪鐢ㄤ簬杩佺Щ鍚?
鎭㈠涓柇鐘舵€併€傝緭鍏ュ弬鏁版槸涓€涓敤鎴风┖闂寸紦鍐插尯
```

  struct kvm_s390_irq_state {
	__u64 buf;
	__u32 flags;        /* will stay unused for compatibility reasons */
	__u32 len;
	__u32 reserved[4];  /* will stay unused for compatibility reasons */
  };

```
鍏充簬 flags 鍜?reserved 鐨勯檺鍒跺悓鏍烽€傜敤銆傦紙瑙?KVM_S390_GET_IRQ_STATE锛?

buf 寮曠敤鐨勭敤鎴风┖闂村唴瀛樺寘鍚瘡涓娉ㄥ叆鍒板鎴锋満鐨勪腑鏂搴旂殑涓€涓?struct kvm_s390_irq銆?

濡傛灉鍏朵腑鏌愪釜涓柇鐢变簬鏌愮鍘熷洜鏃犳硶娉ㄥ叆锛宨octl 浼氫腑姝€?

len 蹇呴』鏄?sizeof(struct kvm_s390_irq) 鐨勫€嶆暟銆傚畠蹇呴』 > 0锛屼笖涓嶅緱瓒呰繃
(max_vcpus + 32) * sizeof(struct kvm_s390_irq)锛屽嵆鍙兘鎸傝捣鐨?cpu 鏈湴涓柇鐨勬渶澶ф暟閲忋€?

### 4.96 KVM_SMI


:Capability: KVM_CAP_X86_SMM
:Architectures: x86
:Type: vcpu ioctl
:Parameters: none
:Returns: 0 on success, -1 on error

鍦ㄧ嚎绋嬬殑 vcpu 涓婃帓闃熶竴涓?SMI銆?

### 4.97 KVM_X86_SET_MSR_FILTER


:Capability: KVM_CAP_X86_MSR_FILTER
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_msr_filter
:Returns: 0 on success, < 0 on error

```

  struct kvm_msr_filter_range {
  #define KVM_MSR_FILTER_READ  (1 << 0)
  #define KVM_MSR_FILTER_WRITE (1 << 1)
	__u32 flags;
	__u32 nmsrs; /* number of msrs in bitmap */
	__u32 base;  /* MSR index the bitmap starts at */
	__u8 *bitmap; /* a 1 bit allows the operations in flags, 0 denies */
  };

  #define KVM_MSR_FILTER_MAX_RANGES 16
  struct kvm_msr_filter {
  #define KVM_MSR_FILTER_DEFAULT_ALLOW (0 << 0)
  #define KVM_MSR_FILTER_DEFAULT_DENY  (1 << 0)
	__u32 flags;
	struct kvm_msr_filter_range ranges[KVM_MSR_FILTER_MAX_RANGES];
  };

```
`struct kvm_msr_filter_range` 鐨?flags 鍊硷細

`KVM_MSR_FILTER_READ`

  浣跨敤缁欏畾鐨勪綅鍥捐繃婊ゅ MSR 鐨勮璁块棶銆備綅鍥句腑涓?0 琛ㄧず搴旀嫆缁濊璁块棶锛屼负 1 琛ㄧず鏃犺榛樿杩囨护鍣?
  鍔ㄤ綔濡備綍锛岄兘搴斿厑璁稿鐗瑰畾 MSR 鐨勮璁块棶銆?

`KVM_MSR_FILTER_WRITE`

  浣跨敤缁欏畾鐨勪綅鍥捐繃婊ゅ MSR 鐨勫啓璁块棶銆備綅鍥句腑涓?0 琛ㄧず搴旀嫆缁濆啓璁块棶锛屼负 1 琛ㄧず鏃犺榛樿杩囨护鍣?
  鍔ㄤ綔濡備綍锛岄兘搴斿厑璁稿鐗瑰畾 MSR 鐨勫啓璁块棶銆?

`struct kvm_msr_filter` 鐨?flags 鍊硷細

`KVM_MSR_FILTER_DEFAULT_ALLOW`

  濡傛灉娌℃湁杩囨护鑼冨洿鍖归厤姝ｅ湪琚闂殑 MSR 绱㈠紩锛孠VM 榛樿鍏佽瀵规墍鏈?MSR 鐨勮闂€?

`KVM_MSR_FILTER_DEFAULT_DENY`

  濡傛灉娌℃湁杩囨护鑼冨洿鍖归厤姝ｅ湪琚闂殑 MSR 绱㈠紩锛孠VM 榛樿鎷掔粷瀵规墍鏈?MSR 鐨勮闂€?

璇?ioctl 鍏佽鐢ㄦ埛绌洪棿瀹氫箟鏈€澶?16 涓?MSR 鑼冨洿浣嶅浘锛屼互鎷掔粷閫氬父琚?KVM 鍏佽鐨勫鏈?MSR 璁块棶銆傚鏋?
鏌愪釜 MSR 鏈鐗瑰畾鑼冨洿瑕嗙洊锛屽垯搴旂敤"榛樿"杩囨护琛屼负銆傛瘡涓綅鍥捐寖鍥磋鐩?[base .. base+nmsrs) 鑼冨洿鍐呯殑
MSR銆?

濡傛灉 MSR 璁块棶琚敤鎴风┖闂存嫆缁濓紝鐢辨浜х敓鐨?KVM 琛屼负鍙栧喅浜庢槸鍚﹀惎鐢ㄤ簡
KVM_CAP_X86_USER_SPACE_MSR 鐨?KVM_MSR_EXIT_REASON_FILTER銆傚鏋滃惎鐢ㄤ簡 KVM_MSR_EXIT_REASON_FILTER锛?
KVM 鍦ㄨ鎷掔粷鐨勮闂笂浼氶€€鍑哄埌鐢ㄦ埛绌洪棿锛屽嵆鐢ㄦ埛绌洪棿瀹為檯涓婃嫤鎴簡璇?MSR 璁块棶銆傚鏋滄湭鍚敤
KVM_MSR_EXIT_REASON_FILTER锛孠VM 浼氬湪琚嫆缁濈殑璁块棶涓婂悜瀹㈡埛鏈烘敞鍏ヤ竴涓?#GP銆傛敞鎰忥紝濡傛灉鍦?VMX 杞崲
鏈熼棿妯℃嫙 MSR 鍔犺浇/瀛樺偍鏃?MSR 璁块棶琚嫆缁濓紝KVM 浼氬拷鐣?KVM_MSR_EXIT_REASON_FILTER銆傚畬鏁寸粏鑺傝鍙傝
涓嬮潰鐨勮鍛娿€?

濡傛灉 MSR 璁块棶琚敤鎴风┖闂村厑璁革紝KVM 灏嗘牴鎹?vCPU 妯″瀷妯℃嫙鍜?鎴栬櫄鎷熷寲璇ヨ闂€傛敞鎰忥紝濡傛灉璁块棶琚敤鎴风┖闂?
鍏佽锛孠VM 鏈€缁堜粛鍙兘娉ㄥ叆 #GP锛屼緥濡?KVM 涓嶆敮鎸佽 MSR锛屾垨鑰呬负浜嗛伒寰 MSR 鐨勬灦鏋勮涓恒€?

榛樿鎯呭喌涓嬶紝KVM 浠?KVM_MSR_FILTER_DEFAULT_ALLOW 妯″紡杩愯锛屼笖娌℃湁 MSR 鑼冨洿杩囨护鍣ㄣ€?

浣跨敤涓€缁勭┖鑼冨洿锛堟墍鏈?nmsrs == 0锛夎皟鐢ㄦ ioctl 浼氱鐢?MSR 杩囨护銆傚湪璇ユā寮忎笅锛宍KVM_MSR_FILTER_DEFAULT_DENY`
鏃犳晥骞朵細瀵艰嚧閿欒銆?

   MSR 璁块棶浣滀负鎸囦护鎵ц锛堟ā鎷熸垨鍘熺敓锛夌殑鍓綔鐢ㄤ笉浼氳杩囨护锛屽洜涓虹‖浠跺湪 RDMSR 鍜?WRMSR 涔嬪涓嶉伒寰?
   MSR 浣嶅浘锛岃€?KVM 鍦ㄦā鎷熸寚浠ゆ椂浼氭ā浠胯琛屼负锛屼互閬垮厤涓庣‖浠朵骇鐢熸棤鎰忎箟鐨勫亸宸€備緥濡傦紝RDPID 璇诲彇
   MSR_TSC_AUX锛孲YSENTER 璇诲彇 SYSENTER MSR锛岀瓑绛夈€?

   MSR 閫氳繃涓撶敤 VMCS 瀛楁鍔犺浇/瀛樺偍鐨勶紝涓嶄細浣滀负 VM-Enter/VM-Exit 妯℃嫙鐨勪竴閮ㄥ垎琚繃婊ゃ€?

   MSR 閫氳繃 VMX 鐨勫姞杞?瀛樺偍鍒楄〃鍔犺浇/瀛樺偍鐨勶紝浼氫綔涓?VM-Enter/VM-Exit 妯℃嫙鐨勪竴閮ㄥ垎琚繃婊ゃ€傚鏋?
   鍦?VM-Enter 鏃?MSR 璁块棶琚嫆缁濓紝KVM 浼氬悎鎴愪竴涓竴鑷存€ф鏌?VM-Exit锛圗XIT_REASON_MSR_LOAD_FAIL锛夈€?
   濡傛灉鍦?VM-Exit 鏃?MSR 璁块棶琚嫆缁濓紝KVM 浼氬悎鎴愪竴涓?VM-Abort銆傜畝鑰岃█涔嬶紝KVM 鎵╁睍浜?Intel 鐨?
   鏋舵瀯鍒楄〃锛屽垪鍑洪偅浜涙棤娉曢€氳繃 VM-Enter/VM-Exit MSR 鍒楄〃鍔犺浇/淇濆瓨鐨?MSR銆傚钩鍙版墍鏈夎€呮湁璐ｄ换灏嗕换浣?
   姝ょ被闄愬埗浼犺揪缁欏叾鏈€缁堢敤鎴枫€?

   x2APIC MSR 璁块棶鏃犳硶琚繃婊わ紙KVM 浼氶潤榛樺拷鐣ヨ鐩栦换浣?x2APIC MSR 鐨勮繃婊ゅ櫒锛夈€?

娉ㄦ剰锛屽湪 vCPU 杩愯鏃惰皟鐢ㄦ ioctl 鏈川涓婃槸绔炴€佺殑銆備絾鏄紝KVM 纭疄淇濊瘉 vCPU 灏嗙湅鍒板厛鍓嶇殑杩囨护鍣?
鎴栨柊鐨勮繃婊ゅ櫒涔嬩竴锛屼緥濡傦紝鍦ㄦ棫杩囨护鍣ㄥ拰鏂拌繃婊ゅ櫒涓叿鏈夌浉鍚岃缃殑 MSR 灏嗗叿鏈夌‘瀹氭€х殑琛屼负銆?

绫讳技鍦帮紝濡傛灉鐢ㄦ埛绌洪棿甯屾湜鍦ㄦ嫆缁濈殑璁块棶涓婅繘琛屾嫤鎴紝蹇呴』鍦ㄦ縺娲讳换浣曡繃婊ゅ櫒涔嬪墠鍚敤
KVM_MSR_EXIT_REASON_FILTER锛屽苟鍦ㄦ墍鏈夎繃婊ゅ櫒鍋滅敤涔嬪悗鎵嶅皢鍏跺叧闂€傚惁鍒欏彲鑳藉鑷?KVM 娉ㄥ叆 #GP 鑰屼笉鏄?
閫€鍑哄埌鐢ㄦ埛绌洪棿銆?

### 4.98 KVM_CREATE_SPAPR_TCE_64


:Capability: KVM_CAP_SPAPR_TCE_64
:Architectures: powerpc
:Type: vm ioctl
:Parameters: struct kvm_create_spapr_tce_64 (in)
:Returns: file descriptor for manipulating the created TCE table

杩欐槸 KVM_CAP_SPAPR_TCE 鐨勬墿灞曪紝鍚庤€呬粎鏀寔 32 浣嶇獥鍙ｏ紝鍦?4.62 KVM_CREATE_SPAPR_TCE 涓弿杩般€?

```

  /* for KVM_CAP_SPAPR_TCE_64 */
  struct kvm_create_spapr_tce_64 {
	__u64 liobn;
	__u32 page_shift;
	__u32 flags;
	__u64 offset;	/* in pages */
	__u64 size; 	/* in pages */
  };

```
璇ユ墿灞曠殑鐩殑鏄敮鎸佷竴涓澶栫殑銆佸叿鏈夊彲鍙橀〉澶у皬鐨勬洿澶?DMA 绐楀彛銆侹VM_CREATE_SPAPR_TCE_64 鎺ユ敹
涓€涓?64 浣嶇殑绐楀彛澶у皬銆佷竴涓?IOMMU 椤靛亸绉伙紙page shift锛変互鍙婄浉搴?DMA 绐楀彛鐨勬€荤嚎鍋忕Щ锛坆us offset锛夛紝
@size 鍜?@offset 鏄?IOMMU 椤电殑鏁伴噺銆?

@flags 鐩墠鏈浣跨敤銆?

鍏朵綑鍔熻兘涓?KVM_CREATE_SPAPR_TCE 鐩稿悓銆?

### 4.99 KVM_REINJECT_CONTROL


:Capability: KVM_CAP_REINJECT_CONTROL
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_reinject_control (in)
:Returns: 0 on success,
         -EFAULT if struct kvm_reinject_control cannot be read,
         -ENXIO if KVM_CREATE_PIT or KVM_CREATE_PIT2 didn't succeed earlier.

i8254锛圥IT锛夋湁涓ょ妯″紡锛宺einject 鍜?!reinject銆傞粯璁ゆ槸 reinject锛屽嵆 KVM 鎺掗槦宸叉祦閫濈殑 i8254
tick 骞剁洃鎺?i8254 娉ㄥ叆鐨勪腑鏂殑瀹屾垚銆俽einject 妯″紡浼氬湪娌℃湁鏉ヨ嚜 i8254 鐨勬寕璧蜂腑鏂椂鍑洪槦涓€涓?tick
骞舵敞鍏ュ叾涓柇銆?reinject 妯″紡鍦?tick 鍒拌揪鏃剁珛鍗虫敞鍏ヤ腑鏂€?

```

  struct kvm_reinject_control {
	__u8 pit_reinject;
	__u8 reserved[31];
  };

```
闄ら潪杩愯浣跨敤 PIT 杩涜瀹氭椂鐨勬棫鎿嶄綔绯荤粺锛堜緥濡?Linux 2.4.x锛夛紝鍚﹀垯寤鸿浣跨敤 pit_reinject = 0
锛?reinject 妯″紡锛夈€?

### 4.100 KVM_PPC_CONFIGURE_V3_MMU


:Capability: KVM_CAP_PPC_MMU_RADIX or KVM_CAP_PPC_MMU_HASH_V3
:Architectures: ppc
:Type: vm ioctl
:Parameters: struct kvm_ppc_mmuv3_cfg (in)
:Returns: 0 on success,
         -EFAULT if struct kvm_ppc_mmuv3_cfg cannot be read,
         -EINVAL if the configuration is invalid

璇?ioctl 鎺у埗瀹㈡埛鏈烘槸浣跨敤 radix 杩樻槸 HPT锛堝搱甯岄〉琛級杞崲锛屽苟璁剧疆鎸囧悜瀹㈡埛鏈鸿繘绋嬭〃鐨勬寚閽堛€?

```

  struct kvm_ppc_mmuv3_cfg {
	__u64	flags;
	__u64	process_table;
  };

```
鍙互鍦?flags 涓缃袱涓綅锛欿VM_PPC_MMUV3_RADIX 鍜?KVM_PPC_MMUV3_GTSE銆侹VM_PPC_MMUV3_RADIX 濡傛灉
缃綅锛屽垯灏嗗鎴锋満閰嶇疆涓轰娇鐢?radix 鏍戣浆鎹紱濡傛灉娓呴浂锛屽垯浣跨敤 HPT 杞崲銆侹VM_PPC_MMUV3_GTSE 濡傛灉
缃綅涓?KVM 鍏佽锛屽垯灏嗗鎴锋満閰嶇疆涓鸿兘澶熶娇鐢ㄥ叏灞€ TLB 鍜?SLB 澶辨晥鎸囦护锛涘鏋滄竻闆讹紝瀹㈡埛鏈轰笉寰椾娇鐢?
杩欎簺鎸囦护銆?

process_table 瀛楁鎸囧畾瀹㈡埛鏈鸿繘绋嬭〃鐨勫湴鍧€鍜屽ぇ灏忥紝璇ヨ〃浣嶄簬瀹㈡埛鏈虹┖闂翠腑銆傝瀛楁鐨勬牸寮忎负鍒嗗尯琛ㄩ」
锛坧artition table entry锛夌殑绗簩涓弻瀛楋紝濡?Power ISA V3.00 绗?III 鍐?5.7.6.1 鑺傛墍瀹氫箟銆?

### 4.101 KVM_PPC_GET_RMMU_INFO


:Capability: KVM_CAP_PPC_MMU_RADIX
:Architectures: ppc
:Type: vm ioctl
:Parameters: struct kvm_ppc_rmmu_info (out)
:Returns: 0 on success,
	 -EFAULT if struct kvm_ppc_rmmu_info cannot be written,
	 -EINVAL if no useful information can be returned

璇?ioctl 杩斿洖涓€涓粨鏋勪綋锛屽叾涓寘鍚袱鏍蜂笢瑗匡細(a) 涓€涓寘鍚彈鏀寔鐨?radix 鏍戝嚑浣曞竷灞€鐨勫垪琛紝浠ュ強
(b) 涓€涓皢椤靛ぇ灏忔槧灏勫埌 tlbie锛圱LB 澶辨晥鏉＄洰锛夋寚浠ょ殑 "AP"锛堝疄闄呴〉澶у皬锛夊瓧娈电殑鍒楄〃銆?

```

  struct kvm_ppc_rmmu_info {
	struct kvm_ppc_radix_geom {
		__u8	page_shift;
		__u8	level_bits[4];
		__u8	pad[3];
	}	geometries[8];
	__u32	ap_encodings[8];
  };

```
geometries[] 瀛楁缁欏嚭鏈€澶?8 绉嶅彈鏀寔鐨?radix 椤佃〃鍑犱綍甯冨眬锛屼互鏈€灏忛〉澶у皬浠?2 涓哄簳鐨勫鏁帮紝浠ュ強
浠?PTE 绾у埌 PGD 绾э紙鎸夋椤哄簭锛夋爲姣忎竴绾х储寮曠殑浣嶆暟琛ㄧず銆備换浣曟湭浣跨敤鐨勬潯鐩湪 page_shift 瀛楁涓负 0銆?

ap_encodings 缁欏嚭鍙楁敮鎸佺殑椤靛ぇ灏忓強鍏?AP 瀛楁缂栫爜锛屼互 AP 鍊间綅浜庨珮 3 浣嶃€侀〉澶у皬浠?2 涓哄簳鐨勫鏁?
浣嶄簬浣?6 浣嶈繘琛岀紪鐮併€?

### 4.102 KVM_PPC_RESIZE_HPT_PREPARE


:Capability: KVM_CAP_SPAPR_RESIZE_HPT
:Architectures: powerpc
:Type: vm ioctl
:Parameters: struct kvm_ppc_resize_hpt (in)
:Returns: 0 on successful completion,
	 >0 if a new HPT is being prepared, the value is an estimated
         number of milliseconds until preparation is complete,
         -EFAULT if struct kvm_reinject_control cannot be read,
	 -EINVAL if the supplied shift or flags are invalid,
	 -ENOMEM if unable to allocate the new HPT,

鐢ㄤ簬瀹炵幇 PAPR 鎵╁睍锛屼互鍦ㄨ繍琛屾椂璋冩暣瀹㈡埛鏈哄搱甯岄〉琛紙HPT锛夌殑澶у皬銆傚叿浣撴潵璇达紝瀹冨惎鍔ㄣ€佸仠姝㈡垨鐩戣
涓哄鎴锋満鍑嗗涓€涓柊鐨勬綔鍦?HPT锛屽疄璐ㄤ笂瀹炵幇浜?H_RESIZE_HPT_PREPARE hypercall銆?

```

  struct kvm_ppc_resize_hpt {
	__u64 flags;
	__u32 shift;
	__u32 pad;
  };

```
濡傛灉鍦ㄥ鎴锋満娌℃湁鎸傝捣鐨?HPT 鏃朵互 shift > 0 璋冪敤锛岃繖灏嗗紑濮嬪噯澶囦竴涓柊鐨勩€佸ぇ灏忎负 2^(shift) 瀛楄妭鐨?
鎸傝捣 HPT銆傜劧鍚庡畠杩斿洖涓€涓鏁存暟锛岃〃绀鸿窛绂诲噯澶囧畬鎴愪及璁＄殑姣鏁般€?

濡傛灉鍦ㄥ瓨鍦ㄦ寕璧风殑 HPT 浣嗗叾澶у皬涓庡弬鏁颁腑璇锋眰鐨勪笉鍖归厤鏃惰皟鐢紝鍒欎涪寮冪幇鏈夌殑鎸傝捣 HPT锛屽苟鎸変笂杩版柟寮?
鍒涘缓涓€涓柊鐨勩€?

濡傛灉鍦ㄥ瓨鍦ㄨ姹傚ぇ灏忕殑鎸傝捣 HPT 鏃惰皟鐢紝灏嗭細

  - 濡傛灉鎸傝捣 HPT 鐨勫噯澶囧凡瀹屾垚锛岃繑鍥?0
  - 濡傛灉鎸傝捣 HPT 鐨勫噯澶囧凡澶辫触锛岃繑鍥為敊璇爜锛岀劧鍚庝涪寮冩寕璧风殑 HPT
  - 濡傛灉鎸傝捣 HPT 鐨勫噯澶囦粛鍦ㄨ繘琛屼腑锛岃繑鍥炶窛绂诲噯澶囧畬鎴愪及璁＄殑姣鏁?

濡傛灉浠?shift == 0 璋冪敤锛屽垯涓㈠純浠讳綍褰撳墠鎸傝捣鐨?HPT 骞惰繑鍥?0锛堝嵆鍙栨秷浠讳綍姝ｅ湪杩涜鐨勫噯澶囷級銆?

flags 淇濈暀鐢ㄤ簬鏈潵鐨勬墿灞曪紝鐩墠璁剧疆 flags 涓殑浠讳綍浣嶉兘灏嗗鑷?-EINVAL銆?

閫氬父杩欏皢浣跨敤鐩稿悓鐨勫弬鏁伴噸澶嶈皟鐢紝鐩村埌瀹冭繑鍥?<= 0銆傜涓€娆¤皟鐢ㄥ皢鍚姩鍑嗗锛屽悗缁皟鐢ㄥ皢鐩戣鍑嗗锛?
鐩村埌瀹屾垚鎴栧け璐ャ€?

### 4.103 KVM_PPC_RESIZE_HPT_COMMIT


:Capability: KVM_CAP_SPAPR_RESIZE_HPT
:Architectures: powerpc
:Type: vm ioctl
:Parameters: struct kvm_ppc_resize_hpt (in)
:Returns: 0 on successful completion,
         -EFAULT if struct kvm_reinject_control cannot be read,
	 -EINVAL if the supplied shift or flags are invalid,
	 -ENXIO is there is no pending HPT, or the pending HPT doesn't
         have the requested size,
	 -EBUSY if the pending HPT is not fully prepared,
	 -ENOSPC if there was a hash collision when moving existing
         HPT entries to the new HPT,
	 -EIO on other error conditions

鐢ㄤ簬瀹炵幇 PAPR 鎵╁睍锛屼互鍦ㄨ繍琛屾椂璋冩暣瀹㈡埛鏈哄搱甯岄〉琛紙HPT锛夌殑澶у皬銆傚叿浣撴潵璇达紝瀹冭姹傚皢瀹㈡埛鏈鸿浆绉诲埌
浣跨敤鏂扮殑 HPT 宸ヤ綔锛屽疄璐ㄤ笂瀹炵幇浜?H_RESIZE_HPT_COMMIT hypercall銆?

```

  struct kvm_ppc_resize_hpt {
	__u64 flags;
	__u32 shift;
	__u32 pad;
  };

```
杩欏彧搴斿湪 KVM_PPC_RESIZE_HPT_PREPARE 浠ョ浉鍚屽弬鏁拌繑鍥?0 涔嬪悗璋冪敤銆傚湪鍏朵粬鎯呭喌涓嬶紝
KVM_PPC_RESIZE_HPT_COMMIT 灏嗚繑鍥為敊璇紙閫氬父鏄?-ENXIO 鎴?-EBUSY锛屼絾濡傛灉鍑嗗宸插紑濮嬩絾澶辫触浜嗭紝
涔熷彲鑳借繑鍥炲叾浠栭敊璇級銆?

濡傛灉瀹㈡埛鏈哄皻鏈娇鑷繁澶勪簬闈欐锛坬uiescent锛夌姸鎬侊紙鍗虫病鏈?vcpu 浼氳繘琛屽惎鐢?MMU 鐨勫唴瀛樿闂級锛岃繖
瀵瑰鎴锋満鐨勫奖鍝嶅皢鏄湭瀹氫箟鐨勩€?

鎴愬姛瀹屾垚鍚庯紝鎸傝捣鐨?HPT 灏嗘垚涓哄鎴锋満鐨勬椿鍔?HPT锛岃€屽厛鍓嶇殑 HPT 灏嗚涓㈠純銆?

澶辫触鏃讹紝瀹㈡埛鏈轰粛灏嗗湪鍏跺厛鍓嶇殑 HPT 涓婅繍琛屻€?

### 4.104 KVM_X86_GET_MCE_CAP_SUPPORTED


:Capability: KVM_CAP_MCE
:Architectures: x86
:Type: system ioctl
:Parameters: u64 mce_cap (out)
:Returns: 0 on success, -1 on error

杩斿洖鍙楁敮鎸佺殑 MCE 鑳藉姏銆倁64 mce_cap 鍙傛暟涓?MSR_IA32_MCG_CAP 瀵勫瓨鍣ㄥ叿鏈夌浉鍚岀殑鏍煎紡銆傚彈鏀寔鐨?
鑳藉姏浼氬皢鍏剁浉搴旂殑浣嶇疆浣嶃€?
### 4.105 KVM_X86_SETUP_MCE


:Capability: KVM_CAP_MCE
:Architectures: x86
:Type: vcpu ioctl
:Parameters: u64 mcg_cap (in)
:Returns: 0 on success,
         -EFAULT if u64 mcg_cap cannot be read,
         -EINVAL if the requested number of banks is invalid,
         -EINVAL if requested MCE capability is not supported.

鍒濆鍖栦互渚涗娇鐢ㄧ殑 MCE 鏀寔銆倁64 mcg_cap 鍙傛暟涓?MSR_IA32_MCG_CAP 瀵勫瓨鍣ㄥ叿鏈夌浉鍚岀殑鏍煎紡锛屽苟鎸囧畾
搴斿惎鐢ㄥ摢浜涜兘鍔涖€傚彈鏀寔鐨勬渶澶ч敊璇姤鍛婏紙error-reporting锛塨ank 鏁伴噺鍙互鍦ㄦ鏌?KVM_CAP_MCE 鏃惰幏鍙栥€?
鍙楁敮鎸佺殑鑳藉姏鍙互閫氳繃 KVM_X86_GET_MCE_CAP_SUPPORTED 鑾峰彇銆?

### 4.106 KVM_X86_SET_MCE


:Capability: KVM_CAP_MCE
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_x86_mce (in)
:Returns: 0 on success,
         -EFAULT if struct kvm_x86_mce cannot be read,
         -EINVAL if the bank number is invalid,
         -EINVAL if VAL bit is not set in status field.

鍚戝鎴锋満娉ㄥ叆涓€涓満鍣ㄦ鏌ラ敊璇紙MCE锛夈€傝緭鍏?
```

  struct kvm_x86_mce {
	__u64 status;
	__u64 addr;
	__u64 misc;
	__u64 mcg_status;
	__u8 bank;
	__u8 pad1[7];
	__u64 pad2[3];
  };

```
濡傛灉鎶ュ憡鐨?MCE 鏄竴涓湭绾犳鐨勯敊璇紙uncorrected error锛夛紝KVM 浼氬皢鍏朵綔涓?MCE 寮傚父娉ㄥ叆瀹㈡埛鏈恒€傚鏋?
瀹㈡埛鏈?MCG_STATUS 瀵勫瓨鍣ㄦ姤鍛?MCE 姝ｅ湪杩涜涓紝KVM 浼氬鑷翠竴涓?KVM_EXIT_SHUTDOWN vmexit銆?

鍚﹀垯锛屽鏋?MCE 鏄竴涓凡绾犳鐨勯敊璇紙corrected error锛夛紝KVM 鍙細灏嗗叾瀛樺偍鍦ㄧ浉搴旂殑 bank 涓紙鍓嶆彁
鏄 bank 娌℃湁鎸佹湁涓€涓厛鍓嶆姤鍛婄殑鏈籂姝ｉ敊璇級銆?

### 4.107 KVM_S390_GET_CMMA_BITS


:Capability: KVM_CAP_S390_CMMA_MIGRATION
:Architectures: s390
:Type: vm ioctl
:Parameters: struct kvm_s390_cmma_log (in, out)
:Returns: 0 on success, a negative value on error

閿欒鐮侊細

  ======     =============================================================
  ENOMEM     鏃犳硶鍒嗛厤瓒冲鐨勫唴瀛樻潵瀹屾垚浠诲姟
  ENXIO      濡傛灉 CMMA 鏈惎鐢?
  EINVAL     濡傛灉鏈缃?KVM_S390_CMMA_PEEK 浣嗚縼绉绘ā寮忔湭鍚敤
  EINVAL     濡傛灉鏈缃?KVM_S390_CMMA_PEEK 浣嗚剰椤佃窡韪凡琚鐢?
             锛堝洜姝よ縼绉绘ā寮忚鑷姩绂佺敤锛?
  EFAULT     濡傛灉鐢ㄦ埛绌洪棿鍦板潃鏃犳晥锛屾垨鍦板潃娌℃湁瀵瑰簲鐨勯〉琛?
             锛堜緥濡備娇鐢ㄥぇ椤垫椂锛夈€?
  ======     =============================================================

璇?ioctl 鐢ㄤ簬鍦?s390 鏋舵瀯涓婅幏鍙?CMMA 浣嶇殑鍊笺€傚畠閫傜敤浜庝袱绉嶅満鏅細

- 鍦ㄥ疄鏃惰縼绉绘湡闂翠繚瀛?CMMA 鍊笺€傚疄鏃惰縼绉婚渶瑕侀€氳繃 KVM_REQ_START_MIGRATION VM 灞炴€у惎鐢ㄣ€?
- 閫氳繃璁剧疆浜嗘爣蹇?KVM_S390_CMMA_PEEK 鏉ラ潪鐮村潖鎬у湴鏌ョ湅 CMMA 鍊笺€?

璇?ioctl 閫氳繃 kvm_s390_cmma_log 缁撴瀯浣撴帴鏀跺弬鏁般€傛墍闇€鐨勫€艰鍐欏叆涓€涓紦鍐插尯锛屽叾浣嶇疆閫氳繃
kvm_s390_cmma_log 缁撴瀯浣撲腑鐨?"values" 鎴愬憳鎸囩ず銆傝緭鍏ョ粨鏋勪綋涓殑鍊间篃浼氭牴鎹渶瑕佹洿鏂般€?

姣忎釜 CMMA 鍊煎崰鐢ㄤ竴涓瓧鑺傘€?

```

  struct kvm_s390_cmma_log {
	__u64 start_gfn;
	__u32 count;
	__u32 flags;
	union {
		__u64 remaining;
		__u64 mask;
	};
	__u64 values;
  };

```
start_gfn 鏄鑾峰彇鍏?CMMA 鍊肩殑绗竴涓鎴锋満甯х殑缂栧彿锛?

count 鏄紦鍐插尯闀垮害鐨勫瓧鑺傛暟锛?

values 鎸囧悜灏嗙粨鏋滃啓鍏ュ叾涓殑缂撳啿鍖恒€?

濡傛灉 count 澶т簬 KVM_S390_SKEYS_MAX锛屽垯琚涓?KVM_S390_SKEYS_MAX銆備负浜嗕笌鍏朵粬 ioctl 淇濇寔涓€鑷达紝
澶嶇敤 KVM_S390_SKEYS_MAX銆?

缁撴灉琚啓鍏?values 瀛楁鎸囧悜鐨勭紦鍐插尯涓紝骞朵笖杈撳叆鍙傛暟鐨勫€兼寜濡備笅鏂瑰紡鏇存柊銆?

鏍规嵁鏍囧織鐨勪笉鍚岋紝浼氭墽琛屼笉鍚岀殑鎿嶄綔銆傚埌鐩墠涓烘鍞竴鍙楁敮鎸佺殑鏍囧織鏄?KVM_S390_CMMA_PEEK銆?

濡傛灉鏈缃?KVM_S390_CMMA_PEEK锛岄粯璁よ涓烘槸锛?
start_gfn 灏嗘寚绀哄叾 CMMA 浣嶄负鑴忕殑绗竴涓〉甯с€傚畠涓嶄竴瀹氫笌浣滀负杈撳叆浼犲叆鐨勭浉鍚岋紝鍥犱负浼氳烦杩囧共鍑€椤点€?

count 灏嗘寚绀虹紦鍐插尯涓疄闄呭啓鍏ョ殑瀛楄妭鏁般€傚畠锛堣€屼笖寰€寰€锛変細灏忎簬杈撳叆鍊硷紝鍥犱负缂撳啿鍖哄彧濉厖鍒版壘鍒?16 瀛楄妭
骞插噣鍊间负姝紙杩欎簺鍊奸殢鍚庝笉浼氳澶嶅埗鍒扮紦鍐插尯涓級銆傜敱浜庝竴涓?CMMA 杩佺Щ鍧楅渶瑕佸熀鍦板潃鍜岄暱搴︼紝鎬诲叡 16 瀛楄妭锛?
鎵€浠ュ彧瑕佸共鍑€鏁版嵁鐨勫ぇ灏忎笉瓒呰繃澶撮儴鐨勫ぇ灏忥紝鎴戜滑灏变細鍦ㄥ悗闈㈡湁涓€浜涜剰鏁版嵁鐨勬儏鍐典笅鍙戝洖涓€浜涘共鍑€鏁版嵁銆傝繖
鍏佽浠ユ洿澶氬湴寰€杩旂敤鎴风┖闂翠负浠ｄ环锛屾渶灏忓寲瑕佷繚瀛樻垨閫氳繃缃戠粶浼犺緭鐨勬暟鎹噺銆俰octl 鐨勪笅涓€娆¤皟鐢ㄥ皢璺宠繃鎵€鏈?
骞插噣鍊硷紝鍙兘鑺傜渷鐨勪笉浠呬粎鏄壘鍒扮殑 16 瀛楄妭銆?

濡傛灉璁剧疆浜?KVM_S390_CMMA_PEEK锛?
鍗充娇涓嶅湪杩佺Щ妯″紡涓嬶紝涔熶細璇诲彇鐜版湁鐨勫瓨鍌ㄥ睘鎬э紝骞朵笖涓嶆墽琛屽叾浠栨搷浣滐紱

杈撳嚭鐨?start_gfn 灏嗙瓑浜庤緭鍏ョ殑 start_gfn锛?

杈撳嚭鐨?count 灏嗙瓑浜庤緭鍏ョ殑 count锛岄櫎闈炲凡鍒拌揪鍐呭瓨鏈熬銆?

鍦ㄨ繖涓ょ鎯呭喌涓嬶細
"remaining" 瀛楁灏嗘寚绀轰粛鐒跺墿浣欑殑鑴?CMMA 鍊肩殑鎬绘暟锛屾垨鑰呭鏋滆缃簡 KVM_S390_CMMA_PEEK 涓旀湭鍚敤
杩佺Щ妯″紡鍒欎负 0銆?

mask 鏈浣跨敤銆?

values 鎸囧悜灏嗗瓨鍌ㄧ粨鏋滅殑鐢ㄦ埛绌洪棿缂撳啿鍖恒€?

### 4.108 KVM_S390_SET_CMMA_BITS


:Capability: KVM_CAP_S390_CMMA_MIGRATION
:Architectures: s390
:Type: vm ioctl
:Parameters: struct kvm_s390_cmma_log (in)
:Returns: 0 on success, a negative value on error

璇?ioctl 鐢ㄤ簬鍦?s390 鏋舵瀯涓婅缃?CMMA 浣嶇殑鍊笺€傚畠鏃ㄥ湪瀹炴椂杩佺Щ鏈熼棿鐢ㄤ簬鎭㈠ CMMA 鍊硷紝浣嗗叾浣跨敤娌℃湁
闄愬埗銆傝 ioctl 閫氳繃 kvm_s390_cmma_values 缁撴瀯浣撴帴鏀跺弬鏁般€傛瘡涓?CMMA 鍊煎崰鐢ㄤ竴涓瓧鑺傘€?

```

  struct kvm_s390_cmma_log {
	__u64 start_gfn;
	__u32 count;
	__u32 flags;
	union {
		__u64 remaining;
		__u64 mask;
 	};
	__u64 values;
  };

```
start_gfn 鎸囩ず璧峰鐨勫鎴锋満甯х紪鍙凤紝

count 鎸囩ず缂撳啿鍖轰腑瑕佽€冭檻澶氬皯涓€硷紝

flags 鏈浣跨敤锛屽繀椤讳负 0銆?

mask 鎸囩ず瑕佽€冭檻鍝簺 PGSTE 浣嶃€?

remaining 鏈浣跨敤銆?

values 鎸囧悜鐢ㄦ埛绌洪棿涓瓨鍌ㄨ繖浜涘€肩殑缂撳啿鍖恒€?

濡傛灉鏃犳硶鍒嗛厤瓒冲鐨勫唴瀛樻潵瀹屾垚浠诲姟锛岃 ioctl 鍙兘浠?-ENOMEM 澶辫触锛涘鏋?CMMA 鏈惎鐢紝浠?-ENXIO
澶辫触锛涘鏋?count 瀛楁杩囧ぇ锛堜緥濡傝秴杩?KVM_S390_CMMA_SIZE_MAX锛夋垨 flags 瀛楁涓嶄负 0锛屼互 -EINVAL
澶辫触锛涘鏋滅敤鎴风┖闂村湴鍧€鏃犳晥銆佸啓鍏ヤ簡鏃犳晥椤碉紙渚嬪鍐呭瓨鏈熬涔嬪悗锛夋垨鍦板潃娌℃湁瀵瑰簲鐨勯〉琛紙渚嬪浣跨敤澶ч〉鏃讹級锛?
浠?-EFAULT 澶辫触銆?

### 4.109 KVM_PPC_GET_CPU_CHAR


:Capability: KVM_CAP_PPC_GET_CPU_CHAR
:Architectures: powerpc
:Type: vm ioctl
:Parameters: struct kvm_ppc_cpu_char (out)
:Returns: 0 on successful completion,
	 -EFAULT if struct kvm_ppc_cpu_char cannot be written

璇?ioctl 鍚戠敤鎴风┖闂存彁渚涙湁鍏?CPU 鏌愪簺鐗规€х殑淇℃伅锛岃繖浜涚壒鎬т笌鎸囦护鐨勬帹娴嬫墽琛屼互鍙婃帹娴嬫墽琛屽彲鑳藉鑷寸殑
淇℃伅娉勬紡鏈夊叧锛堝弬瑙?CVE-2017-5715銆丆VE-2017-5753 鍜?CVE-2017-5754锛夈€備俊鎭綅浜?
```

  struct kvm_ppc_cpu_char {
	__u64	character;		/* characteristics of the CPU */
	__u64	behaviour;		/* recommended software behaviour */
	__u64	character_mask;		/* valid bits in character */
	__u64	behaviour_mask;		/* valid bits in behaviour */
  };

```
涓轰簡鍙墿灞曟€э紝character_mask 鍜?behaviour_mask 瀛楁鎸囩ず character 鍜?behaviour 涓殑鍝簺浣嶅凡鐢?
鍐呮牳濉厖銆傚鏋滃皢鏉ュ畾涔夌殑浣嶉泦鍚堣鎵╁睍锛岀敤鎴风┖闂村皢鑳藉鍒ゆ柇瀹冩槸鍚﹁繍琛屽湪鐭ユ檽鏂颁綅鐨勫唴鏍镐笂銆?

character 瀛楁鎻忚堪鏈夊姪浜庨槻姝㈡棤鎰忎俊鎭硠闇茬殑 CPU 灞炴€?鈥斺€?鍏蜂綋鏉ヨ锛屾槸鍚﹀瓨鍦ㄧ敤浜庡埛鏂板け鏁堬紙flash-invalidate锛?
L1 鏁版嵁缂撳瓨鐨勬寚浠わ紙ori 30,30,0 鎴?mtspr SPRN_TRIG2,rN锛夛紝L1 鏁版嵁缂撳瓨鏄惁璁剧疆涓轰竴绉嶆ā寮忥紙鍏朵腑
鏉＄洰鍙兘鐢卞垱寤哄畠浠殑绾跨▼浣跨敤锛夛紝bcctr[l] 鎸囦护鏄惁鑳介槻姝㈡帹娴嬫墽琛岋紝浠ュ強鏄惁鎻愪緵鎺ㄦ祴灞忛殰鎸囦护
锛坥ri 31,31,0锛夈€?

behaviour 瀛楁鎻忚堪杞欢涓洪槻姝㈡棤鎰忎俊鎭硠闇茶€屽簲閲囧彇鐨勬搷浣滐紝浠庤€屾弿杩扮‖浠跺彈鍝簺婕忔礊褰卞搷锛涘叿浣撴潵璇达紝
浠庡唴鏍歌繑鍥炵敤鎴锋ā寮忔椂鏄惁搴斿埛鏂?L1 鏁版嵁缂撳瓨锛屼互鍙婃槸鍚﹀簲鍦ㄦ暟缁勮竟鐣屾鏌ュ拰鏁扮粍璁块棶涔嬮棿鏀剧疆鎺ㄦ祴灞忛殰銆?

杩欎簺瀛楁浣跨敤涓庢柊鐨?H_GET_CPU_CHARACTERISTICS hypercall 鐩稿悓鐨勪綅瀹氫箟銆?

### 4.110 KVM_MEMORY_ENCRYPT_OP


:Capability: basic
:Architectures: x86
:Type: vm ioctl, vcpu ioctl
:Parameters: an opaque platform specific structure (in/out)
:Returns: 0 on success; -1 on error

濡傛灉骞冲彴鏀寔鍒涘缓鍔犲瘑鐨?VM锛屽垯鍙互浣跨敤姝?ioctl 鍙戝嚭鐗瑰畾浜庡钩鍙扮殑銆佺敤浜庣鐞嗚繖浜涘姞瀵?VM 鐨勫唴瀛樺姞瀵?
鍛戒护銆?

鐩墠锛屾 ioctl 鐢ㄤ簬鍙戝嚭 AMD 澶勭悊鍣ㄤ笂鐨勫畨鍏ㄥ姞瀵嗚櫄鎷熷寲锛圫EV锛夊懡浠ゅ拰 Intel 澶勭悊鍣ㄤ笂鐨勪俊浠诲煙鎵╁睍
锛圱DX锛夊懡浠ゃ€傝缁嗙殑鍛戒护瀹氫箟鍦?Documentation/virt/kvm/x86/amd-memory-encryption.rst 鍜?
Documentation/virt/kvm/x86/intel-tdx.rst 涓€?

### 4.111 KVM_MEMORY_ENCRYPT_REG_REGION


:Capability: basic
:Architectures: x86
:Type: system
:Parameters: struct kvm_enc_region (in)
:Returns: 0 on success; -1 on error

璇?ioctl 鍙敤浜庢敞鍐屼竴涓彲鑳藉寘鍚姞瀵嗘暟鎹殑瀹㈡埛鏈哄唴瀛樺尯鍩燂紙渚嬪瀹㈡埛鏈?RAM銆丼MRAM 绛夛級銆?

瀹冪敤浜庡惎鐢?SEV 鐨勫鎴锋満涓€傚綋鍚敤鍔犲瘑鏃讹紝瀹㈡埛鏈哄唴瀛樺尯鍩熷彲鑳藉寘鍚姞瀵嗘暟鎹€係EV 鍐呭瓨鍔犲瘑寮曟搸浣跨敤
涓€绉嶈皟鏁达紙tweak锛夋満鍒讹紝浣垮緱涓や釜鐩稿悓鐨勬槑鏂囬〉锛屽嵆浣夸綅浜庝笉鍚屼綅缃紝涔熶細鍏锋湁涓嶅悓鐨勫瘑鏂囥€傚洜姝や氦鎹㈡垨
绉诲姩杩欎簺椤电殑瀵嗘枃涓嶄細瀵艰嚧鏄庢枃琚氦鎹€傚洜姝わ紝涓?SEV 瀹㈡埛鏈洪噸瀹氫綅锛堟垨杩佺Щ锛夌墿鐞嗗悗澶囬〉灏嗛渶瑕佷竴浜涢澶?
鐨勬楠ゃ€?

娉ㄦ剰锛氬綋鍓嶇殑 SEV 瀵嗛挜绠＄悊瑙勮寖娌℃湁鎻愪緵浜ゆ崲鎴栬縼绉伙紙绉诲姩锛夊瘑鏂囬〉鐨勫懡浠ゃ€傚洜姝わ紝鐩墠鎴戜滑鍥哄畾锛坧in锛?
閫氳繃姝?ioctl 娉ㄥ唽鐨勫鎴锋満鍐呭瓨鍖哄煙銆?

### 4.112 KVM_MEMORY_ENCRYPT_UNREG_REGION


:Capability: basic
:Architectures: x86
:Type: system
:Parameters: struct kvm_enc_region (in)
:Returns: 0 on success; -1 on error

璇?ioctl 鍙敤浜庢敞閿€涓婅堪閫氳繃 KVM_MEMORY_ENCRYPT_REG_REGION ioctl 娉ㄥ唽鐨勫鎴锋満鍐呭瓨鍖哄煙銆?

### 4.113 KVM_HYPERV_EVENTFD


:Capability: KVM_CAP_HYPERV_EVENTFD
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_hyperv_eventfd (in)

璇?ioctl锛堟敞閿€锛夋敞鍐屼竴涓?eventfd锛屼互閫氳繃 SIGNAL_EVENT hypercall 浠庡鎴锋満鎺ユ敹鍏充簬鎸囧畾 Hyper-V
杩炴帴 id 鐨勯€氱煡锛岃€屼笉浼氬鑷寸敤鎴烽€€鍑恒€傚甫鏈夐潪闆朵簨浠舵爣蹇楀彿锛堜綅 24-31锛夌殑 SIGNAL_EVENT hypercall 浠嶄細
瑙﹀彂 KVM_EXIT_HYPERV_HCALL 鐢ㄦ埛閫€鍑恒€?

```

  struct kvm_hyperv_eventfd {
	__u32 conn_id;
	__s32 fd;
	__u32 flags;
	__u32 padding[3];
  };

```
```

  #define KVM_HYPERV_CONN_ID_MASK		0x00ffffff

```
```

  #define KVM_HYPERV_EVENTFD_DEASSIGN	(1 << 0)

```
:Returns: 0 on success,
 	  -EINVAL if conn_id or flags is outside the allowed range,
	  -ENOENT on deassign if the conn_id isn't registered,
	  -EEXIST on assign if the conn_id is already registered

### 4.114 KVM_GET_NESTED_STATE


:Capability: KVM_CAP_NESTED_STATE
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_nested_state (in/out)
:Returns: 0 on success, -1 on error

閿欒鐮侊細

  =====      =============================================================
  E2BIG     鎬荤姸鎬佸ぇ灏忚秴杩囦簡鐢ㄦ埛鎸囧畾鐨?'size' 鍊硷紱鎵€闇€鐨勫ぇ灏忓皢琚啓鍏?size銆?
  =====      =============================================================

```

  struct kvm_nested_state {
	__u16 flags;
	__u16 format;
	__u32 size;

	union {
		struct kvm_vmx_nested_state_hdr vmx;
		struct kvm_svm_nested_state_hdr svm;

		/* Pad the header to 128 bytes.  */
		__u8 pad[120];
	} hdr;

	union {
		struct kvm_vmx_nested_state_data vmx[0];
		struct kvm_svm_nested_state_data svm[0];
	} data;
  };

  #define KVM_STATE_NESTED_GUEST_MODE		0x00000001
  #define KVM_STATE_NESTED_RUN_PENDING		0x00000002
  #define KVM_STATE_NESTED_EVMCS		0x00000004

  #define KVM_STATE_NESTED_FORMAT_VMX		0
  #define KVM_STATE_NESTED_FORMAT_SVM		1

  #define KVM_STATE_NESTED_VMX_VMCS_SIZE	0x1000

  #define KVM_STATE_NESTED_VMX_SMM_GUEST_MODE	0x00000001
  #define KVM_STATE_NESTED_VMX_SMM_VMXON	0x00000002

  #define KVM_STATE_VMX_PREEMPTION_TIMER_DEADLINE 0x00000001

  struct kvm_vmx_nested_state_hdr {
	__u64 vmxon_pa;
	__u64 vmcs12_pa;

	struct {
		__u16 flags;
	} smm;

	__u32 flags;
	__u64 preemption_timer_deadline;
  };

  struct kvm_vmx_nested_state_data {
	__u8 vmcs12[KVM_STATE_NESTED_VMX_VMCS_SIZE];
	__u8 shadow_vmcs12[KVM_STATE_NESTED_VMX_VMCS_SIZE];
  };

```
璇?ioctl 灏?vcpu 鐨勫祵濂楄櫄鎷熷寲鐘舵€佷粠鍐呮牳澶嶅埗鍒扮敤鎴风┖闂淬€?

鐘舵€佺殑鏈€澶уぇ灏忓彲浠ラ€氳繃鍚?KVM_CHECK_EXTENSION ioctl() 浼犲叆 KVM_CAP_NESTED_STATE 鑾峰彇銆?

### 4.115 KVM_SET_NESTED_STATE


:Capability: KVM_CAP_NESTED_STATE
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_nested_state (in)
:Returns: 0 on success, -1 on error

杩欏皢 vcpu 鐨?kvm_nested_state 缁撴瀯浣撲粠鐢ㄦ埛绌洪棿澶嶅埗鍒板唴鏍搞€傚叧浜?struct kvm_nested_state 鐨勫畾涔夛紝
璇峰弬瑙?KVM_GET_NESTED_STATE銆?

### 4.116 KVM_(UN)REGISTER_COALESCED_MMIO


:Capability: KVM_CAP_COALESCED_MMIO (for coalesced mmio)
	     KVM_CAP_COALESCED_PIO (for coalesced pio)
:Architectures: all
:Type: vm ioctl
:Parameters: struct kvm_coalesced_mmio_zone
:Returns: 0 on success, < 0 on error

鍚堝苟 I/O锛圕oalesced I/O锛夋槸涓€绉嶆€ц兘浼樺寲锛屽畠鎺ㄨ繜纭欢瀵勫瓨鍣ㄥ啓鍏ョ殑妯℃嫙锛屼粠鑰岄伩鍏嶇敤鎴风┖闂撮€€鍑恒€傚畠
閫氬父鐢ㄤ簬鍑忓皯妯℃嫙棰戠箒璁块棶鐨勭‖浠跺瘎瀛樺櫒鐨勫紑閿€銆?

褰撶‖浠跺瘎瀛樺櫒琚厤缃负鍚堝苟 I/O 鏃讹紝鍐欒闂笉浼氶€€鍑哄埌鐢ㄦ埛绌洪棿锛屽叾鍊艰璁板綍鍦ㄤ竴涓唴鏍镐笌鐢ㄦ埛绌洪棿涔嬮棿
鍏变韩鐨勭幆褰㈢紦鍐插尯涓€?

濡傛灉瀵圭‖浠跺瘎瀛樺櫒鐨勪竴娆℃垨澶氭鍐欒闂彲浠ユ帹杩熷埌瀵瑰悓涓€璁惧涓婂彟涓€涓‖浠跺瘎瀛樺櫒鐨勮鎴栧啓锛屽垯浣跨敤鍚堝苟
I/O銆傛渶鍚庝竴娆¤闂皢瀵艰嚧 vmexit锛岀敤鎴风┖闂村皢鍦ㄦā鎷熷畠涔嬪墠澶勭悊鏉ヨ嚜鐜舰缂撳啿鍖虹殑璁块棶銆傝繖灏嗛伩鍏嶅湪閲嶅
鍐欏叆鏃堕€€鍑哄埌鐢ㄦ埛绌洪棿銆?

鍚堝苟 pio 鍩轰簬鍚堝苟 mmio銆傚悎骞?mmio 涓庡悎骞?pio 涔嬮棿鍑犱箮娌℃湁鍖哄埆锛屽彧鏄悎骞?pio 璁板綍瀵?I/O 绔彛鐨?
璁块棶銆?

### 4.117 KVM_CLEAR_DIRTY_LOG


:Capability: KVM_CAP_MANUAL_DIRTY_LOG_PROTECT2
:Architectures: x86, arm64, mips
:Type: vm ioctl
:Parameters: struct kvm_clear_dirty_log (in)
:Returns: 0 on success, -1 on error

```

  /* for KVM_CLEAR_DIRTY_LOG */
  struct kvm_clear_dirty_log {
	__u32 slot;
	__u32 num_pages;
	__u64 first_page;
	union {
		void __user *dirty_bitmap; /* one bit per page */
		__u64 padding;
	};
  };

```
璇?ioctl 鏍规嵁 struct kvm_clear_dirty_log 鐨?dirty_bitmap 瀛楁涓紶鍏ョ殑浣嶅浘锛屾竻闄ゅ唴瀛樻Ы涓〉鐨?
鑴忕姸鎬併€備綅鍥剧殑浣?0 瀵瑰簲浜庡唴瀛樻Ы涓殑椤?"first_page"锛宯um_pages 鏄緭鍏ヤ綅鍥剧殑澶у皬锛堜互浣嶄负鍗曚綅锛夈€?
first_page 蹇呴』鏄?64 鐨勫€嶆暟锛涢櫎闈?first_page + num_pages 绛変簬鍐呭瓨妲界殑澶у皬锛屽惁鍒?num_pages 涔?
蹇呴』鏄?64 鐨勫€嶆暟銆傚浜庤緭鍏ヤ綅鍥句腑姣忎釜琚疆浣嶇殑浣嶏紝鐩稿簲鐨勯〉鍦?KVM 鐨勮剰浣嶅浘涓鏍囪涓?骞插噣"锛屽苟涓?
涓鸿椤甸噸鏂板惎鐢ㄨ剰椤佃窡韪紙渚嬪閫氳繃鍐欎繚鎶わ紝鎴栨竻闄ら〉琛ㄩ」涓殑鑴忎綅锛夈€?

濡傛灉 KVM_CAP_MULTI_ADDRESS_SPACE 鍙敤锛宻lot 瀛楁鐨?16-31 浣嶆寚瀹氳娓呴櫎鑴忕姸鎬佺殑鍦板潃绌洪棿銆傚叧浜?
slot 瀛楁鐨勭敤娉曠粏鑺傦紝璇峰弬瑙?KVM_SET_USER_MEMORY_REGION銆?

褰撳惎鐢ㄤ簡 KVM_CAP_MANUAL_DIRTY_LOG_PROTECT2 鏃讹紝姝?ioctl 鏈€鏈夌敤锛涙洿澶氫俊鎭鍙傝璇ヨ兘鍔涚殑鎻忚堪銆?
浣嗘槸锛屽彧瑕?KVM_CHECK_EXTENSION 纭 KVM_CAP_MANUAL_DIRTY_LOG_PROTECT2 瀛樺湪锛屽畠灏卞彲浠ュ缁堣浣跨敤銆?

### 4.118 KVM_GET_SUPPORTED_HV_CPUID


:Capability: KVM_CAP_HYPERV_CPUID (vcpu), KVM_CAP_SYS_HYPERV_CPUID (system)
:Architectures: x86
:Type: system ioctl, vcpu ioctl
:Parameters: struct kvm_cpuid2 (in/out)
:Returns: 0 on success, -1 on error

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
璇?ioctl 杩斿洖 KVM 涓笌 Hyper-V 妯℃嫙鐩稿叧鐨?x86 cpuid 鐗规€у彾瀛愶紙leaf锛夈€傜敤鎴风┖闂村彲浠ヤ娇鐢ㄨ ioctl
杩斿洖鐨勪俊鎭潵鏋勯€犲憟鐜扮粰浣跨敤 Hyper-V 澧炲己锛坋nlightenment锛夌殑瀹㈡埛鏈猴紙渚嬪 Windows 鎴?Hyper-V 瀹㈡埛鏈猴級
鐨?cpuid 淇℃伅銆?

姝?ioctl 杩斿洖鐨?CPUID 鐗规€у彾瀛愮敱 Hyper-V 椤跺眰鍔熻兘瑙勮寖锛圱LFS锛夊畾涔夈€傝繖浜涘彾瀛愭棤娉曢€氳繃
KVM_GET_SUPPORTED_CPUID ioctl 鑾峰彇锛屽洜涓哄叾涓竴浜涗笌 KVM 鐗规€у彾瀛愶紙0x40000000銆?x40000001锛夌浉浜ゃ€?

鐩墠锛岃繑鍥炰互涓?CPUID 鍙跺瓙鍒楄〃锛?

 - HYPERV_CPUID_VENDOR_AND_MAX_FUNCTIONS
 - HYPERV_CPUID_INTERFACE
 - HYPERV_CPUID_VERSION
 - HYPERV_CPUID_FEATURES
 - HYPERV_CPUID_ENLIGHTMENT_INFO
 - HYPERV_CPUID_IMPLEMENT_LIMITS
 - HYPERV_CPUID_NESTED_FEATURES
 - HYPERV_CPUID_SYNDBG_VENDOR_AND_MAX_FUNCTIONS
 - HYPERV_CPUID_SYNDBG_INTERFACE
 - HYPERV_CPUID_SYNDBG_PLATFORM_CAPABILITIES

鐢ㄦ埛绌洪棿閫氳繃浼犲叆涓€涓?kvm_cpuid2 缁撴瀯浣撴潵璋冪敤 KVM_GET_SUPPORTED_HV_CPUID锛屽叾涓?'nent' 瀛楁鎸囩ず
鍙彉闀挎暟缁?'entries' 涓殑鏉＄洰鏁伴噺銆傚鏋滄潯鐩暟閲忓お灏戣€屾棤娉曟弿杩版墍鏈?Hyper-V 鐗规€у彾瀛愶紝浼氳繑鍥為敊璇?
锛圗2BIG锛夈€傚鏋滄暟閲忓ぇ浜庢垨绛変簬 Hyper-V 鐗规€у彾瀛愮殑鏁伴噺锛?nent' 瀛楁浼氳璋冩暣涓?'entries' 鏁扮粍涓?
鏈夋晥鏉＄洰鐨勬暟閲忥紝骞堕殢鍚庤濉厖銆?

'struct kvm_cpuid_entry2' 涓殑 'index' 鍜?'flags' 瀛楁鐩墠淇濈暀锛岀敤鎴风┖闂翠笉搴旀湡鏈涘湪閭ｉ噷鑾峰緱浠讳綍
鐗瑰畾鍊笺€?

娉ㄦ剰锛孠VM_GET_SUPPORTED_HV_CPUID 鐨?vcpu 鐗堟湰鐩墠宸茶搴熷純銆備笌鏃犳潯浠舵毚闇叉墍鏈夊彈鏀寔鐗规€т綅鐨勭郴缁?
ioctl 涓嶅悓锛寁cpu 鐗堟湰鏈変互涓嬫€紓涔嬪锛?

- HYPERV_CPUID_NESTED_FEATURES 鍙跺瓙鍜?HV_X64_ENLIGHTENED_VMCS_RECOMMENDED 鐗规€т綅浠呭湪鐩稿簲鐨?
  vCPU 鍏堝墠鍚敤浜?Enlightened VMCS锛圞VM_CAP_HYPERV_ENLIGHTENED_VMCS锛夋椂鎵嶄細鏆撮湶銆?
- HV_STIMER_DIRECT_MODE_AVAILABLE 浣嶄粎鍦ㄥ叿鏈夊唴鏍告€?LAPIC 鏃舵墠鏆撮湶銆?
  锛堝亣瀹氬凡璋冪敤 KVM_CREATE_IRQCHIP銆傦級

### 4.119 KVM_ARM_VCPU_FINALIZE


:Architectures: arm64
:Type: vcpu ioctl
:Parameters: int feature (in)
:Returns: 0 on success, -1 on error

閿欒鐮侊細

  ======     ==============================================================
  EPERM      鐗规€ф湭鍚敤銆侀渶瑕侀厤缃紝鎴栧凡缁忓畾绋?
  EINVAL     鐗规€ф湭鐭ユ垨涓嶅瓨鍦?
  ======     ==============================================================

feature 鐨勫凡璇嗗埆鍊硷細

  =====      ===========================================
  arm64      KVM_ARM_VCPU_SVE (requires KVM_CAP_ARM_SVE)
  =====      ===========================================

瀹氱锛坒inalize锛夋寚瀹?vcpu 鐗规€х殑閰嶇疆銆?

vcpu 蹇呴』宸茬粡閫氳繃涓€娆℃垚鍔熺殑 KVM_ARM_VCPU_INIT <KVM_ARM_VCPU_INIT> 璋冪敤锛堝湪 features[] 涓缃簡
鐩稿簲鐨勬爣蹇楋級瀹屾垚浜嗗垵濮嬪寲锛屽惎鐢ㄤ簡鍙楀奖鍝嶇殑鐗规€с€?

瀵逛簬鍙楀奖鍝嶇殑 vcpu 鐗规€э紝杩欐槸鍦?vcpu 瀹屽叏鍙敤涔嬪墠蹇呴』鎵ц鐨勫己鍒舵€ф楠ゃ€?

鍦?KVM_ARM_VCPU_INIT 鍜?KVM_ARM_VCPU_FINALIZE 涔嬮棿锛屽彲浠ラ€氳繃浣跨敤璇稿 KVM_SET_ONE_REG 涔嬬被鐨?
ioctl 鏉ラ厤缃鐗规€с€傚簲鎵ц鐨勭‘鍒囬厤缃互鍙婂浣曟墽琛屾槸鐗规€х浉鍏崇殑銆?

鍏朵粬渚濊禆浜庣壒瀹氱壒鎬ц瀹氱鐨勮皟鐢紝渚嬪 KVM_RUN銆並VM_GET_REG_LIST銆並VM_GET_ONE_REG 鍜?
KVM_SET_ONE_REG锛岄櫎闈炶鐗规€у凡缁忛€氳繃 KVM_ARM_VCPU_FINALIZE 璋冪敤瀹氱锛屽惁鍒欏皢浠?-EPERM 澶辫触銆?

闇€瑕佷娇鐢ㄦ ioctl 瀹氱鐨?vcpu 鐗规€х殑缁嗚妭锛岃鍙傝 KVM_ARM_VCPU_INIT銆?

### 4.120 KVM_SET_PMU_EVENT_FILTER


:Capability: KVM_CAP_PMU_EVENT_FILTER
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_pmu_event_filter (in)
:Returns: 0 on success, -1 on error

閿欒鐮侊細

  ======     ============================================================
  EFAULT     args[^0^] cannot be accessed
  EINVAL     args[^0^] contains invalid data in the filter or filter events
  E2BIG      nevents is too large
  EBUSY      not enough memory to allocate the filter
  ======     ============================================================

```

  struct kvm_pmu_event_filter {
	__u32 action;
	__u32 nevents;
	__u32 fixed_counter_bitmap;
	__u32 flags;
	__u32 pad[4];
	__u64 events[0];
  };

```
璇?ioctl 閫氳繃闄愬埗鍏佽鐨?event select 鍜?unit mask 缁勫悎锛屾潵闄愬埗瀹㈡埛鏈哄彲浠ョ紪绋嬬殑 PMU 浜嬩欢闆嗗悎銆?

鍙傛暟鎸佹湁涓€涓皢琚厑璁告垨鎷掔粷鐨勮繃婊や簨浠跺垪琛ㄣ€?

杩囨护浜嬩欢鍙帶鍒堕€氱敤璁℃暟鍣紱鍥哄畾鐢ㄩ€旇鏁板櫒鐢?fixed_counter_bitmap 鎺у埗銆?

```

```
`0`

瑕佷娇鐢ㄦ妯″紡锛岃娓呯┖ 'flags' 瀛楁銆?

鍦ㄦ妯″紡涓嬶紝姣忎釜浜嬩欢灏嗗寘鍚竴涓?event select + unit mask銆?

褰撳鎴锋満灏濊瘯缂栫▼ PMU 鏃讹紝瀹㈡埛鏈虹殑 event select + unit mask 浼氫笌杩囨护浜嬩欢杩涜姣旇緝锛屼互纭畾瀹㈡埛鏈?
鏄惁搴斿叿鏈夎闂潈闄愩€?

`KVM_PMU_EVENT_FLAG_MASKED_EVENTS`
:Capability: KVM_CAP_PMU_EVENT_MASKED_EVENTS

鍦ㄦ妯″紡涓嬶紝姣忎釜杩囨护浜嬩欢灏嗗寘鍚竴涓?event select銆乵ask銆乵atch 鍜?
```

  KVM_PMU_ENCODE_MASKED_ENTRY()

```
```

  Bits   Description
  ----   -----------
  7:0    event select (low bits)
  15:8   umask match
  31:16  unused
  35:32  event select (high bits)
  36:54  unused
  55     exclude bit
  63:56  umask mask

```
褰撳鎴锋満灏濊瘯缂栫▼ PMU 鏃讹紝鎸変互涓嬫楠ょ‘瀹氬鎴锋満鏄惁搴斿叿鏈夎闂潈闄愶細

 1. 灏嗗鎴锋満鐨?event select 涓庤繃婊や簨浠惰繘琛屽尮閰嶃€?
 2. 濡傛灉鎵惧埌鍖归厤锛屽皢瀹㈡埛鏈虹殑 unit mask 涓庢墍鍖呭惈杩囨护浜嬩欢鐨?mask 鍜?match 鍊艰繘琛屽尮閰嶃€?
    I.e. (unit mask & mask) == match && !exclude銆?
 3. 濡傛灉鎵惧埌鍖归厤锛屽皢瀹㈡埛鏈虹殑 unit mask 涓庢墍鎺掗櫎杩囨护浜嬩欢鐨?mask 鍜?match 鍊艰繘琛屽尮閰嶃€?
    I.e. (unit mask & mask) == match && exclude銆?
 4.
   a. 濡傛灉鎵惧埌鍖呭惈鍖归厤涓旀湭鎵惧埌鎺掗櫎鍖归厤锛屽垯杩囨护璇ヤ簨浠躲€?
   b. 瀵逛簬鎵€鏈夊叾浠栨儏鍐碉紝涓嶈繃婊よ浜嬩欢銆?
 5.
   a. 濡傛灉浜嬩欢琚繃婊や笖瀹冩槸鍏佽鍒楄〃锛屽垯鍏佽瀹㈡埛鏈虹紪绋嬭浜嬩欢銆?
   b. 濡傛灉浜嬩欢琚繃婊や笖瀹冩槸鎷掔粷鍒楄〃锛屽垯涓嶅厑璁稿鎴锋満缂栫▼璇ヤ簨浠躲€?

璁剧疆鏂扮殑 pmu 浜嬩欢杩囨护鍣ㄦ椂锛屽鏋滆缃簡浠讳綍鏈娇鐢ㄥ瓧娈碉紝鎴栬€呭湪 Intel 涓婅皟鐢ㄦ椂璁剧疆浜?event select
涓殑浠讳綍楂樹綅锛?5:32锛夛紝灏嗚繑鍥?-EINVAL銆?

```

  #define KVM_PMU_EVENT_ALLOW 0
  #define KVM_PMU_EVENT_DENY 1

```
閫氳繃姝?API锛孠VM 鐢ㄦ埛绌洪棿杩樺彲浠ラ€氳繃閰嶇疆 "action" 鍜?"fixed_counter_bitmap" 瀛楁鏉ユ帶鍒?VM 鐨?
鍥哄畾璁℃暟鍣ㄧ殑琛屼负锛堝鏋滄湁锛夈€?

鍏蜂綋鏉ヨ锛孠VM 鍦ㄧ‘瀹氭槸鍚?
```

  FixCtr[i]_is_allowed = (action == ALLOW) && (bitmap & BIT(i)) ||
    (action == DENY) && !(bitmap & BIT(i));
  FixCtr[i]_is_denied = !FixCtr[i]_is_allowed;

```
KVM 鎬绘槸浣跨敤 fixed_counter_bitmap锛岀‘淇?fixed_counter_bitmap 璁剧疆姝ｇ‘鏄敤鎴风┖闂寸殑璐ｄ换锛屼緥濡傦紝濡傛灉
鐢ㄦ埛绌洪棿鎯宠瀹氫箟涓€涓彧褰卞搷閫氱敤璁℃暟鍣ㄧ殑杩囨护鍣ㄣ€?

娉ㄦ剰锛?events" 瀛楁涔熼€傜敤浜庡浐瀹氳鏁板櫒鐨勭‖缂栫爜 event_select 鍜?unit_mask 鍊笺€?fixed_counter_bitmap"
鐨勪紭鍏堢骇楂樹簬 "events"锛屽鏋滀袱鑰呬箣闂村瓨鍦ㄧ煕鐩俱€?

### 4.121 KVM_PPC_SVM_OFF


:Capability: basic
:Architectures: powerpc
:Type: vm ioctl
:Parameters: none
:Returns: 0 on successful completion,

閿欒鐮侊細

  ======     ================================================================
  EINVAL     濡傛灉 ultravisor 鏈兘缁堟瀹夊叏瀹㈡埛鏈?
  ENOMEM     濡傛灉 hypervisor 鏈兘涓哄鎴锋満鍒嗛厤鏂扮殑 radix 椤佃〃
  ======     ================================================================

璇?ioctl 鐢ㄤ簬鍏抽棴瀹㈡埛鏈虹殑瀹夊叏妯″紡锛屾垨灏嗗鎴锋満浠庡畨鍏ㄦā寮忚浆鎹㈠埌姝ｅ父妯″紡銆傝繖鍦ㄥ鎴锋満琚噸缃椂璋冪敤銆?
濡傛灉閽堝姝ｅ父瀹㈡埛鏈鸿皟鐢紝鍒欐病鏈夋晥鏋溿€?

璇?ioctl 鍙戝嚭涓€涓?ultravisor 璋冪敤鏉ョ粓姝㈠畨鍏ㄥ鎴锋満锛岃В闄?VPA 椤电殑鍥哄畾锛屽苟閲婃斁鎵€鏈夌敱 hypervisor
鐢ㄤ簬璺熻釜瀹夊叏椤电殑璁惧椤点€?

### 4.122 KVM_S390_NORMAL_RESET


:Capability: KVM_CAP_S390_VCPU_RESETS
:Architectures: s390
:Type: vcpu ioctl
:Parameters: none
:Returns: 0

璇?ioctl 鏍规嵁 POP锛圥rinciples Of Operation锛屾搷浣滃師鐞嗭級涓殑 cpu 閲嶇疆瀹氫箟閲嶇疆 VCPU 瀵勫瓨鍣ㄥ拰鎺у埗缁撴瀯銆?

### 4.123 KVM_S390_INITIAL_RESET


:Capability: basic
:Architectures: s390
:Type: vcpu ioctl
:Parameters: none
:Returns: 0

璇?ioctl 鏍规嵁 POP 涓殑鍒濆 cpu 閲嶇疆瀹氫箟閲嶇疆 VCPU 瀵勫瓨鍣ㄥ拰鎺у埗缁撴瀯銆備絾鏄紝cpu 涓嶄細琚疆浜?ESA 妯″紡銆?
姝ら噸缃槸姝ｅ父閲嶇疆鐨勮秴闆嗐€?

### 4.124 KVM_S390_CLEAR_RESET


:Capability: KVM_CAP_S390_VCPU_RESETS
:Architectures: s390
:Type: vcpu ioctl
:Parameters: none
:Returns: 0

璇?ioctl 鏍规嵁 POP 涓殑娓呴櫎 cpu 閲嶇疆瀹氫箟閲嶇疆 VCPU 瀵勫瓨鍣ㄥ拰鎺у埗缁撴瀯銆備絾鏄紝cpu 涓嶄細琚疆浜?ESA 妯″紡銆?
姝ら噸缃槸鍒濆閲嶇疆鐨勮秴闆嗐€?


### 4.125 KVM_S390_PV_COMMAND


:Capability: KVM_CAP_S390_PROTECTED
:Architectures: s390
:Type: vm ioctl
:Parameters: struct kvm_pv_cmd
:Returns: 0 on success, < 0 on error

```

  struct kvm_pv_cmd {
	__u32 cmd;	/* Command to be executed */
	__u16 rc;	/* Ultravisor return code */
	__u16 rrc;	/* Ultravisor return reason code */
	__u64 data;	/* Data or address */
	__u32 flags;    /* flags for future extensions. Must be 0 for now */
	__u32 reserved[3];
  };

```
**Ultravisor 杩斿洖鐮侊紙Ultravisor return codes锛?*
濡傛灉涓轰簡瀹炵幇鍛戒护棰勬湡鐨勭粨鏋滆€屾墽琛屼簡 Ultravisor 璋冪敤锛屽垯鐢卞唴鏍告彁渚?Ultravisor 杩斿洖锛堝師鍥狅級鐮併€傚洜姝?
瀹冧滑涓?IOCTL 杩斿洖鐮佹棤鍏炽€傚鏋?KVM 鏀瑰彉浜?`rc`锛屽叾鍊煎皢濮嬬粓澶т簬 0锛屽洜姝ゅ缓璁湪鍙戝嚭 PV 鍛戒护涔嬪墠灏嗗叾
璁剧疆涓?0锛屼互渚胯兘澶熸娴嬪埌 `rc` 鐨勫彉鍖栥€?

**cmd 鍊硷細**

KVM_PV_ENABLE
  鍒嗛厤鍐呭瓨骞跺皢 VM 娉ㄥ唽鍒?Ultravisor锛屼粠鑰屽皢鍐呭瓨鎹愯禒缁?Ultravisor锛屼娇鍏舵垚涓?KVM 涓嶅彲璁块棶鐨勩€?
  鎵€鏈夌幇鏈夌殑 CPU 閮借杞崲涓哄彈淇濇姢鐨?CPU銆傚湪姝ゅ懡浠ゆ垚鍔熶箣鍚庯紝浠讳綍閫氳繃鐑彃鎷旀坊鍔犵殑 CPU 鍦ㄥ垱寤烘椂
  涔熶細鍙樻垚鍙椾繚鎶ょ殑銆?

  閿欒鐮侊細

  =====      =============================
  EINTR      瀛樺湪鏈睆钄界殑鎸傝捣淇″彿
  =====      =============================

KVM_PV_DISABLE
  浠?Ultravisor 娉ㄩ攢 VM锛屽苟鍥炴敹鎹愯禒缁?Ultravisor 鐨勫唴瀛橈紝浣垮叾閲嶆柊鍙鍐呮牳浣跨敤銆傛墍鏈夋敞鍐岀殑 VCPU
  閮借杞崲鍥為潪鍙椾繚鎶ょ殑銆傚鏋滃厛鍓嶇殑涓€涓彈淇濇姢 VM 宸茬粡閫氳繃 KVM_PV_ASYNC_CLEANUP_PREPARE 鍑嗗濂借繘琛?
  寮傛鎷嗛櫎锛屽苟涓旈殢鍚庢病鏈夐€氳繃 KVM_PV_ASYNC_CLEANUP_PERFORM 鎷嗛櫎锛屽垯瀹冨皢鍦ㄦ湰娆¤皟鐢ㄤ腑涓庡綋鍓嶇殑鍙椾繚鎶?
  VM 涓€璧疯鎷嗛櫎銆?

KVM_PV_VM_SET_SEC_PARMS
  灏嗛暅鍍忓ご浠?VM 鍐呭瓨浼犻€掔粰 Ultravisor锛屼互鍑嗗闀滃儚鐨勮В鍖呭拰楠岃瘉銆?

KVM_PV_VM_UNPACK
  瑙ｅ寘锛堜繚鎶ゅ拰瑙ｅ瘑锛夊姞瀵嗗惎鍔ㄩ暅鍍忕殑涓€椤点€?

KVM_PV_VM_VERIFY
  楠岃瘉瑙ｅ寘闀滃儚鐨勫畬鏁存€с€傚彧鏈夊畠鎴愬姛锛屾墠鍏佽 KVM 鍚姩鍙椾繚鎶ょ殑 VCPU銆?

KVM_PV_INFO
  :Capability: KVM_CAP_S390_PROTECTED_DUMP

  鎻愪緵涓€涓?API锛岄€氳繃瀛愬懡浠ゅ悜鐢ㄦ埛绌洪棿鎻愪緵 Ultravisor 鐩稿叧鏁版嵁銆俵en_max 鏄敤鎴风┖闂寸紦鍐插尯鐨勫ぇ灏忥紝
  len_written 鏄?KVM 鎸囩ず瀹為檯鍐欏叆璇ョ紦鍐插尯鐨勫瓧鑺傛暟銆傚鏋滃皢鏉ユ坊鍔犳洿澶氬搷搴斿瓧娈碉紝len_written 鍙敤浜?
  纭畾鏈夋晥瀛楁銆?

```

     enum pv_cmd_info_id {
	KVM_PV_INFO_VM,
	KVM_PV_INFO_DUMP,
     };

     struct kvm_s390_pv_info_header {
	__u32 id;
	__u32 len_max;
	__u32 len_written;
	__u32 reserved;
     };

     struct kvm_s390_pv_info {
	struct kvm_s390_pv_info_header header;
	struct kvm_s390_pv_info_dump dump;
	struct kvm_s390_pv_info_vm vm;
     };

```
**瀛愬懡浠わ細**

  KVM_PV_INFO_VM
    姝ゅ瓙鍛戒护涓?PV 瀹夸富鏈烘彁渚涘熀鏈殑 Ultravisor 淇℃伅銆傝繖浜涘€间篃鍙兘浣滀负鏂囦欢瀵煎嚭鍦?sysfs 鍥轰欢 UV
    鏌ヨ鎺ュ彛涓紝浣嗗湪姝?API 涓▼搴忔洿瀹规槗鑾峰彇銆?

    inst_calls 鍜?feature_indication 鎴愬憳鎻愪緵宸插畨瑁呯殑 UV 璋冪敤鍜?UV 鐨勫叾浠栫壒鎬ф寚绀恒€?

    max_* 鎴愬憳鎻愪緵鍏充簬 PV vCPU銆丳V 瀹㈡埛鏈哄拰 PV 瀹㈡埛鏈哄唴瀛樺ぇ灏忔渶澶у€肩殑淇℃伅銆?

```

      struct kvm_s390_pv_info_vm {
	__u64 inst_calls_list[4];
	__u64 max_cpus;
	__u64 max_guests;
	__u64 max_guest_addr;
	__u64 feature_indication;
      };


  KVM_PV_INFO_DUMP
    姝ゅ瓙鍛戒护鎻愪緵涓庤浆鍌?PV 瀹㈡埛鏈虹浉鍏崇殑淇℃伅銆?

    ::

      struct kvm_s390_pv_info_dump {
	__u64 dump_cpu_buffer_len;
	__u64 dump_config_mem_buffer_per_1m;
	__u64 dump_config_finalize_len;
      };

```
KVM_PV_DUMP
  :Capability: KVM_CAP_S390_PROTECTED_DUMP

  鎻愪緵涓€涓?API锛屾彁渚涙湁鍔╀簬杞偍鍙椾繚鎶?VM 鐨勮皟鐢ㄣ€?

```

    struct kvm_s390_pv_dmp {
      __u64 subcmd;
      __u64 buff_addr;
      __u64 buff_len;
      __u64 gaddr;		/* For dump storage state */
    };

  **瀛愬懡浠わ細**

  KVM_PV_DUMP_INIT
    鍒濆鍖栧彈淇濇姢 VM 鐨勮浆鍌ㄨ繃绋嬨€傚鏋滄璋冪敤涓嶆垚鍔燂紝鎵€鏈夊叾浠栧瓙鍛戒护灏嗕互 -EINVAL 澶辫触銆傚鏋?
    杞偍杩囩▼灏氭湭瀹屾垚锛屾瀛愬懡浠ゅ皢杩斿洖 -EINVAL銆?

    骞堕潪鎵€鏈?PV vm 閮藉彲浠ヨ杞偍锛屾墍鏈夎€呴渶瑕佸湪 SE 澶翠腑璁剧疆 `dump allowed` PCF 浣?34 浠ュ厑璁歌浆鍌ㄣ€?

  KVM_PV_DUMP_CONFIG_STOR_STATE
     瀛樺偍 `buff_len` 瀛楄妭鐨勮皟鏁达紙tweak锛夌粍浠跺€硷紝浠庣粷瀵瑰鎴锋満鍦板潃锛坄gaddr`锛夋寚瀹氱殑 1MB 鍧楀紑濮嬨€?
     `buff_len` 闇€瑕佷笌 `conf_dump_storage_state_len` 瀵归綈锛屼笖鑷冲皯 >= dump uv_info 鏁版嵁鎻愪緵鐨?
     `conf_dump_storage_state_len` 鍊笺€傚嵆浣胯繑鍥炰簡閿欒 rc锛宐uff_user 涔熷彲鑳借鍐欏叆銆備緥濡傦紝濡傛灉鎴戜滑
     鍦ㄥ啓鍏ョ涓€椤垫暟鎹悗閬囧埌缂洪〉銆?

  KVM_PV_DUMP_COMPLETE
    濡傛灉瀛愬懡浠ゆ垚鍔燂紝瀹冨皢瀹屾垚杞偍杩囩▼锛屽苟鍏佽鍐嶆璋冪敤 KVM_PV_DUMP_INIT銆?

    鎴愬姛鏃讹紝`conf_dump_finalize_len` 瀛楄妭鐨勫畬鎴愭暟鎹皢琚瓨鍌ㄥ埌 `buff_addr`銆傚畬鎴愭暟鎹寘鍚瘑閽ユ淳鐢?
    绉嶅瓙銆両V銆佽皟鏁撮殢鏈烘暟鍜屽姞瀵嗗瘑閽ワ紝浠ュ強璁よ瘉鏍囩锛屾墍鏈夎繖浜涢兘闇€瑕佸湪浠ュ悗瑙ｅ瘑杞偍鏃朵娇鐢ㄣ€?

```
KVM_PV_ASYNC_CLEANUP_PREPARE
  :Capability: KVM_CAP_S390_PROTECTED_ASYNC_DISABLE

  涓哄綋鍓嶇殑鍙椾繚鎶?VM 鍑嗗寮傛鎷嗛櫎銆傚綋鍓嶅彈淇濇姢 VM 浣跨敤鐨勫ぇ澶氭暟璧勬簮灏嗚鎼佺疆锛屼互渚涘悗缁紓姝ユ媶闄ゃ€傚綋鍓?
  鍙椾繚鎶?VM 闅忓悗灏嗙珛鍗充綔涓洪潪鍙椾繚鎶ょ殑 VM 鎭㈠鎵ц銆備换浣曟椂鍒绘渶澶氬彧鑳芥湁涓€涓彈淇濇姢 VM 琚噯澶囧ソ杩涜
  寮傛鎷嗛櫎銆傚鏋滄煇涓彈淇濇姢 VM 宸茬粡鍑嗗濂芥媶闄わ紝鑰屾病鏈夐殢鍚庤皟鐢?KVM_PV_ASYNC_CLEANUP_PERFORM锛屽垯姝?
  璋冪敤灏嗗け璐ャ€傚湪杩欑鎯呭喌涓嬶紝鐢ㄦ埛绌洪棿杩涚▼搴斿彂鍑轰竴涓甯哥殑 KVM_PV_DISABLE銆傞€氳繃姝よ皟鐢ㄦ悂缃殑璧勬簮
  闇€瑕侀€氳繃鍚庣画璋冪敤 KVM_PV_ASYNC_CLEANUP_PERFORM 鎴?KVM_PV_DISABLE 鏉ユ竻鐞嗭紝鍚﹀垯瀹冧滑灏嗗湪 KVM 缁堟
  鏃惰娓呯悊銆備竴鏃︽竻鐞嗗紑濮嬶紝鍗?KVM_PV_ASYNC_CLEANUP_PERFORM 瀹屾垚涔嬪墠锛屽氨鍙互鍐嶆璋冪敤
  KVM_PV_ASYNC_CLEANUP_PREPARE銆?

KVM_PV_ASYNC_CLEANUP_PERFORM
  :Capability: KVM_CAP_S390_PROTECTED_ASYNC_DISABLE

  鎷嗛櫎鍏堝墠閫氳繃 KVM_PV_ASYNC_CLEANUP_PREPARE 鍑嗗濂芥媶闄ょ殑鍙椾繚鎶?VM銆傛悂缃殑璧勬簮灏嗗湪姝ゅ懡浠ゆ墽琛屾湡闂?
  琚噴鏀俱€傛 PV 鍛戒护鐞嗘兂鎯呭喌涓嬪簲鐢辩敤鎴风┖闂翠粠鍗曠嫭鐨勭嚎绋嬪彂鍑恒€傚鏋滄敹鍒拌嚧鍛戒俊鍙凤紙鎴栬繘绋嬭嚜鐒剁粓姝級锛?
  璇ュ懡浠ゅ皢绔嬪嵆缁堟鑰屼笉瀹屾垚锛屾甯哥殑 KVM 鍏抽棴杩囩▼灏嗚礋璐ｆ竻鐞嗘墍鏈夊墿浣欑殑鍙椾繚鎶?VM锛屽寘鎷偅浜涙媶闄よ
  杩涚▼缁堟涓柇鐨?VM銆?

### 4.126 KVM_XEN_HVM_SET_ATTR


:Capability: KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_SHARED_INFO
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_xen_hvm_attr
:Returns: 0 on success, < 0 on error

```

  struct kvm_xen_hvm_attr {
	__u16 type;
	__u16 pad[3];
	union {
		__u8 long_mode;
		__u8 vector;
		__u8 runstate_update_flag;
		union {
			__u64 gfn;
			__u64 hva;
		} shared_info;
		struct {
			__u32 send_port;
			__u32 type; /* EVTCHNSTAT_ipi / EVTCHNSTAT_interdomain */
			__u32 flags;
			union {
				struct {
					__u32 port;
					__u32 vcpu;
					__u32 priority;
				} port;
				struct {
					__u32 port; /* Zero for eventfd */
					__s32 fd;
				} eventfd;
				__u32 padding[4];
			} deliver;
		} evtchn;
		__u32 xen_version;
		__u64 pad[8];
	} u;
  };

```
type 鍊硷細

KVM_XEN_ATTR_TYPE_LONG_MODE
  灏?VM 鐨?ABI 妯″紡璁剧疆涓?32 浣嶆垨 64 浣嶏紙闀挎ā寮忥級銆傝繖鍐冲畾浜嗘毚闇茬粰 VM 鐨?shared_info 椤电殑甯冨眬銆?

KVM_XEN_ATTR_TYPE_SHARED_INFO
  璁剧疆 Xen shared_info 椤垫墍鍦ㄧ殑瀹㈡埛鏈虹墿鐞嗗抚鍙枫€傛敞鎰忥紝灏界 Xen 灏嗗墠 32 涓?vCPU 鐨?vcpu_info 鏀惧湪
  shared_info 椤典腑锛屼絾 KVM 涓嶄細鑷姩杩欐牱鍋氾紝鑰屾槸瑕佹眰鍗充娇缁欏畾 vCPU 鐨?vcpu_info 浣嶄簬 shared_info
  椤典腑鐨?榛樿"浣嶇疆鏃讹紝涔熻鏄惧紡浣跨敤 KVM_XEN_VCPU_ATTR_TYPE_VCPU_INFO 鎴?
  KVM_XEN_VCPU_ATTR_TYPE_VCPU_INFO_HVA銆傝繖鏄洜涓?KVM 鍙兘涓嶇煡閬撶敤浣?vcpu_info[] 鏁扮粍绱㈠紩鐨?Xen
  CPU id锛屽洜姝ゅ彲鑳戒笉鐭ラ亾姝ｇ‘鐨勯粯璁や綅缃€?

  娉ㄦ剰锛宻hared_info 椤靛彲鑳借 KVM 鎸佺画鍐欏叆锛涢櫎鍏朵粬鍐呭澶栵紝瀹冨寘鍚敤浜庡悜 Xen 瀹㈡埛鏈烘姇閫掍腑鏂殑浜嬩欢閫氶亾
  浣嶅浘銆傚畠鍏嶄簬鑴忛〉璺熻釜鏈哄埗 鈥斺€?姣忔鍚戝鎴锋満鎶曢€掍竴涓簨浠堕€氶亾涓柇鏃讹紝KVM 涓嶄細鏄惧紡灏嗚椤垫爣璁颁负鑴忥紒
  鍥犳锛屽鏋滀换浣?vCPU 涓€鐩村湪杩愯锛屾垨鑰呬换浣曚簨浠堕€氶亾涓柇鍙互琚矾鐢卞埌瀹㈡埛鏈猴紝鐢ㄦ埛绌洪棿搴斿缁堝亣瀹氭寚瀹氱殑
  GFN 鏄剰鐨勩€?

  灏?gfn 璁剧疆涓?KVM_XEN_INVALID_GFN 灏嗙鐢?shared_info 椤点€?

KVM_XEN_ATTR_TYPE_SHARED_INFO_HVA
  濡傛灉鍦?Xen 鑳藉姏涓篃璁剧疆浜?KVM_XEN_HVM_CONFIG_SHARED_INFO_HVA 鏍囧織锛屽垯鍙互浣跨敤姝ゅ睘鎬ф潵璁剧疆
  shared_info 椤垫墍鍦ㄧ殑鐢ㄦ埛绌洪棿鍦板潃锛屾棤璁哄畠鏄犲皠鍦ㄥ鎴锋満鐗╃悊鍦板潃绌洪棿鐨勪綍澶勶紝璇ュ湴鍧€鍦?VMM 涓缁堟槸
  鍥哄畾鐨勩€傚簲浼樺厛浣跨敤姝ゅ睘鎬ц€屼笉鏄?KVM_XEN_ATTR_TYPE_SHARED_INFO锛屽洜涓哄畠閬垮厤鍦ㄩ〉琚噸鏂版槧灏勫埌瀹㈡埛鏈?
  鐗╃悊鍦板潃绌洪棿鏃跺鍐呴儴缂撳瓨杩涜涓嶅繀瑕佺殑澶辨晥銆?

  灏?hva 璁剧疆涓洪浂灏嗙鐢?shared_info 椤点€?

KVM_XEN_ATTR_TYPE_UPCALL_VECTOR
  璁剧疆鐢ㄤ簬鎶曢€?Xen 浜嬩欢閫氶亾 upcall 鐨勫紓甯稿悜閲忋€傝繖鏄敱 hypervisor 鐩存帴娉ㄥ叆鐨勩€乂M 鑼冨洿鐨勫悜閲忥紙涓?
  閫氳繃鏈湴 APIC锛夛紝閫氬父鐢卞鎴锋満閫氳繃 HVM_PARAM_CALLBACK_IRQ 閰嶇疆銆傚彲浠ラ€氳繃灏嗗叾璁剧疆涓洪浂鏉ュ啀娆＄鐢?
  锛堜緥濡傚浜庡鎴锋満 SHUTDOWN_soft_reset锛夈€?

KVM_XEN_ATTR_TYPE_EVTCHN
  褰?KVM_CAP_XEN_HVM ioctl 鎸囩ず鏀寔 KVM_XEN_HVM_CONFIG_EVTCHN_SEND 鐗规€ф椂锛屾灞炴€у彲鐢ㄣ€傚畠閰嶇疆
  涓€涓嚭绔欑鍙ｅ彿锛岀敤浜庢嫤鎴潵鑷鎴锋満鐨?EVTCHNOP_send 璇锋眰銆傜粰瀹氱殑鍙戦€佺鍙ｅ彿鍙互琚畾鍚戝洖瀹㈡埛鏈?
  涓婃寚瀹氱殑 vCPU锛堥€氳繃 APIC ID锛?绔彛/浼樺厛绾э紝鎴栬Е鍙?eventfd 涓婄殑浜嬩欢銆傚彲浠ラ€氳繃鍦ㄥ悗缁皟鐢ㄤ腑璁剧疆
  KVM_XEN_EVTCHN_UPDATE 鏉ユ洿鏀?vCPU 鍜屼紭鍏堢骇锛屼絾瀵逛簬缁欏畾鐨勫彂閫佺鍙ｏ紝鍏朵粬瀛楁涓嶈兘鏇存敼銆傞€氳繃鍦?
  flags 瀛楁涓娇鐢?KVM_XEN_EVTCHN_DEASSIGN 鏉ョЩ闄ょ鍙ｆ槧灏勩€傚湪 flags 瀛楁涓紶鍏?KVM_XEN_EVTCHN_RESET
  浼氱Щ闄ゅ鎵€鏈夊嚭绔欎簨浠堕€氶亾鐨勬嫤鎴€俧lags 瀛楁鐨勫€兼槸浜掓枼鐨勶紝涓嶈兘缁勫悎鎴愪綅鎺╃爜銆?

KVM_XEN_ATTR_TYPE_XEN_VERSION
  褰?KVM_CAP_XEN_HVM ioctl 鎸囩ず鏀寔 KVM_XEN_HVM_CONFIG_EVTCHN_SEND 鐗规€ф椂锛屾灞炴€у彲鐢ㄣ€傚畠閰嶇疆
  瀹㈡埛鏈鸿皟鐢?XENVER_version 鏃惰繑鍥炵殑 32 浣嶇増鏈爜锛涢€氬父鏄紙XEN_MAJOR << 16 | XEN_MINOR锛夈€侾V Xen
  瀹㈡埛鏈洪€氬父浼氫娇鐢ㄥ畠浣滀负铏氭嫙 hypercall 鏉ヨЕ鍙戜簨浠堕€氶亾鎶曢€掞紝鍥犳鍦ㄥ唴鏍镐腑鍝嶅簲鑰屼笉閫€鍑哄埌鐢ㄦ埛绌洪棿鏄?
  鏈夌泭鐨勩€?

KVM_XEN_ATTR_TYPE_RUNSTATE_UPDATE_FLAG
  褰?KVM_CAP_XEN_HVM ioctl 鎸囩ず鏀寔 KVM_XEN_HVM_CONFIG_RUNSTATE_UPDATE_FLAG 鏃讹紝姝ゅ睘鎬у彲鐢ㄣ€傚畠
  鍚敤 XEN_RUNSTATE_UPDATE 鏍囧織锛岃鏍囧織鍏佽瀹㈡埛鏈?vCPU 瀹夊叏鍦拌鍙栧叾浠?vCPU 鐨?vcpu_runstate_info銆?
  Xen 瀹㈡埛鏈洪€氳繃 HYPERVISOR_vm_assist hypercall 鐨?VMASST_TYPE_runstate_update_flag 鏉ュ惎鐢ㄦ鐗规€с€?

### 4.127 KVM_XEN_HVM_GET_ATTR


:Capability: KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_SHARED_INFO
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_xen_hvm_attr
:Returns: 0 on success, < 0 on error

鍏佽璇诲彇 Xen VM 灞炴€с€傚叧浜庣粨鏋勪綋鍜岀被鍨嬶紝璇峰弬瑙佷笂闈㈢殑 KVM_XEN_HVM_SET_ATTR銆侹VM_XEN_ATTR_TYPE_EVTCHN
灞炴€т笉鑳借璇诲彇銆?
### 4.128 KVM_XEN_VCPU_SET_ATTR


:Capability: KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_SHARED_INFO
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_xen_vcpu_attr
:Returns: 0 on success, < 0 on error

```

  struct kvm_xen_vcpu_attr {
	__u16 type;
	__u16 pad[3];
	union {
		__u64 gpa;
		__u64 pad[4];
		struct {
			__u64 state;
			__u64 state_entry_time;
			__u64 time_running;
			__u64 time_runnable;
			__u64 time_blocked;
			__u64 time_offline;
		} runstate;
		__u32 vcpu_id;
		struct {
			__u32 port;
			__u32 priority;
			__u64 expires_ns;
		} timer;
		__u8 vector;
	} u;
  };

```
type 鍊硷細

KVM_XEN_VCPU_ATTR_TYPE_VCPU_INFO
  璁剧疆缁欏畾 vCPU 鐨?vcpu_info 鐨勫鎴锋満鐗╃悊鍦板潃銆備笌 VM 鐨?shared_info 椤典竴鏍凤紝濡傛灉鍚敤浜嗕簨浠堕€氶亾
  涓柇鎶曢€掞紝鐩稿簲椤靛彲鑳介殢鏃惰寮勮剰锛屽洜姝ょ敤鎴风┖闂村簲濮嬬粓鍋囪璇ラ〉鏄剰鐨勶紝鑰屼笉渚濊禆浜庤剰椤佃褰曘€傚皢 gpa
  璁剧疆涓?KVM_XEN_INVALID_GPA 灏嗙鐢?vcpu_info銆?

KVM_XEN_VCPU_ATTR_TYPE_VCPU_INFO_HVA
  濡傛灉鍦?Xen 鑳藉姏涓篃璁剧疆浜?KVM_XEN_HVM_CONFIG_SHARED_INFO_HVA 鏍囧織锛屽垯鍙互浣跨敤姝ゅ睘鎬ф潵璁剧疆
  缁欏畾 vCPU 鐨?vcpu_info 鐨勭敤鎴风┖闂村湴鍧€銆傚畠鍙簲鍦?vcpu_info 浣嶄簬 shared_info 椤典腑鐨?榛樿"浣嶇疆
  鏃朵娇鐢ㄣ€傚湪杩欑鎯呭喌涓嬶紝鍙互瀹夊叏鍦板亣璁剧敤鎴风┖闂村湴鍧€涓嶄細鏀瑰彉锛屽洜涓?shared_info 椤垫槸瀹㈡埛鏈哄唴瀛樹笂鐨?
  涓€涓鐩栧眰锛坥verlay锛夛紝鏃犺瀹冩槧灏勫湪瀹㈡埛鏈虹墿鐞嗗湴鍧€绌洪棿鐨勪綍澶勶紝閮戒繚鎸佸湪鍥哄畾鐨勫涓绘満鍦板潃锛屽洜姝ゅ鏋?
  瀹㈡埛鏈哄唴瀛樺竷灞€琚慨鏀癸紝鍙互閬垮厤瀵瑰唴閮ㄧ紦瀛樿繘琛屼笉蹇呰鐨勫け鏁堛€傚鏋?vcpu_info 涓嶄綅浜?榛樿"浣嶇疆锛屽垯
  涓嶈兘淇濊瘉瀹冧繚鎸佸湪鐩稿悓鐨勫涓绘満鍦板潃锛屽洜姝ら渶瑕佷笂杩扮殑缂撳瓨澶辨晥銆?

KVM_XEN_VCPU_ATTR_TYPE_VCPU_TIME_INFO
  璁剧疆缁欏畾 vCPU 鐨勯澶?pvclock 缁撴瀯鐨勫鎴锋満鐗╃悊鍦板潃銆傝繖閫氬父鐢ㄤ簬瀹㈡埛鏈?vsyscall 鏀寔銆傚皢 gpa 璁剧疆
  涓?KVM_XEN_INVALID_GPA 灏嗙鐢ㄨ缁撴瀯銆?

KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_ADDR
  璁剧疆缁欏畾 vCPU 鐨?vcpu_runstate_info 鐨勫鎴锋満鐗╃悊鍦板潃銆俋en 瀹㈡埛鏈洪€氳繃瀹冩潵璺熻釜 steal time 绛?CPU
  鐘舵€併€傚皢 gpa 璁剧疆涓?KVM_XEN_INVALID_GPA 灏嗙鐢?runstate 鍖哄煙銆?

KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_CURRENT
  浠庣粨鏋勪綋鐨?.u.runstate.state 鎴愬憳璁剧疆缁欏畾 vCPU 鐨?runstate锛圧UNSTATE_running/_runnable/_blocked/
  _offline锛夈€侹VM 鑷姩璁＄畻 running 鍜?runnable 鏃堕棿锛屼絾 blocked 鍜?offline 鐘舵€佸彧鑳芥樉寮忚繘鍏ャ€?

KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_DATA
  浠庣粨鏋勪綋鐨?.u.runstate 鎴愬憳璁剧疆 vCPU runstate 鏁版嵁鐨勬墍鏈夊瓧娈碉紝鍖呮嫭褰撳墠 runstate銆俿tate_entry_time
  蹇呴』绛変簬鍏朵粬鍥涗釜鏃堕棿鐨勬€诲拰銆?

KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_ADJUST
  杩欏皢缁撴瀯浣撶殑 .u.runstate 鎴愬憳鐨勫唴瀹?*鍔?*鍒扮粰瀹?vCPU 鐨?runstate 鏁版嵁鐨勭浉搴旀垚鍛樹笂锛屼粠鑰屽厑璁?
  瀵?runstate 鏃堕棿杩涜鍘熷瓙璋冩暣銆傚 state_entry_time 鐨勮皟鏁村繀椤荤瓑浜庡鍏朵粬鍥涗釜鏃堕棿鐨勮皟鏁翠箣鍜屻€?
  state 瀛楁蹇呴』璁剧疆涓?-1锛屾垨璁剧疆涓烘湁鏁堢殑 runstate 鍊硷紙RUNSTATE_running銆丷UNSTATE_runnable銆?
  RUNSTATE_blocked 鎴?RUNSTATE_offline锛夛紝浠ュ皢褰撳墠璁″叆鐘舵€佽缃负璋冩暣鍚庣殑 state_entry_time 鏃剁殑鐘舵€併€?

KVM_XEN_VCPU_ATTR_TYPE_VCPU_ID
  褰?KVM_CAP_XEN_HVM ioctl 鎸囩ず鏀寔 KVM_XEN_HVM_CONFIG_EVTCHN_SEND 鐗规€ф椂锛屾灞炴€у彲鐢ㄣ€傚畠璁剧疆
  缁欏畾 vCPU 鐨?Xen vCPU ID锛屼互鍏佽涓庡畾鏃跺櫒鐩稿叧鐨?VCPU 鎿嶄綔琚?KVM 鎷︽埅銆?

KVM_XEN_VCPU_ATTR_TYPE_TIMER
  褰?KVM_CAP_XEN_HVM ioctl 鎸囩ず鏀寔 KVM_XEN_HVM_CONFIG_EVTCHN_SEND 鐗规€ф椂锛屾灞炴€у彲鐢ㄣ€傚畠璁剧疆
  璇?vCPU 鐨?VIRQ_TIMER 鐨勪簨浠堕€氶亾绔彛/浼樺厛绾э紝骞跺厑璁镐繚瀛?鎭㈠涓€涓寕璧风殑瀹氭椂鍣ㄣ€傚皢瀹氭椂鍣ㄧ鍙?
  璁剧疆涓洪浂浼氱鐢ㄥ唴鏍稿璇ュ崟娆¤Е鍙戯紙singleshot锛夊畾鏃跺櫒鐨勫鐞嗐€?

KVM_XEN_VCPU_ATTR_TYPE_UPCALL_VECTOR
  褰?KVM_CAP_XEN_HVM ioctl 鎸囩ず鏀寔 KVM_XEN_HVM_CONFIG_EVTCHN_SEND 鐗规€ф椂锛屾灞炴€у彲鐢ㄣ€傚畠璁剧疆
  姣?vCPU 鐨勬湰鍦?APIC upcall 鍚戦噺锛岀敱 Xen 瀹㈡埛鏈洪€氳繃 HVMOP_set_evtchn_upcall_vector hypercall 閰嶇疆銆?
  杩欓€氬父鐢?Windows 瀹㈡埛鏈轰娇鐢紝骞朵笖涓庨€氳繃 HVM_PARAM_CALLBACK_IRQ 閰嶇疆鐨?VM 鑼冨洿鐨?upcall 鍚戦噺涓嶅悓銆?
  閫氳繃灏嗗悜閲忚缃负闆舵潵绂佺敤瀹冦€?


### 4.129 KVM_XEN_VCPU_GET_ATTR


:Capability: KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_SHARED_INFO
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_xen_vcpu_attr
:Returns: 0 on success, -1 on error

鍏佽璇诲彇 Xen vCPU 灞炴€с€傚叧浜庣粨鏋勪綋鍜岀被鍨嬶紝璇峰弬瑙佷笂闈㈢殑 KVM_XEN_VCPU_SET_ATTR銆?

KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_ADJUST 绫诲瀷涓嶈兘涓?KVM_XEN_VCPU_GET_ATTR ioctl 涓€璧蜂娇鐢ㄣ€?

### 4.130 KVM_ARM_MTE_COPY_TAGS


:Capability: KVM_CAP_ARM_MTE
:Architectures: arm64
:Type: vm ioctl
:Parameters: struct kvm_arm_copy_mte_tags
:Returns: number of bytes copied, < 0 on error (-EINVAL for incorrect
          arguments, -EFAULT if memory cannot be accessed).

```

  struct kvm_arm_copy_mte_tags {
	__u64 guest_ipa;
	__u64 length;
	void __user *addr;
	__u64 flags;
	__u64 reserved[2];
  };

```
鍦ㄥ鎴锋満鏍囩鍐呭瓨涔嬮棿澶嶅埗鍐呭瓨鏍囪鎵╁睍锛圡TE锛夋爣绛俱€俙guest_ipa` 鍜?`length` 瀛楁蹇呴』涓?`PAGE_SIZE`
瀵归綈銆俙length` 涓嶅緱澶т簬 2^31 - PAGE_SIZE 瀛楄妭銆俙addr` 瀛楁蹇呴』鎸囧悜涓€涓紦鍐插尯锛屾爣绛惧皢琚鍒惰繘鍑哄叾涓€?

`flags` 鎸囧畾澶嶅埗鐨勬柟鍚戯紝鍙互鏄?`KVM_ARM_TAGS_TO_GUEST` 鎴?`KVM_ARM_TAGS_FROM_GUEST`銆?

鐢ㄤ簬瀛樺偍鏍囩鐨勭紦鍐插尯澶у皬涓?`(length / 16)` 瀛楄妭锛圡TE 涓殑绮掑害涓?16 瀛楄妭锛夈€傛瘡涓瓧鑺傚寘鍚竴涓?
鏍囩鍊笺€傝繖涓?`PTRACE_PEEKMTETAGS` 鍜?`PTRACE_POKEMTETAGS` 鐨勬牸寮忓尮閰嶃€?

濡傛灉鍦ㄥ鍒朵换浣曟暟鎹箣鍓嶅彂鐢熼敊璇紝鍒欒繑鍥炶礋鐨勯敊璇爜銆傚鏋滃湪鍙戠敓閿欒涔嬪墠宸插鍒朵簡涓€浜涙爣绛撅紝鍒欒繑鍥?
鎴愬姛澶嶅埗鐨勫瓧鑺傛暟銆傚鏋滆皟鐢ㄦ垚鍔熷畬鎴愶紝鍒欒繑鍥?`length`銆?

### 4.131 KVM_GET_SREGS2


:Capability: KVM_CAP_SREGS2
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_sregs2 (out)
:Returns: 0 on success, -1 on error

浠?vcpu 璇诲彇鐗规畩瀵勫瓨鍣ㄣ€傛 ioctl锛堝湪鍙楁敮鎸佹椂锛夊彇浠?KVM_GET_SREGS銆?

```

        struct kvm_sregs2 {
                /* out (KVM_GET_SREGS2) / in (KVM_SET_SREGS2) */
                struct kvm_segment cs, ds, es, fs, gs, ss;
                struct kvm_segment tr, ldt;
                struct kvm_dtable gdt, idt;
                __u64 cr0, cr2, cr3, cr4, cr8;
                __u64 efer;
                __u64 apic_base;
                __u64 flags;
                __u64 pdptrs[4];
        };

```
`kvm_sregs2` 鐨?flags 鍊硷細

`KVM_SREGS2_FLAGS_PDPTRS_VALID`

  鎸囩ず缁撴瀯浣撳寘鍚湁鏁堢殑 PDPTR 鍊笺€?


### 4.132 KVM_SET_SREGS2


:Capability: KVM_CAP_SREGS2
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_sregs2 (in)
:Returns: 0 on success, -1 on error

灏嗙壒娈婂瘎瀛樺櫒鍐欏叆 vcpu銆傛暟鎹粨鏋勮鍙傝 KVM_GET_SREGS2銆傛 ioctl锛堝湪鍙楁敮鎸佹椂锛夊彇浠?KVM_SET_SREGS銆?

### 4.133 KVM_GET_STATS_FD


:Capability: KVM_CAP_STATS_BINARY_FD
:Architectures: all
:Type: vm ioctl, vcpu ioctl
:Parameters: none
:Returns: statistics file descriptor on success, < 0 on error

閿欒鐮侊細

  ======     ======================================================
  ENOMEM     濡傛灉鐢变簬鍐呭瓨涓嶈冻鑰屾棤娉曞垱寤?fd
  EMFILE     濡傛灉鎵撳紑鐨勬枃浠舵暟瓒呰繃浜嗛檺鍒?
  ======     ======================================================

杩斿洖鐨勬枃浠舵弿杩扮鍙敤浜庝互浜岃繘鍒舵牸寮忚鍙?VM/vCPU 缁熻鏁版嵁銆傛枃浠舵弿杩扮涓殑鏁版嵁鐢卞洓涓潡缁勬垚锛岀粍缁?
濡備笅锛?

+-------------+
|   Header    |
+-------------+
|  id string  |
+-------------+
| Descriptors |
+-------------+
| Stats Data  |
+-------------+

闄や簡浠庡亸绉?0 寮€濮嬬殑澶撮儴涔嬪锛岃娉ㄦ剰锛屼笉淇濊瘉杩欏洓涓潡鏄浉閭荤殑鎴栨寜涓婅堪椤哄簭鎺掑垪锛沬d銆乨escriptors 鍜?
data 鍧楃殑鍋忕Щ閲忓湪澶撮儴涓壘鍒般€備絾鏄紝鎵€鏈夊洓涓潡閮藉湪鏂囦欢涓寜 64 浣嶅亸绉诲榻愶紝骞朵笖瀹冧滑涓嶉噸鍙犮€?

闄?data 鍧椾箣澶栫殑鎵€鏈夊潡閮芥槸涓嶅彲鍙樼殑銆傜敤鎴风┖闂村湪鑾峰彇鏂囦欢鎻忚堪绗﹀悗鍙兘璇诲彇瀹冧滑涓€娆★紝鐒跺悗浣跨敤 `pread`
鎴?`lseek` 閲嶅璇诲彇缁熻鏁版嵁銆?

鎵€鏈夋暟鎹噰鐢ㄧ郴缁熷瓧鑺傚簭銆?

```

	struct kvm_stats_header {
		__u32 flags;
		__u32 name_size;
		__u32 num_desc;
		__u32 id_offset;
		__u32 desc_offset;
		__u32 data_offset;
	};

```
`flags` 瀛楁鐩墠鏈浣跨敤銆傚畠鎬绘槸琚鍙栦负 0銆?

`name_size` 瀛楁鏄粺璁℃暟鎹悕绉板瓧绗︿覆鐨勫ぇ灏忥紙浠ュ瓧鑺備负鍗曚綅锛屽寘鎷粨灏剧殑 '\0'锛夛紝璇ュ瓧绗︿覆鍖呭惈鍦?
"id string" 鍧椾腑锛屽苟闄勫姞鍦ㄦ瘡涓弿杩扮鐨勬湯灏俱€?

`num_desc` 瀛楁鏄弿杩扮鍧椾腑鍖呭惈鐨勬弿杩扮鏁伴噺銆傦紙data 鍧椾腑鐨勫疄闄呭€兼暟閲忓彲鑳芥洿澶э紝鍥犱负姣忎釜鎻忚堪绗?
鍙兘鍖呭惈澶氫釜鍊硷級銆?

`id_offset` 瀛楁鏄?id 瀛楃涓茬浉瀵逛簬鏂囦欢鎻忚堪绗︽墍鎸囩ず鐨勬枃浠惰捣濮嬩綅缃殑鍋忕Щ閲忋€傚畠鏄?8 鐨勫€嶆暟銆?

`desc_offset` 瀛楁鏄?Descriptors 鍧楃浉瀵逛簬鏂囦欢鎻忚堪绗︽墍鎸囩ず鐨勬枃浠惰捣濮嬩綅缃殑鍋忕Щ閲忋€傚畠鏄?8 鐨勫€嶆暟銆?

`data_offset` 瀛楁鏄?Stats Data 鍧楃浉瀵逛簬鏂囦欢鎻忚堪绗︽墍鎸囩ず鐨勬枃浠惰捣濮嬩綅缃殑鍋忕Щ閲忋€傚畠鏄?8 鐨勫€嶆暟銆?

id 瀛楃涓插潡鍖呭惈涓€涓瓧绗︿覆锛岀敤浜庢爣璇嗚皟鐢?KVM_GET_STATS_FD 鐨勬枃浠舵弿杩扮銆傝鍧楃殑澶у皬锛堝寘鎷粨灏剧殑
`'\0'`锛夌敱澶撮儴涓殑 `name_size` 瀛楁鎸囩ず銆?

鎻忚堪绗﹀潡鍙渶瑕佸湪鏂囦欢鎻忚堪绗︾殑鐢熷懡鍛ㄦ湡鍐呰鍙栦竴娆★紝瀹冨寘鍚竴涓?`struct kvm_stats_desc` 搴忓垪锛屾瘡涓?
鍚庨潰璺熺潃涓€涓ぇ灏忎负 `name_size` 鐨勫瓧绗︿覆銆?
```

	#define KVM_STATS_TYPE_SHIFT		0
	#define KVM_STATS_TYPE_MASK		(0xF << KVM_STATS_TYPE_SHIFT)
	#define KVM_STATS_TYPE_CUMULATIVE	(0x0 << KVM_STATS_TYPE_SHIFT)
	#define KVM_STATS_TYPE_INSTANT		(0x1 << KVM_STATS_TYPE_SHIFT)
	#define KVM_STATS_TYPE_PEAK		(0x2 << KVM_STATS_TYPE_SHIFT)
	#define KVM_STATS_TYPE_LINEAR_HIST	(0x3 << KVM_STATS_TYPE_SHIFT)
	#define KVM_STATS_TYPE_LOG_HIST		(0x4 << KVM_STATS_TYPE_SHIFT)
	#define KVM_STATS_TYPE_MAX		KVM_STATS_TYPE_LOG_HIST

	#define KVM_STATS_UNIT_SHIFT		4
	#define KVM_STATS_UNIT_MASK		(0xF << KVM_STATS_UNIT_SHIFT)
	#define KVM_STATS_UNIT_NONE		(0x0 << KVM_STATS_UNIT_SHIFT)
	#define KVM_STATS_UNIT_BYTES		(0x1 << KVM_STATS_UNIT_SHIFT)
	#define KVM_STATS_UNIT_SECONDS		(0x2 << KVM_STATS_UNIT_SHIFT)
	#define KVM_STATS_UNIT_CYCLES		(0x3 << KVM_STATS_UNIT_SHIFT)
	#define KVM_STATS_UNIT_BOOLEAN		(0x4 << KVM_STATS_UNIT_SHIFT)
	#define KVM_STATS_UNIT_MAX		KVM_STATS_UNIT_BOOLEAN

	#define KVM_STATS_BASE_SHIFT		8
	#define KVM_STATS_BASE_MASK		(0xF << KVM_STATS_BASE_SHIFT)
	#define KVM_STATS_BASE_POW10		(0x0 << KVM_STATS_BASE_SHIFT)
	#define KVM_STATS_BASE_POW2		(0x1 << KVM_STATS_BASE_SHIFT)
	#define KVM_STATS_BASE_MAX		KVM_STATS_BASE_POW2

	struct kvm_stats_desc {
		__u32 flags;
		__s16 exponent;
		__u16 size;
		__u32 offset;
		__u32 bucket_size;
		char name[];
	};

```
`flags` 瀛楁鍖呭惈姝ゆ弿杩扮鎵€鎻忚堪鐨勭粺璁℃暟鎹暟鎹殑绫诲瀷鍜屽崟浣嶃€傚叾瀛楄妭搴忎负 CPU 鍘熺敓瀛楄妭搴忋€傛敮鎸佷互涓?
鏍囧織锛?

`flags` 鐨勪綅 0-3 缂栫爜绫诲瀷锛?

  - `KVM_STATS_TYPE_CUMULATIVE`
    缁熻鎶ュ憡涓€涓疮绉鏁般€傛暟鎹殑鍊煎彧鑳藉鍔犮€侹VM 涓娇鐢ㄧ殑澶у鏁拌鏁板櫒閮芥槸杩欑绫诲瀷銆傝绫诲瀷瀵瑰簲鐨?
    `size` 瀛楁濮嬬粓涓?1銆傛墍鏈夌疮绉粺璁℃暟鎹兘鏄/鍐欑殑銆?
  - `KVM_STATS_TYPE_INSTANT`
    缁熻鎶ュ憡涓€涓灛鏃跺€笺€傚叾鍊煎彲浠ュ鍔犳垨鍑忓皯銆傝繖绉嶇被鍨嬮€氬父鐢ㄤ簬娴嬮噺鏌愪簺璧勬簮锛屼緥濡傝剰椤垫暟銆佸ぇ椤垫暟绛夈€?
    鎵€鏈夌灛鏃剁粺璁￠兘鏄彧璇荤殑銆傝绫诲瀷瀵瑰簲鐨?`size` 瀛楁濮嬬粓涓?1銆?
  - `KVM_STATS_TYPE_PEAK`
    缁熻鏁版嵁鎶ュ憡涓€涓嘲鍊硷紝渚嬪鍝堝笇琛ㄦ《涓殑鏈€澶ч」鏁般€佹渶闀跨殑绛夊緟鏃堕棿绛夈€傛暟鎹殑鍊煎彧鑳藉鍔犮€傝绫诲瀷
    瀵瑰簲鐨?`size` 瀛楁濮嬬粓涓?1銆?
  - `KVM_STATS_TYPE_LINEAR_HIST`
    缁熻鎶ュ憡涓虹嚎鎬х洿鏂瑰浘銆傛《鐨勬暟閲忕敱 `size` 瀛楁鎸囧畾銆傛《鐨勫ぇ灏忕敱 `hist_param` 瀛楁鎸囧畾銆傜 N 涓?
    妗讹紙1 <= N < `size`锛夌殑鑼冨洿鏄?[`hist_param`**(N-1), `hist_param`**N)锛岃€屾渶鍚庝竴涓《鐨勮寖鍥存槸
    [`hist_param`*(`size`-1), +INF)銆傦紙+INF 琛ㄧず姝ｆ棤绌峰€笺€傦級
  - `KVM_STATS_TYPE_LOG_HIST`
    缁熻鎶ュ憡涓哄鏁扮洿鏂瑰浘銆傛《鐨勬暟閲忕敱 `size` 瀛楁鎸囧畾銆傜涓€涓《鐨勮寖鍥存槸 [0, 1)锛岃€屾渶鍚庝竴涓《鐨勮寖鍥?
    鏄?[pow(2, `size`-2), +INF)銆傚惁鍒欙紝绗?N 涓《锛? < N < `size`锛夎鐩?[pow(2, N-2), pow(2, N-1))銆?

`flags` 鐨勪綅 4-7 缂栫爜鍗曚綅锛?

  - `KVM_STATS_UNIT_NONE`
    缁熻鏁版嵁鍊兼病鏈夊崟浣嶃€傝繖閫氬父鎰忓懗鐫€璇ュ€兼槸涓€涓簨浠剁殑绠€鍗曡鏁板櫒銆?
  - `KVM_STATS_UNIT_BYTES`
    瀹冭〃绀虹粺璁℃暟鎹敤浜庢祴閲忓唴瀛樺ぇ灏忥紝鍗曚綅涓?Byte銆並iByte銆丮iByte銆丟iByte 绛夈€傛暟鎹殑鍗曚綅鐢辨弿杩扮涓殑
    `exponent` 瀛楁鍐冲畾銆?
  - `KVM_STATS_UNIT_SECONDS`
    瀹冭〃绀虹粺璁℃暟鎹敤浜庢祴閲忔椂闂存垨寤惰繜銆?
  - `KVM_STATS_UNIT_CYCLES`
    瀹冭〃绀虹粺璁℃暟鎹敤浜庢祴閲?CPU 鏃堕挓鍛ㄦ湡銆?
  - `KVM_STATS_UNIT_BOOLEAN`
    瀹冭〃绀虹粺璁″€煎皢濮嬬粓涓?0 鎴?1銆傚嘲鍊肩被鍨嬬殑甯冨皵缁熻姘歌繙涓嶄細浠?1 鍥炲埌 0銆傚竷灏旂粺璁″彲浠ユ槸绾挎€х洿鏂瑰浘
    锛堟湁涓や釜妗讹級锛屼絾涓嶈兘鏄鏁扮洿鏂瑰浘銆?

娉ㄦ剰锛屽浜庣洿鏂瑰浘锛屽崟浣嶉€傜敤浜庢《鐨勮寖鍥达紝鑰屾《鍊兼寚绀鸿惤鍏ヨ妗惰寖鍥村唴鐨勬牱鏈暟閲忋€?

`flags` 鐨勪綅 8-11 涓?`exponent` 涓€璧风紪鐮佸崟浣嶇殑閲忕骇锛?

  - `KVM_STATS_BASE_POW10`
    閲忕骇鍩轰簬 10 鐨勫箓銆傚畠鐢ㄤ簬娴嬮噺鏃堕棿鍜?CPU 鏃堕挓鍛ㄦ湡銆備緥濡傦紝鎸囨暟 -9 鍙互涓?`KVM_STATS_UNIT_SECONDS`
    涓€璧蜂娇鐢紝琛ㄧず鍗曚綅鏄撼绉掋€?
  - `KVM_STATS_BASE_POW2`
    閲忕骇鍩轰簬 2 鐨勫箓銆傚畠鐢ㄤ簬娴嬮噺鍐呭瓨澶у皬銆備緥濡傦紝鎸囨暟 20 鍙互涓?`KVM_STATS_UNIT_BYTES` 涓€璧蜂娇鐢紝琛ㄧず
    鍗曚綅鏄?MiB銆?

`size` 瀛楁鏄缁熻鏁版嵁鍊肩殑鏁伴噺銆傚浜庡ぇ澶氭暟绠€鍗曠粺璁★紝鍏跺€奸€氬父涓?1銆? 琛ㄧず瀹冨寘鍚竴涓棤绗﹀彿 64
浣嶆暟鎹€?

`offset` 瀛楁鏄粠 Data Block 璧峰浣嶇疆鍒扮浉搴旂粺璁℃暟鎹捣濮嬩綅缃殑鍋忕Щ閲忋€?

`bucket_size` 瀛楁鐢ㄤ綔鐩存柟鍥剧粺璁℃暟鎹殑鍙傛暟銆傚畠浠呯敱绾挎€х洿鏂瑰浘缁熻鏁版嵁浣跨敤锛屾寚瀹氫竴涓《鐨勫ぇ灏忥紝鍗曚綅
鐢?`flags` 鐨勪綅 4-11 涓?`exponent` 涓€璧疯〃绀恒€?

`name` 瀛楁鏄粺璁℃暟鎹殑鍚嶇О瀛楃涓层€傚悕绉板瓧绗︿覆浠?`struct kvm_stats_desc` 鐨勬湯灏惧紑濮嬨€傚寘鎷粨灏?
`'\0'` 鍦ㄥ唴鐨勬渶澶ч暱搴︾敱澶撮儴涓殑 `name_size` 鎸囩ず銆?

Stats Data 鍧楀寘鍚竴涓?64 浣嶅€兼暟缁勶紝椤哄簭涓?Descriptors 鍧椾腑鐨勬弿杩扮鐩稿悓銆?

### 4.134 KVM_GET_XSAVE2


:Capability: KVM_CAP_XSAVE2
:Architectures: x86
:Type: vcpu ioctl
:Parameters: struct kvm_xsave (out)
:Returns: 0 on success, -1 on error


```

  struct kvm_xsave {
	__u32 region[1024];
	__u32 extra[0];
  };

```
璇?ioctl 浼氬皢褰撳墠 vcpu 鐨?xsave 缁撴瀯浣撳鍒跺埌鐢ㄦ埛绌洪棿銆傚畠澶嶅埗鐨勫瓧鑺傛暟绛変簬 KVM_CHECK_EXTENSION(KVM_CAP_XSAVE2)
鍦?vm 鏂囦欢鎻忚堪绗︿笂璋冪敤鏃惰繑鍥炵殑鍊笺€侹VM_CHECK_EXTENSION(KVM_CAP_XSAVE2) 杩斿洖鐨勫ぇ灏忓€兼€绘槸鑷冲皯涓?4096銆?
鐩墠锛屽彧鏈夊綋鏌愪釜鍔ㄦ€佺壒鎬у凡閫氳繃 `arch_prctl()` 鍚敤鏃跺畠鎵嶅ぇ浜?4096锛屼絾杩欏湪鏈潵鍙兘浼氭敼鍙樸€?

struct kvm_xsave 涓悇鐘舵€佷繚瀛樺尯鍩熺殑鍋忕Щ閲忛伒寰涓绘満涓?CPUID 鍙跺瓙 0xD 鐨勫唴瀹广€?

### 4.135 KVM_XEN_HVM_EVTCHN_SEND


:Capability: KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_EVTCHN_SEND
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_irq_routing_xen_evtchn
:Returns: 0 on success, < 0 on error


```

   struct kvm_irq_routing_xen_evtchn {
	__u32 port;
	__u32 vcpu;
	__u32 priority;
   };

```
璇?ioctl 灏嗕簨浠堕€氶亾涓柇鐩存帴娉ㄥ叆瀹㈡埛鏈?vCPU銆?

### 4.136 KVM_S390_PV_CPU_COMMAND


:Capability: KVM_CAP_S390_PROTECTED_DUMP
:Architectures: s390
:Type: vcpu ioctl
:Parameters: none
:Returns: 0 on success, < 0 on error

璇?ioctl 涓?`KVM_S390_PV_COMMAND` 闈炲父鐩镐技锛屼絾澶勭悊閽堝 vcpu 鐨勮姹傘€傚畠澶嶇敤浜?kvm_s390_pv_dmp
缁撴瀯浣擄紝鍥犳涔熷叡浜懡浠?id銆?

**command锛?*

KVM_PV_DUMP
  鎻愪緵涓€涓?API锛屾彁渚涙湁鍔╀簬杞偍鍙椾繚鎶?VM 鐨?vcpu 鐨勮皟鐢ㄣ€?

**subcommand锛?*

KVM_PV_DUMP_CPU
  鎻愪緵鍔犲瘑鐨勮浆鍌ㄦ暟鎹紝濡傚瘎瀛樺櫒鍊笺€傝繑鍥炴暟鎹殑闀垮害鐢?uv_info.guest_cpu_stor_len 鎻愪緵銆?

### 4.137 KVM_S390_ZPCI_OP


:Capability: KVM_CAP_S390_ZPCI_OP
:Architectures: s390
:Type: vm ioctl
:Parameters: struct kvm_s390_zpci_op (in)
:Returns: 0 on success, <0 on error

鐢ㄤ簬绠＄悊 zPCI 璁惧鐨勭‖浠惰緟鍔╄櫄鎷熷寲鐗规€с€?

```

  struct kvm_s390_zpci_op {
	/* in */
	__u32 fh;		/* target device */
	__u8  op;		/* operation to perform */
	__u8  pad[3];
	union {
		/* for KVM_S390_ZPCIOP_REG_AEN */
		struct {
			__u64 ibv;	/* Guest addr of interrupt bit vector */
			__u64 sb;	/* Guest addr of summary bit */
			__u32 flags;
			__u32 noi;	/* Number of interrupts */
			__u8 isc;	/* Guest interrupt subclass */
			__u8 sbo;	/* Offset of guest summary bit vector */
			__u16 pad;
		} reg_aen;
		__u64 reserved[8];
	} u;
  };

```
鎿嶄綔绫诲瀷鍦?"op" 瀛楁涓寚瀹氥€侹VM_S390_ZPCIOP_REG_AEN 鐢ㄤ簬涓?VM 娉ㄥ唽閫傞厤鍣ㄤ簨浠堕€氱煡瑙ｉ噴锛坅dapter
event notification interpretation锛夛紝杩欏皢鍏佽鍥轰欢鐩存帴灏嗛€傞厤鍣ㄤ簨浠舵姇閫掑埌 vm锛岀敱 KVM 鎻愪緵澶囦唤鎶曢€?
鏈哄埗锛汯VM_S390_ZPCIOP_DEREG_AEN 鐢ㄤ簬闅忓悗绂佺敤閫傞厤鍣ㄤ簨浠堕€氱煡鐨勮В閲娿€?

鐩爣 zPCI 鍔熻兘涔熷繀椤婚€氳繃 "fh" 瀛楁鎸囧畾銆傚浜?KVM_S390_ZPCIOP_REG_AEN 鎿嶄綔锛屽繀椤婚€氳繃 "reg_aen"
缁撴瀯浣撴彁渚涘缓绔嬪浐浠舵姇閫掓墍闇€鐨勯澶栦俊鎭€?

"pad" 鍜?"reserved" 瀛楁鍙敤浜庢湭鏉ョ殑鎵╁睍锛岀敤鎴风┖闂村簲灏嗗叾璁剧疆涓?0銆?

### 4.138 KVM_ARM_SET_COUNTER_OFFSET


:Capability: KVM_CAP_COUNTER_OFFSET
:Architectures: arm64
:Type: vm ioctl
:Parameters: struct kvm_arm_counter_offset (in)
:Returns: 0 on success, < 0 on error

璇ヨ兘鍔涙寚绀虹敤鎴风┖闂磋兘澶熶娇鐢?KVM_ARM_SET_CNT_OFFSET ioctl 浠ュ強浠ヤ笅鏁版嵁缁撴瀯锛屽皢鍗曚竴 VM 鑼冨洿鐨勫亸绉?
搴旂敤鍒板鎴锋満鎵€瑙佺殑铏氭嫙璁℃暟鍣ㄥ拰鐗╃悊璁℃暟鍣細

```

	struct kvm_arm_counter_offset {
		__u64 counter_offset;
		__u64 reserved;
	};

```
璇ュ亸绉绘弿杩颁簡浠庤櫄鎷熷拰鐗╃悊璁℃暟鍣ㄨ鍥句腑鍑忓幓鐨勮鏁板櫒鍛ㄦ湡鏁帮紙绫讳技浜?CNTVOFF_EL2 鍜?CNTPOFF_EL2 绯荤粺
瀵勫瓨鍣ㄧ殑鏁堟灉锛屼絾浠呭叏灞€鐢熸晥锛夈€傝鍋忕Щ濮嬬粓搴旂敤浜庢 VM 鐨勬墍鏈?vcpu锛堝凡鍒涘缓鎴栧湪璋冪敤姝?ioctl 涔嬪悗
鍒涘缓鐨勶級銆?

璁＄畻鍋忕Щ鏄敤鎴风┖闂寸殑璐ｄ换锛屼緥濡傚熀浜庡鎴锋満璁℃暟鍣ㄧ殑鍏堝墠鍊笺€?

"reserved" 瀛楁鐨勪换浣曢潪 0 鍊奸兘鍙兘瀵艰嚧杩斿洖閿欒锛?EINVAL锛夈€傚鏋滃悓鏃跺彂鍑轰簡浠讳綍 vcpu ioctl锛屾
ioctl 涔熷彲鑳借繑鍥?-EBUSY銆?

娉ㄦ剰锛屼娇鐢ㄦ ioctl 浼氬鑷?KVM 蹇界暐闅忓悗鐢ㄦ埛绌洪棿浣跨敤 SET_ONE_REG 鎺ュ彛瀵?CNTVCT_EL0 鍜?CNTPCT_EL0
瀵勫瓨鍣ㄧ殑鍐欏叆銆備笉浼氳繑鍥為敊璇紝浣嗙粨鏋滃亸绉讳笉浼氳搴旂敤銆?


### 4.139 KVM_ARM_GET_REG_WRITABLE_MASKS


:Capability: KVM_CAP_ARM_SUPPORTED_REG_MASK_RANGES
:Architectures: arm64
:Type: vm ioctl
:Parameters: struct reg_mask_range (in/out)
:Returns: 0 on success, < 0 on error


```

        #define KVM_ARM_FEATURE_ID_RANGE	0
        #define KVM_ARM_FEATURE_ID_RANGE_SIZE	(3 * 8 * 8)

        struct reg_mask_range {
                __u64 addr;             /* Pointer to mask array */
                __u32 range;            /* Requested range */
                __u32 reserved[13];
        };

```
璇?ioctl 灏嗘墍閫夊瘎瀛樺櫒鑼冨洿鐨?writable 鎺╃爜澶嶅埗鍒扮敤鎴风┖闂淬€?

`addr` 瀛楁鏄寚鍚戠洰鏍囨暟缁勭殑鎸囬拡锛孠VM 灏?writable 鎺╃爜澶嶅埗鍒伴偅閲屻€?

`range` 瀛楁鎸囩ず璇锋眰鐨勫瘎瀛樺櫒鑼冨洿銆俙KVM_CHECK_EXTENSION` 瀵?`KVM_CAP_ARM_SUPPORTED_REG_MASK_RANGES`
鑳藉姏鐨勬煡璇㈣繑鍥炲彈鏀寔鐨勮寖鍥达紝琛ㄧず涓轰竴缁勬爣蹇椼€傛瘡涓爣蹇楃殑浣嶇储寮曚唬琛?`range` 瀛楁鐨勪竴涓彲鑳藉€笺€傛墍鏈?
鍏朵粬鍊间繚鐣欎緵灏嗘潵浣跨敤锛孠VM 鍙兘杩斿洖閿欒銆?

`reserved[^13^]` 鏁扮粍淇濈暀渚涘皢鏉ヤ娇鐢紝搴斾负 0锛屽惁鍒?KVM 鍙兘杩斿洖閿欒銆?

##### KVM_ARM_FEATURE_ID_RANGE (0)


Feature ID 鑼冨洿瀹氫箟涓?AArch64 绯荤粺瀵勫瓨鍣ㄧ┖闂达紝鍏朵腑 op0==3銆乷p1=={0, 1, 3}銆丆Rn==0銆丆Rn=={0-7}銆?
op2=={0-7}銆?

`addr` 鎸囧悜鐨勮繑鍥炴帺鐮佹暟缁勭敱瀹?`ARM64_FEATURE_ID_RANGE_IDX(op0, op1, crn, crm, op2)` 绱㈠紩锛屼娇
鐢ㄦ埛绌洪棿鑳藉鐭ラ亾 `op0, op1, crn, crm, op2` 鎵€鎻忚堪鐨勭郴缁熷瘎瀛樺櫒鍙互鏇存敼鍝簺瀛楁銆侹VM 浼氭嫆缁濇弿杩?
绯荤粺鎵€鏀寔鐗规€ц秴闆嗙殑 ID 瀵勫瓨鍣ㄥ€笺€?

### 4.140 KVM_SET_USER_MEMORY_REGION2


:Capability: KVM_CAP_USER_MEMORY2
:Architectures: all
:Type: vm ioctl
:Parameters: struct kvm_userspace_memory_region2 (in)
:Returns: 0 on success, -1 on error

KVM_SET_USER_MEMORY_REGION2 鏄?KVM_SET_USER_MEMORY_REGION 鐨勬墿灞曪紝鍏佽灏?guest_memfd 鍐呭瓨鏄犲皠鍒?
瀹㈡埛鏈恒€傛墍鏈変笌 KVM_SET_USER_MEMORY_REGION 鍏变韩鐨勫瓧娈甸兘瀹屽叏鐩稿悓銆傜敤鎴风┖闂村彲浠ュ湪 flags 涓缃?
KVM_MEM_GUEST_MEMFD锛岃 KVM 灏嗗唴瀛樺尯鍩熺粦瀹氬埌缁欏畾鐨?guest_memfd 鑼冨洿
[guest_memfd_offset, guest_memfd_offset + memory_size]銆傜洰鏍?guest_memfd 蹇呴』鎸囧悜閫氳繃褰撳墠 VM 涓婄殑
KVM_CREATE_GUEST_MEMFD 鍒涘缓鐨勬枃浠讹紝涓旂洰鏍囪寖鍥翠笉寰楃粦瀹氬埌浠讳綍鍏朵粬鍐呭瓨鍖哄煙銆傛墍鏈夋爣鍑嗙殑杈圭晫妫€鏌ラ兘
閫傜敤锛堣杩愮敤甯歌瘑锛夈€?

```

  struct kvm_userspace_memory_region2 {
	__u32 slot;
	__u32 flags;
	__u64 guest_phys_addr;
	__u64 memory_size; /* bytes */
	__u64 userspace_addr; /* start of the userspace allocated memory */
	__u64 guest_memfd_offset;
	__u32 guest_memfd;
	__u32 pad1;
	__u64 pad2[14];
  };

```
KVM_MEM_GUEST_MEMFD 鍖哄煙_蹇呴』_鏈変竴涓湁鏁堢殑 guest_memfd锛堢鏈夊唴瀛橈級鍜?userspace_addr锛堝叡浜唴瀛橈級銆?
浣嗘槸锛屽浜?userspace_addr 鏉ヨ锛?鏈夋晥"浠呬粎鎰忓懗鐫€鍦板潃鏈韩蹇呴』鏄竴涓悎娉曠殑鐨勭敤鎴风┖闂村湴鍧€銆倁serspace_addr
鐨勫悗澶囨槧灏勪笉闇€瑕佸湪 KVM_SET_USER_MEMORY_REGION2 鏃舵湁鏁?宸插～鍏咃紝渚嬪鍏变韩鍐呭瓨鍙互鎸夐渶鎯版€ф槧灏?鍒嗛厤銆?

褰撳皢 gfn 鏄犲皠鍒板鎴锋満鏃讹紝KVM 鏍规嵁 gfn 鐨?KVM_MEMORY_ATTRIBUTE_PRIVATE 鐘舵€侀€夋嫨鍏变韩杩樻槸绉佹湁锛屽嵆
浣跨敤 userspace_addr 杩樻槸 guest_memfd銆傚湪鍒涘缓 VM 鏃讹紝鎵€鏈夊唴瀛橀兘鏄叡浜殑锛屽嵆鎵€鏈?gfn 鐨?PRIVATE
灞炴€т负 '0'銆傜敤鎴风┖闂村彲浠ラ€氳繃鎸夐渶閫氳繃 KVM_SET_MEMORY_ATTRIBUTES 鍒囨崲 KVM_MEMORY_ATTRIBUTE_PRIVATE
鏉ユ帶鍒跺唴瀛樻槸鍏变韩杩樻槸绉佹湁銆?

##### S390锛?


濡傛灉 VM 璁剧疆浜?KVM_VM_S390_UCONTROL 鏍囧織锛屽垯杩斿洖 -EINVAL銆?
濡傛灉鏄湪鍙椾繚鎶ょ殑 VM 涓婅皟鐢紝鍒欒繑鍥?-EINVAL銆?

### 4.141 KVM_SET_MEMORY_ATTRIBUTES


:Capability: KVM_CAP_MEMORY_ATTRIBUTES
:Architectures: x86
:Type: vm ioctl
:Parameters: struct kvm_memory_attributes (in)
:Returns: 0 on success, <0 on error

KVM_SET_MEMORY_ATTRIBUTES 鍏佽鐢ㄦ埛绌洪棿涓轰竴娈靛鎴锋満鐗╃悊鍐呭瓨璁剧疆鍐呭瓨灞炴€с€?

```

  struct kvm_memory_attributes {
	__u64 address;
	__u64 size;
	__u64 attributes;
	__u64 flags;
  };

  #define KVM_MEMORY_ATTRIBUTE_PRIVATE           (1ULL << 3)

```
address 鍜?size 蹇呴』涓庨〉瀵归綈銆傚彈鏀寔鐨勫睘鎬у彲浠ラ€氳繃鍦?KVM_CAP_MEMORY_ATTRIBUTES 涓婅皟鐢?
ioctl(KVM_CHECK_EXTENSION) 鑾峰彇銆傚鏋滃湪 VM 涓婃墽琛岋紝KVM_CAP_MEMORY_ATTRIBUTES 绮剧‘杩斿洖璇?VM 鏀寔鐨?
灞炴€с€傚鏋滃湪绯荤粺鑼冨洿鎵ц锛孠VM_CAP_MEMORY_ATTRIBUTES 杩斿洖 KVM 鏀寔鐨勬墍鏈夊睘鎬с€傜洰鍓嶅畾涔夌殑鍞竴灞炴€ф槸
KVM_MEMORY_ATTRIBUTE_PRIVATE锛屽畠灏嗙浉鍏崇殑 gfn 鏍囪涓哄鏈虹鏈夊唴瀛樸€?

娉ㄦ剰锛屾病鏈?get" API銆傜敤鎴风┖闂磋礋璐ｆ牴鎹渶瑕佹樉寮忚窡韪?gfn/椤电殑鐘舵€併€?

"flags" 瀛楁淇濈暀渚涘皢鏉ユ墿灞曪紝蹇呴』涓?'0'銆?

### 4.142 KVM_CREATE_GUEST_MEMFD


:Capability: KVM_CAP_GUEST_MEMFD
:Architectures: none
:Type: vm ioctl
:Parameters: struct kvm_create_guest_memfd(in)
:Returns: A file descriptor on success, <0 on error

KVM_CREATE_GUEST_MEMFD 鍒涘缓涓€涓尶鍚嶆枃浠讹紝骞惰繑鍥炰竴涓紩鐢ㄥ畠鐨勬枃浠舵弿杩扮銆俫uest_memfd 鏂囦欢澶ц嚧绫讳技浜?
閫氳繃 memfd_create() 鍒涘缓鐨勬枃浠讹紝渚嬪锛実uest_memfd 鏂囦欢椹荤暀鍦?RAM 涓紝鍏锋湁鏄撳け鎬у瓨鍌紝骞跺湪鏈€鍚庝竴涓?
寮曠敤琚噴鏀炬椂鑷姩閲婃斁銆備笌"甯歌" memfd_create() 鏂囦欢涓嶅悓锛実uest_memfd 鏂囦欢缁戝畾鍒板叾鎷ユ湁鐨勮櫄鎷熸満
锛堣涓嬫枃锛夛紝涓嶈兘琚敤鎴风┖闂存槧灏勩€佽鍙栨垨鍐欏叆锛屽苟涓斾笉鑳借皟鏁村ぇ灏忥紙涓嶈繃 guest_memfd 鏂囦欢鏀寔
PUNCH_HOLE锛夈€?

```

  struct kvm_create_guest_memfd {
	__u64 size;
	__u64 flags;
	__u64 reserved[6];
  };

```
浠庢蹇典笂璁诧紝鏀拺 guest_memfd 鏂囦欢鐨?inode 浠ｈ〃鐗╃悊鍐呭瓨锛屽嵆涓庤櫄鎷熸満浣滀负涓€涓簨鐗╄€﹀悎锛岃€屼笉鏄笌
"struct kvm" 鑰﹀悎銆傛枃浠舵湰韬粦瀹氬埌 "struct kvm"锛屾槸璇ュ疄渚嬪搴曞眰鍐呭瓨鐨勮鍥撅紝渚嬪鏈夋晥鍦版彁渚涘鎴锋満
鍦板潃鍒板涓绘満鍐呭瓨鐨勮浆鎹€傝繖鍏佽杩欐牱鐨勭敤渚嬶細澶氫釜 KVM 缁撴瀯鐢ㄤ簬绠＄悊鍗曚釜铏氭嫙鏈猴紝渚嬪鍦ㄦ墽琛岃櫄鎷熸満鐨?
瀹夸富鏈哄唴锛坕ntrahost锛夎縼绉绘椂銆?

KVM 鐩墠浠呮敮鎸侀€氳繃 KVM_SET_USER_MEMORY_REGION2 鏄犲皠 guest_memfd锛屾洿鍏蜂綋鍦拌锛岄€氳繃
"struct kvm_userspace_memory_region2" 涓殑 guest_memfd 鍜?guest_memfd_offset 瀛楁锛屽叾涓?
guest_memfd_offset 鏄繘鍏?guest_memfd 瀹炰緥鐨勫亸绉婚噺銆傚浜庣粰瀹氱殑 guest_memfd 鏂囦欢锛屾瘡椤垫渶澶氭湁涓€涓?
鏄犲皠锛屽嵆涓嶅厑璁稿皢澶氫釜鍐呭瓨鍖哄煙缁戝畾鍒板崟涓?guest_memfd 鑼冨洿锛堜换浣曟暟閲忕殑鍐呭瓨鍖哄煙閮藉彲浠ョ粦瀹氬埌鍗曚釜
guest_memfd 鏂囦欢锛屼絾缁戝畾鐨勮寖鍥翠笉寰楅噸鍙狅級銆?

鑳藉姏 KVM_CAP_GUEST_MEMFD_FLAGS 鏋氫妇浜嗗彲閫氳繃 KVM_CREATE_GUEST_MEMFD 鎸囧畾鐨?`flags`銆傚綋鍓嶅畾涔夌殑鏍囧織锛?

  ============================ ================================================
  GUEST_MEMFD_FLAG_MMAP        鍚敤鍦?guest_memfd 鏂囦欢鎻忚堪绗︿笂浣跨敤 mmap()銆?
  GUEST_MEMFD_FLAG_INIT_SHARED 鍦?KVM_CREATE_GUEST_MEMFD 鏈熼棿浣挎枃浠朵腑鐨勬墍鏈夊唴瀛樹负鍏变韩
                               锛堝湪娌℃湁 INIT_SHARED 鐨勬儏鍐典笅鍒涘缓鐨勫唴瀛樻枃浠跺皢琚爣璁颁负绉佹湁锛夈€?
                               鍏变韩鍐呭瓨鍙互缂洪〉鏄犲皠鍒板涓绘満鐢ㄦ埛绌洪棿椤佃〃銆傜鏈夊唴瀛樺垯涓嶈兘銆?
  ============================ ================================================

褰?KVM MMU 鎵ц PFN 鏌ユ壘浠ユ湇鍔″鎴锋満缂洪〉锛屼笖鍚庡 guest_memfd 璁剧疆浜?GUEST_MEMFD_FLAG_MMAP 鏃讹紝
鏃犺璇ョ己椤垫槸鍏变韩杩樻槸绉佹湁鐨勶紝缂洪〉閮藉皢濮嬬粓浠?guest_memfd 娑堣垂銆?

鏇村缁嗚妭璇峰弬瑙?KVM_SET_USER_MEMORY_REGION2銆?

### 4.143 KVM_PRE_FAULT_MEMORY


:Capability: KVM_CAP_PRE_FAULT_MEMORY
:Architectures: none
:Type: vcpu ioctl
:Parameters: struct kvm_pre_fault_memory (in/out)
:Returns: 0 if at least one page is processed, < 0 on error

閿欒鐮侊細

  ========== ===============================================================
  EINVAL     鎸囧畾鐨?`gpa` 鍜?`size` 鏃犳晥锛堜緥濡傛湭椤靛榻愩€佸鑷存孩鍑猴紝鎴?size
             涓洪浂锛夈€?
  ENOENT     鎸囧畾鐨?`gpa` 鍦ㄥ凡瀹氫箟鐨?memslot 涔嬪銆?
  EINTR      瀛樺湪鏈睆钄界殑鎸傝捣淇″彿锛屼笖鏈鐞嗕换浣曢〉銆?
  EFAULT     鍙傛暟鍦板潃鏃犳晥銆?
  EOPNOTSUPP 涓?GPA 鏄犲皠鍐呭瓨涓嶅彈 hypervisor 鏀寔锛屽拰/鎴栭拡瀵瑰綋鍓?vCPU 鐘舵€?妯″紡
             涓嶆敮鎸併€?
  EIO        鎰忓閿欒鏉′欢锛堜篃浼氬鑷?WARN锛?
  ========== ===============================================================

```

  struct kvm_pre_fault_memory {
	/* in/out */
	__u64 gpa;
	__u64 size;
	/* in */
	__u64 flags;
	__u64 padding[5];
  };

```
KVM_PRE_FAULT_MEMORY 濉厖 KVM 鐢ㄤ簬涓哄綋鍓?vCPU 鐘舵€佹槧灏勫唴瀛樼殑 stage-2 椤佃〃銆侹VM 鍍?vCPU 浜х敓浜?
stage-2 璇荤己椤典竴鏍锋槧灏勫唴瀛橈紝渚嬪鎸夐渶缂洪〉鏄犲皠鍐呭瓨锛屼絾涓嶆墦鐮村啓鏃跺鍒讹紙CoW锛夈€備絾鏄紝KVM 涓嶄細灏嗕换浣曟柊
鍒涘缓鐨?stage-2 PTE 鏍囪涓?Accessed銆?

鍦ㄦ満瀵?VM 绫诲瀷涓紝鍦ㄥ鎴锋満琚?瀹氱"/搴﹂噺涔嬪墠闇€瑕佸绉佹湁瀹㈡満鍐呭瓨杩涜鍒濆璁剧疆鐨勬儏鍐典笅锛屾 ioctl 搴?
浠呭湪瀹屾垚鎵€鏈夊繀瑕佺殑璁剧疆浠ュ皢瀹㈡埛鏈虹疆浜?瀹氱"鐘舵€佷箣鍚庡彂鍑猴紝浠ヤ究涓婅堪璇箟鑳藉琚彲闈犲湴淇濊瘉銆?

鍦ㄦ煇浜涙儏鍐典笅锛屽涓?vCPU 鍙兘鍏变韩椤佃〃銆傚湪杩欑鎯呭喌涓嬶紝璇?ioctl 鍙互骞惰璋冪敤銆?

褰?ioctl 杩斿洖鏃讹紝杈撳叆鍊艰鏇存柊浠ユ寚鍚戝墿浣欒寖鍥淬€傚鏋滆繑鍥炴椂 `size` > 0锛岃皟鐢ㄨ€呭彲浠ュ啀娆′娇鐢ㄧ浉鍚岀殑
`struct kvm_map_memory` 鍙傛暟鍙戝嚭璇?ioctl銆?

褰卞瓙椤佃〃鏃犳硶鏀寔姝?ioctl锛屽洜涓哄畠浠槸閫氳繃铏氭嫙鍦板潃鎴栧祵濂楀鎴锋満鐗╃悊鍦板潃绱㈠紩鐨勩€傚綋瀹㈡埛鏈轰娇鐢ㄥ奖瀛愰〉琛?
鏃讹紙渚嬪鍥犱负瀹冩鍦ㄨ繍琛屽甫鏈夊祵濂楅〉琛ㄧ殑宓屽瀹㈡埛鏈猴級璋冪敤姝?ioctl锛屽嵆浣?`KVM_CHECK_EXTENSION` 鎶ュ憡璇?
鑳藉姏瀛樺湪锛屼篃浼氫互 `EOPNOTSUPP` 澶辫触銆?

`flags` 鐩墠蹇呴』涓洪浂銆?

### 4.144 KVM_S390_KEYOP


:Capability: KVM_CAP_S390_KEYOP
:Architectures: s390
:Type: vm ioctl
:Parameters: struct kvm_s390_keyop (in/out)
:Returns: 0 in case of success, < 0 on error

瀵圭粰瀹氱殑瀹㈡埛鏈哄湴鍧€鎵ц鎸囧畾鐨勫瘑閽ユ搷浣溿€傚厛鍓嶇殑瀛樺偍閿紙鎴栧叾鐩稿叧閮ㄥ垎锛夊皢鍦?`key` 涓繑鍥炪€?

```

  struct kvm_s390_keyop {
	__u64 guest_addr;
	__u8  key;
	__u8  operation;
  };

```
鐩墠 `operation` 鏀寔鐨勫涓嬪€硷細

KVM_S390_KEYOP_ISKE
  鍦?`key` 涓繑鍥炲鎴锋満鍦板潃 `guest_addr` 鐨勫瓨鍌ㄩ敭銆?

KVM_S390_KEYOP_RRBE
  閲嶇疆瀹㈡埛鏈哄湴鍧€ `guest_addr` 鐨勫紩鐢ㄤ綅锛坮eference bit锛夛紝鍦?`key` 涓繑鍥炴棫瀛樺偍閿殑 R 鍜?C 浣嶏紱
  瀛樺偍閿殑鍏朵綑瀛楁灏嗚璁剧疆涓?0銆?

KVM_S390_KEYOP_SSKE
  灏嗗鎴锋満鍦板潃 `guest_addr` 鐨勫瓨鍌ㄩ敭璁剧疆涓?`key` 涓寚瀹氱殑閿紝鍦?`key` 涓繑鍥炲厛鍓嶇殑鍊笺€?


## 5. The kvm_run structure


搴旂敤绋嬪簭浠ｇ爜閫氳繃 mmap() 涓€涓?vcpu fd 鏉ヨ幏鍙栨寚鍚?kvm_run 缁撴瀯浣撶殑鎸囬拡銆備粠閭ｆ椂璧凤紝搴旂敤绋嬪簭浠ｇ爜鍙互閫氳繃
鍦ㄨ皟鐢?KVM_RUN ioctl 涔嬪墠鏇存敼 kvm_run 涓殑瀛楁鏉ユ帶鍒舵墽琛岋紝骞堕€氳繃鏌ユ壘缁撴瀯浣撴垚鍛樻潵鑾峰彇鍏充簬 KVM_RUN
杩斿洖鍘熷洜鐨勪俊鎭€?

```

  struct kvm_run {
	/* in */
	__u8 request_interrupt_window;

```
璇锋眰 KVM_RUN 鍦ㄥ彲浠ュ皢浼氬閮ㄤ腑鏂敞鍏ュ鎴锋満鏃惰繑鍥炪€備笌 KVM_INTERRUPT 閰嶅悎浣跨敤寰堟湁鐢ㄣ€?

```

	__u8 immediate_exit;

```
璇ュ瓧娈靛湪 KVM_RUN 鍚姩鏃惰疆璇竴娆★紱濡傛灉闈為浂锛孠VM_RUN 绔嬪嵆閫€鍑猴紝杩斿洖 -EINTR銆傚湪閫氬父浣跨敤淇″彿灏?VCPU
"韪?鍑?KVM_RUN 鐨勫父瑙佸満鏅腑锛岃瀛楁鍙敤浜庨伩鍏嶄娇鐢?KVM_SET_SIGNAL_MASK锛屽悗鑰呯殑鍙墿灞曟€ц緝宸€備笌鍏?
鍦?KVM_RUN 涔嬪闃诲淇″彿锛岀敤鎴风┖闂村彲浠ヨ缃竴涓俊鍙峰鐞嗙▼搴忥紝灏?run->immediate_exit 璁剧疆涓洪潪闆跺€笺€?

濡傛灉 KVM_CAP_IMMEDIATE_EXIT 涓嶅彲鐢紝鍒欏拷鐣ユ瀛楁銆?

```

	__u8 padding1[6];

	/* out */
	__u32 exit_reason;

```
褰?KVM_RUN 鎴愬姛杩斿洖锛堣繑鍥炲€?0锛夋椂锛岃繖鍛婄煡搴旂敤绋嬪簭浠ｇ爜 KVM_RUN 涓轰綍杩斿洖銆傛瀛楁鐨勫厑璁稿€煎湪涓嬮潰璇﹁堪銆?

```

	__u8 ready_for_interrupt_injection;

```
濡傛灉宸叉寚瀹?request_interrupt_window锛屽垯姝ゅ瓧娈垫寚绀虹幇鍦ㄥ彲浠ヤ娇鐢?KVM_INTERRUPT 娉ㄥ叆涓柇銆?

```

	__u8 if_flag;

```
褰撳墠涓柇鏍囧織鐨勫€笺€備粎鍦ㄥ唴鏍告€佹湰鍦?APIC 鏈娇鐢ㄦ椂鏈夋晥銆?

```

	__u16 flags;

```
鏇村鏋舵瀯鐩稿叧鐨勬爣蹇楋紝璇︾粏璇存槑 VCPU 鐨勭姸鎬侊紝鍙兘
```

  /* x86, set if the VCPU is in system management mode */
  #define KVM_RUN_X86_SMM          (1 << 0)
  /* x86, set if bus lock detected in VM */
  #define KVM_RUN_X86_BUS_LOCK     (1 << 1)
  /* x86, set if the VCPU is executing a nested (L2) guest */
  #define KVM_RUN_X86_GUEST_MODE   (1 << 2)

  /* arm64, set for KVM_EXIT_DEBUG */
  #define KVM_DEBUG_ARCH_HSR_HIGH_VALID  (1 << 0)

```
```

	/* in (pre_kvm_run), out (post_kvm_run) */
	__u64 cr8;

```
cr8 瀵勫瓨鍣ㄧ殑鍊笺€備粎鍦ㄥ唴鏍告€佹湰鍦?APIC 鏈娇鐢ㄦ椂鏈夋晥銆傛棦杈撳叆鍙堣緭鍑恒€?

```

	__u64 apic_base;

```
APIC BASE msr 鐨勫€笺€備粎鍦ㄥ唴鏍告€佹湰鍦?APIC 鏈娇鐢ㄦ椂鏈夋晥銆傛棦杈撳叆鍙堣緭鍑恒€?

```

	union {
		/* KVM_EXIT_UNKNOWN */
		struct {
			__u64 hardware_exit_reason;
		} hw;

```
濡傛灉 exit_reason 鏄?KVM_EXIT_UNKNOWN锛屽垯 vcpu 鐢变簬鏈煡鍘熷洜閫€鍑恒€傝繘涓€姝ョ殑鏋舵瀯鐩稿叧淇℃伅鍙湪
hardware_exit_reason 涓幏寰椼€?

```

		/* KVM_EXIT_FAIL_ENTRY */
		struct {
			__u64 hardware_entry_failure_reason;
			__u32 cpu; /* if KVM_LAST_CPU */
		} fail_entry;

```
濡傛灉 exit_reason 鏄?KVM_EXIT_FAIL_ENTRY锛屽垯鐢变簬鏈煡鍘熷洜 vcpu 鏃犳硶杩愯銆傝繘涓€姝ョ殑鏋舵瀯鐩稿叧淇℃伅鍙湪
hardware_entry_failure_reason 涓幏寰椼€?

```

		/* KVM_EXIT_EXCEPTION */
		struct {
			__u32 exception;
			__u32 error_code;
		} ex;

```
鏈娇鐢ㄣ€?

```

		/* KVM_EXIT_IO */
		struct {
  #define KVM_EXIT_IO_IN  0
  #define KVM_EXIT_IO_OUT 1
			__u8 direction;
			__u8 size; /* bytes */
			__u16 port;
			__u32 count;
			__u64 data_offset; /* relative to kvm_run start */
		} io;

```
濡傛灉 exit_reason 鏄?KVM_EXIT_IO锛屽垯 vcpu 鎵ц浜嗕竴鏉℃棤娉曡 kvm 婊¤冻鐨勭鍙?I/O 鎸囦护銆俤ata_offset
鎻忚堪浜嗘暟鎹墍鍦ㄧ殑浣嶇疆锛圞VM_EXIT_IO_OUT锛夋垨 kvm 鏈熸湜搴旂敤绋嬪簭浠ｇ爜涓轰笅涓€娆?KVM_RUN 璋冪敤鏀剧疆鏁版嵁鐨勪綅缃?
锛圞VM_EXIT_IO_IN锛夈€傛暟鎹牸寮忔槸鎵撳寘鏁扮粍銆?

```

		/* KVM_EXIT_DEBUG */
		struct {
			struct kvm_debug_exit_arch arch;
		} debug;

```
濡傛灉 exit_reason 鏄?KVM_EXIT_DEBUG锛屽垯 vcpu 姝ｅ湪澶勭悊涓€涓皟璇曚簨浠讹紝杩斿洖鏋舵瀯鐩稿叧鐨勪俊鎭€?

```

		/* KVM_EXIT_MMIO */
		struct {
			__u64 phys_addr;
			__u8  data[8];
			__u32 len;
			__u8  is_write;
		} mmio;

```
濡傛灉 exit_reason 鏄?KVM_EXIT_MMIO锛屽垯 vcpu 鎵ц浜嗕竴鏉℃棤娉曡 kvm 婊¤冻鐨勫唴瀛樻槧灏?I/O 鎸囦护銆?data'
鎴愬憳鍖呭惈鍐欏叆鐨勬暟鎹紙濡傛灉 'is_write' 涓?true锛夛紝鍚﹀垯搴旂敱搴旂敤绋嬪簭浠ｇ爜濉厖銆?

'data' 鎴愬憳鍦ㄥ叾鍓?'len' 涓瓧鑺備腑鍖呭惈璇ュ€硷紝灏卞儚 VCPU 鐩存帴瀵瑰瓧鑺傛暟缁勬墽琛屼簡閫傚綋瀹藉害鐨勫姞杞芥垨瀛樺偍涓€鏍枫€?


      For KVM_EXIT_IO, KVM_EXIT_MMIO, KVM_EXIT_OSI, KVM_EXIT_PAPR, KVM_EXIT_XEN,
      KVM_EXIT_EPR, KVM_EXIT_HYPERCALL, KVM_EXIT_TDX,
      KVM_EXIT_X86_RDMSR and KVM_EXIT_X86_WRMSR the corresponding
      operations are complete (and guest state is consistent) only after userspace
      has re-entered the kernel with KVM_RUN.  The kernel side will first finish
      incomplete operations and then check for pending signals.

      鎿嶄綔鐨勯潪鎸傝捣鐘舵€佷笉淇濆瓨鍦ㄧ敤鎴风┖闂村彲瑙佺殑鐘舵€佷腑锛屽洜姝ょ敤鎴风┖闂村簲纭繚鍦ㄦ墽琛屽疄鏃惰縼绉讳箣鍓嶆搷浣滃凡
      瀹屾垚銆傜敤鎴风┖闂村彲浠ラ€氳繃甯︽湁鏈睆钄芥寕璧蜂俊鍙锋垨璁剧疆浜?immediate_exit 瀛楁閲嶆柊杩涘叆瀹㈡埛鏈烘潵瀹屾垚
      鎸傝捣鐨勬搷浣滐紝鑰屼笉鍏佽鎵ц浠讳綍杩涗竴姝ョ殑鎸囦护銆?

```

		/* KVM_EXIT_HYPERCALL */
		struct {
			__u64 nr;
			__u64 args[6];
			__u64 ret;
			__u64 flags;
		} hypercall;


```
寮虹儓寤鸿鐢ㄦ埛绌洪棿浣跨敤 `KVM_EXIT_IO`锛坸86锛夋垨 `KVM_EXIT_MMIO`锛堥櫎 s390 澶栫殑鎵€鏈夋灦鏋勶級鏉ュ疄鐜伴渶瑕?
瀹㈡埛鏈轰笌瀹夸富鏈虹敤鎴风┖闂翠氦浜掔殑鍔熻兘銆?
### 瀵逛簬 arm64锛?


SMCCC 閫€鍑哄彲鏍规嵁 SMCCC 杩囨护鍣ㄧ殑閰嶇疆鍚敤銆傛洿澶氱粏鑺傝鍙傞槄
Documentation/virt/kvm/devices/vm.rst 涓殑 `KVM_ARM_SMCCC_FILTER`銆?

`nr` 鍖呭惈瀹㈡埛鏈?SMCCC 璋冪敤鐨勫姛鑳?ID銆傜敤鎴风┖闂村簲浣跨敤 `KVM_GET_ONE_REG`
ioctl 浠?vCPU 鐨?GPR 涓绱㈣皟鐢ㄥ弬鏁般€?

`flags` 鐨勫畾涔夛細
 - `KVM_HYPERCALL_EXIT_SMC`锛氳〃绀哄鎴锋満浣跨敤 SMC 閫氶亾鍙戣捣 SMCCC 璋冪敤銆?
   鑻ヨ浣嶄负 0锛屽垯瀹㈡埛鏈轰娇鐢?HVC 閫氶亾鍙戣捣 SMCCC 璋冪敤銆?

 - `KVM_HYPERCALL_EXIT_16BIT`锛氳〃绀哄鎴锋満浣跨敤 16 浣嶆寚浠ゅ彂璧?SMCCC 璋冪敤銆?
   鑻ヨ浣嶄负 0锛屽垯瀹㈡埛鏈轰娇鐢?32 浣嶆寚浠ゃ€侫Arch64 瀹㈡埛鏈鸿浣嶅缁堜负 0銆?

閫€鍑烘椂锛孭C 鎸囧悜闄烽槺鎸囦护涔嬪悗鐨勯偅鏉℃寚浠ゃ€?

```

		/* KVM_EXIT_TPR_ACCESS */
		struct {
			__u64 rip;
			__u32 is_write;
			__u32 pad;
		} tpr_access;

```
寰呰ˉ鍏呮枃妗ｏ紙KVM_TPR_ACCESS_REPORTING锛夈€?

```

		/* KVM_EXIT_S390_SIEIC */
		struct {
			__u8 icptcode;
			__u64 mask; /* psw 涓婂崐閮ㄥ垎 */
			__u64 addr; /* psw 涓嬪崐閮ㄥ垎 */
			__u16 ipa;
			__u32 ipb;
		} s390_sieic;

```
s390 鐗规湁銆?

```

		/* KVM_EXIT_S390_RESET */
  #define KVM_S390_RESET_POR       1
  #define KVM_S390_RESET_CLEAR     2
  #define KVM_S390_RESET_SUBSYSTEM 4
  #define KVM_S390_RESET_CPU_INIT  8
  #define KVM_S390_RESET_IPL       16
		__u64 s390_reset_flags;

```
s390 鐗规湁銆?

```

		/* KVM_EXIT_S390_UCONTROL */
		struct {
			__u64 trans_exc_code;
			__u32 pgm_code;
		} s390_ucontrol;

```
s390 鐗规湁銆傜敤鎴锋帶鍒剁殑铏氭嫙鏈猴紙KVM_VM_S390_UNCONTROL锛夊湪鍏跺涓婚〉琛ㄤ笂鍙戠敓浜?
鍐呮牳鏃犳硶瑙ｆ瀽鐨勭己椤垫晠闅溿€?
鏀剧疆鍦?CPU lowcore 涓殑绋嬪簭浠ｇ爜鍜岃浆鎹㈠紓甯镐唬鐮佸湪姝ゅ鎸?z 鏋舵瀯鎿嶄綔鍘熺悊
锛圥rinciples of Operation锛変竴涔︿腑鍔ㄦ€佸湴鍧€杞崲锛圖AT锛夌珷鑺傜殑瀹氫箟鍛堢幇銆?

```

		/* KVM_EXIT_DCR */
		struct {
			__u32 dcrn;
			__u32 data;
			__u8  is_write;
		} dcr;

```
宸插簾寮冣€斺€旀浘鐢ㄤ簬 440 KVM銆?

```

		/* KVM_EXIT_OSI */
		struct {
			__u64 gprs[32];
		} osi;

```
MOL 浣跨敤浜嗕竴绉嶅畠绉颁负鈥淥SI鈥濈殑鐗规畩瓒呯骇璋冪敤鎺ュ彛銆備负浜嗗惎鐢ㄥ畠锛屾垜浠崟鑾?
瓒呯骇璋冪敤骞朵互璇ラ€€鍑虹粨鏋勯€€鍑猴紝鍏朵腑鍖呭惈浜嗗鎴锋満鐨勫叏閮?GPR銆?

濡傛灉 exit_reason 涓?KVM_EXIT_OSI锛屽垯琛ㄧず vCPU 瑙﹀彂浜嗘绫昏秴绾ц皟鐢ㄣ€?
鐢ㄦ埛绌洪棿鐜板湪鍙互澶勭悊璇ヨ秴绾ц皟鐢紝骞跺湪澶勭悊瀹屾垚鍚庢寜闇€淇敼 GPR銆傚鎴锋満
閲嶆柊杩涘叆鏃讹紝瀹㈡埛鏈烘墍鏈?GPR 閮藉皢琚缁撴瀯涓殑鍊兼浛鎹€?

```

		/* KVM_EXIT_PAPR_HCALL */
		struct {
			__u64 nr;
			__u64 ret;
			__u64 args[9];
		} papr_hcall;

```
鍦?64 浣?PowerPC 涓婃ā鎷?pSeries 鍒嗗尯锛堜緥濡傚湪 qemu 涓娇鐢ㄢ€減series鈥濇満鍨嬶級
鏃朵娇鐢ㄣ€傚綋瀹㈡埛鏈轰娇鐢ㄢ€渟c 1鈥濇寚浠ゅ彂璧疯秴绾ц皟鐢ㄦ椂鍙戠敓銆傗€渘r鈥濆瓧娈靛寘鍚?
瓒呯骇璋冪敤鍙凤紙鍙栬嚜瀹㈡埛鏈?R3锛夛紝鈥渁rgs鈥濆寘鍚弬鏁帮紙鍙栬嚜瀹㈡埛鏈?R4 - R12锛夈€?
鐢ㄦ埛绌洪棿搴斿皢杩斿洖鐮佹斁鍏モ€渞et鈥濓紝骞跺皢浠讳綍棰濆鐨勮繑鍥炲€兼斁鍏?args[]銆?
鍙兘鐨勮秴绾ц皟鐢ㄥ畾涔変簬 Power Architecture Platform Requirements锛圥APR锛?
鏂囨。锛屽彲浠?www.power.org 鑾峰彇锛堣闂渶鍏嶈垂寮€鍙戣€呮敞鍐岋級銆?

```

		/* KVM_EXIT_S390_TSCH */
		struct {
			__u16 subchannel_id;
			__u16 subchannel_nr;
			__u32 io_int_parm;
			__u32 io_int_word;
			__u32 ipb;
			__u8 dequeued;
		} s390_tsch;

```
s390 鐗规湁銆傚綋鍚敤浜?KVM_CAP_S390_CSS_SUPPORT 涓旀嫤鎴埌 TEST SUBCHANNEL
鏃朵細鍙戠敓姝ら€€鍑恒€傚鏋?dequeued 琚疆浣嶏紝鍒欑洰鏍囧瓙閫氶亾涓婃寕璧风殑 I/O 涓柇
宸茶鍑洪槦锛屽苟涓?subchannel_id銆乻ubchannel_nr銆乮o_int_parm 鍜?io_int_word
鍖呭惈浜嗚涓柇鐨勫弬鏁般€俰pb 鐢ㄤ簬鎸囦护鍙傛暟瑙ｇ爜銆?

```

		/* KVM_EXIT_EPR */
		struct {
			__u32 epr;
		} epr;

```
鍦?FSL BookE PowerPC 鑺墖涓婏紝涓柇鎺у埗鍣ㄦ湁涓€鏉″埌鏍稿績鐨勫揩閫熻矾寰勪腑鏂?
搴旂瓟閫氶亾銆傚綋鏍稿績鎴愬姛閫掗€佷竴涓腑鏂椂锛屽畠浼氳嚜鍔ㄧ敤涓柇鍚戦噺鍙峰～鍏?EPR
瀵勫瓨鍣紝骞跺湪涓柇鎺у埗鍣ㄥ唴閮ㄧ‘璁よ涓柇銆?

褰撲腑鏂帶鍒跺櫒浣嶄簬鐢ㄦ埛绌洪棿鏃讹紝鎴戜滑闇€瑕侀€氳繃瀹冩潵瀹屾垚涓柇纭鍛ㄦ湡锛?
浠ヤ娇鐢ㄦ閫€鍑鸿幏鍙栦笅涓€涓緟閫掗€佺殑涓柇鍚戦噺銆?

鍙 KVM_CAP_PPC_EPR 琚惎鐢ㄤ笖鏈夊閮ㄤ腑鏂垰鍒氳閫掗€佸埌瀹㈡埛鏈猴紝灏变細瑙﹀彂瀹冦€?
鐢ㄦ埛绌洪棿搴斿皢宸茬‘璁ょ殑涓柇鍚戦噺鏀惧叆鈥渆pr鈥濆瓧娈点€?

```

		/* KVM_EXIT_SYSTEM_EVENT */
		struct {
  #define KVM_SYSTEM_EVENT_SHUTDOWN       1
  #define KVM_SYSTEM_EVENT_RESET          2
  #define KVM_SYSTEM_EVENT_CRASH          3
  #define KVM_SYSTEM_EVENT_WAKEUP         4
  #define KVM_SYSTEM_EVENT_SUSPEND        5
  #define KVM_SYSTEM_EVENT_SEV_TERM       6
  #define KVM_SYSTEM_EVENT_TDX_FATAL      7
			__u32 type;
                        __u32 ndata;
                        __u64 data[16];
		} system_event;

```
濡傛灉 exit_reason 涓?KVM_EXIT_SYSTEM_EVENT锛屽垯琛ㄧず vCPU 閫氳繃鏌愮鏋舵瀯
鐗瑰畾鐨勬満鍒讹紙瓒呯骇璋冪敤鎴栨煇浜涚壒娈婃寚浠わ級瑙﹀彂浜嗙郴缁熺骇浜嬩欢銆傚湪 ARM64 涓婏紝
杩欐槸鐢?vCPU 鍩轰簬 HVC 鎸囦护鐨?PSCI 璋冪敤瑙﹀彂鐨勩€?

鈥渢ype鈥濆瓧娈垫弿杩颁簡绯荤粺绾т簨浠剁殑绫诲瀷銆?
鈥渢ype鈥濈殑鏈夋晥鍙栧€间负锛?

 - KVM_SYSTEM_EVENT_SHUTDOWN鈥斺€斿鎴锋満璇锋眰鍏抽棴铏氭嫙鏈恒€傜敤鎴风┖闂翠笉蹇?
   閬典粠璇ヨ姹傦紝濡傛灉閬典粠锛屼篃涓嶅繀鍚屾閿€姣佽櫄鎷熸満锛堝嵆瀹冨彲浠ュ湪鏈€缁堝叧闂?
   鍙戠敓涔嬪墠鍐嶆璋冪敤 KVM_RUN锛夈€?
 - KVM_SYSTEM_EVENT_RESET鈥斺€斿鎴锋満璇锋眰閲嶇疆铏氭嫙鏈恒€備笌 SHUTDOWN 涓€鏍凤紝
   鐢ㄦ埛绌洪棿鍙互閫夋嫨蹇界暐璇ヨ姹傦紝鎴栬€呰皟搴﹀湪鏈潵鐨勬煇涓椂鍒昏繘琛岄噸缃紝
   骞跺彲浠ュ啀娆¤皟鐢?KVM_RUN銆?
 - KVM_SYSTEM_EVENT_CRASH鈥斺€斿鎴锋満鍙戠敓浜嗗穿婧冿紝骞惰姹傝繘琛屽穿婧冪姸鎬佺淮鎶ゃ€?
   鐢ㄦ埛绌洪棿鍙互閫夋嫨蹇界暐璇ヨ姹傦紝鎴栬€呮敹闆嗚櫄鎷熸満鍐呭瓨鏍稿績杞偍鍜?鎴?
   瀵硅櫄鎷熸満杩涜閲嶇疆/鍏抽棴銆?
 - KVM_SYSTEM_EVENT_SEV_TERM鈥斺€斾竴涓?AMD SEV 瀹㈡埛鏈鸿姹傜粓姝€傚鎴锋満
   GHCB 鐨勫鎴锋満鐗╃悊鍦板潃瀛樺偍鍦?`data[^0^]` 涓€?
 - KVM_SYSTEM_EVENT_TDX_FATAL鈥斺€擳DX 瀹㈡埛鏈烘姤鍛婁簡鑷村懡閿欒鐘舵€併€侹VM 涓嶅仛
   浠讳綍瑙ｆ瀽鎴栬浆鎹紝鍙槸灏?16 涓€氱敤瀵勫瓨鍣ㄦ寜鎸囦护缂栫爜涓?x86-64 閫氱敤
   瀵勫瓨鍣?4 浣嶇储寮曠殑鍗囧簭杞偍鍒扮敤鎴风┖闂达紝濡?Intel SDM 涓墍瀹氫箟銆?
 - KVM_SYSTEM_EVENT_WAKEUP鈥斺€旈€€鍑虹殑 vCPU 澶勪簬鎸傝捣鐘舵€侊紝KVM 璇嗗埆鍒颁簡
   鍞ら啋浜嬩欢銆傜敤鎴风┖闂村彲浠ラ€氳繃灏嗚 vCPU 鏍囪涓哄彲杩愯鏉ユ帴鍙楄浜嬩欢锛?
   鎴栬€呮嫆缁濆畠骞跺啀娆¤皟鐢?KVM_RUN銆?
 - KVM_SYSTEM_EVENT_SUSPEND鈥斺€斿鎴锋満璇锋眰鎸傝捣铏氭嫙鏈恒€?

濡傛灉 KVM_CAP_SYSTEM_EVENT_DATA 瀛樺湪锛屽垯鈥渄ata鈥濆瓧娈靛彲浠ュ寘鍚绯荤粺鐨?
鏋舵瀯鐗瑰畾淇℃伅銆俤ata 鏁扮粍涓彧鏈夊墠 `ndata` 椤癸紙鍙兘涓洪浂锛夋槸鏈夋晥鐨勩€?

 - 瀵逛簬 arm64锛屽鏋滃鎴锋満鎸夌収 PSCI 瑙勮寖 v1.1 鍙戝嚭浜?SYSTEM_RESET2 璋冪敤锛?
   鍒?data[^0^] 琚涓?KVM_SYSTEM_EVENT_RESET_FLAG_PSCI_RESET2銆?

 - 瀵逛簬 arm64锛屽鏋滃鎴锋満鎸夌収 PSCI 瑙勮寖 v1.3 鍙戝嚭浜?SYSTEM_OFF2 璋冪敤锛?
   鍒?data[^0^] 琚涓?KVM_SYSTEM_EVENT_SHUTDOWN_FLAG_PSCI_OFF2銆?

 - 瀵逛簬 RISC-V锛宒ata[^0^] 琚涓?`sbi_system_reset` 璋冪敤绗簩涓弬鏁扮殑鍊笺€?

鏃╂湡鐗堟湰鐨?Linux 鍦ㄨ缁撴瀯涓畾涔変簡涓€涓?`flags` 鎴愬憳銆傝瀛楁鐜板湪宸插埆鍚?
涓?`data[^0^]`銆傜敤鎴风┖闂村彲浠ュ亣瀹氫粎褰?ndata 澶т簬 0 鏃舵墠浼氳鍐欏叆銆?

### 瀵逛簬 arm/arm64锛?


KVM_SYSTEM_EVENT_SUSPEND 閫€鍑洪€氳繃 KVM_CAP_ARM_SYSTEM_SUSPEND 铏氭嫙鏈鸿兘鍔?
鍚敤銆傚鏋滃鎴锋満璋冪敤 PSCI SYSTEM_SUSPEND 鍑芥暟锛孠VM 灏嗕互璇ヤ簨浠剁被鍨嬮€€鍑?
鍒扮敤鎴风┖闂淬€?

鐢ㄦ埛绌洪棿鍏ㄦ潈璐熻矗鎸夌収 ARM DEN0022D.b 5.19鈥淪YSTEM_SUSPEND鈥濆疄鐜?PSCI
SYSTEM_SUSPEND 璋冪敤銆侹VM 鍦ㄩ€€鍑哄埌鐢ㄦ埛绌洪棿涔嬪墠涓嶄細鏀瑰彉 vCPU 鐨勭姸鎬侊紝鍥犳
璋冪敤鍙傛暟鍘熷湴鐣欏湪 vCPU 瀵勫瓨鍣ㄤ腑銆?

鐢ㄦ埛绌洪棿_蹇呴』_瀵规绫婚€€鍑洪噰鍙栬鍔ㄣ€傚畠蹇呴』锛?

 - 鎺ュ彈瀹㈡埛鏈烘寕璧疯櫄鎷熸満鐨勮姹傘€傜敤鎴风┖闂村彲浠ラ€氳繃灏嗚璋冪敤 vCPU 鐨勭姸鎬?
   璁句负 KVM_MP_STATE_SUSPENDED 鏉ヨ姹傚湪鍐呮牳涓ā鎷熸寕璧枫€傝璋冪敤 vCPU 鎭㈠鏃讹紝
   鐢ㄦ埛绌洪棿蹇呴』鎸夌収浼犻€掔粰 PSCI 鍑芥暟鐨勫弬鏁伴厤缃?vCPU 鐘舵€併€傛湁鍏冲嚱鏁板弬鏁扮殑
   璇︽儏璇峰弬瑙?ARM DEN0022D.b 5.19.1鈥滈鏈熺敤閫斺€濄€?

 - 鎷掔粷瀹㈡埛鏈烘寕璧疯櫄鎷熸満鐨勮姹傘€傚彲鑳界殑杩斿洖鍊艰鍙傝 ARM DEN0022D.b 5.19.2
   鈥滆皟鐢ㄨ€呰亴璐ｂ€濄€?

浣跨敤 PSCI SYSTEM_OFF2 璋冪敤鐨勪紤鐪犲湪鍚敤 PSCI v1.3 鏃跺惎鐢ㄣ€傚鏋滃鎴锋満璋冪敤
PSCI SYSTEM_OFF2 鍑芥暟锛孠VM 灏嗕互 KVM_SYSTEM_EVENT_SHUTDOWN 浜嬩欢绫诲瀷閫€鍑哄埌
鐢ㄦ埛绌洪棿锛屼笖 data[^0^] 琚涓?KVM_SYSTEM_EVENT_SHUTDOWN_FLAG_PSCI_OFF2銆?
SYSTEM_OFF2 鍑芥暟鏀寔鐨勪紤鐪犵被鍨嬪彧鏈?HIBERNATE_OFF銆?

```

		/* KVM_EXIT_IOAPIC_EOI */
		struct {
			__u8 vector;
		} eoi;

```
琛ㄧず vCPU 鐨勫唴鏍告€佹湰鍦?APIC 鏀跺埌浜嗕竴涓數骞宠Е鍙戝瀷 IOAPIC 涓柇鐨?EOI銆?
姝ら€€鍑轰粎鍦?IOAPIC 瀹炵幇浜庣敤鎴风┖闂达紙鍗冲惎鐢ㄤ簡 KVM_CAP_SPLIT_IRQCHIP锛夋椂
瑙﹀彂锛涚敤鎴风┖闂?IOAPIC 搴斿鐞嗚 EOI锛屽苟鍦ㄤ腑鏂粛琚柇瑷€鏃堕噸鏂拌Е鍙戣涓柇銆?
vector 鏄敹鍒?EOI 鐨?LAPIC 涓柇鍚戦噺銆?

```

		struct kvm_hyperv_exit {
  #define KVM_EXIT_HYPERV_SYNIC          1
  #define KVM_EXIT_HYPERV_HCALL          2
  #define KVM_EXIT_HYPERV_SYNDBG         3
			__u32 type;
			__u32 pad1;
			union {
				struct {
					__u32 msr;
					__u32 pad2;
					__u64 control;
					__u64 evt_page;
					__u64 msg_page;
				} synic;
				struct {
					__u64 input;
					__u64 result;
					__u64 params[2];
				} hcall;
				struct {
					__u32 msr;
					__u32 pad2;
					__u64 control;
					__u64 status;
					__u64 send_page;
					__u64 recv_page;
					__u64 pending_page;
				} syndbg;
			} u;
		};
		/* KVM_EXIT_HYPERV */
                struct kvm_hyperv_exit hyperv;

```
琛ㄧず vCPU 閫€鍑哄埌鐢ㄦ埛绌洪棿浠ュ鐞嗕笌 Hyper-V 妯℃嫙鐩稿叧鐨勪竴浜涗换鍔°€?

鈥渢ype鈥濈殑鏈夋晥鍙栧€间负锛?

 - KVM_EXIT_HYPERV_SYNIC鈥斺€斿悓姝ラ€氱煡鐢ㄦ埛绌洪棿 Hyper-V SynIC 鐘舵€佸彉鏇淬€?
   璇ラ€氱煡鐢ㄤ簬灏?SynIC 浜嬩欢/娑堟伅椤甸噸鏂版槧灏勶紝浠ュ強鍦ㄧ敤鎴风┖闂翠腑鍚敤/绂佺敤
   SynIC 娑堟伅/浜嬩欢澶勭悊銆?

 - KVM_EXIT_HYPERV_SYNDBG鈥斺€斿悓姝ラ€氱煡鐢ㄦ埛绌洪棿 Hyper-V 鍚堟垚璋冭瘯鍣ㄧ姸鎬佸彉鏇淬€?
   璇ラ€氱煡鐢ㄤ簬鏇存柊 pending_page 浣嶇疆锛屾垨鍙戦€佹帶鍒跺懡浠わ紙鍙戦€佷綅浜?send_page
   涓殑缂撳啿鍖猴紝鎴栨帴鏀剁紦鍐插尯鍒?recv_page锛夈€?

```

		/* KVM_EXIT_ARM_NISV / KVM_EXIT_ARM_LDST64B */
		struct {
			__u64 esr_iss;
			__u64 fault_ipa;
		} arm_nisv;

```
- KVM_EXIT_ARM_NISV锛?

鐢ㄤ簬 arm64 绯荤粺銆傚鏋滃鎴锋満璁块棶浜嗕笉鍦?memslot 涓殑鍐呭瓨锛孠VM 閫氬父浼氳繑鍥?
鍒扮敤鎴风┖闂村苟璇锋眰瀹冧唬涓鸿繘琛?MMIO 妯℃嫙銆備絾鏄紝瀵逛簬鏌愪簺绫诲埆鐨勬寚浠わ紝涓嶆彁渚?
鎸囦护瑙ｇ爜锛堟柟鍚戙€佸唴瀛樿闂暱搴︼級锛岃€屼粠铏氭嫙鏈轰腑鍙栧嚭骞惰В鐮佹寚浠ょ殑杩囩▼鍦?
鍐呮牳涓繃浜庡鏉傘€?

鍘嗗彶涓婏紝鍙戠敓杩欑鎯呭喌鏃讹紝KVM 浼氭墦鍗拌鍛婂苟鏉€姝昏櫄鎷熸満銆侹VM 鍋囪濡傛灉瀹㈡埛鏈?
璁块棶浜嗛潪 memslot 鍐呭瓨锛屽畠灏辨槸鍦ㄥ皾璇曡繘琛?I/O锛岃€岃 I/O 鏃犳硶琚ā鎷燂紝璀﹀憡
娑堟伅涔熸槸鎹鎺緸鐨勩€傜劧鑰岋紝鏇村父瑙佺殑鎯呭喌鏄鎴锋満 bug 瀵艰嚧璁块棶浜嗗鎴锋満
鍐呭瓨鍖哄煙涔嬪鐨勫湴鏂癸紝杩欏簲褰撳鑷存洿鏈夋剰涔夌殑璀﹀憡娑堟伅锛屽苟涓斿鏋滆闂病鏈夎惤鍦?
I/O 绐楀彛鍐咃紝鍒欏簲鍦ㄥ鎴锋満涓Е鍙戝閮ㄤ腑姝€?

鐢ㄦ埛绌洪棿瀹炵幇鍙互鏌ヨ KVM_CAP_ARM_NISV_TO_USER锛屽苟鍦ㄥ垱寤鸿櫄鎷熸満鏃跺惎鐢ㄨ
鑳藉姏銆備竴鏃﹀畬鎴愶紝姝ょ被閿欒灏嗘敼涓轰互 KVM_EXIT_ARM_NISV 杩斿洖鍒扮敤鎴风┖闂达紝鍏朵腑
ESR_EL2 涓殑鏈夋晥浣嶄綅浜?esr_iss 瀛楁锛屾晠闅?IPA 浣嶄簬 fault_ipa 瀛楁銆?
鐢ㄦ埛绌洪棿鍙互閫氳繃浠庡鎴锋満鍐呭瓨涓В鐮佹寚浠わ紙濡傛灉瀹冮潪甯稿媷鏁級鏉ヤ慨澶嶈璁块棶
锛堝鏋滄槸鐪熸鐨?I/O 璁块棶锛夊苟缁х画鎵ц瀹㈡埛鏈猴紝鎴栬€呭畠鍙互閫夋嫨鎸傝捣銆佽浆鍌ㄦ垨
閲嶅惎瀹㈡埛鏈恒€?

娉ㄦ剰 KVM 涓嶄細鍍忓 KVM_EXIT_MMIO 閭ｆ牱璺宠繃鏁呴殰鎸囦护锛屼絾濡傛灉鐢ㄦ埛绌洪棿鍐冲畾
瑙ｇ爜骞舵ā鎷熻鎸囦护锛屽垯蹇呴』妯℃嫙瀵瑰鐞嗙姸鎬佺殑浠讳綍鏇存敼銆?

姝ょ壒鎬у鍙椾繚鎶ょ殑铏氭嫙鏈轰笉鍙敤锛屽洜涓虹敤鎴风┖闂存棤鏉冭闂墽琛屾ā鎷熸墍闇€鐨?
鐘舵€併€傜浉鍙嶏紝浼氱洿鎺ュ悜瀹㈡埛鏈烘敞鍏ヤ竴涓暟鎹腑姝㈠紓甯搞€傛敞鎰忥紝灏界鍦ㄥ彈淇濇姢
铏氭嫙鏈轰笂涓嬫枃涔嬪鏌ヨ鏃朵細鎶ュ憡 KVM_CAP_ARM_NISV_TO_USER锛屼絾鍦ㄥ彈淇濇姢铏氭嫙鏈?
鏂囦欢鎻忚堪绗︿笂鏌ヨ鏃惰鐗规€т笉浼氭毚闇层€?

- KVM_EXIT_ARM_LDST64B锛?

鐢ㄤ簬 arm64 绯荤粺銆傚綋瀹㈡埛鏈哄湪 memslot 涔嬪浣跨敤 LD64B銆丼T64B銆丼T64BV銆?
ST64BV0 鏃讹紝KVM 灏嗕互 KVM_EXIT_ARM_LDST64B 杩斿洖鍒扮敤鎴风┖闂达紝鏆撮湶鐩稿叧鐨?
ESR_EL2 淇℃伅鍜屾晠闅?IPA锛屼笌 KVM_EXIT_ARM_NISV 绫讳技銆?

鐢ㄦ埛绌洪棿搴斿畬鏁存ā鎷熻繖浜涙寚浠わ紝鍖呮嫭锛?

 - 鍙栧嚭瀛樺偍鎿嶄綔鏁帮紝鍖呮嫭 ST64BV0 鎸囦护鎯呭喌涓嬬殑 ACCDATA_EL1
 - 澶勭悊瀹㈡埛鏈轰负澶х搴忔椂鐨勫瓧鑺傚簭闂
 - 妯℃嫙璁块棶锛屽寘鎷闂湭鎴愬姛鏃堕€掗€佸紓甯?
 - 鍦?ST64BV/ST64BV0 鎯呭喌涓嬫彁渚涜繑鍥炲€?
 - 鍦ㄥ姞杞芥儏鍐典笅杩斿洖鏁版嵁
 - 鎸囦护鎴愬姛鎵ц鏃堕€掑 PC

娉ㄦ剰瀵规妯℃嫙娌℃湁鎬ц兘鏂归潰鐨勯鏈燂紝鍥犱负瀹冩秹鍙婁笌瀹㈡埛鏈虹姸鎬佺殑澶ч噺浜や簰銆?
鐒惰€岋紝鏈熸湜鑳藉淇濈暀鎸囦护鐨勮涔夛紝灏ゅ叾鏄?64 瀛楄妭璁块棶鐨勫崟鍓湰鍘熷瓙鎬у睘鎬с€?

濡傛灉鐢ㄦ埛绌洪棿灏?ID_AA64ISAR1_EL1.LS64 璁句负闈為浂鍊硷紙琛ㄧず鍚敤浜?FEAT_LS64*锛夛紝
鍒欏繀椤诲鐞嗘閫€鍑哄師鍥犮€?

```

		/* KVM_EXIT_X86_RDMSR / KVM_EXIT_X86_WRMSR */
		struct {
			__u8 error; /* user -> kernel */
			__u8 pad[7];
			__u32 reason; /* kernel -> user */
			__u32 index; /* kernel -> user */
			__u64 data; /* kernel <-> user */
		} msr;

```
鐢ㄤ簬 x86 绯荤粺銆傚綋铏氭嫙鏈鸿兘鍔?KVM_CAP_X86_USER_SPACE_MSR 鍚敤鏃讹紝瀵逛細寮曞彂
KVM 鍐呮牳浠ｇ爜 #GP 鐨勫瘎瀛樺櫒鐨?MSR 璁块棶锛屽彲鑳芥敼涓鸿Е鍙戣鏂瑰悜鐨?
KVM_EXIT_X86_RDMSR 閫€鍑哄拰鍐欐柟鍚戠殑 KVM_EXIT_X86_WRMSR 閫€鍑恒€?

鈥渞eason鈥濆瓧娈垫寚瀹氫簡 MSR 鎷︽埅鍙戠敓鐨勫師鍥犮€傜敤鎴风┖闂村彧浼氬湪閫氳繃 ENABLE_CAP
璇锋眰浜嗙壒瀹氬師鍥犳椂鎵嶄細鏀跺埌 MSR 閫€鍑恒€傚綋鍓嶆湁鏁堢殑閫€鍑哄師鍥犳湁锛?

============================ ========================================
 KVM_MSR_EXIT_REASON_UNKNOWN 璁块棶 KVM 鏈煡鐨?MSR
 KVM_MSR_EXIT_REASON_INVAL   璁块棶鏃犳晥 MSR 鎴栦繚鐣欎綅
 KVM_MSR_EXIT_REASON_FILTER  琚?KVM_X86_SET_MSR_FILTER 鎷︽埅鐨勮闂?
============================ ========================================

瀵逛簬 KVM_EXIT_X86_RDMSR锛屸€渋ndex鈥濆瓧娈靛憡璇夌敤鎴风┖闂村鎴锋満鎯宠璇诲彇鍝釜 MSR銆?
瑕佷互涓€娆℃垚鍔熺殑璇诲彇鍝嶅簲姝よ姹傦紝鐢ㄦ埛绌洪棿灏嗙浉搴旀暟鎹啓鍏モ€渄ata鈥濆瓧娈碉紝骞朵笖
蹇呴』缁х画鎵ц瀹㈡埛鏈轰互纭繚璇诲彇鐨勬暟鎹浼犻€佽繘瀹㈡埛鏈哄瘎瀛樺櫒鐘舵€併€?

濡傛灉 RDMSR 璇锋眰涓嶆垚鍔燂紝鐢ㄦ埛绌洪棿閫氳繃鍦ㄢ€渆rror鈥濆瓧娈典腑鍐欏叆鈥?鈥濇潵鎸囩ず銆?
杩欎細鍦?VCPU 鍐嶆琚墽琛屾椂鍚戝鎴锋満娉ㄥ叆涓€涓?#GP銆?

瀵逛簬 KVM_EXIT_X86_WRMSR锛屸€渋ndex鈥濆瓧娈靛憡璇夌敤鎴风┖闂村鎴锋満鎯宠鍐欏叆鍝釜 MSR銆?
澶勭悊瀹岃浜嬩欢鍚庯紝鐢ㄦ埛绌洪棿蹇呴』缁х画鎵ц vCPU銆傚鏋?MSR 鍐欏叆涓嶆垚鍔燂紝鐢ㄦ埛绌洪棿
涔熷皢鈥渆rror鈥濆瓧娈佃涓衡€?鈥濄€?

鏈夊叧涓?MSR 杩囨护浜や簰鐨勭粏鑺傦紝璇峰弬闃?KVM_X86_SET_MSR_FILTER銆?

```

		struct kvm_xen_exit {
  #define KVM_EXIT_XEN_HCALL          1
			__u32 type;
			union {
				struct {
					__u32 longmode;
					__u32 cpl;
					__u64 input;
					__u64 result;
					__u64 params[6];
				} hcall;
			} u;
		};
		/* KVM_EXIT_XEN */
                struct kvm_hyperv_exit xen;

```
琛ㄧず vCPU 閫€鍑哄埌鐢ㄦ埛绌洪棿浠ュ鐞嗕笌 Xen 妯℃嫙鐩稿叧鐨勪竴浜涗换鍔°€?

鈥渢ype鈥濈殑鏈夋晥鍙栧€间负锛?

  - KVM_EXIT_XEN_HCALL鈥斺€斿悓姝ラ€氱煡鐢ㄦ埛绌洪棿 Xen 瓒呯骇璋冪敤銆傜敤鎴风┖闂村簲褰撳湪
    鍐嶆璋冪敤 KVM_RUN 涔嬪墠灏嗚秴绾ц皟鐢ㄧ粨鏋滄斁鍏ョ浉搴斿瓧娈点€?

```

		/* KVM_EXIT_RISCV_SBI */
		struct {
			unsigned long extension_id;
			unsigned long function_id;
			unsigned long args[6];
			unsigned long ret[2];
		} riscv_sbi;

```
濡傛灉閫€鍑哄師鍥犱负 KVM_EXIT_RISCV_SBI锛屽垯琛ㄧず VCPU 鎵ц浜嗕笉鐢?KVM RISC-V
鍐呮牳妯″潡澶勭悊鐨?SBI 璋冪敤銆係BI 璋冪敤鐨勭粏鑺傚彲鍦?kvm_run 缁撴瀯鐨勨€渞iscv_sbi鈥?
鎴愬憳涓幏寰椼€傗€渞iscv_sbi鈥濈殑鈥渆xtension_id鈥濆瓧娈佃〃绀?SBI 鎵╁睍 ID锛岃€?
鈥渇unction_id鈥濆瓧娈佃〃绀虹粰瀹?SBI 鎵╁睍鐨勫嚱鏁?ID銆傗€渞iscv_sbi鈥濈殑鈥渁rgs鈥濇暟缁?
瀛楁琛ㄧず SBI 璋冪敤鐨勫弬鏁帮紝鈥渞et鈥濇暟缁勮〃绀鸿繑鍥炲€笺€傜敤鎴风┖闂村簲鍦ㄦ仮澶?VCPU
涔嬪墠鏇存柊 SBI 璋冪敤鐨勮繑鍥炲€笺€傛湁鍏?RISC-V SBI 瑙勮寖鐨勬洿澶氱粏鑺傦紝璇峰弬闃?
https://github.com/riscv/riscv-sbi-doc銆?

```

		/* KVM_EXIT_MEMORY_FAULT */
		struct {
  #define KVM_MEMORY_EXIT_FLAG_PRIVATE	(1ULL << 3)
			__u64 flags;
			__u64 gpa;
			__u64 size;
		} memory_fault;

```
KVM_EXIT_MEMORY_FAULT 琛ㄧず vCPU 閬囧埌浜?KVM 鏃犳硶瑙ｆ瀽鐨勫唴瀛樻晠闅溿€傗€済pa鈥濆拰
鈥渟ize鈥濓紙浠ュ瓧鑺備负鍗曚綅锛夋弿杩颁簡鏁呴殰鐨勫鎴锋満鐗╃悊鍦板潃鑼冨洿 [gpa, gpa + size)銆?
鈥渇lags鈥濆瓧娈垫弿杩颁簡鍙兘涓庢晠闅滅浉鍏崇殑璁块棶灞炴€э細

 - KVM_MEMORY_EXIT_FLAG_PRIVATE鈥斺€旂疆浣嶆椂锛岃〃绀哄唴瀛樻晠闅滃彂鐢熷湪绉佹湁鍐呭瓨
   璁块棶涓婏紱娓呴浂鏃讹紝琛ㄧず鏁呴殰鍙戠敓鍦ㄥ叡浜闂笂銆?

娉ㄦ剰锛並VM_EXIT_MEMORY_FAULT 鍦ㄦ墍鏈?KVM 閫€鍑哄師鍥犱腑鐙竴鏃犱簩锛屽畠浼撮殢鐨勮繑鍥?
鐮佹槸鈥?1鈥濊€岄潪鈥?鈥濓紒褰?KVM 浠?KVM_EXIT_MEMORY_FAULT 閫€鍑烘椂锛宔rrno 灏嗗缁?
璁句负 EFAULT 鎴?EHWPOISON锛屽浜庢墍鏈夊叾浠栭敊璇爜锛岀敤鎴风┖闂村簲鍋囧畾
kvm_run.exit_reason 鏄繃鏈?鏈畾涔夌殑銆?

```

    /* KVM_EXIT_NOTIFY */
    struct {
  #define KVM_NOTIFY_CONTEXT_INVALID	(1 << 0)
      __u32 flags;
    } notify;

```
鐢ㄤ簬 x86 绯荤粺銆傚綋铏氭嫙鏈鸿兘鍔?KVM_CAP_X86_NOTIFY_VMEXIT 鍚敤鏃讹紝濡傛灉鍦?VM
闈炴牴妯″紡涓嬬粡杩囨寚瀹氭椂闀夸粛鏃犱簨浠剁獥鍙ｅ彂鐢燂紝鍒欑敓鎴?VM 閫€鍑恒€備竴鏃﹀湪鍚敤璇ヨ兘鍔涙椂
璁剧疆浜?KVM_X86_NOTIFY_VMEXIT_USER锛屽畠灏嗕互閫€鍑哄師鍥?KVM_EXIT_NOTIFY 閫€鍑哄埌
鐢ㄦ埛绌洪棿浠ヨ繘琛岃繘涓€姝ュ鐞嗐€傗€渇lags鈥濆瓧娈靛寘鍚洿璇︾粏鐨勪俊鎭€?

鈥渇lags鈥濈殑鏈夋晥鍙栧€间负锛?

  - KVM_NOTIFY_CONTEXT_INVALID鈥斺€擵M 涓婁笅鏂囧凡鎹熷潖涓斿湪 VMCS 涓棤鏁堛€傚鏋滄仮澶?
    鐩爣铏氭嫙鏈猴紝灏嗗鑷存湭鐭ョ粨鏋溿€?

```

		/* KVM_EXIT_TDX */
		struct {
			__u64 flags;
			__u64 nr;
			union {
				struct {
					u64 ret;
					u64 data[5];
				} unknown;
				struct {
					u64 ret;
					u64 gpa;
					u64 size;
				} get_quote;
				struct {
					u64 ret;
					u64 leaf;
					u64 r11, r12, r13, r14;
				} get_tdvmcall_info;
				struct {
					u64 ret;
					u64 vector;
				} setup_event_notify;
			};
		} tdx;

```
澶勭悊鏉ヨ嚜瀹㈡埛鏈虹殑 TDVMCALL銆侹VM 鍩轰簬 Guest-Hypervisor 閫氫俊鎺ュ彛锛圙HCI锛夎鑼?
杞彂閫夊畾鐨?TDVMCALL锛汯VM 浠ユ渶灏忔敼鍔ㄥ皢杩欎簺璇锋眰妗ユ帴鍒扮敤鎴风┖闂?VMM锛屽皢杈撳叆
鏀惧叆 union锛屽苟鍦ㄩ噸鏂拌繘鍏ユ椂澶嶅埗鍥炲鎴锋満銆?

flags 褰撳墠濮嬬粓涓洪浂锛岃€?`nr` 鍖呭惈鏉ヨ嚜 R11 瀵勫瓨鍣ㄧ殑 TDVMCALL 鍙枫€倁nion 鐨?
鍏朵綑瀛楁鎻愪緵浜?TDVMCALL 鐨勮緭鍏ュ拰杈撳嚭銆傚綋鍓嶅畾涔変簡浠ヤ笅 `nr` 鍊硷細

 - `TDVMCALL_GET_QUOTE`锛氬鎴锋満宸茶姹傜敓鎴愮敱杩愯鍦ㄥ涓讳笂鐨?TD-Quoting
   椋炲湴锛圗nclave锛夌鍚嶇殑 TD-Quote銆傚弬鏁板拰杩斿洖鍊间綅浜?union 鐨?`get_quote`
   瀛楁銆俙gpa` 瀛楁鍜?`size` 鎸囧畾浜嗗鎴锋満鐗╃悊鍦板潃锛堟湭璁剧疆鍏变韩浣嶏級浠ュ強
   鍏变韩鍐呭瓨缂撳啿鍖虹殑澶у皬锛孴DX 瀹㈡埛鏈洪€氳繃璇ョ紦鍐插尯浼犻€?TD Report銆俙ret`
   瀛楁琛ㄧず GetQuote 璇锋眰鐨勮繑鍥炲€笺€傚綋璇锋眰鎴愬姛鍏ラ槦鍚庯紝TDX 瀹㈡埛鏈哄彲浠ヨ疆璇?
   鍏变韩鍐呭瓨鍖哄煙涓殑鐘舵€佸瓧娈碉紝浠ユ鏌?Quote 鐢熸垚鏄惁瀹屾垚銆傚畬鎴愬悗锛岀敓鎴愮殑
   Quote 閫氳繃鍚屼竴缂撳啿鍖鸿繑鍥炪€?

 - `TDVMCALL_GET_TD_VM_CALL_INFO`锛氬鎴锋満宸茶姹?TDVMCALL 鐨勬敮鎸佺姸鎬併€傜粰瀹?
   leaf 鐨勮緭鍑哄€煎簲鏀惧叆 union 鐨?`get_tdvmcall_info` 瀛楁涓粠 `r11` 鍒?
   `r14` 鐨勫瓧娈点€?

 - `TDVMCALL_SETUP_EVENT_NOTIFY_INTERRUPT`锛氬鎴锋満宸茶姹備负鍚戦噺 `vector`
   璁剧疆閫氱煡涓柇銆?

KVM 灏嗘潵鍙兘浼氬鍔犲鏇村鍊肩殑鏀寔锛岃繖浜涘€煎彲鑳藉鑷寸敤鎴风┖闂撮€€鍑猴紝鍗充娇娌℃湁
璋冪敤 `KVM_ENABLE_CAP` 鎴栫被浼兼帴鍙ｃ€傚湪杩欑鎯呭喌涓嬶紝瀹冨皢甯︾潃宸叉湁鏁堢殑杈撳嚭瀛楁
杩涘叆锛涢€氬父鎯呭喌涓嬶紝union 鐨?`unknown.ret` 瀛楁涓?
`TDVMCALL_STATUS_SUBFUNC_UNSUPPORTED`銆傚鏋滅敤鎴风┖闂翠笉甯屾湜鏀寔鏌愪釜 TDVMCALL锛?
鍒欐棤闇€鍋氫换浣曞鐞嗐€?

```

		/* KVM_EXIT_ARM_SEA */
		struct {
  #define KVM_EXIT_ARM_SEA_FLAG_GPA_VALID   (1ULL << 0)
			__u64 flags;
			__u64 esr;
			__u64 gva;
			__u64 gpa;
		} arm_sea;

```
鐢ㄤ簬 arm64 绯荤粺銆傚綋铏氭嫙鏈鸿兘鍔?`KVM_CAP_ARM_SEA_TO_USER` 鍚敤鏃讹紝濡傛灉瀹㈡埛鏈?
璁块棶瀵艰嚧浜嗗悓姝ュ閮ㄤ腑姝紙SEA锛変笖瀹夸富 APEI 鏃犳硶澶勭悊璇?SEA锛孠VM 浼氶€€鍑哄埌
鐢ㄦ埛绌洪棿銆?

`esr` 琚涓轰粠杩涘叆 KVM 鐨勫紓甯镐腑鍙栧嚭鐨?ESR_EL2 鐨勫噣鍖栧€硷紝鍖呭惈浠ヤ笅瀛楁锛?

 - `ESR_EL2.EC`
 - `ESR_EL2.IL`
 - `ESR_EL2.FnV`
 - `ESR_EL2.EA`
 - `ESR_EL2.CM`
 - `ESR_EL2.WNR`
 - `ESR_EL2.FSC`
 - `ESR_EL2.SET`锛堝綋涓?VM 瀹炵幇浜?FEAT_RAS 鏃讹級

褰?`ESR_EL2.FnV == 0` 鏃讹紝`gva` 琚涓轰粠杩涘叆 KVM 鐨勫紓甯镐腑鍙栧嚭鐨?FAR_EL2
鐨勫€笺€傚惁鍒欙紝`gva` 鐨勫€兼湭鐭ャ€?

褰?`KVM_EXIT_ARM_SEA_FLAG_GPA_VALID` 鏍囧織缃綅鏃讹紝`gpa` 琚涓轰粠杩涘叆 KVM 鐨?
寮傚父涓彇鍑虹殑鏁呴殰 IPA銆傚惁鍒欙紝`gpa` 鐨勫€兼湭鐭ャ€?

```

		/* 鍥哄畾 union 鐨勫ぇ灏忋€?*/
		char padding[256];
	};

	/*
	 * kvm 涓庣敤鎴风┖闂翠箣闂村叡浜殑瀵勫瓨鍣ㄣ€?
	 * kvm_valid_regs 鎸囧畾鐢卞涓昏缃殑瀵勫瓨鍣ㄧ被鍒?
	 * kvm_dirty_regs 鎸囧畾鐢辩敤鎴风┖闂村紕鑴忕殑瀵勫瓨鍣ㄧ被鍒?
	 * struct kvm_sync_regs 鏄灦鏋勭壒瀹氱殑锛宬vm_valid_regs 鍜?
	 * kvm_dirty_regs 鐨勪綅涔熸槸鏋舵瀯鐗瑰畾鐨?
	 */
	__u64 kvm_valid_regs;
	__u64 kvm_dirty_regs;
	union {
		struct kvm_sync_regs regs;
		char padding[SYNC_REGS_SIZE_BYTES];
	} s;

```
濡傛灉瀹氫箟浜?KVM_CAP_SYNC_REGS锛岃繖浜涘瓧娈靛厑璁哥敤鎴风┖闂翠笉蹇呰皟鐢?SET/GET_*REGS
鍗冲彲璁块棶鏌愪簺瀹㈡埛鏈哄瘎瀛樺櫒銆傚洜姝わ紝濡傛灉鐢ㄦ埛绌洪棿闇€瑕佸鐞嗛€€鍑猴紝鎴戜滑鍙互閬垮厤
涓€浜涚郴缁熻皟鐢ㄥ紑閿€銆傜敤鎴风┖闂村彲浠ラ€氳繃妫€鏌?kvm_valid_regs 鐨勭壒瀹氫綅鏉ユ煡璇㈣
缁撴瀯鐨勬湁鏁堟€с€傝繖浜涗綅鏄灦鏋勭壒瀹氱殑锛岄€氬父瀹氫箟涓€缁勫瘎瀛樺櫒鐨勬湁鏁堟€э紙渚嬪锛屼竴浣?
瀵瑰簲閫氱敤瀵勫瓨鍣級銆?

璇锋敞鎰忥紝鍐呮牳琚厑璁镐娇鐢?kvm_run 缁撴瀯浣滀负鏌愪簺瀵勫瓨鍣ㄧ被鍨嬬殑涓诲瓨鍌ㄣ€傚洜姝わ紝鍗充娇
kvm_dirty_regs 涓浉搴旂殑浣嶆湭缃綅锛屽唴鏍镐篃鍙兘浣跨敤 kvm_run 涓殑鍊笺€?

```

		/* KVM_EXIT_SNP_REQ_CERTS */
		struct kvm_exit_snp_req_certs {
			__u64 gpa;
			__u64 npages;
			__u64 ret;
		};

```
KVM_EXIT_SNP_REQ_CERTS 琛ㄧず涓€涓惎鐢ㄤ簡璇佷功鑾峰彇鐨?SEV-SNP 瀹㈡埛鏈猴紙瑙?
KVM_SEV_SNP_ENABLE_REQ_CERTS锛夌敓鎴愪簡涓€涓墿灞曞瀷瀹㈡埛鏈鸿姹?NAE #VMGEXIT
锛圫NP_GUEST_REQUEST锛夛紝娑堟伅绫诲瀷涓?MSG_REPORT_REQ锛屽嵆宸蹭粠鍥轰欢璇锋眰浜嗚瘉鏄?
鎶ュ憡锛屽苟甯屾湜鐢辫櫄鎷熸満鐩戞帶鍣ㄩ殢璇锋眰涓€骞舵彁渚涗笌璇佹槑鎶ュ憡绛惧悕鐩稿搴旂殑璇佷功鏁版嵁銆?

涓轰簡鍏佽鐢ㄦ埛绌洪棿鎻愪緵璇佷功锛屸€済pa鈥濆拰鈥渘pages鈥濆師鏍蜂粠瀹㈡埛鏈鸿姹傝浆鍙?
锛堝垎鍒负 RAX 鍜?RBX GHCB 瀛楁锛夈€傗€渞et鈥濅笉鏄潵鑷?KVM 鐨勨€滆緭鍑衡€濓紝閫€鍑烘椂
濮嬬粓涓衡€?鈥濄€侹VM 鍦ㄩ€€鍑哄埌鐢ㄦ埛绌洪棿涔嬪墠浼氶獙璇佲€済pa鈥濇槸 4KiB 瀵归綈鐨勶紝浣?
闄ゆ涔嬪涓嶄細楠岃瘉鏉ヨ嚜瀹㈡埛鏈虹殑淇℃伅銆?

鍦ㄤ笅涓€娆?KVM_RUN 鏃讹紙渚嬪鐢ㄦ埛绌洪棿宸叉湇鍔¤璇锋眰鎴栨病鏈夋湇鍔′箣鍚庯級锛孠VM 灏?
瀹屾垚 #VMGEXIT锛屼娇鐢ㄢ€渞et鈥濆瓧娈电‘瀹氭槸鍚戝鎴锋満鍙戜俊鍙锋垚鍔熻繕鏄け璐ワ紝澶辫触鏃?
閫氳繃 SW_EXITINFO2 鍛婄煡浣曠鍘熷洜鐮併€傚鏋溾€渞et鈥濊璁句负涓嶆敮鎸佺殑鍊硷紙瑙佷笅琛級锛?
KVM_RUN 灏嗕互 -EINVAL 澶辫触銆傚浜庘€渞et鈥濅负鈥淓NOSPC鈥濈殑鎯呭喌锛孠VM 杩樻秷璐光€渘pages鈥?
瀛楁锛屽嵆鐢ㄦ埛绌洪棿鍙互鐢ㄨ瀛楁鍛婄煡瀹㈡埛鏈轰繚瀛樺叏閮ㄨ瘉涔︽暟鎹墍闇€鐨勯〉鏁般€?

鏀寔鐨勨€渞et鈥濆€煎強鍏跺搴旂殑 SW_EXITINFO2 缂栫爜锛?

  ======     =============================================================
  0          0x0锛屽嵆鎴愬姛銆侹VM 灏嗗悜 SNP 鍥轰欢鍙戝嚭 SNP_GUEST_REQUEST 鍛戒护
  ENOSPC     0x0000000100000000锛屽嵆瀹㈡埛鏈洪〉涓嶈冻浠ュ绾宠瘉涔﹁〃鍜岃瘉涔︽暟鎹€?
             KVM 杩樹細鍦?GHBC 涓皢 RBX 瀛楁璁句负鈥渘pages鈥濄€?
  EAGAIN     0x0000000200000000锛屽嵆瀹夸富姝ｅ繖锛屽鎴锋満搴旈噸璇曡璇锋眰銆?
  EIO        0xffffffff00000000锛岀敤浜庢墍鏈夊叾浠栭敊璇紙姝よ繑鍥炵爜鏄?KVM 瀹氫箟鐨?
             铏氭嫙鏈虹洃鎺у櫒鍊硷紝濡?GHCB 鎵€鍏佽锛?
  ======     =============================================================


## 6. 鍙湪 vCPU 涓婂惎鐢ㄧ殑鑳藉姏


鏈夋煇浜涜兘鍔涘湪鍚敤鏃朵細鏀瑰彉铏氭嫙 CPU 鎴栬櫄鎷熸満鐨勮涓恒€傝鍚敤瀹冧滑锛岃鍙傞槄
KVM_ENABLE_CAP銆?

涓嬮潰浣犲彲浠ユ壘鍒颁竴浠借兘鍔涘垪琛紝浠ュ強鍚敤瀹冧滑鏃跺 vCPU 鎴栬櫄鎷熸満鐨勫奖鍝嶃€?

闅忔弿杩颁竴骞舵彁渚涗互涓嬩俊鎭細

  Architectures锛堟灦鏋勶級锛?
      鍝簺鎸囦护闆嗘灦鏋勬彁渚涙 ioctl銆倄86 鍚屾椂鍖呭惈 i386 鍜?x86_64銆?

  Target锛堢洰鏍囷級锛?
      杩欐槸姣?vCPU 杩樻槸姣?VM 鐨勮兘鍔涖€?

  Parameters锛堝弬鏁帮級锛?
      璇ヨ兘鍔涙帴鍙楀摢浜涘弬鏁般€?

  Returns锛堣繑鍥炲€硷級锛?
      杩斿洖鐨勫€笺€傞€氱敤閿欒鐮侊紙EBADF銆丒NOMEM銆丒INVAL锛変笉鍋氳缁嗚鏄庯紝浣嗗叿鏈?
      鐗瑰畾鍚箟鐨勯敊璇細浜堜互璇存槑銆?


### 6.1 KVM_CAP_PPC_OSI


:Architectures: ppc
:Target: vcpu
:Parameters: none
:Returns: 0 on success; -1 on error

姝よ兘鍔涘惎鐢?OSI 瓒呯骇璋冪敤鐨勬嫤鎴紝鍚﹀垯杩欎簺璋冪敤浼氳褰撲綔娉ㄥ叆鍒板鎴锋満鐨勬櫘閫?
绯荤粺璋冪敤銆侽SI 瓒呯骇璋冪敤鐢?Mac-on-Linux 鍙戞槑锛岀敤浜庡湪瀹㈡埛鏈哄拰瀹夸富涔嬮棿鎻愪緵
鏍囧噯鍖栫殑閫氫俊鏈哄埗銆?

鍚敤姝よ兘鍔涙椂锛屽彲鑳藉彂鐢?KVM_EXIT_OSI銆?


### 6.2 KVM_CAP_PPC_PAPR


:Architectures: ppc
:Target: vcpu
:Parameters: none
:Returns: 0 on success; -1 on error

姝よ兘鍔涘惎鐢?PAPR 瓒呯骇璋冪敤鐨勬嫤鎴€侾APR 瓒呯骇璋冪敤浣跨敤瓒呯骇璋冪敤鎸囦护鈥渟c 1鈥濆彂璧枫€?

瀹冭繕灏嗗鎴锋満鐗规潈绾у埆璁句负鈥渟upervisor鈥濇ā寮忋€傞€氬父瀹㈡埛鏈鸿繍琛屽湪鈥渉ypervisor鈥?
鐗规潈妯″紡涓嬶紝浣嗙己灏戜竴浜涚壒鎬с€?

闄や互涓婁箣澶栵紝瀹冭繕鏀瑰彉浜?SDR1 鐨勮涔夈€傚湪姝ゆā寮忎笅锛孲DR1 鐨?HTAB 鍦板潃閮ㄥ垎
鍖呭惈 HVA 鑰岄潪 GPA锛屽洜涓?PAPR 瀵瑰鎴锋満闅愯棌浜?HTAB銆?

鍚敤姝よ兘鍔涙椂锛屽彲鑳藉彂鐢?KVM_EXIT_PAPR_HCALL銆?


### 6.3 KVM_CAP_SW_TLB


:Architectures: ppc
:Target: vcpu
:Parameters: args[^0^] 鏄竴涓?struct kvm_config_tlb 鐨勫湴鍧€
:Returns: 0 on success; -1 on error

```

  struct kvm_config_tlb {
	__u64 params;
	__u64 array;
	__u32 mmu_type;
	__u32 array_len;
  };

```
閰嶇疆铏氭嫙 CPU 鐨?TLB 鏁扮粍锛屽湪鐢ㄦ埛绌洪棿鍜?KVM 涔嬮棿寤虹珛鍏变韩鍐呭瓨鍖哄煙銆傗€減arams鈥?
鍜屸€渁rray鈥濆瓧娈垫槸 mmu 绫诲瀷鐗瑰畾鏁版嵁缁撴瀯鐨勭敤鎴风┖闂村湴鍧€銆傗€渁rray_len鈥濆瓧娈?
鏄竴涓畨鍏ㄦ満鍒讹紝搴旇涓虹敤鎴风┖闂翠负鏁扮粍淇濈暀鐨勫唴瀛樺ぇ灏忥紙浠ュ瓧鑺傝锛夈€傚畠鑷冲皯
蹇呴』鏄€渕mu_type鈥濆拰鈥減arams鈥濇墍瑕佹眰鐨勫ぇ灏忋€?

褰?KVM_RUN 澶勪簬娲诲姩鐘舵€佹椂锛屽叡浜尯鍩熺敱 KVM 鎺у埗銆傚叾鍐呭鏈畾涔夛紝鐢ㄦ埛绌洪棿
瀵瑰叾杩涜鐨勪换浣曚慨鏀归兘浼氬鑷存湁鐣岀殑鏈畾涔夎涓恒€?

浠?KVM_RUN 杩斿洖鏃讹紝鍏变韩鍖哄煙灏嗗弽鏄犲鎴锋満 TLB 鐨勫綋鍓嶇姸鎬併€傚鏋滅敤鎴风┖闂?
杩涜浠讳綍鏇存敼锛屽畠蹇呴』鍦ㄥ啀娆″姝?vcpu 璋冪敤 KVM_RUN 涔嬪墠璋冪敤 KVM_DIRTY_TLB
鏉ュ憡鐭?KVM 鍝簺鏉＄洰宸茶鏇存敼銆?

瀵逛簬 mmu 绫诲瀷 KVM_MMU_FSL_BOOKE_NOHV 鍜?KVM_MMU_FSL_BOOKE_HV锛?

 - 鈥減arams鈥濆瓧娈电殑绫诲瀷涓衡€渟truct kvm_book3e_206_tlb_params鈥濄€?
 - 鈥渁rray鈥濆瓧娈垫寚鍚戜竴涓€渟truct kvm_book3e_206_tlb_entry鈥濈被鍨嬬殑鏁扮粍銆?
 - 璇ユ暟缁勭敱绗竴涓?TLB 涓殑鍏ㄩ儴鏉＄洰缁勬垚锛屽悗璺熺浜屼釜 TLB 涓殑鍏ㄩ儴鏉＄洰銆?
 - 鍦ㄤ竴涓?TLB 鍐呴儴锛屾潯鐩厛鎸夐泦鍚堝彿閫掑鎺掑簭銆傚湪涓€涓泦鍚堝唴閮紝鏉＄洰鎸?
   璺紙way锛岄€掑鐨?ESEL锛夋帓搴忋€?
 - 纭畾 TLB0 涓泦鍚堝彿鐨勫搱甯屼负锛?MAS2 >> 12) & (num_sets - 1)锛屽叾涓?
   鈥渘um_sets鈥濇槸 tlb_sizes[] 鍊奸櫎浠?tlb_ways[] 鍊笺€?
 - mas1 鐨?tsize 瀛楁鍦?TLB0 涓婂簲璁句负 4K锛屽敖绠＄‖浠跺姝ゅ€煎拷鐣ヤ笉璁°€?

### 6.4 KVM_CAP_S390_CSS_SUPPORT


:Architectures: s390
:Target: vcpu
:Parameters: none
:Returns: 0 on success; -1 on error

姝よ兘鍔涘惎鐢ㄥ閫氶亾 I/O 鎸囦护澶勭悊鏀寔銆?

TEST PENDING INTERRUPTION 浠ュ強 TEST SUBCHANNEL 鐨勪腑鏂儴鍒嗗湪鍐呮牳涓鐞嗭紝
鑰屽叾浠?I/O 鎸囦护鍒欎紶閫掔粰鐢ㄦ埛绌洪棿銆?

鍚敤姝よ兘鍔涙椂锛屼細鍦?TEST SUBCHANNEL 鎷︽埅鏃跺彂鐢?KVM_EXIT_S390_TSCH銆?

娉ㄦ剰锛屽嵆浣挎鑳藉姏鏄寜 vCPU 鍚敤鐨勶紝鏁翠釜铏氭嫙鏈洪兘浼氬彈鍒板奖鍝嶃€?

### 6.5 KVM_CAP_PPC_EPR


:Architectures: ppc
:Target: vcpu
:Parameters: args[^0^] 瀹氫箟浠ｇ悊璁炬柦鏄惁澶勪簬娲诲姩鐘舵€?
:Returns: 0 on success; -1 on error

姝よ兘鍔涘惎鐢ㄦ垨绂佺敤閫氳繃澶栭儴浠ｇ悊璁炬柦閫掗€佷腑鏂€?

鍚敤鏃讹紙args[^0^] != 0锛夛紝姣忔瀹㈡埛鏈烘敹鍒颁竴涓閮ㄤ腑鏂€掗€佹椂锛屽畠浼氳嚜鍔?
浠?KVM_EXIT_EPR 閫€鍑鸿繘鍏ョ敤鎴风┖闂达紝浠ユ帴鏀舵渶椤跺眰鐨勭粓绔悜閲忋€?

绂佺敤鏃讹紙args[^0^] == 0锛夛紝琛屼负濡傚悓姝よ鏂戒笉鍙楁敮鎸併€?

鍚敤姝よ兘鍔涙椂锛屽彲鑳藉彂鐢?KVM_EXIT_EPR銆?

### 6.6 KVM_CAP_IRQ_MPIC


:Architectures: ppc
:Parameters: args[^0^] 鏄?MPIC 璁惧 fd锛?
             args[^1^] 鏄 vcpu 鐨?MPIC CPU 鍙?

姝よ兘鍔涘皢 vcpu 杩炴帴鍒板唴鏍告€?MPIC 璁惧銆?

### 6.7 KVM_CAP_IRQ_XICS


:Architectures: ppc
:Target: vcpu
:Parameters: args[^0^] 鏄?XICS 璁惧 fd锛?
             args[^1^] 鏄 vcpu 鐨?XICS CPU 鍙凤紙server ID锛?

姝よ兘鍔涘皢 vcpu 杩炴帴鍒板唴鏍告€?XICS 璁惧銆?

### 6.8 KVM_CAP_S390_IRQCHIP


:Architectures: s390
:Target: vm
:Parameters: none

姝よ兘鍔涘惎鐢?s390 鐨勫唴鏍告€?irqchip銆傝鎯呰鍙傞槄鈥?.24 KVM_CREATE_IRQCHIP鈥濄€?

### 6.9 KVM_CAP_MIPS_FPU


:Architectures: mips
:Target: vcpu
:Parameters: args[^0^] 涓哄皢鏉ヤ繚鐣欙紙搴斾负 0锛夈€?

姝よ兘鍔涘厑璁稿鎴锋満浣跨敤瀹夸富鐨勬诞鐐瑰崟鍏冿紙FPU锛夈€傚畠鍏佽璁剧疆 Config1.FP 浣嶄互鍦?
瀹㈡埛鏈轰腑鍚敤 FPU銆備竴鏃﹀畬鎴愶紝灏卞彲浠ヨ闂?`KVM_REG_MIPS_FPR_**` 鍜?
`KVM_REG_MIPS_FCR_**` 瀵勫瓨鍣紙鍙栧喅浜庡綋鍓嶅鎴锋満 FPU 瀵勫瓨鍣ㄦā寮忥級锛屽苟涓?
Status.FR銆丆onfig5.FRE 浣嶅彲閫氳繃 KVM API 浠ュ強浠庡鎴锋満璁块棶锛屽墠鎻愭槸 FPU
鏀寔瀹冧滑銆?

### 6.10 KVM_CAP_MIPS_MSA


:Architectures: mips
:Target: vcpu
:Parameters: args[^0^] 涓哄皢鏉ヤ繚鐣欙紙搴斾负 0锛夈€?

姝よ兘鍔涘厑璁稿鎴锋満浣跨敤 MIPS SIMD 鏋舵瀯锛圡SA锛夈€傚畠鍏佽璁剧疆 Config3.MSAP 浣嶄互
鍦ㄥ鎴锋満涓惎鐢?MSA 鐨勪娇鐢ㄣ€備竴鏃﹀畬鎴愶紝灏卞彲浠ヨ闂?`KVM_REG_MIPS_VEC_**` 鍜?
`KVM_REG_MIPS_MSA_**` 瀵勫瓨鍣紝骞朵笖 Config5.MSAEn 浣嶅彲閫氳繃 KVM API 浠ュ強浠?
瀹㈡埛鏈鸿闂€?

### 6.74 KVM_CAP_SYNC_REGS


:Architectures: s390, x86
:Target: s390锛氬缁堝惎鐢紝x86锛歷cpu
:Parameters: none
:Returns: x86锛欿VM_CHECK_EXTENSION 杩斿洖涓€涓綅鏁扮粍锛屾寚绀烘敮鎸佸摢浜涘瘎瀛樺櫒闆?
          锛堜綅鍩熷畾涔変簬 arch/x86/include/uapi/asm/kvm.h锛夈€?

濡備笂鏂?kvm_run 涓?kvm_sync_regs 缁撴瀯淇℃伅鎵€杩帮紝KVM_CAP_SYNC_REGS
鈥滃厑璁竅鐢ㄦ埛绌洪棿]涓嶅繀璋冪敤 SET/GET_*REGS 鍗冲彲璁块棶鏌愪簺瀹㈡埛鏈哄瘎瀛樺櫒鈥濄€傝繖閫氳繃
娑堥櫎璁剧疆/鑾峰彇瀵勫瓨鍣ㄥ€肩殑閲嶅 ioctl 璋冪敤鍑忓皯浜嗗紑閿€銆傚綋鐢ㄦ埛绌洪棿姝ｅ湪杩涜
鍚屾鐨勫鎴锋満鐘舵€佷慨鏀癸紙渚嬪锛屽湪鐢ㄦ埛绌洪棿涓ā鎷熷拰/鎴栨嫤鎴寚浠わ級鏃讹紝杩欎竴鐐?
灏や负閲嶈銆?

鏈夊叧 s390 鐨勭粏鑺傦紝璇峰弬闃呮簮浠ｇ爜銆?

瀵逛簬 x86锛?

- 瑕佸鍒跺埌 kvm_run 鐨勫瘎瀛樺櫒闆嗗彲鐢辩敤鎴风┖闂撮€夋嫨锛堣€屼笉鏄瘡娆￠€€鍑洪兘澶嶅埗鍑?
  鎵€鏈夊瘎瀛樺櫒闆嗭級銆?
- 闄?regs 鍜?sregs 澶栵紝杩樺彲浣跨敤 vcpu_events銆?

瀵逛簬 x86锛宻truct kvm_run 鐨勨€渒vm_valid_regs鈥濆瓧娈佃閲嶈浇锛屽厖褰撶敱鐢ㄦ埛绌洪棿
璁剧疆鐨勮緭鍏ヤ綅鏁扮粍瀛楁锛屼互鎸囩ず鍦ㄤ笅涓€娆￠€€鍑烘椂瑕佸鍒跺嚭鐨勭壒瀹氬瘎瀛樺櫒闆嗐€?

涓轰簡鎸囩ず鐢ㄦ埛绌洪棿宸蹭慨鏀逛簡搴斿鍒惰繘 vCPU 鐨勫€硷紝蹇呴』璁剧疆鎵€鏈夋灦鏋勯€氱敤鐨勪綅鏁扮粍
瀛楁鈥渒vm_dirty_regs鈥濄€傝繖浣跨敤涓庘€渒vm_valid_regs鈥濆瓧娈电浉鍚岀殑浣嶆爣蹇楀畬鎴愩€?
濡傛灉鏈缃?dirty 浣嶏紝鍒欏嵆浣垮瘎瀛樺櫒闆嗗€煎凡琚慨鏀癸紝涔熶笉浼氳澶嶅埗杩?vCPU銆?

浣嶆暟缁勪腑鏈娇鐢ㄧ殑浣嶅瓧娈靛繀椤昏涓洪浂銆?

```

  struct kvm_sync_regs {
        struct kvm_regs regs;
        struct kvm_sregs sregs;
        struct kvm_vcpu_events events;
  };

```
### 6.75 KVM_CAP_PPC_IRQ_XIVE


:Architectures: ppc
:Target: vcpu
:Parameters: args[^0^] 鏄?XIVE 璁惧 fd锛?
             args[^1^] 鏄 vcpu 鐨?XIVE CPU 鍙凤紙server ID锛?

姝よ兘鍔涘皢 vcpu 杩炴帴鍒板唴鏍告€?XIVE 璁惧銆?

### 6.76 KVM_CAP_HYPERV_SYNIC


:Architectures: x86
:Target: vcpu

姝よ兘鍔涳紝鑻?KVM_CHECK_EXTENSION 鎸囩ず鍏跺彲鐢紝鎰忓懗鐫€鍐呮牳瀹炵幇浜?Hyper-V 鍚堟垚
涓柇鎺у埗鍣紙SynIC锛夈€侶yper-V SynIC 鐢ㄤ簬鏀寔鍩轰簬 Windows Hyper-V 鐨勫鎴锋満
鍗婅櫄鎷熷寲椹卞姩锛圴MBus锛夈€?

涓轰簡浣跨敤 SynIC锛屽繀椤婚€氳繃 vcpu fd 涓婄殑 KVM_ENABLE_CAP ioctl 璁剧疆姝よ兘鍔涙潵
婵€娲诲畠銆傛敞鎰忚繖浼氱鐢?APIC 纭欢铏氭嫙鍖栫殑浣跨敤锛堝嵆浣?CPU 鏀寔锛夛紝鍥犱负瀹冧笌
SynIC 鐨勮嚜鍔?EOI 琛屼负涓嶅吋瀹广€?

### 6.77 KVM_CAP_HYPERV_SYNIC2


:Architectures: x86
:Target: vcpu

姝よ兘鍔涘惎鐢ㄦ洿鏂扮増鏈殑 Hyper-V 鍚堟垚涓柇鎺у埗鍣紙SynIC锛夈€備笌 KVM_CAP_HYPERV_SYNIC
鍞竴鐨勫尯鍒槸锛屽綋閫氳繃鍐欏叆鐩稿簲鐨?MSR 鍚敤鏃讹紝KVM 涓嶄細娓呴櫎 SynIC 娑堟伅鍜屼簨浠?
鏍囧織椤点€?

### 6.78 KVM_CAP_HYPERV_DIRECT_TLBFLUSH


:Architectures: x86
:Target: vcpu

姝よ兘鍔涜〃绀鸿繍琛屽湪 Hyper-V 铏氭嫙鏈虹洃鎺у櫒涔嬩笂鐨?KVM 涓哄叾瀹㈡埛鏈哄惎鐢ㄧ洿鎺?TLB
鍒锋柊锛屾剰鍛崇潃 TLB 鍒锋柊瓒呯骇璋冪敤鐢?0 绾ц櫄鎷熸満鐩戞帶鍣紙Hyper-V锛夊鐞嗭紝缁曡繃
KVM銆傜敱浜?Hyper-V 鍜?KVM 涔嬮棿瓒呯骇璋冪敤鍙傛暟鐨?ABI 涓嶅悓锛屽惎鐢ㄦ鑳藉姏浼氭湁鏁?
绂佺敤 KVM 鐨勬墍鏈夎秴绾ц皟鐢ㄥ鐞嗭紙鍥犱负鏌愪簺 KVM 瓒呯骇璋冪敤鍙兘琚?Hyper-V 璇綋浣?
TLB 鍒锋柊瓒呯骇璋冪敤锛夛紝鍥犳鐢ㄦ埛绌洪棿搴斿湪 CPUID 涓鐢?KVM 鏍囪瘑锛屽彧鏆撮湶 Hyper-V
鏍囪瘑銆傚湪杩欑鎯呭喌涓嬶紝瀹㈡埛鏈轰互涓鸿嚜宸辫繍琛屽湪 Hyper-V 涓婏紝骞朵笖鍙娇鐢?Hyper-V
瓒呯骇璋冪敤銆?

### 6.79 KVM_CAP_HYPERV_ENFORCE_CPUID


:Architectures: x86
:Target: vcpu

鍚敤鏃讹紝KVM 灏嗘牴鎹?Hyper-V CPUID 鐗规€у彾涓殑浣嶏紝绂佺敤鎻愪緵缁欏鎴锋満鐨勬ā鎷?
Hyper-V 鐗规€с€傚惁鍒欙紝鍙鍦?HYPERV_CPUID_INTERFACE锛?x40000001锛夊彾涓缃簡
Hyper-V 鏍囪瘑锛屾墍鏈夊綋鍓嶅凡瀹炵幇鐨?Hyper-V 鐗规€ч兘浼氭棤鏉′欢鎻愪緵銆?

### 6.80 KVM_CAP_ENFORCE_PV_FEATURE_CPUID


:Architectures: x86
:Target: vcpu

鍚敤鏃讹紝KVM 灏嗘牴鎹?KVM_CPUID_FEATURES CPUID 鍙讹紙0x40000001锛変腑鐨勪綅锛岀鐢?
鎻愪緵缁欏鎴锋満鐨勫崐铏氭嫙鍖栫壒鎬с€傚惁鍒欙紝瀹㈡埛鏈哄彲鑳戒娇鐢ㄥ崐铏氭嫙鍖栫壒鎬э紝鑰屼笉璁?
瀹為檯閫氳繃 CPUID 鍙舵毚闇蹭簡浠€涔堛€?



## 7. 鍙湪 VM 涓婂惎鐢ㄧ殑鑳藉姏


鏈夋煇浜涜兘鍔涘湪鍚敤鏃朵細鏀瑰彉铏氭嫙鏈虹殑琛屼负銆傝鍚敤瀹冧滑锛岃鍙傞槄 KVM_ENABLE_CAP
涓€鑺傘€備笅闈綘鍙互鎵惧埌涓€浠借兘鍔涘垪琛紝浠ュ強鍚敤瀹冧滑鏃跺 VM 鐨勫奖鍝嶃€?

闅忔弿杩颁竴骞舵彁渚涗互涓嬩俊鎭細

  Architectures锛堟灦鏋勶級锛?
      鍝簺鎸囦护闆嗘灦鏋勬彁渚涙 ioctl銆倄86 鍚屾椂鍖呭惈 i386 鍜?x86_64銆?

  Parameters锛堝弬鏁帮級锛?
      璇ヨ兘鍔涙帴鍙楀摢浜涘弬鏁般€?

  Returns锛堣繑鍥炲€硷級锛?
      杩斿洖鐨勫€笺€傞€氱敤閿欒鐮侊紙EBADF銆丒NOMEM銆丒INVAL锛変笉鍋氳缁嗚鏄庯紝浣嗗叿鏈?
      鐗瑰畾鍚箟鐨勯敊璇細浜堜互璇存槑銆?


### 7.1 KVM_CAP_PPC_ENABLE_HCALL


:Architectures: ppc
:Parameters: args[^0^] 鏄?sPAPR hcall 鍙凤紱
	     args[^1^] 涓?0 琛ㄧず绂佺敤锛? 琛ㄧず鍚敤鍐呮牳鎬佸鐞?

姝よ兘鍔涙帶鍒跺悇涓?sPAPR 瓒呯骇璋冪敤锛坔call锛夋槸鐢卞唴鏍稿鐞嗚繕鏄笉澶勭悊銆傚惎鐢ㄦ垨
绂佺敤鏌愪釜 hcall 鐨勫唴鏍告€佸鐞嗗湪鏁翠釜 VM 鑼冨洿鍐呯敓鏁堛€傚垱寤烘椂锛屼細鍚敤涓€缁勫垵濮?
鐨?hcall 杩涜鍐呮牳鎬佸鐞嗭紝杩欎簺 hcall 鐢卞湪鏈兘鍔涘疄鐜颁箣鍓嶅氨宸茬粡瀹炵幇浜嗗唴鏍告€?
澶勭悊鍑芥暟鐨勯偅浜涜秴绾ц皟鐢ㄧ粍鎴愩€傚鏋滅鐢紝鍐呮牳灏嗕笉浼氬皾璇曞鐞嗚 hcall锛岃€屾槸
鎬绘槸閫€鍑哄埌鐢ㄦ埛绌洪棿澶勭悊瀹冦€傛敞鎰忥紝鍚敤涓€缁勭浉鍏?hcall 涓殑鏌愪簺鑰岀鐢ㄥ彟涓€浜?
鍙兘娌℃湁鎰忎箟锛屼絾 KVM 涓嶄細闃绘鐢ㄦ埛绌洪棿杩欐牱鍋氥€?

濡傛灉鎸囧畾鐨?hcall 鍙蜂笉鏄叿鏈夊唴鏍告€佸疄鐜扮殑閭ｄ釜锛屽垯 KVM_ENABLE_CAP ioctl 灏?
浠?EINVAL 閿欒澶辫触銆?

### 7.2 KVM_CAP_S390_USER_SIGP


:Architectures: s390
:Parameters: none

姝よ兘鍔涙帶鍒跺摢浜?SIGP 椤哄簭灏嗗畬鍏ㄥ湪鐢ㄦ埛绌洪棿澶勭悊銆傚惎鐢ㄦ鑳藉姏鍚庯紝鎵€鏈夊揩閫熼『搴?
灏嗗畬鍏ㄥ湪鍐呮牳涓鐞嗭細

- SENSE
- SENSE RUNNING
- EXTERNAL CALL
- EMERGENCY SIGNAL
- CONDITIONAL EMERGENCY SIGNAL

鎵€鏈夊叾浠栭『搴忓皢瀹屽叏鍦ㄧ敤鎴风┖闂村鐞嗐€?

鍙湁鐗规潈鎿嶄綔寮傚父浼氬湪鍐呮牳涓紙鎴栧湪鎷︽埅涔嬪墠鐨勭‖浠朵腑锛夋鏌ャ€傚鏋滄湭鍚敤姝よ兘鍔涳紝
鍒欎娇鐢ㄦ棫鐨?SIGP 椤哄簭澶勭悊鏂瑰紡锛堥儴鍒嗗湪鍐呮牳銆侀儴鍒嗗湪鐢ㄦ埛绌洪棿锛夈€?

### 7.3 KVM_CAP_S390_VECTOR_REGISTERS


:Architectures: s390
:Parameters: none
:Returns: 0 on success, negative value on error

鍏佽浣跨敤闅?z13 澶勭悊鍣ㄥ紩鍏ョ殑鍚戦噺瀵勫瓨鍣紝骞朵负涓绘満鍜岀敤鎴风┖闂翠箣闂寸殑鍚屾鎻愪緵鏀寔銆?
濡傛灉鏈哄櫒涓嶆敮鎸佸悜閲忥紝灏嗚繑鍥?-EINVAL銆?

### 7.4 KVM_CAP_S390_USER_STSI


:Architectures: s390
:Parameters: none

姝よ兘鍔涘厑璁?STSI 鎸囦护鐨勫悗澶勭悊鍣ㄣ€傚湪鍐呮牳涓垵姝ュ鐞嗕箣鍚庯紝KVM 浠?KVM_EXIT_S390_STSI
閫€鍑哄埌鐢ㄦ埛绌洪棿锛屼互鍏佽鐢ㄦ埛绌洪棿鎻掑叆杩涗竴姝ョ殑鏁版嵁銆?

鍦ㄩ€€鍑哄埌鐢ㄦ埛绌洪棿涔嬪墠锛宬vm 澶勭悊鍣ㄥ簲濉厖 kvm_run 鐨?s390_stsi 瀛楁锛?

```

  struct {
	__u64 addr;
	__u8 ar;
	__u8 reserved;
	__u8 fc;
	__u8 sel1;
	__u16 sel2;
  } s390_stsi;

  @addr - STSI SYSIB 鐨勫鎴锋満鍦板潃
  @fc   - 鍔熻兘鐮?
  @sel1 - 閫夋嫨鍣?1
  @sel2 - 閫夋嫨鍣?2
  @ar   - 璁块棶瀵勫瓨鍣ㄥ彿

```
KVM 澶勭悊鍣ㄥ簲浠?rc = -EREMOTE 閫€鍑哄埌鐢ㄦ埛绌洪棿銆?

### 7.5 KVM_CAP_SPLIT_IRQCHIP


:Architectures: x86
:Parameters: args[^0^] - 涓虹敤鎴风┖闂?IOAPIC 淇濈暀鐨勮矾鐢辨暟
:Returns: 0 on success, -1 on error

鍦ㄥ唴鏍镐腑涓烘瘡涓鐞嗗櫒鍒涘缓涓€涓湰鍦?apic銆傚鏋滅敤鎴风┖闂?VMM 甯屾湜妯℃嫙 IOAPIC 鍜?
PIC锛堜互鍙?PIT锛屽敖绠?PIT 蹇呴』鍗曠嫭鍚敤锛夛紝鍙互鐢ㄥ畠鏇夸唬 KVM_CREATE_IRQCHIP銆?

姝よ兘鍔涜繕鍚敤浜嗗唴鏍告€佺殑涓柇璇锋眰璺敱锛涘綋鍚敤 KVM_CAP_SPLIT_IRQCHIP 鏃讹紝IRQ
璺敱琛ㄤ腑鍙娇鐢?KVM_IRQ_ROUTING_MSI 绫诲瀷鐨勮矾鐢便€傚墠 args[^0^] 涓?MSI 璺敱涓?
IOAPIC 寮曡剼淇濈暀銆傛瘡褰?LAPIC 鏀跺埌杩欎簺璺敱鐨?EOI 鏃讹紝灏变細鍚戠敤鎴风┖闂存姤鍛婁竴涓?
KVM_EXIT_IOAPIC_EOI vmexit銆?

濡傛灉宸插垱寤轰簡 VCPU锛屾垨鑰?irqchip 宸茬粡鍦ㄥ唴鏍镐腑锛堝嵆宸茬粡璋冪敤杩?
KVM_CREATE_IRQCHIP锛夛紝鍒欏け璐ャ€?

### 7.6 KVM_CAP_S390_RI


:Architectures: s390
:Parameters: none

鍏佽浣跨敤闅?zEC12 澶勭悊鍣ㄥ紩鍏ョ殑杩愯鏃舵寚浠わ紙runtime-instrumentation锛夈€傚鏋?
鏈哄櫒涓嶆敮鎸佽繍琛屾椂鎸囦护锛屽皢杩斿洖 -EINVAL銆傚鏋滃凡鍒涘缓浜?VCPU锛屽皢杩斿洖 -EBUSY銆?
### 7.7 KVM_CAP_X2APIC_API


:Architectures: x86
:Parameters: args[^0^] - 搴斿惎鐢ㄧ殑鐗规€?
:Returns: 0 on success, -EINVAL when args[^0^] contains invalid features

```

  #define KVM_X2APIC_API_USE_32BIT_IDS                          (1ULL << 0)
  #define KVM_X2APIC_API_DISABLE_BROADCAST_QUIRK                (1ULL << 1)
  #define KVM_X2APIC_ENABLE_SUPPRESS_EOI_BROADCAST              (1ULL << 2)
  #define KVM_X2APIC_DISABLE_SUPPRESS_EOI_BROADCAST             (1ULL << 3)

```
鍚敤 KVM_X2APIC_API_USE_32BIT_IDS 鏀瑰彉浜?KVM_SET_GSI_ROUTING銆並VM_SIGNAL_MSI銆?
KVM_SET_LAPIC 鍜?KVM_GET_LAPIC 鐨勮涓猴紝鍏佽浣跨敤 32 浣?APIC ID銆傝鍙傞槄鍚勮嚜
绔犺妭涓殑 KVM_CAP_X2APIC_API銆?

蹇呴』鍚敤 KVM_X2APIC_API_DISABLE_BROADCAST_QUIRK锛寈2APIC 鎵嶈兘鍦ㄩ€昏緫妯″紡鎴?
瓒呰繃 255 涓?VCPU 鐨勬儏鍐典笅宸ヤ綔銆傚惁鍒欙紝鍗充娇鍦?x2APIC 妯″紡涓嬶紝KVM 涔熶細鎶?0xff
褰撲綔骞挎挱锛屼互鏀寔娌℃湁涓柇閲嶆槧灏勭殑鐗╃悊 x2APIC銆傝繖鍦ㄩ€昏緫妯″紡涓嬫槸涓嶅彲鍙栫殑锛屽洜涓?
0xff 琛ㄧず cluster 0 涓殑 CPU 0-7銆?

璁剧疆 KVM_X2APIC_ENABLE_SUPPRESS_EOI_BROADCAST 鎸囩ず KVM 鍚敤鎶戝埗 EOI 骞挎挱
锛圫uppress EOI Broadcasts锛夈€傚綋瀹㈡埛鏈哄湪 SPIV 瀵勫瓨鍣ㄤ腑璁剧疆浜嗘姂鍒?EOI 骞挎挱浣嶆椂锛?
KVM 浼氬悜瀹㈡埛鏈洪€氬憡瀵规姂鍒?EOI 骞挎挱鐨勬敮鎸侊紝骞跺湪瀹㈡埛鏈鸿缃浣嶆椂鎶戝埗 LAPIC
鐨?EOI 骞挎挱銆傛鏍囧織浠呭湪浣跨敤 split IRQCHIP 鏃跺彈鏀寔銆?

璁剧疆 KVM_X2APIC_DISABLE_SUPPRESS_EOI_BROADCAST 鍒欏畬鍏ㄧ鐢ㄥ鎶戝埗 EOI 骞挎挱鐨?
鏀寔锛屽嵆鎸囩ず KVM 涓嶈鍚戝鎴锋満閫氬憡鏀寔銆?

鐜颁唬 VMM 搴斿綋鍚敤 KVM_X2APIC_ENABLE_SUPPRESS_EOI_BROADCAST 鎴?
KVM_X2APIC_DISABLE_SUPPRESS_EOI_BROADCAST 涔嬩竴銆傚惁鍒欏皢浣跨敤 KVM 鐨勯仐鐣欏彜鎬?
琛屼负锛氬湪 split IRQCHIP 妯″紡涓嬶紝KVM 浼氬悜瀹㈡埛鏈洪€氬憡瀵规姂鍒?EOI 骞挎挱鐨勬敮鎸侊紝
浣嗗疄闄呬笂骞朵笉鎶戝埗 EOI 骞挎挱锛涘湪鍐呮牳鎬?IRQCHIP 妯″紡涓嬶紝KVM 涓嶄細閫氬憡瀵规姂鍒?EOI
骞挎挱鐨勬敮鎸併€?

鍚屾椂璁剧疆 KVM_X2APIC_ENABLE_SUPPRESS_EOI_BROADCAST 鍜?
KVM_X2APIC_DISABLE_SUPPRESS_EOI_BROADCAST 灏嗕互 EINVAL 閿欒澶辫触锛屽湪鏈娇鐢?
split IRQCHIP 鐨勬儏鍐典笅璁剧疆 KVM_X2APIC_ENABLE_SUPPRESS_EOI_BROADCAST 鍚屾牱浼?
澶辫触銆?

### 7.8 KVM_CAP_S390_USER_INSTR0


:Architectures: s390
:Parameters: none

鍚敤姝よ兘鍔涘悗锛岄潪娉曠殑鎸囦护 0x0000锛? 瀛楄妭锛夊皢琚嫤鎴苟杞彂鍒扮敤鎴风┖闂淬€傜敤鎴风┖闂?
鍙互鍒╃敤姝ゆ満鍒跺疄鐜颁緥濡?2 瀛楄妭杞欢鏂偣銆傚唴鏍镐笉浼氫负杩欎簺鎸囦护娉ㄥ叆鎿嶄綔寮傚父锛?
鐢ㄦ埛绌洪棿蹇呴』鑷澶勭悊銆?

鍗充娇鍦?VCPU 宸茶鍒涘缓骞舵鍦ㄨ繍琛岀殑鎯呭喌涓嬶紝涔熷彲浠ュ姩鎬佸惎鐢ㄦ鑳藉姏銆?

### 7.9 KVM_CAP_S390_GS


:Architectures: s390
:Parameters: none
:Returns: 0 on success; -EINVAL if the machine does not support
          guarded storage; -EBUSY if a VCPU has already been created.

鍏佽 KVM 瀹㈡埛鏈轰娇鐢ㄥ畧鎶ゅ瓨鍌紙guarded storage锛夈€?

### 7.10 KVM_CAP_S390_AIS


:Architectures: s390
:Parameters: none

鍏佽浣跨敤閫傞厤鍣ㄤ腑鏂姂鍒讹紙adapter-interruption suppression锛夈€?
:Returns: 0 on success; -EBUSY if a VCPU has already been created.

### 7.11 KVM_CAP_PPC_SMT


:Architectures: ppc
:Parameters: vsmt_mode, flags

鍦?VM 涓婂惎鐢ㄦ鑳藉姏涓虹敤鎴风┖闂存彁渚涗簡涓€绉嶈缃湡鏈涚殑铏氭嫙 SMT 妯″紡锛堝嵆姣忎釜铏氭嫙
鏍稿績鐨勮櫄鎷?CPU 鏁帮級鐨勬柟娉曘€傝櫄鎷?SMT 妯″紡 vsmt_mode 蹇呴』鏄?1 鍒?8 涔嬮棿鐨?2 鐨?
骞傘€傚湪 POWER8 涓婏紝vsmt_mode 杩樹笉寰楀ぇ浜庡涓绘瘡涓瓙鏍哥殑绾跨▼鏁般€傚綋鍓?flags 蹇呴』
涓?0銆傛垚鍔熻皟鐢ㄤ互鍚敤姝よ兘鍔涘悗锛屽綋闅忓悗涓?VM 鏌ヨ KVM_CAP_PPC_SMT 鑳藉姏鏃讹紝灏?
杩斿洖 vsmt_mode銆傛鑳藉姏浠呯敱 HV KVM 鏀寔锛屽苟涓斿彧鑳藉湪鍒涘缓浠讳綍 VCPU 涔嬪墠璁剧疆銆?
KVM_CAP_PPC_SMT_POSSIBLE 鑳藉姏鎸囩ず鍝簺铏氭嫙 SMT 妯″紡鍙敤銆?

### 7.12 KVM_CAP_PPC_FWNMI


:Architectures: ppc
:Parameters: none

鍊熷姪姝よ兘鍔涳紝瀹㈡埛鏈哄湴鍧€绌洪棿涓殑鏈哄櫒妫€鏌ュ紓甯稿皢瀵艰嚧 KVM 浠?NMI 閫€鍑哄師鍥犻€€鍑?
瀹㈡埛鏈恒€傝繖浣垮緱 QEMU 鑳藉鏋勫缓閿欒鏃ュ織骞惰烦杞埌瀹㈡埛鏈哄唴鏍告敞鍐岀殑鏈哄櫒妫€鏌ュ鐞?
渚嬬▼銆傝嫢娌℃湁姝よ兘鍔涳紝KVM 灏嗚烦杞埌瀹㈡埛鏈虹殑 0x200 涓柇鍚戦噺銆?

### 7.13 KVM_CAP_X86_DISABLE_EXITS


:Architectures: x86
:Parameters: args[^0^] 瀹氫箟绂佺敤鍝簺閫€鍑?
:Returns: 0 on success, -EINVAL when args[^0^] contains invalid exits
          or if any vCPUs have already been created

```

  #define KVM_X86_DISABLE_EXITS_MWAIT            (1 << 0)
  #define KVM_X86_DISABLE_EXITS_HLT              (1 << 1)
  #define KVM_X86_DISABLE_EXITS_PAUSE            (1 << 2)
  #define KVM_X86_DISABLE_EXITS_CSTATE           (1 << 3)
  #define KVM_X86_DISABLE_EXITS_APERFMPERF       (1 << 4)

```
鍦?VM 涓婂惎鐢ㄦ鑳藉姏涓虹敤鎴风┖闂存彁渚涗簡涓€绉嶄笉鍐嶆嫤鎴煇浜涙寚浠ょ殑鏂规硶锛屼粠鑰屽湪鏌愪簺
宸ヤ綔璐熻浇涓嬫敼鍠勫欢杩燂紝寤鸿鍦?vCPU 鍏宠仈鍒颁笓鐢ㄧ墿鐞?CPU 鏃朵娇鐢ㄣ€傛湭鏉ュ彲浠ユ坊鍔犳洿澶?
浣嶏紱鐢ㄦ埛绌洪棿鍙渶灏?KVM_CHECK_EXTENSION 鐨勭粨鏋滀紶缁?KVM_ENABLE_CAP 鍗冲彲绂佺敤
鎵€鏈夋绫?vmexit銆?

濡傛灉绂佺敤浜?HLT 閫€鍑猴紝璇峰嬁鍚敤 KVM_FEATURE_PV_UNHALT銆?

铏氭嫙鍖?`IA32_APERF` 鍜?`IA32_MPERF` MSR 闇€瑕佺殑涓嶄粎浠呮槸绂佺敤 APERF/MPERF 閫€鍑恒€?
铏界劧 Intel 鍜?AMD 閮借褰曚簡杩欎簺 MSR 鐨勪弗鏍间娇鐢ㄦ潯浠垛€斺€斿己璋冨彧鏈夊畠浠湪涓€娈垫椂闂?
鍖洪棿锛圱0 鍒?T1锛夊唴澧為噺鐨勬瘮鍊煎湪鏋舵瀯涓婃湁瀹氫箟鈥斺€斾絾绠€鍗曞湴閫忎紶杩欎簺 MSR 浠嶅彲鑳?
浜х敓涓嶆纭殑姣斿€笺€?

濡傛灉鍦?T0 鍜?T1 涔嬮棿鍙戠敓浠ヤ笅鎯呭喌锛屽氨鍙兘鍑虹幇杩欎釜閿欒鐨勬瘮鍊硷細

1. vCPU 绾跨▼鍦ㄩ€昏緫澶勭悊鍣ㄤ箣闂磋縼绉汇€?
2. 鍙戠敓瀹炴椂杩佺Щ鎴栨寕璧?鎭㈠鎿嶄綔銆?
3. 鍙︿竴涓换鍔″叡浜?vCPU 鐨勯€昏緫澶勭悊鍣ㄣ€?
4. 妯℃嫙浜嗕綆浜?C0 鐨?C-state锛堜緥濡傞€氳繃 HLT 鎷︽埅锛夈€?
5. 瀹㈡埛鏈?TSC 棰戠巼涓庡涓?TSC 棰戠巼涓嶅尮閰嶃€?

鐢变簬杩欎簺澶嶆潅鎬э紝KVM 涓嶄細鑷姩灏嗘閫忎紶鑳藉姏涓庡鎴锋満 CPUID 浣?
`CPUID.6:ECX.APERFMPERF[bit 0]` 鐩稿叧鑱斻€傝涓烘鏈哄埗瓒充互铏氭嫙鍖?`IA32_APERF`
鍜?`IA32_MPERF` MSR 鐨勭敤鎴风┖闂?VMM 蹇呴』鏄惧紡璁剧疆瀹㈡埛鏈?CPUID 浣嶃€?


### 7.14 KVM_CAP_S390_HPAGE_1M


:Architectures: s390
:Parameters: none
:Returns: 0 on success, -EINVAL if hpage module parameter was not set
	  or cmma is enabled, or the VM has the KVM_VM_S390_UCONTROL
	  flag set

鍊熷姪姝よ兘鍔涳紝鍙互涓?VM 鍚敤 KVM 瀵归€氳繃 hugetlbfs 鐢?1M 椤靛仛鍐呭瓨鍚庣鐨勬敮鎸併€?
鍚敤璇ヨ兘鍔涘悗锛宑mma 涓嶈兘鍐嶈鍚敤锛宲fmfi 鍜屽瓨鍌ㄩ敭瑙ｉ噴涔熻绂佺敤銆傚鏋?cmma 宸茬粡
琚惎鐢ㄦ垨鑰?hpage 妯″潡鍙傛暟鏈涓?1锛屽垯杩斿洖 -EINVAL銆?

铏界劧閫氬父鍙互鍦ㄦ病鏈夋鑳藉姏鐨勬儏鍐典笅鍒涘缓浣跨敤澶ч〉鍚庣鐨?VM锛屼絾 VM 灏嗕笉鑳借繍琛屻€?

### 7.15 KVM_CAP_MSR_PLATFORM_INFO


:Architectures: x86
:Parameters: args[^0^] 鐗规€ф槸鍚﹀簲鍚敤

鍊熷姪姝よ兘鍔涳紝瀹㈡埛鏈哄彲浠ヨ鍙?MSR_PLATFORM_INFO MSR銆傚惁鍒欙紝褰撳鎴锋満灏濊瘯璁块棶鏃朵細
寮曞彂 #GP銆傚綋鍓嶏紝姝よ兘鍔涗笉鍚敤璇?MSR 瀵瑰鎴锋満鐨勫啓鍏ユ潈闄愩€?

### 7.16 KVM_CAP_PPC_NESTED_HV


:Architectures: ppc
:Parameters: none
:Returns: 0 on success, -EINVAL when the implementation doesn't support
	  nested-HV virtualization.

POWER9 鍙婁互鍚庣郴缁熶笂鐨?HV-KVM 鍏佽鈥滃祵濂?HV鈥濊櫄鎷熷寲锛屽畠涓哄鎴风殑瀹㈡埛鏈猴紙guest
VM锛夋彁渚涗簡涓€绉嶈兘澶熶娇鐢?CPU 瓒呯骇visor 妯″紡锛堢壒鏉冮潪铏氭嫙鏈虹洃鎺у櫒鐘舵€侊級杩愯鐨?
鏂瑰紡銆傚湪 VM 涓婂惎鐢ㄦ鑳藉姏鍙栧喅浜?CPU 鏄惁鍏锋湁蹇呰鐨勫姛鑳斤紝浠ュ強璇ヨ鏂芥槸鍚﹂€氳繃
kvm-hv 妯″潡鍙傛暟鍚敤銆?

### 7.17 KVM_CAP_EXCEPTION_PAYLOAD


:Architectures: x86
:Parameters: args[^0^] 鐗规€ф槸鍚﹀簲鍚敤

鍚敤姝よ兘鍔涘悗锛屽綋 L1 鎷︽埅鍙戠敓鍦?L2 涓殑 #PF 寮傚父鏃讹紝鍦ㄦā鎷熺殑 VM-exit 涔嬪墠涓嶄細
淇敼 CR2銆傜被浼煎湴锛屼粎瀵?kvm-intel锛屽綋 L1 鎷︽埅鍙戠敓鍦?L2 涓殑 #DB 寮傚父鏃讹紝鍦?
妯℃嫙鐨?VM-exit 涔嬪墠涓嶄細淇敼 DR6銆傚洜姝わ紝褰?KVM_GET_VCPU_EVENTS 鎶ュ憡 L2 鏈変竴涓?
鎸傝捣鐨?#PF锛堟垨 #DB锛夊紓甯告椂锛宔xception.has_payload 灏嗚缃綅锛屽苟涓旀晠闅滃湴鍧€锛堟垨
鏂扮殑 DR6 浣峔*锛夊皢鎶ュ憡鍦?exception_payload 瀛楁涓€傜被浼煎湴锛屽綋鐢ㄦ埛绌洪棿浣跨敤
KVM_SET_VCPU_EVENTS 鍚?L2 娉ㄥ叆涓€涓?#PF锛堟垨 #DB锛夋椂锛屽簲缃綅
exception.has_payload锛屽苟灏嗘晠闅滃湴鍧€鈥斺€旀垨鏂扮殑 DR6 浣峔 [#]_鈥斺€旀斁鍏?exception_payload
瀛楁銆?

姝よ兘鍔涜繕鍚敤浜?struct kvm_vcpu_events 涓殑 exception.pending锛岃繖鍏佽鐢ㄦ埛绌洪棿
鍖哄垎鎸傝捣鐨勫紓甯稿拰娉ㄥ叆鐨勫紓甯搞€?


       will clear DR6.RTM.

### 7.18 KVM_CAP_MANUAL_DIRTY_LOG_PROTECT2


:Architectures: x86, arm64, mips
:Parameters: args[^0^] 鐗规€ф槸鍚﹀簲鍚敤

```

  #define KVM_DIRTY_LOG_MANUAL_PROTECT_ENABLE   (1 << 0)
  #define KVM_DIRTY_LOG_INITIALLY_SET           (1 << 1)

```
璁剧疆浜?KVM_DIRTY_LOG_MANUAL_PROTECT_ENABLE 鏃讹紝KVM_GET_DIRTY_LOG 涓嶄細鑷姩娓呴櫎
骞跺啓淇濇姢鎵€鏈変綔涓鸿剰椤佃繑鍥炵殑鍐呭瓨椤点€傜浉鍙嶏紝鐢ㄦ埛绌洪棿蹇呴』浣跨敤 KVM_CLEAR_DIRTY_LOG
鍗曠嫭鎵ц姝ゆ搷浣溿€?

浠ョ暐寰洿澶嶆潅鐨勬搷浣滀负浠ｄ环锛岃繖鍦ㄤ袱鏂归潰鎻愪緵浜嗘洿濂界殑鍙墿灞曟€у拰鍝嶅簲鎬с€傞鍏堬紝
KVM_CLEAR_DIRTY_LOG ioctl 鍙互浠?64 椤电殑绮掑害鎿嶄綔锛岃€屼笉闇€瑕佸悓姝ユ暣涓?memslot锛?
杩欑‘淇濅簡 KVM 涓嶄細闀挎椂闂存寔鏈夊叧鑷棆閿併€傚叾娆★紝鍦ㄦ煇浜涙儏鍐典笅锛屽湪璋冪敤
KVM_GET_DIRTY_LOG 鍜岀敤鎴风┖闂村疄闄呬娇鐢ㄩ〉涓暟鎹箣闂翠細缁忚繃澶ч噺鏃堕棿銆傚湪姝ゆ湡闂撮〉
鍙兘琚慨鏀癸紝杩欏瀹㈡埛鏈哄拰鐢ㄦ埛绌洪棿閮芥槸浣庢晥鐨勶細瀹㈡埛鏈哄皢鍥犲啓淇濇姢鏁呴殰鑰屾壙鍙楁洿楂樼殑
鎯╃綒锛岃€岀敤鎴风┖闂村彲鑳界湅鍒拌剰椤电殑璇姤銆傛墜鍔ㄩ噸鏂颁繚鎶ゆ湁鍔╀簬鍑忓皯杩欐鏃堕棿锛屾敼鍠勫鎴锋満
鎬ц兘骞跺噺灏戣剰鏃ュ織鐨勫亣闃虫€ф暟閲忋€?

璁剧疆浜?KVM_DIRTY_LOG_INITIALLY_SET 鏃讹紝鑴忎綅鍥剧殑鎵€鏈変綅鍦ㄥ垱寤烘椂閮藉垵濮嬪寲涓?1銆?
杩欎篃鏀瑰杽浜嗘€ц兘锛屽洜涓鸿剰鏃ュ織鍙互鍦ㄩ娆¤皟鐢?KVM_CLEAR_DIRTY_LOG 鏃朵互灏忓潡閫愭
鍚敤銆侹VM_DIRTY_LOG_INITIALLY_SET 渚濊禆
KVM_DIRTY_LOG_MANUAL_PROTECT_ENABLE锛堢洰鍓嶅畠涔熷彧鍦?x86銆乤rm64 鍜?riscv 涓婂彲鐢級銆?

KVM_CAP_MANUAL_DIRTY_LOG_PROTECT2 姝ゅ墠鏇句互 KVM_CAP_MANUAL_DIRTY_LOG_PROTECT
涔嬪悕鎻愪緵锛屼絾鍏跺疄鐜板瓨鍦ㄧ己闄凤紝瀵艰嚧闅句互鎴栨棤娉曟纭娇鐢ㄣ€傛彁渚?
KVM_CAP_MANUAL_DIRTY_LOG_PROTECT2 鍗宠〃绀鸿繖浜涚己闄峰凡琚慨澶嶃€傜敤鎴风┖闂翠笉搴斿皾璇?
浣跨敤 KVM_CAP_MANUAL_DIRTY_LOG_PROTECT銆?

### 7.19 KVM_CAP_PPC_SECURE_GUEST


:Architectures: ppc

姝よ兘鍔涜〃绀?KVM 姝ｈ繍琛屽湪鎷ユ湁 ultravisor 鍥轰欢銆佸洜鑰岃兘澶熸敮鎸佸畨鍏ㄥ鎴锋満鐨勪富鏈轰笂銆?
鍦ㄨ繖鏍风殑绯荤粺涓婏紝瀹㈡埛鏈哄彲浠ヨ姹?ultravisor 浣垮叾鎴愪负瀹夊叏瀹㈡埛鏈猴紝鍏跺唴瀛樺湪瀹㈡埛鏈?
涔嬪瀵瑰涓讳笉鍙闂紝闄ら潪鏄樉寮忚姹備笌瀹㈡埛鏈哄叡浜殑椤点€傚綋瀹㈡埛鏈鸿姹傛垚涓哄畨鍏ㄥ鎴锋満
鏃讹紝ultravisor 浼氶€氱煡 KVM锛孠VM 鏈夋満浼氬惁鍐宠繖涓€杞崲銆?

濡傛灉瀛樺湪锛屾鑳藉姏鍙互涓?VM 鍚敤锛屾剰鍛崇潃 KVM 灏嗗厑璁歌浆鎹㈠埌瀹夊叏瀹㈡埛鏈烘ā寮忋€傚惁鍒?
KVM 灏嗗惁鍐宠杞崲銆?

### 7.20 KVM_CAP_HALT_POLL


:Architectures: all
:Target: VM
:Parameters: args[^0^] 鏄互绾崇涓哄崟浣嶇殑鏈€澶ц疆璇㈡椂闂?
:Returns: 0 on success; -1 on error

KVM_CAP_HALT_POLL 瑕嗙洊 kvm.halt_poll_ns 妯″潡鍙傛暟锛屼互璁剧疆鐩爣 VM 涓墍鏈?vCPU 鐨?
鏈€澶ф殏鍋滆疆璇紙halt-polling锛夋椂闂淬€傛鑳藉姏鍙互鍦ㄤ换浣曟椂闂淬€佷换鎰忔鏁拌皟鐢紝浠ュ姩鎬?
鏇存敼鏈€澶ф殏鍋滆疆璇㈡椂闂淬€?

鏈夊叧鏆傚仠杞鐨勬洿澶氫俊鎭紝璇峰弬闃?Documentation/virt/kvm/halt-polling.rst銆?

### 7.21 KVM_CAP_X86_USER_SPACE_MSR


:Architectures: x86
:Target: VM
:Parameters: args[^0^] 鍖呭惈瑕佹姤鍛婄殑 KVM_MSR_EXIT_REASON_* 浜嬩欢鎺╃爜
:Returns: 0 on success; -1 on error

姝よ兘鍔涘厑璁哥敤鎴风┖闂村湪 MSR 璁块棶琚嫆缁濇椂鎷︽埅 RDMSR 鍜?WRMSR 鎸囦护銆傞粯璁ゆ儏鍐典笅锛?
KVM 鍦ㄨ鎷掔粷鐨勮闂笂娉ㄥ叆 #GP銆?

褰撳鎴锋満璇锋眰璇诲彇鎴栧啓鍏ユ煇涓?MSR 鏃讹紝KVM 鍙兘鏃犳硶瀹炵幇涓庣浉搴旂郴缁熺浉鍏崇殑鎵€鏈?MSR銆?
瀹冧篃涓嶄細鎸?CPU 绫诲瀷鍖哄垎銆?

涓轰簡瀵?MSR 澶勭悊杩涜鏇寸粏绮掑害鐨勬帶鍒讹紝鐢ㄦ埛绌洪棿鍙互鍚敤姝よ兘鍔涖€傚惎鐢ㄥ悗锛屽尮閰?
args[^0^] 涓寚瀹氭帺鐮併€佸苟涓斾細鍦ㄥ鎴锋満鍐呰Е鍙?#GP 鐨?MSR 璁块棶灏嗘敼涓鸿Е鍙?
KVM_EXIT_X86_RDMSR 鍜?KVM_EXIT_X86_WRMSR 閫€鍑洪€氱煡銆傜劧鍚庣敤鎴风┖闂村彲浠ュ疄鐜扮壒瀹?
鍨嬪彿鐨?MSR 澶勭悊锛屽拰/鎴栧悜鐢ㄦ埛鍙戝嚭閫氱煡锛屽憡鐭ユ煇涓?MSR 鏈 KVM 妯℃嫙/铏氭嫙鍖栥€?

鏈夋晥鐨勬帺鐮佹爣蹇椾负锛?

============================ ===============================================
 KVM_MSR_EXIT_REASON_UNKNOWN 鎷︽埅瀵癸紙KVM 鏈煡鐨勶級MSR 鐨勮闂?
 KVM_MSR_EXIT_REASON_INVAL   鎷︽埅鏍规嵁 vCPU 鍨嬪彿鍜?鎴栨ā寮忓湪鏋舵瀯涓婇潪娉曠殑璁块棶
 KVM_MSR_EXIT_REASON_FILTER  鎷︽埅琚敤鎴风┖闂撮€氳繃 KVM_X86_SET_MSR_FILTER 鎷掔粷鐨勮闂?
============================ ===============================================

### 7.22 KVM_CAP_X86_BUS_LOCK_EXIT


:Architectures: x86
:Target: VM
:Parameters: args[^0^] 瀹氫箟瀹㈡埛鏈轰腑妫€娴嬪埌鎬荤嚎閿佹椂浣跨敤鐨勭瓥鐣?
:Returns: 0 on success, -EINVAL when args[^0^] contains invalid bits

```

  #define KVM_BUS_LOCK_DETECTION_OFF      (1 << 0)
  #define KVM_BUS_LOCK_DETECTION_EXIT     (1 << 1)

```
鍦?VM 涓婂惎鐢ㄦ鑳藉姏涓虹敤鎴风┖闂存彁渚涗簡涓€绉嶉€夋嫨绛栫暐鏉ュ鐞嗗鎴锋満涓娴嬪埌鐨勬€荤嚎閿?
鐨勬柟娉曘€傜敤鎴风┖闂村彲浠ヤ粠 KVM_CHECK_EXTENSION 鐨勭粨鏋滀腑鑾峰彇鍙楁敮鎸佺殑妯″紡锛屽苟閫氳繃
KVM_ENABLE_CAP 杩涜瀹氫箟銆傚彈鏀寔鐨勬ā寮忔槸浜掓枼鐨勩€?

姝よ兘鍔涘厑璁哥敤鎴风┖闂村己鍒跺湪瀹㈡埛鏈轰腑妫€娴嬪埌鐨勬€荤嚎閿佷笂鍙戠敓 VM 閫€鍑猴紝鏃犺瀹夸富鏄惁
鍚敤浜?split-lock 妫€娴嬶紙鍚庤€呬細瑙﹀彂 KVM 鎷︽埅鐨?#AC 寮傚父锛夈€傛鑳藉姏鏃ㄥ湪缂撹В鎭舵剰/
鏈?bug 鐨勫鎴锋満鍒╃敤鎬荤嚎閿侀檷浣庢暣涓郴缁熸€ц兘鐨勬敾鍑汇€?

濡傛灉璁剧疆浜?KVM_BUS_LOCK_DETECTION_OFF锛孠VM 涓嶄細寮哄埗瀹㈡埛鏈烘€荤嚎閿佸彂鐢?VM 閫€鍑猴紝
灏界瀹夸富鍐呮牳鐨?split-lock #AC 妫€娴嬶紙濡傛灉鍚敤锛変粛鐒堕€傜敤銆?

濡傛灉璁剧疆浜?KVM_BUS_LOCK_DETECTION_EXIT锛孠VM 浼氬惎鐢ㄤ竴涓?CPU 鐗规€э紝纭繚瀹㈡埛鏈轰腑
鐨勬€荤嚎閿佽Е鍙?VM 閫€鍑猴紝骞朵笖 KVM 涓烘墍鏈夋绫?VM 閫€鍑洪€€鍑哄埌鐢ㄦ埛绌洪棿锛屼緥濡傚厑璁哥敤鎴?
绌洪棿瀵硅繚瑙勭殑瀹㈡埛鏈鸿繘琛岄檺娴佸拰/鎴栧簲鐢ㄥ叾浠栧熀浜庣瓥鐣ョ殑缂撹В鎺柦銆傞€€鍑哄埌鐢ㄦ埛绌洪棿鏃讹紝
KVM 鍦?vcpu-run->flags 涓缃?KVM_RUN_X86_BUS_LOCK锛屽苟鏈夋潯浠跺湴灏?exit_reason 璁句负
KVM_EXIT_X86_BUS_LOCK銆?

鐢变簬搴曞眰纭欢瀹炵幇鐨勫樊寮傦紝閫€鍑烘椂 vCPU 鐨?RIP 鍦?Intel 鍜?AMD 涔嬮棿鏈夋墍涓嶅悓銆傚湪
Intel 瀹夸富涓婏紝RIP 鎸囧悜涓嬩竴鏉℃寚浠わ紝鍗抽€€鍑烘槸闄烽槺寮忕殑锛坱rap-like锛夈€傚湪 AMD 瀹夸富涓婏紝
RIP 鎸囧悜杩濊鎸囦护锛屽嵆閫€鍑烘槸鏁呴殰寮忕殑锛坒ault-like锛夈€?

娉ㄦ剰锛佹娴嬪埌鐨勬€荤嚎閿佸彲鑳戒笌鍏朵粬閫€鍑哄埌鐢ㄦ埛绌洪棿鍚屾椂鍙戠敓锛屽嵆濡傛灉鐢ㄦ埛绌洪棿甯屾湜瀵?
鎵€鏈夋娴嬪埌鐨勬€荤嚎閿侀噰鍙栬鍔紝鍒欏簲妫€鏌?KVM_RUN_X86_BUS_LOCK锛岃€屼笉璁轰富閫€鍑哄師鍥?
涓轰綍銆?

### 7.23 KVM_CAP_PPC_DAWR1


:Architectures: ppc
:Parameters: none
:Returns: 0 on success, -EINVAL when CPU doesn't support 2nd DAWR

姝よ兘鍔涘彲鐢ㄤ簬妫€鏌?鍚敤鐢?POWER10 澶勭悊鍣ㄦ彁渚涚殑绗?2 涓?DAWR 鐗规€с€?


### 7.24 KVM_CAP_VM_COPY_ENC_CONTEXT_FROM


:Architectures: x86 SEV enabled
:Type: vm
:Parameters: args[^0^] 鏄簮 vm 鐨?fd
:Returns: 0 on success; ENOTTY on error

姝よ兘鍔涘厑璁哥敤鎴风┖闂村皢鍔犲瘑涓婁笅鏂囦粠鐢辫 fd 鎸囩ず鐨?vm 澶嶅埗鍒拌皟鐢ㄦ鑳藉姏鐨?vm 涓娿€?

杩欐棬鍦ㄦ敮鎸佺敱瀹夸富璋冨害鐨勫鎴锋満鍐呭伐浣滆礋杞姐€傝繖浣垮緱瀹㈡埛鏈哄唴宸ヤ綔璐熻浇鑳藉缁存姢鍏惰嚜韬殑
NPT锛屽苟浣夸袱涓?vm 涓嶄細鍥犱负涓柇绛夎€屾剰澶栦簰鐩哥牬鍧忥紙鐙珛鐨?APIC/MSR 绛夛級銆?

### 7.25 KVM_CAP_SGX_ATTRIBUTE


:Architectures: x86
:Target: VM
:Parameters: args[^0^] 鏄?securityfs 涓?SGX 灞炴€ф枃浠剁殑鏂囦欢鍙ユ焺
:Returns: 0 on success, -EINVAL if the file handle is invalid or if a requested
          attribute is not supported by KVM.

KVM_CAP_SGX_ATTRIBUTE 浣跨敤鎴风┖闂?VMM 鑳藉鎺堜簣 VM 瀵逛竴涓垨澶氫釜鐗规潈椋炲湴锛坋nclave锛?
灞炴€х殑璁块棶鏉冮檺銆俛rgs[^0^] 蹇呴』鎸佹湁涓?KVM 鏀寔/闄愬埗鐨勫睘鎬э紙褰撳墠鍙湁 PROVISIONKEY锛?
鐩稿搴旂殑鏈夋晥 SGX 灞炴€ф枃浠剁殑鏂囦欢鍙ユ焺銆?

SGX 瀛愮郴缁熼檺鍒跺涓€閮ㄥ垎椋炲湴灞炴€х殑璁块棶锛屼互渚夸负鏈鏀荤牬鐨勫唴鏍告彁渚涢澶栧畨鍏ㄦ€э紝渚嬪
PROVISIONKEY 鐨勪娇鐢ㄥ彈鍒伴檺鍒讹紝浠ラ樆姝㈡伓鎰忚蒋浠跺埄鐢?PROVISIONKEY 鑾峰緱绋冲畾鐨勭郴缁熸寚绾广€?
涓轰簡闃叉鐢ㄦ埛绌洪棿閫氳繃鍦?VM 涓繍琛岄鍦版潵瑙勯伩姝ょ被闄愬埗锛孠VM 榛樿闃绘瀵圭壒鏉冨睘鎬х殑
璁块棶銆?

鏇村缁嗚妭璇峰弬闃?Documentation/arch/x86/sgx.rst銆?

### 7.27 KVM_CAP_EXIT_ON_EMULATION_FAILURE


:Architectures: x86
:Parameters: args[^0^] 鐗规€ф槸鍚﹀簲鍚敤

褰撳惎鐢ㄦ鑳藉姏鏃讹紝妯℃嫙澶辫触灏嗗鑷翠互 KVM_INTERNAL_ERROR 閫€鍑哄埌鐢ㄦ埛绌洪棿锛堣皟鐢ㄦā鎷熷櫒
澶勭悊 VMware 鍚庨棬鎸囦护鐨勬儏鍐甸櫎澶栵級銆傛澶栵紝KVM 鐜板湪灏嗕负浠讳綍鍥犳ā鎷熷け璐ュ鑷寸殑閫€鍑哄埌
鐢ㄦ埛绌洪棿鎻愪緵鏈€澶?15 鏉℃寚浠ゅ瓧鑺傘€傚綋鍙戠敓杩欎簺閫€鍑哄埌鐢ㄦ埛绌洪棿鏃讹紝浣跨敤 emulation_failure
缁撴瀯鑰岄潪 internal 缁撴瀯銆傚畠浠叿鏈夌浉鍚岀殑甯冨眬锛屼絾 emulation_failure 缁撴瀯鏇磋创鍚堝唴瀹广€?
瀹冭繕鏄惧紡瀹氫箟浜嗏€渇lags鈥濆瓧娈碉紝鐢ㄤ簬鎻忚堪缁撴瀯涓湁鏁堢殑瀛楁锛堝嵆锛氬鏋滃湪鈥渇lags鈥濆瓧娈典腑
璁剧疆浜?KVM_INTERNAL_ERROR_EMULATION_FLAG_INSTRUCTION_BYTES锛屽垯鈥渋nsn_size鈥濆拰
鈥渋nsn_bytes鈥濋兘鍖呭惈鏈夋晥鏁版嵁锛夈€?

### 7.28 KVM_CAP_ARM_MTE


:Architectures: arm64
:Parameters: none

姝よ兘鍔涜〃绀?KVM锛堜互鍙婄‖浠讹級鏀寔鍚戝鎴锋満鏆撮湶鍐呭瓨鏍囪鎵╁睍锛圡TE锛夈€傚湪鍒涘缓浠讳綍 VCPU
涔嬪墠锛屽畠涔熷繀椤荤敱 VMM 鍚敤锛屼互鍏佽瀹㈡埛鏈鸿闂€傛敞鎰?MTE 浠呭瀹㈡埛鏈哄湪 AArch64 妯″紡涓?
杩愯鏃跺彲鐢紝鍚敤姝よ兘鍔涘皢瀵艰嚧灏濊瘯鍒涘缓 AArch32 VCPU 澶辫触銆?

鍚敤鍚庯紝瀹㈡埛鏈鸿兘澶熻闂笌鎻愪緵缁欏鎴锋満鐨勪换浣曞唴瀛樼浉鍏宠仈鐨勬爣璁般€侹VM 灏嗙‘淇濆湪瀹夸富鐨?
浜ゆ崲鎴栦紤鐪犳湡闂寸淮鎶よ繖浜涙爣璁帮紱浣嗘槸锛屽鏋?VM 琚縼绉伙紝VMM 闇€瑕侀€傚綋鍦版墜鍔ㄤ繚瀛?鎭㈠
杩欎簺鏍囪銆?

鍚敤姝よ兘鍔涙椂锛宮emslot 涓殑鎵€鏈夊唴瀛樺繀椤绘槧灏勪负 `MAP_ANONYMOUS` 鎴栦娇鐢ㄥ熀浜?RAM 鐨?
鏂囦欢鏄犲皠锛坄tmpfs`銆乣memfd`锛夛紝灏濊瘯鐢ㄦ棤鏁堢殑 mmap 鍒涘缓 memslot 灏嗗鑷磋繑鍥?-EINVAL銆?

鍚敤鏃讹紝VMM 鍙互鍒╃敤 `KVM_ARM_MTE_COPY_TAGS` ioctl 鍦ㄥ鎴锋満涔嬮棿鎵归噺澶嶅埗鏍囪銆?

### 7.29 KVM_CAP_VM_MOVE_ENC_CONTEXT_FROM


:Architectures: x86 SEV enabled
:Type: vm
:Parameters: args[^0^] 鏄簮 vm 鐨?fd
:Returns: 0 on success

姝よ兘鍔涘厑璁哥敤鎴风┖闂村皢鍔犲瘑涓婁笅鏂囦粠鐢辫 fd 鎸囩ず鐨?VM 杩佺Щ鍒拌皟鐢ㄦ鑳藉姏鐨?VM 涓娿€?

杩欐棬鍦ㄦ敮鎸佺敤鎴风┖闂?VMM 涔嬮棿鐨?VM 瀹垮唴杩佺Щ锛屽湪涓嶄腑鏂鎴锋満鐨勬儏鍐典笅鍗囩骇 VMM 杩涚▼銆?

### 7.31 KVM_CAP_DISABLE_QUIRKS2


:Parameters: args[^0^] - 瑕佺鐢ㄧ殑 KVM 鎬櫀锛坬uirk锛夐泦鍚?
:Architectures: x86
:Type: vm

姝よ兘鍔涘鏋滃惎鐢紝灏嗗鑷?KVM 绂佺敤涓€浜涜涓烘€櫀锛坬uirk锛夈€?

涓烘鑳藉姏璋冪敤 KVM_CHECK_EXTENSION 灏嗚繑鍥炲彲鍦?KVM 涓鐢ㄧ殑鎬櫀鐨勪綅鎺╃爜銆?

涓烘鑳藉姏璋冪敤 KVM_ENABLE_CAP 鐨勫弬鏁版槸涓€涓绂佺敤鐨勬€櫀鐨勪綅鎺╃爜锛屼笖蹇呴』鏄?
KVM_CHECK_EXTENSION 杩斿洖鐨勪綅鎺╃爜鐨勫瓙闆嗐€?

cap.args[^0^] 涓殑鏈夋晥浣嶄负锛?

========================================   ================================================
KVM_X86_QUIRK_LINT0_REENABLED              榛樿鎯呭喌涓嬶紝LVT LINT0 瀵勫瓨鍣ㄧ殑澶嶄綅鍊兼槸 0x700
                                           锛圓PIC_MODE_EXTINT锛夈€傜鐢ㄦ鎬櫀鏃讹紝澶嶄綅鍊间负
                                           0x10000锛圓PIC_LVT_MASKED锛夈€?

KVM_X86_QUIRK_CD_NW_CLEARED                榛樿鎯呭喌涓嬶紝KVM 娓呴櫎 AMD CPU 涓婄殑 CR0.CD 鍜?
                                           CR0.NW锛屼互瑙勯伩浠?CR0.CD锛堝嵆缂撳瓨澶勪簬鈥渘o fill鈥?
                                           妯″紡锛夋案涔呰繍琛岀殑瀹㈡埛鏈哄浐浠?bug銆?

                                           绂佺敤姝ゆ€櫀鏃讹紝KVM 涓嶄細鏀瑰彉 CR0.CD 鍜?CR0.NW
                                           鐨勫€笺€?

KVM_X86_QUIRK_LAPIC_MMIO_HOLE              榛樿鎯呭喌涓嬶紝鍗充娇閰嶇疆涓?x2APIC 妯″紡锛孧MIO
                                           LAPIC 鎺ュ彛涔熷彲鐢ㄣ€傜鐢ㄦ鎬櫀鏃讹紝濡傛灉 LAPIC 澶勪簬
                                           x2APIC 妯″紡锛孠VM 浼氱鐢?MMIO LAPIC 鎺ュ彛銆?

KVM_X86_QUIRK_OUT_7E_INC_RIP               榛樿鎯呭喌涓嬶紝KVM 鍦ㄩ€€鍑哄埌鐢ㄦ埛绌洪棿澶勭悊鍚?0x7e
                                           绔彛鐨?OUT 鎸囦护涔嬪墠棰勯€掑 %rip銆傜鐢ㄦ鎬櫀鏃讹紝
                                           KVM 鍦ㄩ€€鍑哄埌鐢ㄦ埛绌洪棿涔嬪墠涓嶄細棰勯€掑 %rip銆?

KVM_X86_QUIRK_MISC_ENABLE_NO_MWAIT         绂佺敤姝ゆ€櫀鏃讹紝濡傛灉 IA32_MISC_ENABLE[bit 18]
                                           锛圡WAIT锛夎缃綅锛孠VM 璁剧疆
                                           CPUID.01H:ECX[bit 3]锛圡ONITOR/MWAIT锛夈€傛澶栵紝
                                           绂佺敤姝ゆ€櫀鏃讹紝濡傛灉 IA32_MISC_ENABLE[bit 18]琚?
                                           娓呴浂锛孠VM 娓呴櫎 CPUID.01H:ECX[bit 3]銆?

KVM_X86_QUIRK_FIX_HYPERCALL_INSN           榛樿鎯呭喌涓嬶紝KVM 閲嶅啓瀹㈡埛鏈?VMMCALL/VMCALL
                                           鎸囦护锛屼互鍖归厤绯荤粺渚涘簲鍟嗙殑瓒呯骇璋冪敤鎸囦护銆傜鐢ㄦ
                                           鎬櫀鏃讹紝KVM 涓嶅啀閲嶅啓鏃犳晥鐨勫鎴锋満瓒呯骇璋冪敤鎸囦护銆?
                                           鎵ц閿欒鐨勮秴绾ц皟鐢ㄦ寚浠ゅ皢鍦ㄥ鎴锋満鍐呯敓鎴?#UD銆?

KVM_X86_QUIRK_MWAIT_NEVER_UD_FAULTS        榛樿鎯呭喌涓嬶紝KVM 灏?MONITOR/MWAIT锛堝鏋滆
                                           鎷︽埅锛夋ā鎷熶负 NOP锛屼笉璁烘牴鎹鎴锋満 CPUID 瀹冧滑鏄惁
                                           鍙楁敮鎸併€傜鐢ㄦ鎬櫀涓旀湭璁剧疆
                                           KVM_X86_DISABLE_EXITS_MWAIT锛圡ONITOR/MWAIT 琚?
                                           鎷︽埅锛夋椂锛屽鏋滄牴鎹鎴锋満 CPUID 瀹冧滑涓嶅彈鏀寔锛?
                                           KVM 灏嗗湪 MONITOR/MWAIT 涓婃敞鍏?#UD銆傛敞鎰忥紝濡傛灉
                                           KVM_X86_QUIRK_MISC_ENABLE_NO_MWAIT 琚鐢紝KVM
                                           灏嗗湪鍐欏叆 MISC_ENABLE 鏃朵慨鏀瑰鎴锋満 CPUID 涓殑
                                           MONITOR/MWAIT 鏀寔銆?

KVM_X86_QUIRK_SLOT_ZAP_ALL                 榛樿鎯呭喌涓嬶紝瀵逛簬 KVM_X86_DEFAULT_VM 绫诲瀷鐨?
                                           VM锛孠VM 鍦ㄥ垹闄ゆ垨绉诲姩 memslot 鏃朵娇鎵€鏈?memslot 鍜?
                                           鍦板潃绌洪棿涓殑鎵€鏈?SPTE 澶辨晥銆傜鐢ㄦ鎬櫀锛堟垨 VM 绫诲瀷
                                           涓嶆槸 KVM_X86_DEFAULT_VM锛夋椂锛孠VM 鍙‘淇濊鍒犻櫎鎴?
                                           绉诲姩鐨?memslot 鐨勫悗澶囧唴瀛樹笉鍙揪锛屽嵆 KVM _鍙兘_ 鍙?
                                           浣夸笌璇?memslot 鐩稿叧鐨?SPTE 澶辨晥銆?

KVM_X86_QUIRK_STUFF_FEATURE_MSRS           榛樿鎯呭喌涓嬶紝鍦ㄥ垱寤?vCPU 鏃讹紝KVM 灏?vCPU 鐨?
                                           MSR_IA32_PERF_CAPABILITIES锛?x345锛夈€?
                                           MSR_IA32_ARCH_CAPABILITIES锛?x10a锛夈€?
                                           MSR_PLATFORM_INFO锛?xce锛変互鍙婃墍鏈?VMX MSR
                                           锛?x480..0x492锛夎涓?KVM 鏀寔鐨勬渶澶ц兘鍔涖€侹VM 杩樺皢
                                           MSR_IA32_UCODE_REV锛?x8b锛夎涓轰换鎰忓€硷紙Intel 涓?AMD
                                           涓嶅悓锛夈€傛渶鍚庯紝褰撹缃鎴锋満 CPUID 鏃讹紙鐢辩敤鎴风┖闂达級锛?
                                           KVM 淇敼閫夊畾鐨?VMX MSR 瀛楁锛屼互寮哄埗瀹㈡埛鏈?CPUID 涓?
                                           L2 鐨勬湁鏁?ISA 涔嬮棿鐨勪竴鑷存€с€傜鐢ㄦ鎬櫀鏃讹紝KVM 灏?
                                           vCPU 鐨?MSR 鍊兼竻闆讹紙鏈変袱涓緥澶栵紝瑙佷笅鏂囷級锛屽嵆灏嗙壒鎬?
                                           MSR 瑙嗕负 CPUID 鍙讹紝缁欎簣鐢ㄦ埛绌洪棿瀵?vCPU 鍨嬪彿瀹氫箟鐨?
                                           瀹屽叏鎺у埗銆傛鎬櫀涓嶅奖鍝?VMX MSR CR0/CR4_FIXED1
                                           锛?x487 鍜?0x489锛夛紝鍥犱负 KVM 鐜板湪涓嶅厑璁稿畠浠敱鐢ㄦ埛绌洪棿
                                           璁剧疆锛圞VM 鏍规嵁瀹㈡埛鏈?CPUID 璁剧疆瀹冧滑锛屽嚭浜庡畨鍏ㄧ洰鐨勶級銆?

KVM_X86_QUIRK_IGNORE_GUEST_PAT             榛樿鎯呭喌涓嬶紝鍦?Intel 骞冲彴涓婏紝KVM 蹇界暐瀹㈡埛鏈?
                                           PAT锛屽苟鍦?EPT 涓己鍒舵湁鏁堝唴瀛樼被鍨嬩负 WB銆傝鎬櫀鍦?
                                           鏃犳硶瀹夊叏灏婇噸瀹㈡埛鏈?PAT 鐨?Intel 骞冲彴锛堝嵆娌℃湁 CPU
                                           鑷梾鎺紝KVM 鎬绘槸蹇界暐瀹㈡埛鏈?PAT 骞跺己鍒舵湁鏁堝唴瀛樼被鍨?
                                           涓?WB锛変笂涓嶅彲鐢ㄣ€傚湪 AMD 骞冲彴鎴栵紙鍦?Intel 涓婏級褰?VM
                                           鍒嗛厤浜嗛潪涓€鑷?DMA 璁惧鏃讹紝瀹冧篃琚拷鐣ワ紱KVM 鍦ㄦ绫?
                                           鎯呭喌涓嬫€绘槸灏婇噸瀹㈡埛鏈?PAT銆傞渶瑕佹鎬櫀浠ラ伩鍏嶆煇浜?Intel
                                           Xeon 骞冲彴锛堜緥濡?ICX銆丼PR锛変笂鐨勬€ц兘涓嬮檷锛岃繖浜涘钩鍙?
                                           鏀寔鑷梾鎺㈢壒鎬э紝浣?UC 瓒冲鎱紝浼氬鑷翠竴浜涗娇鐢?UC 鑰岄潪
                                           WC 鏄犲皠鏄惧瓨鐨勮緝鑰佸鎴锋満鍑虹幇闂銆傚鏋滅敤鎴风┖闂寸煡閬撴病鏈?
                                           姝ょ被瀹㈡埛鏈鸿蒋浠讹紝渚嬪瀹冩病鏈夋毚闇?bochs 鍥惧舰璁惧锛堝凡鐭?
                                           鍏堕┍鍔ㄦ湁 bug锛夛紝鍒欏彲浠ョ鐢ㄦ鎬櫀浠ュ皧閲嶅鎴锋満 PAT銆?

KVM_X86_QUIRK_VMCS12_ALLOW_FREEZE_IN_SMM   榛樿鎯呭喌涓嬶紝KVM 鏀惧瀵?vmcs12 涓?
                                           GUEST_IA32_DEBUGCTL 鐨勪竴鑷存€ф鏌ワ紝浠ュ厑璁歌缃?
                                           FREEZE_IN_SMM銆傜鐢ㄦ鎬櫀鏃讹紝KVM 瑕佹眰璇ヤ綅琚竻闆躲€?
                                           娉ㄦ剰锛屾棤璁烘€櫀璁剧疆濡備綍锛寁mcs02 鐨勮浣嶄粛瀹屽叏鐢卞涓?
                                           鎺у埗銆?
========================================   ================================================

### 7.32 KVM_CAP_MAX_VCPU_ID


:Architectures: x86
:Target: VM
:Parameters: args[^0^] - 涓哄綋鍓?VM 璁剧疆鐨勬渶澶?APIC ID 鍊?
:Returns: 0 on success, -EINVAL if args[^0^] is beyond KVM_MAX_VCPU_IDS
          supported in KVM or if it has been set.

姝よ兘鍔涘厑璁哥敤鎴峰湪鍒涘缓 vCPU 涔嬪墠锛屼负褰撳墠 VM 浼氳瘽鎸囧畾鍒嗛厤鐨勬渶澶у彲鑳?APIC ID锛屼粠鑰屼负
鎸?APIC ID 绱㈠紩鐨勬暟鎹粨鏋勮妭鐪佸唴瀛樸€傜敤鎴风┖闂磋兘澶熸牴鎹寚瀹氱殑 CPU 鎷撴墤璁＄畻鍑?APIC ID
鍊肩殑闄愬埗銆?

璇ュ€煎彧鑳藉湪 KVM_ENABLE_CAP 琚涓洪潪闆跺€间箣鍓嶏紝鎴栫洿鍒板垱寤?vCPU 涔嬪墠鏇存敼銆傚湪鍒涘缓
绗竴涓?vCPU 鏃讹紝濡傛灉鍊艰璁句负 0 鎴栨湭璋冪敤 KVM_ENABLE_CAP锛孠VM 灏嗕娇鐢?
KVM_CHECK_EXTENSION(KVM_CAP_MAX_VCPU_ID) 鐨勮繑鍥炲€间綔涓烘渶澶?APIC ID銆?

### 7.33 KVM_CAP_X86_NOTIFY_VMEXIT


:Architectures: x86
:Target: VM
:Parameters: args[^0^] 鏄€氱煡绐楀彛鐨勫€间互鍙婁竴浜涙爣蹇?
:Returns: 0 on success, -EINVAL if args[^0^] contains invalid flags or notify
          VM exit is unsupported.

args[^0^] 鐨?63:32 浣嶇敤浜庨€氱煡绐楀彛銆?
```

  #define KVM_X86_NOTIFY_VMEXIT_ENABLED    (1 << 0)
  #define KVM_X86_NOTIFY_VMEXIT_USER       (1 << 1)

```
姝よ兘鍔涘厑璁哥敤鎴峰湪 VM 鍒涘缓鏈熼棿鍦ㄦ瘡 VM 鑼冨洿鍐呴厤缃€氱煡 VM 閫€鍑虹殑寮€/鍏炽€傞粯璁ゆ儏鍐典笅
绂佺敤閫氱煡 VM 閫€鍑恒€傚綋鐢ㄦ埛绌洪棿鍦?args[^0^] 涓缃?KVM_X86_NOTIFY_VMEXIT_ENABLED
浣嶆椂锛孷MM 灏嗕娇鐢ㄦ彁渚涚殑閫氱煡绐楀彛鍚敤姝ょ壒鎬э紝濡傛灉鍦?VM 闈炴牴妯″紡涓嬬粡杩囨寚瀹氭椂闂达紙閫氱煡
绐楀彛锛変粛鏃犱簨浠剁獥鍙ｅ彂鐢燂紝灏嗙敓鎴?VM 閫€鍑恒€?

濡傛灉鍦?args[^0^] 涓缃簡 KVM_X86_NOTIFY_VMEXIT_USER锛屽垯鍦ㄥ彂鐢熼€氱煡 VM 閫€鍑烘椂锛?
KVM 灏嗛€€鍑哄埌鐢ㄦ埛绌洪棿杩涜澶勭悊銆?

姝よ兘鍔涙棬鍦ㄧ紦瑙ｆ伓鎰?VM 瀵艰嚧 CPU 鍗′綇锛堢敱浜庝簨浠剁獥鍙ｆ湭鎵撳紑锛夊苟浣?CPU 瀵瑰涓绘垨鍏朵粬
VM 涓嶅彲鐢ㄧ殑濞佽儊銆?

### 7.35 KVM_CAP_X86_APIC_BUS_CYCLES_NS


:Architectures: x86
:Target: VM
:Parameters: args[^0^] 鏄湡鏈涚殑 APIC 鎬荤嚎鏃堕挓棰戠巼锛屼互绾崇涓哄崟浣?
:Returns: 0 on success, -EINVAL if args[^0^] contains an invalid value for the
          frequency or if any vCPUs have been created, -ENXIO if a virtual
          local APIC has not been created using KVM_CREATE_IRQCHIP.

姝よ兘鍔涜缃?VM 鐨?APIC 鎬荤嚎鏃堕挓棰戠巼锛孠VM 鐨勫唴鏍告€佽櫄鎷?APIC 鍦ㄦā鎷?APIC 瀹氭椂鍣ㄦ椂
浣跨敤瀹冦€侹VM 鐨勯粯璁ゅ€煎彲閫氳繃 KVM_CHECK_EXTENSION 鑾峰彇銆?

娉ㄦ剰锛氬鏋滃皢闈為浂鐨?CPUID 0x15 鏆撮湶缁欏鎴锋満锛岀敤鎴风┖闂磋礋璐ｆ纭厤缃?CPUID 0x15锛屽嵆
鏍稿績鏅舵尟鏃堕挓棰戠巼銆?

### 7.36 KVM_CAP_DIRTY_LOG_RING/KVM_CAP_DIRTY_LOG_RING_ACQ_REL


:Architectures: x86, arm64, riscv
:Type: vm
:Parameters: args[^0^] - 鑴忔棩蹇楃幆鐨勫ぇ灏?

KVM 鑳藉浣跨敤 mmap 鍒扮敤鎴风┖闂寸殑鐜舰缂撳啿鍖烘潵璺熻釜鑴忓唴瀛橈紱姣忎釜 vcpu 鏈変竴涓剰鐜€?

鑴忕幆瀵圭敤鎴风┖闂村彲鐢紝鏄竴涓?
```

  struct kvm_dirty_gfn {
          __u32 flags;
          __u32 slot; /* as_id | slot_id */
          __u64 offset;
  };

```
涓哄畾涔?flags 瀛楁锛屽畾涔変簡浠ヤ笅鍊?
```

  #define KVM_DIRTY_GFN_F_DIRTY           BIT(0)
  #define KVM_DIRTY_GFN_F_RESET           BIT(1)
  #define KVM_DIRTY_GFN_F_MASK            0x3

```
鐢ㄦ埛绌洪棿搴斿湪 KVM_CREATE_VM ioctl 涔嬪悗绔嬪嵆璋冪敤 KVM_ENABLE_CAP ioctl锛屼负鏂板鎴锋満
鍚敤姝よ兘鍔涘苟璁剧疆鐜殑澶у皬銆傚惎鐢ㄨ鑳藉姏鍙厑璁稿湪鍒涘缓浠讳綍 vCPU 涔嬪墠杩涜锛屼笖鐜殑澶у皬
蹇呴』鏄?2 鐨勫箓銆傜幆缂撳啿鍖鸿秺澶э紝鐜弧涓?VM 琚揩閫€鍑哄埌鐢ㄦ埛绌洪棿鐨勫彲鑳芥€ц秺灏忋€傛渶浼樺ぇ灏?
鍙栧喅浜庡伐浣滆礋杞斤紝浣嗗缓璁嚦灏戜负 64 KiB锛?096 涓潯鐩級銆?

涓庤剰椤典綅鍥句竴鏍凤紝缂撳啿鍖鸿窡韪璁剧疆浜?KVM_MEM_LOG_DIRTY_PAGES 鏍囧織鐨?KVM_SET_USER_MEMORY_REGION
鐨勬墍鏈夌敤鎴峰唴瀛樺尯鍩熺殑鍐欏叆銆備竴鏃﹀唴瀛樺尯鍩熶互璇ユ爣蹇楁敞鍐岋紝鐢ㄦ埛绌洪棿灏卞彲浠ュ紑濮嬩粠鐜舰缂撳啿鍖?
鏀堕泦鑴忛〉銆?

鐜舰缂撳啿鍖轰腑鐨勪竴涓潯鐩彲浠ユ槸鏈娇鐢ㄧ殑锛堟爣蹇椾綅 `00`锛夈€佽剰鐨勶紙鏍囧織浣?`01`锛夋垨宸叉敹闆嗙殑
锛堟爣蹇椾綅 `1X`锛夈€?
```

          dirtied         harvested        reset
     00 -----------> 01 -------------> 1X -------+
      ^                                          |
      |                                          |
      +------------------------------------------+

```
瑕佹敹闆嗚剰椤碉紝鐢ㄦ埛绌洪棿璁块棶 mmap 鐨勭幆褰㈢紦鍐插尯浠ヨ鍙栬剰鐨?GFN銆傚鏋?flags 璁剧疆浜?DIRTY
浣嶏紙鍦ㄦ闃舵 RESET 浣嶅繀椤绘竻闆讹級锛屽垯鎰忓懗鐫€姝?GFN 鏄剰 GFN銆傜敤鎴风┖闂村簲鏀堕泦姝?GFN 骞跺皢
鏍囧織浠庣姸鎬?`01b` 鏀逛负 `1Xb`锛堜綅 0 灏嗚 KVM 蹇界暐锛屼絾浣?1 蹇呴』璁剧疆浠ヨ〃鏄庢 GFN 宸茶
鏀堕泦骞剁瓑寰呴噸缃級锛岀劧鍚庣户缁笅涓€涓?GFN銆傜敤鎴风┖闂村簲鎸佺画姝ゆ搷浣滐紝鐩村埌鏌愪釜 GFN 鐨?flags
鐨?DIRTY 浣嶈娓呴浂锛屾剰鍛崇潃瀹冨凡鏀堕泦浜嗘墍鏈夊彲鐢ㄧ殑鑴?GFN銆?

娉ㄦ剰锛屽湪寮卞唴瀛樺簭鏋舵瀯涓婏紝鐢ㄦ埛绌洪棿瀵圭幆褰㈢紦鍐插尯锛堟洿鍏蜂綋鍦拌鏄€渇lags鈥濆瓧娈碉級鐨勮闂繀椤?
鏈夊簭锛屽湪鍙敤鏃朵娇鐢?load-acquire/store-release 璁块棶鍣紝鎴栦娇鐢ㄤ换浣曞叾浠栬兘纭繚姝ゆ湁搴忔€?
鐨勫唴瀛樺睆闅溿€?

鐢ㄦ埛绌洪棿娌℃湁蹇呰涓€娆℃€ф敹闆嗘墍鏈夎剰 GFN銆備絾瀹冨繀椤绘寜椤哄簭鏀堕泦鑴?GFN锛屽嵆鐢ㄦ埛绌洪棿绋嬪簭涓嶈兘
璺宠繃鏌愪釜鑴?GFN 鍘绘敹闆嗗畠鏃佽竟鐨勯偅涓€?

鍦ㄥ鐞嗙幆褰㈢紦鍐插尯涓殑涓€涓垨澶氫釜鏉＄洰涔嬪悗锛岀敤鎴风┖闂磋皟鐢?VM ioctl KVM_RESET_DIRTY_RINGS
鏉ラ€氱煡鍐呮牳锛屼互渚垮唴鏍搁噸鏂颁繚鎶ら偅浜涘凡鏀堕泦鐨?GFN銆傚洜姝わ紝蹇呴』鍦ㄨ鍙栬剰椤靛唴瀹筥涔嬪墠_璋冪敤
姝?ioctl銆?

鑴忕幆鍙兘浼氬彉婊°€傚綋杩欑鎯呭喌鍙戠敓鏃讹紝vcpu 鐨?KVM_RUN 灏嗕互閫€鍑哄師鍥?KVM_EXIT_DIRTY_RING_FULL
杩斿洖銆?

鑴忕幆鎺ュ彛涓?KVM_GET_DIRTY_LOG 鎺ュ彛鐩告瘮鏈変竴涓富瑕佸尯鍒細浠庣敤鎴风┖闂磋鍙栬剰鐜椂锛屽唴鏍镐粛
鍙兘灏氭湭灏嗗鐞嗗櫒鐨勮剰椤电紦鍐插尯鍒锋柊鍒板唴鏍哥紦鍐插尯锛堣€屽浜庤剰浣嶅浘锛屽埛鏂版槸鐢?
KVM_GET_DIRTY_LOG ioctl 瀹屾垚鐨勶級銆備负姝わ紝闇€瑕佷娇鐢ㄤ俊鍙峰皢 vcpu 韪㈠嚭 KVM_RUN銆傜敱姝や骇鐢熺殑
vmexit 纭繚鎵€鏈夎剰 GFN 閮借鍒锋柊鍒拌剰鐜腑銆?

娉ㄦ剰锛欿VM_CAP_DIRTY_LOG_RING_ACQ_REL 鏄急鍐呭瓨搴忔灦鏋勫敮涓€搴旀毚闇茬殑鑳藉姏锛屼互鎸囩ず鍦ㄨ鍙?
鏉＄洰鐘舵€佸苟灏嗗叾浠?DIRTY 鍙樹负 HARVESTED 鏃跺鐢ㄦ埛绌洪棿鏂藉姞鐨勯澶栧唴瀛樻湁搴忔€ц姹傘€傚叿鏈夌被 TSO
鏈夊簭鎬э紙濡?x86锛夌殑鏋舵瀯鍏佽鍚屾椂鍚戠敤鎴风┖闂存毚闇?KVM_CAP_DIRTY_LOG_RING 鍜?
KVM_CAP_DIRTY_LOG_RING_ACQ_REL銆?

鍚敤鑴忕幆鍚庯紝鐢ㄦ埛绌洪棿闇€瑕佹娴?KVM_CAP_DIRTY_LOG_RING_WITH_BITMAP 鑳藉姏锛屼互鏌ョ湅鐜粨鏋?
鏄惁鍙互鐢辨瘡鎻掓Ы锛坧er-slot锛変綅鍥炬敮鎸併€傞€氬憡姝よ兘鍔涙剰鍛崇潃璇ユ灦鏋勫彲浠ュ湪娌℃湁 vcpu/鐜笂涓嬫枃
鐨勬儏鍐典笅寮勮剰瀹㈡埛鏈洪〉锛屽洜姝ら儴鍒嗚剰淇℃伅浠嶅皢缁存姢鍦ㄤ綅鍥剧粨鏋勪腑銆傚鏋滃皻鏈惎鐢?
KVM_CAP_DIRTY_LOG_RING_ACQ_REL 鑳藉姏锛屾垨宸插瓨鍦ㄤ换浣?memslot锛屽垯涓嶈兘鍚敤
KVM_CAP_DIRTY_LOG_RING_WITH_BITMAP銆?

娉ㄦ剰锛岃繖閲岀殑浣嶅浘鍙槸鐜粨鏋勭殑澶囦唤銆備粎褰撳彧鏈夋瀬灏戦噺鍐呭瓨鍦?vcpu/鐜笂涓嬫枃涔嬪琚紕鑴忔椂锛?
浣跨敤鐜拰浣嶅浘缁勫悎鎵嶆湁鐩娿€傚惁鍒欙紝闇€瑕佽€冭檻鐙珛鐨勬瘡鎻掓Ы浣嶅浘鏈哄埗銆?

瑕佹敹闆嗗浠戒綅鍥句腑鐨勮剰浣嶏紝鐢ㄦ埛绌洪棿鍙互浣跨敤鐩稿悓鐨?KVM_GET_DIRTY_LOG ioctl銆傚彧瑕佹墍鏈夎剰浣?
鐨勭敓鎴愰兘鍦ㄥ崟娆￠亶鍘嗕腑瀹屾垚锛屽氨涓嶉渶瑕?KVM_CLEAR_DIRTY_LOG銆傛敹闆嗚剰浣嶅浘搴旇鏄?VMM 鍦ㄨ涓?
鐘舵€佸畬鏁翠箣鍓嶅仛鐨勬渶鍚庝竴浠朵簨銆俈MM 闇€瑕佺‘淇濊剰鐘舵€佹槸鏈€缁堢殑锛屽苟閬垮厤涓㈠け鍦ㄦ瘮鐗瑰浘鏀堕泦涔嬪悗
鎺掑簭鐨勫彟涓€涓?ioctl 浜х敓鐨勮剰椤点€?

娉ㄦ剰锛氫娇鐢ㄥ浠戒綅鍥剧殑澶氫釜绀轰緥锛氾紙1锛夐€氳繃 KVM 璁惧鈥渒vm-arm-vgic-its鈥濅笂鐨勫懡浠?
KVM_DEV_ARM_{VGIC_GRP_CTRL, ITS_SAVE_TABLES} 淇濆瓨 vgic/its 琛ㄣ€傦紙2锛夐€氳繃 KVM 璁惧
鈥渒vm-arm-vgic-its鈥濅笂鐨勫懡浠?KVM_DEV_ARM_{VGIC_GRP_CTRL, ITS_RESTORE_TABLES} 鎭㈠
vgic/its 琛ㄣ€俈GICv3 LPI 鎸傝捣鐘舵€佽鎭㈠銆傦紙3锛夐€氳繃 KVM 璁惧鈥渒vm-arm-vgic-v3鈥濅笂鐨?
鍛戒护 KVM_DEV_ARM_VGIC_{GRP_CTRL, SAVE_PENDING_TABLES} 淇濆瓨 vgic3 鎸傝捣琛ㄣ€?

### 7.37 KVM_CAP_PMU_CAPABILITY


:Architectures: x86
:Type: vm
:Parameters: arg[^0^] 鏄?PMU 铏氭嫙鍖栬兘鍔涚殑浣嶆帺鐮併€?
:Returns: 0 on success, -EINVAL when arg[^0^] contains invalid bits

姝よ兘鍔涙敼鍙?KVM 涓殑 PMU 铏氭嫙鍖栥€?

涓烘鑳藉姏璋冪敤 KVM_CHECK_EXTENSION 灏嗚繑鍥炲彲鍦?VM 涓婅皟鏁寸殑 PMU 铏氭嫙鍖栬兘鍔涚殑浣嶆帺鐮併€?

KVM_ENABLE_CAP 鐨勫弬鏁颁篃鏄竴涓綅鎺╃爜锛屽苟閫夋嫨瑕佸簲鐢ㄥ埌 VM 鐨勭壒瀹?PMU 铏氭嫙鍖栬兘鍔涖€傝繖
鍙兘鍦ㄥ垱寤?VCPU 涔嬪墠瀵?VM 璋冪敤銆?

鐩墠锛孠VM_PMU_CAP_DISABLE 鏄敮涓€鐨勮兘鍔涖€傝缃鑳藉姏灏嗙鐢ㄨ VM 鐨?PMU 铏氭嫙鍖栥€?
鐢ㄦ埛鎬佸簲璋冩暣 CPUID 鍙?0xA 浠ュ弽鏄?PMU 宸茬鐢ㄣ€?

### 7.38 KVM_CAP_VM_DISABLE_NX_HUGE_PAGES


:Architectures: x86
:Type: vm
:Parameters: arg[^0^] 蹇呴』涓?0銆?
:Returns: 0 on success, -EPERM if the userspace process does not
          have CAP_SYS_BOOT, -EINVAL if args[^0^] is not 0 or any vCPUs have been
          created.

姝よ兘鍔涚鐢ㄩ拡瀵?iTLB MULTIHIT 鐨?NX 澶ч〉缂撹В鎺柦銆?

濡傛灉鏈缃?nx_huge_pages 妯″潡鍙傛暟锛屽垯璇ヨ兘鍔涙棤鏁堛€?

姝よ兘鍔涘彧鑳藉湪鍒涘缓浠讳綍 vCPU 涔嬪墠璁剧疆銆?

### 7.39 KVM_CAP_ARM_EAGER_SPLIT_CHUNK_SIZE


:Architectures: arm64
:Type: vm
:Parameters: arg[^0^] 鏄柊鐨勬媶鍒嗗潡澶у皬銆?
:Returns: 0 on success, -EINVAL if any memslot was already created.

姝よ兘鍔涜缃?Eager Page Splitting锛堢Н鏋侀〉鎷嗗垎锛変腑浣跨敤鐨勫潡澶у皬銆?

褰撳鎴锋満鍐呭瓨鐢卞ぇ椤碉紙huge-page锛夋敮鎸佹椂锛孍ager Page Splitting 鏀瑰杽浜嗚剰鏃ュ織锛堢敤浜?
瀹炴椂杩佺Щ锛夌殑鎬ц兘銆傚畠閫氳繃鍦ㄥ惎鐢ㄨ剰鏃ュ織锛堜负鍐呭瓨鍖哄煙璁剧疆 KVM_MEM_LOG_DIRTY_PAGES
鏍囧織锛夋垨浣跨敤 KVM_CLEAR_DIRTY_LOG 鏃剁Н鏋佸湴鎷嗗垎锛岄伩鍏嶅湪缂洪〉鏃舵媶鍒嗗ぇ椤碉紙涓?PAGE_SIZE
椤碉級銆?

鍧楀ぇ灏忔寚瀹氭瘡娆℃媶鍒嗗灏戦〉锛屼负姣忎釜鍧椾娇鐢ㄥ崟娆″垎閰嶃€傚潡澶у皬瓒婂ぇ锛岄渶瑕佹彁鍓嶅垎閰嶇殑椤佃秺澶氥€?

鍧楀ぇ灏忓繀椤绘槸鏈夋晥鐨勫潡澶у皬銆傚彲鎺ュ彈鐨勫潡澶у皬鍒楄〃浣滀负 64 浣嶄綅鍥炬毚闇插湪
KVM_CAP_ARM_SUPPORTED_BLOCK_SIZES 涓紙姣忎釜浣嶆弿杩颁竴涓潡澶у皬锛夈€傞粯璁ゅ€间负 0锛屽嵆绂佺敤
绉瀬椤垫媶鍒嗐€?

### 7.40 KVM_CAP_EXIT_HYPERCALL


:Architectures: x86
:Type: vm

姝よ兘鍔涘鏋滃惎鐢紝灏嗗鑷?KVM 浠?KVM_EXIT_HYPERCALL 閫€鍑哄師鍥犻€€鍑哄埌鐢ㄦ埛绌洪棿浠ュ鐞嗘煇浜?
瓒呯骇璋冪敤銆?

涓烘鑳藉姏璋冪敤 KVM_CHECK_EXTENSION 灏嗚繑鍥炲彲閰嶇疆涓洪€€鍑哄埌鐢ㄦ埛绌洪棿鐨勮秴绾ц皟鐢ㄧ殑浣嶆帺鐮併€?
鐩墠锛屽敮涓€鐨勬绫昏秴绾ц皟鐢ㄦ槸 KVM_HC_MAP_GPA_RANGE銆?

KVM_ENABLE_CAP 鐨勫弬鏁颁篃鏄竴涓綅鎺╃爜锛屼笖蹇呴』鏄?KVM_CHECK_EXTENSION 缁撴灉鐨勫瓙闆嗐€侹VM
灏嗘妸瀵瑰簲浣嶅湪鍙傛暟涓殑瓒呯骇璋冪敤杞彂鍒扮敤鎴风┖闂达紝骞跺鍏朵綑鐨勮繑鍥?ENOSYS銆?

### 7.41 KVM_CAP_ARM_SYSTEM_SUSPEND


:Architectures: arm64
:Type: vm

鍚敤鏃讹紝KVM 灏嗕互绫诲瀷涓?KVM_SYSTEM_EVENT_SUSPEND 鐨?KVM_EXIT_SYSTEM_EVENT 閫€鍑哄埌
鐢ㄦ埛绌洪棿锛屼互澶勭悊瀹㈡埛鏈烘寕璧疯姹傘€?

### 7.42 KVM_CAP_ARM_WRITABLE_IMP_ID_REGS


:Architectures: arm64
:Target: VM
:Parameters: None
:Returns: 0 on success, -EINVAL if vCPUs have been created before enabling this
          capability.

姝よ兘鍔涙敼鍙樹簡鏍囪瘑 Arm 鏋舵瀯 PE 瀹炵幇鐨勫瘎瀛樺櫒鐨勮涓猴細MIDR_EL1銆丷EVIDR_EL1 鍜?
AIDR_EL1銆傞粯璁ゆ儏鍐典笅锛岃繖浜涘瘎瀛樺櫒瀵圭敤鎴风┖闂村彲瑙侊紝浣嗚瑙嗕负涓嶅彉閲忋€?

鍚敤姝よ兘鍔涙椂锛孠VM 鍏佽鐢ㄦ埛鍦ㄧ涓€娆?KVM_RUN 涔嬪墠鏇存敼涓婅堪瀵勫瓨鍣ㄣ€傝繖浜涘瘎瀛樺櫒鏄?VM
浣滅敤鍩熺殑锛屾剰鍛崇潃鍚屼竴缁勫€间細鍛堢幇缁欑粰瀹?VM 涓殑鎵€鏈?vCPU銆?

### 7.43 KVM_CAP_RISCV_MP_STATE_RESET


:Architectures: riscv
:Type: VM
:Parameters: None
:Returns: 0 on success, -EINVAL if arg[^0^] is not zero

鍚敤姝よ兘鍔涙椂锛孠VM 鍦ㄩ€氳繃 IOCTL 璁剧疆 MP_STATE_INIT_RECEIVED 鏃堕噸缃?VCPU銆傚師濮嬬殑
MP_STATE 琚繚鐣欍€?
### 7.44 KVM_CAP_ARM_CACHEABLE_PFNMAP_SUPPORTED


:Architectures: arm64
:Target: VM
:Parameters: None

姝よ兘鍔涘悜鐢ㄦ埛绌洪棿鎸囩ず涓€涓?PFNMAP 鍐呭瓨鍖哄煙鏄惁鍙互瀹夊叏鍦版槧灏勪负鍙紦瀛橈紙cacheable锛夈€?
杩欎緷璧栦簬纭欢涓婃槸鍚﹀瓨鍦ㄥ己鍒跺啓鍥烇紙force write back锛孎WB锛夌壒鎬ф敮鎸併€?

### 7.45 KVM_CAP_ARM_SEA_TO_USER


:Architecture: arm64
:Target: VM
:Parameters: none
:Returns: 0 on success, -EINVAL if unsupported.

鍚敤姝よ兘鍔涙椂锛孠VM 鍙兘浼氬洜瀹㈡埛鏈鸿闂鑷寸殑銆佽繘鍏?EL2 鐨?SEA 鑰岄€€鍑哄埌鐢ㄦ埛绌洪棿銆?
鏇村淇℃伅璇峰弬闃?`KVM_EXIT_ARM_SEA`銆?

### 7.46 KVM_CAP_S390_USER_OPEREXEC


:Architectures: s390
:Parameters: none

鍚敤姝よ兘鍔涙椂锛孠VM 浼氬皢鍏惰嚜韬笉澶勭悊鐨勬搷浣滃紓甯稿叏閮ㄨ浆鍙戝埌鐢ㄦ埛绌洪棿銆傝繖涔熷寘鎷敱
KVM_CAP_S390_USER_INSTR0 绠＄悊鐨?0x0000 鎸囦护銆傚鏋滅敤鎴风┖闂村笇鏈涙ā鎷燂紙灏氾級鏈湪纭欢
涓疄鐜扮殑鎸囦护锛岃繖浼氬緢鏈夊府鍔┿€?

鍗充娇鍦?VCPU 宸茶鍒涘缓骞舵鍦ㄨ繍琛岀殑鎯呭喌涓嬶紝涔熷彲浠ュ姩鎬佸惎鐢ㄦ鑳藉姏銆?

## 8. 鍏朵粬鑳藉姏銆?


鏈妭鍒楀嚭鎻愪緵鏈夊叧 KVM 瀹炵幇鍏朵粬鐗规€т俊鎭殑鑳藉姏銆?

### 8.1 KVM_CAP_PPC_HWRNG


:Architectures: ppc

姝よ兘鍔涳紝濡傛灉 KVM_CHECK_EXTENSION 鎸囩ず鍏跺彲鐢紝鎰忓懗鐫€鍐呮牳瀹炵幇浜嗙敱纭欢闅忔満鏁扮敓鎴愬櫒
鏀拺鐨?H_RANDOM 瓒呯骇璋冪敤銆傚鏋滃瓨鍦紝鍐呮牳鐨?H_RANDOM 澶勭悊绋嬪簭鍙互閫氳繃
KVM_CAP_PPC_ENABLE_HCALL 鑳藉姏涓哄鎴锋満浣跨敤鑰屽惎鐢ㄣ€?

### 8.3 KVM_CAP_PPC_MMU_RADIX


:Architectures: ppc

姝よ兘鍔涳紝濡傛灉 KVM_CHECK_EXTENSION 鎸囩ず鍏跺彲鐢紝鎰忓懗鐫€鍐呮牳鍙互鏀寔浣跨敤 Power ISA
V3.00锛堝 POWER9 澶勭悊鍣ㄤ腑鎵€瀹炵幇锛変腑瀹氫箟鐨?radix MMU 鐨勫鎴锋満銆?

### 8.4 KVM_CAP_PPC_MMU_HASH_V3


:Architectures: ppc

姝よ兘鍔涳紝濡傛灉 KVM_CHECK_EXTENSION 鎸囩ず鍏跺彲鐢紝鎰忓懗鐫€鍐呮牳鍙互鏀寔浣跨敤 Power ISA
V3.00锛堝 POWER9 澶勭悊鍣ㄤ腑鎵€瀹炵幇锛変腑瀹氫箟鐨勫搱甯岄〉琛?MMU 鐨勫鎴锋満锛屽寘鎷唴瀛樹腑鐨勬琛ㄣ€?

### 8.5 KVM_CAP_MIPS_VZ


:Architectures: mips

姝よ兘鍔涳紝濡傛灉鍦ㄤ富 kvm 鍙ユ焺涓婃墽琛?KVM_CHECK_EXTENSION 鎸囩ず鍏跺彲鐢紝鎰忓懗鐫€鍙互閫氳繃
KVM 浣跨敤纭欢鐨勫畬鍏ㄧ‖浠惰緟鍔╄櫄鎷熷寲鑳藉姏銆傚繀椤诲悜 KVM_CREATE_VM 浼犻€掍竴涓悎閫傜殑
KVM_VM_MIPS_* 绫诲瀷鏉ュ垱寤轰竴涓埄鐢ㄥ畠鐨?VM銆?

濡傛灉鍦?kvm VM 鍙ユ焺涓婃墽琛?KVM_CHECK_EXTENSION 鎸囩ず姝よ兘鍔涘彲鐢紝鍒欐剰鍛崇潃璇?VM 姝ｅ湪
浣跨敤纭欢鐨勫畬鍏ㄧ‖浠惰緟鍔╄櫄鎷熷寲鑳藉姏銆傝繖鍦ㄧ敤 KVM_VM_MIPS_DEFAULT 鍒涘缓 VM 涔嬪悗妫€鏌?
寰堟湁鐢ㄣ€?

KVM_CHECK_EXTENSION 杩斿洖鐨勫€煎簲涓庡凡鐭ュ€硷紙瑙佷笅鏂囷級杩涜姣旇緝銆傛墍鏈夊叾浠栧€煎潎淇濈暀銆傝繖鏄?
涓轰簡鍏佽鍏朵粬鍙兘涓?MIPS VZ ASE 涓嶅吋瀹圭殑纭欢杈呭姪铏氭嫙鍖栧疄鐜板瓨鍦ㄧ殑鍙兘鎬с€?

==  ==========================================================================
 0  浣跨敤 trap & emulate 瀹炵幇鍦ㄧ敤鎴锋ā寮忎笅杩愯瀹㈡埛鏈轰唬鐮併€傚鎴锋満铏氭嫙鍐呭瓨娈佃閲嶆帓浠?
    浣垮鎴锋満閫傚簲浜庣敤鎴锋ā寮忓湴鍧€绌洪棿銆?

 1  浣跨敤 MIPS VZ ASE锛屾彁渚涘畬鍏ㄧ‖浠惰緟鍔╄櫄鎷熷寲锛屽寘鎷爣鍑嗙殑瀹㈡埛鏈鸿櫄鎷熷唴瀛樻銆?
==  ==========================================================================

### 8.7 KVM_CAP_MIPS_64BIT


:Architectures: mips

姝よ兘鍔涙寚绀哄鎴锋満鏀寔鐨勬灦鏋勭被鍨嬶紝鍗虫敮鎸佺殑瀵勫瓨鍣ㄥ拰鍦板潃瀹藉害銆?

褰撳湪 kvm VM 鍙ユ焺涓婇€氳繃 KVM_CHECK_EXTENSION 妫€鏌ユ鑳藉姏鏃讹紝杩斿洖鐨勫€煎ぇ鑷村搴斾簬
CP0_Config.AT 瀵勫瓨鍣ㄥ瓧娈碉紝骞跺簲閽堝宸茬煡鍊硷紙瑙佷笅鏂囷級涓撻棬妫€鏌ャ€傛墍鏈夊叾浠栧€煎潎淇濈暀銆?

==  ========================================================================
 0  MIPS32 鎴?microMIPS32銆傚瘎瀛樺櫒鍜屽湴鍧€鍧囦负 32 浣嶅銆傚彧鑳借繍琛?32 浣嶅鎴锋満浠ｇ爜銆?

 1  MIPS64 鎴?microMIPS64锛屼絾鍙兘璁块棶 32 浣嶅吋瀹规銆傚瘎瀛樺櫒涓?64 浣嶅锛屼絾鍦板潃涓?
    32 浣嶅銆傚彲浠ヨ繍琛?64 浣嶅鎴锋満浠ｇ爜锛屼絾鏃犳硶璁块棶 MIPS64 鍐呭瓨娈点€備篃鍙互杩愯 32 浣?
    瀹㈡埛鏈轰唬鐮併€?

 2  MIPS64 鎴?microMIPS64锛屽彲璁块棶鎵€鏈夊湴鍧€娈点€傚瘎瀛樺櫒鍜屽湴鍧€鍧囦负 64 浣嶅銆傚彲浠ヨ繍琛?
    64 浣嶆垨 32 浣嶅鎴锋満浠ｇ爜銆?
==  ========================================================================

### 8.9 KVM_CAP_ARM_USER_IRQ


:Architectures: arm64

姝よ兘鍔涳紝濡傛灉 KVM_CHECK_EXTENSION 鎸囩ず鍏跺彲鐢紝鎰忓懗鐫€濡傛灉鐢ㄦ埛绌洪棿鍒涘缓浜嗘病鏈夊唴鏍告€?
涓柇鎺у埗鍣ㄧ殑 VM锛屽畠灏嗘敹鍒板鍐呮牳鎬佹ā鎷熻澶囪緭鍑虹數骞冲彉鍖栫殑閫氱煡锛岃繖浜涜澶囧彲浠ョ敓鎴?
铏氭嫙涓柇骞跺憟鐜扮粰 VM銆傚浜庢绫?VM锛屾瘡娆¤繑鍥炲埌鐢ㄦ埛绌洪棿鏃讹紝鍐呮牳閮戒細鏇存柊 vcpu 鐨?
run->s.regs.device_irq_level 瀛楁浠ヨ〃绀鸿澶囩殑瀹為檯杈撳嚭鐢靛钩銆?

姣忓綋 kvm 妫€娴嬪埌璁惧杈撳嚭鐢靛钩鍙戠敓鍙樺寲鏃讹紝kvm 淇濊瘉鍦ㄨ繍琛?VM 涔嬪墠鑷冲皯杩斿洖涓€娆＄敤鎴风┖闂淬€?
姝ら€€鍑哄彲浠ユ槸 KVM_EXIT_INTR 鎴栦换浣曞叾浠栭€€鍑轰簨浠讹紝濡?KVM_EXIT_MMIO銆傝繖鏍凤紝鐢ㄦ埛绌洪棿
鎬绘槸鍙互閲囨牱璁惧杈撳嚭鐢靛钩骞堕噸鏂拌绠楃敤鎴风┖闂翠腑鏂帶鍒跺櫒鐨勭姸鎬併€傜敤鎴风┖闂村簲鎬绘槸鍦ㄦ瘡娆?
kvm 閫€鍑烘椂妫€鏌?run->s.regs.device_irq_level 鐨勭姸鎬併€俽un->s.regs.device_irq_level
涓殑鍊煎彲浠ヨ〃绀虹數骞宠Е鍙戝拰杈规部瑙﹀彂鐨勪腑鏂俊鍙凤紝鍙栧喅浜庤澶囥€傝竟娌胯Е鍙戠殑涓柇淇″彿灏嗗湪姣忔
杈规部淇″彿鏃朵互 run->s.regs.device_irq_level 涓殑浣嶆伆濂界疆浣嶄竴娆＄殑鏂瑰紡閫€鍑哄埌鐢ㄦ埛绌洪棿銆?

run->s.regs.device_irq_level 瀛楁鐨勫彲鐢ㄦ€т笉渚濊禆浜?run->kvm_valid_regs 鎴?
run->kvm_dirty_regs 浣嶃€?

濡傛灉鏀寔 KVM_CAP_ARM_USER_IRQ锛孠VM_CHECK_EXTENSION ioctl 杩斿洖涓€涓ぇ浜?0 鐨勬暟瀛楋紝
鎸囩ず鎵€瀹炵幇鐨勬鑳藉姏鐗堟湰锛屼粠鑰屾寚绀?run->s.regs.device_irq_level 涓殑鍝簺浣嶅彲浠ュ彂鍑?
淇″彿鍊笺€?

```

  KVM_CAP_ARM_USER_IRQ >= 1:

    KVM_ARM_DEV_EL1_VTIMER -  EL1 铏氭嫙瀹氭椂鍣?
    KVM_ARM_DEV_EL1_PTIMER -  EL1 鐗╃悊瀹氭椂鍣?
    KVM_ARM_DEV_PMU        -  ARM PMU 婧㈠嚭涓柇淇″彿

```
kvm 鐨勬湭鏉ョ増鏈彲鑳藉疄鐜伴澶栫殑浜嬩欢銆傝繖浜涘皢閫氳繃浠?KVM_CHECK_EXTENSION 杩斿洖鏇撮珮鐨勬暟瀛?
鏉ユ寚绀猴紝骞跺皢鍦ㄤ笂闈㈠垪鍑恒€?

### 8.10 KVM_CAP_PPC_SMT_POSSIBLE


:Architectures: ppc

鏌ヨ姝よ兘鍔涜繑鍥炰竴涓綅鍥撅紝鎸囩ず鍙互浣跨敤 KVM_CAP_PPC_SMT 璁剧疆鐨勮櫄鎷?SMT 妯″紡銆傚鏋?
锛堜粠鍙宠捣锛夌 N 浣嶈缃綅锛屽垯 2^N 鐨勮櫄鎷?SMT 妯″紡鍙敤銆?

### 8.12 KVM_CAP_HYPERV_VP_INDEX


:Architectures: x86

姝よ兘鍔涙寚绀虹敤鎴风┖闂村彲浠ュ姞杞?HV_X64_MSR_VP_INDEX msr銆傚叾鍊肩敤浜庤〃绀?SynIC 涓柇鐨?
鐩爣 vcpu銆備负浜嗗吋瀹规€э紝KVM 灏嗘 msr 鍒濆鍖栦负 KVM 鐨勫唴閮?vcpu 绱㈠紩銆傚綋姝よ兘鍔涗笉瀛樺湪
鏃讹紝鐢ㄦ埛绌洪棿浠嶅彲浠ユ煡璇㈡ msr 鐨勫€笺€?

### 8.13 KVM_CAP_S390_AIS_MIGRATION


:Architectures: s390

姝よ兘鍔涙寚绀?flic 璁惧鏄惁灏嗚兘澶熼€氳繃 KVM_DEV_FLIC_AISM_ALL 灞炴€ц幏鍙?璁剧疆鐢ㄤ簬杩佺Щ鐨?
AIS 鐘舵€侊紝骞跺厑璁稿湪涓嶅繀鍒涘缓 flic 璁惧鐨勬儏鍐典笅鍙戠幇杩欎竴鐐广€?

### 8.14 KVM_CAP_S390_PSW


:Architectures: s390

姝よ兘鍔涙寚绀?PSW 閫氳繃 kvm_run 缁撴瀯鏆撮湶銆?

### 8.15 KVM_CAP_S390_GMAP


:Architectures: s390

姝よ兘鍔涙寚绀虹敤浣滃鎴锋満鏄犲皠鐨勭敤鎴风┖闂村唴瀛樺彲浠ヤ綅浜庣敤鎴峰唴瀛樺湴鍧€绌洪棿涓殑浠讳綍浣嶇疆锛屽彧瑕?
鍐呭瓨妲芥寜娈碉紙1MB锛夎竟鐣屽榻愬苟璋冩暣澶у皬銆?

### 8.16 KVM_CAP_S390_COW


:Architectures: s390

姝よ兘鍔涙寚绀虹敤浣滃鎴锋満鏄犲皠鐨勭敤鎴风┖闂村唴瀛樺彲浠ヤ娇鐢ㄥ啓鏃跺鍒讹紙copy-on-write锛夎涔夛紝浠ュ強
閫氳繃鍙椤佃〃杩涜鑴忛〉璺熻釜銆?

### 8.17 KVM_CAP_S390_BPB


:Architectures: s390

姝よ兘鍔涙寚绀?kvm 灏嗗疄鐜扮敤浜庡鐞嗗垎鏀娴嬮樆濉炵殑閲嶇疆銆佽縼绉诲拰宓屽 KVM 鐨勬帴鍙ｃ€傚鏋滄病鏈?
姝よ兘鍔涳紝涓嶅簲鍚戝鎴锋満鎻愪緵 stfle facility 82銆?

### 8.18 KVM_CAP_HYPERV_TLBFLUSH


:Architectures: x86

姝よ兘鍔涙寚绀?KVM 鏀寔鍗婅櫄鎷熷寲 Hyper-V TLB 鍒锋柊瓒呯骇璋冪敤锛?
HvFlushVirtualAddressSpace銆丠vFlushVirtualAddressSpaceEx銆?
HvFlushVirtualAddressList銆丠vFlushVirtualAddressListEx銆?

### 8.19 KVM_CAP_ARM_INJECT_SERROR_ESR


:Architectures: arm64

姝よ兘鍔涙寚绀虹敤鎴风┖闂村彲浠ユ寚瀹氾紙閫氳繃 KVM_SET_VCPU_EVENTS ioctl锛夊綋瀹㈡埛鏈哄彂鐢熻櫄鎷?SError
涓柇寮傚父鏃舵姤鍛婄粰瀹㈡埛鏈虹殑缁煎悎寰侊紙syndrome锛夊€笺€傚鏋?KVM 閫氬憡姝よ兘鍔涳紝鐢ㄦ埛绌洪棿鍙兘鎸囧畾
ESR 缁煎悎寰佺殑 ISS 瀛楁銆侲SR 鐨勫叾浠栭儴鍒嗭紙渚嬪 EC锛夊湪寮傚父鍙戠敓鏃剁敱 CPU 鐢熸垚銆傚鏋滆繖涓?
铏氭嫙 SError 浣跨敤 AArch64 杩涘叆 EL1锛屾鍊煎皢鎶ュ憡鍦?ESR_ELx 鐨?ISS 瀛楁涓€?

鏇村缁嗚妭璇峰弬闃?KVM_CAP_VCPU_EVENTS銆?

### 8.20 KVM_CAP_HYPERV_SEND_IPI


:Architectures: x86

姝よ兘鍔涙寚绀?KVM 鏀寔鍗婅櫄鎷熷寲 Hyper-V IPI 鍙戦€佽秴绾ц皟鐢細
HvCallSendSyntheticClusterIpi銆丠vCallSendSyntheticClusterIpiEx銆?

### 8.22 KVM_CAP_S390_VCPU_RESETS


:Architectures: s390

姝よ兘鍔涙寚绀?KVM_S390_NORMAL_RESET 鍜?KVM_S390_CLEAR_RESET ioctl 鍙敤銆?

### 8.23 KVM_CAP_S390_PROTECTED


:Architectures: s390

姝よ兘鍔涙寚绀?Ultravisor 宸插垵濮嬪寲锛屽洜姝?KVM 鍙互鍚姩鍙椾繚鎶ょ殑 VM銆傛鑳藉姏绠¤緰
KVM_S390_PV_COMMAND ioctl 鍜?KVM_MP_STATE_LOAD MP_STATE銆傚浜庡彈淇濇姢鐨勫鎴锋満锛屽綋
鐘舵€佸彉鏇存棤鏁堟椂锛孠VM_SET_MP_STATE 鍙兘澶辫触銆?

### 8.24 KVM_CAP_STEAL_TIME


:Architectures: arm64, x86

姝よ兘鍔涙寚绀?KVM 鏀寔绐冨彇鏃堕棿锛坰teal time锛夎璐︺€傚綋鏀寔绐冨彇鏃堕棿璁拌处鏃讹紝鍙互閫氳繃
鏋舵瀯鐗瑰畾鐨勬帴鍙ｅ惎鐢ㄣ€傛鑳藉姏鍜屾灦鏋勭壒瀹氱殑鎺ュ彛蹇呴』涓€鑷达紝鍗冲鏋滀竴涓鏀寔璇ョ壒鎬э紝鍙︿竴涓?
涔熷簲璇ユ敮鎸侊紝鍙嶄箣浜︾劧銆傚浜?arm64锛岃鍙傞槄 Documentation/virt/kvm/devices/vcpu.rst 鐨?
鈥淜VM_ARM_VCPU_PVTIME_CTRL鈥濄€傚浜?x86锛岃鍙傞槄 Documentation/virt/kvm/x86/msr.rst 鐨?
鈥淢SR_KVM_STEAL_TIME鈥濄€?

### 8.25 KVM_CAP_S390_DIAG318


:Architectures: s390

姝よ兘鍔涗娇瀹㈡埛鏈鸿兘澶熻缃湁鍏冲叾鎺у埗绋嬪簭锛堝嵆瀹㈡埛鏈哄唴鏍哥被鍨嬪拰鐗堟湰锛夌殑淇℃伅銆傝繖浜涗俊鎭湪
绯荤粺/鍥轰欢鏈嶅姟浜嬩欢鏈熼棿寰堟湁甯姪锛屾彁渚涘叧浜庢満鍣ㄤ笂杩愯鐨勫鎴锋満鐜鐨勯澶栨暟鎹€?

璇ヤ俊鎭笌 DIAGNOSE 0x318 鎸囦护鐩稿叧鑱旓紝璇ユ寚浠よ缃竴涓?8 瀛楄妭鐨勫€硷紝鐢变竴涓瓧鑺傜殑鎺у埗
绋嬪簭鍚嶄唬鐮侊紙CPNC锛夊拰 7 瀛楄妭鐨勬帶鍒剁▼搴忕増鏈唬鐮侊紙CPVC锛夌粍鎴愩€侰PNC 纭畾鎺у埗绋嬪簭杩愯
浜庝綍绉嶇幆澧冿紙渚嬪 Linux銆亃/VM鈥︹€︼級锛孋PVC 鐢ㄤ簬 OS 鐗瑰畾鐨勪俊鎭紙渚嬪 Linux 鐗堟湰銆?
Linux 鍙戣鐗堚€︹€︼級銆?

濡傛灉姝よ兘鍔涘彲鐢紝鍒?CPNC 鍜?CPVC 鍙互閫氳繃鍚屾瀵勫瓨鍣ㄦ満鍒讹紙KVM_SYNC_DIAG318锛夊湪 KVM
鍜岀敤鎴风┖闂翠箣闂村悓姝ャ€?

### 8.26 KVM_CAP_X86_USER_SPACE_MSR


:Architectures: x86

姝よ兘鍔涙寚绀?KVM 鏀寔灏?MSR 璇诲彇鍜屽啓鍏ヨ浆鍚戠敤鎴风┖闂淬€傚畠鍙互鍦?VM 绾у埆鍚敤銆傚鏋滃惎鐢紝
閫氬父浼氱敱 KVM 鍚戝鎴锋満瑙﹀彂 #GP 鐨?MSR 璁块棶锛屽皢鏀逛负閫氳繃 KVM_EXIT_X86_RDMSR 鍜?
KVM_EXIT_X86_WRMSR 閫€鍑洪€氱煡寮瑰洖鐢ㄦ埛绌洪棿銆?

### 8.27 KVM_CAP_X86_MSR_FILTER


:Architectures: x86

姝よ兘鍔涙寚绀?KVM 鏀寔鎷掔粷璁块棶鐢ㄦ埛瀹氫箟鐨?MSR銆傛毚闇叉鑳藉姏鍚庯紝KVM 瀵煎嚭鏂扮殑 VM ioctl
KVM_X86_SET_MSR_FILTER锛岀敤鎴风┖闂村彲浠ヨ皟鐢ㄥ畠鏉ユ寚瀹?KVM 搴旀嫆缁濊闂殑 MSR 鑼冨洿鐨勪綅鍥俱€?

缁撳悎 KVM_CAP_X86_USER_SPACE_MSR锛岃繖鍏佽鐢ㄦ埛绌洪棿鎹曡幏骞舵ā鎷熻秴鍑?KVM 鑼冨洿鐨?MSR锛屼互鍙?
闄愬埗 KVM 鐨?MSR 妯℃嫙浠ｇ爜鐨勬敾鍑婚潰銆?

### 8.30 KVM_CAP_XEN_HVM


:Architectures: x86

姝よ兘鍔涙寚绀?Xen 鏀寔鐨勭敤浜庢墭绠?Xen 鐨勭壒鎬?
```

  #define KVM_XEN_HVM_CONFIG_HYPERCALL_MSR		(1 << 0)
  #define KVM_XEN_HVM_CONFIG_INTERCEPT_HCALL		(1 << 1)
  #define KVM_XEN_HVM_CONFIG_SHARED_INFO		(1 << 2)
  #define KVM_XEN_HVM_CONFIG_RUNSTATE			(1 << 3)
  #define KVM_XEN_HVM_CONFIG_EVTCHN_2LEVEL		(1 << 4)
  #define KVM_XEN_HVM_CONFIG_EVTCHN_SEND		(1 << 5)
  #define KVM_XEN_HVM_CONFIG_RUNSTATE_UPDATE_FLAG	(1 << 6)
  #define KVM_XEN_HVM_CONFIG_PVCLOCK_TSC_UNSTABLE	(1 << 7)

```
KVM_XEN_HVM_CONFIG_HYPERCALL_MSR 鏍囧織鎸囩ず KVM_XEN_HVM_CONFIG ioctl 鍙敤锛屼緵瀹㈡埛鏈?
璁剧疆鍏惰秴绾ц皟鐢ㄩ〉銆?

濡傛灉涔熻缃簡 KVM_XEN_HVM_CONFIG_INTERCEPT_HCALL锛屽垯鍙互鍦ㄦ彁渚涚粰 KVM_XEN_HVM_CONFIG
鐨?flags 涓彁渚涚浉鍚岀殑鏍囧織锛堜笉鎻愪緵瓒呯骇璋冪敤椤靛唴瀹癸級锛屼互璇锋眰 KVM 鑷姩鐢熸垚瓒呯骇璋冪敤椤?
鍐呭锛屽苟鍚敤瀵瑰鎴锋満瓒呯骇璋冪敤鐨勬嫤鎴紙KVM_EXIT_XEN锛夈€?

KVM_XEN_HVM_CONFIG_SHARED_INFO 鏍囧織鎸囩ず KVM_XEN_HVM_SET_ATTR銆並VM_XEN_HVM_GET_ATTR銆?
KVM_XEN_VCPU_SET_ATTR 鍜?KVM_XEN_VCPU_GET_ATTR ioctl 鐨勫彲鐢ㄦ€э紝浠ュ強鍦?vcpu 鐨?
vcpu_info 鐨?evtchn_upcall_pending 瀛楁琚疆浣嶆椂閫掗€佷簨浠堕€氶亾 upcall 鐨勫紓甯稿悜閲忋€?

KVM_XEN_HVM_CONFIG_RUNSTATE 鏍囧織鎸囩ず runstate 鐩稿叧鐗规€?
KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_ADDR/_CURRENT/_DATA/_ADJUST 鍙?
KVM_XEN_VCPU_SET_ATTR/KVM_XEN_VCPU_GET_ATTR ioctl 鏀寔銆?

KVM_XEN_HVM_CONFIG_EVTCHN_2LEVEL 鏍囧織鎸囩ず鏀寔绫诲瀷涓?KVM_IRQ_ROUTING_XEN_EVTCHN 鐨?
IRQ 璺敱鏉＄洰锛屽叾 priority 瀛楁琚涓鸿〃绀?2 绾т簨浠堕€氶亾閫掗€併€?

KVM_XEN_HVM_CONFIG_EVTCHN_SEND 鏍囧織鎸囩ず KVM 鏀寔浣跨敤 KVM_XEN_HVM_EVTCHN_SEND ioctl
灏嗕簨浠堕€氶亾浜嬩欢鐩存帴娉ㄥ叆瀹㈡埛鏈恒€傚畠杩樻寚绀烘敮鎸?KVM_XEN_ATTR_TYPE_EVTCHN/XEN_VERSION HVM
灞炴€э紝浠ュ強 KVM_XEN_VCPU_ATTR_TYPE_VCPU_ID/TIMER/UPCALL_VECTOR vCPU 灞炴€э紝杩欎簺涓庝簨浠?
閫氶亾閫掗€併€佸畾鏃跺櫒浠ュ強 XENVER_version 鎷︽埅鐩稿叧銆?

KVM_XEN_HVM_CONFIG_RUNSTATE_UPDATE_FLAG 鏍囧織鎸囩ず KVM 鍦?KVM_XEN_SET_ATTR 鍜?
KVM_XEN_GET_ATTR ioctl 涓敮鎸?KVM_XEN_ATTR_TYPE_RUNSTATE_UPDATE_FLAG 灞炴€с€傝繖鎺у埗 KVM
鏄惁浼氬湪鏇存柊 runstate 淇℃伅鏃惰缃鎴锋満鍐呭瓨鏄犲皠鐨?vcpu_runstate_info 涓殑
XEN_RUNSTATE_UPDATE 鏍囧織銆傛敞鎰忥紝鏀寔涓婅堪 RUNSTATE 鐗规€т絾涓嶆敮鎸?RUNSTATE_UPDATE_FLAG
鐗规€х殑 KVM 鐗堟湰锛屽湪鏇存柊瀹㈡埛鏈虹粨鏋勬椂鎬绘槸浼氳缃?XEN_RUNSTATE_UPDATE 鏍囧織锛岃繖涔熻鏈夋倴
鐩磋銆傚綋閫氬憡姝ゆ爣蹇楁椂锛孠VM 鐨勮涓哄皢鏇存纭紝鍦紙鐢卞鎴锋満鍙戣捣瓒呯骇璋冪敤銆佸鑷?VMM 鍚敤
KVM_XEN_ATTR_TYPE_RUNSTATE_UPDATE_FLAG 灞炴€э級涔嬪墠涓嶄細浣跨敤 XEN_RUNSTATE_UPDATE 鏍囧織銆?

KVM_XEN_HVM_CONFIG_PVCLOCK_TSC_UNSTABLE 鏍囧織鎸囩ず KVM 鏀寔鍦?Xen pvclock 婧愪腑娓呴櫎
PVCLOCK_TSC_STABLE_BIT 鏍囧織銆傝繖灏嗗湪 KVM_CAP_XEN_HVM ioctl 璁剧疆
KVM_XEN_HVM_CONFIG_PVCLOCK_TSC_UNSTABLE 鏍囧織鏃跺畬鎴愩€?

### 8.31 KVM_CAP_SPAPR_MULTITCE


:Architectures: ppc
:Type: vm

姝よ兘鍔涙剰鍛崇潃鍐呮牳鑳藉澶勭悊瓒呰皟鐢?H_PUT_TCE_INDIRECT 鍜?H_STUFF_TCE锛岃€屾棤闇€灏嗚繖浜涗紶閫?
鍒扮敤鎴风┖闂淬€傝繖鏄捐憲鍔犻€熶簡 PPC KVM 瀹㈡埛鏈虹殑 DMA 鎿嶄綔銆傚鏋滅敤鎴风┖闂翠箣鍓嶅凡鍦?KVM 涓?
娉ㄥ唽浜?LIOBN锛堥€氳繃 KVM_CREATE_SPAPR_TCE 鎴栫被浼艰皟鐢級锛岀敤鎴风┖闂村簲棰勬湡杩欎簺瓒呯骇璋冪敤鐨?
澶勭悊绋嬪簭涓嶄細琚皟鐢ㄣ€?

涓轰簡鍦ㄥ鎴锋満涓惎鐢?H_PUT_TCE_INDIRECT 鍜?H_STUFF_TCE 鐨勪娇鐢紝鐢ㄦ埛绌洪棿鍙兘蹇呴』涓哄鎴锋満
閫氬憡瀹冦€備緥濡傦紝濡傛灉鈥渋bm,hypertas-functions鈥濊澶囨爲灞炴€т腑瀛樺湪鈥渉call-multi-tce鈥濓紝IBM
pSeries锛坰PAPR锛夊鎴锋満灏变細寮€濮嬩娇鐢ㄥ畠浠€?

涓婅堪瓒呯骇璋冪敤鍙兘鍦ㄤ篃鍙兘涓嶅湪鍩轰簬鍐呮牳鐨勫揩閫熻矾寰勪腑鎴愬姛澶勭悊銆傚鏋滃唴鏍告棤娉曞鐞嗗畠浠紝瀹冧滑
灏嗚浼犻€掔粰鐢ㄦ埛绌洪棿銆傚洜姝わ紝灏界鏈夊唴鏍告€佸姞閫燂紝鐢ㄦ埛绌洪棿浠嶇劧蹇呴』涓鸿繖浜涜秴绾ц皟鐢ㄤ繚鐣欏疄鐜般€?

姝よ兘鍔涘缁堝惎鐢ㄣ€?

### 8.32 KVM_CAP_PTP_KVM


:Architectures: arm64

姝よ兘鍔涙寚绀哄涓绘敮鎸?KVM 铏氭嫙 PTP 鏈嶅姟銆俈MM 鍙互鍦ㄨ縼绉绘椂妫€鏌ヨ鏈嶅姟瀵瑰鎴锋満鏄惁鍙敤銆?

### 8.37 KVM_CAP_S390_PROTECTED_DUMP


:Architectures: s390
:Type: vm

姝よ兘鍔涙寚绀?KVM 鍜?Ultravisor 鏀寔杞偍 PV 瀹㈡埛鏈恒€俙KVM_PV_DUMP` 鍛戒护鍙敤浜?
`KVM_S390_PV_COMMAND` ioctl锛宍KVM_PV_INFO` 鍛戒护鎻愪緵涓庤浆鍌ㄧ浉鍏崇殑 UV 鏁版嵁銆傛澶栵紝vcpu
ioctl `KVM_S390_PV_CPU_COMMAND` 涔熷彲鐢紝骞舵敮鎸?`KVM_PV_DUMP_CPU` 瀛愬懡浠ゃ€?

### 8.39 KVM_CAP_S390_CPU_TOPOLOGY


:Architectures: s390
:Type: vm

姝よ兘鍔涙寚绀?KVM 灏嗘彁渚?S390 CPU 鎷撴墤璁炬柦锛屽畠鍖呮嫭瀵瑰姛鑳界爜 2 鐨?PTF 鎸囦护鐨勮В閲婏紝浠ュ強瀵?
鍔熻兘鐮?0 鎴?1 鐨?PTF 鎸囦护涓?STSI(15,1,x) 鎸囦护鐨勬嫤鎴拰杞彂鍒扮敤鎴锋€佽櫄鎷熸満鐩戞帶鍣ㄣ€?

濡傛灉娌℃湁姝よ兘鍔涳紝涓嶅簲鍚戝鎴锋満鎸囩ず stfle facility 11锛圕PU 鎷撴墤璁炬柦锛夈€?

瀛樺湪姝よ兘鍔涙椂锛孠VM 鍦?vm fd 涓婃彁渚涗竴涓柊鐨勫睘鎬х粍 KVM_S390_VM_CPU_TOPOLOGY銆傝繖涓柊鐨?
灞炴€у厑璁搁€氳繃 kvm_device_attr 缁撴瀯鑾峰彇銆佽缃垨娓呴櫎 SCA 鐨?Modified Change Topology
Report锛圡TCR锛変綅銆?

褰撹幏鍙?Modified Change Topology Report 鍊兼椂锛宎ttr->addr 蹇呴』鎸囧悜涓€涓瓧鑺傦紝鍊煎皢瀛樺偍鍒?
鍏朵腑鎴栦粠涓彇鍑恒€?

### 8.41 KVM_CAP_VM_TYPES


:Architectures: x86
:Type: system ioctl

姝よ兘鍔涜繑鍥炲彈鏀寔 VM 绫诲瀷鐨勪綅鍥俱€備綅 @n 缃?1 琛ㄧず
```

  #define KVM_X86_DEFAULT_VM	0
  #define KVM_X86_SW_PROTECTED_VM	1
  #define KVM_X86_SEV_VM	2
  #define KVM_X86_SEV_ES_VM	3

```
娉ㄦ剰锛孠VM_X86_SW_PROTECTED_VM 鐩墠浠呯敤浜庡紑鍙戝拰娴嬭瘯銆備笉瑕佸皢 KVM_X86_SW_PROTECTED_VM
鐢ㄤ簬鈥滅湡姝ｇ殑鈥漋M锛屽挨鍏舵槸涓嶈鐢ㄤ簬鐢熶骇鐜銆傝蒋浠朵繚鎶ょ殑 VM 鐨勮涓哄拰鏈夋晥 ABI 鏄笉绋冲畾鐨勩€?

### 8.42 KVM_CAP_PPC_RPT_INVALIDATE


:Architectures: ppc

姝よ兘鍔涙寚绀哄唴鏍歌兘澶熷鐞?H_RPT_INVALIDATE 瓒呯骇璋冪敤銆?

涓轰簡鍦ㄥ鎴锋満涓惎鐢?H_RPT_INVALIDATE 鐨勪娇鐢紝鐢ㄦ埛绌洪棿鍙兘蹇呴』涓哄鎴锋満閫氬憡瀹冦€備緥濡傦紝
濡傛灉鈥渋bm,hypertas-functions鈥濊澶囨爲灞炴€т腑瀛樺湪鈥渉call-rpt-invalidate鈥濓紝IBM pSeries
锛坰PAPR锛夊鎴锋満灏变細寮€濮嬩娇鐢ㄥ畠銆?

姝よ兘鍔涘湪鏀寔 radix MMU 鐨?POWER9 绛夊钩鍙颁笂鐨勮櫄鎷熸満鐩戞帶鍣ㄤ腑鍚敤銆?

### 8.43 KVM_CAP_PPC_AIL_MODE_3


:Architectures: ppc

姝よ兘鍔涙寚绀哄唴鏍告敮鎸侀€氳繃 H_SET_MODE 瓒呯骇璋冪敤鎺у埗鐨勨€滀腑鏂椂鐨勫湴鍧€杞崲妯″紡鈥濓紙Address
Translation Mode on Interrupt锛夛紝鍙堢О鈥滃鐢ㄤ腑鏂綅缃€濓紙Alternate Interrupt Location锛?
璧勬簮鐨勬ā寮?3 璁剧疆銆?

姝よ兘鍔涘厑璁稿鎴锋満鍐呮牳浣跨敤鏇撮珮鎬ц兘鐨勬ā寮忔潵澶勭悊涓柇鍜岀郴缁熻皟鐢ㄣ€?

### 8.44 KVM_CAP_MEMORY_FAULT_INFO


:Architectures: x86

瀛樺湪姝よ兘鍔涙寚绀猴紝濡傛灉 KVM 鏃犳硶瑙ｆ瀽瀹㈡埛鏈洪〉鏁呴殰 VM-Exit锛堜緥濡傚瓨鍦ㄦ湁鏁堢殑 memslot 浣?
鐩稿簲鐨勫涓昏櫄鎷熷湴鍧€娌℃湁鍚庡 VMA锛夛紝KVM_RUN 灏嗗～鍏?kvm_run.memory_fault銆?

kvm_run.memory_fault 涓殑淇℃伅褰撲笖浠呭綋 KVM_RUN 浠?errno=EFAULT 鎴?errno=EHWPOISON
閿欒杩斿洖 **骞朵笖** kvm_run.exit_reason 琚涓?KVM_EXIT_MEMORY_FAULT 鏃舵墠鏈夋晥銆?

娉ㄦ剰锛氬皾璇曡В鍐冲唴瀛樻晠闅滀互閲嶈瘯 KVM_RUN 鐨勭敤鎴风┖闂村簲娉ㄦ剰闃叉閲嶅鏀跺埌鐩稿悓鐨勯敊璇?甯︽敞瑙?
鏁呴殰銆?

鏇村淇℃伅璇峰弬闃?KVM_EXIT_MEMORY_FAULT銆?

### 8.45 KVM_CAP_X86_GUEST_MODE


:Architectures: x86

瀛樺湪姝よ兘鍔涙寚绀?KVM_RUN 灏嗘洿鏂?kvm_run.flags 涓殑 KVM_RUN_X86_GUEST_MODE 浣嶏紝浠ユ寚绀?
vCPU 閫€鍑烘椂鏄惁姝ｅ湪鎵ц宓屽瀹㈡埛鏈轰唬鐮併€?

### 8.46 KVM_CAP_S390_KEYOP


:Architectures: s390

瀛樺湪姝よ兘鍔涙寚绀?KVM_S390_KEYOP ioctl 鍙敤銆?

KVM 閫€鍑烘椂甯︽湁 L1 鎴?L2 瀹㈡埛鏈虹殑瀵勫瓨鍣ㄧ姸鎬侊紝鍙栧喅浜庨€€鍑烘椂鎵ц鐨勬槸鍝竴涓€傜敤鎴风┖闂村繀椤?
娉ㄦ剰鍖哄垎杩欎簺鎯呭喌銆?

### 8.47 KVM_CAP_S390_VSIE_ESAMODE


:Architectures: s390

瀛樺湪姝よ兘鍔涙寚绀哄祵濂?KVM 瀹㈡埛鏈哄彲浠ヤ互 ESA 妯″紡鍚姩銆?

## 9. 宸茬煡鐨?KVM API 闂


鍦ㄦ煇浜涙儏鍐典笅锛孠VM 鐨?API 瀛樺湪涓€浜涗笉涓€鑷存垨鐢ㄦ埛绌洪棿闇€瑕佹敞鎰忕殑甯歌闄烽槺銆傛湰鑺傝杩板叾涓?
涓€浜涢棶棰樸€?

鍏朵腑澶ч儴鍒嗘槸鏋舵瀯鐗瑰畾鐨勶紝鍥犳鏈妭鎸夋灦鏋勫垝鍒嗐€?

### 9.1. x86


##### ``KVM_GET_SUPPORTED_CPUID`` 闂


閫氬父锛宍KVM_GET_SUPPORTED_CPUID` 鐨勮璁′娇寰楀彲浠ュ皢鍏剁粨鏋滅洿鎺ヤ紶缁?`KVM_SET_CPUID2`銆?
鏈妭璁板綍浜嗕竴浜涢渶瑕佺壒鍒皬蹇冪殑鎯呭喌銆?

#### 鏈湴 APIC 鐗规€?


CPU[EAX=1]:ECX[^21^]锛圶2APIC锛夌敱 `KVM_GET_SUPPORTED_CPUID` 鎶ュ憡锛屼絾鍙湁鍦ㄤ娇鐢?
`KVM_CREATE_IRQCHIP` 鎴?`KVM_ENABLE_CAP(KVM_CAP_IRQCHIP_SPLIT)` 鏉ュ惎鐢ㄦ湰鍦?APIC 鐨?
鍐呮牳鎬佹ā鎷熸椂锛屾墠鑳藉惎鐢ㄥ畠銆?

瀵逛簬 `KVM_FEATURE_PV_UNHALT` 鍗婅櫄鎷熷寲鐗规€т篃鏄姝ゃ€?

鍦ㄨ緝鏃х増鏈殑 Linux 涓婏紝`KVM_GET_SUPPORTED_CPUID` 涓嶆姤鍛?CPU[EAX=1]:ECX[^24^]
锛圱SC_DEADLINE锛夛紝浣嗗鏋滃瓨鍦?`KVM_CAP_TSC_DEADLINE_TIMER` 涓斿唴鏍稿凡鍚敤鏈湴 APIC 鐨?
鍐呮牳鎬佹ā鎷燂紝鍒欏彲浠ュ惎鐢ㄥ畠銆傚湪杈冩柊鐗堟湰涓婏紝`KVM_GET_SUPPORTED_CPUID` 纭疄灏嗚浣嶆姤鍛婁负
鍙敤銆?

#### CPU 鎷撴墤


鍑犱釜 CPUID 鍊煎寘鍚涓?CPU 鐨勬嫇鎵戜俊鎭細Intel 绯荤粺鐨?0x0b 鍜?0x1f锛孉MD 绯荤粺鐨?
0x8000001e銆備笉鍚岀増鏈殑 KVM 涓烘淇℃伅杩斿洖涓嶅悓鐨勫€硷紝鐢ㄦ埛绌洪棿涓嶅簲渚濊禆瀹冦€傚綋鍓嶅畠浠繑鍥?
鍏ㄩ浂銆?

濡傛灉鐢ㄦ埛绌洪棿甯屾湜璁剧疆瀹㈡埛鏈烘嫇鎵戯紝搴旀敞鎰忚繖涓変釜鍙讹紙leaf锛夌殑鍊煎浜庢瘡涓?CPU 閮戒笉鍚屻€傜壒鍒?
鏄紝APIC ID 浣嶄簬 0x0b 鍜?0x1f 鎵€鏈夊瓙鍙剁殑 EDX 涓紝浠ュ強 0x8000001e 鐨?EAX 涓紱鍚庤€呰繕灏?
鏍稿績 id 鍜岃妭鐐?id 鍒嗗埆缂栫爜鍦?EBX 鍜?ECX 鐨?7:0 浣嶄腑銆?

##### 宸插簾寮冪殑 ioctl 涓庤兘鍔?


KVM_CAP_DISABLE_QUIRKS 涓嶄細璁╃敤鎴风┖闂寸煡閬撳摢浜涙€櫀瀹為檯鍙敤銆傚鏋滃彲鐢紝璇锋敼鐢?
`KVM_CHECK_EXTENSION(KVM_CAP_DISABLE_QUIRKS2)`銆?

##### KVM_GET_*/KVM_SET_* ioctl 鐨勯『搴?


TBD
