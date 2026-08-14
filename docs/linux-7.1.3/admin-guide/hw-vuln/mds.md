## MDS - 寰灦鏋勬暟鎹噰鏍?
寰灦鏋勬暟鎹噰鏍凤紙Microarchitectural Data Sampling锛夋槸涓€绉嶇‖浠舵紡娲烇紝瀹冨厑璁稿 CPU 鍐呴儴鍚勭被缂撳啿鍖轰腑鍙敤鐨勬暟鎹繘琛屾棤鐗规潈鐨勬帹娴嬫€ц闂€?
### 鍙楀奖鍝嶇殑澶勭悊鍣?
璇ユ紡娲炲奖鍝嶈寖鍥村箍娉涚殑 Intel 澶勭悊鍣ㄣ€備互涓嬪鐞嗗櫒涓嶅彈褰卞搷锛?
   - 鏉ヨ嚜 AMD銆丆entaur 浠ュ強鍏朵粬闈?Intel 鍘傚晢鐨勫鐞嗗櫒

   - CPU 绯诲垪锛坒amily锛? 6 鐨勮緝鏃у鐞嗗櫒鍨嬪彿

   - 閮ㄥ垎 Atom 澶勭悊鍣紙Bonnell銆丼altwell銆丟oldmont銆丟oldmontPlus锛?
   - 鍦?IA32_ARCH_CAPABILITIES MSR 涓缃簡 ARCH_CAP_MDS_NO 浣嶇殑 Intel 澶勭悊鍣ㄣ€?
鏌愪釜澶勭悊鍣ㄦ槸鍚﹀彈褰卞搷锛屽彲浠ヤ粠 sysfs 涓殑 MDS 婕忔礊鏂囦欢涓鍑恒€傚弬瑙?mds_sys_info銆?
骞堕潪鎵€鏈夊鐞嗗櫒閮戒細鍙楀埌 MDS 鎵€鏈夊彉浣撶殑褰卞搷锛屼絾瀵瑰畠浠殑缂撹В鎺柦閮芥槸鐩稿悓鐨勶紝鍥犳鍐呮牳灏嗗畠浠綋浣滃崟涓€婕忔礊鏉ュ鐞嗐€?
### 鐩稿叧鐨?CVE

浠ヤ笅 CVE 鏉＄洰涓?MDS 婕忔礊鐩稿叧锛?
   ==============  =====  ===================================================
   CVE-2018-12126  MSBDS  Microarchitectural Store Buffer Data Sampling
   CVE-2018-12130  MFBDS  Microarchitectural Fill Buffer Data Sampling
   CVE-2018-12127  MLPDS  Microarchitectural Load Port Data Sampling
   CVE-2019-11091  MDSUM  Microarchitectural Data Sampling Uncacheable Memory
   ==============  =====  ===================================================

### 闂

鍦ㄦ墽琛屽瓨鍌紙store锛夈€佸姞杞斤紙load锛夈€丩1 濉厖锛坮efill锛夌瓑鎿嶄綔鏃讹紝澶勭悊鍣ㄤ細灏嗘暟鎹啓鍏ヤ复鏃剁殑寰灦鏋勭粨鏋勶紙缂撳啿鍖猴級涓€備綔涓轰紭鍖栨墜娈碉紝缂撳啿鍖轰腑鐨勬暟鎹彲浠ヨ杞彂缁欏姞杞芥搷浣溿€?
鍦ㄦ煇浜涙潯浠朵笅锛堥€氬父鏄敱鏌愪釜鍔犺浇鎿嶄綔寮曡捣鐨?fault/assist锛夛紝涓庡姞杞藉唴瀛樺湴鍧€鏃犲叧鐨勬暟鎹彲鑳戒細浠庣紦鍐插尯涓鎺ㄦ祴鎬у湴杞彂銆傜敱浜庡姞杞芥搷浣滃鑷翠簡 fault 鎴?assist锛屽叾缁撴灉灏嗚涓㈠純锛屽洜姝よ杞彂鐨勬暟鎹笉浼氬鑷撮敊璇殑绋嬪簭鎵ц鎴栫姸鎬佹敼鍙樸€備絾鎭舵剰鎿嶄綔鍙兘鑳藉灏嗘鎺ㄦ祴鎬ф暟鎹浆鍙戝埌涓€涓硠闇诧紙disclosure锛塯adget锛屼粠鑰屽彲浠ラ€氳繃缂撳瓨渚т俊閬擄紙cache side channel锛夋敾鍑绘帹鏂嚭鍏跺€笺€?
鐢变簬缂撳啿鍖烘湁鍙兘鍦ㄨ秴绾跨▼锛圚yper-Thread锛変箣闂村叡浜紝鍥犳璺ㄨ秴绾跨▼鐨勬敾鍑绘槸鍙兘鐨勩€?
鏇存繁鍏ョ殑鎶€鏈俊鎭彲鍦?MDS 鐗瑰畾鐨?x86 浣撶郴缁撴瀯绔犺妭涓壘鍒帮細Documentation/arch/x86/mds.rst <mds>銆?
### 鏀诲嚮鍦烘櫙

閽堝 MDS 婕忔礊鐨勬敾鍑诲彲浠ョ敱杩愯鍦ㄥ涓绘満鎴栧鎴锋満涓婄殑鎭舵剰銆佹棤鐗规潈鐨勭敤鎴风┖闂村簲鐢ㄧ▼搴忓彂璧枫€傛伓鎰忕殑瀹㈡埛鏈烘搷浣滅郴缁熸樉鐒朵篃鍙互鍙戣捣鏀诲嚮銆?
涓庡叾浠栧熀浜庢帹娴嬬殑婕忔礊涓嶅悓锛孧DS 婕忔礊涓嶅厑璁告敾鍑昏€呮帶鍒跺唴瀛樼洰鏍囧湴鍧€銆傚洜姝わ紝鏀诲嚮绾补鏄熀浜庨噰鏍风殑锛屼絾姝ｅ TLBleed 鏀诲嚮鎵€灞曠ず鐨勶紝鏍锋湰鍙互琚垚鍔熷湴杩涜鍚庡鐞嗐€?
##### Web 娴忚鍣?
  鐩墠灏氫笉娓呮閫氳繃 Web 娴忚鍣ㄥ彂璧锋敾鍑绘槸鍚﹀彲鑳姐€傞€氳繃 Java-Script 杩涜鍒╃敤琚涓烘瀬涓嶅彲鑳斤紝浣嗗叾浠栧箍娉涗娇鐢ㄧ殑 Web 鎶€鏈紙濡?Webassembly锛夋湁鍙兘琚互鐢ㄣ€?
### MDS 绯荤粺淇℃伅

Linux 鍐呮牳鎻愪緵涓€涓?sysfs 鎺ュ彛锛岀敤浜庢灇涓剧郴缁熷綋鍓嶇殑 MDS 鐘舵€侊細绯荤粺鏄惁鏄撳彈鏀诲嚮锛屼互鍙婂摢浜涚紦瑙ｆ帾鏂藉浜庢椿鍔ㄧ姸鎬併€傜浉鍏崇殑 sysfs 鏂囦欢鏄細

/sys/devices/system/cpu/vulnerabilities/mds

璇ユ枃浠朵腑鍙兘鐨勫€间负锛?
```

     * - 'Not affected'
       - The processor is not vulnerable
     * - 'Vulnerable'
       - The processor is vulnerable, but no mitigation enabled
     * - 'Vulnerable: Clear CPU buffers attempted, no microcode'
       - The processor is vulnerable but microcode is not updated. The
         mitigation is enabled on a best effort basis.

         If the processor is vulnerable but the availability of the microcode
         based mitigation mechanism is not advertised via CPUID, the kernel
         selects a best effort mitigation mode. This mode invokes the mitigation
         instructions without a guarantee that they clear the CPU buffers.

         This is done to address virtualization scenarios where the host has the
         microcode update applied, but the hypervisor is not yet updated to
         expose the CPUID to the guest. If the host has updated microcode the
         protection takes effect; otherwise a few CPU cycles are wasted
         pointlessly.
     * - 'Mitigation: Clear CPU buffers'
       - The processor is vulnerable and the CPU buffer clearing mitigation is
         enabled.

```
濡傛灉澶勭悊鍣ㄦ槗鍙楁敾鍑伙紝鍒欎細鍦ㄤ笂杩颁俊鎭箣鍚庤拷鍔犱互涓嬩俊鎭細

    ========================  ============================================
    'SMT vulnerable'          SMT is enabled
    'SMT mitigated'           SMT is enabled and mitigated
    'SMT disabled'            SMT is disabled
    'SMT Host state unknown'  Kernel runs in a VM, Host SMT state unknown
    ========================  ============================================

### 缂撹В鏈哄埗

鍐呮牳浼氭娴嬪彈褰卞搷 CPU锛屼互鍙婃墍闇€寰爜鐨勫瓨鍦ㄣ€?
濡傛灉鏌愪釜 CPU 鍙楀奖鍝嶄笖寰爜鍙敤锛屽垯鍐呮牳榛樿鍚敤缂撹В鎺柦銆傝缂撹В鎺柦鍙互鍦ㄥ惎鍔ㄦ椂閫氳繃鍐呮牳鍛戒护琛岄€夐」杩涜鎺у埗銆傚弬瑙?mds_mitigation_control_command_line銆?
##### CPU 缂撳啿鍖烘竻闄?
  閽堝 MDS 鐨勭紦瑙ｆ帾鏂戒細鍦ㄨ繑鍥炵敤鎴风┖闂翠互鍙婅繘鍏ュ鎴锋満鏃舵竻闄ゅ彈褰卞搷鐨?CPU 缂撳啿鍖恒€?
  濡傛灉鍚敤浜?SMT锛屽苟涓旇 CPU 鍙槸鍙?MSBDS 褰卞搷鑰屼笉鍙楀叾浠栦换浣?MDS 鍙樹綋褰卞搷锛岄偅涔堝畠杩樹細鍦ㄧ┖闂诧紙idle锛夎繘鍏ユ椂娓呴櫎缂撳啿鍖猴紝鍥犱负鍏朵粬鍙樹綋鏃犳硶闃插尽璺ㄨ秴绾跨▼鏀诲嚮銆?
  瀵逛簬浠呭彈 MSBDS 褰卞搷鐨?CPU锛岀敤鎴风┖闂淬€佸鎴锋満鍜岀┖闂插垏鎹㈣繖鍑犵缂撹В鎺柦宸茬粡瓒冲锛孲MT 涓嶅彈褰卞搷銆?
##### 铏氭嫙鍖栫紦瑙?
  瀹夸富鍒板鎴锋満鐨勫垏鎹繚鎶ゅ彇鍐充簬 CPU 鐨?L1TF 婕忔礊鎯呭喌锛?
  - CPU 鍙?L1TF 褰卞搷锛?
    濡傛灉鍚敤浜?L1D flush 缂撹В鎺柦锛屽苟涓斿彲鐢ㄧ殑寰爜鏄渶鏂扮殑锛岄偅涔?L1D flush 缂撹В鎺柦浼氳嚜鍔ㄤ繚鎶ゅ鎴锋満鍒囨崲銆?
    濡傛灉绂佺敤浜?L1D flush 缂撹В鎺柦锛屽垯褰撳涓?MDS 缂撹В鎺柦鍚敤鏃讹紝浼氭樉寮忓湴璋冪敤 MDS 缂撹В鎺柦銆?
    鏈夊叧 L1TF 涓庤櫄鎷熷寲鐨勭粏鑺傦紝鍙傝锛?    Documentation/admin-guide/hw-vuln//l1tf.rst <mitigation_control_kvm>銆?
  - CPU 涓嶅彈 L1TF 褰卞搷锛?
    褰撳涓?MDS 缂撹В鎺柦鍚敤鏃讹紝浼氬湪杩涘叆瀹㈡埛鏈轰箣鍓嶅埛鏂?CPU 缂撳啿鍖恒€?
  瀹夸富鍒板鎴锋満鍒囨崲鎵€寰楀埌鐨?MDS 淇濇姢鐭╅樀濡備笅锛?
  ============ ===== ============= ============ =================
   L1TF         MDS   VMX-L1FLUSH   Host MDS     MDS-State

   Don't care   No    Don't care    N/A          Not affected

   Yes          Yes   Disabled      Off          Vulnerable

   Yes          Yes   Disabled      Full         Mitigated

   Yes          Yes   Enabled       Don't care   Mitigated

   No           Yes   N/A           Off          Vulnerable

   No           Yes   N/A           Full         Mitigated
  ============ ===== ============= ============ =================

  杩欎粎娑电洊瀹夸富鍒板鎴锋満鐨勫垏鎹紝鍗抽槻姝粠瀹夸富娉勯湶鍒板鎴锋満锛屼絾骞朵笉鑳戒繚鎶ゅ鎴锋満鍐呴儴銆傚鎴锋満闇€瑕佹湁鍏惰嚜韬殑淇濇姢鎺柦銆?
##### XEON PHI 鐩稿叧娉ㄦ剰浜嬮」

  XEON PHI 澶勭悊鍣ㄧ郴鍒楀彈 MSBDS 褰卞搷锛屽湪杩涘叆绌洪棽鐘舵€佹椂鍙兘琚法瓒呯嚎绋嬪埄鐢ㄣ€傞儴鍒?XEON PHI 鍙樹綋鍏佽鍦ㄧ敤鎴风┖闂达紙Ring 3锛変娇鐢?MWAIT锛岃繖涓烘伓鎰忕敤鎴风┖闂存墦寮€浜嗕竴涓綔鍦ㄧ殑鏀诲嚮鍚戦噺銆傝鏆撮湶鍙互閫氳繃鍐呮牳鍛戒护琛岄€夐」 'ring3mwait=disable' 绂佺敤銆?
  XEON PHI 涓嶅彈鍏朵粬 MDS 鍙樹綋褰卞搷锛屽苟涓?MSBDS 浼氬湪 CPU 杩涘叆绌洪棽鐘舵€佷箣鍓嶅緱鍒扮紦瑙ｃ€傜敱浜?XEON PHI 涔熶笉鍙?L1TF 褰卞搷锛屽洜姝ゅ畬鍏ㄤ繚鎶ゅ苟涓嶉渶瑕佺鐢?SMT銆?
##### SMT 鎺у埗

  闄?MSBDS 澶栫殑鎵€鏈?MDS 鍙樹綋閮藉彲鑳借璺ㄨ秴绾跨▼鏀诲嚮銆傝繖鎰忓懗鐫€鍦ㄥ彈 MFBDS 鎴?MLPDS 褰卞搷鐨?CPU 涓婏紝蹇呴』绂佺敤 SMT 鎵嶈兘鑾峰緱瀹屽叏鐨勪繚鎶ゃ€傝繖浜涙槸澶у鏁板彈褰卞搷鐨?CPU锛涗緥澶栨槸 XEON PHI锛屽弬瑙?xeon_phi銆?
  绂佺敤 SMT 鍙兘浼氬甫鏉ユ樉钁楃殑鎬ц兘褰卞搷锛屼絾鍏蜂綋褰卞搷鍙栧喅浜庡伐浣滆礋杞界殑绫诲瀷銆?
  璇﹁ L1TF 缂撹В鏂囨。涓殑鐩稿叧绔犺妭锛欴ocumentation/admin-guide/hw-vuln/l1tf.rst <smt_control>銆?
### 鍐呮牳鍛戒护琛屼笂鐨勭紦瑙ｆ帶鍒?
鍐呮牳鍛戒护琛屽厑璁稿湪鍚姩鏃堕€氳繃 "mds=" 閫夐」鎺у埗 MDS 缂撹В鎺柦銆傝閫夐」鐨勬湁鏁堝弬鏁颁负锛?
  ============  =============================================================
  full		If the CPU is vulnerable, enable all available mitigations
		for the MDS vulnerability, CPU buffer clearing on exit to
		userspace and when entering a VM. Idle transitions are
		protected as well if SMT is enabled.

		It does not automatically disable SMT.

  full,nosmt	The same as mds=full, with SMT disabled on vulnerable
		CPUs.  This is the complete mitigation.

  off		Disables MDS mitigations completely.

  ============  =============================================================

鏈寚瀹氳閫夐」绛夊悓浜?"mds=full"銆傚浜庡悓鏃跺彈 TAA锛圱SX 寮傛涓锛孴SX Asynchronous Abort锛夊拰 MDS 褰卞搷鐨勫鐞嗗櫒锛屼粎鎸囧畾 "mds=off" 鑰屾病鏈夊悓鏃舵寚瀹?"tsx_async_abort=off" 灏嗕笉璧蜂綔鐢紝鍥犱负杩欎袱绉嶆紡娲炰娇鐢ㄧ殑鏄浉鍚岀殑缂撹В鎺柦銆?
### 缂撹В鎺柦閫夋嫨鎸囧崡

##### 1. 鍙椾俊浠荤殑鐢ㄦ埛绌洪棿

   濡傛灉鎵€鏈夌敤鎴风┖闂村簲鐢ㄧ▼搴忛兘鏉ヨ嚜鍙椾俊浠荤殑鏉ユ簮锛屼笖涓嶆墽琛屽閮ㄦ彁渚涚殑涓嶅彲淇′唬鐮侊紝鍒欏彲浠ョ鐢ㄧ紦瑙ｆ帾鏂姐€?
##### 2. 浣跨敤鍙椾俊浠诲鎴锋満鐨勮櫄鎷熷寲

   涓婅堪鍏充簬鍙椾俊浠荤敤鎴风┖闂寸殑鑰冮噺鍚屾牱閫傜敤銆?
##### 3. 浣跨敤涓嶅彲淇″鎴锋満鐨勮櫄鎷熷寲

   淇濇姢鍙栧喅浜?L1TF 缂撹В鎺柦鐨勭姸鎬併€傚弬瑙?virt_mechanism銆?
   濡傛灉鍚敤浜?MDS 缂撹В鎺柦骞剁鐢ㄤ簡 SMT锛屽垯鍙互闃叉瀹㈡埛鏈哄埌瀹夸富浠ュ強瀹㈡埛鏈哄埌瀹㈡埛鏈虹殑鏀诲嚮銆?
### 榛樿缂撹В鎺柦

  鍐呮牳閽堝鍙楀奖鍝嶅鐞嗗櫒鐨勯粯璁ょ紦瑙ｆ帾鏂芥槸锛?
  - 鍚敤 CPU 缂撳啿鍖烘竻闄?
  鍐呮牳榛樿涓嶅己鍒剁鐢?SMT锛岃繖瀵艰嚧杩愯涓嶅彲淇′唬鐮佹椂 SMT 绯荤粺浠嶇劧鏄撳彈鏀诲嚮銆傚叾鐞嗙敱涓?L1TF 鐩稿悓銆傚弬瑙?Documentation/admin-guide/hw-vuln//l1tf.rst <default_mitigations>銆?