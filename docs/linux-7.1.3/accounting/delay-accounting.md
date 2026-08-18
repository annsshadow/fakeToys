## 寤惰繜缁熻锛圖elay accounting锛?

褰撲换鍔＄瓑寰呮煇涓唴鏍歌祫婧愬彉涓哄彲鐢ㄦ椂锛堜緥濡傦紝涓€涓彲杩愯浠诲姟鍙兘瑕佺瓑寰呬竴涓┖闂?CPU 鏉ヨ繍琛岋級锛屽叾鎵ц灏变細閬囧埌寤惰繜銆?
姣忎换鍔″欢杩熺粺璁″姛鑳芥祴閲忎竴涓换鍔″湪浠ヤ笅鎯呭舰涓粡鍘嗙殑寤惰繜锛?
a) 绛夊緟 CPU锛堝浜庡彲杩愯鐘舵€佹椂锛?b) 璇ヤ换鍔″彂璧风殑鍚屾鍧?I/O 瀹屾垚
c) 鎹㈠叆椤碉紙swap in锛?d) 鍐呭瓨鍥炴敹锛坢emory reclaim锛?e) 棰犵案锛坱hrashing锛?f) 鐩存帴鍐呭瓨瑙勬暣锛坉irect compact锛?g) 鍐欎繚鎶ゆ嫹璐濓紙write-protect copy锛?h) IRQ/SOFTIRQ

骞堕€氳繃 taskstats 鎺ュ彛鎶婅繖浜涚粺璁′俊鎭彁渚涚粰鐢ㄦ埛绌洪棿銆?
杩欑被寤惰繜涓哄悎鐞嗚缃换鍔＄殑 CPU 浼樺厛绾с€両/O 浼樺厛绾у拰 RSS 闄愬埗鍊兼彁渚涗簡鍙嶉銆傞噸瑕佷换鍔＄殑闀挎椂闂村欢杩燂紝鍙兘鎴愪负鎻愬崌鍏剁浉搴斾紭鍏堢骇鐨勮Е鍙戞潯浠躲€?
璇ュ姛鑳藉€熷姪 taskstats 鎺ュ彛锛岃繕鎻愪緵灞炰簬鏌愪釜绾跨▼缁勶紙瀵瑰簲浜庝紶缁?Unix 杩涚▼锛夌殑鎵€鏈変换鍔★紙鎴栫嚎绋嬶級鐨勮仛鍚堝欢杩熺粺璁°€傝繖鏄竴绉嶉€氬父闇€瑕佺殑鑱氬悎锛岀敱鍐呮牳鏉ュ畬鎴愭晥鐜囨洿楂樸€?
鐢ㄦ埛绌洪棿宸ュ叿鈥斺€斿挨鍏舵槸璧勬簮绠＄悊绫诲簲鐢ㄢ€斺€斾篃鍙互鎶婂欢杩熺粺璁¤仛鍚堟垚浠绘剰鍒嗙粍銆備负姝わ紝浠诲姟鐨勫欢杩熺粺璁″湪鍏剁敓鍛藉懆鏈熷唴浠ュ強閫€鍑烘椂閮藉彲鐢紝浠庤€岀‘淇濊兘澶熻繘琛岃繛缁笖瀹屾暣鐨勭洃鎺с€?

### 鎺ュ彛


寤惰繜缁熻浣跨敤 taskstats 鎺ュ彛锛岃鎺ュ彛鍦ㄦ湰鐩綍鐨勫崟鐙枃妗ｄ腑鏈夎缁嗘弿杩般€俆askstats 鍚戠敤鎴风┖闂磋繑鍥炰竴涓搴斾簬姣?PID 鍜屾瘡 TGID 缁熻鐨勯€氱敤鏁版嵁缁撴瀯銆傚欢杩熺粺璁″姛鑳藉～鍏呰缁撴瀯鐨勭壒瀹氬瓧娈点€傚弬瑙?
     include/uapi/linux/taskstats.h

浜嗚В涓庡欢杩熺粺璁＄浉鍏冲瓧娈电殑璇存槑銆傝繖浜涘瓧娈甸€氬父閲囩敤璁℃暟鍣ㄧ殑褰㈠紡锛岃繑鍥為拡瀵?CPU銆佸悓姝ュ潡 I/O銆乻wapin銆佸唴瀛樺洖鏀躲€侀绨搁〉缂撳瓨銆佺洿鎺ヨ鏁淬€佸啓淇濇姢鎷疯礉銆両RQ/SOFTIRQ 绛夋墍瑙傚療鍒扮殑绱寤惰繜銆?
瀵规煇涓粰瀹氳鏁板櫒锛堜緥濡?cpu_delay_total锛夌殑涓ゆ杩炵画璇绘暟鍙栧樊鍊硷紝鍗冲彲寰楀埌璇ヤ换鍔″湪璇ユ椂闂撮棿闅斿唴绛夊緟鐩稿簲璧勬簮鎵€缁忓巻鐨勫欢杩熴€?
褰撲换鍔￠€€鍑烘椂锛屽寘鍚瘡浠诲姟缁熻鐨勮褰曚細鍦ㄦ棤闇€鍛戒护鐨勬儏鍐典笅鍙戦€佺粰鐢ㄦ埛绌洪棿銆傚鏋滃畠鏄煇涓嚎绋嬬粍鏈€鍚庝竴涓€€鍑虹殑浠诲姟锛屾瘡 TGID 缁熻涔熶細涓€骞跺彂閫併€傛洿澶氱粏鑺傝 taskstats 鎺ュ彛鎻忚堪銆?
tools/accounting 鐩綍涓嬬殑 getdelays.c 鐢ㄦ埛绌洪棿宸ュ叿鍙互杩愯绠€鍗曞懡浠ゅ苟鏄剧ず鐩稿簲鐨勫欢杩熺粺璁★紝瀹冨悓鏃朵篃浣滀负浣跨敤 taskstats 鎺ュ彛鐨勪竴涓ず渚嬨€?
### 鐢ㄦ硶


```

	CONFIG_TASK_DELAY_ACCT=y
	CONFIG_TASKSTATS=y

```
寤惰繜缁熻鍦ㄥ惎鍔ㄩ粯璁ゆ槸鍏抽棴鐨勩€?```

   delayacct

```
鍔犲叆鍐呮牳鍚姩閫夐」銆備笅闈㈠叾浣欑殑璇存槑閮藉亣瀹氬凡鎵ц姝ゆ搷浣溿€傛垨鑰咃紝涔熷彲浠ヤ娇鐢?sysctl kernel.task_delayacct 鍦ㄨ繍琛屾椂鍒囨崲鐘舵€併€備絾闇€娉ㄦ剰锛屽彧鏈夊湪鍏跺惎鐢ㄤ箣鍚庡惎鍔ㄧ殑浠诲姟鎵嶄細鎷ユ湁 delayacct 淇℃伅銆?
绯荤粺鍚姩鍚庯紝浣跨敤涓€涓被浼?getdelays.c 鐨勫伐鍏锋潵璁块棶缁欏畾浠诲姟鎴栦换鍔＄粍锛坱gid锛夋墍缁忓巻鐨勫欢杩熴€傝宸ュ叿涔熷厑璁告墽琛岀粰瀹氬懡浠ゅ苟鏌ョ湅鐩稿簲鐨勫欢杩熴€?
```

	getdelays [-dilv] [-t tgid] [-p pid]

```
```

	# ./getdelays -d -p 10
	(output similar to next case)

```
```

	bash-4.4# ./getdelays -d -t 242
	print delayacct stats ON
	TGID    242




	CPU         count     real total  virtual total    delay total  delay average      delay max      delay min      delay max timestamp
	               46      188000000      192348334        4098012          0.089ms     0.429260ms     0.051205ms    2026-01-15T15:06:58
	IO          count    delay total  delay average      delay max      delay min      delay max timestamp
	                0              0          0.000ms     0.000000ms     0.000000ms                    N/A
	SWAP        count    delay total  delay average      delay max      delay min      delay max timestamp
	                0              0          0.000ms     0.000000ms     0.000000ms                    N/A
	RECLAIM     count    delay total  delay average      delay max      delay min      delay max timestamp
	                0              0          0.000ms     0.000000ms     0.000000ms                    N/A
	THRASHING   count    delay total  delay average      delay max      delay min      delay max timestamp
	                0              0          0.000ms     0.000000ms     0.000000ms                    N/A
	COMPACT     count    delay total  delay average      delay max      delay min      delay max timestamp
	                0              0          0.000ms     0.000000ms     0.000000ms                    N/A
	WPCOPY      count    delay total  delay average      delay max      delay min      delay max timestamp
	              182       19413338          0.107ms     0.547353ms     0.022462ms    2026-01-15T15:05:24
	IRQ         count    delay total  delay average      delay max      delay min      delay max timestamp
	                0              0          0.000ms     0.000000ms     0.000000ms                    N/A

```
```

	# ./getdelays -i -p 1
	printing IO accounting
	linuxrc: read=65536, write=0, cancelled_write=0

```
涓婅堪鍛戒护鍙笌 -v 涓€璧蜂娇鐢ㄤ互鑾峰彇鏇村璋冭瘯淇℃伅銆?
绯荤粺鍚姩鍚庯紝浣跨敤 `delaytop` 鑾峰彇绯荤粺绾у欢杩熶俊鎭紝鍏朵腑鍖呭惈绯荤粺绾?PSI 淇℃伅鍜屽欢杩熸渶楂樼殑 Top-N 浠诲姟銆?娉ㄦ剰锛歅SI 鏀寔闇€瑕?`CONFIG_PSI=y` 浠ュ強 `psi=1` 鎵嶈兘瀹屾暣浣撳伐浣溿€?
`delaytop` 鏄竴涓敤浜庣洃鎺х郴缁熷帇鍔涗笌浠诲姟寤惰繜鐨勪氦浜掑紡宸ュ叿銆傚畠鏀寔澶氱鎺掑簭閫夐」銆佹樉绀烘ā寮忎互鍙婂疄鏃堕敭鐩樻帶鍒躲€?
```

	bash# ./delaytop
	System Pressure Information: (avg10/avg60vg300/total)
	CPU some:       0.0%/   0.0%/   0.0%/  106137(ms)
	CPU full:       0.0%/   0.0%/   0.0%/       0(ms)
	Memory full:    0.0%/   0.0%/   0.0%/       0(ms)
	Memory some:    0.0%/   0.0%/   0.0%/       0(ms)
	IO full:        0.0%/   0.0%/   0.0%/    2240(ms)
	IO some:        0.0%/   0.0%/   0.0%/    2783(ms)
	IRQ full:       0.0%/   0.0%/   0.0%/       0(ms)
	[o]sort [M]memverbose [q]quit
	Top 20 processes (sorted by cpu delay):
		PID      TGID  COMMAND           CPU(ms)   IO(ms)  IRQ(ms)  MEM(ms)
	------------------------------------------------------------------------
		110       110  kworker/15:0H-s   27.91     0.00     0.00     0.00
		57        57  cpuhp/7            3.18     0.00     0.00     0.00
		99        99  cpuhp/14           2.97     0.00     0.00     0.00
		51        51  cpuhp/6            0.90     0.00     0.00     0.00
		44        44  kworker/4:0H-sy    0.80     0.00     0.00     0.00
		60        60  ksoftirqd/7        0.74     0.00     0.00     0.00
		76        76  idle_inject/10     0.31     0.00     0.00     0.00
		100       100  idle_inject/14     0.30     0.00     0.00     0.00
		1309      1309  systemsettings     0.29     0.00     0.00     0.00
		45        45  cpuhp/5            0.22     0.00     0.00     0.00
		63        63  cpuhp/8            0.20     0.00     0.00     0.00
		87        87  cpuhp/12           0.18     0.00     0.00     0.00
		93        93  cpuhp/13           0.17     0.00     0.00     0.00
		1265      1265  acpid              0.17     0.00     0.00     0.00
		1552      1552  sshd               0.17     0.00     0.00     0.00
		2584      2584  sddm-helper        0.16     0.00     0.00     0.00
		1284      1284  rtkit-daemon       0.15     0.00     0.00     0.00
		1326      1326  nde-netfilter      0.14     0.00     0.00     0.00
		27        27  cpuhp/2            0.13     0.00     0.00     0.00
		631       631  kworker/11:2-rc    0.11     0.00     0.00     0.00

```
```

	o - Select sort field (CPU, IO, IRQ, Memory, etc.)
	M - Toggle display mode (Default/Memory Verbose)
	q - Quit

```
```

	cpu(c)       - CPU delay
	blkio(i)     - I/O delay
	irq(q)       - IRQ delay
	mem(m)       - Total memory delay
	swapin(s)    - Swapin delay (memory verbose mode only)
	freepages(r) - Freepages reclaim delay (memory verbose mode only)
	thrashing(t) - Thrashing delay (memory verbose mode only)
	compact(p)   - Compaction delay (memory verbose mode only)
	wpcopy(w)    - Write page copy delay (memory verbose mode only)

```
```

	# ./delaytop -s blkio
	Sorted by IO delay

	# ./delaytop -s mem -M
	Sorted by memory delay in memory verbose mode

	# ./delaytop -p pid
	Print delayacct stats

	# ./delaytop -P num
	Display the top N tasks

	# ./delaytop -n num
	Set delaytop refresh frequency (num times)

	# ./delaytop -d secs
	Specify refresh interval as secs

```
