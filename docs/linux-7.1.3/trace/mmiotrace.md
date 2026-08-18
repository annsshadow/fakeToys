## 鍐呮牳鎬佸唴瀛樻槧灏?I/O 璺熻釜


涓婚〉鍙婂彲閫夌敤鎴风┖闂村伐鍏风殑閾炬帴锛?

	https://nouveau.freedesktop.org/wiki/MmioTrace

MMIO 璺熻釜鏈€鍒濈敱 Intel 鍦?2003 骞村乏鍙充负鍏舵晠闅滄敞鍏ユ祴璇曟鏋讹紙Fault Injection
Test Harness锛夊紑鍙戙€傚湪 2006 骞?12 鏈堣嚦 2007 骞?1 鏈堟湡闂达紝Jeff Muizelaar 鍒╃敤
Intel 鐨勪唬鐮侊紝鍒涘缓浜嗕竴涓敤浜庤窡韪?MMIO 璁块棶鐨勫伐鍏凤紝鍏跺垵琛锋槸涓?Nouveau 椤圭洰
鏈嶅姟銆傛鍚庢湁璁稿浜轰綔鍑轰簡璐＄尞銆?

Mmiotrace 鏄负瀵逛换浣曞唴瀛樻槧灏?IO 璁惧杩涜閫嗗悜宸ョ▼鑰屾瀯寤虹殑锛孨ouveau 椤圭洰鏄叾
绗竴涓湡瀹炵敤鎴枫€備粎鏀寔 x86 鍜?x86_64 鏋舵瀯銆?

鏍戝锛坥ut-of-tree锛夌殑 mmiotrace 鏈€鍒濈敱 Pekka Paalanen <pq@iki.fi> 淇敼涓哄彲
鍚堝叆涓荤嚎锛屽苟閫傞厤 ftrace 妗嗘灦銆?


### 鍑嗗


Mmiotrace 鍔熻兘閫氳繃 CONFIG_MMIOTRACE 閫夐」缂栬瘧杩涘唴鏍搞€傝窡韪粯璁ゆ槸鍏抽棴鐨勶紝鍥犳
灏嗗叾璁句负 yes 鏄畨鍏ㄧ殑銆係MP 绯荤粺鍙楁敮鎸侊紝浣嗗鏋滃浜庝竴涓?CPU 澶勪簬鍦ㄧ嚎鐘舵€侊紝
璺熻釜灏嗕笉鍙潬骞跺彲鑳戒涪澶变簨浠讹紝鍥犳 mmiotrace 鍦ㄨ繍琛屾椂婵€娲绘湡闂翠細浣块櫎涓€涓?CPU
涔嬪鐨勬墍鏈?CPU 绂荤嚎銆備綘鍙互鎵嬪姩閲嶆柊鍚敤 CPU锛屼絾宸茬粡璀﹀憡杩囦綘锛氱敱浜?CPU 绔炰簤锛?
鏃犳硶鑷姩妫€娴嬫槸鍚︽鍦ㄤ涪澶变簨浠躲€?


### 鐢ㄦ硶蹇€熷弬鑰?

```

	$ mount -t debugfs debugfs /sys/kernel/debug
	$ echo mmiotrace > /sys/kernel/tracing/current_tracer
	$ cat /sys/kernel/tracing/trace_pipe > mydump.txt &
	Start X or whatever.
	$ echo "X is up" > /sys/kernel/tracing/trace_marker
	$ echo nop > /sys/kernel/tracing/current_tracer
	Check for lost events.


```
### 鐢ㄦ硶


纭繚 debugfs 宸叉寕杞藉埌 /sys/kernel/debug銆?
```

	$ mount -t debugfs debugfs /sys/kernel/debug

```
纭浣犲嵆灏嗚窡韪殑椹卞姩灏氭湭鍔犺浇銆?

```

	$ echo mmiotrace > /sys/kernel/tracing/current_tracer

```
```

	$ cat /sys/kernel/tracing/trace_pipe > mydump.txt &

```
'cat' 杩涚▼搴斿綋淇濇寔锛堢潯鐪狅級鍦ㄥ悗鍙拌繍琛屻€?

鍔犺浇浣犳兂瑕佽窡韪殑椹卞姩骞朵娇鐢ㄥ畠銆侻miotrace 鍙細鎹曡幏鍦?mmiotrace 澶勪簬娲诲姩鐘舵€?
鏈熼棿琚?ioremap 鐨勫尯鍩熺殑 MMIO 璁块棶銆?

鍦ㄨ窡韪湡闂达紝浣犲彲浠ラ€氳繃
$ echo "X is up" > /sys/kernel/tracing/trace_marker
灏嗘敞閲婏紙鏍囪锛夋斁鍏ヨ窡韪褰曚腑銆傝繖鏍锋洿瀹规槗鐪嬫竻锛堝簽澶х殑锛夎窡韪褰曠殑鍝竴閮ㄥ垎
瀵瑰簲鍝釜鎿嶄綔銆傚缓璁斁缃叧浜庝綘鎵€鍋氭搷浣滅殑鎻忚堪鎬ф爣璁般€?

```

	$ echo nop > /sys/kernel/tracing/current_tracer

```
'cat' 杩涚▼閫€鍑恒€傚鏋滃畠娌℃湁閫€鍑猴紝閫氳繃鎵ц 'fg' 鍛戒护骞舵寜涓?ctrl+c 鏉ョ粓姝㈠畠銆?

```

	$ grep -i lost mydump.txt

```
```

	$ dmesg

```
浠ユ煡鐪嬪唴鏍告棩蹇楀苟鏌ユ壘 "mmiotrace has lost events" 璀﹀憡銆傚鏋滀簨浠朵涪澶变簡锛?
璺熻釜璁板綍灏变笉瀹屾暣銆備綘搴旇鎵╁ぇ缂撳啿鍖哄苟閲嶈瘯銆傜紦鍐插尯鍙€氳繃鍏堟煡鐪嬪綋鍓嶇紦鍐插尯
鏈夊澶ф潵鎵╁ぇ
```

	$ cat /sys/kernel/tracing/buffer_size_kb

```
浼氱粰鍑轰竴涓暟瀛椼€傚皢璇ユ暟瀛楀ぇ绾︾炕鍊嶅苟鍐欏洖锛屼緥濡?
```

	$ echo 128000 > /sys/kernel/tracing/buffer_size_kb

```
鐒跺悗浠庡ご閲嶆柊寮€濮嬨€?

濡傛灉浣犳鍦ㄤ负鏌愪釜椹卞姩椤圭洰锛堜緥濡?Nouveau锛夊仛璺熻釜锛屼綘杩樺簲褰?
```

	$ lspci -vvv > lspci.txt
	$ dmesg > dmesg.txt
	$ tar zcf pciid-nick-mmiotrace.tar.gz mydump.txt lspci.txt dmesg.txt

```
鐒跺悗鍙戦€佽 .tar.gz 鏂囦欢銆傝窡韪褰曞帇缂╂晥鏋滄樉钁椼€傚皢 "pciid" 鍜?"nick" 鏇挎崲
涓烘鍦ㄨ皟鏌ョ‖浠剁殑 PCI ID 鎴栧瀷鍙峰悕绉颁互鍙婁綘鐨勬樀绉般€?


### Mmiotrace 鐨勫伐浣滃師鐞?


瀵圭‖浠?IO 鍐呭瓨鐨勮闂槸閫氳繃璋冪敤鏌愪釜 ioremap_*() 鍑芥暟锛屽皢浠?PCI 鎬荤嚎鏄犲皠鍦板潃
鏉ヨ幏寰椼€侻miotrace 鎸傝浇鍒?__ioremap() 鍑芥暟锛屽苟鍦ㄦ瘡娆″垱寤烘槧灏勬椂琚皟鐢ㄣ€傛槧灏勬槸
涓€涓璁板綍鍒拌窡韪棩蹇椾腑鐨勪簨浠躲€傛敞鎰?ISA 鑼冨洿鐨勬槧灏勪笉浼氳鎹曡幏锛屽洜涓鸿鏄犲皠濮嬬粓
瀛樺湪骞朵細琚洿鎺ヨ繑鍥炪€?

MMIO 璁块棶閫氳繃椤甸敊璇潵璁板綍銆傚氨鍦?__ioremap() 杩斿洖涔嬪墠锛岃鏄犲皠鐨勯〉琚爣璁颁负
涓嶅瓨鍦ㄣ€傚璇ラ〉鐨勪换浣曡闂兘浼氬紩鍙戦敊璇€傞〉閿欒澶勭悊绋嬪簭璋冪敤 mmiotrace 鏉ュ鐞?
璇ラ敊璇€侻miotrace 灏嗚椤垫爣璁颁负瀛樺湪锛岃缃?TF 鏍囧織浠ュ疄鐜板崟姝ユ墽琛岋紝骞堕€€鍑洪敊璇?
澶勭悊绋嬪簭銆傚紩鍙戦敊璇殑鎸囦护琚墽琛屽苟杩涘叆璋冭瘯闄烽槺銆傚湪杩欓噷 mmiotrace 鍐嶆灏嗚椤?
鏍囪涓轰笉瀛樺湪銆傝鎸囦护琚В鐮佷互鑾峰彇鎿嶄綔绫诲瀷锛堣/鍐欙級銆佹暟鎹搴︿互鍙婅鍐欑殑鏁板€笺€?
杩欎簺淇℃伅琚瓨鍌ㄥ埌璺熻釜鏃ュ織涓€?

鍦ㄩ〉閿欒澶勭悊绋嬪簭涓皢椤垫爣璁颁负瀛樺湪鍦?SMP 鏈哄櫒涓婂瓨鍦ㄧ珵浜夋潯浠躲€傚湪鍗曟鎵ц鏈熼棿锛?
鍏朵粬 CPU 鍙兘鍦ㄨ椤典笂鑷敱杩愯锛屼簨浠跺彲鑳藉湪鏃犳彁绀虹殑鎯呭喌涓嬩涪澶便€備笉榧撳姳鍦ㄨ窡韪?
鏈熼棿閲嶆柊鍚敤鍏朵粬 CPU銆?


### 璺熻釜鏃ュ織鏍煎紡


鍘熷鏃ュ織鏄枃鏈紝鍙互寰堝鏄撳湴鐢?grep銆乤wk 绛夊伐鍏疯繘琛岃繃婊ゃ€備竴鏉¤褰曟槸鏃ュ織涓殑
涓€琛屻€傝褰曚互涓€涓叧閿瓧寮€澶达紝鍚庤窡璇ュ叧閿瓧鎵€渚濊禆鐨勫弬鏁般€傚弬鏁颁箣闂寸敤绌烘牸鍒嗛殧锛?
鎴栧欢缁埌琛屽熬銆傜増鏈?20070824 鐨勬牸寮忓涓嬶細

### 璇存槑	鍏抽敭瀛?浠ョ┖鏍煎垎闅旂殑鍙傛暟


璇讳簨浠?R	width, timestamp, map id, physical, value, PC, PID
鍐欎簨浠?W	width, timestamp, map id, physical, value, PC, PID
ioremap 浜嬩欢	MAP	timestamp, map id, physical, virtual, length, PC, PID
iounmap 浜嬩欢	UNMAP	timestamp, map id, PC, PID
鏍囪		MARK	timestamp, text
鐗堟湰		VERSION	the string "20070824"
渚涜鍙栬€呭弬鑰冪殑淇℃伅	LSPCI	one line from lspci -v
PCI 鍦板潃鏄犲皠	PCIDEV	space-separated /proc/bus/pci/devices data
鏈煡鎿嶄綔鐮?UNKNOWN	timestamp, map id, physical, data, PC, PID

鏃堕棿鎴充互绉掍负鍗曚綅锛屽甫鏈夊皬鏁伴儴鍒嗐€侾hysical 鏄?PCI 鎬荤嚎鍦板潃锛寁irtual 鏄唴鏍歌櫄鎷?
鍦板潃銆俉idth 鏄暟鎹殑瀛楄妭瀹藉害锛寁alue 鏄暟鎹€笺€侻ap id 鏄竴涓换鎰忕殑鏍囪瘑鍙凤紝鐢ㄤ簬
鏍囪瘑鍦ㄦ煇涓搷浣滀腑浣跨敤鐨勬槧灏勩€侾C 鏄▼搴忚鏁板櫒锛孭ID 鏄繘绋?id銆傚鏋滄湭琚褰曪紝
PC 涓洪浂銆侾ID 濮嬬粓涓洪浂锛屽洜涓哄皻涓嶆敮鎸佽窡韪簮鑷敤鎴风┖闂村唴瀛樼殑 MMIO 璁块棶銆?

渚嬪锛屼笅闈㈢殑 awk 杩囨护鍣ㄤ細鏀捐鎵€鏈夐拡瀵圭墿鐞嗗湴鍧€鑼冨洿
[0xfb73ce40, 0xfb800000] 鐨?32 浣嶅啓鎿嶄綔
```

	$ awk '/W 4 / { adr=strtonum($5); if (adr >= 0xfb73ce40 &&
	adr < 0xfb800000) print; }


```
### 闈㈠悜寮€鍙戣€呯殑宸ュ叿


鐢ㄦ埛绌洪棿宸ュ叿鍖呭惈浠ヤ笅瀹炵敤绋嬪簭锛?
  - 鐢ㄧ‖浠跺瘎瀛樺櫒鍚嶆浛鎹㈡暟瀛楀湴鍧€鍜屾暟鍊?
  - 鍥炴斁 MMIO 鏃ュ織锛屽嵆閲嶆柊鎵ц琚褰曠殑鍐欐搷浣?

