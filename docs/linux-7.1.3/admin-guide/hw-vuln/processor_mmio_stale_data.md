## 澶勭悊鍣?MMIO 闄堟棫鏁版嵁婕忔礊锛圥rocessor MMIO Stale Data Vulnerabilities锛?
澶勭悊鍣?MMIO 闄堟棫鏁版嵁婕忔礊锛圥rocessor MMIO Stale Data Vulnerabilities锛夋槸涓€绫诲彲鑳芥毚闇叉暟鎹殑
鍐呭瓨鏄犲皠 I/O锛圡MIO锛夋紡娲炪€傛毚闇叉暟鎹殑鎿嶄綔搴忓垪浠庣畝鍗曞埌闈炲父澶嶆潅涓嶇瓑銆傜敱浜庡ぇ澶氭暟婕忔礊閮借姹?鏀诲嚮鑰呰兘澶熻闂?MMIO锛岃澶氱幆澧冨苟涓嶄細鍙楀埌褰卞搷銆傚湪浣跨敤铏氭嫙鍖栥€佸苟鍚戜笉鍙椾俊浠荤殑 guest 鎻愪緵 MMIO
璁块棶鐨勭郴缁熺幆澧冧腑锛屽彲鑳介渶瑕佺紦瑙ｆ帾鏂姐€傝繖浜涙紡娲炲苟闈炵灛鎬佹墽琛岋紙transient execution锛夋敾鍑汇€備笉杩囷紝
杩欎簺婕忔礊鍙兘浼氭妸闄堟棫鏁版嵁浼犳挱鍒版牳蹇冨～鍏呯紦鍐插尯锛坈ore fill buffer锛変腑锛屼箣鍚庡彲鑳借鏈紦瑙ｇ殑
鐬€佹墽琛屾敾鍑绘帹鏂嚭鏉ャ€傞拡瀵硅繖浜涙紡娲炵殑缂撹В鎺柦瑙嗗钩鍙板拰鐢ㄦ硶鐨勪笉鍚岋紝鍖呭惈寰爜锛坢icrocode锛夋洿鏂?涓庤蒋浠舵敼鍔ㄧ殑缁勫悎銆傚叾涓儴鍒嗙紦瑙ｆ帾鏂戒笌鐢ㄤ簬缂撹В寰灦鏋勬暟鎹噰鏍凤紙MDS锛夋垨涓撶敤瀵勫瓨鍣ㄧ紦鍐插尯鏁版嵁閲囨牱
锛圫RBDS锛夌殑鎺柦绫讳技銆?
## 鏁版嵁浼犳挱鑰咃紙Data Propagators锛?
浼犳挱鑰咃紙Propagator锛夋槸鎸囦細鎶婇檲鏃ф暟鎹粠涓€涓井鏋舵瀯缂撳啿鍖烘垨瀵勫瓨鍣ㄥ鍒舵垨绉诲姩鍒板彟涓€涓殑鎿嶄綔銆?澶勭悊鍣?MMIO 闄堟棫鏁版嵁婕忔礊鏄寚鍙兘鎶婇檲鏃ф暟鎹洿鎺ヨ鍙栧埌鏋舵瀯鍖栫殑銆佽蒋浠跺彲瑙佺殑鐘舵€佷腑锛屾垨浠庣紦鍐插尯
鎴栧瘎瀛樺櫒涓噰鏍峰埌鐨勬搷浣溿€?
### 濉厖缂撳啿鍖洪檲鏃ф暟鎹紶鎾€咃紙FBSDP锛孎ill Buffer Stale Data Propagator锛?
鍦ㄦ煇浜涢潪涓€鑷村啓锛坣on-coherent write锛夋搷浣滀腑锛岄檲鏃ф暟鎹彲鑳戒細浠庡～鍏呯紦鍐插尯锛團B锛変紶鎾埌 uncore
鐨勯潪涓€鑷撮儴鍒嗐€傚～鍏呯紦鍐插尯浼犳挱鏈韩骞朵笉浼氳闄堟棫鏁版嵁鍦ㄦ灦鏋勪笂鍙銆傞檲鏃ф暟鎹繀椤昏浼犳挱鍒颁竴涓細琚?璇诲彇鎴栭噰鏍风殑浣嶇疆銆?
### 杈瑰甫闄堟棫鏁版嵁浼犳挱鑰咃紙SSDP锛孲ideband Stale Data Propagator锛?
杈瑰甫闄堟棫鏁版嵁浼犳挱鑰咃紙SSDP锛変粎闄愪簬瀹㈡埛绔紙鍖呮嫭 Intel Xeon E3 鏈嶅姟鍣級鐨?uncore 瀹炵幇銆傝竟甯?鍝嶅簲缂撳啿鍖虹敱鎵€鏈夊鎴风鏍稿績鍏变韩銆傚浜庡彂寰€杈瑰甫鐩爣鐨勯潪涓€鑷磋锛寀ncore 閫昏緫浼氫粠浜嬪姟缂撳啿鍖哄拰杈瑰甫
鍝嶅簲缂撳啿鍖鸿繑鍥?64 瀛楄妭鏁版嵁缁欐牳蹇冿紝鏃㈠寘鎷姹傜殑鏁版嵁锛屼篃鍖呮嫭鏈璇锋眰鐨勯檲鏃ф暟鎹€傜粨鏋滐紝鏉ヨ嚜
杈瑰甫鍝嶅簲鍜屼簨鍔＄紦鍐插尯鐨勯檲鏃ф暟鎹幇鍦ㄥ彲鑳介┗鐣欏湪鏍稿績濉厖缂撳啿鍖轰腑銆?
### 涓婚檲鏃ф暟鎹紶鎾€咃紙PSDP锛孭rimary Stale Data Propagator锛?
涓婚檲鏃ф暟鎹紶鎾€咃紙PSDP锛変粎闄愪簬瀹㈡埛绔紙鍖呮嫭 Intel Xeon E3 鏈嶅姟鍣級鐨?uncore 瀹炵幇銆備笌杈瑰甫
鍝嶅簲缂撳啿鍖虹被浼硷紝涓诲搷搴旂紦鍐插尯鐢辨墍鏈夊鎴风鏍稿績鍏变韩銆傚浜庢煇浜涘鐞嗗櫒锛孧MIO 涓昏浼氳繑鍥?64 瀛楄妭
鏁版嵁缁欐牳蹇冨～鍏呯紦鍐插尯锛屾棦鍖呮嫭璇锋眰鐨勬暟鎹紝涔熷寘鎷湭琚姹傜殑闄堟棫鏁版嵁銆傝繖涓庤竟甯﹂檲鏃ф暟鎹紶鎾€?绫讳技銆?
## 婕忔礊锛圴ulnerabilities锛?
### 璁惧瀵勫瓨鍣ㄩ儴鍒嗗啓锛圖RPW锛孌evice Register Partial Write锛夛紙CVE-2022-21166锛?
鏌愪簺绔偣 MMIO 瀵勫瓨鍣ㄥ灏忎簬瀵勫瓨鍣ㄥぇ灏忕殑鍐欏鐞嗕笉褰撱€傚畠涓嶄細涓鍐欐搷浣滐紝涔熶笉浼氬彧澶嶅埗姝ｇ‘鐨?瀛楄妭瀛愰泦锛堜緥濡傦紝2 瀛楄妭鍐欏氨鍙鍒?2 瀛楄妭锛夛紝鑰屾槸鍙兘鍐欏叆姣斿啓浜嬪姟鎵€鎸囧畾鐨勬洿澶氬瓧鑺傚埌瀵勫瓨鍣ㄤ腑銆?鍦ㄥ彈 FBSDP 褰卞搷鐨勫鐞嗗櫒涓婏紝杩欏彲鑳戒細鏆撮湶鍑哄垱寤鸿鍐欎簨鍔＄殑閭ｄ釜鏍稿績鐨勫～鍏呯紦鍐插尯涓殑闄堟棫鏁版嵁銆?
### 鍏变韩缂撳啿鍖烘暟鎹噰鏍凤紙SBDS锛孲hared Buffers Data Sampling锛夛紙CVE-2022-21125锛?
鍦ㄤ紶鎾€呭彲鑳藉凡缁忔妸鏁版嵁鍦?uncore 涓惉绉汇€佸苟鎶婇檲鏃ф暟鎹鍒跺埌瀹㈡埛绔牳蹇冨～鍏呯紦鍐插尯涔嬪悗锛屽彈 MFBDS
褰卞搷鐨勫鐞嗗櫒鍙互浠庡～鍏呯紦鍐插尯娉勬紡鏁版嵁銆傝婕忔礊浠呴檺浜庡鎴风锛堝寘鎷?Intel Xeon E3 鏈嶅姟鍣級鐨?uncore 瀹炵幇銆?
### 鍏变韩缂撳啿鍖烘暟鎹锛圫BDR锛孲hared Buffers Data Read锛夛紙CVE-2022-21123锛?
瀹冧笌鍏变韩缂撳啿鍖烘暟鎹噰鏍凤紙SBDS锛夌被浼硷紝鍖哄埆鍦ㄤ簬鏁版嵁鏄洿鎺ヤ粠鏋舵瀯涓婅蒋浠跺彲瑙佺殑鐘舵€佷腑璇诲彇鐨勩€傝
婕忔礊浠呴檺浜庡鎴风锛堝寘鎷?Intel Xeon E3 鏈嶅姟鍣級鐨?uncore 瀹炵幇銆?
## 鍙楀奖鍝嶇殑澶勭悊鍣紙Affected Processors锛?
骞堕潪鎵€鏈?CPU 閮戒細鍙楀埌鎵€鏈夊彉浣撶殑褰卞搷銆備緥濡傦紝澶у鏁伴潰鍚戞湇鍔″櫒甯傚満鐨勫鐞嗗櫒锛堜笉鍖呮嫭 Intel Xeon
E3 澶勭悊鍣級鍙彈璁惧瀵勫瓨鍣ㄩ儴鍒嗗啓锛圖RPW锛夊奖鍝嶃€?
浠ヤ笅鏄彈褰卞搷鐨?Intel 澶勭悊鍣ㄥ垪琛?[#f1]_锛?
   ===================  ============  =========
   Common name          Family_Model  Steppings
   ===================  ============  =========
   HASWELL_X            06_3FH        2,4
   SKYLAKE_L            06_4EH        3
   BROADWELL_X          06_4FH        All
   SKYLAKE_X            06_55H        3,4,6,7,11
   BROADWELL_D          06_56H        3,4,5
   SKYLAKE              06_5EH        3
   ICELAKE_X            06_6AH        4,5,6
   ICELAKE_D            06_6CH        1
   ICELAKE_L            06_7EH        5
   ATOM_TREMONT_D       06_86H        All
   LAKEFIELD            06_8AH        1
   KABYLAKE_L           06_8EH        9 to 12
   ATOM_TREMONT         06_96H        1
   ATOM_TREMONT_L       06_9CH        0
   KABYLAKE             06_9EH        9 to 13
   COMETLAKE            06_A5H        2,3,5
   COMETLAKE_L          06_A6H        0,1
   ROCKETLAKE           06_A7H        1
   ===================  ============  =========

濡傛灉鏌愪釜 CPU 鍦ㄥ彈褰卞搷澶勭悊鍣ㄥ垪琛ㄤ腑锛屼絾娌℃湁鍙楀埌鏌愪釜鍙樹綋鐨勫奖鍝嶏紝鍒欓€氳繃 MSR IA32_ARCH_CAPABILITIES
涓殑鏂颁綅鏉ヨ〃绀恒€傚鍚庨潰灏忚妭鎵€杩帮紝瀵逛簬鎵€鏈夊彉浣擄紝缂撹В鎺柦澶т綋鐩稿悓锛屽嵆閫氳繃 VERW 鎸囦护鏉ユ竻绌?CPU
濉厖缂撳啿鍖恒€?
## MSR 涓殑鏂颁綅锛圢ew bits in MSRs锛?
杈冩柊鐨勫鐞嗗櫒浠ュ強瀵圭幇鏈夊彈褰卞搷澶勭悊鍣ㄨ繘琛岀殑寰爜鏇存柊锛屽悜 IA32_ARCH_CAPABILITIES MSR 娣诲姞浜嗘柊鐨勪綅銆?杩欎簺浣嶅彲鐢ㄤ簬鏋氫妇澶勭悊鍣?MMIO 闄堟棫鏁版嵁婕忔礊鐨勭壒瀹氬彉浣擄紝浠ュ強缂撹В鑳藉姏銆?
### MSR IA32_ARCH_CAPABILITIES

Bit 13 - SBDR_SSDP_NO - 缃綅鏃讹紝澶勭悊鍣ㄤ笉鍙楀叡浜紦鍐插尯鏁版嵁璇伙紙SBDR锛夋紡娲烇紝涔熶笉鍙楄竟甯﹂檲鏃?	鏁版嵁浼犳挱鑰咃紙SSDP锛夌殑褰卞搷銆?Bit 14 - FBSDP_NO - 缃綅鏃讹紝澶勭悊鍣ㄤ笉鍙楀～鍏呯紦鍐插尯闄堟棫鏁版嵁浼犳挱鑰咃紙FBSDP锛夌殑褰卞搷銆?Bit 15 - PSDP_NO - 缃綅鏃讹紝澶勭悊鍣ㄤ笉鍙椾富闄堟棫鏁版嵁浼犳挱鑰咃紙PSDP锛夌殑褰卞搷銆?Bit 17 - FB_CLEAR - 缃綅鏃讹紝VERW 鎸囦护灏嗕綔涓?MD_CLEAR 鎿嶄綔鐨勪竴閮ㄥ垎瑕嗙洊 CPU 濉厖缂撳啿鍖虹殑
	鍊笺€傛湭鏋氫妇 MDS_NO锛堝嵆鍙?MDS 褰卞搷锛変絾鍚屾椂鏋氫妇浜嗗 L1D_FLUSH 鍜?MD_CLEAR 鏀寔鐨勫鐞嗗櫒锛?	浼氶殣寮忓湴鎶?FB_CLEAR 浣滀负鍏?MD_CLEAR 鏀寔鐨勪竴閮ㄥ垎杩涜鏋氫妇銆?Bit 18 - FB_CLEAR_CTRL - 澶勭悊鍣ㄦ敮鎸佸 MSR IA32_MCU_OPT_CTRL[FB_CLEAR_DIS] 鐨勮鍐欍€傚湪姝ょ被
	澶勭悊鍣ㄤ笂锛屽彲浠ヨ缃?FB_CLEAR_DIS 浣嶏紝浣?VERW 鎸囦护涓嶆墽琛?FB_CLEAR 鍔ㄤ綔銆傚苟闈炴墍鏈夋敮鎸?	FB_CLEAR 鐨勫鐞嗗櫒閮芥敮鎸?FB_CLEAR_CTRL銆?
### MSR IA32_MCU_OPT_CTRL

Bit 3 - FB_CLEAR_DIS - 缃綅鏃讹紝VERW 鎸囦护涓嶆墽琛?FB_CLEAR 鍔ㄤ綔銆傚湪绯荤粺杞欢璁や负鏈夊繀瑕佹椂锛堜緥濡傦紝
褰撴€ц兘鏇翠负鍏抽敭锛屾垨涓嶅彈淇′换鐨勮蒋浠舵病鏈?MMIO 璁块棶鏉冮檺鏃讹級锛岃繖鍙敤浜庨檷浣?FB_CLEAR 甯︽潵鐨勬€ц兘
褰卞搷銆傛敞鎰忥紝FB_CLEAR_DIS 瀵规灇涓炬病鏈夊奖鍝嶏紙渚嬪锛屽畠涓嶄細鏀瑰彉 FB_CLEAR 鎴?MD_CLEAR 鐨勬灇涓撅級锛?骞朵笖瀹冨彲鑳戒笉琚墍鏈夋灇涓句簡 FB_CLEAR 鐨勫鐞嗗櫒鎵€鏀寔銆?
## 缂撹В鎺柦锛圡itigation锛?
涓?MDS 绫讳技锛屽鐞嗗櫒 MMIO 闄堟棫鏁版嵁婕忔礊鐨勬墍鏈夊彉浣撻兘閲囩敤鐩稿悓鐨勭紦瑙ｇ瓥鐣ワ細鍦ㄦ敾鍑昏€呰兘澶熸彁鍙栨満瀵?涔嬪墠锛屽己鍒?CPU 娓呯┖鍙楀奖鍝嶇殑缂撳啿鍖恒€?
杩欐槸閫氳繃缁撳悎浣跨敤鍘熸湰鏈娇鐢ㄤ笖宸插簾寮冪殑 VERW 鎸囦护涓庡井鐮佹洿鏂版潵瀹炵幇鐨勩€傚綋鎵ц VERW 鎸囦护鏃讹紝
寰爜浼氭竻绌哄彈褰卞搷鐨?CPU 缂撳啿鍖恒€?
鍐呮牳閫氳繃 x86_clear_cpu_buffers() 鎵ц缂撳啿鍖烘竻绌恒€?
鍦ㄥ彈 MDS 褰卞搷鐨勫鐞嗗櫒涓婏紝鍐呮牳宸茬粡鍦ㄥ唴鏍?鐢ㄦ埛绌洪棿銆佽櫄鎷熸満鐩戞帶鍣?guest 浠ュ強 C-state锛堢┖闂诧級
鍒囨崲鏃惰皟鐢ㄤ簡 CPU 缂撳啿鍖烘竻绌恒€傝繖绫诲鐞嗗櫒涓婃棤闇€棰濆鐨勭紦瑙ｆ帾鏂姐€?
瀵逛簬涓嶅彈 MDS 鎴?TAA 褰卞搷鐨勫鐞嗗櫒锛屽彧鏈夊湪鍏锋湁 MMIO 鑳藉姏鐨勬敾鍑昏€呮儏鍐典笅鎵嶉渶瑕佺紦瑙ｃ€傚洜姝わ紝
鍐呮牳/鐢ㄦ埛绌洪棿涓嶉渶瑕?VERW銆傚浜庤櫄鎷熷寲鍦烘櫙锛孷ERW 浠呴渶鍦ㄨ繘鍏ュ叿鏈?MMIO 鑳藉姏鐨?guest 鏃讹紙VMENTER锛?鎵ц銆?
### 缂撹В鐐癸紙Mitigation points锛?
##### 杩斿洖鐢ㄦ埛绌洪棿锛圧eturn to user space锛?
鍦ㄥ彈 MDS/TAA 褰卞搷鏃讹紝缂撹В鎺柦涓?MDS 鐩稿悓锛涘惁鍒欎笉闇€瑕佺紦瑙ｃ€?
##### C-State 鍒囨崲锛圕-State transition锛?
CPU 鍦?C-state 鍒囨崲鏈熼棿鐨勬帶鍒跺瘎瀛樺櫒鍐欐搷浣滃彲鑳芥妸鏁版嵁浠庡～鍏呯紦鍐插尯浼犳挱鍒?uncore 缂撳啿鍖恒€傚湪
C-state 鍒囨崲涔嬪墠鎵ц VERW锛屼互娓呯┖ CPU 濉厖缂撳啿鍖恒€?
##### Guest 杩涘叆鐐癸紙Guest entry point锛?
鍦ㄥ鐞嗗櫒鍚屾椂涔熷彈 MDS/TAA 褰卞搷鏃讹紝缂撹В鎺柦涓?MDS 鐩稿悓锛涘惁鍒欙紝浠呭鍏锋湁 MMIO 鑳藉姏鐨?guest 鍦?VMENTER 鏃舵墽琛?VERW銆傚湪涓嶈 MDS/TAA 褰卞搷鐨勫鐞嗗櫒涓婏紝娌℃湁 MMIO 璁块棶鑳藉姏鐨?guest 鏃犳硶鍒╃敤
澶勭悊鍣?MMIO 闄堟棫鏁版嵁婕忔礊鎻愬彇鏈哄瘑锛屽洜姝ゅ姝ょ被 guest 娌℃湁蹇呰鎵ц VERW銆?
### 鍐呮牳鍛戒护琛屼笂鐨勭紦瑙ｆ帶鍒讹紙Mitigation control on the kernel command line锛?
鍐呮牳鍛戒护琛屽厑璁稿湪鍚姩鏃堕€氳繃 "mmio_stale_data=" 閫夐」鎺у埗澶勭悊鍣?MMIO 闄堟棫鏁版嵁婕忔礊鐨勭紦瑙ｃ€?璇ラ€夐」鐨勬湁鏁堝弬鏁颁负锛?
  ==========  =================================================================
  full        If the CPU is vulnerable, enable mitigation; CPU buffer clearing
              on exit to userspace and when entering a VM. Idle transitions are
              protected as well. It does not automatically disable SMT.
  full,nosmt  Same as full, with SMT disabled on vulnerable CPUs. This is the
              complete mitigation.
  off         Disables mitigation completely.
  ==========  =================================================================

濡傛灉 CPU 鍙楀奖鍝嶏紝涓斿唴鏍稿懡浠よ娌℃湁鎻愪緵 mmio_stale_data=off锛岄偅涔堝唴鏍镐細閫夋嫨閫傚綋鐨勭紦瑙ｆ帾鏂姐€?
### 缂撹В鐘舵€佷俊鎭紙Mitigation status information锛?
Linux 鍐呮牳鎻愪緵浜嗕竴涓?sysfs 鎺ュ彛锛岀敤浜庢灇涓剧郴缁熷綋鍓嶇殑婕忔礊鐘舵€侊細绯荤粺鏄惁鏄撳彈鏀诲嚮锛屼互鍙婂摢浜?缂撹В鎺柦澶勪簬婵€娲荤姸鎬併€傜浉鍏崇殑 sysfs 鏂囦欢鏄細

	/sys/devices/system/cpu/vulnerabilities/mmio_stale_data

璇ユ枃浠跺彲鑳界殑鍙栧€间负锛?
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
     * - 'Unknown: No mitigations'
       - The processor vulnerability status is unknown because it is
	 out of Servicing period. Mitigation is not attempted.

```
### 瀹氫箟锛圖efinitions锛夛細

Servicing period锛堟湇鍔℃湡锛夛細鍒╃敤 Intel 骞冲彴鏇存柊锛圛PU锛夋祦绋嬫垨鍏跺畠绫讳技鏈哄埗锛屽悜 Intel 澶勭悊鍣ㄦ垨
骞冲彴鎻愪緵鍔熻兘鍜屽畨鍏ㄦ洿鏂扮殑杩囩▼銆?
End of Servicing Updates锛圗SU锛屾湇鍔℃洿鏂扮粓姝級锛欵SU 鏄?Intel 涓嶅啀鎻愪緵鏈嶅姟锛堜緥濡傞€氳繃 IPU 鎴栧叾瀹?绫讳技鏇存柊娴佺▼锛夌殑鏃ユ湡銆侲SU 鏃ユ湡閫氬父浼氫笌瀛ｅ害鏈榻愩€?
濡傛灉澶勭悊鍣ㄦ槗鍙楁敾鍑伙紝鍒欎細鍦ㄤ笂杩颁俊鎭箣鍚庨檮鍔犱互涓嬩俊鎭細

  ========================  ===========================================
  'SMT vulnerable'          SMT is enabled
  'SMT disabled'            SMT is disabled
  'SMT Host state unknown'  Kernel runs in a VM, Host SMT state unknown
  ========================  ===========================================

### 鍙傝€冭祫鏂欙紙References锛?
   https://www.intel.com/content/www/us/en/developer/topic-technology/software-security-guidance/processors-affected-consolidated-product-cpu-model.html
