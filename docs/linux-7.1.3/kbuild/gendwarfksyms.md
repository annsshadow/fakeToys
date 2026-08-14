## 鍩轰簬 DWARF 鐨勬ā鍧楃増鏈帶鍒?

## 绠€浠?

褰撳惎鐢?CONFIG_MODVERSIONS 鏃讹紝妯″潡鐨勭鍙风増鏈€氬父浣跨敤 **genksyms** 宸ュ叿浠庨澶勭悊鍚庣殑婧愪唬鐮佽绠椼€傜劧鑰岋紝杩欏浜?Rust 绛夎瑷€涓嶅吋瀹癸紝鍥犱负婧愪唬鐮佺己涔忓叧浜庢渶缁?ABI 鐨勮冻澶熶俊鎭€傚湪閫夋嫨浜?CONFIG_GENDWARFKSYMS锛堜互鍙?CONFIG_DEBUG_INFO锛夋椂锛屾敼涓轰娇鐢?**gendwarfksyms** 浠?DWARF 璋冭瘯淇℃伅璁＄畻绗﹀彿鐗堟湰锛屽叾涓寘鍚簡鍏充簬鏈€缁堟ā鍧?ABI 鐨勫繀瑕佺粏鑺傘€?
### 渚濊禆


gendwarfksyms 渚濊禆浜?libelf銆乴ibdw 涓?zlib 搴撱€?
浠ヤ笅鏄畨瑁呰繖浜涗緷璧栫殑鍑犱釜绀轰緥锛?
```

	sudo pacman --needed -S libelf zlib

```
```

	sudo apt install libelf-dev libdw-dev zlib1g-dev

```
```

	sudo dnf install elfutils-libelf-devel elfutils-devel zlib-devel

```
```

	sudo zypper install libelf-devel libdw-devel zlib-devel

```
### 鐢ㄦ硶


gendwarfksyms 鍦ㄥ懡浠よ鎺ュ彈涓€涓洰鏍囨枃浠跺垪琛紝鐢ㄦ硶濡備笅锛?
```

	Usage: gendwarfksyms [options] elf-object-file ... < symbol-list

	Options:
	  -d, --debug          Print debugging information
	      --dump-dies      Dump DWARF DIE contents
	      --dump-die-map   Print debugging information about die_map changes
	      --dump-types     Dump type strings
	      --dump-versions  Dump expanded type strings used for symbol versions
	  -s, --stable         Support kABI stability features
	  -T, --symtypes file  Write a symtypes file
	  -h, --help           Print this message


```
## 绫诲瀷淇℃伅鐨勫彲鐢ㄦ€?

铏界劧绗﹀彿閫氬父鍦ㄥ畾涔夊畠浠殑鍚屼竴缈昏瘧鍗曞厓锛圱U锛変腑瀵煎嚭锛屼絾 TU 瀵煎嚭澶栭儴绗﹀彿涔熷畬鍏ㄦ病闂銆備緥濡傦紝鍦ㄤ负鐙珛姹囩紪浠ｇ爜涓殑瀵煎嚭璁＄畻绗﹀彿鐗堟湰鏃跺氨鏄繖鏍峰仛鐨勩€?
涓虹‘淇濈紪璇戝櫒鍦ㄧ鍙峰疄闄呭鍑虹殑 TU 涓彂鍑哄繀瑕佺殑 DWARF 绫诲瀷淇℃伅锛実endwarfksyms 浣跨敤浠ヤ笅鏂瑰紡鍦?`EXPORT_SYMBOL()` 瀹忎腑娣诲姞涓€涓寚鍚戣瀵煎嚭绗﹀彿鐨勬寚閽堬細

```

	#define __GENDWARFKSYMS_EXPORT(sym)				\
		static typeof(sym) *__gendwarfksyms_ptr_##sym __used	\
			__section(".discard.gendwarfksyms") = &sym;


```
褰撳湪 DWARF 涓彂鐜扮鍙锋寚閽堟椂锛屽嵆浣跨鍙峰畾涔夊湪鍏朵粬鍦版柟锛実endwarfksyms 涔熻兘浣跨敤鍏剁被鍨嬫潵璁＄畻绗﹀彿鐗堟湰銆傜鍙锋寚閽堢殑鍚嶇О搴斾互 `__gendwarfksyms_ptr_` 寮€澶达紝鍚庤窡琚鍑虹鍙风殑鍚嶇О銆?
## Symtypes 杈撳嚭鏍煎紡


涓?genksyms 绫讳技锛実endwarfksyms 鏀寔涓烘瘡涓澶勭悊鐨勭洰鏍囧啓鍏ヤ竴涓?symtypes 鏂囦欢锛屽叾涓寘鍚鍑虹鍙风殑绫诲瀷浠ュ強璁＄畻绗﹀彿鐗堟湰鏃舵墍鐢ㄥ埌鐨勬瘡涓寮曠敤绫诲瀷銆傝繖浜涙枃浠跺湪璇曞浘纭畾鏋勫缓涔嬮棿绗﹀彿鐗堟湰鍙樺寲鐨勫叿浣撳師鍥犳椂寰堟湁鐢ㄣ€傝鍦ㄥ唴鏍告瀯寤烘湡闂寸敓鎴?symtypes 鏂囦欢锛岃璁剧疆 `KBUILD_SYMTYPES=1`銆?
涓庣幇鏈夋牸寮忎竴鑷达紝姣忚鐨勭涓€鍒楀寘鍚被鍨嬪紩鐢ㄦ垨绗﹀彿鍚嶃€傜被鍨嬪紩鐢ㄦ湁涓€涓崟瀛楁瘝鍓嶇紑锛屽悗璺?"#" 鍜岀被鍨嬪悕銆傚叡鏈夊洓绉嶇被鍨嬶細

```

	e#<type> = enum
	s#<type> = struct
	t#<type> = typedef
	u#<type> = union


```
```

	s#'core::result::Result<u8, core::num::error::ParseIntError>'

```
璇ヨ鍏朵綑閮ㄥ垎鍖呭惈涓€涓被鍨嬪瓧绗︿覆銆備笌鐢熸垚 C 椋庢牸绫诲瀷瀛楃涓茬殑 genksyms 涓嶅悓锛実endwarfksyms 浣跨敤 **--dump-dies** 鐢熸垚鐨勭浉鍚岀畝鍗曡В鏋?DWARF 鏍煎紡锛屼絾浣跨敤绫诲瀷寮曠敤鑰岄潪瀹屽叏灞曞紑鐨勫瓧绗︿覆銆?
## 缁存姢绋冲畾鐨?kABI


鐢变簬 LTS 鏇存柊鎴栧悜鍚庣Щ妞嶏紝鍙戣鐗堢淮鎶よ€呭父甯搁渶瑕佽兘澶熷鍐呮牳鏁版嵁缁撴瀯鍋氬嚭 ABI 鍏煎鐨勪慨鏀广€備娇鐢ㄤ紶缁熺殑 `#ifndef __GENKSYMS__` 鏉ュ悜绗﹀彿鐗堟湰鎺у埗闅愯棌杩欎簺淇敼锛屽湪澶勭悊鐩爣鏂囦欢鏃朵笉璧蜂綔鐢ㄣ€備负鏀寔姝ょ敤渚嬶紝gendwarfksyms 鎻愪緵浜?kABI 绋冲畾鎬х壒鎬э紝鐢ㄤ簬鍦ㄨ绠楃増鏈椂闅愯棌閭ｄ簺涓嶄細褰卞搷 ABI 鐨勪慨鏀广€傝繖浜涚壒鎬ч兘鍙?**--stable** 鍛戒护琛屾爣蹇楁帶鍒讹紝涓斾笉鍦ㄤ富绾垮唴鏍镐腑浣跨敤銆傝鍦ㄥ唴鏍告瀯寤烘湡闂翠娇鐢ㄧǔ瀹氱壒鎬э紝璇疯缃?`KBUILD_GENDWARFKSYMS_STABLE=1`銆?
浣跨敤杩欎簺鐗规€х殑绀轰緥鍦?**scripts/gendwarfksyms/examples** 鐩綍涓彁渚涳紝鍖呮嫭鐢ㄤ簬婧愪唬鐮佹爣娉ㄧ殑杈呭姪瀹忋€傝娉ㄦ剰锛岀敱浜庤繖浜涚壒鎬т粎鐢ㄤ簬杞崲绗﹀彿鐗堟湰鎺у埗鐨勮緭鍏ワ紝鐢ㄦ埛鏈夎矗浠荤‘淇濆叾淇敼瀹為檯涓婁笉浼氱牬鍧?ABI銆?
### kABI 瑙勫垯


kABI 瑙勫垯鍏佽鍙戣鐗堝井璋?gendwarfksyms 杈撳嚭鐨勬煇浜涢儴鍒嗭紝浠庤€屾帶鍒剁鍙风増鏈殑璁＄畻鏂瑰紡銆傝繖浜涜鍒欏畾涔夊湪璇ョ洰鏍囨枃浠剁殑 `.discard.gendwarfksyms.kabi_rules` 鑺備腑锛屽舰寮忎负浠ヤ笅浠?NUL 缁撳熬瀛楁缁勬垚鐨勫瓧绗︿覆搴忓垪锛?
```

	version\0type\0target\0value\0

```
璇ュ瓧绗︿覆搴忓垪鎸夐渶閲嶅澶氭浠ヨ〃杈炬墍鏈夎鍒欍€傚悇瀛楁濡備笅锛?
- `version`锛氱‘淇濆鏈潵缁撴瀯淇敼鐨勫悜鍚庡吋瀹规€с€傚綋鍓嶉鏈熶负 "1"銆?- `type`锛氭寚绀烘墍搴旂敤瑙勫垯鐨勭被鍨嬨€?- `target`锛氭寚瀹氳鍒欑殑鐩爣锛岄€氬父鏄?DWARF 璋冭瘯淇℃伅鏉＄洰锛圖IE锛夌殑瀹屽叏闄愬畾鍚嶃€?- `value`锛氭彁渚涜鍒欑壒瀹氱殑鏁版嵁銆?
渚嬪锛屼互涓嬭緟鍔╁畯鍙敤浜庢寚瀹氳鍒欙細

```

	#define ___KABI_RULE(hint, target, value)                            \
		static const char __PASTE(__gendwarfksyms_rule_,             \
					  __COUNTER__)[] __used __aligned(1) \
			__section(".discard.gendwarfksyms.kabi_rules") =     \
				"1\0" #hint "\0" target "\0" value

	#define __KABI_RULE(hint, target, value) \
		___KABI_RULE(hint, #target, #value)


```
鐩墠浠呮敮鎸佹湰鑺傝璁虹殑瑙勫垯锛屼絾璇ユ牸寮忓叿鏈夎冻澶熺殑鎵╁睍鎬э紝鍙湪闇€瑕佹椂娣诲姞鏇村瑙勫垯銆?
#### 绠＄悊瀹氫箟鍙鎬?

褰撻澶栫殑 include 琚紩鍏ョ炕璇戝崟鍏冩椂锛屽０鏄庡彲鑳藉彉鎴愬畬鏁村畾涔夈€傝繖浼氭敼鍙樹换浣曞紩鐢ㄨ绫诲瀷鐨勭鍙风殑鐗堟湰锛屽嵆浣?ABI 鏈敼鍙樸€傜敱浜庝笉鐮村潖鏋勫缓鍙兘鏃犳硶鍘绘帀 include锛屽洜姝ゅ彲浠ヤ娇鐢?`declonly` 瑙勫垯灏嗕竴涓被鍨嬫寚瀹氫负浠呭０鏄庯紝鍗充娇璋冭瘯淇℃伅鍖呭惈瀹屾暣瀹氫箟銆?
瑙勫垯瀛楁棰勬湡濡備笅锛?
- `type`锛?declonly"
- `target`锛氱洰鏍囨暟鎹粨鏋勭殑瀹屽叏闄愬畾鍚嶏紙濡?**--dump-dies** 杈撳嚭鎵€绀猴級銆?- `value`锛氭瀛楁琚拷鐣ャ€?
```

	#define KABI_DECLONLY(fqn) __KABI_RULE(declonly, fqn, )

```
```

	struct s {
		/* definition */
	};

	KABI_DECLONLY(s);

```
#### 娣诲姞鏋氫妇鍣?

瀵逛簬鏋氫妇锛屾墍鏈夋灇涓惧櫒鍙婂叾鍊奸兘琚撼鍏ョ鍙风増鏈殑璁＄畻锛屽鏋滀箣鍚庨渶瑕佸湪涓嶆敼鍙樼鍙风増鏈殑鎯呭喌涓嬫坊鍔犳洿澶氭灇涓惧櫒锛岃繖灏变細鎴愪负闂銆俙enumerator_ignore` 瑙勫垯鍏佽鎴戜滑浠庤緭鍏ヤ腑闅愯棌鍏峰悕鏋氫妇鍣ㄣ€?
瑙勫垯瀛楁棰勬湡濡備笅锛?
- `type`锛?enumerator_ignore"
- `target`锛氱洰鏍囨灇涓剧殑瀹屽叏闄愬畾鍚嶏紙濡?**--dump-dies** 杈撳嚭鎵€绀猴級涓庢灇涓惧櫒瀛楁鍚嶏紝浠ョ┖鏍煎垎闅斻€?- `value`锛氭瀛楁琚拷鐣ャ€?
```

	#define KABI_ENUMERATOR_IGNORE(fqn, field) \
		__KABI_RULE(enumerator_ignore, fqn field, )

```
```

	enum e {
		A, B, C, D,
	};

	KABI_ENUMERATOR_IGNORE(e, B);
	KABI_ENUMERATOR_IGNORE(e, C);

```
濡傛灉鏋氫妇杩樺寘鍚竴涓粨鏉熸爣璁帮紝涓斿繀椤诲湪涓棿娣诲姞鏂板€硷紝鎴戜滑鍦ㄨ绠楃増鏈椂鍙兘闇€瑕佷负鏈€鍚庝竴涓灇涓惧櫒浣跨敤鏃у€笺€俙enumerator_value` 瑙勫垯鍏佽鎴戜滑涓虹増鏈绠楄鐩栨灇涓惧櫒鐨勫€硷細

- `type`锛?enumerator_value"
- `target`锛氱洰鏍囨灇涓剧殑瀹屽叏闄愬畾鍚嶏紙濡?**--dump-dies** 杈撳嚭鎵€绀猴級涓庢灇涓惧櫒瀛楁鍚嶏紝浠ョ┖鏍煎垎闅斻€?- `value`锛氱敤浜庤瀛楁鐨勬暣鏁板€笺€?
```

	#define KABI_ENUMERATOR_VALUE(fqn, field, value) \
		__KABI_RULE(enumerator_value, fqn field, value)

```
```

	enum e {
		A, B, C, LAST,
	};

	KABI_ENUMERATOR_IGNORE(e, C);
	KABI_ENUMERATOR_VALUE(e, LAST, 2);

```
#### 绠＄悊缁撴瀯浣撳ぇ灏忓彉鍖?

濡傛灉鏁版嵁缁撴瀯鐨勫唴瀛樺垎閰嶇敱鏍稿績鍐呮牳澶勭悊锛岃€屾ā鍧楀彧闇€璁块棶鍏朵腑閮ㄥ垎鎴愬憳锛岄偅涔堣鏁版嵁缁撴瀯瀵规ā鍧楀彲浠ユ槸閮ㄥ垎涓嶉€忔槑鐨勩€傚湪杩欑鎯呭喌涓嬶紝鍙鍘熸湁鎴愬憳鐨勫竷灞€淇濇寔涓嶅彉锛屽氨鍙互鍚戠粨鏋勪綋涓拷鍔犳柊鎴愬憳鑰屼笉鐮村潖 ABI銆?
瑕佽拷鍔犳柊鎴愬憳锛屾垜浠彲浠ユ寜鐓р€滈殣钘忔垚鍛?<hiding_members>鈥濅竴鑺傛墍杩板皢鍏朵粠绗﹀彿鐗堟湰鎺у埗涓殣钘忥紝浣嗘垜浠棤娉曢殣钘忕粨鏋勪綋澶у皬鐨勫鍔犮€俙byte_size` 瑙勫垯鍏佽鎴戜滑瑕嗙洊鐢ㄤ簬绗﹀彿鐗堟湰鎺у埗鐨勭粨鏋勪綋澶у皬銆?
瑙勫垯瀛楁棰勬湡濡備笅锛?
- `type`锛?byte_size"
- `target`锛氱洰鏍囨暟鎹粨鏋勭殑瀹屽叏闄愬畾鍚嶏紙濡?**--dump-dies** 杈撳嚭鎵€绀猴級銆?- `value`锛氭寚绀虹粨鏋勪綋澶у皬锛堝瓧鑺傦級鐨勬鍗佽繘鍒舵暟銆?
```

	#define KABI_BYTE_SIZE(fqn, value) \
		__KABI_RULE(byte_size, fqn, value)

```
```

	struct s {
		/* Unchanged original members */
		unsigned long a;
		void *p;

		/* Appended new members */
		KABI_IGNORE(0, unsigned long n);
	};

	KABI_BYTE_SIZE(s, 16);

```
#### 瑕嗙洊绫诲瀷瀛楃涓?

鍦ㄦ瀬灏戞暟鎯呭喌涓嬶紝鍙戣鐗堝繀椤诲閭ｄ簺鏃犳剰涓鍖呭惈鍦ㄥ凡鍙戝竷 ABI 涓殑銆佹湰搴斾笉閫忔槑鐨勬暟鎹粨鏋勫仛鍑洪噸澶т慨鏀癸紝姝ゆ椂浣跨敤鏇撮拡瀵规€х殑 kABI 瑙勫垯鏉ヤ繚鎸佺鍙风増鏈ǔ瀹氫細鍙樺緱绻佺悙銆俙type_string` 瑙勫垯鍏佽鎴戜滑瑕嗙洊绫诲瀷鎴栫鍙风殑瀹屾暣绫诲瀷瀛楃涓诧紝鐢氳嚦娣诲姞鍐呮牳涓凡涓嶅啀瀛樺湪鐨勩€佺敤浜庣増鏈帶鍒剁殑绫诲瀷銆?
瑙勫垯瀛楁棰勬湡濡備笅锛?
- `type`锛?type_string"
- `target`锛氱洰鏍囨暟鎹粨鏋勭殑瀹屽叏闄愬畾鍚嶏紙濡?**--dump-dies** 杈撳嚭鎵€绀猴級鎴栫鍙枫€?- `value`锛氫竴涓湁鏁堢殑绫诲瀷瀛楃涓诧紙濡?**--symtypes** 杈撳嚭鎵€绀猴級锛岀敤浜庢浛浠ｇ湡瀹炵被鍨嬨€?
```

	#define KABI_TYPE_STRING(type, str) \
		___KABI_RULE("type_string", type, str)

```
```

	/* Override type for a structure */
	KABI_TYPE_STRING("s#s",
		"structure_type s { "
			"member base_type int byte_size(4) "
				"encoding(5) n "
			"data_member_location(0) "
		"} byte_size(8)");

	/* Override type for a symbol */
	KABI_TYPE_STRING("my_symbol", "variable s#s");

```
`type_string` 瑙勫垯搴斾粎鍦ㄥ叾浠栨墜娈垫棤娉曞悎鐞嗙淮鎸佺ǔ瀹氱鍙风増鏈椂鎵嶄綔涓烘渶鍚庢墜娈典娇鐢ㄣ€傝鐩栫被鍨嬪瓧绗︿覆浼氬鍔犲疄闄?ABI 鐮村潖琚拷鐣ョ殑椋庨櫓锛屽洜涓哄畠闅愯棌浜嗗璇ョ被鍨嬬殑鎵€鏈変慨鏀广€?
### 娣诲姞缁撴瀯浣撴垚鍛?

涔熻鏈€甯歌鐨?ABI 鍏煎淇敼鏄悜鍐呮牳鏁版嵁缁撴瀯娣诲姞鎴愬憳銆傚綋棰勬湡缁撴瀯浣撲細琚慨鏀规椂锛屽彂琛岀増缁存姢鑰呭彲浠ラ鍏堝湪缁撴瀯涓繚鐣欑┖闂达紝骞跺湪涔嬪悗浣跨敤瀹冭€屼笉鐮村潖 ABI銆傚鏋滈渶瑕佸娌℃湁淇濈暀绌洪棿鐨勬暟鎹粨鏋勮繘琛屼慨鏀癸紝涔熷彲浠ユ敼鐢ㄥ凡鏈夌殑瀵归綈绌洪殭銆傝櫧鐒跺彲浠ヤ负杩欑被淇敼娣诲姞 kABI 瑙勫垯锛屼絾浣跨敤鑱斿悎浣撻€氬父鏄洿鑷劧鐨勬柟娉曘€傛湰鑺傛弿杩?gendwarfksyms 瀵逛娇鐢ㄦ暟鎹粨鏋勪腑鐨勪繚鐣欑┖闂淬€佷互鍙婇殣钘忓湪璁＄畻绗﹀彿鐗堟湰鏃朵笉浼氭敼鍙?ABI 鐨勬垚鍛樼殑鏀寔銆?
#### 棰勭暀绌洪棿涓庢浛鎹㈡垚鍛?

绌洪棿閫氬父閫氳繃鍦ㄦ暟鎹粨鏋勬湯灏捐拷鍔犳暣鏁扮被鍨嬫垨鏁扮粍鏉ヤ负浠ュ悗浣跨敤鑰岄鐣欙紝浣嗕换浣曠被鍨嬮兘鍙互浣跨敤銆傛瘡涓繚鐣欐垚鍛橀渶瑕佸敮涓€鍚嶇О锛屼絾鐢变簬棰勭暀绌洪棿鏃堕€氬父涓嶇煡閬撳叾瀹為檯鐢ㄩ€旓紝涓烘柟渚胯捣瑙侊紝閫氬父閲囩敤鐨勫懡鍚嶅涓嬶細

```

	struct s {
		long a;
		long __kabi_reserved_0; /* reserved for future use */
	};

```
鍙互閫氳繃灏嗘垚鍛樺寘瑁呭湪涓€涓仈鍚堜綋涓潵浣跨敤棰勭暀绌洪棿锛?
```

	struct s {
		long a;
		union {
			long __kabi_reserved_0; /* original type */
			struct b b; /* replaced field */
		};
	};

```
濡傛灉鍦ㄩ鐣欑┖闂存椂浣跨敤浜?`__kabi_` 鍛藉悕鏂规锛屽垯鑱斿悎浣撶涓€涓垚鍛樼殑鍚嶇О蹇呴』浠?`__kabi_reserved` 寮€澶淬€傝繖纭繚鍦ㄨ绠楃増鏈椂浣跨敤鍘熷绫诲瀷锛屼絾鍚嶇О鍐嶆琚拷鐣ャ€傝仈鍚堜綋鐨勫叾浣欓儴鍒嗚蹇界暐銆?
濡傛灉鎴戜滑瑕佹浛鎹㈢殑鎴愬憳涓嶉伒寰鍛藉悕绾﹀畾锛屾垜浠繕闇€瑕佷繚鐣欏師濮嬪悕绉颁互閬垮厤鏀瑰彉鐗堟湰锛屼负姝ゅ彲灏嗚仈鍚堜綋绗竴涓垚鍛樼殑鍚嶇О鏀逛负浠?`__kabi_renamed` 寮€澶达紝鍚庤窡鍘熷鍚嶇О銆?
绀轰緥涓寘鍚?`KABI_(RESERVE|USE|REPLACE)*` 瀹忥紝鍙府鍔╃畝鍖栨杩囩▼锛屽苟纭繚鏇挎崲鎴愬憳姝ｇ‘瀵归綈涓斿叾澶у皬涓嶄細瓒呰繃棰勭暀绌洪棿銆?

#### 闅愯棌鎴愬憳


棰勬祴鍦ㄦ敮鎸佸懆鏈熷唴鍝簺缁撴瀯闇€瑕佷慨鏀瑰苟闈炴€绘槸鍙锛屽湪杩欑鎯呭喌涓嬪彲鑳戒笉寰椾笉姹傚姪浜庡凡鏈夌殑瀵归綈绌洪殭銆備緥濡傦細

```

	struct s {
		int a;
		/* a 4-byte alignment hole */
		unsigned long b;
	};


```
铏界劧杩欎笉浼氭敼鍙樻暟鎹粨鏋勭殑澶у皬锛屼絾闇€瑕佽兘澶熷皢娣诲姞鐨勬垚鍛樹粠绗﹀彿鐗堟湰鎺у埗涓殣钘忋€備笌淇濈暀瀛楁绫讳技锛岃繖鍙互閫氳繃灏嗘坊鍔犵殑鎴愬憳鍖呰鍒颁竴涓仈鍚堜綋涓疄鐜帮紝鍏朵腑鏌愪釜瀛楁鐨勫悕绉颁互 `__kabi_` 寮€澶达細

```

	struct s {
		int a;
		union {
			char __kabi_ignored_0;
			int n;
		};
		unsigned long b;
	};

```
浣跨敤 **--stable** 鏃讹紝涓や釜鐗堟湰浜х敓鐩稿悓鐨勭鍙风増鏈€傜ず渚嬩腑鍖呭惈 `KABI_IGNORE` 瀹忎互绠€鍖栦唬鐮併€?