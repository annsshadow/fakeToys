## 鍐呮牳椹卞姩 abituguru3


鏀寔鑺墖锛?  - Abit uGuru revision 3锛堢‖浠剁洃鎺ч儴鍒嗭紝浠呰鍙栵級

    Prefix: 'abituguru3'

    Addresses scanned: ISA 0x0E0

    Datasheet: 涓嶅彲鐢紝璇ラ┍鍔ㄥ熀浜庨€嗗悜宸ョ▼銆?
    Note:
	The uGuru 鏄竴涓甫鏉胯浇鍥轰欢鐨勫井鎺у埗鍣紝鍥轰欢灏嗗叾缂栫▼涓鸿〃鐜板緱鍍忎竴涓?	hwmon IC銆傚浐浠舵湁璁稿涓嶅悓鐨勭増鏈紝鍥犳瀹為檯涓婁篃鏈夎澶氫笉鍚岀増鏈殑 uGuru銆?	浠ヤ笅鏄悇涓绘澘浣跨敤鍝簺鐗堟湰鐨勪笉瀹屾暣鍒楄〃锛?
 - uGuru 1.00    ~ 1.24    (AI7, KV8-MAX3, AN7)
 - uGuru 2.0.0.0 ~ 2.0.4.2 (KV8-PRO)
 - uGuru 2.1.0.0 ~ 2.1.2.8 (AS8, AV8, AA8, AG8, AA8XE, AX8)
 - uGuru 2.3.0.0 ~ 2.3.0.9 (AN8)
 - uGuru 3.0.0.0 ~ 3.0.x.x (AW8, AL8, AT8, NI8 SLI, AT8 32X, AN8 32X,
	  AW9D-MAX)

	abituguru3 椹卞姩浠呴€傜敤浜?3.0.x.x 鐗堟湰鐨勪富鏉匡紝璇ラ┍鍔ㄥ湪杈冩棫鐨勪富鏉夸笂
	鏃犳硶宸ヤ綔銆傚浜庤緝鏃х殑涓绘澘锛岃浣跨敤 abituguru锛堜笉甯?3锛侊級椹卞姩銆?
Authors:
 - Hans de Goede <j.w.r.degoede@hhs.nl>,
 - 锛堢敱 Louis Kruger 瀹屾垚鍒濆閫嗗悜宸ョ▼锛?
### 妯″潡鍙傛暟


- force: bool
			寮哄埗妫€娴嬨€傛敞鎰忚鍙傛暟鍙細瀵艰嚧璺宠繃妫€娴嬶紝浠庤€?			浣?insmod 鎴愬姛銆傚鏋滄棤娉曡鍙?uGuru锛屽疄闄呯殑 hwmon
			椹卞姩灏嗕笉浼氬姞杞斤紝鍥犳涓嶄細鏈?hwmon 璁惧琚敞鍐屻€?- verbose: bool
			椹卞姩鏄惁搴斿綋杈撳嚭璇︾粏淇℃伅锛?
   - 0/off/false  姝ｅ父杈撳嚭
   - 1/on/true    + 璇︾粏閿欒鎶ュ憡锛堥粯璁わ級

			榛樿锛?锛堥┍鍔ㄤ粛澶勪簬娴嬭瘯闃舵锛?
### 鎻忚堪


璇ラ┍鍔ㄦ敮鎸佹渶杩戝甫鏈?Abit uGuru 鐨勪富鏉夸笂鎵€浣跨敤鐨勭涓変唬 Abit uGuru 鑺墖鐨勭‖浠剁洃鎺х壒鎬с€?
uGuru 鑺墖鐨勭涓夌増瀹為檯涓婃槸涓€涓?Winbond W83L951G銆傞仐鎲剧殑鏄繖骞舵棤甯姪锛屽洜涓?W83L951G
鏄竴涓繍琛岀潃瀹氬埗 Abit 搴旂敤鐨勯€氱敤寰帶鍒跺櫒銆?
灏界 Abit 娌℃湁鍙戝竷浠讳綍鍏充簬 uGuru 绗笁鐗堢殑淇℃伅锛孡ouis Kruger 杩樻槸鎴愬姛閫嗗悜宸ョ▼浜?uGuru
鐨勪紶鎰熷櫒閮ㄥ垎銆傛病鏈変粬鐨勫伐浣滐紝璇ラ┍鍔ㄥ氨涓嶅彲鑳藉疄鐜般€?
### 宸茬煡闂


Abit uGuru 鐨勭數鍘嬪拰棰戠巼鎺у埗閮ㄥ垎涓嶅彈鏀寔锛屽啓鍏ヤ换浣曚紶鎰熷櫒璁剧疆浠ュ強鍐欏叆/璇诲彇椋庢墖杞€熸帶鍒?瀵勫瓨鍣紙FanEQ锛変篃涓嶅彈鏀寔銆?
濡傛灉閬囧埌浠讳綍闂锛岃鍙戦偖浠剁粰鎴?<j.w.r.degoede@hhs.nl> 骞堕檮涓婁互涓嬪懡浠ょ殑杈撳嚭锛?`dmesg | grep abituguru`
