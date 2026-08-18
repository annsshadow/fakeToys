
## NAPI


NAPI 鏄?Linux 缃戠粶鏍堜娇鐢ㄧ殑浜嬩欢澶勭悊鏈哄埗銆侼API 杩欎釜鍚嶅瓧涓嶅啀浠ｈ〃浠讳綍鐗瑰畾鐨勫惈涔?[#]_銆?

鍦ㄥ熀鏈搷浣滀腑锛岃澶囬€氳繃涓柇灏嗘柊浜嬩欢閫氱煡涓绘満銆傜劧鍚庝富鏈鸿皟搴︿竴涓?NAPI 瀹炰緥鏉ュ鐞嗚繖浜?
浜嬩欢銆備篃鍙互鍦ㄤ笉鍏堟敹鍒颁腑鏂殑鎯呭喌涓嬮€氳繃 NAPI 杞璁惧浠ヨ幏鍙栦簨浠讹紙蹇欒疆璇?poll>锛夈€?

NAPI 澶勭悊閫氬父鍙戠敓鍦ㄨ蒋浠朵腑鏂笂涓嬫枃涓紝浣嗕篃鍙互閫夋嫨浣跨敤鍗曠嫭鐨勫収鏍哥嚎绋?threaded>鏉ヨ繘琛?
NAPI 澶勭悊銆?

鎬昏€岃█涔嬶紝NAPI 鍚戦┍鍔ㄥ睆钄戒簡浜嬩欢锛堟暟鎹寘鎺ユ敹涓庡彂閫侊級澶勭悊鐨勪笂涓嬫枃鍜岄厤缃€?

## 椹卞姩 API


NAPI 鏈€閲嶈鐨勪袱涓厓绱犳槸 struct napi_struct 鍜岀浉鍏崇殑 poll 鏂规硶銆俿truct napi_struct
淇濆瓨 NAPI 瀹炰緥鐨勭姸鎬侊紝鑰岃鏂规硶鍒欐槸椹卞姩鐗瑰畾鐨勪簨浠跺鐞嗙▼搴忋€傝鏂规硶閫氬父浼氶噴鏀惧凡鍙戦€佺殑
Tx 鏁版嵁鍖呭苟澶勭悊鏂版敹鍒扮殑鏁版嵁鍖呫€?


### 鎺у埗 API


netif_napi_add() 鍜?netif_napi_del() 鐢ㄤ簬浠庣郴缁熶腑娣诲姞/鍒犻櫎涓€涓?NAPI 瀹炰緥銆傝繖浜涘疄渚?
琚檮鍔犲埌浣滀负鍙傛暟浼犲叆鐨?netdevice 涓婏紙骞朵笖鍦?netdevice 娉ㄩ攢鏃朵細鑷姩鍒犻櫎锛夈€傚疄渚嬩互
绂佺敤鐘舵€佽娣诲姞銆?

napi_enable() 鍜?napi_disable() 绠＄悊绂佺敤鐘舵€併€備竴涓绂佺敤鐨?NAPI 涓嶈兘琚皟搴︼紝骞朵笖
淇濊瘉涓嶄細璋冪敤瀹冪殑 poll 鏂规硶銆俷api_disable() 浼氱瓑寰?NAPI 瀹炰緥鐨勬墍鏈夋潈琚噴鏀俱€?

鎺у埗 API 涓嶆槸骞傜瓑鐨勩€傛帶鍒?API 鐨勮皟鐢ㄥ浜庢暟鎹矾寰?API 鐨勫苟鍙戜娇鐢ㄦ槸瀹夊叏鐨勶紝浣嗘槸涓嶆纭?
鐨勬帶鍒?API 璋冪敤搴忓垪鍙兘瀵艰嚧宕╂簝銆佹閿佹垨绔炰簤鏉′欢銆備緥濡傦紝杩炵画澶氭璋冪敤 napi_disable()
浼氭閿併€?

### 鏁版嵁璺緞 API


napi_schedule() 鏄皟搴?NAPI 杞鐨勫熀鏈柟娉曘€傞┍鍔ㄥ簲璇ュ湪瀹冧滑鐨勪腑鏂鐞嗙▼搴忎腑璋冪敤姝?
鍑芥暟锛堟洿澶氫俊鎭 drv_sched锛夈€備竴娆″ napi_schedule() 鐨勬垚鍔熻皟鐢ㄥ皢鍙栧緱 NAPI 瀹炰緥鐨?
鎵€鏈夋潈銆?

涔嬪悗锛屽湪 NAPI 琚皟搴﹀悗锛岄┍鍔ㄧ殑 poll 鏂规硶浼氳璋冪敤鏉ュ鐞嗕簨浠?鏁版嵁鍖呫€傝鏂规硶鎺ュ彈涓€涓?
`budget` 鍙傛暟鈥斺€旈┍鍔ㄥ彲浠ヤ负浠绘剰鏁伴噺鐨?Tx 鏁版嵁鍖呭鐞嗗畬鎴愶紝浣嗗彧搴斿鐞嗘渶澶?`budget` 涓?
Rx 鏁版嵁鍖呫€俁x 澶勭悊閫氬父鏄傝吹寰楀銆?

鎹㈠彞璇濊锛屽浜?Rx 澶勭悊锛宍budget` 鍙傛暟闄愬埗浜嗛┍鍔ㄥ湪涓€娆¤疆璇腑鍙互澶勭悊鐨勬暟鎹寘鏁伴噺銆傚綋
`budget` 涓?0 鏃讹紝瀹屽叏涓嶈兘浣跨敤鍍忛〉姹犳垨 XDP 杩欐牱鐨?Rx 鐗瑰畾 API銆傛棤璁?`budget` 濡備綍閮?
搴旇繘琛?skb Tx 澶勭悊锛屼絾濡傛灉鍙傛暟涓?0锛岄┍鍔ㄤ笉鑳借皟鐢ㄤ换浣?XDP锛堟垨椤垫睜锛堿PI銆?


   `budget` 鍙傛暟鍙兘涓?0锛屽鏋滄牳蹇冨彧灏濊瘯澶勭悊 skb Tx 瀹屾垚鑰屾病鏈?Rx 鎴?XDP 鏁版嵁鍖呫€?

poll 鏂规硶杩斿洖宸插畬鎴愮殑宸ヤ綔閲忋€傚鏋滈┍鍔ㄤ粛鏈夋湭瀹屾垚鐨勫伐浣滆鍋氾紙渚嬪 `budget` 宸茶€楀敖锛夛紝
poll 鏂规硶搴旀濂借繑鍥?`budget`銆傚湪杩欑鎯呭喌涓嬶紝NAPI 瀹炰緥灏嗚鍐嶆鏈嶅姟/杞锛堟棤闇€鍐嶈
璋冨害锛夈€?

濡傛灉浜嬩欢澶勭悊宸插畬鎴愶紙鎵€鏈夋湭瀹屾垚鐨勬暟鎹寘閮藉凡澶勭悊锛夛紝poll 鏂规硶搴斿湪杩斿洖鍓嶈皟鐢?
napi_complete_done()銆俷api_complete_done() 閲婃斁璇ュ疄渚嬬殑鎵€鏈夋潈銆?


   澶勭悊瀹屾墍鏈変簨浠朵笖鎭板ソ鐢ㄤ簡 `budget` 鐨勬儏鍐靛繀椤诲皬蹇冨鐞嗐€傛病鏈夊姙娉曞悜鏍堟姤鍛婅繖涓紙缃曡锛?
   鐨勬儏鍐碉紝鎵€浠ラ┍鍔ㄥ繀椤昏涔堜笉璋冪敤 napi_complete_done() 骞剁瓑寰呭啀娆¤璋冪敤锛岃涔堣繑鍥?
   `budget - 1`銆?

   濡傛灉 `budget` 涓?0锛屽垯缁濅笉搴旇皟鐢?napi_complete_done()銆?

### 璋冪敤搴忓垪


椹卞姩涓嶅簲鍋囧畾璋冪敤鐨勭‘鍒囧簭鍒椼€傚嵆浣挎病鏈夐┍鍔ㄨ皟搴﹁瀹炰緥锛堥櫎闈炶瀹炰緥琚鐢級锛宲oll 鏂规硶
涔熷彲鑳借璋冪敤銆傜被浼煎湴锛屽嵆浣?napi_schedule() 鎴愬姛浜嗭紝涔熶笉鑳戒繚璇佷細璋冪敤 poll 鏂规硶
锛堜緥濡傦紝濡傛灉璇ュ疄渚嬭绂佺敤锛夈€?

濡?drvctrl 涓€鑺傛墍杩扳€斺€攏api_disable() 鍙婂叾鍚庡 poll 鏂规硶鐨勮皟鐢ㄥ彧绛夊緟瀹炰緥鐨勬墍鏈夋潈
琚噴鏀撅紝鑰屼笉鏄瓑寰?poll 鏂规硶閫€鍑恒€傝繖鎰忓懗鐫€椹卞姩鍦ㄨ皟鐢?napi_complete_done() 涔嬪悗搴?
閬垮厤璁块棶浠讳綍鏁版嵁缁撴瀯銆?


### 璋冨害涓?IRQ 灞忚斀


椹卞姩鍦ㄨ皟搴?NAPI 瀹炰緥鍚庡簲浣夸腑鏂繚鎸佸睆钄解€斺€斿湪 NAPI 杞瀹屾垚涔嬪墠锛屼换浣曡繘涓€姝ョ殑涓柇閮?
鏄笉蹇呰鐨勩€?

闇€瑕佹樉寮忓睆钄戒腑鏂殑椹卞姩锛堜笌涓柇琚澶囪嚜鍔ㄥ睆钄界浉鍙嶏級搴斾娇鐢?napi_schedule_prep() 鍜?
__napi_schedule() 璋冪敤锛?


  if (napi_schedule_prep(&v->napi)) {
      mydrv_mask_rxtx_irq(v->idx);
      /** 鍦ㄥ睆钄戒箣鍚庤皟搴︿互閬垮厤绔炰簤 **/
      __napi_schedule(&v->napi);
  }

鍙湁褰撳 napi_complete_done() 鐨勮皟鐢ㄦ垚鍔熷悗锛屾墠搴旇В闄や腑鏂睆钄斤細


  if (budget && napi_complete_done(&v->napi, work_done)) {
    mydrv_unmask_rxtx_irq(v->idx);
    return min(work_done, budget - 1);
  }

napi_schedule_irqoff() 鏄?napi_schedule() 鐨勪竴涓彉浣擄紝瀹冨埄鐢ㄤ簡鍦?IRQ 涓婁笅鏂囦腑琚皟鐢?
鎵€鎻愪緵鐨勪繚璇侊紙鏃犻渶灞忚斀涓柇锛夈€傚鏋?IRQ 琚嚎绋嬪寲锛堜緥濡傚惎鐢ㄤ簡 `PREEMPT_RT`锛夛紝
napi_schedule_irqoff() 浼氬洖閫€鍒?napi_schedule()銆?

### 瀹炰緥鍒伴槦鍒楃殑鏄犲皠


鐜颁唬璁惧姣忎釜鎺ュ彛鏈夊涓?NAPI 瀹炰緥锛坰truct napi_struct锛夈€傚浜庡疄渚嬪浣曟槧灏勫埌闃熷垪鍜?
涓柇锛屾病鏈変弗鏍肩殑瑕佹眰銆侼API 涓昏鏄竴涓疆璇?澶勭悊鎶借薄锛屾病鏈夌壒瀹氱殑闈㈠悜鐢ㄦ埛鐨勮涔夈€傝瘽铏?
濡傛锛屽ぇ澶氭暟缃戠粶璁惧鏈€缁堥兘浠ョ浉褰撶被浼肩殑鏂瑰紡浣跨敤 NAPI銆?

NAPI 瀹炰緥鏈€甯歌鐨勬槸涓庝腑鏂拰闃熷垪瀵癸紙闃熷垪瀵规槸涓€缁勫崟涓?Rx 鍜屽崟涓?Tx 闃熷垪锛変互 1:1:1 鐨?
鏂瑰紡瀵瑰簲銆?

鍦ㄤ笉澶父瑙佺殑鎯呭喌涓嬶紝涓€涓?NAPI 瀹炰緥鍙兘鐢ㄤ簬澶氫釜闃熷垪锛屾垨鑰?Rx 鍜?Tx 闃熷垪鍙互鐢卞崟涓?
鏍稿績涓婄殑鐙珛 NAPI 瀹炰緥鏈嶅姟銆備笉杩囷紝鏃犺闃熷垪濡備綍鍒嗛厤锛孨API 瀹炰緥鍜屼腑鏂箣闂撮€氬父浠嶇劧鏄?
1:1 鐨勬槧灏勩€?

鍊煎緱娉ㄦ剰鐨勬槸锛宔thtool API 浣跨敤鈥渃hannel鈥濇湳璇紝鍏朵腑姣忎釜 channel 鍙互鏄?`rx`銆乣tx`
鎴?`combined`銆傜洰鍓嶈繕涓嶆竻妤氫粈涔堟瀯鎴愪竴涓?channel锛涙帹鑽愮殑璇犻噴鏄皢 channel 鐞嗚В涓烘湇鍔′簬
缁欏畾绫诲瀷闃熷垪鐨?IRQ/NAPI銆備緥濡傦紝1 涓?`rx`銆? 涓?`tx` 鍜?1 涓?`combined` channel 鐨?
閰嶇疆棰勬湡浼氫娇鐢?3 涓腑鏂€? 涓?Rx 鍜?2 涓?Tx 闃熷垪銆?

### 鎸佷箙 NAPI 閰嶇疆


椹卞姩閫氬父鍔ㄦ€佸垎閰嶅拰閲婃斁 NAPI 瀹炰緥銆傝繖瀵艰嚧姣忔閲嶆柊鍒嗛厤 NAPI 瀹炰緥鏃堕兘浼氫涪澶变笌 NAPI 鐩稿叧鐨?
鐢ㄦ埛閰嶇疆銆俷etif_napi_add_config() API 閫氳繃鍩轰簬椹卞姩瀹氫箟鐨勭储寮曞€硷紙濡傞槦鍒楀彿锛夊皢姣忎釜 NAPI
瀹炰緥涓庢寔涔呯殑 NAPI 閰嶇疆鍏宠仈璧锋潵锛岄槻姝㈣繖绉嶉厤缃涪澶便€?

浣跨敤姝?API 鍙互瀹炵幇鎸佷箙鐨?NAPI ID锛堜互鍙婂叾浠栬缃級锛岃繖瀵逛娇鐢?`SO_INCOMING_NAPI_ID` 鐨?
鐢ㄦ埛绌洪棿绋嬪簭鏄湁鐩婄殑銆傚叾浠?NAPI 閰嶇疆璁剧疆瑙佷笅鏂囧悇鑺傘€?

椹卞姩搴斿敖鍙兘灏濊瘯浣跨敤 netif_napi_add_config()銆?

## 鐢ㄦ埛 API


鐢ㄦ埛涓?NAPI 鐨勪氦浜掍緷璧栦簬 NAPI 瀹炰緥 ID銆傝繖浜涘疄渚?ID 鍙湁閫氳繃 `SO_INCOMING_NAPI_ID` 濂楁帴瀛?
閫夐」瀵圭敤鎴峰彲瑙併€?

鐢ㄦ埛鍙互浣跨敤 netlink 鏌ヨ璁惧鎴栬澶囬槦鍒楃殑 NAPI ID銆傝繖鍙互鍦ㄧ敤鎴峰簲鐢ㄧ▼搴忎腑浠ョ紪绋嬫柟寮?
瀹屾垚锛屾垨鑰呬娇鐢ㄥ唴鏍告簮鐮佹爲涓檮甯︾殑鑴氭湰锛歚tools/net/ynl/pyynl/cli.py`銆?

渚嬪锛屼娇鐢ㄨ剼鏈浆鍌ㄨ澶囩殑鎵€鏈夐槦鍒楋紙杩欏皢鏄剧ず姣忎釜闃熷垪鐨?NAPI ID锛夛細


   $ kernel-source/tools/net/ynl/pyynl/cli.py \
             --spec Documentation/netlink/specs/netdev.yaml \
             --dump queue-get \
             --json='{"ifindex": 2}'

鍏充簬鍙敤鎿嶄綔鍜屽睘鎬х殑鏇村缁嗚妭锛岃鍙傝 `Documentation/netlink/specs/netdev.yaml`銆?

### 杞欢 IRQ 鍚堝苟


榛樿鎯呭喌涓嬶紝NAPI 涓嶆墽琛屼换浣曟樉寮忕殑浜嬩欢鍚堝苟銆傚湪澶у鏁板満鏅腑锛屾壒澶勭悊鏄敱浜庤澶囧畬鎴愮殑
IRQ 鍚堝苟鑰屽彂鐢熺殑銆傛湁浜涙儏鍐典笅杞欢鍚堝苟鏄湁甯姪鐨勩€?

NAPI 鍙互閰嶇疆涓哄湪鏁版嵁鍖呭叏閮ㄥ鐞嗗畬鍚庯紝鎸傝捣涓€涓噸鏂拌疆璇㈠畾鏃跺櫒锛岃€屼笉鏄В闄ょ‖浠朵腑鏂殑
灞忚斀銆俷etdevice 鐨?`gro_flush_timeout` sysfs 閰嶇疆琚鐢ㄧ敤浜庢帶鍒惰瀹氭椂鍣ㄧ殑寤惰繜锛岃€?
`napi_defer_hard_irqs` 鎺у埗鍦?NAPI 鏀惧純骞跺洖鍒颁娇鐢ㄧ‖浠?IRQ 涔嬪墠杩炵画绌鸿疆璇㈢殑娆℃暟銆?

涓婅堪鍙傛暟涔熷彲浠ヤ娇鐢?netlink 閫氳繃 netdev-genl 鍦ㄦ瘡涓?NAPI 鐨勫熀纭€涓婅缃€傚綋涓?netlink
涓€璧蜂娇鐢ㄥ苟鍩轰簬姣忎釜 NAPI 閰嶇疆鏃讹紝涓婅堪鍙傛暟浣跨敤杩炲瓧绗﹁€屼笉鏄笅鍒掔嚎锛歚gro-flush-timeout`
鍜?`napi-defer-hard-irqs`銆?

鍩轰簬姣忎釜 NAPI 鐨勯厤缃彲浠ュ湪鐢ㄦ埛搴旂敤绋嬪簭涓互缂栫▼鏂瑰紡瀹屾垚锛屾垨鑰呬娇鐢ㄥ唴鏍告簮鐮佹爲涓檮甯︾殑
鑴氭湰锛歚tools/net/ynl/pyynl/cli.py`銆?

渚嬪锛屼娇鐢ㄨ剼鏈細


  $ kernel-source/tools/net/ynl/pyynl/cli.py \
            --spec Documentation/netlink/specs/netdev.yaml \
            --do napi-set \
            --json='{"id": 345,
                     "defer-hard-irqs": 111,
                     "gro-flush-timeout": 11111}'

绫讳技鍦帮紝鍙傛暟 `irq-suspend-timeout` 鍙互浣跨敤 netlink 閫氳繃 netdev-genl 璁剧疆銆傛病鏈夌敤浜?
姝ゅ€肩殑鍏ㄥ眬 sysfs 鍙傛暟銆?

`irq-suspend-timeout` 鐢ㄤ簬纭畾搴旂敤绋嬪簭鍙互瀹屽叏鎸傝捣 IRQ 澶氶暱鏃堕棿銆傚畠涓?SO_PREFER_BUSY_POLL
缁撳悎浣跨敤锛屽悗鑰呭彲浠ュ熀浜庢瘡涓?epoll 涓婁笅鏂囬€氳繃 `EPIOCSPARAMS` ioctl 璁剧疆銆?


### 蹇欒疆璇?


蹇欒疆璇㈠厑璁哥敤鎴峰湪璁惧涓柇瑙﹀彂涔嬪墠妫€鏌ユ槸鍚︽湁浼犲叆鐨勬暟鎹寘銆備笌浠讳綍褰㈠紡鐨勫繖杞涓€鏍凤紝瀹?
浠?CPU 鍛ㄦ湡涓轰唬浠锋崲鍙栨洿浣庣殑寤惰繜锛圢API 蹇欒疆璇㈢殑鐢熶骇鐢ㄩ€斿皻涓嶄负浜烘墍鐭ワ級銆?

蹇欒疆璇㈤€氳繃瑕佷箞鍦ㄩ€夊畾鐨勫鎺ュ瓧涓婅缃?`SO_BUSY_POLL`锛岃涔堜娇鐢ㄥ叏灞€鐨?`net.core.busy_poll`
鍜?`net.core.busy_read` sysctl 鏉ュ惎鐢ㄣ€備篃瀛樺湪涓€涓敤浜?NAPI 蹇欒疆璇㈢殑 io_uring API銆侼API
鐨勭嚎绋嬪寲杞涔熸湁涓€绉嶆ā寮忥紝浣跨敤 NAPI 澶勭悊 kthread 鏉ュ繖杞鏁版嵁鍖咃紙绾跨▼鍖栧繖杞
<threaded_busy_poll>锛夈€?

### 鍩轰簬 epoll 鐨勫繖杞


鍙互鐩存帴浠庡 `epoll_wait` 鐨勮皟鐢ㄨЕ鍙戞暟鎹寘澶勭悊銆備负浜嗕娇鐢ㄦ鍔熻兘锛岀敤鎴峰簲鐢ㄧ▼搴忓繀椤荤‘淇?
娣诲姞鍒?epoll 涓婁笅鏂囩殑鎵€鏈夋枃浠舵弿杩扮鍏锋湁鐩稿悓鐨?NAPI ID銆?

濡傛灉搴旂敤绋嬪簭浣跨敤涓撶敤鐨勬帴鏀剁嚎绋嬶紝搴旂敤绋嬪簭鍙互浣跨敤 SO_INCOMING_NAPI_ID 鑾峰彇浼犲叆杩炴帴鐨?
NAPI ID锛岀劧鍚庡皢璇ユ枃浠舵弿杩扮鍒嗗彂缁欏伐浣滅嚎绋嬨€傚伐浣滅嚎绋嬩細灏嗚鏂囦欢鎻忚堪绗︽坊鍔犲埌瀹冪殑 epoll
涓婁笅鏂囥€傝繖灏嗙‘淇濇瘡涓伐浣滅嚎绋嬮兘鏈変竴涓寘鍚叿鏈夌浉鍚?NAPI ID 鐨?FD 鐨?epoll 涓婁笅鏂囥€?

鎴栬€咃紝濡傛灉搴旂敤绋嬪簭浣跨敤 SO_REUSEPORT锛屽彲浠ユ彃鍏ヤ竴涓?bpf 鎴?ebpf 绋嬪簭鏉ュ皢浼犲叆杩炴帴鍒嗗彂鍒?
绾跨▼锛屼娇寰楁瘡涓嚎绋嬪彧寰楀埌鍏锋湁鐩稿悓 NAPI ID 鐨勪紶鍏ヨ繛鎺ャ€傚繀椤诲皬蹇冨鐞嗙郴缁熷彲鑳芥湁澶氫釜 NIC
鐨勬儏鍐点€?

涓轰簡鍚敤蹇欒疆璇紝鏈変袱涓€夋嫨锛?

1. `/proc/sys/net/core/busy_poll` 鍙互璁剧疆涓轰互寰涓哄崟浣嶇殑鏃堕棿锛岀敤浜庡繖寰幆绛夊緟浜嬩欢銆?
   杩欐槸涓€涓郴缁熻寖鍥寸殑璁剧疆锛屽皢瀵艰嚧鎵€鏈夊熀浜?epoll 鐨勫簲鐢ㄧ▼搴忓湪璋冪敤 epoll_wait 鏃跺繖杞銆?
   杩欏彲鑳藉苟涓嶅彲鍙栵紝鍥犱负璁稿搴旂敤绋嬪簭鍙兘涓嶉渶瑕佸繖杞銆?

2. 浣跨敤杈冩柊鍐呮牳鐨勫簲鐢ㄧ▼搴忓彲浠ュ湪 epoll 涓婁笅鏂囨枃浠舵弿杩扮涓婂彂鍑?ioctl 鏉ヨ缃紙`EPIOCSPARAMS`锛?
   鎴栬幏鍙栵紙`EPIOCGPARAMS`锛塦`struct epoll_params``:锛岀敤鎴风▼搴忓彲浠ュ涓嬪畾涔夛細


  struct epoll_params {
      uint32_t busy_poll_usecs;
      uint16_t busy_poll_budget;
      uint8_t prefer_busy_poll;

      /** 灏嗙粨鏋勫～鍏呭埌 64 浣嶇殑鍊嶆暟 **/
      uint8_t __pad;
  };

### IRQ 缂撹В


铏界劧蹇欒疆璇㈠簲璇ヨ浣庡欢杩熷簲鐢ㄧ▼搴忎娇鐢紝浣嗙被浼肩殑鏈哄埗鍙敤浜?IRQ 缂撹В銆?

闈炲父楂樻瘡绉掕姹傛暟鐨勫簲鐢ㄧ▼搴忥紙灏ゅ叾鏄矾鐢?杞彂搴旂敤绋嬪簭锛屽挨鍏舵槸浣跨敤 AF_XDP 濂楁帴瀛楃殑搴旂敤
绋嬪簭锛夊彲鑳藉笇鏈涘湪瀹屾垚澶勭悊涓€涓姹傛垨涓€鎵规暟鎹寘涔嬪墠涓嶈涓柇銆?

姝ょ被搴旂敤绋嬪簭鍙互鍚戝唴鏍镐繚璇佸畠浠皢瀹氭湡鎵ц蹇欒疆璇㈡搷浣滐紝骞朵笖椹卞姩搴旇浣胯澶?IRQ 姘镐箙灞忚斀銆?
姝ゆā寮忛€氳繃浣跨敤 `SO_PREFER_BUSY_POLL` 濂楁帴瀛楅€夐」鍚敤銆備负浜嗛伩鍏嶇郴缁熷紓甯歌涓猴紝濡傛灉
`gro_flush_timeout` 鍦ㄦ病鏈夊繖杞璋冪敤鐨勬儏鍐典笅杩囧幓锛岃淇濊瘉灏嗚鎾ら攢銆傚浜庡熀浜?epoll 鐨?
蹇欒疆璇㈠簲鐢ㄧ▼搴忥紝``struct epoll_params` 鐨?`prefer_busy_poll` 瀛楁鍙互璁句负 1锛屽苟鍙互鍙戝嚭
`EPIOCSPARAMS`` ioctl 鏉ュ惎鐢ㄦ妯″紡銆傛洿澶氱粏鑺傝涓婁竴鑺傘€?

NAPI 蹇欒疆璇㈢殑棰勭畻浣庝簬榛樿锛堣€冭檻鍒版甯稿繖杞鐨勪綆寤惰繜鎰忓浘锛岃繖鏄悎鐞嗙殑锛夈€傜劧鑰岋紝IRQ 缂撹В
骞堕潪濡傛锛屽洜姝ら绠楀彲浠ラ€氳繃 `SO_BUSY_POLL_BUDGET` 濂楁帴瀛楅€夐」璋冩暣銆傚浜庡熀浜?epoll 鐨勫繖
杞搴旂敤绋嬪簭锛屽彲浠ュ湪 `struct epoll_params` 涓皟鏁?`busy_poll_budget` 瀛楁涓烘墍闇€鐨勫€硷紝骞?
浣跨敤 `EPIOCSPARAMS` ioctl 璁剧疆鍦ㄧ壒瀹氱殑 epoll 涓婁笅鏂囦笂銆傛洿澶氱粏鑺傝涓婁竴鑺傘€?

闇€瑕佹敞鎰忕殑鏄紝涓?`gro_flush_timeout` 閫夋嫨涓€涓緝澶х殑鍊煎皢鎺ㄨ繜 IRQ 浠ュ厑璁告洿濂界殑鎵瑰鐞嗭紝浣?
浼氬湪绯荤粺鏈畬鍏ㄥ姞杞芥椂寮曞叆寤惰繜銆備负 `gro_flush_timeout` 閫夋嫨涓€涓緝灏忕殑鍊煎彲鑳戒細鍥犱负璁惧 IRQ
鍜岃蒋涓柇澶勭悊鑰屽共鎵版鍦ㄥ皾璇曞繖杞鐨勭敤鎴峰簲鐢ㄧ▼搴忋€傚簲鍦ㄨ€冭檻杩欎簺鏉冭　鐨勬儏鍐典笅浠旂粏閫夋嫨姝ゅ€笺€?
鍩轰簬 epoll 鐨勫繖杞搴旂敤绋嬪簭涔熻鑳藉閫氳繃涓?`maxevents` 閫夋嫨鍚堥€傜殑鍊兼潵缂撹В鏈夊灏戠敤鎴?
澶勭悊鍙戠敓銆?

鐢ㄦ埛鍙兘鎯宠€冭檻涓€绉嶆浛浠ｆ柟娉曪紝IRQ 鎸傝捣锛屾潵甯姪澶勭悊杩欎簺鏉冭　銆?

### IRQ 鎸傝捣


IRQ 鎸傝捣鏄竴绉嶆満鍒讹紝鍏朵腑鍦?epoll 瑙﹀彂 NAPI 鏁版嵁鍖呭鐞嗘椂灞忚斀璁惧 IRQ銆?

褰撳簲鐢ㄧ▼搴忓 epoll_wait 鐨勮皟鐢ㄦ垚鍔熸绱㈠埌浜嬩欢鏃讹紝鍐呮牳灏嗘帹杩?IRQ 鎸傝捣瀹氭椂鍣ㄣ€傚鏋滃唴鏍稿湪
蹇欒疆璇㈡椂娌℃湁妫€绱㈠埌浠讳綍浜嬩欢锛堜緥濡傦紝鍥犱负缃戠粶娴侀噺姘村钩涓嬮檷锛夛紝IRQ 鎸傝捣琚鐢紝骞跺惎鐢ㄤ笂杩?
IRQ 缂撹В绛栫暐銆?

杩欏厑璁哥敤鎴峰钩琛?CPU 娑堣€椾笌缃戠粶澶勭悊鏁堢巼銆?

瑕佷娇鐢ㄦ鏈哄埗锛?

  1. 搴斿皢鍩轰簬姣忎釜 NAPI 鐨勯厤缃弬鏁?`irq-suspend-timeout` 璁剧疆涓哄簲鐢ㄧ▼搴忓彲浠ユ寕璧峰叾 IRQ 鐨?
     鏈€闀挎椂闂达紙浠ョ撼绉掍负鍗曚綅锛夈€傝繖鏄娇鐢?netlink 瀹屾垚鐨勶紝濡備笂鎵€杩般€傛瓒呮椂浣滀负涓€涓畨鍏?
     鏈哄埗锛屽湪搴旂敤绋嬪簭鍋滄粸鏃堕噸鏂板惎鍔?IRQ 椹卞姩鐨勪腑鏂鐞嗐€傚簲閫夋嫨姝ゅ€间互瑕嗙洊鐢ㄦ埛搴旂敤绋嬪簭
     浠庡叾瀵?epoll_wait 鐨勮皟鐢ㄥ鐞嗘暟鎹墍闇€鐨勬椂闂撮噺锛屾敞鎰忓簲鐢ㄧ▼搴忓彲浠ラ€氳繃鍦ㄨ皟鐢?epoll_wait
     鏃惰缃?`max_events` 鏉ユ帶鍒跺畠浠绱㈠灏戞暟鎹€?

  2. sysfs 鍙傛暟鎴栧熀浜庢瘡涓?NAPI 鐨勯厤缃弬鏁?`gro_flush_timeout` 鍜?`napi_defer_hard_irqs`
     鍙互璁剧疆涓鸿緝灏忕殑鍊笺€傚畠浠皢鐢ㄤ簬鍦ㄥ繖杞娌℃湁鎵惧埌鏁版嵁鍚庢帹杩?IRQ銆?

  3. 蹇呴』灏?`prefer_busy_poll` 鏍囧織璁句负 true銆傝繖鍙互浣跨敤濡備笂鎵€杩扮殑 `EPIOCSPARAMS` ioctl
     瀹屾垚銆?

  4. 搴旂敤绋嬪簭濡備笂鎵€杩颁娇鐢?epoll 鏉ヨЕ鍙?NAPI 鏁版嵁鍖呭鐞嗐€?

濡備笂鎵€杩帮紝鍙鍚庣画瀵?epoll_wait 鐨勮皟鐢ㄥ悜鐢ㄦ埛绌洪棿杩斿洖浜嬩欢锛宍irq-suspend-timeout` 灏辫
鎺ㄨ繜锛孖RQ 琚鐢ㄣ€傝繖鍏佽搴旂敤绋嬪簭涓嶅彈骞叉壈鍦板鐞嗘暟鎹€?

涓€鏃﹀ epoll_wait 鐨勮皟鐢ㄦ病鏈夋壘鍒颁换浣曚簨浠讹紝IRQ 鎸傝捣琚嚜鍔ㄧ鐢紝骞朵笖 `gro_flush_timeout`
鍜?`napi_defer_hard_irqs` 缂撹В鏈哄埗鎺ョ銆?

棰勬湡 `irq-suspend-timeout` 浼氳璁剧疆涓烘瘮 `gro_flush_timeout` 澶у緱澶氱殑鍊硷紝鍥犱负
`irq-suspend-timeout` 搴旇鍦ㄤ竴涓敤鎴风┖闂村鐞嗗懆鏈熷唴鎸傝捣 IRQ銆?

铏界劧浣跨敤 IRQ 鎸傝捣骞朵笉涓ユ牸闇€瑕佷娇鐢?`napi_defer_hard_irqs` 鍜?`gro_flush_timeout`锛屼絾寮虹儓
寤鸿浣跨敤瀹冧滑銆?

IRQ 鎸傝捣浣跨郴缁熷湪杞妯″紡鍜屼腑鏂┍鍔ㄧ殑鏁版嵁鍖呬氦浠樹箣闂翠氦鏇裤€傚湪绻佸繖鏈熼棿锛宍irq-suspend-timeout`
瑕嗙洊 `gro_flush_timeout` 骞朵娇绯荤粺淇濇寔蹇欒疆璇紝浣嗗綋 epoll 娌℃湁鎵惧埌浜嬩欢鏃讹紝`gro_flush_timeout`
鍜?`napi_defer_hard_irqs` 鐨勮缃喅瀹氫笅涓€姝ャ€?

缃戠粶澶勭悊鍜屾暟鎹寘浜や粯鍩烘湰涓婃湁涓変釜鍙兘鐨勫惊鐜細

1) hardirq -> softirq -> napi poll锛涘熀鏈殑涓柇浜や粯
2) timer -> softirq -> napi poll锛涙帹杩熺殑 irq 澶勭悊
3) epoll -> busy-poll -> napi poll锛涘繖寰幆

濡傛灉璁剧疆浜?`gro_flush_timeout` 鍜?`napi_defer_hard_irqs`锛孡oop 2 鍙互浠?Loop 1 澶哄彇鎺у埗銆?

濡傛灉璁剧疆浜?`gro_flush_timeout` 鍜?`napi_defer_hard_irqs`锛孡oop 2 鍜?3 浼氱浉浜掆€滀簤澶衡€?
鎺у埗鏉冦€?

鍦ㄧ箒蹇欐湡闂达紝`irq-suspend-timeout` 鍦?Loop 2 涓敤浣滃畾鏃跺櫒锛岃繖鏈川涓婁娇缃戠粶澶勭悊鍋忓悜
Loop 3銆?

濡傛灉鏈缃?`gro_flush_timeout` 鍜?`napi_defer_hard_irqs`锛孡oop 3 涓嶈兘浠?Loop 1 澶哄彇
鎺у埗銆?

鍥犳锛屽缓璁缃?`gro_flush_timeout` 鍜?`napi_defer_hard_irqs`锛屽洜涓哄惁鍒欒缃?
`irq-suspend-timeout` 鍙兘娌℃湁浠讳綍鍙鲸鍒殑鏁堟灉銆?


### 绾跨▼鍖?NAPI 蹇欒疆璇?


绾跨▼鍖?NAPI 蹇欒疆璇㈡墿灞曚簡绾跨▼鍖?NAPI锛屽苟娣诲姞浜嗗 NAPI 杩涜杩炵画蹇欒疆璇㈢殑鏀寔銆傝繖瀵硅浆鍙戞垨
AF_XDP 搴旂敤绋嬪簭寰堟湁鐢ㄣ€?

绾跨▼鍖?NAPI 蹇欒疆璇㈠彲浠ヤ娇鐢?Netlink 鍦ㄦ瘡涓?NIC 闃熷垪鐨勫熀纭€涓婂惎鐢ㄣ€?

渚嬪锛屼娇鐢ㄤ互涓嬭剼鏈細


  $ ynl --family netdev --do napi-set \
            --json='{"id": 66, "threaded": "busy-poll"}'

鍐呮牳灏嗗垱寤轰竴涓湪璇?NAPI 涓婂繖杞鐨?kthread銆?

鐢ㄦ埛鍙互閫夋嫨灏嗘 kthread 鐨?CPU 浜插拰鎬ц缃负涓€涓湭浣跨敤鐨?CPU 鏍稿績锛屼互鎻愰珮 NAPI 琚疆璇?
鐨勯鐜囷紝浠ｄ环鏄氮璐?CPU 鍛ㄦ湡銆傛敞鎰忥紝杩欏皢浣胯 CPU 鏍稿績淇濇寔 100% 鐨勪娇鐢ㄧ巼銆?

涓€鏃︿负鏌愪釜 NAPI 鍚敤浜嗙嚎绋嬪寲蹇欒疆璇紝灏卞彲浠ヤ娇鐢?Netlink 鑾峰彇璇?kthread 鐨?PID锛屼互渚胯缃?
璇?kthread 鐨勪翰鍜屾€с€?

渚嬪锛屽彲浠ヤ娇鐢ㄤ互涓嬭剼鏈幏鍙?PID锛?


  $ ynl --family netdev --do napi-get --json='{"id": 66}'

杩欏皢杈撳嚭绫讳技浠ヤ笅鍐呭锛宲id `258` 鏄鍦ㄨ疆璇㈡ NAPI 鐨?kthread 鐨?PID銆?


  $ {'defer-hard-irqs': 0,
     'gro-flush-timeout': 0,
     'id': 66,
     'ifindex': 2,
     'irq-suspend-timeout': 0,
     'pid': 258,
     'threaded': 'busy-poll'}


### 绾跨▼鍖?NAPI


绾跨▼鍖?NAPI 鏄竴绉嶆搷浣滄ā寮忥紝瀹冧娇鐢ㄤ笓鐢ㄧ殑鍐呮牳绾跨▼鑰屼笉鏄蒋浠?IRQ 涓婁笅鏂囨潵杩涜 NAPI 澶勭悊銆?
姣忎釜绾跨▼鍖?NAPI 瀹炰緥灏嗙敓鎴愪竴涓崟鐙殑绾跨▼锛堢О涓?`napi/${ifc-name}-${napi-id}`锛夈€?

寤鸿灏嗘瘡涓唴鏍哥嚎绋嬪浐瀹氬埌鍗曚釜 CPU锛屽嵆鏈嶅姟璇ヤ腑鏂殑鍚屼竴涓?CPU銆傛敞鎰忥紝IRQ 鍜?NAPI 瀹炰緥涔嬮棿
鐨勬槧灏勫彲鑳藉苟涓嶇畝鍗曪紙骞朵笖渚濊禆浜庨┍鍔級銆侼API 瀹炰緥 ID 灏嗕互涓庡唴鏍哥嚎绋嬬殑杩涚▼ ID 鐩稿弽鐨勯『搴?
鍒嗛厤銆?

绾跨▼鍖?NAPI 閫氳繃灏?0/1 鍐欏叆 netdev 鐨?sysfs 鐩綍涓殑 `threaded` 鏂囦欢鏉ユ帶鍒躲€傚畠涔熷彲浠ヤ娇鐢?
netlink 鎺ュ彛涓虹壒瀹氱殑 NAPI 鍚敤銆?

渚嬪锛屼娇鐢ㄨ剼鏈細


  $ ynl --family netdev --do napi-set --json='{"id": 66, "threaded": 1}'
