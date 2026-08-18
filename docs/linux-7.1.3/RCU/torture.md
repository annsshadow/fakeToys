
## RCU 鍘嬪姏娴嬭瘯鎿嶄綔


## CONFIG_RCU_TORTURE_TEST


鎵€鏈?RCU 瀹炵幇閮芥彁渚涗簡 `CONFIG_RCU_TORTURE_TEST` 閰嶇疆閫夐」銆傚畠浼氬垱寤轰竴涓?`rcutorture` 鍐呮牳妯″潡锛屽姞杞借妯″潡鍗冲彲杩愯涓€椤瑰帇鍔涙祴璇曘€傛祴璇曚細鍛ㄦ湡鎬у湴閫氳繃 printk() 杈撳嚭鐘舵€佷俊鎭紝鍙€氳繃 dmesg 鍛戒护锛堟垨璁搁厤鍚?grep "torture"锛夋煡鐪嬨€傛ā鍧楀姞杞芥椂娴嬭瘯寮€濮嬶紝妯″潡鍗歌浇鏃舵祴璇曞仠姝€?

妯″潡鍙傛暟鍦?`Documentation/admin-guide/kernel-parameters.txt` 涓互 "rcutorture." 涓哄墠缂€銆?

## 杈撳嚭


```
	rcu-torture:--- Start of test: nreaders=16 nfakewriters=4 stat_interval=30 verbose=0 test_no_idle_hz=1 shuffle_interval=3 stutter=5 irqreader=1 fqs_duration=0 fqs_holdoff=0 fqs_stutter=3 test_boost=1/0 test_boost_interval=7 test_boost_duration=4
	rcu-torture: rtc:           (null) ver: 155441 tfle: 0 rta: 155441 rtaf: 8884 rtf: 155440 rtmbe: 0 rtbe: 0 rtbke: 0 rtbre: 0 rtbf: 0 rtb: 0 nt: 3055767
	rcu-torture: Reader Pipe:  727860534 34213 0 0 0 0 0 0 0 0 0
	rcu-torture: Reader Batch:  727877838 17003 0 0 0 0 0 0 0 0 0
	rcu-torture: Free-Block Circulation:  155440 155440 155440 155440 155440 155440 155440 155440 155440 155440 0
	rcu-torture:--- End of test: SUCCESS: nreaders=16 nfakewriters=4 stat_interval=30 verbose=0 test_no_idle_hz=1 shuffle_interval=3 stutter=5 irqreader=1 fqs_duration=0 fqs_holdoff=0 fqs_stutter=3 test_boost=1/0 test_boost_interval=7 test_boost_duration=4
```

鍦ㄥぇ澶氭暟绯荤粺涓婏紝鍛戒护 "dmesg | grep torture:" 鍗冲彲鎻愬彇杩欎簺淇℃伅銆傚湪杈冧负鐗规畩鐨勯厤缃笅锛屽彲鑳介渶瑕佷娇鐢ㄥ叾浠栧懡浠ゆ潵璁块棶 RCU 鍘嬪姏娴嬭瘯鎵€鐢ㄧ殑 printk() 杈撳嚭銆傝繖浜?printk() 浣跨敤 KERN_ALERT锛屽洜姝ゅ簲褰撳緢鏄庢樉銆?;-)

棣栬涓庢湯琛屾樉绀轰簡 rcutorture 妯″潡鍙傛暟锛屾湯琛屽垯鏍规嵁 rcutorture 瀵?RCU 鏄惁姝ｇ‘杩愯鐨勮嚜鍔ㄥ垽瀹氾紝鏄剧ず "SUCCESS" 鎴?"FAILURE"銆?

鍚勬潯鐩惈涔夊涓嬶細

- "rtc"锛氬綋鍓嶅璇昏€呭彲瑙佺殑缁撴瀯浣撶殑鍗佸叚杩涘埗鍦板潃銆?

- "ver"锛氳嚜鍚姩浠ユ潵锛孯CU 鍐欒€呬换鍔℃洿鏀硅鑰呭彲瑙佺粨鏋勭殑娆℃暟銆?

- "tfle"锛氳嫢闈為浂锛岃〃绀虹敤浜庢斁鍏?"rtc" 鍖哄煙鐨?"torture freelist"锛堢┖闂查摼琛級涓虹┖銆傛鐘跺喌寰堥噸瑕侊紝鍥犱负瀹冨彲鑳借浣犺浠ヤ负 RCU 鍦ㄥ伐浣滆€屽疄闄呭苟闈炲姝ゃ€?:-/

- "rta"锛氫粠 torture 绌洪棽閾捐〃鍒嗛厤鐨勭粨鏋勬暟閲忋€?

- "rtaf"锛氬洜閾捐〃涓虹┖鑰屼粠 torture 绌洪棽閾捐〃鍒嗛厤澶辫触鐨勬鏁般€傝鍊奸潪闆跺苟涓嶇綍瑙侊紝浣嗚嫢鍗犲埌 "rta" 鎵€鎸囩ず鍊肩殑寰堝ぇ姣斾緥鍒欐槸涓嶅ソ鐨勩€?

- "rtf"锛氶噴鏀惧洖 torture 绌洪棽閾捐〃鐨勬暟閲忋€?

- "rtmbe"锛氶潪闆跺€艰〃绀?rcutorture 璁や负 rcu_assign_pointer() 涓?rcu_dereference() 宸ヤ綔涓嶆甯搞€傝鍊煎簲涓洪浂銆?

- "rtbe"锛氶潪闆跺€艰〃绀?rcu_barrier() 绯诲垪鍑芥暟涔嬩竴宸ヤ綔涓嶆甯搞€?

- "rtbke"锛歳cutorture 鏃犳硶鍒涘缓鐢ㄤ簬寮哄埗 RCU 浼樺厛绾у弽杞殑瀹炴椂 kthread銆傝鍊煎簲涓洪浂銆?

- "rtbre"锛氬敖绠?rcutorture 鎴愬姛鍒涘缓浜嗙敤浜庡己鍒?RCU 浼樺厛绾у弽杞殑 kthread锛屼絾鏃犳硶灏嗗叾璁剧疆涓哄疄鏃朵紭鍏堢骇 1銆傝鍊煎簲涓洪浂銆?

- "rtbf"锛歊CU 浼樺厛绾ф彁鍗囨湭鑳借В鍐?RCU 浼樺厛绾у弽杞殑娆℃暟銆?

- "rtb"锛歳cutorture 灏濊瘯寮哄埗 RCU 浼樺厛绾у弽杞潯浠剁殑娆℃暟銆傝嫢浣犳閫氳繃 "test_boost" 妯″潡鍙傛暟娴嬭瘯 RCU 浼樺厛绾ф彁鍗囷紝璇ュ€煎簲涓洪潪闆躲€?

- "nt"锛歳cutorture 鍦ㄥ畾鏃跺櫒澶勭悊绋嬪簭涓繍琛?RCU 璇荤浠ｇ爜鐨勬鏁般€備粎褰撲綘鎸囧畾浜?"irqreader" 妯″潡鍙傛暟鏃讹紝璇ュ€兼墠搴斾负闈為浂銆?

- "Reader Pipe"锛氳鑰呮墍瑙佺粨鏋勪綋 "age"锛堝勾榫勶級鐨勭洿鏂瑰浘銆傝嫢鍓嶄袱椤逛箣澶栫殑浠讳綍鏉＄洰闈為浂锛屽垯 RCU 宸叉崯鍧忋€俽cutorture 浼氭墦鍗伴敊璇爣蹇楀瓧绗︿覆 "!!!" 浠ョ‘淇濅綘娉ㄦ剰鍒般€傛柊鍒嗛厤缁撴瀯浣撶殑 age 涓洪浂锛屼粠璇昏€呭彲瑙佹€т腑绉婚櫎鏃跺彉涓?1锛屼箣鍚庢瘡缁忚繃涓€涓闄愭湡閫掑涓€娆♀€斺€斿苟鍦ㄧ粡杩?(RCU_TORTURE_PIPE_LEN-2) 涓闄愭湡鍚庤閲婃斁銆?

涓婇潰鏄剧ず鐨勮緭鍑哄彇鑷甯稿伐浣滅殑 RCU銆傝嫢鎯崇湅鐪嬫崯鍧忔椂鐨勬牱瀛愶紝鑷繁寮勫潖瀹冨嵆鍙€?;-)

- "Reader Batch"锛氳鑰呮墍瑙佺粨鏋勪綋 "age" 鐨勫彟涓€浠界洿鏂瑰浘锛屼絾鎸夎鏁板櫒缈昏浆锛堟垨鎵规锛夎€岄潪瀹介檺鏈熸潵缁熻銆傚悎娉曠殑闈為浂鏉＄洰鏁伴噺鍚屾牱涓轰袱涓€備箣鎵€浠ユ彁渚涜繖涓€鐙珛瑙嗗浘锛屾槸鍥犱负鏈夋椂鏇村鏄撹绗笁涓潯鐩嚭鐜板湪 "Reader Batch" 鍒楄〃涓紝鑰岄潪 "Reader Pipe" 鍒楄〃銆?

- "Free-Block Circulation"锛氭樉绀哄埌杈炬祦姘寸嚎涓粰瀹氫綅缃殑 torture 缁撴瀯鏁伴噺銆傜涓€涓厓绱犲簲澶ц嚧瀵瑰簲宸插垎閰嶇殑缁撴瀯鏁伴噺锛岀浜屼釜瀵瑰簲宸蹭粠璇昏€呰鍥剧Щ闄ょ殑鏁伴噺锛屽叾浣欙紙闄ゆ渶鍚庝竴涓锛夊搴旂粡杩囩浉搴旀鏁板闄愭湡鐨勬暟閲忋€傛渶鍚庝竴涓潯鐩簲涓洪浂锛屽洜涓哄畠浠呭湪鏌愪釜 torture 缁撴瀯鐨勮鏁板櫒琚敊璇湴閫掑瓒呰繃搴旀湁鑼冨洿鏃舵墠閫掑銆?

涓嶅悓鐨?RCU 瀹炵幇鍙互鎻愪緵鐗瑰畾浜庡疄鐜扮殑棰濆淇℃伅銆備緥濡傦紝Tree SRCU 鎻愪緵濡備笅鍐呭

```
	srcud-torture: Tree SRCU per-CPU(idx=0): 0(35,-21) 1(-4,24) 2(1,1) 3(-26,20) 4(28,-47) 5(-9,4) 6(-10,14) 7(-14,11) T(1,6)
```

璇ヨ鏄剧ず浜嗘瘡 CPU 璁℃暟鍣ㄧ姸鎬侊紝姝ゅ涓轰娇鐢ㄥ姩鎬佸垎閰嶇殑 srcu_struct 鐨?Tree SRCU锛堝洜姝ゅ墠缂€涓?"srcud-" 鑰岄潪 "srcu-"锛夈€傛嫭鍙蜂腑鐨勬暟瀛楁槸瀵瑰簲 CPU 鐨?"old" 涓?"current" 璁℃暟鍣ㄥ€笺€?idx" 鍊煎皢 "old" 涓?"current" 鍊兼槧灏勫埌搴曞眰鏁扮粍锛屽璋冭瘯寰堟湁鐢ㄣ€傛渶鍚庣殑 "T" 鏉＄洰鍖呭惈璁℃暟鍣ㄧ殑鍚堣鍊笺€?

## 鍦ㄧ壒瀹氬唴鏍告瀯寤轰笂鐨勭敤娉?


鏈夋椂闇€瑕佸湪鐗瑰畾鐨勫唴鏍告瀯寤轰笂瀵?RCU 杩涜鍘嬪姏娴嬭瘯锛屼緥濡傚噯澶囧皢璇ュ唴鏍告瀯寤烘姇鍏ョ敓浜х幆澧冩椂銆傛鏃讹紝鍐呮牳搴斾互 CONFIG_RCU_TORTURE_TEST=m 鏋勫缓锛屼粠鑰屽彲浠ヤ娇鐢?modprobe 鍚姩娴嬭瘯銆佷娇鐢?rmmod 缁堟娴嬭瘯銆?

```
	#!/bin/sh

	modprobe rcutorture
	sleep 3600
	rmmod rcutorture
	dmesg | grep torture:
```

杈撳嚭鍙汉宸ユ鏌ュ叾涓殑 "!!!" 閿欒鏍囧織銆傚綋鐒讹紝涔熷彲浠ョ紪鍐欐洿瀹屽杽鐨勮剼鏈潵鑷姩妫€鏌ユ绫婚敊璇€?rmmod" 鍛戒护浼氬己鍒堕€氳繃 printk() 鎵撳嵃 "SUCCESS"銆?FAILURE" 鎴?"RCU_HOTPLUG" 鎸囩ず銆傚墠涓よ€呬笉瑷€鑷槑锛屾渶鍚庝竴涓〃绀鸿櫧鐒舵病鏈?RCU 澶辫触锛屼絾妫€娴嬪埌浜?CPU 鐑彃鎷旈棶棰樸€?


## 鍦ㄤ富绶氬唴鏍镐笂鐨勭敤娉?


褰撲娇鐢?rcutorture 娴嬭瘯瀵?RCU 鑷韩鐨勬敼鍔ㄦ椂锛屽線寰€鏈夊繀瑕佹瀯寤哄涓唴鏍革紝浠ュ湪鐩稿叧 Kconfig 閫夐」涓庡唴鏍稿惎鍔ㄥ弬鏁扮殑澶ч噺缁勫悎涓嬫祴璇曡鏀瑰姩銆傚湪杩欑鎯呭喌涓嬶紝浣跨敤 modprobe 涓?rmmod 鍙兘鐩稿綋鑰楁椂涓斿鏄撳嚭閿欍€?

鍥犳锛屾彁渚涗簡 `tools/testing/selftests/rcutorture/bin/kvm.sh` 鑴氭湰鐢ㄤ簬 x86銆乤rm64 鍜?powerpc 鐨勪富绾挎祴璇曘€傞粯璁ゆ儏鍐典笅锛屽畠浼氳繍琛?`tools/testing/selftests/rcutorture/configs/rcu/CFLIST` 鎵€鎸囧畾鐨勪竴绯诲垪娴嬭瘯锛屾瘡涓祴璇曞湪瀹㈡埛鏈烘搷浣滅郴缁熶腑杩愯 30 鍒嗛挓锛屼娇鐢ㄨ嚜鍔ㄧ敓鎴愮殑 initrd 鎻愪緵鐨勬渶灏?userspace銆傛祴璇曞畬鎴愬悗锛屼細瀵圭敓鎴愮殑鏋勫缓浜х墿涓庢帶鍒跺彴杈撳嚭杩涜閿欒鍒嗘瀽锛屽苟姹囨€昏繍琛岀粨鏋溿€?

鍦ㄨ緝澶х殑绯荤粺涓婏紝鍙€氳繃鍚?kvm.sh 浼犻€?--cpus 鍙傛暟鏉ュ姞閫?rcutorture 娴嬭瘯銆備緥濡傦紝鍦?64 CPU 鐨勭郴缁熶笂锛?--cpus 43" 浼氫娇鐢ㄦ渶澶?43 涓?CPU 骞跺彂杩愯娴嬭瘯锛岃嚜 v5.4 璧峰彲鍦ㄤ袱鎵瑰唴瀹屾垚鍏ㄩ儴鍦烘櫙锛屽皢瀹屾垚鏃堕棿浠庣害鍏皬鏃剁缉鐭埌绾︿竴灏忔椂锛堜笉鍚瀯寤哄崄鍏釜鍐呮牳鎵€闇€鐨勬椂闂达級銆?--dryrun sched" 鍙傛暟涓嶄細杩愯娴嬭瘯锛岃€屾槸鎸囩ず娴嬭瘯灏嗗浣曡皟搴﹀垎鎵广€傝繖鍦ㄧ‘瀹?--cpus 鍙傛暟搴旀寚瀹氬灏戜釜 CPU 鏃跺緢鏈夌敤銆?

骞堕潪鎵€鏈夋敼鍔ㄩ兘闇€瑕佽繍琛屽叏閮ㄥ満鏅€備緥濡傦紝瀵?Tree SRCU 鐨勬敼鍔ㄥ彲鑳藉彧杩愯 SRCU-N 涓?SRCU-P 鍦烘櫙锛岄€氳繃 kvm.sh 鐨?--configs 鍙傛暟瀹炵幇锛?"--configs 'SRCU-N SRCU-P'"銆傚ぇ鍨嬬郴缁熷彲浠ヨ繍琛屽畬鏁村満鏅泦鐨勫浠藉壇鏈紝渚嬪锛屾嫢鏈?448 涓‖浠剁嚎绋嬬殑绯荤粺鍙繍琛屼簲浠藉疄渚?

```
	kvm.sh --cpus 448 --configs '5*CFLIST'
```

鎴栬€咃紝姝ょ被绯荤粺鍙繍琛屽崟涓満鏅殑 56 涓苟鍙戝疄渚?

```
	kvm.sh --cpus 448 --configs '56*TREE04'
```

```
	kvm.sh --cpus 448 --configs '28*TREE03 28*TREE04'
```

褰撶劧锛屾瘡涓苟鍙戝疄渚嬮兘浼氬崰鐢ㄥ唴瀛橈紝鍙€氳繃 --memory 鍙傛暟闄愬埗锛屽叾榛樿涓?512M銆傝緝灏忕殑鍐呭瓨鍊煎彲鑳介渶瑕佷娇鐢ㄤ笅鏂囪璁虹殑 --bootargs 鍙傛暟绂佺敤鍥炶皟娲硾娴嬭瘯銆?

鏈夋椂棰濆鐨勮皟璇曞緢鏈夌敤锛屾鏃跺彲浣跨敤 kvm.sh 鐨?--kconfig 鍙傛暟锛屼緥濡?`--kconfig 'CONFIG_RCU_EQS_DEBUG=y'`銆傛澶栬繕鏈?--gdb銆?-kasan 鍜?--kcsan 鍙傛暟銆傛敞鎰?--gdb 浼氬皢姣忔 kvm.sh 杩愯闄愬埗涓哄崟涓満鏅紝骞惰姹備綘鎵撳紑鍙︿竴涓獥鍙ｏ紝鎸夌収鑴氭湰鎸囩ず浠庝腑杩愯 `gdb`銆?

涔熷彲浠ユ彁渚涘唴鏍稿惎鍔ㄥ弬鏁帮紝渚嬪鐢ㄤ簬鎺у埗 rcutorture 鐨勬ā鍧楀弬鏁般€備緥濡傦紝瑕佹祴璇曞 RCU CPU stall 璀﹀憡浠ｇ爜鐨勬敼鍔紝鍙娇鐢?"--bootargs 'rcutorture.stall_cpu=30'"銆傝繖褰撶劧浼氬鑷磋剼鏈姤鍛婂け璐ワ紝鍗虫墍浜х敓鐨?RCU CPU stall 璀﹀憡銆傚涓婃墍杩帮紝鍑忓皯鍐呭瓨鍙兘

```
	kvm.sh --cpus 448 --configs '56*TREE04' --memory 128M \
		--bootargs 'rcutorture.fwd_progress=0'
```

鏈夋椂鎵€闇€鐨勫彧鏄畬鏁寸殑涓€缁勫唴鏍告瀯寤恒€傝繖姝ｆ槸 --buildonly 鍙傛暟鐨勪綔鐢ㄣ€?

--duration 鍙傛暟鍙鐩栭粯璁ょ殑 30 鍒嗛挓杩愯鏃堕棿銆備緥濡傦紝`--duration 2d` 杩愯涓ゅぉ锛宍--duration 3h` 杩愯涓夊皬鏃讹紝`--duration 5m` 杩愯浜斿垎閽燂紝`--duration 45s` 杩愯 45 绉掋€傛渶鍚庤繖涓€椤瑰杩借釜缃曡鐨勫惎鍔ㄦ湡澶辫触寰堟湁鐢ㄣ€?

鏈€鍚庯紝--trust-make 鍙傛暟鍏佽姣忎釜鍐呮牳鏋勫缓澶嶇敤涓婁竴涓唴鏍告瀯寤轰腑鍙鐢ㄧ殑鍐呭銆傝娉ㄦ剰锛岃嫢涓嶄娇鐢?--trust-make 鍙傛暟锛屼綘鐨?tags 鏂囦欢鍙兘浼氳娓呴櫎銆?

kvm.sh 鑴氭湰鐨勬簮浠ｇ爜涓繕璁板綍浜嗗叾浠栨洿涓洪殣绉樼殑鍙傛暟銆?

濡傛灉鏌愭杩愯鍖呭惈澶辫触锛屾瀯寤烘湡涓庤繍琛屾湡澶辫触鐨勬暟閲忎細鍒楀湪 kvm.sh 杈撳嚭鐨勬湯灏撅紝浣犵‘瀹炲簲褰撳皢鍏堕噸瀹氬悜鍒版枃浠躲€傛瘡娆¤繍琛岀殑鏋勫缓浜х墿涓庢帶鍒跺彴杈撳嚭淇濆瓨鍦?`tools/testing/selftests/rcutorture/res` 涓殑甯︽椂闂存埑鐩綍閲屻€傚彲灏嗘煇涓洰褰曟彁渚涚粰 kvm-find-errors.sh 浠?

```
	tools/testing/selftests/rcutorture/bin/kvm-find-errors.sh \
		tools/testing/selftests/rcutorture/res/2020.01.20-15.54.23
```

涓嶈繃锛岀洿鎺ヨ闂繖浜涙枃浠堕€氬父鏇存柟渚裤€備笌鏌愭杩愯涓墍鏈夊満鏅浉鍏崇殑鏂囦欢浣嶄簬椤跺眰鐩綍锛堜笂渚嬩腑鐨?2020.01.20-15.54.23锛夛紝鑰屼笌鍗曚釜鍦烘櫙鐩稿叧鐨勬枃浠朵綅浜庝互璇ュ満鏅懡鍚嶇殑瀛愮洰褰曚腑锛堜緥濡?"TREE04"锛夈€傝嫢鏌愪釜鍦烘櫙杩愯浜嗗娆★紙濡備笂渚嬩腑鐨?"--configs '56*TREE04'"锛夛紝瀵瑰簲绗簩娆″強鍚庣画杩愯鐨勭洰褰曚細鍖呭惈搴忓彿锛屼緥濡?"TREE04.2"銆?TREE04.3" 绛夈€?

椤跺眰鐩綍涓渶甯哥敤鐨勬枃浠舵槸 testid.txt銆傚鏋滄祴璇曡繍琛屼簬 git 浠撳簱涓紝鍒欒鏂囦欢鍖呭惈琚祴璇曠殑 commit 浠ュ強浠讳綍浠?diff 鏍煎紡瀛樺湪鐨勬湭鎻愪氦鏀瑰姩銆?

姣忎釜鍗曞満鏅繍琛岀洰褰曚腑鏈€甯哥敤鐨勬枃浠舵湁锛?

.config:
	璇ユ枃浠跺寘鍚?Kconfig 閫夐」銆?

Make.out:
	璇ユ枃浠跺寘鍚壒瀹氬満鏅殑鏋勫缓杈撳嚭銆?

console.log:
	璇ユ枃浠跺寘鍚壒瀹氬満鏅殑鎺у埗鍙拌緭鍑恒€傚唴鏍稿惎鍔ㄥ悗鍙緵鏌ョ湅锛屼絾濡傛灉鏋勫缓澶辫触瀹冨彲鑳戒笉瀛樺湪銆?

vmlinux:
	璇ユ枃浠跺寘鍚唴鏍革紝鍙笌 objdump 鍜?gdb 绛夊伐鍏烽厤鍚堜娇鐢ㄣ€?

杩樻湁鍏朵粬鑻ュ共鏂囦欢鍙敤锛屼絾浣跨敤棰戠巼杈冧綆銆傝澶氫笓涓鸿皟璇?rcutorture 鑷韩鎴栧叾鑴氭湰鑰岃銆?

鑷?v5.4 璧凤紝浣跨敤榛樿鍦烘櫙闆嗙殑鎴愬姛杩愯浼氫骇鐢?

```
    SRCU-N ------- 804233 GPs (148.932/s) [srcu: g10008272 f0x0 ]
    SRCU-P ------- 202320 GPs (37.4667/s) [srcud: g1809476 f0x0 ]
    SRCU-t ------- 1122086 GPs (207.794/s) [srcu: g0 f0x0 ]
    SRCU-u ------- 1111285 GPs (205.794/s) [srcud: g1 f0x0 ]
    TASKS01 ------- 19666 GPs (3.64185/s) [tasks: g0 f0x0 ]
    TASKS02 ------- 20541 GPs (3.80389/s) [tasks: g0 f0x0 ]
    TASKS03 ------- 19416 GPs (3.59556/s) [tasks: g0 f0x0 ]
    TINY01 ------- 836134 GPs (154.84/s) [rcu: g0 f0x0 ] n_max_cbs: 34198
    TINY02 ------- 850371 GPs (157.476/s) [rcu: g0 f0x0 ] n_max_cbs: 2631
    TREE01 ------- 162625 GPs (30.1157/s) [rcu: g1124169 f0x0 ]
    TREE02 ------- 333003 GPs (61.6672/s) [rcu: g2647753 f0x0 ] n_max_cbs: 35844
    TREE03 ------- 306623 GPs (56.782/s) [rcu: g2975325 f0x0 ] n_max_cbs: 1496497
    CPU count limited from 16 to 12
    TREE04 ------- 246149 GPs (45.5831/s) [rcu: g1695737 f0x0 ] n_max_cbs: 434961
    TREE05 ------- 314603 GPs (58.2598/s) [rcu: g2257741 f0x2 ] n_max_cbs: 193997
    TREE07 ------- 167347 GPs (30.9902/s) [rcu: g1079021 f0x0 ] n_max_cbs: 478732
    CPU count limited from 16 to 12
    TREE09 ------- 752238 GPs (139.303/s) [rcu: g13075057 f0x0 ] n_max_cbs: 99011
```

## 閲嶅杩愯


鍋囪浣犳鍦ㄨ拷韪竴涓綍瑙佺殑鍚姩鏈熷け璐ャ€傚敖绠″彲浠ヤ娇鐢?kvm.sh锛屼絾杩欐牱姣忔杩愯閮戒細閲嶆柊鏋勫缓鍐呮牳銆傚鏋滀綘闇€瑕侊紙姣斿锛夎繍琛?1,000 娆′互纭俊宸蹭慨澶嶈 bug锛岃繖浜涙棤鎰忎箟鐨勯噸寤轰細鍙樺緱鏋佸叾鐑︿汉銆?

杩欐鏄?kvm-again.sh 瀛樺湪鐨勫師鍥犮€?

```
	tools/testing/selftests/rcutorture/res/2022.11.03-11.26.28
```

```
	kvm-again.sh tools/testing/selftests/rcutorture/res/2022.11.03-11.26.28
```

鍙互瑕嗙洊鍘熷杩愯鐨勯儴鍒?kvm.sh 鍙傛暟锛屼緥濡?

```
	kvm-again.sh tools/testing/selftests/rcutorture/res/2022.11.03-11.26.28 \
		--duration 45s
```

灏嗛噸鏂拌繍琛屼箣鍓嶇殑娴嬭瘯锛屼絾鍙繍琛?45 绉掞紝浠庤€屼究浜庤拷韪墠杩扮綍瑙佺殑鍚姩鏈熷け璐ャ€?


## 鍒嗗竷寮忚繍琛?


灏界 kvm.sh 鐩稿綋鏈夌敤锛屼絾鍏舵祴璇曞眬闄愪簬鍗曚竴绯荤粺銆備娇鐢ㄤ綘鍠滄鐨勬鏋惰锛堟瘮濡傦級5 涓?kvm.sh 瀹炰緥鍦ㄤ綘鐨?5 涓郴缁熶笂杩愯骞朵笉绠楅毦锛屼絾杩欐瀬鏈夊彲鑳戒細涓嶅繀瑕佸湴閲嶅缓鍐呮牳銆傛澶栵紝鎵嬪姩灏嗘墍闇€鐨?rcutorture 鍦烘櫙鍒嗗竷鍒板彲鐢ㄧ郴缁熶笂鏃㈣垂鍔涘張瀹规槗鍑洪敊銆?

杩欐鏄?kvm-remote.sh 鑴氭湰瀛樺湪鐨勫師鍥犮€?

```
	ssh system0 date
```

濡傛灉瀹冨 system1銆乻ystem2銆乻ystem3銆乻ystem4 鍜?system5 涔熸湁鏁堬紝

```
	kvm-remote.sh "system0 system1 system2 system3 system4 system5" \
		--cpus 64 --duration 8h --configs "5*CFLIST"
```

杩欏皢鍦ㄦ湰鍦扮郴缁熶笂鏋勫缓姣忎釜榛樿鍦烘櫙鐨勫唴鏍革紝鐒跺悗灏嗘瘡涓満鏅殑浜斾釜瀹炰緥鍒嗗竷鍒版墍鍒楃郴缁熶笂锛屾瘡涓満鏅繍琛屽叓灏忔椂銆傝繍琛岀粨鏉熸椂锛岀粨鏋滀細琚敹闆嗐€佽褰曞苟鎵撳嵃銆俴vm.sh 鍙帴鍙楃殑澶ч儴鍒嗗弬鏁伴兘鍙紶閫掔粰 kvm-remote.sh锛屼絾绯荤粺鍒楄〃蹇呴』鏀惧湪鏈€鍓嶃€?

kvm.sh 鐨?`--dryrun scenarios` 鍙傛暟鏈夊姪浜庣‘瀹氬湪涓€缁勭郴缁熶笂涓€鎵瑰彲杩愯澶氬皯涓満鏅€?

```
	kvm-remote.sh "system0 system1 system2 system3 system4 system5" \
		tools/testing/selftests/rcutorture/res/2022.11.03-11.26.28-remote \
		--duration 24h
```

鍦ㄨ繖绉嶆儏鍐典笅锛屽ぇ澶氭暟 kvm-again.sh 鍙傛暟鍙湪鏃ц繍琛岀粨鏋滅洰褰曠殑璺緞鍚嶄箣鍚庢彁渚涖€?
