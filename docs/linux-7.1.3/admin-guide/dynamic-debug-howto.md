Dynamic debug
+++++++++++++

## 绠€浠嬶紙Introduction锛?
Dynamic debug锛堝姩鎬佽皟璇曪級鍏佽浣犲姩鎬佸湴鍚敤/绂佺敤鍐呮牳鐨勮皟璇曟墦鍗颁唬鐮侊紝浠ヨ幏鍙栭澶栫殑鍐呮牳淇℃伅銆?
濡傛灉 `/proc/dynamic_debug/control` 瀛樺湪锛岃鏄庝綘鐨勫唴鏍告敮鎸?dynamic debug銆備綘闇€瑕?root 鏉冮檺
锛坰udo su锛夋潵浣跨敤瀹冦€?
Dynamic debug 鎻愪緵锛?
 - 涓€浠戒綘鍐呮牳涓墍鏈?**prdbg** 鐨勭洰褰曪紙Catalog锛夈€?   `cat /proc/dynamic_debug/control` 鍗冲彲鏌ョ湅瀹冧滑銆?
 - 涓€绉嶇畝鍗曠殑鏌ヨ/鍛戒护璇█锛岄€氳繃浠庝互涓嬪睘鎬т腑浠婚€?0 涓垨 1 涓殑缁勫悎鏉ラ€夊彇骞朵慨鏀?**prdbg**锛?
   - 婧愭枃浠跺悕锛坰ource filename锛?   - 鍑芥暟鍚嶏紙function name锛?   - 琛屽彿锛堝寘鎷鍙疯寖鍥达級
   - 妯″潡鍚嶏紙module name锛?   - 鏍煎紡瀛楃涓诧紙format string锛?   - 绫诲悕锛坈lass name锛岀敱姣忎釜妯″潡鎵€鐭?鎵€澹版槑锛?
娉ㄦ剰锛氳鐪熸鍦ㄦ帶鍒跺彴涓婄湅鍒拌皟璇曟墦鍗拌緭鍑猴紝浣犲彲鑳介渶瑕佽皟鏁村唴鏍哥殑 `loglevel=`锛屾垨浣跨敤
`ignore_loglevel`銆傚叧浜庤繖浜涘唴鏍稿弬鏁帮紝璇峰弬闃?Documentation/admin-guide/kernel-parameters.rst銆?
## 鏌ョ湅 Dynamic Debug 琛屼负锛圴iewing Dynamic Debug Behaviour锛?
```

  :#> head -n7 /proc/dynamic_debug/control
  # filename:lineno [module]function flags format
  init/main.c:1179 [main]initcall_blacklist =_ "blacklisting initcall %s\012
  init/main.c:1218 [main]initcall_blacklisted =_ "initcall %s blacklisted\012"
  init/main.c:1424 [main]run_init_process =_ "  with arguments:\012"
  init/main.c:1426 [main]run_init_process =_ "    %s\012"
  init/main.c:1427 [main]run_init_process =_ "  with environment:\012"
  init/main.c:1429 [main]run_init_process =_ "    %s\012"

```
绗?3 涓互绌烘牸鍒嗛殧鐨勫垪鏄剧ず褰撳墠鐨?flags锛堟爣蹇楋級锛屽墠闈㈠甫鏈?`=` 浠ユ柟渚夸笌 grep/cut 閰嶅悎浣跨敤銆?`=p` 琛ㄧず宸插惎鐢ㄧ殑璋冪敤鐐癸紙callsite锛夈€?
## 鎺у埗 dynamic debug 琛屼负锛圕ontrolling dynamic debug Behaviour锛?
**prdbg** 璋冪敤鐐圭殑琛屼负鏄€氳繃鍐欏叆浠ヤ笅鍐呭鏉ユ帶鍒剁殑锛?```

  # grease the interface
  :#> alias ddcmd='echo $* > /proc/dynamic_debug/control'

  :#> ddcmd '-p; module main func run* +p'
  :#> grep =p /proc/dynamic_debug/control
  init/main.c:1424 [main]run_init_process =p "  with arguments:\012"
  init/main.c:1426 [main]run_init_process =p "    %s\012"
  init/main.c:1427 [main]run_init_process =p "  with environment:\012"
  init/main.c:1429 [main]run_init_process =p "    %s\012"

```
```

  :#> ddcmd mode foo +p
  dyndbg: unknown keyword "mode"
  dyndbg: query parse failed
  bash: echo: write error: Invalid argument

```
濡傛灉 debugfs 涔熷凡鍚敤骞舵寕杞斤紝閭ｄ箞 `dynamic_debug/control` 涔熶細浣嶄簬鎸傝浇鐩綍涓嬶紝閫氬父鏄?`/sys/kernel/debug/`銆?
## 鍛戒护璇█鍙傝€冿紙Command Language Reference锛?
鍦ㄥ熀鏈殑璇嶆硶灞傞潰锛屼竴鏉″懡浠ゆ槸涓€绯诲垪鐢辩┖鏍煎垎闅旂殑鍗曡瘝
```

  :#> ddcmd file svcsock.c line 1603 +p
  :#> ddcmd "file svcsock.c line 1603 +p"
  :#> ddcmd '  file   svcsock.c     line  1603 +p  '

```
鍛戒护鎻愪氦浠ヤ竴娆?write() 绯荤粺璋冪敤涓虹晫銆?```

  :#> ddcmd "func pnpacpi_get_resources +p; func pnp_assign_mem +p"
  :#> ddcmd <<"EOC"
  func pnpacpi_get_resources +p
  func pnp_assign_mem +p
  EOC
  :#> cat query-batch-file > /proc/dynamic_debug/control

```
浣犺繕鍙互鍦ㄦ瘡涓煡璇㈤」涓娇鐢ㄩ€氶厤绗︺€傚尮閰嶈鍒欐敮鎸?`*` 锛堝尮閰嶉浂涓垨澶氫釜瀛楃锛夊拰 `?` 锛堢簿纭尮閰?涓€涓瓧绗︼級锛?```

  :#> ddcmd file "drivers/usb/*" +p	# "" 鐢ㄤ簬鎶戝埗 shell 灞曞紑

```
浠庤娉曚笂璁诧紝涓€鏉″懡浠ゆ槸鎴愬鐨勫叧閿瓧-鍊硷紝鍚庤窡涓€涓?```

  command ::= match-spec* flags-spec

```
match-spec 浠庣洰褰曚腑閫夋嫨 **prdbg**锛岀劧鍚庡湪鍏朵笂搴旂敤 flags-spec锛屾墍鏈夌害鏉熶箣闂存槸涓庯紙AND锛夌殑鍏崇郴銆?鐪佺暐鐨勫叧閿瓧绛夊悓浜庡叧閿瓧 "*"銆?
match 瑙勮寖鐢变竴涓叧閿瓧锛堢敤浜庨€夋嫨瑕佹瘮杈冪殑璋冪敤鐐圭殑灞炴€э級鍜屼竴涓敤浜庢瘮杈冪殑鍊肩粍鎴愩€傚彲鑳界殑
鍏抽敭瀛楀涓嬶細
```

  match-spec ::= 'func' string |
		 'file' string |
		 'module' string |
		 'format' string |
		 'class' string |
		 'line' line-range

  line-range ::= lineno |
		 '-'lineno |
		 lineno'-' |
		 lineno'-'lineno

  lineno ::= unsigned-int

```
  `line-range` 涓嶈兘鍖呭惈绌烘牸锛屼緥濡?"1-30" 鏄湁鏁堢殑鑼冨洿锛屼絾 "1 - 30" 涓嶆槸銆?
姣忎釜鍏抽敭瀛楃殑鍚箟濡備笅锛?
func
    缁欏畾鐨勫瓧绗︿覆涓庡嚱鏁板悕杩涜姣旇緝
```

	func svc_tcp_accept
	func *recv*		# in rfcomm, bluetooth, ping, tcp

```
file
    缁欏畾鐨勫瓧绗︿覆涓庢瘡涓皟鐢ㄧ偣鐨勩€佺浉瀵逛簬 src-root 鐨勮矾寰勫悕鎴栨簮鏂囦欢鍩哄悕杩涜姣旇緝
```

	file svcsock.c
	file kernel/freezer.c	# 鍗虫帶鍒舵枃浠剁 1 鍒?	file drivers/usb/*	# 鍏朵笅鐨勬墍鏈夎皟鐢ㄧ偣
	file inode.c:start_*	# 鎶?:tail 瑙ｆ瀽涓?func锛堣涓婏級
	file inode.c:1-100	# 鎶?:tail 瑙ｆ瀽涓?line-range锛堣涓婏級

```
module
    缁欏畾鐨勫瓧绗︿覆涓庢瘡涓皟鐢ㄧ偣鐨勬ā鍧楀悕杩涜姣旇緝銆傛ā鍧楀悕鏄?`lsmod` 涓湅鍒扮殑瀛楃涓诧紝鍗充笉甯?    鐩綍涔熶笉甯?`.ko` 鍚庣紑
```

	module sunrpc
	module nfsd
	module drm*	# 鍚屾椂鍖归厤 drm 涓?drm_kms_helper

```
format
    缁欏畾鐨勫瓧绗︿覆浼氬湪 dynamic debug 鐨?format 瀛楃涓蹭腑鎼滅储銆傛敞鎰忥紝瀛楃涓蹭笉闇€瑕佸尮閰嶆暣涓?    format锛屽彧闇€鍖归厤鍏朵腑涓€閮ㄥ垎鍗冲彲銆傜┖鐧藉瓧绗﹀拰鍏跺畠鐗规畩瀛楃鍙互浣跨敤 C 鐨勫叓杩涘埗杞箟
    `\ooo` 琛ㄧず娉曟潵杞箟锛屼緥濡傜┖鏍煎瓧绗︽槸 `\040`銆傚彟澶栵紝瀛楃涓蹭篃鍙互鐢ㄥ弻寮曞彿锛坄"`锛夋垨
    鍗曞紩鍙凤紙`'`锛夋嫭璧锋潵銆?```

	format svcrdma:         // 璁稿 NFS/RDMA 鏈嶅姟绔?pr_debug
	format readahead        // readahead 缂撳瓨涓殑閮ㄥ垎 pr_debug
	format nfsd:\040SETATTR // 鍖归厤甯︾┖鐧界殑 format 鐨勪竴绉嶆柟寮?	format "nfsd: SETATTR"  // 鍖归厤甯︾┖鐧界殑 format 鐨勪竴绉嶆洿鏁存磥鐨勬柟寮?	format 'nfsd: SETATTR'  // 鍙堜竴绉嶅尮閰嶅甫绌虹櫧鐨?format 鐨勬柟寮?
```
class
    缁欏畾鐨?class_name 浼氶拡瀵规瘡涓ā鍧楄繘琛屾牎楠岋紝妯″潡鍙兘宸茬粡澹版槑浜嗕竴涓凡鐭ョ殑 class_name 鍒楄〃銆?    濡傛灉鏌愪釜妯″潡鎵惧埌浜嗚 class_name锛屽垯璋冪敤鐐逛笌绫荤殑鍖归厤鍜岃皟鏁?```

	class DRM_UT_KMS	# 涓€涓?DRM.debug 绫诲埆
	class JUNK		# 闈欓粯涓嶅尮閰?	// class TLD_*		# 娉ㄦ剰锛歝lass 鍚嶄腑涓嶆敮鎸侀€氶厤绗?
```
line
    缁欏畾鐨勫崟涓鍙锋垨琛屽彿鑼冨洿浼氫笌姣忎釜 `pr_debug()` 璋冪敤鐐圭殑琛屽彿杩涜姣旇緝銆傚崟涓鍙蜂細绮剧‘鍖归厤
    璋冪敤鐐圭殑琛屽彿銆傝鍙疯寖鍥翠細鍖归厤浠庨琛屽彿鍒版湯琛屽彿锛堝惈锛変箣闂寸殑浠讳綍璋冪敤鐐广€傞琛屽彿涓虹┖琛ㄧず鏂囦欢
    涓殑绗竴琛岋紝鏈鍙蜂负绌鸿〃绀?```

	line 1603           // 绮剧‘鍖归厤绗?1603 琛?	line 1600-1605      // 浠庣 1600 琛屽埌绗?1605 琛岀殑鍏
	line -1605          // 浠庣 1 琛屽埌绗?1605 琛岀殑 1605 琛?	line 1600-          // 浠庣 1600 琛屽埌鏂囦欢鏈熬鐨勬墍鏈夎

```
flags 瑙勮寖鐢变竴涓慨鏀规搷浣滐紝鍚庤窡涓€涓垨澶氫釜鏍囧織瀛楃缁勬垚銆備慨鏀规搷浣滄槸涓嬪垪涔嬩竴锛?```

  -    remove the given flags
  +    add the given flags
  =    set the flags to the given flags

```
```

  p    enables the pr_debug() callsite.
  _    enables no flags.

  Decorator flags add to the message-prefix, in order:
  t    Include thread ID, or <intr>
  m    Include module name
  f    Include the function name
  s    Include the source file name
  l    Include line number
  d    Include call trace

```
瀵逛簬 `print_hex_dump_debug()` 鍜?`print_hex_dump_bytes()`锛屽彧鏈?`p` 鏍囧織鏈夋剰涔夛紝鍏跺畠鏍囧織
浼氳蹇界暐銆?
娉ㄦ剰锛屾鍒欒〃杈惧紡 `^[-+=][fslmptd_]+$` 鍖归厤涓€涓?flags 瑙勮寖銆傝涓€娆℃€ф竻闄ゆ墍鏈夋爣蹇楋紝鍙互浣跨敤
`=_` 鎴?`-fslmptd`銆?
## 鍚姩杩囩▼涓殑璋冭瘯娑堟伅锛圖ebug messages during Boot Process锛?
瑕佸湪鍚姩杩囩▼涓紙鐢氳嚦鏃╀簬鐢ㄦ埛绌洪棿鍜?debugfs 鍑虹幇涔嬪墠锛夋縺娲绘牳蹇冧唬鐮佷笌鍐呭缓妯″潡鐨勮皟璇曟秷鎭紝
鍙娇鐢?`dyndbg="QUERY"` 鎴?`module.dyndbg="QUERY"`銆俀UERY 閬靛惊涓婅堪璇硶锛屼絾涓嶅緱瓒呰繃 1023
涓瓧绗︺€備綘鐨?bootloader 鍙兘浼氭柦鍔犳洿浣庣殑闄愬埗銆?
杩欎簺 `dyndbg` 鍙傛暟浼氬湪 ddebug 琛ㄨ澶勭悊涔嬪悗銆佷綔涓?early_initcall 鐨勪竴閮ㄥ垎琚鐞嗐€傚洜姝わ紝浣?鍙互閫氳繃杩欎釜鍚姩鍙傛暟锛屽惎鐢ㄥ湪姝?early_initcall 涔嬪悗杩愯鐨勬墍鏈変唬鐮佷腑鐨勮皟璇曟秷鎭€?```

   dyndbg="file ec.c +p"

```
濡傛灉浣犵殑鏈哄櫒锛堥€氬父鏄瑪璁版湰锛夊甫鏈夊祵鍏ュ紡鎺у埗鍣紙Embedded Controller锛夛紝涓婅堪鍛戒护浼氬湪 ACPI 璁剧疆
鏈熼棿鏄剧ず鏃╂湡鐨勫祵鍏ュ紡鎺у埗鍣ㄤ簨鍔°€侾CI锛堟垨鍏跺畠璁惧锛夊垵濮嬪寲涔熸槸浣跨敤璇ュ惎鍔ㄥ弬鏁拌繘琛岃皟璇曠殑鐑棬
鍊欓€夊満鏅€?
濡傛灉 `foo` 妯″潡涓嶆槸鍐呭缓鐨勶紝`foo.dyndbg` 浠嶄細鍦ㄥ惎鍔ㄦ椂澶勭悊锛屼絾涓嶄細鏈変换浣曟晥鏋滐紝涓嶈繃瀹冧細鍦ㄦā鍧?绋嶅悗琚姞杞芥椂閲嶆柊琚鐞嗐€傚崟鐙殑 `dyndbg=` 鍙湪鍚姩鏃跺鐞嗐€?
## 妯″潡鍒濆鍖栨椂鐨勮皟璇曟秷鎭紙Debug Messages at Module Initialization Time锛?
褰撹皟鐢?`modprobe foo` 鏃讹紝modprobe 浼氭壂鎻?`/proc/cmdline` 涓殑 `foo.params`锛屽幓鎺?`foo.`锛?骞朵笌 modprobe 鍙傛暟鎴?`/etc/modprobe.d/*.conf` 鏂囦欢涓粰瀹氱殑鍙傛暟涓€璧蜂紶缁欏唴鏍革紝椤哄簭濡備笅锛?```

	options foo dyndbg=+pt
	options foo dyndbg # defaults to +p

```
```

	foo.dyndbg=" func bar +p; func buz +mp"

```
```

	modprobe foo dyndbg==pmf # override previous settings

```
杩欎簺 `dyndbg` 鏌ヨ鎸夐『搴忓簲鐢紝鏈€鍚庝竴鏉″叿鏈夋渶缁堝喅瀹氭潈銆傝繖鏍凤紝鍚姩鍙傛暟鍙互瑕嗙洊鎴栦慨鏀规潵鑷?`/etc/modprobe.d` 鐨勮缃紙杩欏緢鍚堢悊锛屽洜涓?1 鏄郴缁熻寖鍥寸殑锛? 鏄唴鏍告垨鍚姩鐗瑰畾鐨勶級锛岃€?modprobe
鍙傛暟鍒欏彲浠ヨ鐩栬繖涓よ€呫€?
鍦?`foo.dyndbg="QUERY"` 褰㈠紡涓紝鏌ヨ蹇呴』鎺掗櫎 `module foo`銆俙foo` 浼氫粠鍙傛暟鍚嶄腑鎻愬彇鍑烘潵锛屽苟
搴旂敤鍒?`QUERY` 涓殑姣忎釜鏌ヨ锛屽苟涓旀瘡绉嶇被鍨嬪彧鍏佽涓€涓?match-spec銆?
`dyndbg` 閫夐」鏄竴涓?浼?妯″潡鍙傛暟锛岃繖鎰忓懗鐫€锛?
- 妯″潡涓嶉渶瑕佹樉寮忓畾涔夊畠
- 姣忎釜妯″潡閮戒細闅愬紡鑾峰緱瀹冿紝鏃犺鏄惁浣跨敤浜?pr_debug
- 瀹冧笉浼氬嚭鐜板湪 `/sys/module/$module/parameters/` 涓?  瑕佹煡鐪嬪畠锛屽彲浠?grep 鎺у埗鏂囦欢锛屾垨妫€鏌?`/proc/cmdline.`

瀵逛簬 `CONFIG_DYNAMIC_DEBUG` 鍐呮牳锛屽惎鍔ㄦ椂缁欏畾鐨勪换浣曡缃紙鎴栧湪缂栬瘧鏈熼棿鐢?`-DDEBUG` 鏍囧織鍚敤
鐨勶級涔嬪悗閮藉彲浠ラ€氳繃濡備笅鏂瑰紡绂佺敤锛?```

   echo "module module_name -p" > /proc/dynamic_debug/control

```
## 绀轰緥锛圗xamples锛?
```

  // enable the message at line 1603 of file svcsock.c
  :#> ddcmd 'file svcsock.c line 1603 +p'

  // enable all the messages in file svcsock.c
  :#> ddcmd 'file svcsock.c +p'

  // enable all the messages in the NFS server module
  :#> ddcmd 'module nfsd +p'

  // enable all 12 messages in the function svc_process()
  :#> ddcmd 'func svc_process +p'

  // disable all 12 messages in the function svc_process()
  :#> ddcmd 'func svc_process -p'

  // enable messages for NFS calls READ, READLINK, READDIR and READDIR+.
  :#> ddcmd 'format "nfsd: READ" +p'

  // enable messages in files of which the paths include string "usb"
  :#> ddcmd 'file *usb* +p'

  // enable all messages
  :#> ddcmd '+p'

  // add module, function to all enabled messages
  :#> ddcmd '+mf'

  // boot-args example, with newlines and comments for readability
  Kernel command line: ...
    // see what's going on in dyndbg=value processing
    dynamic_debug.verbose=3
    // enable pr_debugs in the btrfs module (can be builtin or loadable)
    btrfs.dyndbg="+p"
    // enable pr_debugs in all files under init/
    // and the function parse_one, #cmt is stripped
    dyndbg="file init/* +p #cmt ; func parse_one +p"
    // enable pr_debugs in 2 functions in a module loaded later
    pc87360.dyndbg="func pc87360_init_device +p; func pc87360_find +p"

```
## 鍐呮牳閰嶇疆锛圞ernel Configuration锛?
```

  CONFIG_DYNAMIC_DEBUG=y	# build catalog, enables CORE
  CONFIG_DYNAMIC_DEBUG_CORE=y	# enable mechanics only, skip catalog

```
濡傛灉浣犱笉鎯冲叏灞€鍚敤 dynamic debug锛堜緥濡傚湪鏌愪簺宓屽叆寮忕郴缁熶腑锛夛紝浣犲彲浠ユ妸 `CONFIG_DYNAMIC_DEBUG_CORE`
璁剧疆涓?dynamic debug 鐨勫熀纭€鏀寔锛屽苟鍦ㄤ綘甯屾湜绋嶅悗杩涜鍔ㄦ€佽皟璇曠殑浠讳綍妯″潡鐨?Makefile 涓姞鍏?`ccflags := -DDYNAMIC_DEBUG_MODULE`銆?
## 鍐呮牳 *prdbg* API

浠ヤ笅鍑芥暟鍦ㄥ惎鐢?dynamic debug 鏃朵細琚紪鍏ョ洰褰曞苟鍙鎺у埗锛?```

  pr_debug()
  dev_dbg()
  print_hex_dump_debug()
  print_hex_dump_bytes()

```
鍚﹀垯锛屽畠浠粯璁ゆ槸鍏抽棴鐨勶紱鍦ㄦ簮鏂囦欢涓娇鐢?`ccflags += -DDEBUG` 鎴?`#define DEBUG` 浼氶€傚綋鍦?鍚敤瀹冧滑銆?
濡傛灉鏈缃?`CONFIG_DYNAMIC_DEBUG`锛屽垯 `print_hex_dump_debug()` 鍙槸 `print_hex_dump(KERN_DEBUG)`
鐨勫揩鎹锋柟寮忋€?
瀵逛簬 `print_hex_dump_debug()`/`print_hex_dump_bytes()`锛屽叾 format 瀛楃涓叉槸 `prefix_str`
鍙傛暟锛堝鏋滃畠鏄父閲忓瓧绗︿覆锛夛紝鎴栬€呮槸鍦?`prefix_str` 琚姩鎬佹瀯閫犳椂鐨?`hexdump`銆?