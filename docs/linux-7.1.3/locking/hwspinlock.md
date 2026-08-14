## 纭欢鑷棆閿佹鏋?


## 绠€浠?


纭欢鑷棆閿佹ā鍧椾负寮傛瀯澶勭悊鍣ㄤ箣闂淬€佷互鍙婇偅浜涗笉鍦ㄥ崟涓€鍏变韩鎿嶄綔绯荤粺涓嬭繍琛岀殑澶勭悊鍣ㄤ箣闂达紝鎻愪緵鐢ㄤ簬鍚屾涓庝簰鏂ョ殑纭欢杈呭姪銆?

渚嬪锛孫MAP4 鎷ユ湁鍙屾牳 Cortex-A9銆佸弻鏍?Cortex-M3 浠ュ強涓€涓?C64x+ DSP锛屾瘡涓€涓兘杩愯鐫€涓嶅悓鐨勬搷浣滅郴缁燂紙涓绘牳 A9 閫氬父杩愯 Linux锛岃€屼粠鏍?M3 涓?DSP 杩愯鏌愮 RTOS锛夈€?

閫氱敤鐨?hwspinlock 妗嗘灦鍏佽涓庡钩鍙版棤鍏崇殑椹卞姩浣跨敤 hwspinlock 璁惧锛屼互璁块棶鍦ㄨ繙绋嬪鐞嗗櫒涔嬮棿鍏变韩鐨勬暟鎹粨鏋勨€斺€斿惁鍒欒繖浜涘鐞嗗櫒娌℃湁鍏朵粬鏈哄埗鏉ュ畬鎴愬悓姝ヤ笌浜掓枼鎿嶄綔銆?

渚嬪锛岃繖瀵逛簬澶勭悊鍣ㄩ棿閫氫俊鏄繀瑕佺殑锛氬湪 OMAP4 涓婏紝CPU 瀵嗛泦鐨勫濯掍綋浠诲姟鐢变富鏍稿嵏杞藉埌杩滅▼鐨?M3 鍜?鎴?C64x+ 浠庢牳澶勭悊鍣紙閫氳繃涓€涓悕涓?Syslink 鐨?IPC 瀛愮郴缁燂級銆?

涓轰簡瀹炵幇蹇€熺殑鍩轰簬娑堟伅鐨勯€氫俊锛岄渶瑕佹渶灏忓寲鐨勫唴鏍告敮鎸侊紝浠ュ皢鏉ヨ嚜杩滅▼澶勭悊鍣ㄧ殑娑堟伅鎶曢€掔粰鐩稿簲鐨勭敤鎴疯繘绋嬨€?

杩欑閫氫俊鍩轰簬鍦ㄨ繙绋嬪鐞嗗櫒涔嬮棿鍏变韩鐨勭畝鍗曟暟鎹粨鏋勶紝瀵瑰叾璁块棶浣跨敤 hwspinlock 妯″潡杩涜鍚屾锛堣繙绋嬪鐞嗗櫒鐩存帴灏嗘柊娑堟伅鏀惧叆璇ュ叡浜暟鎹粨鏋勪腑锛夈€?

閫氱敤鐨?hwspinlock 鎺ュ彛浣垮緱缂栧啓閫氱敤鐨勩€佷笌骞冲彴鏃犲叧鐨勯┍鍔ㄦ垚涓哄彲鑳姐€?

## 鐢ㄦ埛 API


```
  struct hwspinlock *hwspin_lock_request_specific(unsigned int id);
```
鍒嗛厤涓€涓壒瀹氱殑 hwspinlock id 骞惰繑鍥炲叾鍦板潃锛屽鏋滆 hwspinlock 宸茶鍗犵敤鍒欒繑鍥?NULL銆傞€氬父鏉跨骇浠ｇ爜浼氳皟鐢ㄦ鍑芥暟鏉ヤ负棰勫畾涔夌殑鐩殑淇濈暀鐗瑰畾鐨?hwspinlock id銆?

搴斾粠杩涚▼涓婁笅鏂囪皟鐢紙鍙兘鐫＄湢锛夈€?

```
  int of_hwspin_lock_get_id(struct device_node *np, int index);
```
妫€绱㈠熀浜?DT phandle 鐨勭壒瀹氶攣鐨勫叏灞€閿?id銆傝鍑芥暟涓?hwspinlock 妯″潡鐨?DT 鐢ㄦ埛鎻愪緵浜嗕竴绉嶈幏鍙栫壒瀹?hwspinlock 鍏ㄥ眬閿?id 鐨勬柟寮忥紝浠庤€屽彲浠ヤ娇鐢ㄥ父瑙勭殑 hwspin_lock_request_specific() API 鏉ヨ姹傚畠銆?

璇ュ嚱鏁版垚鍔熸椂杩斿洖涓€涓攣 id 鍙凤紝鑻?hwspinlock 璁惧灏氭湭鍚戞牳蹇冩敞鍐屽垯杩斿洖 -EPROBE_DEFER锛屽叾浠栨儏鍐典笅杩斿洖鍏朵粬閿欒鍊笺€?

搴斾粠杩涚▼涓婁笅鏂囪皟鐢紙鍙兘鐫＄湢锛夈€?

```
  int hwspin_lock_free(struct hwspinlock *hwlock);
```
閲婃斁鍏堝墠鍒嗛厤鐨?hwspinlock锛涙垚鍔熸椂杩斿洖 0锛屽け璐ユ椂杩斿洖鐩稿簲鐨勯敊璇爜锛堜緥濡傦紝鑻ヨ hwspinlock 宸茬粡绌洪棽锛屽垯杩斿洖 -EINVAL锛夈€?

搴斾粠杩涚▼涓婁笅鏂囪皟鐢紙鍙兘鐫＄湢锛夈€?

```
  int hwspin_lock_bust(struct hwspinlock *hwlock, unsigned int id);
```
鍦ㄩ獙璇?hwspinlock 鐨勬嫢鏈夎€呬箣鍚庯紝閲婃斁涓€涓厛鍓嶈幏鍙栫殑 hwspinlock锛涙垚鍔熸椂杩斿洖 0锛屽け璐ユ椂杩斿洖鐩稿簲鐨勯敊璇爜锛堜緥濡傦紝鑻ヨ bust 鎿嶄綔瀵圭壒瀹?hwspinlock 鏈畾涔夛紝鍒欒繑鍥?-EOPNOTSUPP锛夈€?

搴斾粠杩涚▼涓婁笅鏂囪皟鐢紙鍙兘鐫＄湢锛夈€?

```
  int hwspin_lock_timeout(struct hwspinlock *hwlock, unsigned int timeout);
```
浠ヨ秴鏃堕檺鍒讹紙浠ユ绉掍负鍗曚綅锛夐攣瀹氫竴涓厛鍓嶅垎閰嶇殑 hwspinlock銆傚鏋滆 hwspinlock 宸茶鍗犵敤锛屽嚱鏁颁細蹇欑瓑浠ョ瓑寰呭叾閲婃斁锛屼絾鍦ㄨ秴鏃惰€楀敖鏃舵斁寮冦€傛垚鍔熶粠姝ゅ嚱鏁拌繑鍥炲悗锛屾姠鍗犺绂佺敤锛屽洜姝よ皟鐢ㄨ€呬笉寰楃潯鐪狅紝骞跺缓璁敖蹇噴鏀?hwspinlock锛屼互鏈€灏忓寲杩滅▼鏍稿湪纭欢浜掕繛涓婄殑杞銆?

鎴愬姛鏃惰繑鍥?0锛屽惁鍒欒繑鍥炵浉搴旂殑閿欒鐮侊紙鏈€鍏稿瀷鐨勬槸 -ETIMEDOUT锛岃〃绀鸿秴鏃舵绉掑悗璇?hwspinlock 浠嶇劧蹇欙級銆傝鍑芥暟姘歌繙涓嶄細鐫＄湢銆?

```
  int hwspin_lock_timeout_irq(struct hwspinlock *hwlock, unsigned int timeout);
```
浠ヨ秴鏃堕檺鍒讹紙浠ユ绉掍负鍗曚綅锛夐攣瀹氫竴涓厛鍓嶅垎閰嶇殑 hwspinlock銆傚鏋滆 hwspinlock 宸茶鍗犵敤锛屽嚱鏁颁細蹇欑瓑浠ョ瓑寰呭叾閲婃斁锛屼絾鍦ㄨ秴鏃惰€楀敖鏃舵斁寮冦€傛垚鍔熶粠姝ゅ嚱鏁拌繑鍥炲悗锛屾姠鍗犱笌鏈湴涓柇琚鐢紝鍥犳璋冪敤鑰呬笉寰楃潯鐪狅紝骞跺缓璁敖蹇噴鏀?hwspinlock銆?

鎴愬姛鏃惰繑鍥?0锛屽惁鍒欒繑鍥炵浉搴旂殑閿欒鐮侊紙鏈€鍏稿瀷鐨勬槸 -ETIMEDOUT锛岃〃绀鸿秴鏃舵绉掑悗璇?hwspinlock 浠嶇劧蹇欙級銆傝鍑芥暟姘歌繙涓嶄細鐫＄湢銆?

```
  int hwspin_lock_timeout_irqsave(struct hwspinlock *hwlock, unsigned int to,
				  unsigned long *flags);
```
浠ヨ秴鏃堕檺鍒讹紙浠ユ绉掍负鍗曚綅锛夐攣瀹氫竴涓厛鍓嶅垎閰嶇殑 hwspinlock銆傚鏋滆 hwspinlock 宸茶鍗犵敤锛屽嚱鏁颁細蹇欑瓑浠ョ瓑寰呭叾閲婃斁锛屼絾鍦ㄨ秴鏃惰€楀敖鏃舵斁寮冦€傛垚鍔熶粠姝ゅ嚱鏁拌繑鍥炲悗锛屾姠鍗犺绂佺敤锛屾湰鍦颁腑鏂绂佺敤锛屽叾鍏堝墠鐨勭姸鎬佷繚瀛樺湪缁欏畾鐨?flags 鍗犱綅绗︿腑銆傝皟鐢ㄨ€呬笉寰楃潯鐪狅紝骞跺缓璁敖蹇噴鏀?hwspinlock銆?

鎴愬姛鏃惰繑鍥?0锛屽惁鍒欒繑鍥炵浉搴旂殑閿欒鐮侊紙鏈€鍏稿瀷鐨勬槸 -ETIMEDOUT锛岃〃绀鸿秴鏃舵绉掑悗璇?hwspinlock 浠嶇劧蹇欙級銆傝鍑芥暟姘歌繙涓嶄細鐫＄湢銆?

```
  int hwspin_lock_timeout_raw(struct hwspinlock *hwlock, unsigned int timeout);
```
浠ヨ秴鏃堕檺鍒讹紙浠ユ绉掍负鍗曚綅锛夐攣瀹氫竴涓厛鍓嶅垎閰嶇殑 hwspinlock銆傚鏋滆 hwspinlock 宸茶鍗犵敤锛屽嚱鏁颁細蹇欑瓑浠ョ瓑寰呭叾閲婃斁锛屼絾鍦ㄨ秴鏃惰€楀敖鏃舵斁寮冦€?

娉ㄦ剰锛氱敤鎴峰繀椤荤敤浜掓枼浣撴垨鑷棆閿佷繚鎶よ幏鍙栫‖浠堕攣鐨勪緥绋嬶紝浠ラ伩鍏嶆閿侊紝浠庤€岃鐢ㄦ埛鑳藉鍦ㄧ‖浠堕攣涓嬫墽琛屼竴浜涜€楁椂鐨勬垨鍙潯鐪犵殑鎿嶄綔銆?

鎴愬姛鏃惰繑鍥?0锛屽惁鍒欒繑鍥炵浉搴旂殑閿欒鐮侊紙鏈€鍏稿瀷鐨勬槸 -ETIMEDOUT锛岃〃绀鸿秴鏃舵绉掑悗璇?hwspinlock 浠嶇劧蹇欙級銆傝鍑芥暟姘歌繙涓嶄細鐫＄湢銆?

```
  int hwspin_lock_timeout_in_atomic(struct hwspinlock *hwlock, unsigned int to);
```
浠ヨ秴鏃堕檺鍒讹紙浠ユ绉掍负鍗曚綅锛夐攣瀹氫竴涓厛鍓嶅垎閰嶇殑 hwspinlock銆傚鏋滆 hwspinlock 宸茶鍗犵敤锛屽嚱鏁颁細蹇欑瓑浠ョ瓑寰呭叾閲婃斁锛屼絾鍦ㄨ秴鏃惰€楀敖鏃舵斁寮冦€?

姝ゅ嚱鏁板彧鑳戒粠鍘熷瓙涓婁笅鏂囪皟鐢紝涓旇秴鏃跺€间笉搴旇秴杩囧嚑姣銆?

鎴愬姛鏃惰繑鍥?0锛屽惁鍒欒繑鍥炵浉搴旂殑閿欒鐮侊紙鏈€鍏稿瀷鐨勬槸 -ETIMEDOUT锛岃〃绀鸿秴鏃舵绉掑悗璇?hwspinlock 浠嶇劧蹇欙級銆傝鍑芥暟姘歌繙涓嶄細鐫＄湢銆?

```
  int hwspin_trylock(struct hwspinlock *hwlock);
```
灏濊瘯閿佸畾涓€涓厛鍓嶅垎閰嶇殑 hwspinlock锛屼絾濡傛灉瀹冨凡琚崰鐢ㄥ垯绔嬪嵆澶辫触銆?

鎴愬姛浠庢鍑芥暟杩斿洖鍚庯紝鎶㈠崰琚鐢紝鍥犳璋冪敤鑰呬笉寰楃潯鐪狅紝骞跺缓璁敖蹇噴鏀?hwspinlock锛屼互鏈€灏忓寲杩滅▼鏍稿湪纭欢浜掕繛涓婄殑杞銆?

鎴愬姛鏃惰繑鍥?0锛屽惁鍒欒繑鍥炵浉搴旂殑閿欒鐮侊紙鏈€鍏稿瀷鐨勬槸 -EBUSY锛岃〃绀鸿 hwspinlock 宸茶鍗犵敤锛夈€傝鍑芥暟姘歌繙涓嶄細鐫＄湢銆?

```
  int hwspin_trylock_irq(struct hwspinlock *hwlock);
```
灏濊瘯閿佸畾涓€涓厛鍓嶅垎閰嶇殑 hwspinlock锛屼絾濡傛灉瀹冨凡琚崰鐢ㄥ垯绔嬪嵆澶辫触銆?

鎴愬姛浠庢鍑芥暟杩斿洖鍚庯紝鎶㈠崰涓庢湰鍦颁腑鏂绂佺敤锛屽洜姝よ皟鐢ㄨ€呬笉寰楃潯鐪狅紝骞跺缓璁敖蹇噴鏀?hwspinlock銆?

鎴愬姛鏃惰繑鍥?0锛屽惁鍒欒繑鍥炵浉搴旂殑閿欒鐮侊紙鏈€鍏稿瀷鐨勬槸 -EBUSY锛岃〃绀鸿 hwspinlock 宸茶鍗犵敤锛夈€傝鍑芥暟姘歌繙涓嶄細鐫＄湢銆?

```
  int hwspin_trylock_irqsave(struct hwspinlock *hwlock, unsigned long *flags);
```
灏濊瘯閿佸畾涓€涓厛鍓嶅垎閰嶇殑 hwspinlock锛屼絾濡傛灉瀹冨凡琚崰鐢ㄥ垯绔嬪嵆澶辫触銆?

鎴愬姛浠庢鍑芥暟杩斿洖鍚庯紝鎶㈠崰琚鐢紝鏈湴涓柇琚鐢紝鍏跺厛鍓嶇殑鐘舵€佷繚瀛樺湪缁欏畾鐨?flags 鍗犱綅绗︿腑銆傝皟鐢ㄨ€呬笉寰楃潯鐪狅紝骞跺缓璁敖蹇噴鏀?hwspinlock銆?

鎴愬姛鏃惰繑鍥?0锛屽惁鍒欒繑鍥炵浉搴旂殑閿欒鐮侊紙鏈€鍏稿瀷鐨勬槸 -EBUSY锛岃〃绀鸿 hwspinlock 宸茶鍗犵敤锛夈€傝鍑芥暟姘歌繙涓嶄細鐫＄湢銆?

```
  int hwspin_trylock_raw(struct hwspinlock *hwlock);
```
灏濊瘯閿佸畾涓€涓厛鍓嶅垎閰嶇殑 hwspinlock锛屼絾濡傛灉瀹冨凡琚崰鐢ㄥ垯绔嬪嵆澶辫触銆?

娉ㄦ剰锛氱敤鎴峰繀椤荤敤浜掓枼浣撴垨鑷棆閿佷繚鎶よ幏鍙栫‖浠堕攣鐨勪緥绋嬶紝浠ラ伩鍏嶆閿侊紝浠庤€岃鐢ㄦ埛鑳藉鍦ㄧ‖浠堕攣涓嬫墽琛屼竴浜涜€楁椂鐨勬垨鍙潯鐪犵殑鎿嶄綔銆?

鎴愬姛鏃惰繑鍥?0锛屽惁鍒欒繑鍥炵浉搴旂殑閿欒鐮侊紙鏈€鍏稿瀷鐨勬槸 -EBUSY锛岃〃绀鸿 hwspinlock 宸茶鍗犵敤锛夈€傝鍑芥暟姘歌繙涓嶄細鐫＄湢銆?

```
  int hwspin_trylock_in_atomic(struct hwspinlock *hwlock);
```
灏濊瘯閿佸畾涓€涓厛鍓嶅垎閰嶇殑 hwspinlock锛屼絾濡傛灉瀹冨凡琚崰鐢ㄥ垯绔嬪嵆澶辫触銆?

姝ゅ嚱鏁板彧鑳戒粠鍘熷瓙涓婁笅鏂囪皟鐢ㄣ€?

鎴愬姛鏃惰繑鍥?0锛屽惁鍒欒繑鍥炵浉搴旂殑閿欒鐮侊紙鏈€鍏稿瀷鐨勬槸 -EBUSY锛岃〃绀鸿 hwspinlock 宸茶鍗犵敤锛夈€傝鍑芥暟姘歌繙涓嶄細鐫＄湢銆?

```
  void hwspin_unlock(struct hwspinlock *hwlock);
```
瑙ｉ攣涓€涓厛鍓嶉攣瀹氱殑 hwspinlock銆傛€绘槸鎴愬姛锛屽苟涓斿彲浠ヤ粠浠讳綍涓婁笅鏂囪皟鐢紙璇ュ嚱鏁版案杩滀笉浼氱潯鐪狅級銆?


  浠ｇ爜**缁濅笉搴?*鍘昏В閿佷竴涓凡缁忚В閿佺殑 hwspinlock锛堝姝ゆ病鏈変换浣曚繚鎶わ級銆?

```
  void hwspin_unlock_irq(struct hwspinlock *hwlock);
```
瑙ｉ攣涓€涓厛鍓嶉攣瀹氱殑 hwspinlock 骞跺惎鐢ㄦ湰鍦颁腑鏂€傝皟鐢ㄨ€?*缁濅笉搴?*鍘昏В閿佷竴涓凡缁忚В閿佺殑 hwspinlock銆傝繖鏍峰仛琚涓轰竴涓己闄凤紙瀵规娌℃湁浠讳綍淇濇姢锛夈€傛垚鍔熶粠姝ゅ嚱鏁拌繑鍥炲悗锛屾姠鍗犱笌鏈湴涓柇琚惎鐢ㄣ€傝鍑芥暟姘歌繙涓嶄細鐫＄湢銆?

```
  void
  hwspin_unlock_irqrestore(struct hwspinlock *hwlock, unsigned long *flags);
```
瑙ｉ攣涓€涓厛鍓嶉攣瀹氱殑 hwspinlock銆?

璋冪敤鑰?*缁濅笉搴?*鍘昏В閿佷竴涓凡缁忚В閿佺殑 hwspinlock銆傝繖鏍峰仛琚涓轰竴涓己闄凤紙瀵规娌℃湁浠讳綍淇濇姢锛夈€傛垚鍔熶粠姝ゅ嚱鏁拌繑鍥炲悗锛屾姠鍗犺閲嶆柊鍚敤锛屾湰鍦颁腑鏂殑鐘舵€佽鎭㈠鍒颁繚瀛樺湪缁欏畾 flags 涓殑鐘舵€併€傝鍑芥暟姘歌繙涓嶄細鐫＄湢銆?

```
  void hwspin_unlock_raw(struct hwspinlock *hwlock);
```
瑙ｉ攣涓€涓厛鍓嶉攣瀹氱殑 hwspinlock銆?

璋冪敤鑰?*缁濅笉搴?*鍘昏В閿佷竴涓凡缁忚В閿佺殑 hwspinlock銆傝繖鏍峰仛琚涓轰竴涓己闄凤紙瀵规娌℃湁浠讳綍淇濇姢锛夈€傝鍑芥暟姘歌繙涓嶄細鐫＄湢銆?

```
  void hwspin_unlock_in_atomic(struct hwspinlock *hwlock);
```
瑙ｉ攣涓€涓厛鍓嶉攣瀹氱殑 hwspinlock銆?

璋冪敤鑰?*缁濅笉搴?*鍘昏В閿佷竴涓凡缁忚В閿佺殑 hwspinlock銆傝繖鏍峰仛琚涓轰竴涓己闄凤紙瀵规娌℃湁浠讳綍淇濇姢锛夈€傝鍑芥暟姘歌繙涓嶄細鐫＄湢銆?

## 鍏稿瀷鐢ㄦ硶


```
	#include <linux/hwspinlock.h>
	#include <linux/err.h>

	int hwspinlock_example(void)
	{
		struct hwspinlock *hwlock;
		int ret;

		/*
		* assign a specific hwspinlock id - this should be called early
		* by board init code.
		*/
		hwlock = hwspin_lock_request_specific(PREDEFINED_LOCK_ID);
		if (!hwlock)
			...

		/* try to take it, but don't spin on it */
		ret = hwspin_trylock(hwlock);
		if (!ret) {
			pr_info("lock is already taken\n");
			return -EBUSY;
		}

		/*
		* we took the lock, do our thing now, but do NOT sleep
		*/

		/* release the lock */
		hwspin_unlock(hwlock);

		/* free the lock */
		ret = hwspin_lock_free(hwlock);
		if (ret)
			...

		return ret;
	}
```
## 闈㈠悜瀹炵幇鑰呯殑 API


```
  int hwspin_lock_register(struct hwspinlock_device *bank, struct device *dev,
		const struct hwspinlock_ops *ops, int base_id, int num_locks);
```
鐢卞簳灞傜殑骞冲彴鐗瑰畾瀹炵幇璋冪敤锛屼互娉ㄥ唽涓€涓柊鐨?hwspinlock 璁惧锛堥€氬父鏄竴缁勬暟閲忎紬澶氱殑閿侊級銆傚簲浠庤繘绋嬩笂涓嬫枃璋冪敤锛堣鍑芥暟鍙兘鐫＄湢锛夈€?

鎴愬姛鏃惰繑鍥?0锛屽け璐ユ椂杩斿洖鐩稿簲鐨勯敊璇爜銆?

```
  int hwspin_lock_unregister(struct hwspinlock_device *bank);
```
鐢卞簳灞傜殑鍘傚晢鐗瑰畾瀹炵幇璋冪敤锛屼互娉ㄩ攢涓€涓?hwspinlock 璁惧锛堥€氬父鏄竴缁勬暟閲忎紬澶氱殑閿侊級銆?

搴斾粠杩涚▼涓婁笅鏂囪皟鐢紙璇ュ嚱鏁板彲鑳界潯鐪狅級銆?

鎴愬姛鏃惰繑鍥?hwspinlock 鐨勫湴鍧€锛岄敊璇椂杩斿洖 NULL锛堜緥濡傦紝鑻ヨ hwspinlock 浠嶅湪浣跨敤涓級銆?

## 閲嶈缁撴瀯浣?


struct hwspinlock_device 鏄竴涓€氬父鍖呭惈涓€缁勭‖浠堕攣鐨勮澶囥€傚畠鐢卞簳灞傜殑 hwspinlock 瀹炵幇閫氳繃 hwspin_lock_register() API 娉ㄥ唽銆?

```
	/**
	* struct hwspinlock_device - a device which usually spans numerous hwspinlocks
	* @dev: underlying device, will be used to invoke runtime PM api
	* @ops: platform-specific hwspinlock handlers
	* @base_id: id index of the first lock in this device
	* @num_locks: number of locks in this device
	* @lock: dynamically allocated array of 'struct hwspinlock'
	*/
	struct hwspinlock_device {
		struct device *dev;
		const struct hwspinlock_ops *ops;
		int base_id;
		int num_locks;
		struct hwspinlock lock[0];
	};
```
struct hwspinlock_device 鍖呭惈涓€涓?hwspinlock 缁撴瀯浣撴暟缁勶紝姣忎釜
```
	/**
	* struct hwspinlock - this struct represents a single hwspinlock instance
	* @bank: the hwspinlock_device structure which owns this lock
	* @lock: initialized and used by hwspinlock core
	* @priv: private data, owned by the underlying platform-specific hwspinlock drv
	*/
	struct hwspinlock {
		struct hwspinlock_device *bank;
		spinlock_t lock;
		void *priv;
	};
```
娉ㄥ唽涓€缁勯攣鏃讹紝hwspinlock 椹卞姩鍙渶瑕佽缃悇閿佺殑 priv 鎴愬憳銆傚叾浣欐垚鍛樼敱 hwspinlock 鏍稿績鑷韩璁剧疆骞跺垵濮嬪寲銆?

## 瀹炵幇鍥炶皟


```
	struct hwspinlock_ops {
		int (*trylock)(struct hwspinlock *lock);
		void (*unlock)(struct hwspinlock *lock);
		void (*relax)(struct hwspinlock *lock);
	};
```
鍓嶄袱涓洖璋冩槸寮哄埗鐨勶細

->trylock() 鍥炶皟搴斿皾璇曚竴娆¤幏鍙栭攣锛屽け璐ユ椂杩斿洖 0锛屾垚鍔熸椂杩斿洖 1銆傝鍥炶皟**涓嶅緱**鐫＄湢銆?

->unlock() 鍥炶皟閲婃斁閿併€傚畠鎬绘槸鎴愬姛锛屽苟涓斿悓鏍?*涓嶅緱**鐫＄湢銆?

->relax() 鍥炶皟鏄彲閫夌殑銆傚綋 hwspinlock 鏍稿績鍦ㄦ煇鎶婇攣涓婅嚜鏃嬫椂浼氳璋冪敤锛屽簳灞傜殑瀹炵幇鍙互鐢ㄥ畠鏉ュ己鍒跺湪涓ゆ杩炵画鐨?->trylock() 璋冪敤涔嬮棿鎻掑叆寤惰繜銆傚畠**涓嶅緱**鐫＄湢銆?
