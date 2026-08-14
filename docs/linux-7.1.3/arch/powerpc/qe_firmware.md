## Freescale QUICC Engine 鍥轰欢涓婁紶


(c) 2007 Timur Tabi <timur at freescale.com>,
    Freescale Semiconductor


   I - 鍥轰欢鐨勮蒋浠惰鍙瘉

   II - 寰爜鍙敤鎬?
   III - 鎻忚堪涓庢湳璇?
   IV - 寰爜缂栫▼缁嗚妭

   V - 鍥轰欢缁撴瀯甯冨眬

   VI - 鐢ㄤ簬鍒涘缓鍥轰欢鏂囦欢鐨勭ず渚嬩唬鐮?
## 淇淇℃伅


2007骞?1鏈?0鏃ワ細Rev 1.0 - 鍒濆鐗堟湰

## 涓€銆佸浐浠剁殑杞欢璁稿彲璇?

姣忎釜鍥轰欢鏂囦欢閮藉甫鏈夊叾鑷繁鐨勮蒋浠惰鍙瘉銆傛湁鍏崇壒瀹氳鍙瘉鐨勪俊鎭紝璇锋煡鐪嬮殢鍥轰欢鍒嗗彂鐨勮鍙瘉鏂囨湰銆?
## 浜屻€佸井鐮佸彲鐢ㄦ€?

鍥轰欢鏂囦欢閫氳繃澶氱娓犻亾鍒嗗彂銆傞儴鍒嗗彲鍦?http://opensource.freescale.com 鑾峰彇銆傚叾浠栧浐浠舵枃浠惰鑱旂郴浣犵殑 Freescale 浠ｈ〃鎴栨搷浣滅郴缁熶緵搴斿晢銆?
## 涓夈€佹弿杩颁笌鏈


鍦ㄦ湰鏂囨。涓紝鏈鈥渕icrocode锛堝井鐮侊級鈥濇寚鐨勬槸鏋勬垚瀹為檯 QE 寰爜鐨勪竴涓?32 浣嶆暣鏁般€?
鏈鈥渇irmware锛堝浐浠讹級鈥濇寚鐨勬槸涓€涓簩杩涘埗 blob锛屽畠鍖呭惈寰爜浠ュ強鍏朵粬鏁版嵁锛岃繖浜涙暟鎹細

 1) 鎻忚堪寰爜鐨勭洰鐨? 2) 鎻忚堪濡備綍浠ュ強鍦ㄥ摢閲屼笂浼犲井鐮? 3) 鎸囧畾鍚勭瀵勫瓨鍣ㄧ殑鍊? 4) 鍖呭惈渚涚壒瀹氳澶囬┍鍔ㄤ娇鐢ㄧ殑棰濆鏁版嵁

鍥轰欢鏂囦欢鏄彧鍖呭惈涓€浠藉浐浠剁殑浜岃繘鍒舵枃浠躲€?
## 鍥涖€佸井鐮佺紪绋嬬粏鑺?

QE 鏋舵瀯鍏佽姣忎釜 RISC 澶勭悊鍣ㄥ湪 I-RAM 涓彧瀛樺湪涓€浠藉井鐮併€傝鏇挎崲浠讳綍褰撳墠鐨勫井鐮侊紝蹇呴』鍏堟墽琛屼竴娆″畬鏁寸殑 QE 澶嶄綅锛堝畠浼氱鐢ㄥ井鐮侊級銆?
QE 寰爜鎸変互涓嬫楠や笂浼狅細

1) 浣跨敤 IRAM.IADD 鍜?IRAM.IDATA 瀵勫瓨鍣紝灏嗗井鐮佹斁缃埌 I-RAM 涓殑鐗瑰畾浣嶇疆銆?
2) 鏍规嵁鍥轰欢鏄惁闇€瑕佸垎绂诲紡 I-RAM锛屽皢 CERCR.CIR 浣嶈涓?0 鎴?1銆傚垎绂诲紡 I-RAM 浠呭鎷ユ湁澶?RISC 澶勭悊鍣?QE 鐨?SOC锛堝 8360锛夋湁鎰忎箟銆傚垎绂?I-RAM 鍏佽姣忎釜澶勭悊鍣ㄨ繍琛屼笉鍚岀殑寰爜锛屼粠鑰屾湁鏁堝湴鏋勬垚涓€涓潪瀵圭О澶氬鐞嗭紙AMP锛夌郴缁熴€?
3) 灏?TIBCR 闄烽槺瀵勫瓨鍣ㄥ姞杞戒负寰爜涓櫡闃卞鐞嗙▼搴忕殑鍦板潃銆?
4) 灏?RSP.ECCR 瀵勫瓨鍣ㄧ紪绋嬩负缁欏畾鐨勫€笺€?
5) 濡傛湁蹇呰锛岄渶瑕佽櫄鎷熼櫡闃卞拰鎵╁睍妯″紡鏁版嵁鐨勮澶囬┍鍔ㄤ細浣跨敤瀹冧滑銆?
铏氭嫙寰爜闄烽槺

杩欎簺铏氭嫙闄烽槺鏄井鐮佷腑鐨勬潯浠跺垎鏀€傝繖浜涙槸 ROM 鐮佷腑寮曞叆鐨勨€滆蒋鈥濅复鏃舵満鍒讹紝鐢ㄤ互鎻愪緵鏇撮珮鐨勭伒娲绘€у苟鑺傜渷纭欢闄烽槺銆傚鏋滄縺娲讳簡鏂扮壒鎬э紝鎴栧湪 RAM 鍖呬腑淇浜嗘煇涓棶棰橈紝搴斿綋婵€娲诲畠浠€傝鏁版嵁缁撴瀯鍚戝井鐮佸彂鍑轰俊鍙凤紝鍛婄煡鍝簺铏氭嫙闄烽槺澶勪簬婵€娲荤姸鎬併€?
璇ョ粨鏋勫寘鍚?6 涓瓧锛屽簲鐢ㄧ▼搴忓簲褰撳皢鍏跺鍒跺埌鏌愬

```

	---------------------------------------------------------------
	| Offset in |                  | Destination Offset | Size of |
	|   array   |     Protocol     |   within PRAM      | Operand |
	--------------------------------------------------------------|
	|     0     | Ethernet         |      0xF8          | 4 bytes |
	|           | interworking     |                    |         |
	---------------------------------------------------------------
	|     4     | ATM              |      0xF8          | 4 bytes |
	|           | interworking     |                    |         |
	---------------------------------------------------------------
	|     8     | PPP              |      0xF8          | 4 bytes |
	|           | interworking     |                    |         |
	---------------------------------------------------------------
	|     12    | Ethernet RX      |      0x22          | 1 byte  |
	|           | Distributor Page |                    |         |
	---------------------------------------------------------------
	|     16    | ATM Globtal      |      0x28          | 1 byte  |
	|           | Params Table     |                    |         |
	---------------------------------------------------------------
	|     20    | Insert Frame     |      0xF8          | 4 bytes |
	---------------------------------------------------------------


```
鎵╁睍妯″紡

杩欐槸涓€涓弻瀛椾綅鏁扮粍锛?4 浣嶏級锛屽畾涔変簡瀵硅蒋浠堕┍鍔ㄦ湁鐗规畩褰卞搷鐨勫姛鑳姐€傛瘡涓€浣嶉兘鏈夎嚜韬殑褰卞搷锛屽苟甯︽湁涓庝箣鐩稿叧鐨勮蒋浠朵笓鐢ㄦ寚浠ゃ€傝缁撴瀯涓?
```

	-----------------------------------------------------------------------
	| Bit #  |     Name     |   Description                               |
	-----------------------------------------------------------------------
	|   0    | General      | Indicates that prior to each host command   |
	|        | push command | given by the application, the software must |
	|        |              | assert a special host command (push command)|
	|        |              | CECDR = 0x00800000.                         |
	|        |              | CECR = 0x01c1000f.                          |
	-----------------------------------------------------------------------
	|   1    | UCC ATM      | Indicates that after issuing ATM RX INIT    |
	|        | RX INIT      | command, the host must issue another special|
	|        | push command | command (push command) and immediately      |
	|        |              | following that re-issue the ATM RX INIT     |
	|        |              | command. (This makes the sequence of        |
	|        |              | initializing the ATM receiver a sequence of |
	|        |              | three host commands)                        |
	|        |              | CECDR = 0x00800000.                         |
	|        |              | CECR = 0x01c1000f.                          |
	-----------------------------------------------------------------------
	|   2    | Add/remove   | Indicates that following the specific host  |
	|        | command      | command: "Add/Remove entry in Hash Lookup   |
	|        | validation   | Table" used in Interworking setup, the user |
	|        |              | must issue another command.                 |
	|        |              | CECDR = 0xce000003.                         |
	|        |              | CECR = 0x01c10f58.                          |
	-----------------------------------------------------------------------
	|   3    | General push | Indicates that the s/w has to initialize    |
	|        | command      | some pointers in the Ethernet thread pages  |
	|        |              | which are used when Header Compression is   |
	|        |              | activated.  The full details of these       |
	|        |              | pointers is located in the software drivers.|
	-----------------------------------------------------------------------
	|   4    | General push | Indicates that after issuing Ethernet TX    |
	|        | command      | INIT command, user must issue this command  |
	|        |              | for each SNUM of Ethernet TX thread.        |
	|        |              | CECDR = 0x00800003.                         |
	|        |              | CECR = 0x7'b{0}, 8'b{Enet TX thread SNUM},  |
	|        |              |        1'b{1}, 12'b{0}, 4'b{1}              |
	-----------------------------------------------------------------------
	| 5 - 31 |     N/A      | Reserved, set to zero.                      |
	-----------------------------------------------------------------------

```
## 浜斻€佸浐浠剁粨鏋勫竷灞€


鏉ヨ嚜 Freescale 鐨?QE 寰爜閫氬父浣滀负澶存枃浠舵彁渚涖€傝澶存枃浠跺寘鍚畾涔夊井鐮佷簩杩涘埗鏈韩浠ュ強鐢ㄤ簬涓婁紶璇ュ井鐮佺殑鍏朵粬鏁版嵁鐨勫畯銆傝繖浜涙枃浠剁殑鏍煎紡涓嶅埄浜庣畝鍗曞湴鍖呭惈鍒板叾浠栦唬鐮佷腑銆傚洜姝わ紝闇€瑕佷竴绉嶆洿鍏峰彲绉绘鎬х殑鏍煎紡銆傛湰鑺傚畾涔夎鏍煎紡銆?
鍒嗗彂鏃朵笉鍐嶄娇鐢ㄥご鏂囦欢锛岃€屾槸灏嗗井鐮佸強鐩稿叧鏁版嵁宓屽叆鍒颁竴涓簩杩涘埗 blob 涓€傝 blob 琚紶缁?qe_upload_firmware() 鍑芥暟锛屽畠瑙ｆ瀽璇?blob 骞舵墽琛屼笂浼犲井鐮佹墍闇€鐨勫叏閮ㄦ搷浣溿€?
鎵€鏈夋暣鏁板潎涓哄ぇ绔簭銆傛湁鍏虫渶鏂板疄鐜颁俊鎭紝璇锋煡鐪?qe_upload_firmware() 鍑芥暟鐨勬敞閲娿€?
璇ョ粨鏋勬敮鎸佺増鏈寲锛岀粨鏋勭殑鐗堟湰宓屽叆鍦ㄧ粨鏋勮嚜韬箣涓€備负纭繚鍓嶅悜涓庡悗鍚戝吋瀹癸紝鎵€鏈夌増鏈殑缁撴瀯閮藉繀椤诲湪寮€澶翠娇鐢ㄧ浉鍚岀殑 'qe_header' 缁撴瀯銆?
'header'锛堢被鍨嬶細struct qe_header锛夛細
	'length' 瀛楁鏄暣浠界粨鏋勭殑澶у皬锛堜互瀛楄妭璁★級锛屽寘鍚叾涓祵鍏ョ殑鎵€鏈夊井鐮佷互鍙?CRC锛堝鏋滃瓨鍦級銆?
	'magic' 瀛楁鏄竴涓敱涓変釜瀛楄妭缁勬垚鐨勬暟缁勶紝鍖呭惈瀛楁瘝 'Q'銆?E' 鍜?'F'銆傝繖鏄竴涓爣璇嗙锛岃〃鏄庤缁撴瀯鏄竴涓?QE 鍥轰欢缁撴瀯銆?
	'version' 瀛楁鏄竴涓崟瀛楄妭锛岃〃鏄庤缁撴瀯鐨勭増鏈€傚鏋滅粨鏋勭殑甯冨眬闇€瑕佹洿鏀逛互娣诲姞瀵瑰叾浠栫被鍨嬪井鐮佺殑鏀寔锛屽垯鐗堟湰鍙蜂篃搴旂浉搴旀洿鏀广€?
'id' 瀛楁鏄竴涓互绌哄瓧绗︾粨灏剧殑瀛楃涓诧紙閫傚悎鎵撳嵃锛夛紝鐢ㄤ簬鏍囪瘑鍥轰欢銆?
'count' 瀛楁琛ㄧず 'microcode' 缁撴瀯鐨勬暟閲忋€傛瘡涓?RISC 澶勭悊鍣ㄥ繀椤绘湁涓斾粎鏈変竴涓?'microcode' 缁撴瀯銆傚洜姝わ紝璇ュ瓧娈典篃琛ㄧず姝?SOC 鐨?RISC 澶勭悊鍣ㄦ暟閲忋€?
'soc' 缁撴瀯鍖呭惈鐢ㄤ簬灏嗗井鐮佷笌 SOC 鏈韩鍖归厤鐨?SOC 缂栧彿鍜屼慨璁㈠彿銆傞€氬父锛屽井鐮佸姞杞界▼搴忓簲褰撳皢璇ョ粨鏋勪腑鐨勬暟鎹笌 SOC 缂栧彿鍜屼慨璁㈠彿杩涜鏍稿锛屼粎褰撳尮閰嶆椂鎵嶄笂浼犲井鐮併€備笉杩囷紝骞堕潪鎵€鏈夊钩鍙伴兘浼氬仛姝ゆ鏌ャ€?
灏界涓嶆帹鑽愶紝浣嗕綘鍙互鍦?soc.model 瀛楁涓寚瀹?'0' 浠ュ畬鍏ㄨ烦杩?SOC 鍖归厤銆?
'model' 瀛楁鏄竴涓?16 浣嶆暟瀛楋紝涓庡疄闄?SOC 鍖归厤銆?major' 鍜?'minor' 瀛楁鍒嗗埆鏄?SOC 鐨勪富淇鍙峰拰娆′慨璁㈠彿銆?
```

     soc.model = 8323
     soc.major = 1
     soc.minor = 0

```
'padding'锛堝～鍏咃級鏄负缁撴瀯瀵归綈鎵€蹇呴渶鐨勩€傝瀛楁纭繚 'extended_modes' 瀛楁鍦?64 浣嶈竟鐣屼笂瀵归綈銆?
'extended_modes' 鏄竴涓綅鍩燂紝瀹氫箟浜嗗璁惧椹卞姩鏈夊奖鍝嶇殑鍔熻兘銆傛瘡涓€浣嶉兘鏈夎嚜韬殑褰卞搷骞跺甫鏈変笌涔嬬浉鍏崇殑椹卞姩涓撶敤鎸囦护銆傝瀛楁瀛樺偍鍦?QE 搴撲腑锛屽彲渚涗换浣曡皟鐢?qe_get_firmware_info() 鐨勯┍鍔ㄤ娇鐢ㄣ€?
'vtraps' 鏄竴涓寘鍚?8 涓瓧鐨勬暟缁勶紝瀛樻斁姣忎釜铏氭嫙闄烽槺鐨勮櫄鎷熼櫡闃卞€笺€備笌 'extended_modes' 鐩稿悓锛岃瀛楁瀛樺偍鍦?QE 搴撲腑锛屽彲渚涗换浣曡皟鐢?qe_get_firmware_info() 鐨勯┍鍔ㄤ娇鐢ㄣ€?
'microcode'锛堢被鍨嬶細struct qe_microcode锛夛細
	姣忎釜 RISC 澶勭悊鍣ㄥ搴斾竴涓?'microcode' 缁撴瀯銆傜涓€涓?'microcode' 缁撴瀯瀵瑰簲绗竴涓?RISC锛屼緷姝ょ被鎺ㄣ€?
	'id' 瀛楁鏄竴涓€傚悎鎵撳嵃鐨勩€佷互绌哄瓧绗︾粨灏剧殑瀛楃涓诧紝鐢ㄤ簬鏍囪瘑姝ょ壒瀹氬井鐮併€?
	'traps' 鏄竴涓寘鍚?16 涓瓧鐨勬暟缁勶紝瀛樻斁 16 涓櫡闃卞悇鑷殑纭欢闄烽槺鍊笺€傚鏋?trap[i] 涓?0锛屽垯蹇界暐姝ょ壒瀹氶櫡闃憋紙鍗充笉鍐欏叆 TIBCR[i]锛夈€傛暣涓€兼寜鍘熸牱鍐欏叆 TIBCR[i] 瀵勫瓨鍣紝鍥犳濡傛湁蹇呰锛岃纭繚璁剧疆 EN 鍜?T_IBP 浣嶃€?
	'eccr' 鏄缂栫▼鍒?ECCR 瀵勫瓨鍣ㄤ腑鐨勫€笺€?
	'iram_offset' 鏄紑濮嬪啓鍏ュ井鐮佹椂鐩稿浜?IRAM 鐨勫亸绉汇€?
	'count' 鏄井鐮佷腑 32 浣嶅瓧鐨勬暟閲忋€?
	'code_offset' 鏄粠鏈粨鏋勫紑澶村埌寰爜鏈韩鎵€鍦ㄤ綅缃殑鍋忕Щ锛堜互瀛楄妭璁★級銆傜涓€涓井鐮佷簩杩涘埗搴旂揣鎺ュ湪 'microcode' 鏁扮粍涔嬪悗銆?
	'major'銆?minor' 鍜?'revision' 鍒嗗埆鏄井鐮佺殑涓荤増鏈彿銆佹鐗堟湰鍙峰拰淇鐗堟湰鍙枫€傚鏋滄墍鏈夊€奸兘涓?0锛屽垯蹇界暐杩欎簺瀛楁銆?
	'reserved' 鏄负缁撴瀯瀵归綈鎵€蹇呴渶鐨勩€傜敱浜?'microcode' 鏄竴涓暟缁勶紝64 浣嶇殑 'extended_modes' 瀛楁闇€瑕佸湪 64 浣嶈竟鐣屼笂瀵归綈锛岃€岃繖鍙湁鍦?'microcode' 鐨勫ぇ灏忎负 8 瀛楄妭鐨勬暣鏁板€嶆椂鎵嶈兘瀹炵幇銆備负纭繚杩欎竴鐐癸紝鎴戜滑鍔犲叆 'reserved'銆?
鏈€鍚庝竴浠藉井鐮佷箣鍚庢槸涓€涓?32 浣?CRC銆傚畠鍙互浣跨敤

```

  u32 crc32(const u8 *p, unsigned int len)
  {
	unsigned int i;
	u32 crc = 0;

	while (len--) {
	   crc ^= *p++;
	   for (i = 0; i < 8; i++)
		   crc = (crc >> 1) ^ ((crc & 1) ? 0xedb88320 : 0);
	}
	return crc;
  }

```
## 鍏€佺敤浜庡垱寤哄浐浠舵枃浠剁殑绀轰緥浠ｇ爜


涓€涓粠 Freescale 閫氬父鍒嗗彂鐨勫ご鏂囦欢鍒涘缓鍥轰欢浜岃繘鍒剁殑 Python 绋嬪簭鍙湪 http://opensource.freescale.com 鎵惧埌銆?