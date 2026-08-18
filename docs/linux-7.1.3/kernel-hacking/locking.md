
## 涓嶅彲闈犵殑鍐呮牳閿佹寚鍗?


:Author: Rusty Russell

## 寮曡█


娆㈣繋闃呰 Rusty 缂栧啓銆佺浉褰撲笉鍙潬鐨勩€婂唴鏍搁攣闂鎸囧崡銆嬨€傛湰鏂囨。鎻忚堪浜?
Linux 2.6 鍐呮牳涓殑閿佹満鍒躲€?

闅忕潃瓒呯嚎绋嬶紙HyperThreading锛夌殑骞挎硾鏅強锛屼互鍙?Linux 鍐呮牳涓姠鍗狅紙preemption锛夌殑寮曞叆锛?
姣忎竴浣嶅湪鍐呮牳涓婂仛寮€鍙戠殑浜洪兘闇€瑕佷簡瑙?SMP 涓嬪苟鍙戜笌閿佺殑鍩烘湰鍘熺悊銆?

## 骞跺彂鐨勯棶棰?


锛堝鏋滀綘宸茬粡鐭ラ亾浠€涔堟槸绔炴€佹潯浠讹紝鍙互璺宠繃鏈妭銆傦級

鍦ㄤ竴涓櫘閫氱▼搴忎腑锛屼綘鍙互鍍忎笅闈㈣繖鏍烽€掑涓€涓鏁板櫒锛?

```

          very_important_count++;


```
杩欐槸浜轰滑鏈熸湜鍙戠敓鐨勬儏鍐碉細


  +------------------------------------+------------------------------------+
  | Instance 1                         | Instance 2                         |
  +====================================+====================================+
  | read very_important_count (5)      |                                    |
  +------------------------------------+------------------------------------+
  | add 1 (6)                          |                                    |
  +------------------------------------+------------------------------------+
  | write very_important_count (6)     |                                    |
  +------------------------------------+------------------------------------+
  |                                    | read very_important_count (6)      |
  +------------------------------------+------------------------------------+
  |                                    | add 1 (7)                          |
  +------------------------------------+------------------------------------+
  |                                    | write very_important_count (7)     |
  +------------------------------------+------------------------------------+

杩欐槸瀹為檯鍙兘鍙戠敓鐨勬儏鍐碉細


  +------------------------------------+------------------------------------+
  | Instance 1                         | Instance 2                         |
  +====================================+====================================+
  | read very_important_count (5)      |                                    |
  +------------------------------------+------------------------------------+
  |                                    | read very_important_count (5)      |
  +------------------------------------+------------------------------------+
  | add 1 (6)                          |                                    |
  +------------------------------------+------------------------------------+
  |                                    | add 1 (6)                          |
  +------------------------------------+------------------------------------+
  | write very_important_count (6)     |                                    |
  +------------------------------------+------------------------------------+
  |                                    | write very_important_count (6)     |
  +------------------------------------+------------------------------------+


### 绔炴€佹潯浠朵笌涓寸晫鍖?


涓婅堪杩欑閲嶅彔锛屽叾鏈€缁堢粨鏋滀緷璧栦簬澶氫釜浠诲姟鐨勭浉瀵规椂搴忥紝琚О涓虹珵鎬佹潯浠?
锛坮ace condition锛夈€傚寘鍚苟鍙戦棶棰樼殑閭ｆ浠ｇ爜绉颁负涓寸晫鍖猴紙critical region锛夈€?
灏ゅ叾鏄嚜浠?Linux 寮€濮嬭繍琛屽湪 SMP 鏈哄櫒涓婁互鏉ワ紝瀹冧滑鎴愪簡鍐呮牳璁捐涓庡疄鐜颁腑鐨?
涓昏闂涔嬩竴銆?

鍗充究鍙湁涓€涓?CPU锛屾姠鍗犱篃浼氫骇鐢熷悓鏍风殑鏁堟灉锛氬鏋滃湪涓寸晫鍖哄唴鎶㈠崰浜嗕竴涓换鍔★紝
鎴戜滑灏卞緱鍒颁簡瀹屽叏鐩稿悓鐨勭珵鎬佹潯浠躲€傚湪杩欑鎯呭喌涓嬶紝鎶㈠崰鎴戜滑鐨勯偅涓嚎绋嬪彲鑳?
鑷繁灏辫繍琛屼簡涓寸晫鍖恒€?

瑙ｅ喅鍔炴硶鏄瘑鍒繖浜涘悓鏃跺彂鐢熺殑璁块棶锛屽苟浣跨敤閿佹潵纭繚浠绘剰鏃跺埢鍙湁涓€涓疄渚嬭兘澶?
杩涘叆涓寸晫鍖恒€侺inux 鍐呮牳涓湁璁稿鍙嬪ソ鐨勫師璇彲浠ュ府鍔╀綘鍋氬埌杩欎竴鐐广€傚綋鐒朵篃
鏈変竴浜涗笉閭ｄ箞鍙嬪ソ鐨勫師璇紝涓嶈繃鎴戜細鍋囪瀹冧滑涓嶅瓨鍦ㄣ€?

## Linux 鍐呮牳涓殑閿?


濡傛灉鍏充簬閿佹垜鍙兘缁欎綘涓€鏉″缓璁紝閭ｅ氨鏄細**淇濇寔绠€鍗?*銆?

涓嶈闅忔剰寮曞叆鏂扮殑閿併€?

### 鍐呮牳閿佺殑涓ゅぇ绫诲瀷锛氳嚜鏃嬮攣涓庝簰鏂ヤ綋


鍐呮牳閿佷富瑕佹湁涓ょ绫诲瀷銆傛渶鍩烘湰鐨勬槸鑷棆閿侊紙`include/asm/spinlock.h`锛夛紝
瀹冩槸涓€绉嶉潪甯哥畝鍗曠殑鍗曚汉鎸佹湁閿侊細濡傛灉浣犳嬁涓嶅埌鑷棆閿侊紝灏变細涓€鐩村皾璇曪紙鑷棆锛?
鐩村埌鎷垮埌涓烘銆傝嚜鏃嬮攣闈炲父灏忓阀涓斿揩閫燂紝鍙湪浠讳綍鍦版柟浣跨敤銆?

绗簩绉嶆槸浜掓枼浣擄紙`include/linux/mutex.h`锛夛細瀹冨緢鍍忚嚜鏃嬮攣锛屼絾鎸佹湁浜掓枼浣撴椂
浣犲彲浠ョ潯鐪犮€傚鏋滄嬁涓嶅埌浜掓枼浣擄紝浣犵殑浠诲姟浼氭寕璧疯嚜宸憋紝骞跺湪浜掓枼浣撹閲婃斁鏃惰鍞ら啋銆?
杩欐剰鍛崇潃鍦ㄤ綘绛夊緟鏈熼棿 CPU 鍙互鍘诲仛鍒殑浜嬫儏銆傚緢澶氭椂鍊欎綘鏍规湰鏃犳硶鐫＄湢
锛堝弬瑙?`What Functions Are Safe To Call From Interrupts?`_锛夛紝
鍥犳涓嶅緱涓嶆敼鐢ㄨ嚜鏃嬮攣銆?

杩欎袱绉嶉攣閮戒笉鏄彲閲嶅叆鐨勶細鍙傝 `Deadlock: Simple and Advanced`_銆?

### 閿佷笌鍗曞鐞嗗櫒鍐呮牳


瀵逛簬娌℃湁寮€鍚?`CONFIG_SMP`銆佷篃娌℃湁寮€鍚?`CONFIG_PREEMPT` 鑰岀紪璇戠殑鍐呮牳锛?
鑷棆閿佹牴鏈笉瀛樺湪銆傝繖鏄竴涓嚭鑹茬殑璁捐鍐崇瓥锛氬綋娌℃湁鍏朵粬浜鸿兘鍚屾椂杩愯鏃讹紝
灏辨病鏈夌悊鐢变娇鐢ㄩ攣銆?

濡傛灉鍐呮牳娌℃湁寮€鍚?`CONFIG_SMP`锛屼絾璁剧疆浜?`CONFIG_PREEMPT`锛岄偅涔堣嚜鏃嬮攣
浠呬粎鏄鐢ㄦ姠鍗狅紝杩欏凡瓒充互闃叉浠讳綍绔炴€併€傚湪澶у鏁版儏鍐典笅锛屾垜浠彲浠ユ妸鎶㈠崰
绛夊悓浜?SMP锛岃€屼笉蹇呭崟鐙€冭檻瀹冦€?

浣犲簲璇ュ缁堝湪寮€鍚?`CONFIG_SMP` 鍜?`CONFIG_PREEMPT` 鐨勬儏鍐典笅娴嬭瘯浣犵殑閿佷唬鐮侊紝
鍗充究浣犳墜澶存病鏈?SMP 娴嬭瘯鏈猴紝鍥犱负瀹冧粛鐒惰兘鎹曡幏鏌愪簺绫诲瀷鐨勫姞閿侀敊璇€?

浜掓枼浣撲緷鐒跺瓨鍦紝鍥犱负瀹冧滑鏄敤鎴蜂笂涓嬫枃涔嬮棿鍚屾鎵€蹇呴渶鐨勶紝姝ｅ鎴戜滑涓嬮潰
灏嗙湅鍒扮殑閭ｆ牱銆?

### 浠呭湪鐢ㄦ埛涓婁笅鏂囦腑鍔犻攣


濡傛灉浣犳湁涓€涓暟鎹粨鏋勶紝鍙細鍦ㄧ敤鎴蜂笂涓嬫枃涓璁块棶锛岄偅涔堜綘鍙互浣跨敤涓€涓畝鍗曠殑
浜掓枼浣擄紙`include/linux/mutex.h`锛夋潵淇濇姢瀹冦€傝繖鏄渶骞冲嚒鐨勬儏褰細浣犲垵濮嬪寲
浜掓枼浣撱€傜劧鍚庝綘鍙互璋冪敤 mutex_lock_interruptible() 鏉ヨ幏鍙栦簰鏂ヤ綋锛?
璋冪敤 mutex_unlock() 鏉ラ噴鏀惧畠銆傝繕鏈変竴涓?mutex_lock()锛?
搴旇閬垮厤浣跨敤瀹冿紝鍥犱负涓€鏃︽敹鍒颁俊鍙峰畠灏变笉浼氳繑鍥炪€?

绀轰緥锛歚net/netfilter/nf_sockopt.c` 鍏佽娉ㄥ唽鏂扮殑 setsockopt() 鍜?
getsockopt() 璋冪敤锛岄€氳繃 nf_register_sockopt()銆傛敞鍐屼笌娉ㄩ攢
鍙湪妯″潡鍔犺浇鍜屽嵏杞芥椂锛堜互鍙婂惎鍔ㄦ椂锛岄偅鏃舵病鏈夊苟鍙戯級杩涜锛岃€屾敞鍐屽垪琛ㄥ彧鍦?
閬囧埌鏈煡鐨?setsockopt() 鎴?getsockopt() 绯荤粺璋冪敤鏃舵墠琚煡闃呫€?
`nf_sockopt_mutex` 闈炲父閫傚悎鐢ㄦ潵淇濇姢瀹冿紝灏ゅ叾鏄洜涓?setsockopt 鍜?
getsockopt 璋冪敤寰堝彲鑳戒細鐫＄湢銆?

### 鐢ㄦ埛涓婁笅鏂囦笌杞腑鏂箣闂寸殑鍔犻攣


濡傛灉杞腑鏂笌鐢ㄦ埛涓婁笅鏂囧叡浜暟鎹紝浣犱細闈复涓や釜闂銆傞鍏堬紝褰撳墠鐨勭敤鎴蜂笂涓嬫枃
鍙兘琚蒋涓柇鎵撴柇锛涘叾娆★紝涓寸晫鍖轰篃鍙兘浠庡彟涓€涓?CPU 杩涘叆銆傝繖鏃跺氨瑕佺敤鍒?
spin_lock_bh()锛坄include/linux/spinlock.h`锛夈€傚畠鍏堝湪璇?CPU 涓?
绂佺敤杞腑鏂紝鐒跺悗鍐嶈幏鍙栭攣銆俿pin_unlock_bh() 鍋氱浉鍙嶇殑浜嬫儏銆?
锛?_bh' 鍚庣紑鏄鈥滃簳鍗婇儴鈥濓紙Bottom Halves锛岃蒋浠朵腑鏂殑鏃хО锛夌殑鍘嗗彶鎸囩О銆?
鍦ㄧ悊鎯充笘鐣岄噷瀹冨叾瀹炲簲璇ュ彨 spin_lock_softirq()銆傦級

娉ㄦ剰锛岃繖閲屼綘涔熷彲浠ヤ娇鐢?spin_lock_irq() 鎴?spin_lock_irqsave()锛?
瀹冧滑鍚屾椂浼氬仠姝㈢‖浠朵腑鏂細鍙傝 `Hard IRQ Context`_銆?

杩欏浜?UP 鍚屾牱瀹岀編閫傜敤锛氳嚜鏃嬮攣娑堝け锛岃繖涓畯绠€鍗曞湴鍙樻垚 local_bh_disable()
锛坄include/linux/interrupt.h`锛夛紝瀹冧繚鎶や綘涓嶈杞腑鏂繍琛屻€?

### 鐢ㄦ埛涓婁笅鏂囦笌 Tasklet 涔嬮棿鐨勫姞閿?


杩欎笌涓婇潰瀹屽叏鐩稿悓锛屽洜涓?tasklet 瀹為檯涓婃槸浠庤蒋涓柇涓繍琛岀殑銆?

### 鐢ㄦ埛涓婁笅鏂囦笌瀹氭椂鍣ㄤ箣闂寸殑鍔犻攣


杩欎篃涓庝笂闈㈠畬鍏ㄧ浉鍚岋紝鍥犱负瀹氭椂鍣ㄥ疄闄呬笂鏄粠杞腑鏂腑杩愯鐨勩€備粠鍔犻攣鐨勮搴︾湅锛?
tasklet 鍜屽畾鏃跺櫒鏄畬鍏ㄧ浉鍚岀殑銆?

### Tasklet/瀹氭椂鍣ㄤ箣闂寸殑鍔犻攣


鏈夋椂涓€涓?tasklet 鎴栧畾鏃跺櫒鍙兘鎯宠涓庡彟涓€涓?tasklet 鎴栧畾鏃跺櫒鍏变韩鏁版嵁銆?

#### 鍚屼竴涓?Tasklet/瀹氭椂鍣?


鐢变簬 tasklet 缁濅笉浼氬悓鏃跺湪涓や釜 CPU 涓婅繍琛岋紝浣犱笉蹇呮媴蹇冧綘鐨?tasklet 浼氳
閲嶅叆锛堝悓鏃惰繍琛屼袱娆★級锛屽嵆浣垮湪 SMP 涓婁篃鏄姝ゃ€?

#### 涓嶅悓鐨?Tasklet/瀹氭椂鍣?


濡傛灉鍙︿竴涓?tasklet/timer 鎯宠涓庝綘鐨?tasklet 鎴栧畾鏃跺櫒鍏变韩鏁版嵁锛屼綘浠?
浜岃€呴兘闇€瑕佷娇鐢?spin_lock() 鍜?spin_unlock() 璋冪敤銆?
spin_lock_bh() 鍦ㄨ繖閲屾槸涓嶅繀瑕佺殑锛屽洜涓轰綘宸茬粡澶勪簬涓€涓?tasklet 涓紝
鍚屼竴涓?CPU 涓婁笉浼氭湁鍏朵粬 tasklet 杩愯銆?

### 杞腑鏂箣闂寸殑鍔犻攣


杞腑鏂粡甯告兂瑕佷笌鑷韩鎴?tasklet/timer 鍏变韩鏁版嵁銆?

#### 鍚屼竴涓蒋涓柇


鍚屼竴涓蒋涓柇鍙互鍦ㄥ叾浠?CPU 涓婅繍琛岋細浣犲彲浠ヤ娇鐢ㄦ瘡-CPU 鏁扮粍
锛堝弬瑙?`Per-CPU Data`_锛夋潵鑾峰緱鏇村ソ鐨勬€ц兘銆傚鏋滀綘閮界敤鍒拌蒋涓柇杩欑绋嬪害浜嗭紝
浣犲ぇ姒傝冻澶熷叧蹇冨彲鎵╁睍鎬ц兘锛屼粠鑰屾効鎰忔壙鍙楅澶栫殑澶嶆潅搴︺€?

浣犻渶瑕佸鍏变韩鏁版嵁浣跨敤 spin_lock() 鍜?spin_unlock()銆?

#### 涓嶅悓鐨勮蒋涓柇


浣犻渶瑕佸鍏变韩鏁版嵁浣跨敤 spin_lock() 鍜?spin_unlock()锛?
鏃犺鏄畾鏃跺櫒銆乼asklet銆佷笉鍚岀殑杞腑鏂紝杩樻槸鐩稿悓鎴栧叾浠栫殑杞腑鏂細瀹冧滑涓换浣曚竴涓?
閮藉彲鑳藉湪涓嶅悓鐨?CPU 涓婅繍琛屻€?

## 纭欢 IRQ 涓婁笅鏂?


纭欢涓柇閫氬父涓庝竴涓?tasklet 鎴栬蒋涓柇閫氫俊銆傝繖閫氬父娑夊強鎶婂伐浣滄斁鍏ヤ竴涓槦鍒楋紝
鐢辫蒋涓柇鍙栧嚭銆?

### 纭欢 IRQ 涓庤蒋涓柇/Tasklet 涔嬮棿鐨勫姞閿?


濡傛灉纭欢 irq 澶勭悊绋嬪簭涓庤蒋涓柇鍏变韩鏁版嵁锛屼綘鏈変袱涓【铏戙€傞鍏堬紝杞腑鏂鐞?
鍙兘琚‖浠朵腑鏂墦鏂紱鍏舵锛屼复鐣屽尯鍙兘琚彟涓€涓?CPU 涓婄殑纭欢涓柇杩涘叆銆?
杩欐椂灏辫鐢ㄥ埌 spin_lock_irq()銆傚畠琚畾涔変负鍏堝湪璇?CPU 涓婄鐢ㄤ腑鏂紝
鐒跺悗鍐嶈幏鍙栭攣銆俿pin_unlock_irq() 鍋氱浉鍙嶇殑浜嬫儏銆?

irq 澶勭悊绋嬪簭涓嶉渶瑕佷娇鐢?spin_lock_irq()锛屽洜涓鸿蒋涓柇鍦?irq 澶勭悊绋嬪簭
杩愯鏃朵笉鍙兘杩愯锛氬畠鍙互浣跨敤 spin_lock()锛岃繖鏍蜂細绋嶅揩涓€浜涖€?
鍞竴鐨勪緥澶栨槸濡傛灉鍙︿竴涓笉鍚岀殑纭欢 irq 澶勭悊绋嬪簭浣跨敤浜嗗悓涓€鎶婇攣锛?
spin_lock_irq() 浼氶樆姝㈠畠鎵撴柇鎴戜滑銆?

杩欏浜?UP 鍚屾牱瀹岀編閫傜敤锛氳嚜鏃嬮攣娑堝け锛岃繖涓畯绠€鍗曞湴鍙樻垚 local_irq_disable()
锛坄include/asm/smp.h`锛夛紝瀹冧繚鎶や綘涓嶈杞腑鏂?tasklet/BH 杩愯銆?

spin_lock_irqsave()锛坄include/linux/spinlock.h`锛夋槸涓€涓彉浣擄紝
瀹冩妸涓柇鏄紑杩樻槸鍏充繚瀛樺湪涓€涓?flags 瀛椾腑锛岃瀛椾細琚紶缁?
spin_unlock_irqrestore()銆傝繖鎰忓懗鐫€鍚屾牱鐨勪唬鐮佹棦鍙互鐢ㄥ湪纭欢 irq 澶勭悊绋嬪簭
鍐呴儴锛堜腑鏂凡缁忓叧闂級锛屼篃鍙互鐢ㄥ湪杞腑鏂腑锛堥渶瑕佺鐢?irq锛夈€?

娉ㄦ剰锛岃蒋涓柇锛堝洜鑰屼篃鍖呮嫭 tasklet 鍜屽畾鏃跺櫒锛夋槸鍦ㄤ粠纭欢涓柇杩斿洖鏃惰繍琛岀殑锛?
鎵€浠?spin_lock_irq() 涔熶細鍋滄杩欎簺銆備粠杩欎釜鎰忎箟涓婅锛宻pin_lock_irqsave()
鏄渶閫氱敤銆佹渶寮哄ぇ鐨勫姞閿佸嚱鏁般€?

### 涓や釜纭欢 IRQ 澶勭悊绋嬪簭涔嬮棿鐨勫姞閿?


鍦ㄤ袱涓?IRQ 澶勭悊绋嬪簭涔嬮棿鍏变韩鏁版嵁鐨勬儏鍐靛緢灏戣锛屼絾濡傛灉纭疄闇€瑕侊紝搴旇浣跨敤
spin_lock_irqsave()锛氬湪 irq 澶勭悊绋嬪簭鑷韩鍐呴儴鏄惁绂佺敤鎵€鏈変腑鏂?
鏄緷璧栦綋绯荤粨鏋勭殑銆?

## 鍔犻攣閫熸煡琛?


Pete Zaitcev 缁欏嚭浜嗗涓嬫€荤粨锛?

- 濡傛灉浣犲浜庤繘绋嬩笂涓嬫枃锛堜换浣曠郴缁熻皟鐢級涓紝骞朵笖鎯宠鎶婂叾浠栬繘绋嬫帓闄ゅ湪澶栵紝
   浣跨敤浜掓枼浣撱€備綘鍙互鎸佹湁浜掓枼浣撳苟鐫＄湢
   锛坄copy_from_user()` 鎴?`kmalloc(x,GFP_KERNEL)`锛夈€?

- 鍚﹀垯锛?= 鏁版嵁鍙兘鍦ㄤ腑鏂腑琚Е鍙婏級锛屼娇鐢?spin_lock_irqsave() 鍜?
   spin_unlock_irqrestore()銆?

- 閬垮厤鎸佹湁鑷棆閿佽秴杩?5 琛屼唬鐮侊紝骞堕伩鍏嶈法瓒婁换浣曞嚱鏁拌皟鐢?
   锛坅ccessors 濡?readb() 闄ゅ锛夈€?

### 鏈€浣庤姹傝〃


涓嬭〃鍒楀嚭浜嗗悇绉嶄笂涓嬫枃涔嬮棿鐨?*鏈€浣?*鍔犻攣瑕佹眰銆傚湪鏌愪簺鎯呭喌涓嬶紝鍚屼竴涓婁笅鏂?
涓€娆″彧鑳藉湪涓€涓?CPU 涓婅繍琛岋紝鍥犳璇ヤ笂涓嬫枃涓嶉渶瑕佸姞閿侊紙渚嬪锛屾煇涓壒瀹氱嚎绋?
涓€娆″彧鑳藉湪涓€涓?CPU 涓婅繍琛岋紝浣嗗鏋滃畠闇€瑕佷笌鍙︿竴涓嚎绋嬪叡浜暟鎹紝灏遍渶瑕佸姞閿侊級銆?

璁颁綇涓婇潰鐨勫缓璁細浣犲缁堝彲浠ヤ娇鐢?spin_lock_irqsave()锛屽畠鏄墍鏈夊叾浠栬嚜鏃嬮攣
鍘熻鐨勮秴闆嗐€?

============== ============= ============= ========= ========= ========= ========= ======= ======= ============== ==============
.              IRQ Handler A IRQ Handler B Softirq A Softirq B Tasklet A Tasklet B Timer A Timer B User Context A User Context B
============== ============= ============= ========= ========= ========= ========= ======= ======= ============== ==============
IRQ Handler A  None
IRQ Handler B  SLIS          None
Softirq A      SLI           SLI           SL
Softirq B      SLI           SLI           SL        SL
Tasklet A      SLI           SLI           SL        SL        None
Tasklet B      SLI           SLI           SL        SL        SL        None
Timer A        SLI           SLI           SL        SL        SL        SL        None
Timer B        SLI           SLI           SL        SL        SL        SL        SL      None
User Context A SLI           SLI           SLBH      SLBH      SLBH      SLBH      SLBH    SLBH    None
User Context B SLI           SLI           SLBH      SLBH      SLBH      SLBH      SLBH    SLBH    MLI            None
============== ============= ============= ========= ========= ========= ========= ======= ======= ============== ==============

Table: Table of Locking Requirements

+--------+----------------------------+
| SLIS   | spin_lock_irqsave          |
+--------+----------------------------+
| SLI    | spin_lock_irq              |
+--------+----------------------------+
| SL     | spin_lock                  |
+--------+----------------------------+
| SLBH   | spin_lock_bh               |
+--------+----------------------------+
| MLI    | mutex_lock_interruptible   |
+--------+----------------------------+

Table: Legend for Locking Requirements Table

## trylock 鍑芥暟


鏈変竴浜涘嚱鏁板彧灏濊瘯鑾峰彇涓€娆￠攣锛屽苟绔嬪嵆杩斿洖涓€涓€艰〃绀鸿幏鍙栨垚鍔熸垨澶辫触銆傚鏋滀綘鍦?
鍏朵粬绾跨▼鎸佹湁閿佹椂涓嶉渶瑕佽闂璇ラ攣淇濇姢鐨勬暟鎹紝灏卞彲浠ヤ娇鐢ㄥ畠浠€傚鏋滀綘涔嬪悗闇€瑕?
璁块棶琚閿佷繚鎶ょ殑鏁版嵁锛屽簲璇ョ◢鍚庡啀鍘昏幏鍙栭攣銆?

spin_trylock() 涓嶄細鑷棆锛屽鏋滅涓€娆″皾璇曞氨鎷垮埌浜嗚嚜鏃嬮攣鍒欒繑鍥為潪闆讹紝
鍚﹀垯杩斿洖 0銆傝繖涓嚱鏁板彲浠ュ儚 spin_lock() 涓€鏍风敤浜庢墍鏈変笂涓嬫枃锛氫綘
蹇呴』宸茬粡绂佺敤浜嗗彲鑳芥墦鏂綘鐨勯偅浜涗笂涓嬫枃骞惰幏鍙栦簡鑷棆閿併€?

mutex_trylock() 涓嶄細鎸傝捣浣犵殑浠诲姟锛屽鏋滅涓€娆″皾璇曞氨鑳介攣瀹氫簰鏂ヤ綋鍒欒繑鍥為潪闆讹紝
鍚﹀垯杩斿洖 0銆傚敖绠″畠骞朵笉鐫＄湢锛屼絾杩欎釜鍑芥暟涓嶈兘鍦ㄧ‖浠舵垨杞欢涓柇涓婁笅鏂囦腑瀹夊叏浣跨敤銆?

## 甯歌绀轰緥


璁╂垜浠€愭鐪嬩竴涓畝鍗曠殑渚嬪瓙锛氫竴涓€滄暟瀛楀埌鍚嶇О鈥濇槧灏勭殑缂撳瓨銆傜紦瀛樿褰曚簡姣忎釜
瀵硅薄琚娇鐢ㄧ殑棰戠巼锛屽苟鍦ㄧ紦瀛樻弧鏃朵涪寮冧娇鐢ㄦ渶灏戠殑閭ｄ釜銆?

### 鍏ㄩ儴鍦ㄧ敤鎴蜂笂涓嬫枃涓?


鍦ㄦ垜浠殑绗竴涓緥瀛愪腑锛屾垜浠亣璁炬墍鏈夋搷浣滈兘鍦ㄧ敤鎴蜂笂涓嬫枃锛堝嵆鏉ヨ嚜绯荤粺璋冪敤锛変腑锛?
鍥犳鎴戜滑鍙互鐫＄湢銆傝繖鎰忓懗鐫€鎴戜滑鍙互浣跨敤浜掓枼浣撱€?

```
    #include <linux/list.h>
    #include <linux/slab.h>
    #include <linux/string.h>
    #include <linux/mutex.h>
    #include <asm/errno.h>

    struct object
    {
            struct list_head list;
            int id;
            char name[32];
            int popularity;
    };

    /* Protects the cache, cache_num, and the objects within it */
    static DEFINE_MUTEX(cache_lock);
    static LIST_HEAD(cache);
    static unsigned int cache_num = 0;
    #define MAX_CACHE_SIZE 10

    /* Must be holding cache_lock */
    static struct object *__cache_find(int id)
    {
            struct object *i;

            list_for_each_entry(i, &cache, list)
                    if (i->id == id) {
                            i->popularity++;
                            return i;
                    }
            return NULL;
    }

    /* Must be holding cache_lock */
    static void __cache_delete(struct object *obj)
    {
            BUG_ON(!obj);
            list_del(&obj->list);
            kfree(obj);
            cache_num--;
    }

    /* Must be holding cache_lock */
    static void __cache_add(struct object *obj)
    {
            list_add(&obj->list, &cache);
            if (++cache_num > MAX_CACHE_SIZE) {
                    struct object *i, *outcast = NULL;
                    list_for_each_entry(i, &cache, list) {
                            if (!outcast || i->popularity < outcast->popularity)
                                    outcast = i;
                    }
                    __cache_delete(outcast);
            }
    }

    int cache_add(int id, const char *name)
    {
            struct object *obj;

            if ((obj = kmalloc(sizeof(*obj), GFP_KERNEL)) == NULL)
                    return -ENOMEM;

            strscpy(obj->name, name, sizeof(obj->name));
            obj->id = id;
            obj->popularity = 0;

            mutex_lock(&cache_lock);
            __cache_add(obj);
            mutex_unlock(&cache_lock);
            return 0;
    }

    void cache_delete(int id)
    {
            mutex_lock(&cache_lock);
            __cache_delete(__cache_find(id));
            mutex_unlock(&cache_lock);
    }

    int cache_find(int id, char *name)
    {
            struct object *obj;
            int ret = -ENOENT;

            mutex_lock(&cache_lock);
            obj = __cache_find(id);
            if (obj) {
                    ret = 0;
                    strcpy(name, obj->name);
            }
            mutex_unlock(&cache_lock);
            return ret;
    }

```
娉ㄦ剰锛屾垜浠湪娣诲姞銆佸垹闄ゆ垨鏌ユ壘缂撳瓨鏃舵€绘槸纭繚鎸佹湁 cache_lock锛氱紦瀛樺熀纭€璁炬柦
鏈韩浠ュ強瀵硅薄鐨勫唴瀹归兘鐢辫繖鎶婇攣淇濇姢銆傚湪杩欑鎯呭喌涓嬭繖寰堝鏄擄紝鍥犱负鎴戜滑鎶婃暟鎹?
澶嶅埗缁欑敤鎴凤紝浠庝笉璁╀粬浠洿鎺ヨ闂璞°€?

杩欓噷鏈変竴涓粏寰紙涔熷緢甯歌锛夌殑浼樺寲锛氬湪 cache_add() 涓紝鎴戜滑鍦ㄨ幏鍙栭攣涔嬪墠
灏辫缃ソ浜嗗璞＄殑鍚勪釜瀛楁銆傝繖鏄畨鍏ㄧ殑锛屽洜涓哄湪鎶婂璞℃斁杩涚紦瀛樹箣鍓嶏紝娌℃湁鍏朵粬浜?
鑳借闂畠銆?

### 浠庝腑鏂笂涓嬫枃璁块棶


鐜板湪鑰冭檻 cache_find() 鍙兘浠庝腑鏂笂涓嬫枃琚皟鐢ㄧ殑鎯呭喌锛氳涔堟槸纭欢涓柇锛?
瑕佷箞鏄蒋涓柇銆備竴涓緥瀛愭槸鏌愪釜瀹氭椂鍣ㄤ細浠庣紦瀛樹腑鍒犻櫎瀵硅薄銆?

涓嬮潰鐨勬敼鍔ㄤ互鏍囧噯琛ヤ竵鏍煎紡灞曠ず锛歚-` 寮€澶寸殑琛屾槸琚垹闄ょ殑琛岋紝`+` 寮€澶寸殑琛屾槸
琚坊鍔犵殑琛屻€?

```
    --- cache.c.usercontext 2003-12-09 13:58:54.000000000 +1100
    +++ cache.c.interrupt   2003-12-09 14:07:49.000000000 +1100
    @@ -12,7 +12,7 @@
             int popularity;
     };

    -static DEFINE_MUTEX(cache_lock);
    +static DEFINE_SPINLOCK(cache_lock);
     static LIST_HEAD(cache);
     static unsigned int cache_num = 0;
     #define MAX_CACHE_SIZE 10
    @@ -55,6 +55,7 @@
     int cache_add(int id, const char *name)
     {
             struct object *obj;
    +        unsigned long flags;

             if ((obj = kmalloc(sizeof(*obj), GFP_KERNEL)) == NULL)
                     return -ENOMEM;
    @@ -63,30 +64,33 @@
             obj->id = id;
             obj->popularity = 0;

    -        mutex_lock(&cache_lock);
    +        spin_lock_irqsave(&cache_lock, flags);
             __cache_add(obj);
    -        mutex_unlock(&cache_lock);
    +        spin_unlock_irqrestore(&cache_lock, flags);
             return 0;
     }

     void cache_delete(int id)
     {
    -        mutex_lock(&cache_lock);
    +        unsigned long flags;
    +
    +        spin_lock_irqsave(&cache_lock, flags);
             __cache_delete(__cache_find(id));
    -        mutex_unlock(&cache_lock);
    +        spin_unlock_irqrestore(&cache_lock, flags);
     }

     int cache_find(int id, char *name)
     {
             struct object *obj;
             int ret = -ENOENT;
    +        unsigned long flags;

    -        mutex_lock(&cache_lock);
    +        spin_lock_irqsave(&cache_lock, flags);
             obj = __cache_find(id);
             if (obj) {
                     ret = 0;
                     strcpy(name, obj->name);
             }
    -        mutex_unlock(&cache_lock);
    +        spin_unlock_irqrestore(&cache_lock, flags);
             return ret;
     }

```
娉ㄦ剰锛宻pin_lock_irqsave() 浼氬湪涓柇寮€鍚椂鍏抽棴涓柇锛屽惁鍒欎粈涔堜篃涓嶅仛
锛堝鏋滄垜浠凡缁忓湪涓柇澶勭悊绋嬪簭涓級锛屽洜姝よ繖浜涘嚱鏁板彲浠ュ畨鍏ㄥ湴浠庝换浣曚笂涓嬫枃涓皟鐢ㄣ€?

閬楁喚鐨勬槸锛宑ache_add() 璋冪敤浜?kmalloc()锛屽苟甯︽湁 `GFP_KERNEL` 鏍囧織锛?
杩欏彧鍦ㄧ敤鎴蜂笂涓嬫枃涓悎娉曘€傛垜宸茬粡鍋囪 cache_add() 浠嶇劧鍙湪鐢ㄦ埛涓婁笅鏂囦腑琚皟鐢紝
鍚﹀垯瀹冨簲璇ユ垚涓?cache_add() 鐨勪竴涓弬鏁般€?

### 鎶婂璞℃毚闇插埌鏈枃浠朵箣澶?


濡傛灉鎴戜滑鐨勫璞″寘鍚洿澶氫俊鎭紝浠呬粎鍦ㄥ唴澶栧鍒朵俊鎭彲鑳藉氨涓嶅浜嗭細浠ｇ爜鐨勫叾浠栭儴鍒?
鍙兘鎯宠淇濈暀鎸囧悜杩欎簺瀵硅薄鐨勬寚閽堬紝渚嬪锛岃€屼笉鏄瘡娆￠兘鎸?id 鍘绘煡鎵俱€傝繖甯︽潵浜?
涓や釜闂銆?

绗竴涓棶棰樻槸锛屾垜浠娇鐢?`cache_lock` 鏉ヤ繚鎶ゅ璞★細鎴戜滑闇€瑕佽杩欎釜閿佸彉鎴愰潪
闈欐€佺殑锛屼互渚夸唬鐮佺殑鍏朵綑閮ㄥ垎鍙互浣跨敤瀹冦€傝繖璁╁姞閿佸彉寰楁洿妫樻墜锛屽洜涓哄畠涓嶅啀鍏ㄩ兘
闆嗕腑鍦ㄤ竴涓湴鏂逛簡銆?

绗簩涓棶棰樻槸鐢熷懡鍛ㄦ湡闂锛氬鏋滃彟涓€涓粨鏋勪綋淇濈暀浜嗕竴涓寚鍚戝璞＄殑鎸囬拡锛屽畠澶ф
鏈熸湜璇ユ寚閽堜繚鎸佹湁鏁堛€傞仐鎲剧殑鏄紝杩欏彧鍦ㄤ綘鎸佹湁閿佹湡闂存墠寰楀埌淇濊瘉锛屽惁鍒欐湁浜哄彲鑳?
璋冪敤 cache_delete()锛岀敋鑷虫洿绯燂紝娣诲姞鍙︿竴涓璞★紝澶嶇敤鍚屼竴鍦板潃銆?

鐢变簬鍙湁涓€鎶婇攣锛屼綘涓嶅彲鑳芥案杩滄寔鏈夊畠锛氬叾浠栦汉閮芥病娉曞共娲讳簡銆?

杩欎釜闂鐨勮В鍐冲姙娉曟槸浣跨敤寮曠敤璁℃暟锛氭瘡涓寔鏈夊璞℃寚閽堢殑浜猴紝鍦ㄧ涓€娆℃嬁鍒板璞℃椂
澧炲姞璁℃暟锛岀敤瀹屾椂鍑忓皯璁℃暟銆傝皝鎶婂畠鍑忓埌闆讹紝璋佸氨鐭ラ亾瀹冨凡鏃犱汉浣跨敤锛屽氨鍙互鐪熸
鍒犻櫎瀹冦€?

```
    --- cache.c.interrupt   2003-12-09 14:25:43.000000000 +1100
    +++ cache.c.refcnt  2003-12-09 14:33:05.000000000 +1100
    @@ -7,6 +7,7 @@
     struct object
     {
             struct list_head list;
    +        unsigned int refcnt;
             int id;
             char name[32];
             int popularity;
    @@ -17,6 +18,35 @@
     static unsigned int cache_num = 0;
     #define MAX_CACHE_SIZE 10

    +static void __object_put(struct object *obj)
    +{
    +        if (--obj->refcnt == 0)
    +                kfree(obj);
    +}
    +
    +static void __object_get(struct object *obj)
    +{
    +        obj->refcnt++;
    +}
    +
    +void object_put(struct object *obj)
    +{
    +        unsigned long flags;
    +
    +        spin_lock_irqsave(&cache_lock, flags);
    +        __object_put(obj);
    +        spin_unlock_irqrestore(&cache_lock, flags);
    +}
    +
    +void object_get(struct object *obj)
    +{
    +        unsigned long flags;
    +
    +        spin_lock_irqsave(&cache_lock, flags);
    +        __object_get(obj);
    +        spin_unlock_irqrestore(&cache_lock, flags);
    +}
    +
     /* Must be holding cache_lock */
     static struct object *__cache_find(int id)
     {
    @@ -35,6 +65,7 @@
     {
             BUG_ON(!obj);
             list_del(&obj->list);
    +        __object_put(obj);
             cache_num--;
     }

    @@ -63,6 +94,7 @@
             strscpy(obj->name, name, sizeof(obj->name));
             obj->id = id;
             obj->popularity = 0;
    +        obj->refcnt = 1; /* The cache holds a reference */

             spin_lock_irqsave(&cache_lock, flags);
             __cache_add(obj);
    @@ -79,18 +111,15 @@
             spin_unlock_irqrestore(&cache_lock, flags);
     }

    -int cache_find(int id, char *name)
    +struct object *cache_find(int id)
     {
             struct object *obj;
    -        int ret = -ENOENT;
             unsigned long flags;

             spin_lock_irqsave(&cache_lock, flags);
             obj = __cache_find(id);
    -        if (obj) {
    -                ret = 0;
    -                strcpy(name, obj->name);
    -        }
    +        if (obj)
    +                __object_get(obj);
             spin_unlock_irqrestore(&cache_lock, flags);
    -        return ret;
    +        return obj;
     }

```
鎴戜滑鎶婂紩鐢ㄨ鏁板皝瑁呭湪鏍囧噯鐨勪袱涓?'get' 鍜?'put' 鍑芥暟涓€傜幇鍦ㄦ垜浠彲浠ヨ
cache_find() 鐩存帴杩斿洖瀵硅薄鏈韩锛岃繖鏍峰仛鐨勫ソ澶勬槸锛岀敤鎴风幇鍦ㄥ彲浠ュ湪鎸佹湁瀵硅薄鏃?
鐫＄湢锛堜緥濡傜敤 copy_to_user() 鎶婂悕瀛楀鍒跺埌鐢ㄦ埛绌洪棿锛夈€?

鍙︿竴鐐硅娉ㄦ剰鐨勬槸锛屾垜璇磋繃姣忎釜鎸囧悜瀵硅薄鐨勬寚閽堥兘搴旇鎸佹湁涓€涓紩鐢細鍥犳褰撳璞?
棣栨鎻掑叆缂撳瓨鏃讹紝寮曠敤璁℃暟涓?1銆傚湪鏌愪簺鐗堟湰涓紝璇ユ鏋跺苟涓嶆寔鏈夊紩鐢ㄨ鏁帮紝浣嗛偅鏍?
浼氭洿澶嶆潅銆?

#### 浣跨敤鍘熷瓙鎿嶄綔瀹炵幇寮曠敤璁℃暟


鍦ㄥ疄璺典腑锛宍atomic_t` 閫氬父鐢ㄤ簬 refcnt銆俙include/asm/atomic.h` 涓畾涔変簡涓€缁?
鍘熷瓙鎿嶄綔锛氬畠浠繚璇佽兘浠庣郴缁熶腑鐨勬墍鏈?CPU 浠ュ師瀛愭柟寮忕湅鍒帮紝鍥犳涓嶉渶瑕侀攣銆傚湪杩欑
鎯呭喌涓嬶紝瀹冩瘮浣跨敤鑷棆閿佹洿绠€鍗曪紝灏界瀵逛簬浠讳綍涓嶅钩鍑＄殑鎯呭喌锛屼娇鐢ㄨ嚜鏃嬮攣浼氭洿娓呮櫚銆?
杩欓噷浣跨敤 atomic_inc() 鍜?atomic_dec_and_test() 鏉ユ浛浠ｆ爣鍑嗙殑
閫掑鍜岄€掑噺杩愮畻绗︼紝骞朵笖涓嶅啀鐢ㄩ攣鏉ヤ繚鎶ゅ紩鐢ㄨ鏁版湰韬€?

```
    --- cache.c.refcnt  2003-12-09 15:00:35.000000000 +1100
    +++ cache.c.refcnt-atomic   2003-12-11 15:49:42.000000000 +1100
    @@ -7,7 +7,7 @@
     struct object
     {
             struct list_head list;
    -        unsigned int refcnt;
    +        atomic_t refcnt;
             int id;
             char name[32];
             int popularity;
    @@ -18,33 +18,15 @@
     static unsigned int cache_num = 0;
     #define MAX_CACHE_SIZE 10

    -static void __object_put(struct object *obj)
    -{
    -        if (--obj->refcnt == 0)
    -                kfree(obj);
    -}
    -
    -static void __object_get(struct object *obj)
    -{
    -        obj->refcnt++;
    -}
    -
     void object_put(struct object *obj)
     {
    -        unsigned long flags;
    -
    -        spin_lock_irqsave(&cache_lock, flags);
    -        __object_put(obj);
    -        spin_unlock_irqrestore(&cache_lock, flags);
    +        if (atomic_dec_and_test(&obj->refcnt))
    +                kfree(obj);
     }

     void object_get(struct object *obj)
     {
    -        unsigned long flags;
    -
    -        spin_lock_irqsave(&cache_lock, flags);
    -        __object_get(obj);
    -        spin_unlock_irqrestore(&cache_lock, flags);
    +        atomic_inc(&obj->refcnt);
     }

     /* Must be holding cache_lock */
    @@ -65,7 +47,7 @@
     {
             BUG_ON(!obj);
             list_del(&obj->list);
    -        __object_put(obj);
    +        object_put(obj);
             cache_num--;
     }

    @@ -94,7 +76,7 @@
             strscpy(obj->name, name, sizeof(obj->name));
             obj->id = id;
             obj->popularity = 0;
    -        obj->refcnt = 1; /* The cache holds a reference */
    +        atomic_set(&obj->refcnt, 1); /* The cache holds a reference */

             spin_lock_irqsave(&cache_lock, flags);
             __cache_add(obj);
    @@ -119,7 +101,7 @@
             spin_lock_irqsave(&cache_lock, flags);
             obj = __cache_find(id);
             if (obj)
    -                __object_get(obj);
    +                object_get(obj);
             spin_unlock_irqrestore(&cache_lock, flags);
             return obj;
     }

```
### 淇濇姢瀵硅薄鑷韩


鍦ㄨ繖浜涗緥瀛愪腑锛屾垜浠亣璁惧璞★紙寮曠敤璁℃暟闄ゅ锛変竴鏃﹀垱寤哄氨鍐嶄篃涓嶄細鏀瑰彉銆傚鏋滄垜浠?
鎯冲厑璁?name 鍙戠敓鏀瑰彉锛屾湁涓夌鍙兘锛?

- 浣犲彲浠ヨ `cache_lock` 鍙樻垚闈為潤鎬佺殑锛屽苟鍛婅瘔浜轰滑鍦ㄤ慨鏀逛换浣曞璞′腑鐨?name
   涔嬪墠鍏堣幏鍙栭偅鎶婇攣銆?

- 浣犲彲浠ユ彁渚涗竴涓?cache_obj_rename()锛屽畠鑾峰彇杩欐妸閿佸苟鏇胯皟鐢ㄨ€呬慨鏀?name锛?
   骞跺憡璇夊ぇ瀹朵娇鐢ㄩ偅涓嚱鏁般€?

- 浣犲彲浠ヨ `cache_lock` 鍙繚鎶ょ紦瀛樻湰韬紝鑰岀敤鍙︿竴鎶婇攣鏉ヤ繚鎶?name銆?

鐞嗚涓婏紝浣犲彲浠ユ妸閿佸仛寰楅潪甯哥粏绮掑害锛岀粏鍒版瘡涓璞℃瘡涓瓧娈典竴鎶婇攣銆傚疄璺典腑锛屾渶
甯歌鐨勫彉浣撴槸锛?

- 涓€鎶婇攣淇濇姢鍩虹璁炬柦锛堟湰渚嬩腑鐨?`cache` 鍒楄〃锛夊拰鎵€鏈夊璞°€傝繖鏄垜浠洰鍓?
   鎵€鍋氱殑銆?

- 涓€鎶婇攣淇濇姢鍩虹璁炬柦锛堝寘鎷璞″唴閮ㄧ殑鍒楄〃鎸囬拡锛夛紝瀵硅薄鍐呰繕鏈変竴鎶婇攣淇濇姢
   璇ュ璞＄殑鍏朵綑閮ㄥ垎銆?

- 澶氭妸閿佷繚鎶ゅ熀纭€璁炬柦锛堜緥濡傛瘡鏉″搱甯岄摼涓€鎶婇攣锛夛紝鍙兘鍐嶉厤鍚堜竴鎶婄嫭绔嬬殑姣忓璞￠攣銆?

涓嬮潰鏄€滄瘡瀵硅薄閿佲€濈殑瀹炵幇锛?

```
    --- cache.c.refcnt-atomic   2003-12-11 15:50:54.000000000 +1100
    +++ cache.c.perobjectlock   2003-12-11 17:15:03.000000000 +1100
    @@ -6,11 +6,17 @@

     struct object
     {
    +        /* These two protected by cache_lock. */
             struct list_head list;
    +        int popularity;
    +
             atomic_t refcnt;
    +
    +        /* Doesn't change once created. */
             int id;
    +
    +        spinlock_t lock; /* Protects the name */
             char name[32];
    -        int popularity;
     };

     static DEFINE_SPINLOCK(cache_lock);
    @@ -77,6 +84,7 @@
             obj->id = id;
             obj->popularity = 0;
             atomic_set(&obj->refcnt, 1); /* The cache holds a reference */
    +        spin_lock_init(&obj->lock);

             spin_lock_irqsave(&cache_lock, flags);
             __cache_add(obj);

```
娉ㄦ剰锛屾垜鍐冲畾璁?popularity 璁℃暟鐢?`cache_lock` 鑰岄潪姣忓璞￠攣鏉ヤ繚鎶わ細
杩欐槸鍥犱负瀹冿紙鍍忓璞″唴閮ㄧ殑 `struct list_head <list_head>` 涓€鏍凤級鍦ㄩ€昏緫涓?
灞炰簬鍩虹璁炬柦銆傝繖鏍蜂竴鏉ワ紝鍦?__cache_add() 瀵绘壘鏈€涓嶅父鐢ㄥ璞℃椂锛屾垜灏变笉蹇呭幓鑾峰彇
姣忎釜瀵硅薄鐨勯攣銆?

鎴戣繕鍐冲畾 id 鎴愬憳涓嶅彲鏇存敼锛屽洜姝ゅ湪 __cache_find() 涓鏌?id 鏃朵笉闇€瑕佸幓鑾峰彇
姣忎釜瀵硅薄鐨勯攣锛氬璞￠攣鍙鎯宠璇诲啓 name 瀛楁鐨勮皟鐢ㄨ€呬娇鐢ㄣ€?

杩樿娉ㄦ剰锛屾垜鍔犱簡涓€鏉℃敞閲婏紝璇存槑鍝簺鏁版嵁鐢卞摢浜涢攣淇濇姢銆傝繖鏋佸叾閲嶈锛屽洜涓哄畠鎻忚堪浜?
浠ｇ爜鐨勮繍琛屾椂琛屼负锛岃€屼笖浠呴潬闃呰寰堥毦鐪嬪嚭鏉ャ€傛濡?Alan Cox 鎵€璇达紝鈥滈攣浣忔暟鎹紝
鑰屼笉鏄唬鐮侊紙Lock data, not code锛夆€濄€?

## 甯歌闂


### 姝婚攣锛氱畝鍗曠殑涓庨珮绾х殑


鏈変竴绉嶇紪鐮侀敊璇紝鏄竴娈典唬鐮佽瘯鍥句袱娆¤幏鍙栧悓涓€涓嚜鏃嬮攣锛氬畠浼氭案杩滆嚜鏃嬶紝绛夊緟閿佽
閲婃斁锛堣嚜鏃嬮攣銆乺wlock 鍜屼簰鏂ヤ綋鍦?Linux 涓兘涓嶆槸鍙噸鍏ョ殑锛夈€傝繖寰堝鏄撹瘖鏂細
涓嶆槸閭ｇ鈥滆繛缁簲涓櫄涓婁笉鐫¤銆佸拰姣涜尭鑼哥殑浠ｇ爜鍏斿瓙瀵硅瘽鈥濇墠鑳芥悶瀹氱殑闂銆?

绋嶅井澶嶆潅涓€鐐圭殑鎯呭喌鏄紝鍋囪浣犳湁涓€涓杞腑鏂拰鐢ㄦ埛涓婁笅鏂囧叡浜殑鍖哄煙銆傚鏋滀綘
浣跨敤 spin_lock() 璋冪敤鏉ヤ繚鎶ゅ畠锛岀敤鎴蜂笂涓嬫枃鏈夊彲鑳藉湪鎸佹湁閿佹椂琚蒋涓柇鎵撴柇锛?
鑰岃蒋涓柇闅忓悗浼氭案杩滆嚜鏃嬶紝璇曞浘鑾峰彇鍚屼竴鎶婇攣銆?

杩欎袱绉嶆儏鍐甸兘鍙仛姝婚攣锛屽涓婃墍绀猴紝鍗充娇鍙湁涓€涓?CPU 涔熷彲鑳藉彂鐢燂紙灏界鍦?UP 缂栬瘧
涓嬩笉浼氾紝鍥犱负鍦?`CONFIG_SMP`\ =n 鐨勫唴鏍哥紪璇戜腑鑷棆閿佷細娑堝け銆備絾鍦ㄧ浜屼釜渚嬪瓙閲?
浣犱粛鐒朵細寰楀埌鏁版嵁鎹熷潖锛夈€?

杩欑瀹屽叏鐨勯攣姝诲緢瀹规槗璇婃柇锛氬湪 SMP 鏈哄櫒涓婏紝鐪嬮棬鐙楀畾鏃跺櫒锛屾垨鑰呯紪璇戞椂璁剧疆
`DEBUG_SPINLOCK`锛坄include/linux/spinlock.h`锛夛紝浼氬湪瀹冨彂鐢熸椂绔嬪嵆鏆撮湶鍑烘潵銆?

涓€涓洿澶嶆潅鐨勯棶棰樻槸鎵€璋撶殑鈥滆嚧鍛芥嫢鎶扁€濓紙deadly embrace锛夛紝娑夊強涓ゆ妸鎴栨洿澶氶攣銆?
鍋囪浣犳湁涓€涓搱甯岃〃锛氳〃涓殑姣忎竴椤归兘鏄竴涓嚜鏃嬮攣锛屼互鍙婁竴鏉″搱甯屽璞＄殑閾俱€傚湪
涓€涓蒋涓柇澶勭悊绋嬪簭涓紝浣犳湁鏃舵兂鎶婃煇涓璞′粠鍝堝笇琛ㄧ殑涓€澶勭Щ鍒板彟涓€澶勶細浣犺幏鍙?
鏃у搱甯岄摼鐨勯攣鍜屾柊鍝堝笇閾剧殑閿侊紝鎶婂璞′粠鏃ч摼鍒犻櫎锛屽啀鎻掑叆鏂伴摼銆?

杩欓噷鏈変袱涓棶棰樸€傜涓€锛屽鏋滀綘鐨勪唬鐮佽瘯鍥炬妸瀵硅薄绉诲埌鍚屼竴鏉￠摼锛屽畠浼氫笌鑷繁姝婚攣锛?
鍥犱负瀹冭瘯鍥惧姞閿佷袱娆°€傜浜岋紝濡傛灉鍙︿竴涓?CPU 涓婂悓涓€涓蒋涓柇姝ｈ瘯鍥炬妸鍙︿竴涓璞?
鍙嶅悜绉诲姩锛屽彲鑳戒細鍙戠敓濡備笅鎯呭喌锛?

+-----------------------+-----------------------+
| CPU 1                 | CPU 2                 |
+=======================+=======================+
| Grab lock A -> OK     | Grab lock B -> OK     |
+-----------------------+-----------------------+
| Grab lock B -> spin   | Grab lock A -> spin   |
+-----------------------+-----------------------+

Table: Consequences

涓や釜 CPU 浼氭案杩滆嚜鏃嬶紝绛夊緟瀵规柟鏀惧純鑷繁鐨勯攣銆傚畠鐪嬭捣鏉ャ€侀椈璧锋潵銆佹懜璧锋潵閮藉儚涓€娆″穿婧冦€?

### 棰勯槻姝婚攣


鏁欑涔︿細鍛婅瘔浣狅紝濡傛灉浣犳€绘槸鎸夌浉鍚岀殑椤哄簭鍔犻攣锛屽氨姘歌繙涓嶄細鍑虹幇杩欑被姝婚攣銆傚疄璺典細
鍛婅瘔浣狅紝杩欑鏂规硶鏃犳硶鎵╁睍锛氬綋鎴戞柊鍒涘缓涓€鎶婇攣鏃讹紝鎴戝鍐呮牳浜嗚В寰楄繕涓嶅澶氾紝鏃犳硶
寮勬竻瀹冭鏀惧湪閭?5000 灞傞攣鐨勫眰绾х粨鏋勪腑鐨勪粈涔堜綅缃€?

鏈€濂界殑閿佹槸灏佽璧锋潵鐨勶細瀹冧滑姘歌繙涓嶄細鍑虹幇鍦ㄥご鏂囦欢涓紝涔熸案杩滀笉浼氬湪璋冪敤鍚屼竴鏂囦欢
涔嬪鐨勯潪骞冲嚒鍑芥暟鏃舵寔鏈夈€備綘鍙互閫氳杩欐浠ｇ爜骞剁湅鍑哄畠姘歌繙涓嶄細姝婚攣锛屽洜涓哄畠鍦?
鎸佹湁杩欐妸閿佹椂浠庝笉璇曞浘鍘昏幏鍙栧彟涓€鎶婇攣銆備娇鐢ㄤ綘浠ｇ爜鐨勪汉鐢氳嚦涓嶉渶瑕佺煡閬撲綘鍦ㄤ娇鐢ㄩ攣銆?

杩欓噷涓€涓吀鍨嬬殑闂鏄紝褰撲綘鎻愪緵鍥炶皟鎴栭挬瀛愭椂锛氬鏋滀綘鍦ㄦ寔鏈夐攣鐨勬儏鍐典笅璋冪敤瀹冧滑锛?
浣犲氨鏈夌畝鍗曟閿佹垨鑷村懡鎷ユ姳鐨勯闄╋紙璋佺煡閬撳洖璋冧細鍋氫粈涔堬紵锛夈€?

#### 杩囧害绉瀬鍦伴闃叉閿?


姝婚攣鍥虹劧鎴愰棶棰橈紝浣嗕笉濡傛暟鎹崯鍧忎弗閲嶃€備竴娈典唬鐮佸厛鑾峰彇璇婚攣銆佹悳绱㈠垪琛ㄣ€佸彂鐜版壘涓嶅埌
鎯宠鐨勩€侀噴鏀捐閿併€佸啀鑾峰彇鍐欓攣骞舵彃鍏ュ璞★紝杩欏氨鏈夌珵鎬佹潯浠躲€?

### 绔為€熺殑瀹氭椂鍣細鍐呮牳鐨勪竴椤规秷閬?


瀹氭椂鍣ㄨ嚜韬篃浼氫骇鐢熷畠鐗规湁鐨勭珵鎬侀棶棰樸€傝€冭檻涓€缁勫璞★紙鍒楄〃銆佸搱甯岃〃绛夛級锛屽叾涓?
姣忎釜瀵硅薄閮芥湁涓€涓畾鏃跺櫒锛屽埌鏈熸椂閿€姣佸畠銆?

濡傛灉浣犳兂閿€姣佹暣涓泦鍚堬紙渚嬪鍦ㄦā鍧楃Щ闄ゆ椂锛夛紝

```

            /* THIS CODE BAD BAD BAD BAD: IF IT WAS ANY WORSE IT WOULD USE
               HUNGARIAN NOTATION */
            spin_lock_bh(&list_lock);

            while (list) {
                    struct foo *next = list->next;
                    timer_delete(&list->timer);
                    kfree(list);
                    list = next;
            }

            spin_unlock_bh(&list_lock);


```
杩熸棭锛岃繖鍦?SMP 涓婁細宕╂簝锛屽洜涓哄畾鏃跺櫒鍙兘鎭板ソ鍦?spin_lock_bh() 涔嬪墠宸茬粡瑙﹀彂锛?
瀹冨彧浼氬湪鎴戜滑 spin_unlock_bh() 涔嬪悗鎵嶆嬁鍒伴攣锛岀劧鍚庤瘯鍥鹃噴鏀鹃偅涓厓绱?
锛堣€屽畠宸茬粡琚噴鏀炬帀浜嗭紒锛夈€?

杩欏彲浠ラ€氳繃妫€鏌?timer_delete() 鐨勮繑鍥炲€兼潵閬垮厤锛氬鏋滆繑鍥?1锛岃鏄庡畾鏃跺櫒宸茶鍒犻櫎銆?
濡傛灉杩斿洖 0锛屽垯鎰忓懗鐫€锛堝湪姝や緥涓級瀹冨綋鍓嶆鍦ㄨ繍琛岋紝鍥犳鎴戜滑鍙互

```

            retry:
                    spin_lock_bh(&list_lock);

                    while (list) {
                            struct foo *next = list->next;
                            if (!timer_delete(&list->timer)) {
                                    /* Give timer a chance to delete this */
                                    spin_unlock_bh(&list_lock);
                                    goto retry;
                            }
                            kfree(list);
                            list = next;
                    }

                    spin_unlock_bh(&list_lock);


```
鍙︿竴涓父瑙侀棶棰樻槸鍒犻櫎閭ｄ簺浼氳嚜鎴戦噸鍚殑瀹氭椂鍣紙鍦ㄥ畾鏃跺櫒鍑芥暟鏈熬璋冪敤 add_timer()锛夈€?
鍥犱负杩欐槸涓€涓浉褰撳父瑙併€佸張瀹规槗鍑虹幇绔炴€佺殑鎯呭喌锛屼綘搴旇浣跨敤 timer_delete_sync()
锛坄include/linux/timer.h`锛夋潵澶勭悊杩欑鎯呭喌銆?

鍦ㄩ噴鏀惧畾鏃跺櫒涔嬪墠锛屽簲璇ヨ皟鐢?timer_shutdown() 鎴?timer_shutdown_sync()锛?
瀹冧滑浼氶槻姝㈠畾鏃跺櫒琚噸鏂板敜璧枫€傞殢鍚庝换浣曢噸鏂板敜璧峰畾鏃跺櫒鐨勫皾璇曢兘浼氳鏍稿績浠ｇ爜闈欓粯蹇界暐銆?

## 鍔犻攣閫熷害


鍦ㄨ€冭檻鏌愪簺鍔犻攣浠ｇ爜鐨勬€ц兘鏃讹紝鏈変笁涓富瑕佹柟闈㈤渶瑕佹媴蹇冦€傜涓€鏄苟鍙戯細褰撳埆浜烘寔鏈?
閿佹椂锛屼細鏈夊灏戜笢瑗垮湪绛夊緟銆傜浜屾槸瀹為檯鑾峰彇鍜岄噴鏀句竴鎶婃棤绔炰簤閿佹墍鑺辩殑鏃堕棿銆傜涓夋槸
浣跨敤鏇村皯鎴栨洿鑱槑鐨勯攣銆傛垜鍋囧畾杩欐妸閿佷娇鐢ㄥ緱鐩稿綋棰戠箒锛氬惁鍒欙紝浣犱笉浼氬叧蹇冩晥鐜囥€?

骞跺彂鍙栧喅浜庨攣閫氬父琚寔鏈夊闀挎椂闂达細浣犲簲璇ユ寜闇€鎸佹湁閿侊紝浣嗙粷涓嶈鏇翠箙銆傚湪缂撳瓨渚嬪瓙涓紝
鎴戜滑鎬绘槸鍦ㄤ笉鎸佹湁閿佺殑鎯呭喌涓嬪垱寤哄璞★紝鐒跺悗鍙湪鍑嗗鎶婂畠鎻掑叆鍒楄〃鏃舵墠鑾峰彇閿併€?

鑾峰彇鏃堕棿鍙栧喅浜庨攣鎿嶄綔瀵规祦姘寸嚎閫犳垚浜嗗澶х牬鍧忥紙娴佹按绾垮仠椤匡級锛屼互鍙婅繖涓?CPU 鏄惁
鏈€鏈夊彲鑳藉氨鏄笂涓€涓嬁鍒伴攣鐨?CPU锛堝嵆杩欐妸閿佸璇?CPU 鏄惁鏄紦瀛樼儹鐨勶級锛氬湪 CPU
鏇村鐨勬満鍣ㄤ笂锛岃繖绉嶅彲鑳芥€т笅闄嶅緱寰堝揩銆備互涓€鍙?700MHz 鐨?Intel Pentium III 涓轰緥锛?
涓€鏉℃寚浠ょ害闇€ 0.7ns锛屼竴娆″師瀛愰€掑绾﹂渶 58ns锛屼竴鎶婂璇?CPU 缂撳瓨鐑殑閿佺害闇€ 160ns锛?
鑰屼粠鍙︿竴涓?CPU 鍋氫竴娆＄紦瀛樿浼犺緭杩樿棰濆 170 鍒?360ns銆傦紙杩欎簺鏁板瓧鏉ヨ嚜 Paul
McKenney 鐨?`Linux Journal RCU 鏂囩珷
<http://www.linuxjournal.com/article.php?sid=6993>`__锛夈€?

杩欎袱涓洰鏍囩浉浜掑啿绐侊細鎶婇攣鎸佹湁寰堢煭鐨勬椂闂达紝鍙互閫氳繃鎶婇攣鎷嗗垎鎴愬涓儴鍒嗘潵瀹炵幇
锛堜緥濡傛垜浠渶鍚庣殑姣忓璞￠攣渚嬪瓙锛夛紝浣嗚繖浼氬鍔犻攣鑾峰彇鐨勬鏁帮紝缁撴灉寰€寰€姣斾娇鐢ㄥ崟涓€
閿佹洿鎱€傝繖鏄富寮犫€滈攣瑕佺畝鍗曗€濈殑鍙︿竴涓悊鐢便€?

绗笁涓柟闈㈢殑椤捐檻鍦ㄤ笅闈㈣璁猴細鏈変竴浜涙柟娉曞彲浠ュ噺灏戦渶瑕佽繘琛岀殑鍔犻攣閲忋€?

### 璇诲啓閿佸彉浣?


鑷棆閿佸拰浜掓枼浣撻兘鏈夎鍐欏彉浣擄細`rwlock_t` 鍜?`struct rw_semaphore
<rw_semaphore>`銆傚畠浠妸浣跨敤鑰呭垎鎴愪袱绫伙細璇昏€呭拰鍐欒€呫€傚鏋滀綘鍙槸璇诲彇鏁版嵁锛?
浣犲彲浠ヨ幏鍙栬閿侊紝浣嗚鍐欐暟鎹氨闇€瑕佸啓閿併€傝澶氫汉鍙互鎸佹湁璇婚攣锛屼絾鍐欒€呭繀椤绘槸鍞竴鐨?
鎸佹湁鑰呫€?

濡傛灉浣犵殑浠ｇ爜鑳芥竻鏅板湴鎸夎鑰?鍐欒€呭垝鍒嗭紙灏卞儚鎴戜滑鐨勭紦瀛樹唬鐮侀偅鏍凤級锛岃€屼笖閿佽璇昏€?
鎸佹湁鐨勬椂闂磋緝闀匡紝浣跨敤杩欎簺閿佷細鏈夊府鍔┿€備笉杩囧畠浠瘮鏅€氶攣绋嶆參锛屾墍浠ュ疄璺典腑 `rwlock_t`
閫氬父骞朵笉鍒掔畻銆?

### 閬垮厤鍔犻攣锛氳-澶嶅埗-鏇存柊锛圧CU锛?


鏈変竴绉嶇壒娈婄殑璇诲啓閿佹柟娉曞彨鍋氳-澶嶅埗-鏇存柊锛圧ead Copy Update锛孯CU锛夈€備娇鐢?RCU锛?
璇昏€呭彲浠ュ畬鍏ㄩ伩鍏嶈幏鍙栭攣锛氬洜涓烘垜浠鏈熺紦瀛樿璇诲彇鐨勬鏁板浜庤鏇存柊鐨勬鏁?
锛堝惁鍒欑紦瀛樺氨鏄氮璐规椂闂达級锛屽畠鏄竴涓繘琛岃繖绉嶄紭鍖栫殑鍊欓€夎€呫€?

鎴戜滑濡備綍鍘绘帀璇婚攣锛熷幓鎺夎閿佹剰鍛崇潃锛屽啓鑰呭彲鑳藉湪璇昏€呮鍦ㄩ亶鍘嗗垪琛ㄦ椂淇敼瀹冦€傝繖鍏跺疄
鐩稿綋绠€鍗曪細濡傛灉鍐欒€呴潪甯稿皬蹇冨湴娣诲姞鍏冪礌锛屾垜浠氨鍙互鍦ㄥ厓绱犺娣诲姞鐨勫悓鏃惰鍙栭摼琛ㄣ€?
渚嬪锛?

```

            new->next = list->next;
            wmb();
            list->next = new;


```
wmb() 鏄竴涓啓鍐呭瓨灞忛殰銆傚畠纭繚绗竴涓搷浣滐紙璁剧疆鏂板厓绱犵殑 `next` 鎸囬拡锛夊凡瀹屾垚锛?
骞朵笖浼氳鎵€鏈?CPU 鐪嬪埌锛岀劧鍚庢墠杩涜绗簩涓搷浣滐紙鎶婃柊鍏冪礌鏀惧叆鍒楄〃锛夈€傝繖寰堥噸瑕侊紝
鍥犱负鐜颁唬缂栬瘧鍣ㄥ拰鐜颁唬 CPU 閮藉彲鑳藉湪鏈鏄庣‘鍛婄煡鐨勬儏鍐典笅閲嶆帓鎸囦护锛氭垜浠笇鏈涜鑰?
瑕佷箞瀹屽叏鐪嬩笉鍒版柊鍏冪礌锛岃涔堢湅鍒板甫鏈夋纭寚鍚戦摼琛ㄥ叾浣欓儴鍒嗙殑 `next` 鎸囬拡鐨勬柊鍏冪礌銆?

骞歌繍鐨勬槸锛屾湁涓€涓嚱鏁板彲浠ヤ负鏍囧噯鐨?`struct list_head <list_head>` 鍒楄〃鍋氳繖浠朵簨锛?
list_add_rcu()锛坄include/linux/list.h`锛夈€?

浠庡垪琛ㄤ腑鍒犻櫎涓€涓厓绱犳洿绠€鍗曪細鎴戜滑鐢ㄦ寚鍚戝叾鍚庣户鐨勬寚閽堟浛鎹㈡寚鍚戞棫鍏冪礌鐨勬寚閽堬紝璇昏€?
瑕佷箞鐪嬪埌瀹冿紝瑕佷箞璺宠繃瀹冦€?

```

            list->next = old->next;


```
鏈?list_del_rcu()锛坄include/linux/list.h`锛夋潵鍋氳繖浠朵簨锛堟櫘閫氱増鏈細姹℃煋鏃у璞★紝
鑰岄偅涓嶆槸鎴戜滑鎯宠鐨勶級銆?

璇昏€呬篃蹇呴』灏忓績锛氭煇浜?CPU 浼氶『鐫€ `next` 鎸囬拡鎻愬墠寮€濮嬭鍙栦笅涓€涓厓绱犵殑鍐呭锛屼絾褰?
`next` 鎸囬拡鍦ㄥ畠浠剼涓嬫敼鍙樻椂锛屽嵈娌℃湁鎰忚瘑鍒伴鍙栫殑鍐呭鏄敊璇殑銆傚啀涓€娆★紝鏈?
list_for_each_entry_rcu()锛坄include/linux/list.h`锛夋潵甯姪浣犮€傚綋鐒讹紝鍐欒€呭彧
闇€浣跨敤 list_for_each_entry() 鍗冲彲锛屽洜涓轰笉鍙兘鏈変袱涓悓鏃剁殑鍐欒€呫€?

鎴戜滑鏈€缁堢殑鍥板鏄細鎴戜滑绌剁珶浠€涔堟椂鍊欐墠鑳界湡姝ｉ攢姣佽鍒犻櫎鐨勫厓绱狅紵璁颁綇锛岃鑰呯幇鍦?
鍙兘姝ｅ湪閬嶅巻鍒楄〃涓殑杩欎釜鍏冪礌锛氬鏋滄垜浠噴鏀句簡杩欎釜鍏冪礌锛岃€?`next` 鎸囬拡闅忎箣鏀瑰彉锛?
璇昏€呭氨浼氳烦杩涘瀮鍦惧苟宕╂簝銆傛垜浠渶瑕佺瓑鍒版垜浠煡閬擄紝鍦ㄦ垜浠垹闄よ鍏冪礌鏃舵墍鏈夋鍦ㄩ亶鍘?
鍒楄〃鐨勮鑰呴兘宸插畬鎴愩€傛垜浠娇鐢?call_rcu() 娉ㄥ唽涓€涓洖璋冿紝涓€鏃︽墍鏈夋棦瀛樼殑璇昏€呴兘瀹屾垚锛?
瀹冨氨浼氱湡姝ｉ攢姣佸璞°€傛垨鑰咃紝涔熷彲浠ヤ娇鐢?synchronize_rcu() 鏉ラ樆濉烇紝鐩村埌鎵€鏈夋棦瀛樼殑
璇昏€呴兘瀹屾垚銆?

浣嗚-澶嶅埗-鏇存柊鎬庝箞鐭ラ亾璇昏€呭凡缁忓畬鎴愪簡鍛紵鏂规硶鏄繖鏍风殑锛氶鍏堬紝璇昏€呮€绘槸鍦?
rcu_read_lock()/rcu_read_unlock() 瀵瑰唴閮ㄩ亶鍘嗗垪琛細瀹冧滑浠呬粎鏄鐢ㄦ姠鍗狅紝杩欐牱
璇昏€呭湪璇诲彇鍒楄〃鏃跺氨涓嶄細鍘荤潯鐪犮€?

鐒跺悗 RCU 绛夊緟锛岀洿鍒版瘡涓叾浠?CPU 鑷冲皯鐫＄湢杩囦竴娆★細鐢变簬璇昏€呬笉鑳界潯鐪狅紝鎴戜滑灏辩煡閬擄紝
鍦ㄥ垹闄ゆ湡闂存鍦ㄩ亶鍘嗗垪琛ㄧ殑浠讳綍璇昏€呴兘宸茬粡瀹屾垚锛屼簬鏄Е鍙戝洖璋冦€傜湡姝ｇ殑璇?澶嶅埗-鏇存柊
浠ｇ爜姣旇繖浼樺寲寰楃◢濂戒竴浜涳紝浣嗚繖灏辨槸鍩烘湰鎬濇兂銆?

```

    --- cache.c.perobjectlock   2003-12-11 17:15:03.000000000 +1100
    +++ cache.c.rcupdate    2003-12-11 17:55:14.000000000 +1100
    @@ -1,15 +1,18 @@
     #include <linux/list.h>
     #include <linux/slab.h>
     #include <linux/string.h>
    +#include <linux/rcupdate.h>
     #include <linux/mutex.h>
     #include <asm/errno.h>

     struct object
     {
    -        /* These two protected by cache_lock. */
    +        /* This is protected by RCU */
             struct list_head list;
             int popularity;

    +        struct rcu_head rcu;
    +
             atomic_t refcnt;

             /* Doesn't change once created. */
    @@ -40,7 +43,7 @@
     {
             struct object *i;

    -        list_for_each_entry(i, &cache, list) {
    +        list_for_each_entry_rcu(i, &cache, list) {
                     if (i->id == id) {
                             i->popularity++;
                             return i;
    @@ -49,19 +52,25 @@
             return NULL;
     }

    +/* Final discard done once we know no readers are looking. */
    +static void cache_delete_rcu(void *arg)
    +{
    +        object_put(arg);
    +}
    +
     /* Must be holding cache_lock */
     static void __cache_delete(struct object *obj)
     {
             BUG_ON(!obj);
    -        list_del(&obj->list);
    -        object_put(obj);
    +        list_del_rcu(&obj->list);
             cache_num--;
    +        call_rcu(&obj->rcu, cache_delete_rcu);
     }

     /* Must be holding cache_lock */
     static void __cache_add(struct object *obj)
     {
    -        list_add(&obj->list, &cache);
    +        list_add_rcu(&obj->list, &cache);
             if (++cache_num > MAX_CACHE_SIZE) {
                     struct object *i, *outcast = NULL;
                     list_for_each_entry(i, &cache, list) {
    @@ -104,12 +114,11 @@
     struct object *cache_find(int id)
     {
             struct object *obj;
    -        unsigned long flags;

    -        spin_lock_irqsave(&cache_lock, flags);
    +        rcu_read_lock();
             obj = __cache_find(id);
             if (obj)
                     object_get(obj);
    -        spin_unlock_irqrestore(&cache_lock, flags);
    +        rcu_read_unlock();
             return obj;
     }

```
娉ㄦ剰锛岃鑰呬細鍦?__cache_find() 涓慨鏀?popularity 鎴愬憳锛岃€岀幇鍦ㄥ畠骞朵笉鎸佹湁閿併€?
涓€绉嶈В鍐冲姙娉曟槸鎶婂畠鏀规垚 `atomic_t`锛屼絾瀵逛簬杩欑鐢ㄦ硶锛屾垜浠叾瀹炲苟涓嶅叧蹇冪珵鎬侊細
涓€涓繎浼肩殑缁撴灉灏辫冻澶熶簡锛屾墍浠ユ垜娌℃敼瀹冦€?

缁撴灉鏄紝cache_find() 涓嶉渶瑕佷笌浠讳綍鍏朵粬鍑芥暟鍚屾锛屽洜姝ゅ湪 SMP 涓婂嚑涔庡拰鍦?UP 涓?
涓€鏍峰揩銆?

杩欓噷杩樻湁杩涗竴姝ョ殑浼樺寲鍙兘锛氬洖鎯虫垜浠渶鍒濈殑缂撳瓨浠ｇ爜锛岄偅鏃舵病鏈夊紩鐢ㄨ鏁帮紝璋冪敤鑰?
鍙鍦ㄤ娇鐢ㄥ璞℃椂灏辩畝鍗曞湴鎸佹湁閿侊紵杩欎粛鐒舵槸鍙兘鐨勶細濡傛灉浣犳寔鏈夐攣锛屽氨娌℃湁浜鸿兘鍒犻櫎
瀵硅薄锛屾墍浠ヤ綘涓嶉渶瑕佸幓澧炲噺寮曠敤璁℃暟銆?

鐜板湪锛岀敱浜?RCU 涓殑鈥滆閿佲€濅粎浠呮槸绂佺敤鎶㈠崰锛屼竴涓缁堝湪璋冪敤 cache_find() 鍜?
object_put() 涔嬮棿绂佺敤鎶㈠崰鐨勮皟鐢ㄨ€咃紝瀹為檯涓婁笉闇€瑕佸幓澧炲噺寮曠敤璁℃暟锛氭垜浠彲浠ヨ
__cache_find() 鍙樻垚闈為潤鎬佺殑鏉ユ毚闇插畠锛岃繖鏍风殑璋冪敤鑰呯洿鎺ヨ皟鐢ㄥ畠鍗冲彲銆?

杩欐牱鍋氱殑濂藉鏄紩鐢ㄨ鏁颁笉浼氳鍐欏叆锛氬璞′笉浼氳浠ヤ换浣曟柟寮忎慨鏀癸紝鐢变簬缂撳瓨鐨勭紭鏁咃紝
杩欏湪 SMP 鏈哄櫒涓婅蹇緱澶氥€?

### 姣?CPU 鏁版嵁


鍙︿竴绉嶈骞挎硾浣跨敤鐨勯伩鍏嶅姞閿佺殑鎶€鏈紝鏄负姣忎釜 CPU 澶嶅埗淇℃伅銆備緥濡傦紝濡傛灉浣犳兂涓轰竴涓?
甯歌鏉′欢淇濆瓨涓€涓鏁帮紝浣犲彲浠ョ敤涓€涓嚜鏃嬮攣鍜屼竴涓崟涓€鐨勮鏁板櫒銆傜畝鍗曞張骞插噣銆?

濡傛灉閭ｆ牱澶參锛堥€氬父骞朵笉浼氾紝浣嗗鏋滀綘鏈変竴鍙扮湡姝ｅぇ鍨嬬殑鏈哄櫒鏉ユ祴璇曪紝骞惰兘璇佹槑瀹冪‘瀹?
鎱級锛屼綘鍙互鏀圭敤姣忎釜 CPU 涓€涓鏁板櫒锛岃繖鏍峰畠浠兘涓嶉渶瑕佺嫭鍗犻攣銆傚弬瑙?
DEFINE_PER_CPU()銆乬et_cpu_var() 鍜?put_cpu_var()
锛坄include/linux/percpu.h`锛夈€?

瀵逛簬绠€鍗曠殑姣?CPU 璁℃暟鍣紝鐗瑰埆鏈夌敤鐨勬槸 `local_t` 绫诲瀷锛屼互鍙?cpu_local_inc() 鍜?
鐩稿叧鍑芥暟锛屽湪鏌愪簺浣撶郴缁撴瀯涓婂畠浠瘮绠€鍗曚唬鐮佹洿楂樻晥
锛坄include/asm/local.h`锛夈€?

娉ㄦ剰锛屽湪涓嶅紩鍏ユ洿澶氶攣鐨勬儏鍐典笅锛屾病鏈夌畝鍗曞彲闈犵殑鏂规硶鑳藉緱鍒拌繖鏍蜂竴涓鏁板櫒鐨勭簿纭€笺€?
杩欏鏌愪簺鐢ㄩ€旀潵璇翠笉鏄棶棰樸€?

### 涓昏琚?IRQ 澶勭悊绋嬪簭浣跨敤鐨勬暟鎹?


濡傛灉鏁版嵁鎬绘槸浠庡悓涓€涓?IRQ 澶勭悊绋嬪簭鍐呴儴璁块棶锛屼綘鏍规湰涓嶉渶瑕侀攣锛氬唴鏍稿凡缁忎繚璇佽 irq
澶勭悊绋嬪簭涓嶄細鍦ㄥ涓?CPU 涓婂悓鏃惰繍琛屻€?

Manfred Spraul 鎸囧嚭锛屽嵆浣挎暟鎹瀬灏戞儏鍐典笅鍦ㄧ敤鎴蜂笂涓嬫枃鎴栬蒋涓柇/tasklet 涓璁块棶锛?
浣犱粛鐒跺彲浠ヨ繖鏍峰仛锛?

```

        mutex_lock(&lock);
        disable_irq(irq);
        ...
        enable_irq(irq);
        mutex_unlock(&lock);


```
disable_irq() 闃绘 irq 澶勭悊绋嬪簭杩愯锛堝鏋滃畠姝ｅ湪鍏朵粬 CPU 涓婅繍琛岋紝鍒欑瓑寰呭畠
瀹屾垚锛夈€傝嚜鏃嬮攣闃绘浠讳綍鍏朵粬璁块棶鍚屾椂杩涜銆傝嚜鐒讹紝杩欐瘮鍗曠嫭涓€娆?spin_lock_irq()
璋冪敤瑕佹參锛屽洜姝ゅ彧鏈夊綋杩欑被璁块棶鏋佸叾缃曡鏃舵墠鏈夋剰涔夈€?

## 鍝簺鍑芥暟鍙互浠庝腑鏂腑瀹夊叏璋冪敤锛?


鍐呮牳涓殑璁稿鍑芥暟浼氱潯鐪狅紙鍗崇洿鎺ユ垨闂存帴璋冪敤 schedule()锛夛細浣犵粷涓嶈兘鍦ㄦ寔鏈夎嚜鏃嬮攣
鎴栫鐢ㄦ姠鍗犳椂璋冪敤瀹冧滑銆傝繖涔熸剰鍛崇潃浣犻渶瑕佸浜庣敤鎴蜂笂涓嬫枃涓細浠庝腑鏂皟鐢ㄥ畠浠槸闈炴硶鐨勩€?

### 涓€浜涗細鐫＄湢鐨勫嚱鏁?


涓嬮潰鍒楀嚭浜嗘渶甯歌鐨勪竴浜涳紝浣嗛€氬父浣犲繀椤婚槄璇讳唬鐮佹墠鑳芥悶娓呮鍏朵粬璋冪敤鏄惁瀹夊叏銆傚鏋?
鎵€鏈夊叾浠栬皟鐢ㄥ畠鐨勪汉閮借兘鐫＄湢锛屼綘澶ф涔熼渶瑕佽兘澶熺潯鐪犮€傜壒鍒槸锛屾敞鍐屽拰娉ㄩ攢鍑芥暟閫氬父
鏈熸湜浠庣敤鎴蜂笂涓嬫枃璋冪敤锛屽苟涓斿彲鑳界潯鐪犮€?

- 瀵?userspace 鐨勮闂細

   - copy_from_user()

   - copy_to_user()

   - get_user()

   - put_user()

- kmalloc(GP_KERNEL) <kmalloc>`

- mutex_lock_interruptible() 鍜?mutex_lock()

   鏈変竴涓?mutex_trylock() 涓嶄細鐫＄湢銆傚敖绠″姝わ紝瀹冧笉鑳藉湪涓柇涓婁笅鏂囧唴閮ㄤ娇鐢紝鍥犱负
   瀹冪殑瀹炵幇瀵规骞朵笉瀹夊叏銆俶utex_unlock() 涔熸案杩滀笉浼氱潯鐪犮€傚畠鍚屾牱涓嶈兘鍦ㄤ腑鏂笂涓嬫枃涓?
   浣跨敤锛屽洜涓轰簰鏂ヤ綋蹇呴』鐢辫幏鍙栧畠鐨勫悓涓€涓换鍔℃潵閲婃斁銆?

### 涓€浜涗笉浼氱潯鐪犵殑鍑芥暟


鏈変簺鍑芥暟鍙互瀹夊叏鍦板湪浠讳綍涓婁笅鏂囦腑璋冪敤锛屾垨鑰呮寔鏈夊嚑涔庝换浣曢攣鏃惰皟鐢ㄣ€?

- printk()

- kfree()

- add_timer() 鍜?timer_delete()

## 浜掓枼浣?API 鍙傝€?


   :internal:

   :export:

## Futex API 鍙傝€?


   :internal:

   :internal:

   :internal:

   :internal:

   :internal:

## 寤朵几闃呰


- `Documentation/locking/spinlocks.rst`锛歀inus Torvalds 鍦ㄥ唴鏍告簮鐮佷腑鐨勮嚜鏃嬮攣
   鏁欑▼銆?

- 銆奤nix Systems for Modern Architectures: Symmetric Multiprocessing and
   Caching for Kernel Programmers銆嬶紙鐜颁唬浣撶郴缁撴瀯涓婄殑 Unix 绯荤粺锛氶潰鍚戝唴鏍?
   绋嬪簭鍛樼殑瀵圭О澶氬鐞嗕笌缂撳瓨锛夛細

   Curt Schimmel 瀵瑰唴鏍哥骇鍔犻攣闈炲父濂界殑鍏ラ棬浠嬬粛锛堝苟闈炰负 Linux 鎵€鍐欙紝浣嗗嚑涔庝竴鍒囬兘
   閫傜敤锛夈€傝繖鏈功寰堣吹锛屼絾瑕佺悊瑙?SMP 鍔犻攣锛屾瘡涓€鍒嗛挶閮藉€煎緱銆?
   [ISBN: 0201633388]

## 鑷磋阿


鎰熻阿 Telsa Gwynne 杩涜 DocBooking銆佹暣鐞嗗苟娣诲姞椋庢牸銆?

鎰熻阿 Martin Pool銆丳hilipp Rumpf銆丼tephen Rothwell銆丳aul Mackerras銆丷uedi
Aschwanden銆丄lan Cox銆丮anfred Spraul銆乀im Waugh銆丳ete Zaitcev銆丣ames Morris銆?
Robert Love銆丳aul McKenney銆丣ohn Ashby 杩涜鏍″銆佺籂姝ｃ€佸悙妲藉拰璇勮銆?

鎰熻阿閭ｄ釜绉樺瘑灏忓洟浣撳鏈枃妗ｆ病鏈変骇鐢熶换浣曞奖鍝嶃€?

## 鏈琛?


preemption
  鍦?2.5 涔嬪墠锛屾垨鑰呭綋 `CONFIG_PREEMPT` 鏈缃椂锛屽浜庡唴鏍镐腑鐨勭敤鎴蜂笂涓嬫枃閲岀殑
  杩涚▼涓嶄細鐩镐簰鎶㈠崰锛堝嵆浣犵嫭鍗犻偅涓?CPU锛岀洿鍒颁綘鏀惧純瀹冿紝涓柇闄ゅ锛夈€傞殢鐫€ 2.5.4 涓?
  鍔犲叆 `CONFIG_PREEMPT`锛岃繖涓€鐐规敼鍙樹簡锛氬湪鐢ㄦ埛涓婁笅鏂囦腑鏃讹紝鏇撮珮浼樺厛绾х殑浠诲姟鍙互
  鈥滄彃闃熲€濓細鑷棆閿佽鏀规垚绂佺敤鎶㈠崰锛屽嵆浣垮湪 UP 涓婁篃鏄姝ゃ€?

bh
  搴曞崐閮紙Bottom Half锛夛細鐢变簬鍘嗗彶鍘熷洜锛屽悕绉颁腑甯︽湁 '_bh' 鐨勫嚱鏁扮幇鍦ㄩ€氬父鎸囦唬浠讳綍
  杞欢涓柇锛屼緥濡?spin_lock_bh() 浼氶樆濉炲綋鍓?CPU 涓婄殑浠讳綍杞欢涓柇銆傚簳鍗婇儴宸茶寮冪敤锛?
  骞舵渶缁堜細琚?tasklet 鍙栦唬銆備换鎰忔椂鍒诲彧浼氭湁涓€涓簳鍗婇儴鍦ㄨ繍琛屻€?

Hardware Interrupt / Hardware IRQ
  纭欢涓柇璇锋眰銆俰n_hardirq() 鍦ㄧ‖浠朵腑鏂鐞嗙▼搴忎腑杩斿洖鐪熴€?

Interrupt Context
  闈炵敤鎴蜂笂涓嬫枃锛氬鐞嗕竴涓‖浠?irq 鎴栬蒋浠?irq銆傜敱 in_interrupt() 瀹忚繑鍥炵湡鏉ユ寚绀恒€?

SMP
  瀵圭О澶氬鐞嗗櫒锛圫ymmetric Multi-Processor锛夛細涓哄 CPU 鏈哄櫒缂栬瘧鐨勫唴鏍搞€?
  锛坄CONFIG_SMP=y`锛夈€?

Software Interrupt / softirq
  杞欢涓柇澶勭悊绋嬪簭銆俰n_hardirq() 杩斿洖鍋囷紱in_softirq() 杩斿洖鐪熴€俆asklet 鍜?
  杞腑鏂兘灞炰簬鈥滆蒋浠朵腑鏂€濊繖涓€绫诲埆銆?

  涓ユ牸鏉ヨ锛宻oftirq 鏄嚦澶?32 涓灇涓捐蒋浠朵腑鏂箣涓€锛屽彲浠ュ湪澶氫釜 CPU 涓婂悓鏃惰繍琛屻€?
  鏈夋椂涔熺敤鏉ユ寚浠?tasklet锛堝嵆鎵€鏈夎蒋浠朵腑鏂級銆?

tasklet
  涓€绉嶅彲鍔ㄦ€佹敞鍐岀殑杞欢涓柇锛屼繚璇佷换鎰忔椂鍒诲彧鍦ㄤ竴涓?CPU 涓婅繍琛屻€?

timer
  涓€绉嶅彲鍔ㄦ€佹敞鍐岀殑杞欢涓柇锛屽湪缁欏畾鐨勬椂闂达紙鎴栨帴杩戣鏃堕棿锛夎繍琛屻€傝繍琛屾椂瀹冨氨鍍忎竴涓?
  tasklet锛堜簨瀹炰笂锛屽畠浠槸浠?`TIMER_SOFTIRQ` 璋冪敤鐨勶級銆?

UP
  鍗曞鐞嗗櫒锛圲ni-Processor锛夛細闈?SMP銆傦紙`CONFIG_SMP=n`锛夈€?

User Context
  浠ｈ〃鏌愪釜鐗瑰畾杩涚▼锛堝嵆涓€娆＄郴缁熻皟鐢ㄦ垨闄烽槺锛夋垨鍐呮牳绾跨▼鍦ㄥ唴鏍镐腑鎵ц銆備綘鍙互鐢?
  `current` 瀹忕煡閬撴槸鍝釜杩涚▼銆備笉瑕佷笌 userspace 娣锋穯銆傚畠鍙互琚蒋浠舵垨纭欢涓柇鎵撴柇銆?

Userspace
  杩涚▼鍦ㄨ嚜宸辩殑浠ｇ爜銆佸浜庡唴鏍镐箣澶栨墽琛屻€?

