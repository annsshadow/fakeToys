
## Linux 鍐呮牳 dpll 瀛愮郴缁?

## DPLL


PLL - Phase Locked Loop锛堥攣鐩哥幆锛夋槸涓€绉嶇數瀛愮數璺紝鐢ㄤ簬灏嗚澶囩殑鏃堕挓淇″彿涓?澶栭儴鏃堕挓淇″彿鍚屾銆傚畠鏈夋晥鍦颁娇璁惧鑳藉鎸夌収 PLL 杈撳叆鎵€鎻愪緵鐨勭浉鍚屾椂閽熻妭鎷嶈繍琛屻€?
DPLL - Digital Phase Locked Loop锛堟暟瀛楅攣鐩哥幆锛夋槸涓€绉嶉泦鎴愮數璺紝闄や簡鏅€?PLL
鐨勮涓哄锛岃繕鍖呭惈涓€涓暟瀛楅壌鐩稿櫒锛屽苟涓斿彲鑳藉湪鐜矾涓甫鏈夋暟瀛楀垎棰戝櫒銆傚洜姝わ紝DPLL
杈撳叆鍜岃緭鍑轰笂鐨勯鐜囧彲鑳芥槸鍙厤缃殑銆?
## 瀛愮郴缁?

dpll 瀛愮郴缁熺殑涓昏鐩殑鏄彁渚涗竴涓€氱敤鎺ュ彛锛岀敤浜庨厤缃偅浜涗娇鐢ㄤ换浣曚竴绫绘暟瀛?PLL銆佸苟
鍙互浣跨敤涓嶅悓鏉ユ簮鐨勮緭鍏ヤ俊鍙疯繘琛屽悓姝ャ€佷互鍙婂叿鏈変笉鍚岀被鍨嬭緭鍑虹殑璁惧銆?鍏朵富瑕佹帴鍙ｆ槸寤虹珛鍦?NETLINK_GENERIC 涔嬩笂鐨勫崗璁紝骞跺畾涔変簡涓€涓簨浠剁洃瑙嗗鎾粍銆?
## 璁惧瀵硅薄


鍗曚釜 dpll 璁惧瀵硅薄琛ㄧず鍗曚釜鏁板瓧 PLL 鐢佃矾浠ュ強涓€缁勭浉杩炵殑寮曡剼銆?瀹冧細鍝嶅簲鐢ㄦ埛瀵?netlink 鍛戒护 `DPLL_CMD_DEVICE_GET` 鐨?`do` 璇锋眰锛屾姤鍛婃墍鏀寔鐨?鎿嶄綔妯″紡鍜屽綋鍓嶇姸鎬侊紱骞堕€氳繃鍚屼竴鍛戒护鐨?`dump` netlink 璇锋眰锛屽垪鍑哄瓙绯荤粺涓凡娉ㄥ唽鐨?dpll 鍒楄〃銆?鏇存敼 dpll 璁惧鐨勯厤缃槸閫氳繃 netlink `DPLL_CMD_DEVICE_SET` 鍛戒护鐨?`do` 璇锋眰瀹屾垚鐨勩€?璁惧鍙ユ焺鏄?`DPLL_A_ID`锛屽湪鑾峰彇鎴栬缃郴缁熶腑鐗瑰畾璁惧鐨勯厤缃椂蹇呴』鎻愪緵瀹冦€傚畠鍙互閫氳繃
`DPLL_CMD_DEVICE_GET` 鐨?`dump` 璇锋眰锛屾垨鑰?`DPLL_CMD_DEVICE_ID_GET` 鐨?`do` 璇锋眰
鑾峰緱锛屽湪鍚庤€呬腑蹇呴』鎻愪緵鑳藉鍞竴鍖归厤鍗曚釜璁惧鐨勫睘鎬с€?
## 寮曡剼瀵硅薄


寮曡剼锛坧in锛夋槸涓€涓舰鎬佷笉鍥哄畾鐨勫璞★紝琛ㄧず杈撳叆鎴栬緭鍑猴紝瀹冨彲浠ユ槸璁惧鐨勫唴閮ㄧ粍浠讹紝
涔熷彲浠ユ槸澶栭儴杩炴帴鐨勩€?姣忎釜 dpll 鐨勫紩鑴氭暟閲忓悇涓嶇浉鍚岋紝浣嗗崟涓?dpll 璁惧閫氬父搴旀彁渚涘涓紩鑴氥€?寮曡剼鐨勫睘鎬с€佽兘鍔涘拰鐘舵€佷細鍦ㄥ搷搴?netlink `DPLL_CMD_PIN_GET` 鍛戒护鐨?`do` 璇锋眰鏃舵彁渚涚粰
鐢ㄦ埛銆?涔熷彲浠ラ€氳繃 `DPLL_CMD_PIN_GET` 鍛戒护鐨?`dump` 璇锋眰鍒楀嚭绯荤粺涓敞鍐岀殑鎵€鏈夊紩鑴氥€?寮曡剼鐨勯厤缃彲浠ラ€氳繃 netlink `DPLL_CMD_PIN_SET` 鍛戒护鐨?`do` 璇锋眰鏇存敼銆?寮曡剼鍙ユ焺鏄?`DPLL_A_PIN_ID`锛屽湪鑾峰彇鎴栬缃郴缁熶腑鐗瑰畾寮曡剼鐨勯厤缃椂蹇呴』鎻愪緵瀹冦€傚畠鍙互閫氳繃
`DPLL_CMD_PIN_GET` 鐨?`dump` 璇锋眰鎴?`DPLL_CMD_PIN_ID_GET` 鐨?`do` 璇锋眰鑾峰緱锛屽湪杩欎袱绉嶆柟寮忎腑
鐢ㄦ埛闇€鎻愪緵鑳藉鍞竴鍖归厤鍗曚釜寮曡剼鐨勫睘鎬с€?
## 寮曡剼閫夋嫨


涓€鑸€岃█锛岃閫変腑鐨勫紩鑴氾紙鍗冲叾淇″彿椹卞姩 dpll 璁惧鐨勫紩鑴氾級鍙互浠?`DPLL_A_PIN_STATE`
灞炴€ц幏寰楋紝骞朵笖瀵逛簬浠讳綍 dpll 璁惧锛屽彧鑳芥湁涓€涓紩鑴氬浜?`DPLL_PIN_STATE_CONNECTED`
鐘舵€併€?
寮曡剼閫夋嫨鍙互鎵嬪姩鎴栬嚜鍔ㄥ畬鎴愶紝鍙栧喅浜庣‖浠惰兘鍔涘拰褰撳墠 dpll 璁惧鐨勫伐浣滄ā寮?锛坄DPLL_A_MODE` 灞炴€э級銆傚叾缁撴灉鏄紝姣忕妯″紡鍦ㄥ彲鐢ㄥ紩鑴氱姸鎬佹柟闈紝浠ュ強鐢ㄦ埛鍙互璇锋眰鐨?dpll 璁惧鐘舵€佹柟闈紝閮藉瓨鍦ㄥ樊寮傘€?
鍦ㄦ墜鍔ㄦā寮忥紙`DPLL_MODE_MANUAL`锛変笅锛岀敤鎴峰彲浠ヨ姹傛垨鎺ユ敹浠ヤ笅寮曡剼鐘舵€佷箣涓€锛?
- `DPLL_PIN_STATE_CONNECTED` - 璇ュ紩鑴氱敤浜庨┍鍔?dpll 璁惧
- `DPLL_PIN_STATE_DISCONNECTED` - 璇ュ紩鑴氫笉鐢ㄤ簬椹卞姩 dpll 璁惧

鍦ㄨ嚜鍔ㄦā寮忥紙`DPLL_MODE_AUTOMATIC`锛変笅锛岀敤鎴峰彲浠ヨ姹傛垨鎺ユ敹浠ヤ笅寮曡剼鐘舵€佷箣涓€锛?
- `DPLL_PIN_STATE_SELECTABLE` - 璇ュ紩鑴氬簲琚涓鸿嚜鍔ㄩ€夋嫨绠楁硶鐨勬湁鏁堣緭鍏?- `DPLL_PIN_STATE_DISCONNECTED` - 璇ュ紩鑴氫笉搴旇瑙嗕负鑷姩閫夋嫨绠楁硶鐨勬湁鏁堣緭鍏?
鍦ㄨ嚜鍔ㄦā寮忥紙`DPLL_MODE_AUTOMATIC`锛変笅锛岀敤鎴峰彧鑳藉湪鑷姩閫夋嫨绠楁硶灏嗘煇涓緭鍏ラ攣瀹氬埌
dpll 璁惧涔嬪悗锛屾帴鏀跺埌寮曡剼鐘舵€?`DPLL_PIN_STATE_CONNECTED`銆?
## 鍏变韩寮曡剼


鍗曚釜寮曡剼瀵硅薄鍙互闄勫姞鍒板涓?dpll 璁惧銆?杩欐椂鏈変袱缁勯厤缃棆閽細

1) 鍦ㄥ紩鑴氫笂璁剧疆 - 璇ラ厤缃奖鍝嶅紩鑴氭敞鍐屽埌鐨勬墍鏈?dpll 璁惧锛堝嵆 `DPLL_A_PIN_FREQUENCY`锛夛紝
2) 鍦ㄥ紩鑴?dpll 鍏冪粍涓婅缃?- 璇ラ厤缃彧褰卞搷琚€変腑鐨?dpll 璁惧锛堝嵆 `DPLL_A_PIN_PRIO`銆?   `DPLL_A_PIN_STATE`銆乣DPLL_A_PIN_DIRECTION`锛夈€?
## MUX 鍨嬪紩鑴?

涓€涓紩鑴氬彲浠ユ槸 MUX 鍨嬶紙澶氳矾澶嶇敤锛夌殑锛屽畠鑱氬悎瀛愬紩鑴氬苟鍏呭綋寮曡剼澶氳矾澶嶇敤鍣ㄣ€備竴涓垨澶氫釜
寮曡剼浠?MUX 鍨嬫敞鍐岋紝鑰屼笉鏄洿鎺ユ敞鍐屽埌鏌愪釜 dpll 璁惧銆?浠?MUX 鍨嬪紩鑴氭敞鍐岀殑寮曡剼锛屽瀹冧滑娉ㄥ唽鍒扮殑姣忎釜鐖跺紩鑴氾紝浼氬悜鐢ㄦ埛鎻愪緵棰濆鐨勫祵濂楀睘鎬?`DPLL_A_PIN_PARENT_PIN`銆?濡傛灉涓€涓紩鑴氭敞鍐屼簡澶氫釜鐖跺紩鑴氾紝瀹冧滑鐨勮涓哄氨鍍忎竴涓杈撳嚭澶氳矾澶嶇敤鍣ㄣ€傝繖绉嶆儏鍐典笅
`DPLL_CMD_PIN_GET` 鐨勮緭鍑轰腑灏嗗寘鍚涓?pin-parent 宓屽
```

        'pin': [{{
          'clock-id': 282574471561216,
          'module-name': 'ice',
          'capabilities': 4,
          'id': 13,
          'parent-pin': [
          {'parent-id': 2, 'state': 'connected'},
          {'parent-id': 3, 'state': 'disconnected'}
          ],
          'type': 'synce-eth-port'
          }}]

```
鍚屼竴鏃跺埢鍙湁涓€涓瓙寮曡剼鑳藉皢鍏朵俊鍙锋彁渚涚粰鐖?MUX 鍨嬪紩鑴氾紝閫夋嫨鏄€氳繃鍦ㄦ湡鏈涚殑鐖跺紩鑴氫笂
璇锋眰鏇存敼鏌愪釜瀛愬紩鑴氱姸鎬佹潵瀹屾垚鐨勶紝浣跨敤 `DPLL_A_PIN_PARENT` 宓屽灞炴€с€俙set state on
parent pin`锛堝湪鐖跺紩鑴氫笂璁剧疆鐘舵€侊級娑堟伅鏍煎紡鐨勭ず渚嬶細

  ========================== =============================================
  `DPLL_A_PIN_ID`          child pin id锛堝瓙寮曡剼 id锛?  `DPLL_A_PIN_PARENT_PIN`  鐢ㄤ簬璇锋眰涓庣埗寮曡剼鐩稿叧閰嶇疆鐨勫祵濂楀睘鎬?                             related to parent pin
    `DPLL_A_PIN_PARENT_ID` parent pin id锛堢埗寮曡剼 id锛?    `DPLL_A_PIN_STATE`     鍦ㄧ埗寮曡剼涓婅姹傜殑寮曡剼鐘舵€?  ========================== =============================================

## 寮曡剼浼樺厛绾?

鏌愪簺璁惧鍙兘鎻愪緵鑷姩寮曡剼閫夋嫨妯″紡鐨勮兘鍔涳紙`DPLL_A_MODE` 灞炴€х殑鏋氫妇鍊?`DPLL_MODE_AUTOMATIC`锛夈€傞€氬父锛岃嚜鍔ㄩ€夋嫨鏄湪纭欢灞傞潰鎵ц鐨勶紝杩欐剰鍛崇潃鍙湁鐩存帴杩炴帴鍒?dpll 鐨勫紩鑴氭墠鑳界敤浜庤嚜鍔ㄨ緭鍏ュ紩鑴氶€夋嫨銆?鍦ㄨ嚜鍔ㄩ€夋嫨妯″紡涓嬶紝鐢ㄦ埛涓嶈兘鎵嬪姩閫夋嫨璁惧鐨勮緭鍏ュ紩鑴氾紝鑰屾槸搴斿綋涓烘墍鏈夌洿鎺ヨ繛鎺ョ殑寮曡剼
鎻愪緵浼樺厛绾?`DPLL_A_PIN_PRIO`锛岃澶囦細鎸戦€変紭鍏堢骇鏈€楂樼殑鏈夋晥淇″彿骞剁敤瀹冩潵鎺у埗 DPLL
璁惧銆俙set priority on parent pin`锛堝湪鐖跺紩鑴氫笂璁剧疆浼樺厛绾э級娑堟伅鏍煎紡鐨勭ず渚嬶細

  ============================ =============================================
  `DPLL_A_PIN_ID`            閰嶇疆鐨勫紩鑴?id
  `DPLL_A_PIN_PARENT_DEVICE` 鐢ㄤ簬璇锋眰涓庣埗 dpll 璁惧鐩稿叧閰嶇疆鐨勫祵濂楀睘鎬?                               related to parent dpll device
    `DPLL_A_PIN_PARENT_ID`   鐖?dpll 璁惧 id
    `DPLL_A_PIN_PRIO`        鍦ㄧ埗 dpll 涓婅姹傜殑寮曡剼浼樺厛绾?  ============================ =============================================

MUX 鍨嬪紩鑴氱殑瀛愬紩鑴氫笉鍏峰鑷姩杈撳叆寮曡剼閫夋嫨鑳藉姏锛屼负浜嗛厤缃?MUX 鍨嬪紩鑴氱殑娲诲姩杈撳叆锛?鐢ㄦ埛闇€瑕佸儚 `MUX 鍨嬪紩鑴歚 涓€绔犳墍鎻忚堪鐨勯偅鏍凤紝鍦ㄧ埗寮曡剼涓婅姹傚瓙寮曡剼鐨勬湡鏈涚姸鎬併€?
## 鐩镐綅鍋忕Щ娴嬮噺涓庤皟鏁?

璁惧鍙兘鎻愪緵娴嬮噺寮曡剼涓庡叾鐖?dpll 璁惧涔嬮棿淇″彿鐩镐綅宸殑鑳藉姏銆傚鏋滄敮鎸佸紩鑴?dpll 鐩镐綅鍋忕Щ
娴嬮噺锛屽簲涓烘瘡涓埗 dpll 璁惧鎻愪緵 `DPLL_A_PIN_PHASE_OFFSET` 灞炴€с€傛姤鍛婄殑鐩镐綅鍋忕Щ鍙互鐢?鍏堝墠鍊煎拰褰撳墠娴嬮噺鐨勫钩鍧囧€兼潵璁＄畻锛屽叕寮忓涓嬶細

   curr\_avg = prev\_avg ** \frac{2^N-1}{2^N} + new\_val ** \frac{1}{2^N}

鍏朵腑 `curr_avg` 鏄綋鍓嶆姤鍛婄殑鐩镐綅鍋忕Щ锛宍prev_avg` 鏄厛鍓嶆姤鍛婄殑鍊硷紝`new_val` 鏄綋鍓?娴嬮噺鍊硷紝`N` 鏄钩鍧囧洜瀛愩€傞厤缃殑骞冲潎鍥犲瓙鍊奸€氳繃璁惧鐨?`DPLL_A_PHASE_OFFSET_AVG_FACTOR`
灞炴€ф彁渚涳紝鍙互浣跨敤鐩稿悓鐨勫睘鎬ч厤鍚?`DPLL_CMD_DEVICE_SET` 鍛戒护璇锋眰鏇存敼鍏跺€笺€?
  ================================== ======================================
  `DPLL_A_PHASE_OFFSET_AVG_FACTOR` 閰嶇疆鐨勭浉浣嶅亸绉诲钩鍧囧洜瀛愬€?                                     attr configured value of phase offset
                                     averaging factor
  ================================== ======================================

璁惧涔熷彲鑳芥彁渚涜皟鏁村紩鑴氫笂淇″彿鐩镐綅鐨勮兘鍔涖€傚鏋滄敮鎸佸紩鑴氱浉浣嶈皟鏁达紝鍒欏紩鑴氬彞鏌勭殑鏈€灏忋€佹渶澶?鍊间互鍙婄矑搴﹀簲鍦?`DPLL_CMD_PIN_GET` 鐨勫搷搴斾腑閫氳繃 `DPLL_A_PIN_PHASE_ADJUST_MIN`銆?`DPLL_A_PIN_PHASE_ADJUST_MAX` 鍜?`DPLL_A_PIN_PHASE_ADJUST_GRAN` 灞炴€ф彁渚涚粰鐢ㄦ埛銆?閰嶇疆鐨勭浉浣嶈皟鏁村€奸€氳繃寮曡剼鐨?`DPLL_A_PIN_PHASE_ADJUST` 灞炴€ф彁渚涳紝鍙互浣跨敤鐩稿悓鐨勫睘鎬?閰嶅悎 `DPLL_CMD_PIN_SET` 鍛戒护璇锋眰鏇存敼鍏跺€笺€?
  ================================ ==========================================
  `DPLL_A_PIN_ID`                閰嶇疆鐨勫紩鑴?id
  `DPLL_A_PIN_PHASE_ADJUST_GRAN` 鐩镐綅璋冩暣鍊肩殑绮掑害灞炴€?  `DPLL_A_PIN_PHASE_ADJUST_MIN`  鐩镐綅璋冩暣鐨勬渶灏忓€煎睘鎬?  `DPLL_A_PIN_PHASE_ADJUST_MAX`  鐩镐綅璋冩暣鐨勬渶澶у€煎睘鎬?  `DPLL_A_PIN_PHASE_ADJUST`      鍦ㄧ埗 dpll 璁惧涓婇厤缃殑鐩镐綅璋冩暣鍊煎睘鎬?                                   adjustment on parent dpll device
  `DPLL_A_PIN_PARENT_DEVICE`     鐢ㄤ簬璇锋眰缁欏畾鐖?dpll 璁惧閰嶇疆鐨勫祵濂楀睘鎬?                                   configuration on given parent dpll
                                   device
    `DPLL_A_PIN_PARENT_ID`       鐖?dpll 璁惧 id
    `DPLL_A_PIN_PHASE_OFFSET`    娴嬮噺鐨勫紩鑴氫笌鐖?dpll 璁惧涔嬮棿鐨勭浉浣嶅樊灞炴€?                                   between a pin and parent dpll device
  ================================ ==========================================

鎵€鏈変笌鐩镐綅鐩稿叧鐨勫€奸兘浠ョ毊绉掞紙pico seconds锛変负鍗曚綅锛岃〃绀轰俊鍙风浉浣嶄箣闂寸殑鏃堕棿宸€傝礋鍊艰〃绀?寮曡剼涓婁俊鍙风殑鐩镐綅鏃╀簬 dpll 鐨勪俊鍙枫€傛鍊艰〃绀哄紩鑴氫笂淇″彿鐨勭浉浣嶆櫄浜?dpll 鐨勪俊鍙枫€?
鐩镐綅璋冩暣锛堜互鍙婃渶灏忓拰鏈€澶у€硷級鏄暣鏁帮紝浣嗘祴閲忕殑鐩镐綅鍋忕Щ鍊兼槸甯?3 浣嶅皬鏁扮殑灏忔暟锛屽簲闄や互
`DPLL_PIN_PHASE_OFFSET_DIVIDER` 寰楀埌鏁存暟閮ㄥ垎锛屽苟鐢ㄥ彇妯￠櫎娉曞緱鍒板皬鏁伴儴鍒嗐€?
## 鐩镐綅鍋忕Щ鐩戣鍣?

鐩镐綅鍋忕Щ娴嬮噺閫氬父閽堝褰撳墠娲诲姩婧愭墽琛屻€傜劧鑰岋紝鏌愪簺 DPLL锛圖igital Phase-Locked Loop锛屾暟瀛?閿佺浉鐜級璁惧鍙兘鎻愪緵鐩戣鎵€鏈夊彲鐢ㄨ緭鍏ョ浉浣嶅亸绉荤殑鑳藉姏銆傚浜庢敮鎸佺殑 DPLL 璁惧锛岃灞炴€у拰
褰撳墠鍔熻兘鐘舵€佸簲鍖呭惈鍦?`DPLL_CMD_DEVICE_GET` 鍛戒护鐨勫搷搴旀秷鎭腑銆傚湪杩欑鎯呭喌涓嬶紝鐢ㄦ埛涔熷彲浠?閫氳繃 `DPLL_CMD_DEVICE_SET` 鍛戒护涓鸿灞炴€ц缃?`enum dpll_feature_state` 鍊兼潵鎺у埗璇ュ姛鑳姐€?涓€鏃﹀惎鐢紝杈撳叆鐨勭浉浣嶅亸绉绘祴閲忓€煎簲鍦?`DPLL_A_PIN_PHASE_OFFSET` 灞炴€т腑杩斿洖銆?
  =============================== ========================
  `DPLL_A_PHASE_OFFSET_MONITOR` 鍔熻兘鐨勭姸鎬佸睘鎬?  =============================== ========================

## 棰戠巼鐩戣鍣?

鏌愪簺 DPLL 璁惧鍙兘鎻愪緵娴嬮噺鎵€鏈夊彲鐢ㄨ緭鍏ュ紩鑴氬疄闄呴鐜囩殑鑳藉姏銆傚浜庢敮鎸佺殑 DPLL 璁惧锛岃灞炴€?鍜屽綋鍓嶅姛鑳界姸鎬佸簲鍖呭惈鍦?`DPLL_CMD_DEVICE_GET` 鍛戒护鐨勫搷搴旀秷鎭腑銆傚湪杩欑鎯呭喌涓嬶紝鐢ㄦ埛涔熷彲浠?閫氳繃 `DPLL_CMD_DEVICE_SET` 鍛戒护涓鸿灞炴€ц缃?`enum dpll_feature_state` 鍊兼潵鎺у埗璇ュ姛鑳姐€?涓€鏃﹀惎鐢紝姣忎釜杈撳叆寮曡剼鐨勬祴閲忚緭鍏ラ鐜囧簲鍦?`DPLL_A_PIN_MEASURED_FREQUENCY` 灞炴€т腑杩斿洖銆傝鍊?浠ユ璧吂锛坢Hz锛変负鍗曚綅锛屼娇鐢?`DPLL_PIN_MEASURED_FREQUENCY_DIVIDER` 浣滀负闄ゆ暟銆?
  =============================== ========================
  `DPLL_A_FREQUENCY_MONITOR`    鍔熻兘鐨勭姸鎬佸睘鎬?  =============================== ========================

## 宓屽叆寮?SYNC


璁惧鍙兘鎻愪緵浣跨敤 Embedded SYNC锛堝祵鍏ュ紡鍚屾锛夌壒鎬х殑鑳藉姏銆傚畠鍏佽灏嗛澶栫殑 SYNC 淇″彿宓屽叆鍒?寮曡剼鐨勫熀鏈鐜囦腑鈥斺€旀瘡褰?SYNC 淇″彿鑴夊啿鍙戠敓鏃讹紝宓屽叆涓€涓熀鏈鐜囦俊鍙风殑鐗规畩鑴夊啿銆傜敤鎴峰彲浠?閰嶇疆 Embedded SYNC 鐨勯鐜囥€侲mbedded SYNC 鑳藉姏濮嬬粓涓庣粰瀹氱殑鍩烘湰棰戠巼鍜岀‖浠惰兘鍔涚浉鍏炽€傛牴鎹?褰撳墠涓哄紩鑴氶厤缃殑鍩烘湰棰戠巼锛屼細鍚戠敤鎴锋彁渚涗竴缁勫彈鏀寔鐨?Embedded SYNC 棰戠巼銆?
  ========================================= =================================
  `DPLL_A_PIN_ESYNC_FREQUENCY`            褰撳墠 Embedded SYNC 棰戠巼
  `DPLL_A_PIN_ESYNC_FREQUENCY_SUPPORTED`  宓屽鐨勫彲鐢?Embedded SYNC 棰戠巼鑼冨洿
                                            frequency ranges
    `DPLL_A_PIN_FREQUENCY_MIN`            棰戠巼鐨勬渶灏忓€煎睘鎬?    `DPLL_A_PIN_FREQUENCY_MAX`            棰戠巼鐨勬渶澶у€煎睘鎬?  `DPLL_A_PIN_ESYNC_PULSE`                Embedded SYNC 鐨勮剦鍐茬被鍨?  ========================================= =================================

## 鍙傝€?SYNC


璁惧鍙兘鏀寔 Reference SYNC锛堝弬鑰冨悓姝ワ級鐗规€э紝瀹冨厑璁稿皢涓や釜杈撳叆缁勫悎鎴愪竴涓緭鍏ュ銆傚湪杩欑
閰嶇疆涓紝鏉ヨ嚜涓や釜杈撳叆鐨勬椂閽熶俊鍙烽兘鐢ㄤ簬鍚屾 DPLL 璁惧銆傞鐜囪緝楂樼殑淇″彿鐢ㄤ簬 DPLL 鐨勭幆璺?甯﹀锛岃€岄鐜囪緝浣庣殑淇″彿鐢ㄤ簬浣?DPLL 璁惧鐨勮緭鍑轰俊鍙峰悓姝ャ€傝鐗规€т娇寰楄兘澶熶粠澶栭儴婧愭彁渚涢珮
璐ㄩ噺鐨勭幆璺甫瀹戒俊鍙枫€?
鍏峰鑳藉姏鐨勮緭鍏ヤ細鎻愪緵涓€浠藉彲涓庝箣缁戝畾浠ュ垱寤?Reference SYNC 鐨勮緭鍏ュ垪琛ㄣ€傝鎺у埗姝ょ壒鎬э紝鐢ㄦ埛
蹇呴』涓虹洰鏍囧紩鑴氳姹傛湡鏈涚殑鐘舵€侊細浣跨敤 `DPLL_PIN_STATE_CONNECTED` 鍚敤锛屾垨浣跨敤
`DPLL_PIN_STATE_DISCONNECTED` 绂佺敤璇ョ壒鎬с€備竴涓緭鍏ュ紩鑴氬湪浠讳綍缁欏畾鏃跺埢鍙兘缁戝畾鍒板彟涓€涓?寮曡剼銆?
  ============================== ==========================================
  `DPLL_A_PIN_REFERENCE_SYNC`  鐢ㄤ簬鎻愪緵淇℃伅鎴栬姹傞厤缃?Reference SYNC 鐗规€х殑
                                 requesting configuration of the Reference
                                 SYNC feature
    `DPLL_A_PIN_ID`             Reference SYNC 鐗规€х殑鐩爣寮曡剼 id
    `DPLL_A_PIN_STATE`          Reference SYNC 杩炴帴鐨勭姸鎬?  ============================== ==========================================

## 閰嶇疆鍛戒护缁?

閰嶇疆鍛戒护鐢ㄤ簬鑾峰彇鏈夊叧宸叉敞鍐?dpll 璁惧锛堝拰寮曡剼锛夌殑淇℃伅锛屼互鍙婅缃澶囨垨寮曡剼鐨勯厤缃€?鐢变簬 dpll 璁惧蹇呴』琚娊璞″苟鍙嶆槧鐪熷疄纭欢锛屽洜姝ゆ棤娉曚粠鐢ㄦ埛绌洪棿閫氳繃 netlink 娣诲姞鏂扮殑 dpll
璁惧锛屾瘡涓澶囬兘搴旂敱鍏堕┍鍔ㄦ敞鍐屻€?
鎵€鏈?netlink 鍛戒护閮介渶瑕?`GENL_ADMIN_PERM`銆傝繖鏄负浜嗛槻姝㈡潵鑷湭鎺堟潈鐢ㄦ埛绌洪棿搴旂敤鐨勪换浣?鍨冨溇淇℃伅/DoS 鏀诲嚮銆?
## 甯︽湁鍙兘灞炴€х殑 netlink 鍛戒护鍒楄〃


鏍囪瘑 dpll 璁惧鍛戒护绫诲瀷鐨勫父閲忎娇鐢?`DPLL_CMD_` 鍓嶇紑锛屽苟鏍规嵁鍛戒护鐢ㄩ€斾娇鐢ㄥ悗缂€銆?dpll 璁惧鐩稿叧灞炴€т娇鐢?`DPLL_A_` 鍓嶇紑锛屽苟鏍规嵁灞炴€х敤閫斾娇鐢ㄥ悗缂€銆?
  ==================================== =================================
  `DPLL_CMD_DEVICE_ID_GET`           鑾峰彇璁惧 ID 鐨勫懡浠?    `DPLL_A_MODULE_NAME`             娉ㄥ唽鑰呯殑妯″潡鍚嶅睘鎬?    `DPLL_A_CLOCK_ID`                鍞竴鏃堕挓鏍囪瘑绗﹀睘鎬?                                       (EUI-64), as defined by the
                                       IEEE 1588 standard
    `DPLL_A_TYPE`                    dpll 璁惧绫诲瀷灞炴€?  ==================================== =================================

  ==================================== =================================
  `DPLL_CMD_DEVICE_GET`              鑾峰彇璁惧淇℃伅鎴栬浆鍌ㄥ彲鐢ㄨ澶囧垪琛ㄧ殑鍛戒护
                                       dump list of available devices
    `DPLL_A_ID`                      鍞竴 dpll 璁惧 ID 灞炴€?    `DPLL_A_MODULE_NAME`             娉ㄥ唽鑰呯殑妯″潡鍚嶅睘鎬?    `DPLL_A_CLOCK_ID`                鍞竴鏃堕挓鏍囪瘑绗﹀睘鎬?                                       (EUI-64), as defined by the
                                       IEEE 1588 standard
    `DPLL_A_MODE`                    閫夋嫨妯″紡灞炴€?    `DPLL_A_MODE_SUPPORTED`          鍙敤閫夋嫨妯″紡灞炴€?    `DPLL_A_LOCK_STATUS`             dpll 璁惧閿佺姸鎬佸睘鎬?    `DPLL_A_TEMP`                    璁惧娓╁害淇℃伅灞炴€?    `DPLL_A_TYPE`                    dpll 璁惧绫诲瀷灞炴€?  ==================================== =================================

  ==================================== =================================
  `DPLL_CMD_DEVICE_SET`              璁剧疆 dpll 璁惧閰嶇疆鐨勫懡浠?    `DPLL_A_ID`                      鍐呴儴 dpll 璁惧绱㈠紩灞炴€?    `DPLL_A_MODE`                    瑕侀厤缃殑閫夋嫨妯″紡灞炴€?  ==================================== =================================

鏍囪瘑寮曡剼鍛戒护绫诲瀷鐨勫父閲忎娇鐢?`DPLL_CMD_PIN_` 鍓嶇紑锛屽苟鏍规嵁鍛戒护鐢ㄩ€斾娇鐢ㄥ悗缂€銆?寮曡剼鐩稿叧灞炴€т娇鐢?`DPLL_A_PIN_` 鍓嶇紑锛屽苟鏍规嵁灞炴€х敤閫斾娇鐢ㄥ悗缂€銆?
  ==================================== =================================
  `DPLL_CMD_PIN_ID_GET`              鑾峰彇寮曡剼 ID 鐨勫懡浠?    `DPLL_A_PIN_MODULE_NAME`         娉ㄥ唽鑰呯殑妯″潡鍚嶅睘鎬?    `DPLL_A_PIN_CLOCK_ID`            鍞竴鏃堕挓鏍囪瘑绗﹀睘鎬?                                       (EUI-64), as defined by the
                                       IEEE 1588 standard
    `DPLL_A_PIN_BOARD_LABEL`         娉ㄥ唽鑰呮彁渚涚殑寮曡剼鏉挎爣绛惧睘鎬?                                       by registerer
    `DPLL_A_PIN_PANEL_LABEL`         娉ㄥ唽鑰呮彁渚涚殑寮曡剼闈㈡澘鏍囩灞炴€?                                       by registerer
    `DPLL_A_PIN_PACKAGE_LABEL`       娉ㄥ唽鑰呮彁渚涚殑寮曡剼灏佽鏍囩灞炴€?                                       by registerer
    `DPLL_A_PIN_TYPE`                寮曡剼绫诲瀷灞炴€?  ==================================== =================================

  ==================================== ==================================
  `DPLL_CMD_PIN_GET`                 鑾峰彇寮曡剼淇℃伅鎴栬浆鍌ㄥ彲鐢ㄥ紩鑴氬垪琛ㄧ殑鍛戒护
                                       list of available pins
    `DPLL_A_PIN_ID`                  鍞竴寮曡剼 ID 灞炴€?    `DPLL_A_PIN_MODULE_NAME`         娉ㄥ唽鑰呯殑妯″潡鍚嶅睘鎬?    `DPLL_A_PIN_CLOCK_ID`            鍞竴鏃堕挓鏍囪瘑绗﹀睘鎬?                                       (EUI-64), as defined by the
                                       IEEE 1588 standard
    `DPLL_A_PIN_BOARD_LABEL`         娉ㄥ唽鑰呮彁渚涚殑寮曡剼鏉挎爣绛惧睘鎬?                                       by registerer
    `DPLL_A_PIN_PANEL_LABEL`         娉ㄥ唽鑰呮彁渚涚殑寮曡剼闈㈡澘鏍囩灞炴€?                                       by registerer
    `DPLL_A_PIN_PACKAGE_LABEL`       娉ㄥ唽鑰呮彁渚涚殑寮曡剼灏佽鏍囩灞炴€?                                       by registerer
    `DPLL_A_PIN_TYPE`                寮曡剼绫诲瀷灞炴€?    `DPLL_A_PIN_FREQUENCY`           寮曡剼鐨勫綋鍓嶉鐜囧睘鎬?    `DPLL_A_PIN_FREQUENCY_SUPPORTED` 鎻愪緵鍙楁敮鎸侀鐜囩殑宓屽灞炴€?                                       frequencies
      `DPLL_A_PIN_ANY_FREQUENCY_MIN` 棰戠巼鐨勬渶灏忓€煎睘鎬?      `DPLL_A_PIN_ANY_FREQUENCY_MAX` 棰戠巼鐨勬渶澶у€煎睘鎬?    `DPLL_A_PIN_PHASE_ADJUST_GRAN`   鐩镐綅璋冩暣鍊肩殑绮掑害灞炴€?                                       adjustment value
    `DPLL_A_PIN_PHASE_ADJUST_MIN`    鐩镐綅璋冩暣鐨勬渶灏忓€煎睘鎬?                                       adjustment
    `DPLL_A_PIN_PHASE_ADJUST_MAX`    鐩镐綅璋冩暣鐨勬渶澶у€煎睘鎬?                                       adjustment
    `DPLL_A_PIN_PHASE_ADJUST`        鍦ㄧ埗璁惧涓婇厤缃殑鐩镐綅璋冩暣鍊煎睘鎬?                                       adjustment on parent device
    `DPLL_A_PIN_PARENT_DEVICE`       寮曡剼鎵€杩炴帴鐨勬瘡涓埗璁惧鐨勫祵濂楀睘鎬?                                       the pin is connected with
      `DPLL_A_PIN_PARENT_ID`         鐖?dpll 璁惧 id 灞炴€?      `DPLL_A_PIN_PRIO`              寮曡剼鍦?dpll 璁惧涓婄殑浼樺厛绾у睘鎬?                                       dpll device
      `DPLL_A_PIN_STATE`             寮曡剼鍦ㄧ埗 dpll 璁惧涓婄殑鐘舵€佸睘鎬?                                       dpll device
      `DPLL_A_PIN_DIRECTION`         寮曡剼鍦ㄧ埗 dpll 璁惧涓婄殑鏂瑰悜灞炴€?                                       dpll device
      `DPLL_A_PIN_PHASE_OFFSET`      寮曡剼涓庣埗 dpll 涔嬮棿娴嬮噺鐨勭浉浣嶅樊灞炴€?                                       between a pin and parent dpll
    `DPLL_A_PIN_PARENT_PIN`          寮曡剼鎵€杩炴帴鐨勬瘡涓埗寮曡剼鐨勫祵濂楀睘鎬?                                       the pin is connected with
      `DPLL_A_PIN_PARENT_ID`         鐖跺紩鑴?id 灞炴€?      `DPLL_A_PIN_STATE`             寮曡剼鍦ㄧ埗寮曡剼涓婄殑鐘舵€佸睘鎬?                                       pin
    `DPLL_A_PIN_CAPABILITIES`        寮曡剼鑳藉姏浣嶆帺鐮佸睘鎬?    `DPLL_A_PIN_MEASURED_FREQUENCY`  浠?mHz 涓哄崟浣嶇殑杈撳叆寮曡剼娴嬮噺棰戠巼灞炴€?                                       an input pin in mHz
  ==================================== ==================================

  ==================================== =================================
  `DPLL_CMD_PIN_SET`                 璁剧疆寮曡剼閰嶇疆鐨勫懡浠?    `DPLL_A_PIN_ID`                  鍞竴寮曡剼 ID 灞炴€?    `DPLL_A_PIN_FREQUENCY`           璇锋眰鐨勫紩鑴氶鐜囧睘鎬?    `DPLL_A_PIN_PHASE_ADJUST`        鍦ㄧ埗璁惧涓婅姹傜殑鐩镐綅璋冩暣鍊煎睘鎬?                                       adjustment on parent device
    `DPLL_A_PIN_PARENT_DEVICE`       姣忎釜鐖?dpll 璁惧閰嶇疆璇锋眰鐨勫祵濂楀睘鎬?                                       device configuration request
      `DPLL_A_PIN_PARENT_ID`         鐖?dpll 璁惧 id 灞炴€?      `DPLL_A_PIN_DIRECTION`         璇锋眰鐨勫紩鑴氭柟鍚戝睘鎬?      `DPLL_A_PIN_PRIO`              鍦?dpll 璁惧涓婅姹傜殑寮曡剼浼樺厛绾у睘鎬?                                       the dpll device
      `DPLL_A_PIN_STATE`             鍦?dpll 璁惧涓婅姹傜殑寮曡剼鐘舵€佸睘鎬?                                       the dpll device
    `DPLL_A_PIN_PARENT_PIN`          姣忎釜鐖跺紩鑴氶厤缃姹傜殑宓屽灞炴€?                                       configuration request
      `DPLL_A_PIN_PARENT_ID`         鐖跺紩鑴?id 灞炴€?      `DPLL_A_PIN_STATE`             鍦ㄧ埗寮曡剼涓婅姹傜殑寮曡剼鐘舵€佸睘鎬?                                       parent pin
  ==================================== =================================

## Netlink dump 璇锋眰


`DPLL_CMD_DEVICE_GET` 鍜?`DPLL_CMD_PIN_GET` 鍛戒护鑳藉杩涜 dump 绫诲瀷鐨?netlink 璇锋眰锛?杩欑鎯呭喌涓嬪搷搴旂殑鏍煎紡涓庡畠浠殑 `do` 璇锋眰鐩稿悓锛屼絾浼氳繑鍥炵郴缁熶腑娉ㄥ唽鐨勬瘡涓澶囨垨寮曡剼銆?
## SET 鍛戒护鏍煎紡


`DPLL_CMD_DEVICE_SET` - 涓轰簡瀹氫綅涓€涓?dpll 璁惧锛岀敤鎴锋彁渚?`DPLL_A_ID`锛屽畠鏄郴缁熶腑
dpll 璁惧鐨勫敮涓€鏍囪瘑绗︼紝浠ュ強姝ｅ湪閰嶇疆鐨勫弬鏁帮紙`DPLL_A_MODE`锛夈€?
`DPLL_CMD_PIN_SET` - 涓轰簡瀹氫綅涓€涓紩鑴氾紝鐢ㄦ埛蹇呴』鎻愪緵 `DPLL_A_PIN_ID`锛屽畠鏄郴缁熶腑寮曡剼鐨?鍞竴鏍囪瘑绗︺€傚悓鏃跺繀椤绘坊鍔犲凡閰嶇疆鐨勫紩鑴氬弬鏁般€?濡傛灉閰嶇疆浜?`DPLL_A_PIN_FREQUENCY`锛岃繖浼氬奖鍝嶄笌璇ュ紩鑴氱浉杩炵殑鎵€鏈?dpll 璁惧锛屽洜姝ら鐜?灞炴€т笉搴旇鍖呭惈鍦?`DPLL_A_PIN_PARENT_DEVICE` 涓€?鍏跺畠灞炴€э細`DPLL_A_PIN_PRIO`銆乣DPLL_A_PIN_STATE` 鎴?`DPLL_A_PIN_DIRECTION` 蹇呴』琚寘鍚湪
`DPLL_A_PIN_PARENT_DEVICE` 涓紝鍥犱负瀹冧滑鐨勯厤缃彧涓庣敱 `DPLL_A_PIN_PARENT_ID` 灞炴€у畾浣嶇殑
鏌愪竴涓埗 dpll 鐩稿叧锛岃€岃灞炴€т篃鏄宓屽涓墍蹇呴渶鐨勩€?瀵逛簬 MUX 鍨嬪紩鑴氾紝`DPLL_A_PIN_STATE` 灞炴€х殑閰嶇疆鏂瑰紡绫讳技锛屽嵆灏嗘墍闇€鐘舵€佸寘鍚湪
`DPLL_A_PIN_PARENT_PIN` 宓屽灞炴€т腑锛屽苟灏嗙洰鏍囩埗寮曡剼 id 鏀惧湪 `DPLL_A_PIN_PARENT_ID` 涓€?
涓€鑸€岃█锛屽彲浠ヤ竴娆℃€ч厤缃涓弬鏁帮紝浣嗗湪鍐呴儴姣忎釜鍙傛暟鏇存敼閮戒細鍗曠嫭璋冪敤锛岄厤缃『搴忔棤娉曚互
浠讳綍鏂瑰紡淇濊瘉銆?
## 閰嶇疆棰勫畾涔夋灇涓?


## 閫氱煡


dpll 璁惧鍙互鎻愪緵鏈夊叧璁惧鐘舵€佸彉鍖栫殑閫氱煡锛屽嵆閿佺姸鎬佸彉鍖栥€佽緭鍏?杈撳嚭鍙樺寲鎴栧叾瀹冨憡璀︺€?鏈変竴涓鎾粍鐢ㄤ簬閫氳繃 netlink 濂楁帴瀛楅€氱煡鐢ㄦ埛绌洪棿搴旂敤锛歚DPLL_MCGRP_MONITOR`

閫氱煡娑堟伅锛?
  ============================== =====================================
  `DPLL_CMD_DEVICE_CREATE_NTF` dpll 璁惧宸插垱寤?  `DPLL_CMD_DEVICE_DELETE_NTF` dpll 璁惧宸插垹闄?  `DPLL_CMD_DEVICE_CHANGE_NTF` dpll 璁惧宸叉敼鍙?  `DPLL_CMD_PIN_CREATE_NTF`    dpll 寮曡剼宸插垱寤?  `DPLL_CMD_PIN_DELETE_NTF`    dpll 寮曡剼宸插垹闄?  `DPLL_CMD_PIN_CHANGE_NTF`    dpll 寮曡剼宸叉敼鍙?  ============================== =====================================

浜嬩欢鏍煎紡涓庣浉搴旂殑 get 鍛戒护鐩稿悓銆?`DPLL_CMD_DEVICE_` 浜嬩欢鐨勬牸寮忎笌 `DPLL_CMD_DEVICE_GET` 鐨勫搷搴旂浉鍚屻€?`DPLL_CMD_PIN_` 浜嬩欢鐨勬牸寮忎笌 `DPLL_CMD_PIN_GET` 鐨勫搷搴旂浉鍚屻€?
## 璁惧椹卞姩瀹炵幇


璁惧閫氳繃 dpll_device_get() 璋冪敤鍒嗛厤銆備娇鐢ㄧ浉鍚屽弬鏁扮殑绗簩娆¤皟鐢ㄤ笉浼氬垱寤烘柊瀵硅薄锛岃€屾槸鎻愪緵
鎸囧悜缁欏畾鍙傛暟鍏堝墠鎵€鍒涜澶囩殑鎸囬拡锛屽悓鏃跺鍔犺瀵硅薄鐨勫紩鐢ㄨ鏁般€?璁惧閫氳繃 dpll_device_put() 璋冪敤閲婃斁锛屽畠棣栧厛鍑忓皯寮曠敤璁℃暟锛屼竴鏃﹀紩鐢ㄨ鏁版竻闆讹紝璇ュ璞″嵆琚?閿€姣併€?
璁惧搴斿疄鐜颁竴缁勬搷浣滐紝骞堕€氳繃 dpll_device_register() 娉ㄥ唽璁惧锛屾鏃跺畠瀵圭敤鎴峰彲鐢ㄣ€傚涓?椹卞姩瀹炰緥鍙互閫氳繃 dpll_device_get() 鑾峰彇瀵瑰畠鐨勫紩鐢紝涔熷彲浠ョ敤瀹冧滑鑷繁鐨?ops 鍜?priv 娉ㄥ唽
dpll 璁惧銆?
寮曡剼閫氳繃 dpll_pin_get() 鍗曠嫭鍒嗛厤锛屽叾宸ヤ綔鏂瑰紡绫讳技浜?dpll_device_get()銆傝鍑芥暟棣栧厛鍒涘缓
瀵硅薄锛岀劧鍚庡浜庢瘡娆′娇鐢ㄧ浉鍚屽弬鏁扮殑璋冪敤锛屽彧澧炲姞瀵硅薄鐨勫紩鐢ㄨ鏁般€俤pll_pin_put() 鐨勫伐浣滄柟寮?涔熺被浼间簬 dpll_device_put()銆?
涓€涓紩鑴氬彲浠ユ牴鎹‖浠堕渶瑕侊紝娉ㄥ唽鍒扮埗 dpll 璁惧鎴栫埗寮曡剼銆傛瘡娆℃敞鍐岄兘瑕佹眰娉ㄥ唽鑰呮彁渚涗竴缁?寮曡剼鍥炶皟锛屼互鍙婄敤浜庤皟鐢ㄥ畠浠殑绉佹湁鏁版嵁鎸囬拡锛?
- dpll_pin_register() - 灏嗗紩鑴氭敞鍐屽埌涓€涓?dpll 璁惧锛?- dpll_pin_on_pin_register() - 灏嗗紩鑴氭敞鍐屽埌鍙︿竴涓?MUX 鍨嬪紩鑴氥€?
娣诲姞鎴栫Щ闄?dpll 璁惧鐨勯€氱煡鏄湪瀛愮郴缁熷唴閮ㄥ垱寤虹殑銆?娉ㄥ唽/娉ㄩ攢寮曡剼鐨勯€氱煡涔熺敱瀛愮郴缁熻皟鐢ㄣ€?鏈夊叧 dpll 璁惧鎴栧紩鑴氱姸鎬佸彉鍖栫殑閫氱煡浠ヤ袱绉嶆柟寮忚皟鐢細

- 鍦?dpll 瀛愮郴缁熶笂鎴愬姛璇锋眰鏇存敼鍚庯紝瀛愮郴缁熻皟鐢ㄧ浉搴旂殑閫氱煡锛?- 鐢辫澶囬┍鍔ㄩ€氳繃 dpll_device_change_ntf() 鎴?dpll_pin_change_ntf() 璇锋眰锛屽綋椹卞姩鎶ュ憡鐘舵€?  鍙樺寲鏃躲€?
浣跨敤 dpll 鎺ュ彛鐨勮澶囬┍鍔ㄤ笉瑕佹眰瀹炵幇鎵€鏈夌殑鍥炶皟鎿嶄綔銆備笉杩囷紝鏈夊皯鏁板嚑涓槸蹇呴』瀹炵幇鐨勩€?dpll 璁惧绾у埆蹇呴渶鐨勫洖璋冩搷浣滐細

- `.mode_get`锛?- `.lock_status_get`銆?
寮曡剼绾у埆蹇呴渶鐨勫洖璋冩搷浣滐細

- `.state_on_dpll_get`锛堟敞鍐屽埌 dpll 璁惧鐨勫紩鑴氾級锛?- `.state_on_pin_get`锛堟敞鍐屽埌鐖跺紩鑴氱殑寮曡剼锛夛紝
- `.direction_get`銆?
姣忎釜鍏跺畠鎿嶄綔澶勭悊绋嬪簭閮戒細妫€鏌ユ槸鍚﹀瓨鍦紝鑻ョ壒瀹氬鐞嗙▼搴忕己澶卞垯杩斿洖 `-EOPNOTSUPP`銆?
鏈€绠€鍗曠殑瀹炵幇鍦?OCP TimeCard 椹卞姩涓€俹ps 缁撴瀯瀹氫箟濡備笅锛?

	static const struct dpll_device_ops dpll_ops = {
		.lock_status_get = ptp_ocp_dpll_lock_status_get,
		.mode_get = ptp_ocp_dpll_mode_get,
		.mode_supported = ptp_ocp_dpll_mode_supported,
	};

	static const struct dpll_pin_ops dpll_pins_ops = {
		.frequency_get = ptp_ocp_dpll_frequency_get,
		.frequency_set = ptp_ocp_dpll_frequency_set,
		.direction_get = ptp_ocp_dpll_direction_get,
		.direction_set = ptp_ocp_dpll_direction_set,
		.state_on_dpll_get = ptp_ocp_dpll_state_get,
	};

娉ㄥ唽閮ㄥ垎鐪嬭捣鏉ュ儚杩欐牱锛?

        clkid = pci_get_dsn(pdev);
        bp->dpll = dpll_device_get(clkid, 0, THIS_MODULE);
        if (IS_ERR(bp->dpll)) {
                err = PTR_ERR(bp->dpll);
                dev_err(&pdev->dev, "dpll_device_alloc failed\n");
                goto out;
        }

        err = dpll_device_register(bp->dpll, DPLL_TYPE_PPS, &dpll_ops, bp);
        if (err)
                goto out;

        for (i = 0; i < OCP_SMA_NUM; i++) {
                bp->sma[i].dpll_pin = dpll_pin_get(clkid, i, THIS_MODULE, &bp->sma[i].dpll_prop);
                if (IS_ERR(bp->sma[i].dpll_pin)) {
                        err = PTR_ERR(bp->dpll);
                        goto out_dpll;
                }

                err = dpll_pin_register(bp->dpll, bp->sma[i].dpll_pin, &dpll_pins_ops,
                                        &bp->sma[i]);
                if (err) {
                        dpll_pin_put(bp->sma[i].dpll_pin);
                        goto out_dpll;
                }
        }

鍦ㄩ敊璇矾寰勪腑锛屾垜浠繀椤讳互鐩稿弽鐨勯『搴忓洖閫€姣忎竴娆″垎閰嶏細


        while (i) {
                --i;
                dpll_pin_unregister(bp->dpll, bp->sma[i].dpll_pin, &dpll_pins_ops, &bp->sma[i]);
                dpll_pin_put(bp->sma[i].dpll_pin);
        }
        dpll_device_put(bp->dpll);

鏇村鏉傜殑绀轰緥鍙互鍦?Intel 鐨?ICE 椹卞姩鎴?nVidia 鐨?mlx5 椹卞姩涓壘鍒般€?
## SyncE 鍚敤


涓轰簡鍚敤 SyncE锛岄渶瑕佸厑璁镐竴涓蒋浠跺簲鐢ㄦ帶鍒?dpll 璁惧锛岃搴旂敤鐩戣骞堕厤缃?dpll 璁惧鐨勮緭鍏ワ紝
浠ュ搷搴?dpll 璁惧鍙婂叾杈撳叆鐨勫綋鍓嶇姸鎬併€?鍦ㄨ繖绉嶅満鏅笅锛宒pll 璁惧鐨勮緭鍏ヤ俊鍙蜂篃搴斿綋鏄彲閰嶇疆鐨勶紝浠ヤ究鐢ㄤ粠 PHY netdevice 鎭㈠鍑虹殑淇″彿
椹卞姩 dpll銆傝繖鏄€氳繃灏嗕竴涓紩鑴氭毚闇茬粰 netdevice鈥斺€旀妸寮曡剼闄勫姞鍒?netdevice 鏈韩鏉ュ疄鐜帮紝浣跨敤
`dpll_netdev_pin_set(struct net_device **dev, struct dpll_pin **dpll_pin)`銆?鏆撮湶鐨勫紩鑴?id 鍙ユ焺 `DPLL_A_PIN_ID` 涔嬪悗鍙敱鐢ㄦ埛璇嗗埆锛屽洜涓哄畠闄勫姞鍦?rtnetlink 瀵?`RTM_NEWLINK` 鍛戒护鐨勫搷搴斾腑鐨勫祵濂楀睘鎬?`IFLA_DPLL_PIN` 涓娿€?