
## Intel(R) Speed Select Technology 鐢ㄦ埛鎸囧崡


Intel(R) Speed Select Technology锛圛ntel(R) SST锛夋彁渚涗簡涓€缁勫己澶х殑鏂扮壒鎬э紝
鍙 CPU 鎬ц兘杩涜鏇寸簿缁嗙殑鎺у埗銆傚€熷姪 Intel(R) SST锛屼竴鍙版湇鍔″櫒鍙互鏍规嵁鍚勭鍚勬牱涓嶅悓鐨?宸ヤ綔璐熻浇闇€姹傦紝涓哄姛鑰椾笌鎬ц兘杩涜閰嶇疆銆?
璇峰弬闃呬互涓嬮摼鎺ヤ互姒傝璇ユ妧鏈細

- https://www.intel.com/content/www/us/en/architecture-and-technology/speed-select-technology-article.html
- https://builders.intel.com/docs/networkbuilders/intel-speed-select-technology-base-frequency-enhancing-performance.pdf

杩欎簺鑳藉姏鍦ㄤ竴浜涜緝鏂颁竴浠ｇ殑鏈嶅姟鍣ㄥ钩鍙颁腑寰楀埌浜嗚繘涓€姝ュ寮猴紝鍦ㄨ繖浜涘钩鍙颁笂锛屾棤闇€閫氳繃 BIOS 璁剧疆閫夐」棰勫厛閰嶇疆锛?鍗冲彲鍔ㄦ€佸湴鏋氫妇骞舵帶鍒惰繖浜涚壒鎬с€傝繖绉嶅姩鎬侀厤缃槸閫氳繃鍚戠‖浠跺彂閫侀偖绠卞懡浠ゅ畬鎴愮殑銆?鏋氫妇骞堕厤缃繖浜涚壒鎬х殑涓€绉嶆柟寮忔槸浣跨敤 Intel Speed Select 宸ュ叿銆?
鏈枃妗ｈВ閲婂浣曚娇鐢?Intel Speed Select 宸ュ叿鏉ユ灇涓惧苟鎺у埗 Intel(R) SST 鐗规€с€?鏈枃妗ｇ粰鍑虹ず渚嬪懡浠わ紝骞惰В閲婅繖浜涘懡浠ゅ浣曟敼鍙樿娴嬬郴缁熶笅鐨勫姛鑰椾笌鎬ц兘profile銆?浠ヨ繖涓伐鍏蜂负渚嬶紝瀹㈡埛鍙互鍦ㄤ粬浠殑鐢熶骇杞欢涓鐜拌宸ュ叿涓墍瀹炵幇鐨勬秷鎭氦浜掋€?
## intel-speed-select 閰嶇疆宸ュ叿


澶у鏁?Linux 鍙戣鐗堣蒋浠跺寘鍙兘鍖呭惈 "intel-speed-select" 宸ュ叿銆傚鏋滄病鏈夛紝
鍙互閫氳繃浠?kernel.org 涓嬭浇 Linux 鍐呮牳鏍戞潵鏋勫缓瀹冦€備笅杞戒箣鍚庯紝鏃犻渶鏋勫缓瀹屾暣鍐呮牳鍗冲彲鏋勫缓璇ュ伐鍏枫€?
```

```
# cd tools/power/x86/intel-speed-select/
# make
# make install

### 鑾峰彇甯姪


```

```
# intel-speed-select --help

top-level 甯姪鎻忚堪浜嗗弬鏁颁笌鐗规€с€傛敞鎰忚繕鏈変竴涓?```

```
# intel-speed-select perf-profile --help

```
# intel-speed-select perf-profile info --help

### 骞冲彴鑳藉姏鎽樿

```

```
#intel-speed-select --info
```

```
 # intel-speed-select --info
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 Platform: API version : 1
 Platform: Driver version : 1
 Platform: mbox supported : 1
 Platform: mmio supported : 1
 Intel(R) SST-PP (feature perf-profile) is supported
 TDP level change control is unlocked, max level: 4
 Intel(R) SST-TF (feature turbo-freq) is supported
 Intel(R) SST-BF (feature base-freq) is not supported
 Intel(R) SST-CP (feature core-power) is supported

```
### Intel(R) Speed Select Technology - Performance Profile锛圛ntel(R) SST-PP锛?

杩欎釜鐗规€у厑璁稿熀浜庡伐浣滆礋杞芥€ц兘闇€姹傚姩鎬佸湴閰嶇疆涓€鍙版湇鍔″櫒銆傝繖鍦ㄩ儴缃叉椂甯姪鐢ㄦ埛锛?鍥犱负浠栦滑涓嶅繀闈欐€佸湴閫夋嫨鏌愪釜鐗瑰畾鐨勬湇鍔″櫒閰嶇疆銆傝繖涓?Intel(R) Speed Select Technology -
Performance Profile锛圛ntel(R) SST-PP锛夌壒鎬у紩鍏ヤ簡涓€绉嶆満鍒讹紝鍏佽姣忎釜绯荤粺鏈夊涓紭鍖栬繃鐨勬€ц兘profile銆?姣忎釜 profile 瀹氫箟浜嗕竴缁勯渶瑕佸湪绾裤€佸叾浣欑绾跨殑 CPU锛屼互缁存寔涓€涓湁淇濊瘉鐨勫熀鍑嗛鐜囥€?涓€鏃︾敤鎴峰彂鍑哄懡浠や互浣跨敤鏌愪釜鐗瑰畾鐨勬€ц兘profile锛屽苟婊¤冻 CPU 鍦ㄧ嚎/绂荤嚎鐨勮姹傦紝鐢ㄦ埛灏卞彲浠ラ鏈熷熀鍑嗛鐜囦細鍔ㄦ€佸湴鏀瑰彉銆?鍦ㄤ娇鐢?Intel Speed Select 宸ュ叿鏃讹紝杩欎釜鐗规€ц绉颁负 "perf-profile"銆?
#### Number or performance levels


涓€涓郴缁熶笂鍙互鏈夊涓€ц兘profile銆傝鑾峰彇鎬ц兘
```

 # intel-speed-select perf-profile get-config-levels
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
        get-config-levels:4
 package-1
  die-0
    cpu-14
        get-config-levels:4

```
鍦ㄨ繖涓娴嬬郴缁熶笂锛岄櫎浜嗗熀纭€鎬ц兘profile锛堝嵆鎬ц兘绾у埆 0锛変箣澶栵紝杩樻湁 4 涓€ц兘profile銆?
#### 閿佸畾/瑙ｉ攣鐘舵€?

鍗充究鏈夊涓€ц兘profile锛屽畠浠篃鏈夊彲鑳芥槸琚攣瀹氱殑銆傚鏋滃畠浠閿佸畾锛岀敤鎴峰氨鏃犳硶鍙戝嚭鍛戒护鏉ユ敼鍙樻€ц兘鐘舵€併€?鏈夊彲鑳藉瓨鍦ㄦ煇涓?BIOS 璁剧疆鍙互瑙ｉ攣锛屾垨鑰呭挩璇綘鐨勭郴缁熶緵搴斿晢銆?
```

 # intel-speed-select perf-profile get-lock-status
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
        get-lock-status:0
 package-1
  die-0
    cpu-14
        get-lock-status:0

```
鍦ㄨ繖绉嶆儏鍐典笅锛岄攣瀹氱姸鎬佷负 0锛屾剰鍛崇潃绯荤粺澶勪簬瑙ｉ攣鐘舵€併€?
#### 鎬ц兘绾у埆鐨勫睘鎬?

```

 # intel-speed-select perf-profile info -l 0
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      perf-profile-level-0
        cpu-count:28
        enable-cpu-mask:000003ff,f0003fff
        enable-cpu-list:0,1,2,3,4,5,6,7,8,9,10,11,12,13,28,29,30,31,32,33,34,35,36,37,38,39,40,41
        thermal-design-power-ratio:26
        base-frequency(MHz):2600
        speed-select-turbo-freq:disabled
        speed-select-base-freq:disabled
	...
	...

```
杩欓噷 -l 閫夐」鐢ㄤ簬鎸囧畾涓€涓€ц兘绾у埆銆?
濡傛灉鐪佺暐 -l 閫夐」锛岄偅涔堣繖涓懡浠ゅ皢鎵撳嵃鎵€鏈夋€ц兘绾у埆鐨勪俊鎭€備笂闈㈢殑鍛戒护鎵撳嵃鐨勬槸鎬ц兘绾у埆 0 鐨勫睘鎬с€?
瀵逛簬杩欎釜鎬ц兘profile锛岀敱 "enable-cpu-mask/enable-cpu-list" 鏄剧ず鐨勬渶澶?CPU 鍒楄〃鍙互 "online"銆?褰撴弧瓒宠繖涓潯浠舵椂锛屽氨鍙互缁存寔 2600 MHz 鐨勫熀鍑嗛鐜囥€傛兂浜嗚В鏇村锛岃鎵ц
"intel-speed-select perf-profile info" 浠ユ煡鐪嬫€ц兘
```

 # intel-speed-select perf-profile info -l 4
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      perf-profile-level-4
        cpu-count:28
        enable-cpu-mask:000000fa,f0000faf
        enable-cpu-list:0,1,2,3,5,7,8,9,10,11,28,29,30,31,33,35,36,37,38,39
        thermal-design-power-ratio:28
        base-frequency(MHz):2800
        speed-select-turbo-freq:disabled
        speed-select-base-freq:unsupported
	...
	...

```
"enable-cpu-mask/enable-cpu-list" 涓殑 CPU 鏇村皯銆傚洜姝わ紝濡傛灉鐢ㄦ埛鍙皢杩欎簺 CPU 淇濇寔鍦ㄧ嚎锛?鑰屽皢鍏朵綑鐨?"offline"锛岄偅涔堝熀鍑嗛鐜囧氨浼氫粠鎬ц兘绾у埆 0 鏃剁殑 2.6 GHz 鎻愬崌鍒?2.8 GHz銆?
#### 鑾峰彇褰撳墠鎬ц兘绾у埆


```

 # intel-speed-select perf-profile get-config-current-level
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
        get-config-current_level:0

```

```

 # cat /sys/devices/system/cpu/cpu0/cpufreq/base_frequency
 2600000

```
杩欎笌浠?"perf-profile info" 鍛戒护涓烘€ц兘绾у埆 0 鏄剧ず鐨?base-frequency (MHz) 瀛楁鍊肩浉鍖归厤
锛坈pufreq 棰戠巼鐨勫崟浣嶆槸 KHz锛夈€?
瑕佹鏌ュ钩鍧囬鐜囨槸鍚︾瓑浜?100% 绻佸繖鏃剁殑鍩哄噯棰戠巼锛屽彲浠?```

```
# echo 1 > /sys/devices/system/cpu/intel_pstate/no_turbo
```

```
#stress -c 64
```

 #turbostat -c 0-13 --show Package,Core,CPU,Bzy_MHz -i 1

  Package	Core	CPU	Bzy_MHz
		-	-	2600
  0		0	0	2600
  0		1	1	2600
  0		2	2	2600
  0		3	3	2600
  0		4	4	2600
  .		.	.	.


```
#### 鏀瑰彉鎬ц兘绾у埆


```

 # intel-speed-select -d perf-profile set-config-level -l 4 -o
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      perf-profile
        set_tdp_level:success

```
鍦ㄤ笂闈㈢殑鍛戒护涓紝"-o" 鏄彲閫夌殑銆傚鏋滄寚瀹氫簡瀹冿紝閭ｄ箞瀹冭繕浼氳涓嶅湪杩欎釜鎬ц兘绾у埆鐨?enable_cpu_mask 涓殑 CPU 绂荤嚎銆?
```

 #cat /sys/devices/system/cpu/cpu0/cpufreq/base_frequency
 2800000

```
杩欒〃鏄庡熀鍑嗛鐜囩幇鍦ㄤ粠鎬ц兘绾у埆 0 鏃剁殑 2600 MHz 鎻愬崌鍒颁簡鎬ц兘绾у埆 4 鏃剁殑 2800 MHz銆?缁撴灉灏辨槸锛屼换浣曡兘澶熶娇鐢ㄦ洿灏?CPU 鐨勫伐浣滆礋杞斤紝鐩告瘮鎬ц兘绾у埆 0 閮藉彲浠ョ湅鍒?200 MHz 鐨勬彁鍗囥€?
#### 閫氳繃 BMC 鎺ュ彛鏀瑰彉鎬ц兘绾у埆


鍙互浣跨敤甯﹀锛圤OB锛変唬鐞嗭紙閫氳繃鏌愪簺杩滅▼绠＄悊鎺у埗鍙帮紝缁忕敱 BMC "Baseboard Management Controller"
鍩烘澘绠＄悊鎺у埗鍣ㄦ帴鍙ｏ級鏉ユ敼鍙?SST-PP 绾у埆銆傝繖绉嶆ā寮忎粠 Sapphire Rapids 澶勭悊鍣ㄤ唬寮€濮嬫敮鎸併€?鏀寔杩欑妯″紡鐨?鍐呮牳涓庡伐鍏锋敼鍔ㄨ鍔犲叆鍒颁簡 Linux 鍐呮牳 5.18 鐗堟湰銆傝鍚敤杩欎釜鐗规€э紝闇€瑕佸唴鏍搁厤缃?"CONFIG_INTEL_HFI_THERMAL"銆傛敮鎸佽繖涓壒鎬х殑宸ュ叿鐨勬渶浣庣増鏈槸 "v1.12"锛屽畠鏄?Linux 鍐呮牳 5.18 鐗堟湰鐨勪竴閮ㄥ垎銆?
涓轰簡鏀寔杩欐牱鐨勯厤缃紝杩欎釜宸ュ叿鍙互浣滀负瀹堟姢杩涚▼浣跨敤銆傛坊鍔?```

 # intel-speed-select --oob
 Intel(R) Speed Select Technology
 Executing on CPU model:143[0x8f]
 OOB mode is enabled and will run as daemon

```
鍦ㄨ繖绉嶆ā寮忎笅锛岃宸ュ叿灏嗘牴鎹柊鐨勬€ц兘绾у埆鏉ヤ娇 CPU 鍦ㄧ嚎/绂荤嚎銆?
### 妫€鏌ュ叾浠?Intel(R) SST 鐗规€х殑瀛樺湪


姣忎釜鎬ц兘profile 涔熸寚鏄庝簡鏄惁鏀寔鍙﹀涓や釜 Intel(R) SST 鐗规€э紙Intel(R) Speed Select Technology -
Base Frequency锛圛ntel(R) SST-BF锛変笌 Intel(R) Speed Select Technology - Turbo Frequency锛圛ntel(R) SST-TF锛夛級銆?
渚嬪锛屼粠涓婇潰鐨?"perf-profile info" 杈撳嚭涓紝瀵逛簬绾у埆 0 涓庣骇鍒?4锛?
```

       speed-select-turbo-freq:disabled
       speed-select-base-freq:disabled

```

```
       speed-select-turbo-freq:disabled
       speed-select-base-freq:unsupported

```
閴翠簬杩欎簺缁撴灉锛岀浉姣旀€ц兘绾у埆 0锛屽湪绾у埆 4 涓?"speed-select-base-freq"锛圛ntel(R) SST-BF锛変粠 "disabled" 鍙樻垚浜?"unsupported"銆?
杩欐剰鍛崇潃锛屽湪鎬ц兘绾у埆 4 鏃讹紝"speed-select-base-freq" 鐗规€т笉琚敮鎸併€傜劧鑰岋紝鍦ㄦ€ц兘绾у埆 0 鏃讹紝杩欎釜鐗规€ф槸 "supported"锛?浣嗗綋鍓?"disabled"锛屾剰鍛崇潃鐢ㄦ埛灏氭湭婵€娲昏繖涓壒鎬с€傝€?"speed-select-turbo-freq"锛圛ntel(R) SST-TF锛夊湪涓や釜鎬ц兘绾у埆閮藉彈鏀寔锛?浣嗗綋鍓嶆湭琚敤鎴锋縺娲汇€?
Intel(R) SST-BF 涓?Intel(R) SST-TF 鐗规€ф瀯寤哄湪涓€涓绉颁负 Intel(R) Speed Select Technology -
Core Power锛圛ntel(R) SST-CP锛夌殑鍩虹鎶€鏈箣涓娿€傚綋骞冲彴涓婃敮鎸?Intel(R) SST-BF 鎴?Intel(R) SST-TF 鏃讹紝
骞冲彴鍥轰欢浼氬惎鐢ㄨ繖涓壒鎬с€?
### Intel(R) Speed Select Technology Core Power锛圛ntel(R) SST-CP锛?

Intel(R) Speed Select Technology Core Power锛圛ntel(R) SST-CP锛夋槸涓€涓厑璁哥敤鎴峰畾涔夋瘡鏍镐紭鍏堢骇鐨勬帴鍙ｃ€?杩欏畾涔変簡涓€绉嶅湪瀛樺湪鍔熻€楀彈闄愬満鏅椂鍦ㄦ牳涔嬮棿鍒嗛厤鍔熻€楃殑鏈哄埗銆傝繖瀹氫箟浜嗕竴绉嶆湇鍔＄瓑绾э紙CLOS锛夐厤缃€?
鐢ㄦ埛鍙互閰嶇疆澶氳揪 4 涓湇鍔＄瓑绾ч厤缃€傛瘡涓?CLOS 缁勯厤缃厑璁稿畾涔変竴浜涘弬鏁帮紝杩欎簺鍙傛暟褰卞搷棰戠巼濡備綍琚檺鍒朵互鍙婂姛鑰楀浣曡鍒嗛厤銆?姣忎釜 CPU 鏍搁兘鍙互缁戝畾鍒颁竴涓湇鍔＄瓑绾э紝浠庤€屽叧鑱斿埌鐩稿簲鐨勪紭鍏堢骇銆傜矑搴︽槸鏍哥骇鍒紝鑰岄潪姣忎釜 CPU 绾у埆銆?
#### 鍚敤鍩轰簬 CLOS 鐨勪紭鍏堢骇鎺掑簭


瑕佷娇鐢ㄥ熀浜?CLOS 鐨勪紭鍏堢骇鎺掑簭鐗规€э紝蹇呴』鍛婄煡鍥轰欢鍚敤骞朵娇鐢ㄦ煇绉嶄紭鍏堢骇绫诲瀷銆傛瘡涓钩鍙版湁涓€涓粯璁ょ殑浼樺厛绾х被鍨嬶紝
瀹冨彲浠ラ€氳繃鍙€夌殑鍛戒护琛屽弬鏁版敼鍙樸€?
```

 # intel-speed-select core-power enable --help
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 Enable core-power for a package/die
	Clos Enable: Specify priority type with [--priority|-p]
		 0: Proportional, 1: Ordered

```
鏈変袱绉嶄紭鍏堢骇绫诲瀷锛?
- Ordered

Ordered 鑺傛祦锛坱hrottling锛夌殑浼樺厛绾ф槸鏍规嵁鎵€鍒嗛厤鐨?CLOS 缁勭殑绱㈠紩瀹氫箟鐨勩€傚叾涓?CLOS0 鑾峰緱鏈€楂樹紭鍏堢骇锛堟渶鍚庤鑺傛祦锛夈€?
浼樺厛绾ч『搴忔槸锛?CLOS0 > CLOS1 > CLOS2 > CLOS3銆?
- Proportional

褰撲娇鐢ㄦ瘮渚嬶紙proportional锛変紭鍏堢骇鏃讹紝鏈変竴涓澶栫殑鍙傛暟鍙仛 frequency_weight锛屽畠鍙互閽堝姣忎釜 CLOS 缁勬寚瀹氥€?姣斾緥浼樺厛绾х殑鐩爣鏄鍏堜负姣忎釜鏍告彁渚涙墍璇锋眰鐨勬渶灏忓€硷紝鐒跺悗鎸夌収瀹氫箟鐨勬潈閲嶆垚姣斾緥鍦板垎閰嶆墍鏈夊墿浣欙紙鐩堜綑/浜忕┖锛夌殑棰勭畻銆?杩欎釜姣斾緥浼樺厛绾у彲浠ヤ娇鐢?"core-power config" 鍛戒护鏉ラ厤缃€?
```

 # intel-speed-select core-power enable
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      core-power
        enable:success
 package-1
  die-0
    cpu-6
      core-power
        enable:success

```
杩欎釜鍚敤鐨勮寖鍥村湪姣忎釜 package 鍖呭惈澶氫釜 die 鏃舵槸 per package 鎴?die 鑼冨洿鐨勩€?瑕佹鏌?CLOS 鏄惁鍚敤骞惰幏鍙栦紭鍏堢骇绫诲瀷锛屽彲浠ヤ娇鐢?"core-power info" 鍛戒护銆?渚嬪锛岃妫€鏌?core-power 鐗规€х殑鐘舵€侊紝
```

 # intel-speed-select -c 0 core-power info
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      core-power
        support-status:supported
        enable-status:enabled
        clos-enable-status:enabled
        priority-type:proportional
 package-1
  die-0
    cpu-24
      core-power
        support-status:supported
        enable-status:enabled
        clos-enable-status:enabled
        priority-type:proportional

```
#### 閰嶇疆 CLOS 缁?

姣忎釜 CLOS 缁勯兘鏈夎嚜宸辩殑灞炴€э紝鍖呮嫭 min銆乵ax銆乫req_weight 涓?desired銆?杩欎簺鍙傛暟鍙互鐢?"core-power config" 鍛戒护鏉ラ厤缃€傚鏋滅敤鎴疯烦杩囦簡璁剧疆鏌愪釜鍙傛暟锛堥櫎浜?clos id 涔嬪锛夛紝
灏嗕娇鐢ㄩ粯璁ゅ€硷紝clos id 鏄?```

 # intel-speed-select core-power config --help
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 Set core-power configuration for one of the four clos ids
	Specify targeted clos id with [--clos|-c]
	Specify clos Proportional Priority [--weight|-w]
	Specify clos min in MHz with [--min|-n]
	Specify clos max in MHz with [--max|-m]

```

```

 # intel-speed-select core-power config -c 0
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 clos epp is not specified, default: 0
 clos frequency weight is not specified, default: 0
 clos min is not specified, default: 0 MHz
 clos max is not specified, default: 25500 MHz
 clos desired is not specified, default: 0
 package-0
  die-0
    cpu-0
      core-power
        config:success
 package-1
  die-0
    cpu-6
      core-power
        config:success

```
鐢ㄦ埛鍙互閫夋嫨鏀瑰彉榛樿鍊笺€備緥濡傦紝鐢ㄦ埛鍙互鏀瑰彉 "min" 骞跺皢鍩哄噯棰戠巼璁句负鎬昏兘鑾峰緱鏈変繚璇佺殑鍩哄噯棰戠巼銆?
#### 鑾峰彇褰撳墠 CLOS 閰嶇疆


瑕佹鏌ュ綋鍓嶉厤缃紝鍙互浣跨敤 "core-power get-config"銆傚浜?```

 # intel-speed-select core-power get-config -c 0
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      core-power
        clos:0
        epp:0
        clos-proportional-priority:0
        clos-min:0 MHz
        clos-max:Max Turbo frequency
        clos-desired:0 MHz
 package-1
  die-0
    cpu-24
      core-power
        clos:0
        epp:0
        clos-proportional-priority:0
        clos-min:0 MHz
        clos-max:Max Turbo frequency
        clos-desired:0 MHz

```
#### 灏嗕竴涓?CPU 涓庝竴涓?CLOS 缁勫叧鑱?

```

 # intel-speed-select core-power assoc --help
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 Associate a clos id to a CPU
	Specify targeted clos id with [--clos|-c]


```

 # intel-speed-select -c 10 core-power assoc -c 3
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-10
      core-power
        assoc:success

```
涓€鏃︿竴涓?CPU 琚叧鑱旓紝瀹冪殑鍏勫紵 CPU 涔熶細琚叧鑱斿埌涓€涓?CLOS 缁勩€備竴鏃﹀叧鑱旓紝瑕侀伩鍏嶆敼鍙?Linux "cpufreq"
瀛愮郴缁熺殑缂╂斁棰戠巼闄愬埗銆?
瑕佹鏌ヤ竴涓?CPU 宸叉湁鐨勫叧鑱旓紝鍙互浣跨敤 "core-power get-assoc" 鍛戒护锛?```

 # intel-speed-select -c 10 core-power get-assoc
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-1
  die-0
    cpu-10
      get-assoc
        clos:3

```
杩欒〃鏄?CPU 10 鏄?CLOS 缁?3 鐨勪竴閮ㄥ垎銆?

#### 绂佺敤鍩轰簬 CLOS 鐨勪紭鍏堢骇鎺掑簭


```

```
# intel-speed-select core-power disable

鍍?Intel(R) SST-TF 杩欐牱鐨勪竴浜涚壒鎬у彧鏈夊湪鍚敤浜嗗熀浜?CLOS 鐨勪紭鍏堢骇鎺掑簭鏃舵墠鑳借鍚敤銆?鍑轰簬杩欎釜鍘熷洜锛屽湪 Intel(R) SST-TF 宸插惎鐢ㄦ椂绂佺敤瀹冨彲鑳藉鑷?Intel(R) SST-TF 澶辫触銆?濡傛灉 Intel(R) SST-TF 宸茬粡鍚敤锛岃繖灏嗗鑷?"disable" 鍛戒护鏄剧ず閿欒銆?鐩稿簲鍦帮紝瑕佺鐢ㄥ畠锛屽繀椤诲厛绂佺敤 Intel(R) SST-TF 鐗规€с€?
### Intel(R) Speed Select Technology - Base Frequency锛圛ntel(R) SST-BF锛?

Intel(R) Speed Select Technology - Base Frequency锛圛ntel(R) SST-BF锛夌壒鎬ц鐢ㄦ埛鑳藉鎺у埗鍩哄噯棰戠巼銆?濡傛灉鏌愪簺鍏抽敭鐨勫伐浣滆礋杞界嚎绋嬭姹傛亽瀹氱殑楂樹繚璇佹€ц兘锛岄偅涔堣繖涓壒鎬у彲浠ョ敤鏉ュ湪鐗瑰畾鐨?CPU 闆嗗悎锛堥珮浼樺厛绾?CPU锛変笂浠ユ洿楂樼殑鍩哄噯棰戠巼
鎵ц璇ョ嚎绋嬶紝浠ｄ环鏄叾浠?CPU 涓婅緝浣庣殑鍩哄噯棰戠巼锛堜綆浼樺厛绾?CPU锛夈€傝繖涓壒鎬т笉闇€瑕佷綆浼樺厛绾?CPU 绂荤嚎銆?
Intel(R) SST-BF 鐨勬敮鎸佷緷璧栦簬 Intel(R) Speed Select Technology -
Performance Profile锛圛ntel(R) SST-PP锛夋€ц兘绾у埆閰嶇疆銆傛湁鍙兘鍙湁鏌愪簺鎬ц兘绾у埆鏀寔 Intel(R) SST-BF銆?涔熸湁鍙兘鍙湁鍩虹鎬ц兘绾у埆锛坙evel = 0锛夋敮鎸?Intel(R) SST-BF銆?鍥犳锛岄鍏堥€夋嫨鎯宠鐨勬€ц兘绾у埆鏉ュ惎鐢ㄨ繖涓壒鎬с€?
鍦ㄨ繖涓娴嬬郴缁熶腑锛孖ntel(R) SST-BF 鍦ㄥ熀纭€
```

 # intel-speed-select -c 0 perf-profile info -l 0
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      perf-profile-level-0
        ...

        speed-select-base-freq:disabled
	...

```
鍦ㄥ惎鐢?Intel(R) SST-BF 骞舵祴閲忓叾瀵瑰伐浣滆礋杞芥€ц兘鐨勫奖鍝嶄箣鍓嶏紝鍏堟墽琛屼竴浜涘伐浣滆礋杞藉苟娴嬮噺鎬ц兘锛屽緱鍒颁竴涓敤浜庢瘮杈冪殑鍩虹嚎鎬ц兘銆?
杩欓噷鐢ㄦ埛鎯宠鏇村鏈変繚璇佺殑鎬ц兘銆傚嚭浜庤繖涓師鍥狅紝寰堝彲鑳?```

```
#echo 1 > /sys/devices/system/cpu/intel_pstate/no_turbo
```
鍩轰簬 "intel-speed-select perf-profile info -l 0" 鐨勮緭鍑猴紝鏈変繚璇侀鐜囩殑鍩哄噯棰戠巼涓?2600 MHz銆?

#### 娴嬮噺鍩虹嚎鎬ц兘浠ヨ繘琛屾瘮杈?

涓轰簡姣旇緝锛屾寫閫変竴涓绾跨▼宸ヤ綔璐熻浇锛屽叾涓瘡涓嚎绋嬪彲浠ヨ璋冨害鍒颁笉鍚岀殑 CPU 涓娿€?"Hackbench pipe" 娴嬭瘯鏄浣曚娇鐢?Intel(R) SST-BF 鎻愬崌鎬ц兘鐨勪竴涓ソ渚嬪瓙銆?
涓嬮潰锛岃繖涓伐浣滆礋杞芥祴閲忕殑鏄钩鍧囪皟搴﹀櫒鍞ら啋寤惰繜锛屽洜姝や竴涓洿浣?```

 # taskset -c 3,4 perf bench -r 100 sched pipe
 # Running 'sched/pipe' benchmark:
 # Executed 1000000 pipe operations between two processes
     Total time: 6.102 [sec]
       6.102445 usecs/op
         163868 ops/sec

```
鍦ㄨ繍琛屼笂闈㈢殑娴嬭瘯鏃讹紝濡傛灉鎴戜滑鍙?turbostat 鐨勮緭鍑猴紝瀹冨皢鍚戞垜浠樉绀烘湁 2 涓?CPU 寰堢箒蹇欏苟杈惧埌浜嗘渶澶ч鐜?锛堝嵆鍩哄噯
```

 #turbostat -c 0-13 --show Package,Core,CPU,Bzy_MHz -i 1
 Package	Core	CPU	Bzy_MHz
 0		0	0	1000
 0		1	1	1005
 0		2	2	1000
 0		3	3	2600
 0		4	4	2600
 0		5	5	1000
 0		6	6	1000
 0		7	7	1005
 0		8	8	1005
 0		9	9	1000
 0		10	10	1000
 0		11	11	995
 0		12	12	1000
 0		13	13	1000

```
浠庝笂闈㈢殑 turbostat 杈撳嚭鍙互鐪嬪嚭锛孋PU 3 涓?4 閮介潪甯哥箒蹇欙紝骞惰揪鍒颁簡 2600 MHz 鐨勫畬鏁存湁淇濊瘉棰戠巼銆?
#### Intel(R) SST-BF 鑳藉姏


瑕佽幏鍙栧綋鍓嶆€ц兘绾у埆 0 涓?Intel(R) SST-BF 鐨勮兘鍔涳紝
```

 # intel-speed-select base-freq info -l 0
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      speed-select-base-freq
        high-priority-base-frequency(MHz):3000
        high-priority-cpu-mask:00000216,00002160
        high-priority-cpu-list:5,6,8,13,33,34,36,41
        low-priority-base-frequency(MHz):2400
        tjunction-temperature(C):125
        thermal-design-power(W):205

```
涓婅堪鑳藉姏琛ㄦ槑锛岃繖涓郴缁熶笂鏈変竴浜?CPU 鍙互鎻愪緵 3000 MHz 鐨勫熀鍑嗛鐜囷紝鑰屽湪杩欎釜鎬ц兘绾у埆涓嬬殑鏍囧噯鍩哄噯棰戠巼鏄?锛堜笉鍚岋級銆傚敖绠″姝わ紝杩欎簺 CPU 鏄浐瀹氱殑锛屽畠浠€氳繃 high-priority-cpu-list/high-priority-cpu-mask 鍛堢幇銆?浣嗗鏋滈€夋嫨浜嗚繖涓?Intel(R) SST-BF 鐗规€э紝浣庝紭鍏堢骇 CPU锛堜笉鍦?high-priority-cpu-list 涓殑锛夋渶澶氬彧鑳芥彁渚?2400 MHz銆?鍥犳锛屽鏋滆繖绉嶅浣庝紭鍏堢骇 CPU 鐨勬埅鏂槸鍙互鎺ュ彈鐨勶紝閭ｄ箞鐢ㄦ埛鍙互閽堝涓婇潰杩欎釜 "sched pipe" 宸ヤ綔璐熻浇鍚敤 Intel(R)
SST-BF 鐗规€э紝鍥犱负鍙娇鐢ㄤ簡涓や釜 CPU锛屽畠浠彲浠ヨ璋冨害鍒伴珮浼樺厛绾?CPU 涓婏紝骞惰幏寰?400 MHz 鐨勬彁鍗囥€?
#### 鍚敤 Intel(R) SST-BF


```

 # intel-speed-select base-freq enable -a
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      base-freq
        enable:success
 package-1
  die-0
    cpu-14
      base-freq
        enable:success

```
鍦ㄨ繖绉嶆儏鍐典笅锛?a 閫夐」鏄彲閫夌殑銆傝繖涓嶄粎鍚敤浜?Intel(R) SST-BF锛岃繕浣跨敤 Intel(R) Speed Select Technology
Core Power锛圛ntel(R) SST-CP锛夌壒鎬ф潵璋冩暣鏍哥殑浼樺厛绾с€傝繖涓€夐」灏嗘瘡涓?Intel(R) Speed Select Technology -
Performance Profile锛圛ntel(R) SST-PP锛夌被鐨勬€ц兘璁句负鏈€澶ф€ц兘锛屼互渚跨‖浠朵负姣忎釜 CPU 鎻愪緵灏藉彲鑳芥渶澶х殑鎬ц兘銆?
濡傛灉涓嶄娇鐢?-a 閫夐」锛岄偅涔堝湪鍚敤 Intel(R) SST-BF 涔嬪墠闇€瑕佷互涓嬫楠わ細

- 鍙戠幇 Intel(R) SST-BF 骞惰涓嬩綆浼樺厛绾т笌楂樹紭鍏堢骇鍩哄噯棰戠巼
- 璁颁笅楂樹紭鍏堢骇 CPU 鍒楄〃
- 浣跨敤 core-power 鐗规€ч泦鍚敤 CLOS
- 閰嶇疆 CLOS 鍙傛暟銆備娇鐢?CLOS.min 璁句负鏈€灏忔€ц兘
- 灏嗘湡鏈涚殑 CPU 璁㈤槄鍒?CLOS 缁?
鍦ㄨ繖绉嶉厤缃笅锛屽鏋滈€氳繃缁戝畾鏉ユ墽琛岀浉鍚岀殑宸ヤ綔璐熻浇锛?```

 #taskset -c 5,6 perf bench -r 100 sched pipe
 # Running 'sched/pipe' benchmark:
 # Executed 1000000 pipe operations between two processes
     Total time: 5.627 [sec]
       5.627922 usecs/op
         177685 ops/sec

```
杩欐牱锛岄€氳繃鍚敤 Intel(R) SST-BF锛岃繖涓熀鍑嗘祴璇曠殑鎬ц兘鎻愬崌浜嗭紙寤惰繜闄嶄綆浜嗭級7.79%銆備粠 turbostat 杈撳嚭鍙互瑙傚療鍒帮紝
楂樹紭鍏堢骇 CPU 杈惧埌浜?3000 MHz锛岃€屼箣鍓嶆槸 2600 MHz銆?```

 #turbostat -c 0-13 --show Package,Core,CPU,Bzy_MHz -i 1
 Package	Core	CPU	Bzy_MHz
 0		0	0	2151
 0		1	1	2166
 0		2	2	2175
 0		3	3	2175
 0		4	4	2175
 0		5	5	3000
 0		6	6	3000
 0		7	7	2180
 0		8	8	2662
 0		9	9	2176
 0		10	10	2175
 0		11	11	2176
 0		12	12	2176
 0		13	13	2661

```
#### 绂佺敤 Intel(R) SST-BF


```

```
# intel-speed-select base-freq disable -a


### Intel(R) Speed Select Technology - Turbo Frequency锛圛ntel(R) SST-TF锛?

杩欎釜鐗规€т娇寰楄兘澶熷熀浜庝紭鍏堢骇涓烘牳璁剧疆涓嶅悓鐨?"All core turbo ratio limits"锛堝叏鏍哥澘棰戞瘮闄愬埗锛夈€?閫氳繃浣跨敤杩欎釜鐗规€э紝涓€浜涙牳鍙互琚厤缃负閫氳繃鎸囧畾瀹冧滑涓洪珮浼樺厛绾ф潵鑾峰緱鏇撮珮鐨勭澘棰戦鐜囷紝
浠ｄ环鏄綆浼樺厛绾ф牳涓婅緝浣庢垨娌℃湁鐫块棰戠巼銆?
鍑轰簬杩欎釜鍘熷洜锛岃繖涓壒鎬у彧鏈夊綋绯荤粺姝ｅ繖浜庡埄鐢ㄦ墍鏈?CPU銆佷絾鐢ㄦ埛鎯宠鏌愪釜鍙厤缃殑閫夐」浠ュ湪鏌愪簺 CPU 涓婅幏寰楅珮鎬ц兘鏃舵墠鏈夌敤銆?
Intel(R) Speed Select Technology - Turbo Frequency锛圛ntel(R) SST-TF锛夌殑鏀寔渚濊禆浜?Intel(R) Speed Select Technology - Performance Profile锛圛ntel(R) SST-PP锛夋€ц兘绾у埆閰嶇疆銆?鏈夊彲鑳藉彧鏈夋煇涓壒瀹氱殑鎬ц兘绾у埆鏀寔 Intel(R) SST-TF銆備篃鏈夊彲鑳藉彧鏈夊熀纭€鎬ц兘绾у埆锛坙evel = 0锛夋敮鎸?Intel(R) SST-TF銆?鍥犳锛岄鍏堥€夋嫨鎯宠鐨勬€ц兘绾у埆鏉ュ惎鐢ㄨ繖涓壒鎬с€?
鍦ㄨ繖涓娴嬬郴缁熶腑锛孖ntel(R) SST-TF 鍦ㄥ熀纭€
```

 # intel-speed-select -c 0 perf-profile info -l 0
 Intel(R) Speed Select Technology
 package-0
  die-0
    cpu-0
      perf-profile-level-0
        ...
        ...
        speed-select-turbo-freq:disabled
        ...
        ...


```
瑕佹鏌ヤ娇鐢?Intel(R) SST-TF 鐗规€ц兘鍚︽敼鍠勬€ц兘锛岃鑾峰彇鍚敤 Intel(R) SST-TF 鏃剁殑鐫块棰戠巼灞炴€э紝
骞朵笌杩欎釜绯荤粺鐨勫熀鍑嗙澘棰戣兘鍔涜繘琛屾瘮杈冦€?
#### 鑾峰彇鍩哄噯鐫块鑳藉姏


```

 # intel-speed-select perf-profile info -l 0
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      perf-profile-level-0
        ...
        ...
        turbo-ratio-limits-sse
          bucket-0
            core-count:2
            max-turbo-frequency(MHz):3200
          bucket-1
            core-count:4
            max-turbo-frequency(MHz):3100
          bucket-2
            core-count:6
            max-turbo-frequency(MHz):3100
          bucket-3
            core-count:8
            max-turbo-frequency(MHz):3100
          bucket-4
            core-count:10
            max-turbo-frequency(MHz):3100
          bucket-5
            core-count:12
            max-turbo-frequency(MHz):3100
          bucket-6
            core-count:14
            max-turbo-frequency(MHz):3100
          bucket-7
            core-count:16
            max-turbo-frequency(MHz):3100

```
鍩轰簬涓婇潰鐨勬暟鎹紝褰撴墍鏈?CPU 閮界箒蹇欐椂锛屽彲浠ヨ揪鍒?3100 MHz 鐨勬渶澶ч鐜囥€傚鏋?cpu 0 - 11 涓婃湁浜涚箒蹇欑殑宸ヤ綔璐熻浇锛堜緥濡?stress锛夛紝
```

 # taskset -c 12,13 perf bench -r 100 sched pipe
 # Running 'sched/pipe' benchmark:
 # Executed 1000000 pipe operations between two processes
     Total time: 5.705 [sec]
       5.705488 usecs/op
         175269 ops/sec

```

```

 #turbostat -c 0-13 --show Package,Core,CPU,Bzy_MHz -i 1
 Package	Core	CPU	Bzy_MHz
 0		0	0	3000
 0		1	1	3000
 0		2	2	3000
 0		3	3	3000
 0		4	4	3000
 0		5	5	3100
 0		6	6	3100
 0		7	7	3000
 0		8	8	3100
 0		9	9	3000
 0		10	10	3000
 0		11	11	3000
 0		12	12	3100
 0		13	13	3100

```
鍩轰簬 turbostat 杈撳嚭锛屾€ц兘鍙楀埌浜?3100 MHz 鐨勯鐜囦笂闄愮殑闄愬埗銆傝妫€鏌ヨ兘鍚︿负 CPU 12 涓?CPU 13
鏀瑰杽 hackbench 鎬ц兘锛岄鍏堟煡鐪嬭繖涓€ц兘绾у埆涓?Intel(R) SST-TF 鐗规€х殑鑳藉姏銆?
#### 鑾峰彇 Intel(R) SST-TF 鑳藉姏


```

 # intel-speed-select turbo-freq info -l 0
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      speed-select-turbo-freq
          bucket-0
            high-priority-cores-count:2
            high-priority-max-frequency(MHz):3200
            high-priority-max-avx2-frequency(MHz):3200
            high-priority-max-avx512-frequency(MHz):3100
          bucket-1
            high-priority-cores-count:4
            high-priority-max-frequency(MHz):3100
            high-priority-max-avx2-frequency(MHz):3000
            high-priority-max-avx512-frequency(MHz):2900
          bucket-2
            high-priority-cores-count:6
            high-priority-max-frequency(MHz):3100
            high-priority-max-avx2-frequency(MHz):3000
            high-priority-max-avx512-frequency(MHz):2900
          speed-select-turbo-freq-clip-frequencies
            low-priority-max-frequency(MHz):2600
            low-priority-max-avx2-frequency(MHz):2400
            low-priority-max-avx512-frequency(MHz):2100

```
鍩轰簬涓婇潰鐨勮緭鍑猴紝鏈変竴涓?Intel(R) SST-TF bucket锛屽叾涓湁 2 涓珮浼樺厛绾ф牳銆?濡傛灉鍙缃?2 涓珮浼樺厛绾ф牳锛岄偅涔堣繖浜涙牳涓婄殑鏈€澶х澘棰戦鐜囧彲浠ユ彁鍗囧埌 3200 MHz銆?杩欐瘮鎵€鏈夋牳鐨勫熀鍑嗙澘棰戣兘鍔涢珮浜?100 MHz銆?
鐩稿簲鍦帮紝瀵逛簬 hackbench 宸ヤ綔璐熻浇锛屽彲浠ュ皢涓や釜 CPU 璁句负楂樹紭鍏堢骇锛屽叾浣欎负浣庝紭鍏堢骇銆?涓€涓壇浣滅敤鏄紝涓€鏃﹀惎鐢紝浣庝紭鍏堢骇鏍稿皢琚埅鏂埌杈冧綆鐨?2600 MHz 棰戠巼銆?
#### 鍚敤 Intel(R) SST-TF


```

 # intel-speed-select -c 12,13 turbo-freq enable -a
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-12
      turbo-freq
        enable:success
 package-0
  die-0
    cpu-13
      turbo-freq
        enable:success
 package--1
  die-0
    cpu-63
      turbo-freq --auto
        enable:success

```
鍦ㄨ繖绉嶆儏鍐典笅锛岄€夐」 "-a" 鏄彲閫夌殑銆傚鏋滆缃紝瀹冧細鍚敤 Intel(R) SST-TF 鐗规€э紝
骞朵娇鐢?Intel Speed Select Technology Core Power锛圛ntel(R) SST-CP锛夌壒鎬у皢 CPU 璁句负楂樹紭鍏堢骇涓庝綆浼樺厛绾с€?閫氳繃 "-c" 鍙傛暟浼犲叆鐨?CPU 缂栧彿琚爣璁颁负楂樹紭鍏堢骇锛屽寘鎷叾鍏勫紵鏍搞€?
濡傛灉涓嶄娇鐢?-a 閫夐」锛岄偅涔堝湪鍚敤 Intel(R) SST-TF 涔嬪墠闇€瑕佷互涓嬫楠わ細

- 鍙戠幇 Intel(R) SST-TF 骞惰涓嬮珮浼樺厛绾ф牳鐨?bucket 涓庢渶澶ч鐜?
- 浣跨敤 core-power 鐗规€ч泦鍚敤 CLOS - 閰嶇疆 CLOS 鍙傛暟

- 灏嗘湡鏈涚殑 CPU 璁㈤槄鍒?CLOS 缁勶紝纭繚楂樹紭鍏堢骇鏍歌璁句负鏈€澶ч鐜?
濡傛灉鎵ц鐩稿悓鐨?hackbench 宸ヤ綔璐熻浇锛屽皢 hackbench 绾跨▼璋冨害鍒伴珮浼樺厛绾ф牳涓婏紝
```

 #taskset -c 12,13 perf bench -r 100 sched pipe
 # Running 'sched/pipe' benchmark:
 # Executed 1000000 pipe operations between two processes
     Total time: 5.510 [sec]
       5.510165 usecs/op
         180826 ops/sec

```
杩欏湪绻佸繖鐨勭郴缁熶笂甯︽潵浜嗙害 3.3% 鐨勬€ц兘鎻愬崌銆傝繖閲?turbostat 杈撳嚭灏嗘樉绀?CPU 12 涓?CPU 13 鑾峰緱浜?100 MHz 鐨勬彁鍗囥€?```

 #turbostat -c 0-13 --show Package,Core,CPU,Bzy_MHz -i 1
 Package	Core	CPU	Bzy_MHz
 ...
 0		12	12	3200
 0		13	13	3200

```
