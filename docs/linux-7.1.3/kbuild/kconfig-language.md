## Kconfig 璇█


### 绠€浠?


閰嶇疆鏁版嵁搴撴槸涓€绯诲垪閰嶇疆閫夐」鐨勯泦鍚?
```

	+- Code maturity level options
	|  +- Prompt for development and/or incomplete code/drivers
	+- General setup
	|  +- Networking support
	|  +- System V IPC
	|  +- BSD Process Accounting
	|  +- Sysctl support
	+- Loadable module support
	|  +- Enable loadable module support
	|     +- Set version information on all module symbols
	|     +- Kernel module loader
	+- ...

```
姣忎釜鏉＄洰閮芥湁鑷繁鐨勪緷璧栧叧绯汇€傝繖浜涗緷璧栧叧绯荤敤浜庣‘瀹氭潯鐩殑鍙鎬с€備换浣曞瓙鏉＄洰鍙湁鍦ㄧ埗鏉＄洰涔熷彲瑙佹椂鎵嶅彲瑙併€?

### 鑿滃崟鏉＄洰


澶氭暟鏉＄洰瀹氫箟涓€涓厤缃€夐」锛涘叾浣欐潯鐩敤浜庣粍缁?
```

  config MODVERSIONS
	bool "Set version information on all module symbols"
	depends on MODULES
	help
	  Usually, modules have to be recompiled whenever you switch to a new
	  kernel.  ...

```
姣忎竴琛岄兘浠ヤ竴涓叧閿瓧寮€澶达紝鍏跺悗鍙窡澶氫釜鍙傛暟銆?config" 寮€濮嬩竴涓柊鐨勯厤缃潯鐩€傞殢鍚庣殑琛屽畾涔夎閰嶇疆閫夐」鐨勫睘鎬с€傚睘鎬у彲浠ユ槸閰嶇疆閫夐」鐨勭被鍨嬨€佽緭鍏ユ彁绀恒€佷緷璧栧叧绯汇€佸府鍔╂枃鏈互鍙婇粯璁ゅ€笺€備竴涓厤缃€夐」鍙互浣跨敤鐩稿悓鍚嶇О澶氭瀹氫箟锛屼絾姣忎釜瀹氫箟鍙兘鏈変竴涓緭鍏ユ彁绀猴紝涓旂被鍨嬩笉寰楀啿绐併€?

### 鑿滃崟灞炴€?


涓€涓彍鍗曟潯鐩彲浠ュ寘鍚嫢骞插睘鎬с€傚苟闈炴墍鏈夊睘鎬у湪浠绘剰浣嶇疆閮介€傜敤锛堝弬瑙佽娉曪級銆?

- 绫诲瀷瀹氫箟锛?bool"/"tristate"/"string"/"hex"/"int"

  姣忎釜閰嶇疆閫夐」閮藉繀椤绘湁涓€涓被鍨嬨€傚熀鏈被鍨嬪彧鏈変袱绉嶏細tristate 鍜?string锛涘叾浠栫被鍨嬮兘鍩轰簬杩欎袱绉嶃€傜被鍨嬪畾涔夊彲閫夋嫨鎬у湴鎺ュ彈涓€涓緭鍏ユ彁绀猴紝鍥犳浠ヤ笅涓や釜绀轰緥
```

	bool "Networking support"

  and::

	bool
	prompt "Networking support"

```
- 杈撳叆鎻愮ず锛?prompt" <prompt> ["if" <expr>]

  姣忎釜鑿滃崟鏉＄洰鏈€澶氬彧鑳芥湁涓€涓彁绀猴紝鐢ㄤ簬鏄剧ず缁欑敤鎴枫€備篃鍙互浠呴拡瀵硅鎻愮ず閫氳繃 "if" 娣诲姞渚濊禆銆傝嫢涓嶅瓨鍦ㄦ彁绀猴紝鍒欒閰嶇疆閫夐」鏄竴涓笉鍙鐨勭鍙凤紝鎰忓懗鐫€鍏跺€兼棤娉曠敱鐢ㄦ埛鐩存帴鏇存敼锛堜緥濡傚湪 `.config` 涓慨鏀硅鍊硷級锛屽苟涓旇閫夐」涓嶄細鍑虹幇鍦ㄤ换浣曢厤缃彍鍗曚腑銆傚叾鍊煎彧鑳介€氳繃 "default" 鍜?"select" 璁剧疆锛堣涓嬫枃锛夈€?

- 榛樿鍊硷細"default" <expr> ["if" <expr>]

  涓€涓厤缃€夐」鍙互鏈変换鎰忔暟閲忕殑榛樿鍊笺€傝嫢澶氫釜榛樿鍊煎彲瑙侊紝鍙湁绗竴涓瀹氫箟鐨勭敓鏁堛€傞粯璁ゅ€煎苟涓嶅眬闄愪簬瀹氫箟瀹冪殑鑿滃崟鏉＄洰銆傝繖鎰忓懗鐫€榛樿鍊煎彲浠ュ湪鍏朵粬鍦版柟瀹氫箟锛屾垨鑰呯敱鏇存棭鐨勫畾涔夎鐩栥€?
  鍙湁褰撶敤鎴锋湭璁剧疆鍏朵粬鍊硷紙閫氳繃涓婇潰鐨勮緭鍏ユ彁绀猴級鏃讹紝榛樿鍊兼墠浼氳祴缁欓厤缃鍙枫€傝嫢杈撳叆鎻愮ず鍙锛屽垯榛樿鍊间細鍛堢幇缁欑敤鎴凤紝骞跺彲鐢辩敤鎴疯鐩栥€?
  涔熷彲浠ヤ粎閽堝璇ラ粯璁ゅ€奸€氳繃 "if" 娣诲姞渚濊禆銆?

 榛樿鍊煎埢鎰忚涓?'n'锛屼互閬垮厤鏋勫缓鍙樺緱鑷冭偪銆傞櫎灏戞暟渚嬪锛屾柊鐨勯厤缃€夐」涓嶅簲鏀瑰彉杩欎竴鐐广€傚叾鎰忓浘鏄 "make oldconfig" 鍦ㄤ笉鍚岀増鏈箣闂村敖閲忓皯鍦板悜閰嶇疆涓柊澧炲唴瀹广€?

 娉ㄦ剰锛?
	绗﹀悎 "default y/m" 鐨勬儏鍐靛寘鎷細

	a) 鏌愬姛鑳借繃鍘绘€绘槸琚瀯寤猴紝涓哄叾鏂板鐨?Kconfig 閫夐」搴旇涓?"default y"銆?

	b) 涓€涓柊鐨勨€滄妸鍏斥€滽config 閫夐」锛岀敤浜庨殣钘?鏄剧ず鍏朵粬 Kconfig 閫夐」锛堜絾鍏惰嚜韬笉鐢熸垚浠讳綍浠ｇ爜锛夛紝搴旇涓?"default y"锛屼互渚跨敤鎴疯兘鐪嬪埌閭ｄ簺鍏朵粬閫夐」銆?

	c) 瀵逛簬 "default n" 鐨勯┍鍔紝鍏跺瓙椹卞姩琛屼负鎴栫被浼奸€夐」銆傝繖鍏佽浣犳彁渚涘悎鐞嗙殑榛樿鍊笺€?

	d) 浜轰汉閮介鏈熷瓨鍦ㄧ殑纭欢鎴栧熀纭€璁炬柦锛屼緥濡?CONFIG_NET 鎴?CONFIG_BLOCK銆傝繖浜涘睘浜庣綍瑙佺殑渚嬪銆?

```

	"def_bool"/"def_tristate" <expr> ["if" <expr>]

  This is a shorthand notation for a type definition plus a value.
  Optionally dependencies for this default value can be added with "if".

```
- 渚濊禆鍏崇郴锛?depends on" <expr> ["if" <expr>]

  杩欎负褰撳墠鑿滃崟鏉＄洰瀹氫箟涓€涓緷璧栥€傝嫢瀹氫箟浜嗗涓緷璧栵紝鍒欏畠浠互 '&&' 杩炴帴銆備緷璧栦細浣滅敤浜庤鑿滃崟鏉＄洰鍐呯殑鎵€鏈夊叾浠栭€夐」锛堝悓鏍蜂篃鍖呮嫭
```

	bool "foo" if BAR
	default y if BAR

  and::

	depends on BAR
	bool "foo"
	default y

  The dependency definition itself may be conditional by appending "if"
  followed by an expression. For example::

    config FOO
	tristate
	depends on BAR if BAZ

  meaning that FOO is constrained by the value of BAR only if BAZ is
  also set.

```
- 鍙嶅悜渚濊禆锛?select" <symbol> ["if" <expr>]

  鏅€氫緷璧栦細闄嶄綆绗﹀彿鐨勪笂闄愶紙瑙佷笅鏂囷級锛岃€屽弽鍚戜緷璧栧彲鐢ㄤ簬寮哄埗鍙︿竴涓鍙风殑涓嬮檺銆傚綋鍓嶈彍鍗曠鍙风殑鍊艰鐢ㄤ綔 <symbol> 鍙缃殑鏈€灏忓€笺€傝嫢 <symbol> 琚娆?select锛屽垯涓嬮檺鍙栨渶澶х殑閫夋嫨鍊笺€?
  鍙嶅悜渚濊禆鍙兘鐢ㄤ簬甯冨皵鎴栦笁鎬佺鍙枫€?

  娉ㄦ剰锛?
	select 搴旇皑鎱庝娇鐢ㄣ€俿elect 浼氬己鍒跺皢涓€涓鍙疯涓烘煇涓€硷紝鑰屼笉浼氭鏌ュ叾渚濊禆銆?
	婊ョ敤 select 鏃讹紝鍗充究 FOO 渚濊禆鐨?BAR 鏈璁剧疆锛屼綘涔熻兘閫変腑绗﹀彿 FOO銆?
	涓€鑸€岃█锛宻elect 浠呯敤浜庝笉鍙绗﹀彿锛堜换浣曞湴鏂归兘娌℃湁鎻愮ず锛変互鍙婃病鏈変緷璧栫殑绗﹀彿銆?
	杩欎細闄嶄綆鍏跺彲鐢ㄦ€э紝浣嗗彟涓€鏂归潰鍙伩鍏嶅埌澶勫嚭鐜扮殑闈炴硶閰嶇疆銆?

	鑻?"select" <symbol> 鍚庤窡 "if" <expr>锛屽垯 <symbol> 灏嗙敱褰撳墠鑿滃崟绗﹀彿鐨勫€间笌 <expr> 鐨勯€昏緫涓庢潵閫変腑銆傝繖鎰忓懗鐫€锛岀敱浜庡瓨鍦?"if" <expr>锛屼笅闄愬彲鑳借闄嶄綆銆傝繖绉嶈涓虹湅浼煎鎬紝浣嗘垜浠湁璧栦簬姝ゃ€傦紙璇ヨ涓虹殑鏈潵璧板悜灏氭湭纭畾銆傦級

- 寮卞弽鍚戜緷璧栵細"imply" <symbol> ["if" <expr>]

  杩欎笌 "select" 绫讳技锛屼篃浼氬鍙︿竴涓鍙峰己鍒朵竴涓笅闄愶紝浣嗗尯鍒湪浜庤 "imply" 鐨勭鍙风殑鍊间粛鍙鐩存帴渚濊禆鎴栧彲瑙佹彁绀鸿涓?n銆?

```

    config FOO
	tristate "foo"
	imply BAZ

    config BAZ
	tristate "baz"
	depends on BAR

  The following values are possible:

	===		===		=============	==============
	FOO		BAR		BAZ's default	choice for BAZ
	===		===		=============	==============
	n		y		n		N/m/y
	m		y		m		M/y/n
	y		y		y		Y/m/n
	n		m		n		N/m
	m		m		m		M/n
	y		m		m		M/n
	y		n		*		N
	===		===		=============	==============

  This is useful e.g. with multiple drivers that want to indicate their
  ability to hook into a secondary subsystem while allowing the user to
  configure that subsystem out without also having to unset these drivers.

  Note: If the feature provided by BAZ is highly desirable for FOO,
  FOO should imply not only BAZ, but also its dependency BAR::

    config FOO
	tristate "foo"
	imply BAR
	imply BAZ

  Note: If "imply" <symbol> is followed by "if" <expr>, the default of <symbol>
  will be the logical AND of the value of the current menu symbol and <expr>.
  (The future of this behavior is undecided.)

```
- 闄愬埗鑿滃崟鏄剧ず锛?visible if" <expr>

  璇ュ睘鎬т粎閫傜敤浜庤彍鍗曞潡锛岃嫢鏉′欢涓哄亣锛屽垯璇ヨ彍鍗曞潡涓嶄細鏄剧ず缁欑敤鎴凤紙涓嶈繃鍏朵腑鍖呭惈鐨勭鍙蜂粛鍙鍏朵粬绗﹀彿閫変腑锛夈€傚畠绫讳技浜庨拡瀵瑰崟涓彍鍗曟潯鐩殑鏉′欢寮?"prompt" 灞炴€с€?visible" 鐨勯粯璁ゅ€间负鐪熴€?

- 鏁板€艰寖鍥达細"range" <symbol> <symbol> ["if" <expr>]

  杩欑敤浜庨檺鍒?int 鍜?hex 绗﹀彿鍙兘杈撳叆鐨勫€肩殑鑼冨洿銆傜敤鎴峰彧鑳借緭鍏ュぇ浜庣瓑浜庣涓€涓鍙枫€佷笖灏忎簬绛変簬绗簩涓鍙风殑鍊笺€?

- 甯姪鏂囨湰锛?help"

  杩欑敤浜庡畾涔夊府鍔╂枃鏈€傚府鍔╂枃鏈殑缁撴潫鐢辩缉杩涘眰绾у喅瀹氾紝鍗抽亣鍒扮涓€琛岀缉杩涘皬浜庡府鍔╂枃鏈琛岀殑閭ｄ竴琛屾椂缁撴潫銆?

- 妯″潡灞炴€э細"modules"
  杩欏０鏄庤绗﹀彿鐢ㄤ綔 MODULES 绗﹀彿锛屽畠涓烘墍鏈夐厤缃鍙峰惎鐢ㄧ涓夌妯″潡鐘舵€併€?
  鏈€澶氬彧鑳芥湁涓€涓鍙疯缃?"modules" 閫夐」銆?

- 杩囨浮灞炴€э細"transitional"
  杩欏０鏄庤绗﹀彿涓鸿繃娓℃€х鍙凤紝鎰忓懗鐫€瀹冨簲鍦ㄩ厤缃湡闂磋澶勭悊锛屼絾浼氳鎺掗櫎鍦ㄦ柊鍐欏叆鐨?.config 鏂囦欢涔嬪銆?
  杩囨浮鎬х鍙峰湪閰嶇疆閫夐」杩佺Щ杩囩▼涓鍚戝悗鍏煎寰堟湁鐢ㄢ€斺€斿畠浠厑璁?olddefconfig 澶勭悊宸叉湁鐨?.config 鏂囦欢锛屽悓鏃剁‘淇濇棫閫夐」涓嶄細鍑虹幇鍦ㄦ柊閰嶇疆涓€?

  杩囨浮鎬х鍙凤細
  - 娌℃湁鎻愮ず锛堝湪鑿滃崟涓鐢ㄦ埛涓嶅彲瑙侊級
  - 鍦ㄩ厤缃湡闂磋姝ｅ父澶勭悊锛堝€间細琚鍙栧拰浣跨敤锛?
  - 鍙鍏朵粬绗﹀彿鐨勯粯璁よ〃杈惧紡寮曠敤
  - 涓嶄細琚啓鍏ユ柊鐨?.config 鏂囦欢
  - 涓嶈兘鎷ユ湁浠讳綍鍏朵粬灞炴€э紙瀹冩槸涓€涓€忎紶閫夐」锛?

```

    config NEW_NAME
	bool "New option name"
	default OLD_NAME
	help
	  This replaces the old CONFIG_OLD_NAME option.

    config OLD_NAME
	bool
	transitional
	help
	  Transitional config for OLD_NAME to NEW_NAME migration.

  With this setup, existing .config files with "CONFIG_OLD_NAME=y" will
  result in "CONFIG_NEW_NAME=y" being set, while CONFIG_OLD_NAME will be
  omitted from newly written .config files.

```
### 鑿滃崟渚濊禆


渚濊禆鍏崇郴瀹氫箟浜嗚彍鍗曟潯鐩殑鍙鎬э紝涔熻兘缂╁皬涓夋€佺鍙风殑杈撳叆鑼冨洿銆傝〃杈惧紡涓娇鐢ㄧ殑涓夋€侀€昏緫姣旀櫘閫氬竷灏旈€昏緫澶氫竴涓姸鎬侊紝鐢ㄤ互琛ㄨ揪
```

  <expr> ::= <symbol>                           (1)
           <symbol> '=' <symbol>                (2)
           <symbol> '!=' <symbol>               (3)
           <symbol1> '<' <symbol2>              (4)
           <symbol1> '>' <symbol2>              (4)
           <symbol1> '<=' <symbol2>             (4)
           <symbol1> '>=' <symbol2>             (4)
           '(' <expr> ')'                       (5)
           '!' <expr>                           (6)
           <expr> '&&' <expr>                   (7)
           <expr> '||' <expr>                   (8)

```
琛ㄨ揪寮忔寜浼樺厛绾т粠楂樺埌浣庡垪鍑恒€?

(1) 灏嗙鍙疯浆鎹负琛ㄨ揪寮忋€傚竷灏斿拰涓夋€佺鍙风洿鎺ヨ浆鎹负鐩稿簲鐨勮〃杈惧紡鍊笺€傛墍鏈夊叾浠栫鍙风被鍨嬬粨鏋滀负 'n'銆?
(2) 鑻ヤ袱涓鍙风殑鍊肩浉绛夛紝杩斿洖 'y'锛屽惁鍒欒繑鍥?'n'銆?
(3) 鑻ヤ袱涓鍙风殑鍊肩浉绛夛紝杩斿洖 'n'锛屽惁鍒欒繑鍥?'y'銆?
(4) 鑻?<symbol1> 鐨勫€煎垎鍒皬浜庛€佸ぇ浜庛€佸皬浜庣瓑浜庢垨澶т簬绛変簬 <symbol2> 鐨勫€硷紝杩斿洖 'y'锛屽惁鍒欒繑鍥?'n'銆?
(5) 杩斿洖琛ㄨ揪寮忕殑鍊笺€傜敤浜庤鐩栦紭鍏堢骇銆?
(6) 杩斿洖 (2-/expr/) 鐨勭粨鏋溿€?
(7) 杩斿洖 min(/expr/, /expr/) 鐨勭粨鏋溿€?
(8) 杩斿洖 max(/expr/, /expr/) 鐨勭粨鏋溿€?

琛ㄨ揪寮忕殑鍊煎彲浠ユ槸 'n'銆?m' 鎴?'y'锛堣绠楁椂鍒嗗埆瀵瑰簲 0銆?銆?锛夈€傚綋鑿滃崟鏉＄洰鐨勮〃杈惧紡姹傚€肩粨鏋滀负 'm' 鎴?'y' 鏃讹紝璇ユ潯鐩彉涓哄彲瑙併€?

绗﹀彿鏈変袱绉嶇被鍨嬶細甯搁噺绗﹀彿鍜岄潪甯搁噺绗﹀彿銆?
闈炲父閲忕鍙锋渶涓哄父瑙侊紝鐢?'config' 璇彞瀹氫箟銆傞潪甯搁噺绗﹀彿瀹屽叏鐢卞瓧姣嶆暟瀛楀瓧绗︽垨涓嬪垝绾跨粍鎴愩€?
甯搁噺绗﹀彿浠呬綔涓鸿〃杈惧紡鐨勪竴閮ㄥ垎瀛樺湪銆傚父閲忕鍙峰缁堣鍗曞紩鍙锋垨鍙屽紩鍙峰寘鍥淬€傚湪寮曞彿鍐呭厑璁稿嚭鐜颁换鎰忓叾浠栧瓧绗︼紝骞朵笖鍙互浣跨敤 '\' 瀵瑰紩鍙疯繘琛岃浆涔夈€?

### 鑿滃崟缁撴瀯


鑿滃崟鏉＄洰鍦ㄦ爲涓殑浣嶇疆鐢变袱绉嶆柟寮忓喅瀹氥€傞鍏?
```

  menu "Network device support"
	depends on NET

  config NETDEVICES
	...

  endmenu

```
"menu" ... "endmenu" 鍧楀唴鐨勬墍鏈夋潯鐩兘浼氭垚涓衡€滅綉缁滆澶囨敮鎸佲€濈殑瀛愯彍鍗曘€傛墍鏈夊瓙鏉＄洰缁ф壙璇ヨ彍鍗曟潯鐩殑渚濊禆锛屼緥濡傝繖鎰忓懗鐫€渚濊禆 "NET" 浼氳鍔犲叆閰嶇疆閫夐」 NETDEVICES 鐨勪緷璧栧垪琛ㄤ腑銆?

鐢熸垚鑿滃崟缁撴瀯鐨勫彟涓€绉嶆柟寮忔槸閫氳繃鍒嗘瀽渚濊禆鍏崇郴銆傝嫢鏌愪釜鑿滃崟鏉＄洰鍦ㄦ煇绉嶇▼搴︿笂渚濊禆浜庡墠涓€涓潯鐩紝鍒欏彲灏嗗叾璁句负鍓嶈€呯殑瀛愯彍鍗曘€傞鍏堬紝鍓嶄竴涓紙鐖剁骇锛夌鍙峰繀椤绘槸渚濊禆鍒楄〃鐨勪竴閮ㄥ垎锛屼笖浠ヤ笅涓や釜鏉′欢涔嬩竴蹇呴』鎴愮珛锛?

- 鑻ョ埗绾ц璁句负 'n'锛屽瓙鏉＄洰蹇呴』鍙樹负涓嶅彲瑙?
```

    config MODULES
	bool "Enable loadable module support"

    config MODVERSIONS
	bool "Set version information on all module symbols"
	depends on MODULES

    comment "module support disabled"
	depends on !MODULES

```
MODVERSIONS 鐩存帴渚濊禆浜?MODULES锛岃繖鎰忓懗鐫€浠呭綋 MODULES 涓嶄负 'n' 鏃舵墠鍙銆傚彟涓€鏂归潰锛岃娉ㄩ噴浠呭綋 MODULES 璁句负 'n' 鏃舵墠鍙銆?


### Kconfig 璇硶


閰嶇疆鏂囦欢鎻忚堪涓€绯诲垪鑿滃崟鏉＄洰锛屽叾涓瘡涓€琛岄兘浠ュ叧閿瓧寮€澶达紙甯姪鏂囨湰闄ゅ锛夈€備互涓嬪叧閿瓧浼氱粨鏉熶竴涓彍鍗曟潯鐩細

- config
- menuconfig
- choice/endchoice
- comment
- menu/endmenu
- if/endif
- source

鍓嶄簲涓叧閿瓧鍚屾椂涔熷紑鍚竴涓彍鍗曟潯鐩殑瀹氫箟銆?

```

	"config" <symbol>
	<config options>

```
杩欏畾涔変簡涓€涓厤缃鍙?<symbol>锛屽苟鎺ュ彈涓婅堪浠绘剰灞炴€т綔涓洪€夐」銆?

```

	"menuconfig" <symbol>
	<config options>

```
杩欎笌涓婇潰鐨勭畝鍗?config 鏉＄洰绫讳技锛屼絾瀹冨悜鍓嶇缁欏嚭鎻愮ず锛氭墍鏈夊瓙閫夐」搴斾綔涓轰竴涓嫭绔嬬殑閫夐」鍒楄〃鏄剧ず銆備负纭繚鎵€鏈夊瓙閫夐」纭疄鍑虹幇鍦?menuconfig 鏉＄洰涔嬩笅銆佽€岄潪鍏朵箣澶栵紝<config options> 鍒楄〃涓殑姣忎竴椤归兘蹇呴』渚濊禆浜庤 menuconfig 绗﹀彿銆?
```

  (1):
  menuconfig M
  if M
      config C1
      config C2
  endif

  (2):
  menuconfig M
  config C1
      depends on M
  config C2
      depends on M

```
鍦ㄤ笅闈㈢殑绀轰緥 (3) 鍜?(4) 涓紝C1 鍜?C2 浠嶇劧鍏锋湁 M 渚濊禆锛屼絾涓嶅啀鍑虹幇鍦?menuconfig M 涔嬩笅锛屽洜涓?
```

  (3):
  menuconfig M
      config C0
  if M
      config C1
      config C2
  endif

  (4):
  menuconfig M
  config C0
  config C1
      depends on M
  config C2
      depends on M

```
```

	"choice"
	<choice options>
	<choice block>
	"endchoice"

```
杩欏畾涔変簡涓€涓?choice 缁勶紝骞舵帴鍙?"prompt"銆?default"銆?depends on" 鍜?"help" 灞炴€т綔涓洪€夐」銆?

涓€涓?choice 鍙厑璁搁€変腑鍗曚釜閰嶇疆鏉＄洰銆?

```

	"comment" <prompt>
	<comment options>

```
杩欏畾涔変簡涓€涓敞閲婏紝鍦ㄩ厤缃繃绋嬩腑鏄剧ず缁欑敤鎴凤紝鍚屾椂涔熶細琚洖鏄惧埌杈撳嚭鏂囦欢涓€傚敮涓€鍙兘鐨勯€夐」鏄緷璧栥€?

```

	"menu" <prompt>
	<menu options>
	<menu block>
	"endmenu"

```
杩欏畾涔変簡涓€涓彍鍗曞潡锛岃瑙佷笂鏂団€滆彍鍗曠粨鏋勨€濄€傚敮涓€鍙兘鐨勯€夐」鏄緷璧栧拰 "visible" 灞炴€с€?

```

	"if" <expr>
	<if block>
	"endif"

```
杩欏畾涔変簡涓€涓?if 鍧椼€備緷璧栬〃杈惧紡 <expr> 浼氳杩藉姞鍒版墍鏈夎鍖呭惈鐨勮彍鍗曟潯鐩笂銆?

```

	"source" <prompt>

```
杩欎細璇诲彇鎸囧畾鐨勯厤缃枃浠躲€傝鏂囦欢鎬绘槸琚В鏋愩€?

```

	"mainmenu" <prompt>

```
鑻ラ厤缃▼搴忛€夋嫨浣跨敤锛岃繖浼氳缃叾鏍囬鏍忋€傚畠搴旀斁鍦ㄩ厤缃殑鏈€椤堕儴銆佷换浣曞叾浠栬鍙ヤ箣鍓嶃€?

'#' Kconfig 婧愭枃浠舵敞閲婏細

鍦ㄦ簮鏂囦欢琛岀殑浠绘剰浣嶇疆锛屾湭鍔犲紩鍙风殑 '#' 瀛楃琛ㄧず璇ユ簮鏂囦欢娉ㄩ噴鐨勫紑濮嬨€傝琛屽墿浣欓儴鍒嗗嵆涓烘敞閲娿€?


### Kconfig 鎻愮ず

杩欐槸涓€缁?Kconfig 鎶€宸э紝鍏朵腑澶ч儴鍒嗕箥鐪嬪苟涓嶆槑鏄撅紝涓斿鏁板凡鎴愪负澶氫釜 Kconfig 鏂囦欢涓殑鎯敤娉曘€?

#### 娣诲姞閫氱敤鐗规€у苟浣跨敤娉曞彲閰嶇疆

瀹炵幇鏌愪簺鐗规€?鍔熻兘锛岃繖浜涚壒鎬т粎涓庨儴鍒嗚€岄潪鍏ㄩ儴鏋舵瀯鐩稿叧锛岃繖鏄竴绉嶅父瑙佹儻鐢ㄦ硶銆?
鎺ㄨ崘鐨勫仛娉曟槸浣跨敤涓€涓悕涓?HAVE_* 鐨勯厤缃彉閲忥紝瀹冨湪閫氱敤鐨?Kconfig 鏂囦欢涓畾涔夛紝骞剁敱鐩稿叧鐨勬灦鏋勯€変腑銆?
閫氱敤 IOMAP 鍔熻兘鍗虫槸涓€渚嬨€?

```

  # Generic IOMAP is used to ...
  config HAVE_GENERIC_IOMAP

  config GENERIC_IOMAP
	depends on HAVE_GENERIC_IOMAP && FOO

```
```

	obj-$(CONFIG_GENERIC_IOMAP) += iomap.o

```
```

  config X86
	select ...
	select HAVE_GENERIC_IOMAP
	select ...

```
娉ㄦ剰锛氭垜浠娇鐢ㄥ凡鏈夌殑閰嶇疆閫夐」锛岄伩鍏嶆柊寤轰竴涓厤缃彉閲忔潵閫変腑 HAVE_GENERIC_IOMAP銆?

娉ㄦ剰锛氳繖閲屼娇鐢ㄤ簡鍐呴儴閰嶇疆鍙橀噺 HAVE_GENERIC_IOMAP锛屽紩鍏ュ畠鏄负浜嗗厠鏈?select 鐨勯檺鍒垛€斺€攕elect 浼氭棤瑙嗕緷璧栬€屽皢閰嶇疆閫夐」寮哄埗璁句负 'y'銆?
渚濊禆琚Щ鍒颁簡绗﹀彿 GENERIC_IOMAP 涓婏紝浠庤€岄伩鍏嶄簡 select 灏嗘煇涓鍙峰己鍒惰涓?'y' 鐨勬儏鍐点€?

#### 娣诲姞闇€瑕佺紪璇戝櫒鏀寔鐨勭壒鎬?


鏈夎嫢骞茬壒鎬ч渶瑕佺紪璇戝櫒鏀寔銆傛弿杩板缂栬瘧鍣ㄧ壒鎬х殑渚濊禆鐨勬帹鑽愭柟寮忔槸浣跨敤 "depends on"
```

  config STACKPROTECTOR
	bool "Stack Protector buffer overflow detection"
	depends on $(cc-option,-fstack-protector)
	...

```
鑻ヤ綘闇€瑕佸悜 makefile 鍜?鎴?C 婧愭枃浠舵毚闇茬紪璇戝櫒鑳藉姏锛?
```

  config CC_HAS_FOO
	def_bool $(success,$(srctree)/scripts/cc-check-foo.sh $(CC))

```

#### 浠呬綔涓烘ā鍧楁瀯寤?

瑕佸皢鏌愮粍浠剁殑鏋勫缓闄愬埗涓轰粎妯″潡锛屽彲瀵瑰叾閰嶇疆绗﹀彿闄愬畾
```

  config FOO
	depends on BAR && m

```
杩欏皢 FOO 闄愬埗涓烘ā鍧楋紙=m锛夋垨绂佺敤锛?n锛夈€?

#### 缂栬瘧娴嬭瘯

鑻ユ煇涓厤缃鍙峰瓨鍦ㄤ緷璧栵紝浣嗙敱璇ラ厤缃鍙锋帶鍒剁殑浠ｇ爜鍦ㄤ緷璧栦笉婊¤冻鏃朵粛鍙紪璇戯紝鍒欏缓璁€氳繃鍦ㄤ緷璧栦腑娣诲姞 "|| COMPILE_TEST" 瀛愬彞鏉ユ彁楂樻瀯寤鸿鐩栫巼銆傝繖瀵逛簬杈冨喎闂ㄧ‖浠剁殑椹卞姩灏ゅ叾鏈夌敤锛屽洜涓哄畠鍏佽鎸佺画闆嗘垚绯荤粺鍦ㄦ洿甯歌鐨勭郴缁熶笂瀵硅浠ｇ爜杩涜缂栬瘧娴嬭瘯锛屼粠鑰屽彂鐜扮己闄枫€?
璇锋敞鎰忥紝琚紪璇戞祴璇曠殑浠ｇ爜搴旈伩鍏嶅湪渚濊禆涓嶆弧瓒崇殑绯荤粺涓婅繍琛屾椂宕╂簝銆?

#### 鏋舵瀯涓庡钩鍙颁緷璧?

鐢变簬瀛樺湪妗╁嚱鏁帮紙stub锛夛紝鐜板湪澶у鏁伴┍鍔ㄩ兘鍙互鍦ㄥぇ澶氭暟鏋舵瀯涓婄紪璇戙€傜劧鑰岋紝杩欏苟涓嶆剰鍛崇潃鍦ㄦ墍鏈夊湴鏂归兘鎻愪緵鎵€鏈夐┍鍔ㄦ槸鍚堢悊鐨勶紝鍥犱负瀹為檯纭欢鍙兘鍙瓨鍦ㄤ簬鐗瑰畾鐨勬灦鏋勫拰骞冲彴涓娿€傚浜庣墖涓婏紙on-SoC锛塈P 鏍稿挨鍏跺姝わ紝瀹冧滑鍙兘浠呴檺浜庣壒瀹氱殑鍘傚晢鎴?SoC 绯诲垪銆?

涓洪伩鍏嶅悜鐢ㄦ埛璇㈤棶閭ｄ簺鏃犳硶鐢ㄤ簬鍏舵鍦ㄧ紪璇戝唴鏍哥殑鐩爣绯荤粺鐨勯┍鍔紝鍦ㄥ悎鐞嗙殑鎯呭喌涓嬶紝鎺у埗椹卞姩缂栬瘧鐨勯厤缃鍙峰簲鍖呭惈閫傚綋鐨勪緷璧栵紝灏嗚绗﹀彿鐨勫彲瑙佹€ч檺鍒跺湪椹卞姩鍙繍琛岀殑骞冲彴锛堢殑瓒呴泦锛変笂銆備緷璧栧彲浠ユ槸涓€涓灦鏋勪緷璧栵紙渚嬪 ARM锛夋垨骞冲彴渚濊禆锛堜緥濡?ARCH_OMAP4锛夈€傝繖涓嶄粎璁╁彂琛岀増閰嶇疆缁存姢鑰呮洿杞绘澗锛屼篃璁╂瘡涓€浣嶉厤缃唴鏍哥殑寮€鍙戣€呮垨鐢ㄦ埛鏇磋交鏉俱€?

杩欑渚濊禆鍙互閫氳繃涓庝笂闈㈢殑缂栬瘧娴嬭瘯瑙勫垯缁撳悎鑰屾斁瀹斤紝鍗筹細

  config FOO
	bool "Support for foo hardware"
	depends on ARCH_FOO_VENDOR || COMPILE_TEST

#### 鍙€変緷璧?


鏌愪簺椹卞姩鑳藉閫夋嫨鎬у湴浣跨敤鏉ヨ嚜鍙︿竴涓ā鍧楃殑鐗规€э紝鎴栧湪绂佺敤璇ユā鍧楁椂骞插噣鍦版瀯寤猴紝浣嗗湪灏濊瘯浠庡唴寤洪┍鍔ㄤ娇鐢ㄨ鍙姞杞芥ā鍧楁椂浼氬鑷撮摼鎺ュけ璐ャ€?

鍦?Kconfig 閫昏緫涓〃杈捐繖绉嶅彲閫変緷璧栫殑鎺ㄨ崘鏂瑰紡鏄?
```

  config FOO
	tristate "Support for foo hardware"
	depends on BAR if BAR

```
```

  config FOO
	tristate "Support for foo hardware"
	depends on BAR || !BAR

```
杩欐剰鍛崇潃瑕佷箞瀛樺湪涓€涓 BAR 鐨勪緷璧栵紝绂佹 FOO=y 涓?BAR=m 缁勫悎锛岃涔?BAR 琚畬鍏ㄧ鐢ㄣ€侭AR 妯″潡蹇呴』涓?!BAR 鐨勬儏鍐垫彁渚涙墍鏈夋々鍑芥暟銆?

鑻ュ瓨鍦ㄥ涓叿鏈夋绫讳緷璧栫殑椹卞姩锛屽彲閲囩敤鏇村舰寮忓寲鐨勬柟娉?
```

  config FOO
	tristate "Support for foo hardware"
	depends on BAR_OPTIONAL

  config BAR_OPTIONAL
	def_tristate BAR || !BAR

```
琛ㄨ揪鍙€変緷璧栬緝涓嶆帹鑽愮殑鏂瑰紡鏄ā鍧椾唬鐮佷腑鐨?IS_REACHABLE()锛屼緥濡傚綋妯″潡 BAR 涓嶆彁渚?
```

	foo_init()
	{
		if (IS_REACHABLE(CONFIG_BAR))
			bar_register(&foo);
		...
	}

```
涓€鑸笉寤鸿浣跨敤 IS_REACHABLE()锛屽洜涓哄綋 CONFIG_BAR=m 涓旇浠ｇ爜涓哄唴寤烘椂锛屼唬鐮佷細琚潤榛樹涪寮冦€傝繖骞堕潪鐢ㄦ埛鍦ㄥ皢 BAR 鍚敤涓烘ā鍧楁椂閫氬父鎵€鏈熸湜鐨勩€?

#### Kconfig 閫掑綊渚濊禆鐨勯檺鍒?


濡傛灉浣犻亣鍒颁簡 Kconfig 閿欒锛氣€渞ecursive dependency detected鈥濓紙妫€娴嬪埌閫掑綊渚濊禆锛夛紝璇存槑浣犵鍒颁簡 Kconfig 鐨勯€掑綊渚濊禆闂锛岄€掑綊渚濊禆鍙鎷负寰幆渚濊禆銆俴config 宸ュ叿闇€瑕佺‘淇?Kconfig 鏂囦欢绗﹀悎鎸囧畾鐨勯厤缃姹傘€備负姝わ紝kconfig 蹇呴』纭畾鎵€鏈?Kconfig 绗﹀彿鍙兘鍙栧埌鐨勫€硷紝鑰屽綋涓や釜鎴栧涓?Kconfig 绗﹀彿涔嬮棿瀛樺湪寰幆鍏崇郴鏃讹紝鐩墠鏃犳硶鍋氬埌杩欎竴鐐广€傛洿澶氱粏鑺傝鍙傞槄涓嬫枃鐨勨€滅畝鍗?Kconfig 閫掑綊闂鈥濆皬鑺傘€侹config 涓嶈繘琛岄€掑綊渚濊禆瑙ｆ瀽锛涜繖瀵?Kconfig 鏂囦欢缂栧啓鑰呮湁鍑犱釜褰卞搷銆傛垜浠皢鍏堣В閲婅闂涓轰綍瀛樺湪锛岀劧鍚庣粰鍑轰竴涓敱姝ゅ甫缁?Kconfig 寮€鍙戣€呯殑鎶€鏈€ч檺鍒剁ず渚嬨€傚笇鏈涘皾璇曡В鍐虫闄愬埗鐨勭Н鏋佸紑鍙戣€呭簲闃呰涓嬮潰鐨勫皬鑺傘€?

#### 绠€鍗?Kconfig 閫掑綊闂


鍙傞槄锛欴ocumentation/kbuild/Kconfig.recursion-issue-01

```

  make KBUILD_KCONFIG=Documentation/kbuild/Kconfig.recursion-issue-01 allnoconfig

```
#### 绱Н鍨?Kconfig 閫掑綊闂


鍙傞槄锛欴ocumentation/kbuild/Kconfig.recursion-issue-02

```

  make KBUILD_KCONFIG=Documentation/kbuild/Kconfig.recursion-issue-02 allnoconfig

```
#### Kconfig 閫掑綊闂鐨勫疄鐢ㄨВ鍐虫柟妗?


閬囧埌 Kconfig 閫掑綊闂鐨勫紑鍙戣€呮湁涓や釜鍙€夋柟妗堛€傛垜浠湪涓嬫枃涓褰曞畠浠紝骞舵彁渚涗竴涓€氳繃杩欎簺涓嶅悓鏂规瑙ｅ喅鐨勫巻鍙查棶棰樺垪琛ㄣ€?

  a) 绉婚櫎浠讳綍澶氫綑鐨?"select FOO" 鎴?"depends on FOO"
  b) 鍖归厤渚濊禆璇箟锛?

	b1) 灏嗘墍鏈?"select FOO" 鏇挎崲涓?"depends on FOO"锛屾垨锛?

	b2) 灏嗘墍鏈?"depends on FOO" 鏇挎崲涓?"select FOO"

鏂规 a) 鐨勮В鍐虫柟寮忓彲浠ョ敤绀轰緥 Kconfig 鏂囦欢 Documentation/kbuild/Kconfig.recursion-issue-01 楠岃瘉锛氫粠 CORE_BELL_A_ADVANCED 涓Щ闄?"select CORE"锛屽洜涓虹敱浜?CORE_BELL_A 渚濊禆浜?CORE锛岃繖宸茬粡鏄殣鍚殑銆傛湁鏃跺彲鑳芥棤娉曠Щ闄ゆ煇浜涗緷璧栨潯浠讹紝杩欑鎯呭喌涓嬪彲浣跨敤鏂规 b)銆?

鏂规 b) 鐨勪袱绉嶄笉鍚岃В鍐虫柟寮忓彲鍦ㄧず渚?Kconfig 鏂囦欢 Documentation/kbuild/Kconfig.recursion-issue-02 涓獙璇併€?

浠ヤ笅鏄鍓嶉拡瀵规绫婚€掑綊闂鐨勪慨澶嶇ず渚嬪垪琛紱鎵€鏈夐敊璇技涔庨兘娑夊強涓€涓垨澶氫釜 "select" 璇彞浠ュ強涓€涓垨澶氫釜 "depends on"銆?

============    ===================================
鎻愪氦            淇
============    ===================================
06b718c01208    select A -> depends on A
c22eacfe82f9    depends on A -> depends on B
6a91e854442c    select A -> depends on A
118c565a8f2e    select A -> select B
f004e5594705    select A -> depends on A
c7861f37b4c6    depends on A -> (null)
80c69915e5fb    select A -> (null)              (1)
c2218e26c0d0    select A -> depends on A        (1)
d6ae99d04e1c    select A -> depends on A
95ca19cf8cbf    select A -> depends on A
8f057d7bca54    depends on A -> (null)
8f057d7bca54    depends on A -> select A
a0701f04846e    select A -> depends on A
0c8b92f7f259    depends on A -> (null)
e4e9e0540928    select A -> depends on A        (2)
7453ea886e87    depends on A > (null)           (1)
7b1fff7e4fdf    select A -> depends on A
86c747d2a4f0    select A -> depends on A
d9f9ab51e55e    select A -> depends on A
0c51a4d8abd6    depends on A -> select A        (3)
e98062ed6dc4    select A -> depends on A        (3)
91e5d284a7f1    select A -> (null)
============    ===================================

(1) 瀵归敊璇殑閮ㄥ垎锛堟垨鏈級寮曠敤銆?
(2) 杩欎技涔庢槸璇ヤ慨澶嶇殑瑕佺偣銆?
(3) 鍚屾牱鐨勯敊璇€?

#### 鏈潵鐨?kconfig 宸ヤ綔


娆㈣繋鍦?kconfig 鐨勪袱涓柟鍚戜笂寮€灞曞伐浣滐細鍘樻竻璇箟锛屼互鍙婅瘎浼颁娇鐢ㄥ畬鏁寸殑 SAT 姹傝В鍣ㄣ€傚畬鏁寸殑 SAT 姹傝В鍣ㄦ湁鍔╀簬鏀寔鏇村鏉傜殑渚濊禆鏄犲皠鍜?鎴栨煡璇紝渚嬪 SAT 姹傝В鍣ㄧ殑涓€涓彲鑳界敤閫旀槸澶勭悊褰撳墠宸茬煡鐨勯€掑綊渚濊禆闂銆傜洰鍓嶅皻涓嶆竻妤氳繖鏄惁鑳借В鍐虫绫婚棶棰橈紝浣嗚繖鏍风殑璇勪及鏄€煎緱鐨勩€傚鏋滃瀹屾暣 SAT 姹傝В鍣ㄧ殑鏀寔琚瘉鏄庤繃浜庡鏉傦紝鎴栨棤娉曡В鍐抽€掑綊渚濊禆闂锛岄偅涔?Kconfig 鑷冲皯搴旀嫢鏈夋竻鏅颁笖瀹氫箟鑹ソ鐨勮涔夛紝骞堕槓鏄庡拰璁板綍璇稿閫掑綊渚濊禆鐩稿叧鐨勯檺鍒舵垨瑕佹眰銆?

Kconfig 娆㈣繋鍦ㄨ繖涓や釜鏂瑰悜涓婄殑杩涗竴姝ュ伐浣溿€傛垜浠湪鎺ヤ笅鏉ヤ袱涓皬鑺備腑鍒嗗埆璇﹁堪銆?

#### Kconfig 鐨勮涔?


Kconfig 鐨勪娇鐢ㄥ崄鍒嗗箍娉涳紝Linux 鐜板湪鍙槸 Kconfig 鐨勭敤鎴蜂箣涓€锛氫竴椤圭爺绌跺凡瀹屾垚瀵?12 涓」鐩腑 Kconfig 鐢ㄦ硶鐨勫箍娉涘垎鏋?[^0^]_銆?
灏界 Kconfig 琚箍娉涗娇鐢紝涓斿敖绠℃湰鏂囨。鍦ㄨ褰曞熀鏈?Kconfig 璇硶鏂归潰鍋氬緱涓嶉敊锛屼絾浠嶆杩庡 Kconfig 璇箟缁欏嚭鏇寸簿纭殑瀹氫箟銆傛湁涓€涓」鐩€氳繃 xconfig 閰嶇疆鍣ㄦ帹瀵煎嚭 Kconfig 璇箟 [^1^]_銆傚簲寮€灞曞伐浣滀互纭鎺ㄥ鍑虹殑璇箟鏄惁绗﹀悎鎴戜滑棰勬湡鐨?Kconfig 璁捐鐩爣銆?
鍙︿竴涓」鐩舰寮忓寲浜?Kconfig 璇█鏍稿績瀛愰泦鐨勬寚绉拌涔?[^10^]_銆?

鎷ユ湁瀹氫箟鑹ソ鐨勮涔夛紝瀵圭敤浜庡疄闄呰瘎浼颁緷璧栫殑宸ュ叿寰堟湁甯姪锛屼緥濡傛湁涓€椤瑰伐浣滃皢鎺ㄥ鍑虹殑 Kconfig 璇箟鐢ㄥ竷灏旀娊璞¤〃杈撅紝灏?Kconfig 閫昏緫杞崲涓哄竷灏斿叕寮忓苟鍦ㄦ涓婅繍琛?SAT 姹傝В鍣紝浠ュ彂鐜版浠ｇ爜/鐗规€э紙濮嬬粓涓嶆椿璺冿級锛屼娇鐢ㄨ鏂规硶鍦?Linux 涓彂鐜颁簡 114 涓鐗规€?[^1^]_锛堢 8 鑺傦細鏈夋晥鎬у▉鑳侊級銆?
鍩轰簬 [^10^]_ 涓涔夌殑 kismet 宸ュ叿锛岃兘鍙戠幇瀵瑰弽鍚戜緷璧栫殑婊ョ敤锛屽苟宸蹭績鎴愬 Linux Kconfig 鏂囦欢鐨勫嚑鍗佸宸插悎鍏ヤ慨澶?[^11^]_銆?

纭杩欎竴鐐瑰彲鑳藉緢鏈夌敤锛屽洜涓?Kconfig 鏄渶閲嶈鐨勫伐涓氱骇鍙樹綋寤烘ā璇█涔嬩竴 [^1^]_ [^2^]_銆傚鍏剁爺绌舵湁鍔╀簬璇勪及姝ょ被璇█鐨勫疄闄呯敤閫旓紝杩囧幓瀹冧滑鐨勪娇鐢ㄤ粎鍋滅暀鍦ㄧ悊璁哄眰闈紝瀹為檯闇€姹傚苟鏈鍏呭垎鐞嗚В銆備笉杩囧氨鐩墠鑰岃█锛屽彧鏈夐€嗗悜宸ョ▼鎶€鏈鐢ㄤ簬浠?Kconfig 绛夊彉浣撳缓妯¤瑷€涓帹瀵艰涔?[^3^]_銆?


#### 鐢ㄤ簬 Kconfig 鐨勫畬鏁?SAT 姹傝В鍣?


灏界 SAT 姹傝В鍣?[^4^]_ 灏氭湭琚?Kconfig 鐩存帴浣跨敤锛屼絾濡備笂涓€灏忚妭鎵€杩帮紝宸叉湁宸ヤ綔灏嗘帹瀵煎嚭鐨?Kconfig 璇箟鐢ㄥ竷灏旀娊璞¤〃杈撅紝杞崲涓哄竷灏斿叕寮忓苟鍦ㄦ涓婅繍琛?SAT 姹傝В鍣?[^5^]_銆傚彟涓€涓凡鐭ョ殑鐩稿叧椤圭洰鏄?CADOS [^6^]_锛堝墠韬?VAMOS [^7^]_锛夊強鍏跺伐鍏凤紝涓昏鏄?undertaker [^8^]_锛屽畠鏈€鏃╁湪 [^9^]_ 涓紩鍏ャ€倁ndertaker 鐨勫熀鏈€濇兂鏄粠 Kconfig 鎶藉彇鍙樹綋妯″瀷锛屽苟灏嗗叾涓庝粠 CPP #ifdef 鍜屾瀯寤鸿鍒欎腑鎶藉彇鐨勫懡棰樺叕寮忎竴璧锋斁鍏?SAT 姹傝В鍣紝浠ュ彂鐜版浠ｇ爜銆佹鏂囦欢鍜屾绗﹀彿銆傝嫢甯屾湜鍦?Kconfig 涓婁娇鐢?SAT 姹傝В鍣紝涓€绉嶆柟娉曟槸璇勪及濡備綍灏嗚繖浜涘伐浣滈噸鏂扮敤浜?Kconfig銆傜幇鏈夐」鐩殑瀵煎笀琛ㄧ幇鍑鸿冻澶熷叴瓒ｏ紝涓嶄粎鎰挎剰灏卞浣曞皢璇ュ伐浣滃悎鍏ヤ笂娓告彁渚涘缓璁紝涔熸効鎰忓府鍔╅暱鏈熺淮鎶ゅ畠銆傛劅鍏磋叮鐨勫紑鍙戣€呭簲璁块棶锛?

https://kernelnewbies.org/KernelProjects/kconfig-sat
