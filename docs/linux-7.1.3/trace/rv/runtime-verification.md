## 杩愯鏃堕獙璇侊紙Runtime Verification锛?

杩愯鏃堕獙璇侊紙Runtime Verification锛孯V锛夋槸涓€绉嶈交閲忥紙浣嗕弗璋級鐨勬柟娉曪紝瀹冪敤涓€绉嶅澶嶆潅绯荤粺鏇村疄鐢ㄧ殑鏂瑰紡锛?琛ュ厖浜嗙粡鍏哥殑绌峰敖寮忛獙璇佹妧鏈紙濡?妯″瀷妫€娴嬶紙model checking锛?鍜?*瀹氱悊璇佹槑锛坱heorem proving锛?*锛夈€?
RV 涓嶄緷璧栦簬绯荤粺鐨勭粏绮掑害妯″瀷锛堜緥濡傚湪鎸囦护绾ч噸鏂板疄鐜帮級锛岃€屾槸閫氳繃鍒嗘瀽涓庣郴缁熻涓虹殑褰㈠紡鍖栬鑼冿紙formal specification锛?鐩稿姣旂殑绯荤粺瀹為檯鎵ц鐨勮抗锛坱race锛夋潵宸ヤ綔銆?
鍏朵富瑕佷紭鍔垮湪浜庯紝RV 鑳藉鎻愪緵琚洃瑙嗙郴缁熻繍琛屾椂琛屼负鐨勭簿纭俊鎭紝鑰岄伩鍏嶄簡寮€鍙戦渶瑕佷互寤烘ā璇█閲嶆柊瀹炵幇鏁翠釜绯荤粺鐨?妯″瀷鎵€甯︽潵鐨勯櫡闃便€傛澶栵紝缁欏畾涓€绉嶉珮鏁堢殑鐩戣鏂规硶锛屽氨鏈夊彲鑳芥墽琛岀郴缁熺殑**鍦ㄧ嚎锛坥nline锛?*楠岃瘉锛?浠庤€屽鎰忓浜嬩欢浣滃嚭**鍙嶅簲锛坮eaction锛?*锛屼緥濡傞伩鍏嶆晠闅滃湪瀹夊叏鍏抽敭锛坰afety-critical锛夌郴缁熶笂鐨勪紶鎾€?
## 杩愯鏃剁洃瑙嗗櫒涓庡弽搴斿櫒


鐩戣鍣紙monitor锛夋槸绯荤粺杩愯鏃堕獙璇佺殑鏍稿績閮ㄥ垎銆傜洃瑙嗗櫒澶勪簬鏈熸湜锛堟垨涓嶆湡鏈涳級琛屼负鐨勫舰寮忓寲瑙勮寖涓庡疄闄呯郴缁熺殑杩逛箣闂淬€?
鐢?Linux 鐨勬湳璇潵璇达紝杩愯鏃堕獙璇佺洃瑙嗗櫒琚皝瑁呭湪 **RV monitor** 鎶借薄涔嬩腑銆備竴涓?**RV monitor** 鍖呭惈涓€涓郴缁熺殑
鍙傝€冩ā鍨嬶紙reference model锛夈€佷竴缁勭洃瑙嗗櫒瀹炰緥锛堜緥濡傛瘡 CPU 鐩戣鍣ㄣ€佹瘡浠诲姟鐩戣鍣ㄧ瓑绛夛級锛屼互鍙婇€氳繃
浠ヤ笅鏂瑰紡灏嗙洃瑙嗗櫒涓庣郴缁熺矘鍚堝湪涓€璧风殑杈呭姪鍑芥暟锛?
```
 Linux   +---- RV Monitor ----------------------------------+ Formal
  Realm  |                                                  |  Realm
  +-------------------+     +----------------+     +-----------------+
  |   Linux kernel    |     |     Monitor    |     |     Reference   |
  |     Tracing       |  -> |   Instance(s)  | <-  |       Model     |
  | (instrumentation) |     | (verification) |     | (specification) |
  +-------------------+     +----------------+     +-----------------+
         |                          |                       |
         |                          V                       |
         |                     +----------+                 |
         |                     | Reaction |                 |
         |                     +--+--+--+-+                 |
         |                        |  |  |                   |
         |                        |  |  +-> trace output ?  |
         +------------------------|--|----------------------+
                                  |  +----> panic ?
                                  +-------> <user-specified>
```
闄や簡瀵圭郴缁熻繘琛岄獙璇佸拰鐩戣澶栵紝鐩戣鍣ㄨ繕鍙互瀵规剰澶栦簨浠朵綔鍑哄弽搴斻€傚弽搴旂殑褰㈠紡鍙互澶氱澶氭牱锛屼粠璁板綍
浜嬩欢鍙戠敓鐨勬棩蹇楋紝鍒板己鍒舵纭涓猴紝鍐嶅埌鏋佺鐨勫叧闂郴缁熶互閬垮厤鏁呴殰浼犳挱銆?
鐢?Linux 鐨勬湳璇潵璇达紝**reactor锛堝弽搴斿櫒锛?* 鏄竴绉嶄緵 **RV monitor** 浣跨敤鐨勫弽搴旀柟娉曘€?榛樿鎯呭喌涓嬶紝鎵€鏈夌洃瑙嗗櫒閮藉簲鎻愪緵鍏跺姩浣滅殑 trace 杈撳嚭锛岃繖鏈韩宸茬粡鏄竴绉嶅弽搴斻€傛澶栵紝杩樹細鎻愪緵鍏朵粬鍙嶅簲锛?浠ヤ究鐢ㄦ埛鏍规嵁闇€瑕佸惎鐢ㄥ畠浠€?
鍏充簬杩愯鏃堕獙璇佸師鐞嗕互鍙婂簲鐢ㄤ簬 Linux 鐨?RV 鐨勬洿澶氫俊鎭細

  Bartocci, Ezio, et al. **Introduction to runtime verification.** In: Lectures on
  Runtime Verification. Springer, Cham, 2018. p. 1-33.

  Falcone, Ylies, et al. **A taxonomy for classifying runtime verification tools.**
  In: International Conference on Runtime Verification. Springer, Cham, 2018. p.
  241-262.

  De Oliveira, Daniel Bristot. *Automata-based formal analysis and
  verification of the real-time Linux kernel.* Ph.D. Thesis, 2020.

## 鍦ㄧ嚎 RV 鐩戣鍣?

鐩戣鍣ㄥ彲鍒嗕负 **offline锛堢绾匡級** 涓?**online锛堝湪绾匡級** 鐩戣鍣ㄣ€?*Offline**
鐩戣鍣ㄥ湪浜嬩欢鍙戠敓鍚庡鐞嗙郴缁熺敓鎴愮殑杩癸紝閫氬父鏄粠姘镐箙瀛樺偍绯荤粺璇诲彇杩规墽琛屻€?*Online** 鐩戣鍣?鍦ㄧ郴缁熸墽琛屾湡闂村鐞嗚抗銆傚鏋滃湪浜嬩欢鐩戣鏈熼棿澶勭悊浜嬩欢闄勫姞浜庣郴缁熸墽琛屻€佸苟鍦ㄤ簨浠剁洃瑙嗘湡闂撮樆濉炵郴缁燂紝
鍒欏湪绾跨洃瑙嗗櫒琚О涓?*鍚屾锛坰ynchronous锛?*銆傚彟涓€鏂归潰锛?*寮傛锛坅synchronous锛?* 鐩戣鍣ㄧ殑鎵ц涓庣郴缁熺浉鍒嗙銆?姣忕绫诲瀷鐨勭洃瑙嗗櫒閮芥湁涓€绯诲垪浼樼偣銆備緥濡傦紝**offline** 鐩戣鍣ㄥ彲浠ュ湪涓嶅悓鐨勬満鍣ㄤ笂鎵ц锛屼絾闇€瑕佸皢鏃ュ織淇濆瓨鍒?鏂囦欢鐨勬搷浣溿€傜浉姣斾箣涓嬶紝**synchronous online锛堝悓姝ュ湪绾匡級** 鏂规硶鍙互鍦ㄨ繚瑙勫彂鐢熺殑纭垏鏃跺埢浣滃嚭鍙嶅簲銆?
鍏充簬鐩戣鍣ㄧ殑鍙︿竴涓噸瑕佹柟闈㈡槸涓庝簨浠跺垎鏋愮浉鍏崇殑寮€閿€銆傚鏋滅郴缁熺敓鎴愪簨浠剁殑棰戠巼楂樹簬鐩戣鍣ㄥ湪鍚屼竴绯荤粺涓?澶勭悊瀹冧滑鐨勮兘鍔涳紝鍒欏彧鏈?**offline** 鏂规硶鏄彲琛岀殑銆傚彟涓€鏂归潰锛屽鏋滀簨浠惰拷韪甫鏉ョ殑寮€閿€楂樹簬鐩戣鍣ㄥ鍗曚釜浜嬩欢鐨?绠€鍗曞鐞嗭紝閭ｄ箞 **synchronous online** 鐩戣鍣ㄥ皢甯︽潵鏇翠綆鐨勫紑閿€銆?
浜嬪疄涓婏紝浠ヤ笅鐮旂┒鎵€鍛堢幇鐨勫唴瀹癸細

  De Oliveira, Daniel Bristot; Cucinotta, Tommaso; De Oliveira, Romulo Silva.
  **Efficient formal verification for the Linux kernel.** In: International
  Conference on Software Engineering and Formal Methods. Springer, Cham, 2019.
  p. 315-332.

琛ㄦ槑锛屽浜庣‘瀹氭€ц嚜鍔ㄦ満锛圖eterministic Automata锛夋ā鍨嬶紝鍦ㄥ唴鏍镐腑鍚屾澶勭悊浜嬩欢閫犳垚鐨勫紑閿€浣庝簬灏嗙浉鍚屼簨浠?淇濆瓨鍒拌抗缂撳啿鍖猴紝鐢氳嚦杩樻病绠椾笂涓虹敤鎴风┖闂村垎鏋愭敹闆嗚抗鐨勫紑閿€銆傝繖鎺ㄥ姩浜嗗唴鏍稿唴鎺ュ彛锛坕n-kernel interface锛夊湪绾跨洃瑙嗗櫒鐨勫紑鍙戙€?
鍏充簬浣跨敤鑷姩鏈哄 Linux 鍐呮牳琛屼负杩涜寤烘ā鐨勬洿澶氫俊鎭紝鍙傝锛?
  De Oliveira, Daniel B.; De Oliveira, Romulo S.; Cucinotta, Tommaso. *A thread
  synchronization model for the PREEMPT_RT Linux kernel.* Journal of Systems
  Architecture, 2020, 107: 101729.

## 鐢ㄦ埛鎺ュ彛


鐢ㄦ埛鎺ュ彛锛堟槸鏁呮剰锛夌被浼间簬杩借釜锛坱racing锛夋帴鍙ｃ€傚畠褰撳墠浣嶄簬 "/sys/kernel/tracing/rv/"銆?
褰撳墠鍙敤鐨勬枃浠?鏂囦欢澶瑰涓嬶細

**available_monitors**

- 璇诲彇瀹冧細閫愯鍒楀嚭鍙敤鐨勭洃瑙嗗櫒

```
   # cat available_monitors
   wip
   wwnr
```

**available_reactors**

- 璇诲彇瀹冧細閫愯鏄剧ず鍙敤鐨勫弽搴斿櫒銆?
```
   # cat available_reactors
   nop
   panic
   printk
```

**enabled_monitors**锛?
- 璇诲彇瀹冧細鍒楀嚭宸插惎鐢ㄧ殑鐩戣鍣紝姣忚涓€涓?- 鍐欏叆瀹冧細鍚敤缁欏畾鐨勭洃瑙嗗櫒
- 鍐欏叆甯?'!' 鍓嶇紑鐨勭洃瑙嗗櫒鍚嶇О浼氱鐢ㄥ畠
- 鎴柇璇ユ枃浠朵細绂佺敤鎵€鏈夊凡鍚敤鐨勭洃瑙嗗櫒

```
   # cat enabled_monitors
   # echo wip > enabled_monitors
   # echo wwnr >> enabled_monitors
   # cat enabled_monitors
   wip
   wwnr
   # echo '!wip' >> enabled_monitors
   # cat enabled_monitors
   wwnr
   # echo > enabled_monitors
   # cat enabled_monitors
   #
```

娉ㄦ剰锛屽彲浠ュ悓鏃跺惎鐢ㄥ涓洃瑙嗗櫒銆?
**monitoring_on**

杩欐槸涓€涓敤浜庣洃瑙嗙殑寮€鍏冲紡鎬诲紑鍏炽€傚畠绫讳技浜?trace 鎺ュ彛涓殑
"tracing_on" 寮€鍏炽€?
- 鍐欏叆 "0" 浼氬仠姝㈢洃瑙?- 鍐欏叆 "1" 浼氱户缁洃瑙?- 璇诲彇瀹冧細杩斿洖鐩戣鐨勫綋鍓嶇姸鎬?
娉ㄦ剰锛屽畠涓嶄細绂佺敤宸插惎鐢ㄧ殑鐩戣鍣紝鑰屾槸鍋滄鐩戣浠庣郴缁熸帴鏀朵簨浠剁殑姣忓疄浣擄紙per-entity锛夌洃瑙嗗櫒銆?
**reacting_on**

- 鍐欏叆 "0" 浼氶樆姝㈠弽搴斿彂鐢?- 鍐欏叆 "1" 浼氬惎鐢ㄥ弽搴?- 璇诲彇瀹冧細杩斿洖鍙嶅簲鐨勫綋鍓嶇姸鎬?
**monitors/**

姣忎釜鐩戣鍣ㄥ湪 "monitors/" 鍐呬細鏈夎嚜宸辩殑鐩綍銆傞偅閲屼細灞曠ず鐩戣鍣ㄧ壒瀹氱殑鏂囦欢銆?monitors/" 鐩綍绫讳技浜?tracefs 涓婄殑 "events" 鐩綍銆?
```
   # cd monitors/wip/
   # ls
   desc  enable
   # cat desc
   wakeup in preemptive per-cpu testing monitor.
   # cat enable
   0
```

**monitors/MONITOR/desc**

- 璇诲彇瀹冧細鏄剧ず鐩戣鍣?**MONITOR** 鐨勬弿杩?
**monitors/MONITOR/enable**

- 鍐欏叆 "0" 浼氱鐢?**MONITOR**
- 鍐欏叆 "1" 浼氬惎鐢?**MONITOR**
- 璇诲彇瀹冧細杩斿洖 **MONITOR** 鐨勫綋鍓嶇姸鎬?
**monitors/MONITOR/reactors**

- 鍒楀嚭鍙敤鐨勫弽搴斿櫒锛岀粰瀹?**MONITOR** 鐨勬墍閫夊弽搴斾綅浜?"[]" 鍐呫€傞粯璁ょ殑鏄?nop锛堟棤鎿嶄綔锛夊弽搴斿櫒銆?- 鍐欏叆鍙嶅簲鍣ㄧ殑鍚嶇О浼氬皢鍏跺惎鐢ㄥ埌缁欏畾鐨?MONITOR銆?
```
   # cat monitors/wip/reactors
   [nop]
   panic
   printk
   # echo panic > monitors/wip/reactors
   # cat monitors/wip/reactors
   nop
   [panic]
   printk
```
