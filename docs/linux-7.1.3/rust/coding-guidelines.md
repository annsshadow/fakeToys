
## 缂栫爜瑙勮寖


鏈枃妗ｆ弿杩颁簡濡備綍鍦ㄥ唴鏍镐腑缂栧啓 Rust 浠ｇ爜銆?

### 椋庢牸涓庢牸寮?

浠ｇ爜搴斿綋浣跨敤 `rustfmt` 杩涜鏍煎紡鍖栥€傝繖鏍凤紝鍋跺皵涓哄唴鏍稿仛璐＄尞鐨勪汉灏变笉闇€瑕佸涔犲苟璁颁綇鍙堜竴浠介鏍兼寚鍗椼€傛洿閲嶈鐨勬槸锛屽闃呰€呭拰缁存姢鑰呬笉鍐嶉渶瑕佽姳璐规椂闂存寚鍑洪鏍奸棶棰橈紝鍥犳鍚堝苟涓€涓敼鍔ㄥ彲鑳介渶瑕佺殑琛ヤ竵寰€杩旀鏁颁篃浼氭洿灏戙€?
  `rustfmt`銆傚洜姝よ繖浜涗粛鐒堕渶瑕佽鐣欐剰銆?
浣跨敤 `rustfmt` 鐨勯粯璁よ缃€傝繖鎰忓懗鐫€閬靛惊鎯敤鐨?Rust 椋庢牸銆備緥濡傦紝浣跨敤 4 涓┖鏍艰€屼笉鏄埗琛ㄧ杩涜缂╄繘銆?
鏂逛究鐨勫仛娉曟槸璁╃紪杈戝櫒/IDE 鍦ㄨ緭鍏ユ椂銆佷繚瀛樻椂鎴栨彁浜ゆ椂鑷姩鏍煎紡鍖栥€備笉杩囷紝濡傛灉鐢变簬鏌愮鍘熷洜鍦ㄦ煇涓椂鍒婚渶瑕侀噸鏂版牸寮忓寲鏁翠釜鍐呮牳鐨?Rust 婧愮爜锛屽彲浠ヤ娇鐢ㄤ互涓嬪懡浠わ細

```
	make LLVM=1 rustfmt
```
涔熷彲浠ユ鏌ユ槸鍚︽墍鏈夊唴瀹归兘宸叉牸寮忓寲锛堟墦鍗板嚭宸紓锛夛細

```
	make LLVM=1 rustfmtcheck
```
涓庡唴鏍稿叾浣欓儴鍒嗕娇鐢?`clang-format` 绫讳技锛宍rustfmt` 浣滅敤浜庡崟涓枃浠讹紝骞朵笖涓嶉渶瑕佸唴鏍搁厤缃€傛湁鏃跺畠鐢氳嚦鍙互鍦ㄤ唬鐮佹湁璇硶閿欒鏃跺伐浣溿€?
#### 瀵煎叆


榛樿鎯呭喌涓嬶紝`rustfmt` 浼氫互鍦ㄥ悎骞跺拰鍙樺熀鏃跺鏄撳紩鍙戝啿绐佺殑鏂瑰紡鏍煎紡鍖栧鍏ワ紝鍥犱负鍦ㄦ煇浜涙儏鍐典笅瀹冧細鎶婂涓」鍘嬬缉鍒板悓涓€琛屻€備緥濡傦細

	// Do not use this style.
	use crate::{
	    example1,
	    example2::{example3, example4, example5},
	    example6, example7,
	    example8::example9,
	};

鐩稿弽锛屽唴鏍镐娇鐢ㄥ涓嬫墍绀虹殑绾靛悜甯冨眬锛?
	use crate::{
	    example1,
	    example2::{
	        example3,
	        example4,
	        example5, //
	    },
	    example6,
	    example7,
	    example8::example9, //
	};

涔熷氨鏄锛屾瘡涓」鐙崰涓€琛岋紝骞朵笖鍙鍒楄〃涓笉姝竴涓」锛屽氨浣跨敤鑺辨嫭鍙枫€?
鏈熬鐨勭┖娉ㄩ噴鐢ㄤ簬淇濈暀杩欑鏍煎紡銆備笉浠呭姝わ紝`rustfmt` 鍦ㄦ坊鍔犱簡璇ョ┖娉ㄩ噴鍚庡疄闄呬笂浼氬皢瀵煎叆绾靛悜閲嶆柊鏍煎紡鍖栥€備篃灏辨槸璇达紝鍙互閫氳繃瀵瑰涓嬭緭鍏ヨ繍琛?`rustfmt`锛岃交鏉惧湴灏嗗師濮嬬ず渚嬮噸鏂版牸寮忓寲涓烘湡鏈涚殑椋庢牸锛?
	// Do not use this style.
	use crate::{
	    example1,
	    example2::{example3, example4, example5, //
	    },
	    example6, example7,
	    example8::example9, //
	};

鏈熬鐨勭┖娉ㄩ噴瀵瑰祵濂楀鍏ワ紙濡備笂鎵€绀猴級浠ュ強鍗曢」瀵煎叆閮芥湁鏁堚€斺€旇繖瀵逛簬鏈€灏忓寲琛ヤ竵绯诲垪鍐呴儴鐨勫樊寮傚緢鏈夌敤锛?
	use crate::{
	    example1, //
	};

鏈熬鐨勭┖娉ㄩ噴鍦ㄨ姳鎷彿鍐呯殑浠绘剰琛岄兘鏈夋晥锛屼絾鏈€濂藉皢鍏朵繚鐣欏湪鏈€鍚庝竴椤逛腑锛屽洜涓鸿繖璁╀汉鑱旀兂鍒板叾浠栨牸寮忓寲宸ュ叿涓殑鏈熬閫楀彿銆傛湁鏃讹紝鐢变簬鍒楄〃涓唴瀹圭殑鍙樺姩锛屽湪琛ヤ竵绯诲垪涓伩鍏嶅娆＄Щ鍔ㄨ娉ㄩ噴鍙兘鏇寸畝鍗曘€?
鍙兘浼氭湁涓€浜涢渶瑕佷緥澶栫殑鎯呭喌锛屽嵆杩欎簺閮戒笉鏄‖鎬ц鍒欍€備篃鏈変竴浜涗唬鐮佸皻鏈縼绉诲埌杩欑椋庢牸锛屼絾璇蜂笉瑕佸紩鍏ュ叾浠栭鏍肩殑浠ｇ爜銆?
鏈€缁堢殑鐩爣鏄 `rustfmt` 鍦ㄧǔ瀹氱増鏈腑鑷姩鏀寔杩欑鏍煎紡锛堟垨绫讳技鐨勬牸寮忥級锛岃€屾棤闇€鏈熬鐨勭┖娉ㄩ噴銆傚洜姝わ紝鍦ㄦ煇涓椂鍊欙紝鐩爣鏄Щ闄よ繖浜涙敞閲娿€?

### 娉ㄩ噴


鈥滄櫘閫氣€濇敞閲婏紙鍗?`//`锛岃€屼笉鏄互 `///` 鎴?`//!` 寮€澶寸殑浠ｇ爜鏂囨。锛変娇鐢?Markdown 缂栧啓锛屾柟寮忎笌鏂囨。娉ㄩ噴鐩稿悓锛屽嵆浣垮畠浠笉浼氳娓叉煋銆傝繖鎻愰珮浜嗕竴鑷存€с€佺畝鍖栦簡瑙勫垯锛屽苟鍏佽鏇村鏄撳湴鍦ㄤ袱绉嶆敞閲婁箣闂寸Щ鍔ㄥ唴瀹广€備緥濡傦細

	// `object` is ready to be handled now.
	f(object);

姝ゅ锛屽氨鍍忔枃妗ｄ竴鏍凤紝娉ㄩ噴鍦ㄥ彞瀛愬紑澶撮瀛楁瘝澶у啓锛屽苟浠ュ彞鍙风粨灏撅紙鍗充娇鍙湁涓€涓彞瀛愶級銆傝繖鍖呮嫭 `// SAFETY:`銆乣// TODO:` 浠ュ強鍏朵粬鈥滃甫鏍囩鈥濈殑娉ㄩ噴锛屼緥濡傦細

	// FIXME: The error should be handled properly.

娉ㄩ噴涓嶅簲褰撶敤浜庢枃妗ｇ洰鐨勶細娉ㄩ噴鐢ㄤ簬瀹炵幇缁嗚妭锛岃€屼笉鏄粰鐢ㄦ埛鐪嬬殑銆傝繖绉嶅尯鍒嗗嵆浣挎簮鏂囦欢鐨勮鑰呭悓鏃舵槸鏌愪釜 API 鐨勫疄鐜拌€呭拰鐢ㄦ埛鏃朵篃鏄湁鐢ㄧ殑銆備簨瀹炰笂锛屾湁鏃跺悓鏃朵娇鐢ㄦ敞閲婂拰鏂囨。浼氬緢鏈夌敤銆備緥濡傦紝瀵逛簬涓€涓?`TODO` 鍒楄〃锛屾垨鑰呭鏂囨。鏈韩杩涜娉ㄩ噴銆傚浜庡悗涓€绉嶆儏鍐碉紝娉ㄩ噴鍙互鎻掑叆鍦ㄤ腑闂达紱涔熷氨鏄锛屾洿闈犺繎瑕佽娉ㄩ噴鐨勯偅琛屾枃妗ｃ€傚浜庝换浣曞叾浠栨儏鍐碉紝娉ㄩ噴鍐欏湪鏂囨。涔嬪悗锛屼緥濡傦細

	/// Returns a new [`Foo`].
	///
	/// # Examples
	///
	// TODO: Find a better example.
	/// ```
	/// let foo = f(42);
	/// ```
	// FIXME: Use fallible approach.
	pub fn f(x: i32) -> Foo {
	    // ...
	}

杩欓€傜敤浜庡叕鍏卞拰绉佹湁椤广€傝繖鎻愰珮浜嗕笌鍏叡椤圭殑涓€鑷存€э紝浣垮緱鍙鎬х殑鍙樻洿娑夊強鏇村皯鐨勬敼鍔紝骞朵笖灏嗗厑璁告垜浠?potentially 涔熶负绉佹湁椤圭敓鎴愭枃妗ｃ€傛崲鍙ヨ瘽璇达紝濡傛灉涓虹鏈夐」缂栧啓浜嗘枃妗ｏ紝閭ｄ箞浠嶇劧搴斿綋浣跨敤 `///`銆備緥濡傦細

	/// My private function.
	// TODO: ...
	fn f() {}

涓€绉嶇壒娈婄殑娉ㄩ噴鏄?`// SAFETY:` 娉ㄩ噴銆傚畠浠繀椤诲嚭鐜板湪姣忎釜 `unsafe` 鍧椾箣鍓嶏紝骞惰В閲婁负浠€涔堣鍧楀唴鐨勪唬鐮佹槸姝ｇ‘鐨?瀹夊叏鐨勶紝鍗充负浠€涔堝畠鍦ㄤ换浣曟儏鍐典笅閮戒笉浼氳Е鍙戞湭瀹氫箟琛屼负锛屼緥濡傦細

	// SAFETY: `p` is valid by the safety requirements.
	unsafe { *p = 0; }

`// SAFETY:` 娉ㄩ噴涓嶅簲涓庝唬鐮佹枃妗ｄ腑鐨?`# Safety` 灏忚妭娣锋穯銆俙# Safety` 灏忚妭瑙勫畾浜嗚皟鐢ㄨ€咃紙瀵逛簬鍑芥暟锛夋垨瀹炵幇鑰咃紙瀵逛簬 trait锛夐渶瑕侀伒瀹堢殑濂戠害銆俙// SAFETY:` 娉ㄩ噴鍒欒鏄庝负浠€涔堟煇娆¤皟鐢紙瀵逛簬鍑芥暟锛夋垨瀹炵幇锛堝浜?trait锛夌‘瀹為伒瀹堜簡 `# Safety` 灏忚妭鎴栬瑷€鍙傝€冧腑鎵€闄堣堪鐨勫墠缃潯浠躲€?

### 浠ｇ爜鏂囨。


Rust 鍐呮牳浠ｇ爜鐨勬枃妗ｆ柟寮忎笉鍚屼簬 C 鍐呮牳浠ｇ爜锛堝嵆閫氳繃 kernel-doc锛夈€傜浉鍙嶏紝浣跨敤涓?Rust 浠ｇ爜缂栧啓鏂囨。鐨勫父瑙勭郴缁燂細`rustdoc` 宸ュ叿锛屽畠浣跨敤 Markdown锛堜竴绉嶈交閲忕骇鏍囪璇█锛夈€?
瑕佸涔?Markdown锛屽闈㈡湁寰堝鍙敤鐨勬寚鍗椼€備緥濡傦紝浣嶄簬锛?
	https://commonmark.org/help/

涓€涓枃妗ｅ畬澶囩殑 Rust 鍑芥暟鍙兘闀胯繖鏍凤細

	/// Returns the contained [`Some`] value, consuming the `self` value,
	/// without checking that the value is not [`None`].
	///
	/// # Safety
	///
	/// Calling this method on [`None`] is **[undefined behavior]**.
	///
	/// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
	///
	/// # Examples
	///
	/// ```
	/// let x = Some("air");
	/// assert_eq!(unsafe { x.unwrap_unchecked() }, "air");
	/// ```
	pub unsafe fn unwrap_unchecked(self) -> T {
	    match self {
	        Some(val) => val,

	        // SAFETY: The safety contract must be upheld by the caller.
	        None => unsafe { hint::unreachable_unchecked() },
	    }
	}

杩欎釜渚嬪瓙灞曠ず浜嗕竴浜?`rustdoc` 鐗规€т互鍙婂唴鏍镐腑閬靛惊鐨勪竴浜涚害瀹氾細

- 绗竴娈靛繀椤绘槸绠€瑕佹弿杩版墍鏂囨。鍖栭」鍔熻兘鐨勫崟涓彞瀛愩€傝繘涓€姝ョ殑瑙ｉ噴蹇呴』鏀惧湪棰濆鐨勬钀戒腑銆?
- 涓嶅畨鍏ㄧ殑鍑芥暟蹇呴』鍦?`# Safety` 灏忚妭涓褰曞叾瀹夊叏鎬у墠缃潯浠躲€?
- 铏界劧杩欓噷娌℃湁灞曠ず锛屼絾濡傛灉涓€涓嚱鏁板彲鑳戒細 panic锛屽垯蹇呴』婊¤冻璇ユ潯浠剁殑鎯呭喌蹇呴』鍦ㄤ竴涓?`# Panics` 灏忚妭涓弿杩般€?
  璇锋敞鎰忥紝panic 搴斿綋闈炲父缃曡锛屽苟涓斿彧鏈夊湪鏈夊厖鍒嗙悊鐢辨椂鎵嶄娇鐢ㄣ€傚湪鍑犱箮鎵€鏈夋儏鍐典笅锛岄兘搴斿綋浣跨敤鍙け璐ョ殑鏂瑰紡锛岄€氬父杩斿洖涓€涓?`Result`銆?
- 濡傛灉鎻愪緵浣跨敤绀轰緥鏈夊姪浜庤鑰咃紝鍒欏繀椤诲啓鍦ㄤ竴涓悕涓?`# Examples` 鐨勫皬鑺備腑銆?
- Rust 椤癸紙鍑芥暟銆佺被鍨嬨€佸父閲忊€︹€︼級蹇呴』閫傚綋鍦伴摼鎺ワ紙`rustdoc` 浼氳嚜鍔ㄥ垱寤洪摼鎺ワ級銆?
- 浠讳綍 `unsafe` 鍧椾箣鍓嶉兘蹇呴』鏈変竴涓?`// SAFETY:` 娉ㄩ噴锛屾弿杩颁负浠€涔堝叾涓殑浠ｇ爜鏄畨鍏ㄧ殑銆?
  铏界劧鏈夋椂鐞嗙敱鐪嬭捣鏉ュ井涓嶈冻閬擄紝鍥犳浼间箮涓嶉渶瑕侊紝浣嗙紪鍐欒繖浜涙敞閲婁笉浠呬粎鏄竴绉嶈褰曞凡鑰冭檻鍥犵礌鐨勫ソ鏂规硶锛屾洿閲嶈鐨勬槸锛屽畠鎻愪緵浜嗕竴绉嶉€斿緞鏉ヨ〃鏄庝笉瀛樺湪**棰濆**鐨勯殣寮忕害鏉熴€?
瑕佷簡瑙ｆ洿澶氬叧浜庡浣曚负 Rust 缂栧啓鏂囨。浠ュ強棰濆鐗规€х殑鍐呭锛岃鍙傞槄 `rustdoc` 涔︾睄锛?
	https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html

姝ゅ锛屽唴鏍告敮鎸侀€氳繃鍦ㄩ摼鎺ョ洰鏍囧墠鍔犱笂 `srctree/` 鏉ュ垱寤虹浉瀵逛簬婧愮爜鏍戠殑閾炬帴銆備緥濡傦細

	//! C header: [`include/linux/printk.h`](srctree/include/linux/printk.h)

鎴栬€咃細

	/// [`struct mutex`]: srctree/include/linux/mutex.h


### C FFI 绫诲瀷


Rust 鍐呮牳浠ｇ爜浣跨敤绫诲瀷鍒悕锛堝 `c_int`锛夋潵寮曠敤 C 绫诲瀷锛堝 `int`锛夛紝杩欎簺鍒悕鍙粠 `kernel` prelude 涓洿鎺ヨ幏寰椼€傝**涓嶈浣跨敤鏉ヨ嚜 ``core``
: ffi`` 鐨勫埆鍚嶁€斺€斿畠浠彲鑳戒笉鑳芥槧灏勫埌姝ｇ‘鐨勭被鍨嬨€?
杩欎簺鍒悕閫氬父搴斿綋鐩存帴閫氳繃鍏舵爣璇嗙寮曠敤锛屽嵆浣滀负涓€涓崟娈佃矾寰勩€備緥濡傦細

	fn f(p: *const c_char) -> c_int {
	    // ...
	}


### 鍛藉悕


Rust 鍐呮牳浠ｇ爜閬靛惊甯歌鐨?Rust 鍛藉悕绾﹀畾锛?
	https://rust-lang.github.io/api-guidelines/naming.html

褰撳皢鐜版湁鐨?C 姒傚康锛堜緥濡傚畯銆佸嚱鏁般€佸璞♀€︹€︼級鍖呰鍒?Rust 鎶借薄涓椂锛屽簲褰撲娇鐢ㄥ敖鍙兘鎺ヨ繎 C 渚х殑鍚嶇О锛屼互閬垮厤娣锋穯锛屽苟鎻愰珮鍦?C 鍜?Rust 涓や晶涔嬮棿鏉ュ洖鍒囨崲鏃剁殑鍙鎬с€備緥濡傦紝鏉ヨ嚜 C 鐨?`pr_info` 绛夊畯鍦?Rust 渚т篃浣跨敤鐩稿悓鐨勫悕绉般€?
璇濊櫧濡傛锛屽ぇ灏忓啓搴斿綋璋冩暣涓洪伒寰?Rust 鐨勫懡鍚嶇害瀹氾紝骞朵笖鐢辨ā鍧楀拰绫诲瀷寮曞叆鐨勫懡鍚嶇┖闂翠笉搴斿湪椤瑰悕涓噸澶嶃€備緥濡傦紝褰撳寘瑁呭涓嬪父閲忔椂锛?
	#define GPIO_LINE_DIRECTION_IN	0
	#define GPIO_LINE_DIRECTION_OUT	1

Rust 涓殑绛変环褰㈠紡鍙兘濡備笅锛堝拷鐣ユ枃妗ｏ級锛?
	pub mod gpio {
	    pub enum LineDirection {
	        In = bindings::GPIO_LINE_DIRECTION_IN as _,
	        Out = bindings::GPIO_LINE_DIRECTION_OUT as _,
	    }
	}

涔熷氨鏄锛宍GPIO_LINE_DIRECTION_IN` 鐨勭瓑浠峰紩鐢ㄥ簲褰撴槸
**``gpio``
: LineDirection::In``銆傜壒鍒湴锛屽畠涓嶅簲琚懡鍚嶄负
**``gpio``
: gpio_line_direction::GPIO_LINE_DIRECTION_IN``銆?

### Lint 妫€鏌?

鍦?Rust 涓紝鍙互鍦ㄥ眬閮?`allow` 鐗瑰畾鐨勮鍛婏紙璇婃柇淇℃伅銆乴int锛夛紝浣跨紪璇戝櫒蹇界暐缁欏畾鍑芥暟銆佹ā鍧椼€佸潡绛夎寖鍥村唴鏌愪釜璀﹀憡鐨勫疄渚嬨€?
瀹冪被浼间簬 C 涓殑 `#pragma GCC diagnostic push` + `ignored` + `pop` [#]_锛?
	#pragma GCC diagnostic push
	#pragma GCC diagnostic ignored "-Wunused-function"
	static void f(void) {}
	#pragma GCC diagnostic pop

       attributes锛圕23 鐨?`[[maybe_unused]]`锛夊彲鑳戒細琚娇鐢紱涓嶈繃锛岃渚嬪瓙鏃ㄥ湪鍙嶆槧涔嬪悗璁ㄨ鐨?Rust 涓浉搴旂殑 lint銆?
浣嗙畝娲佸緱澶氾細

	#[allow(dead_code)]
	fn f() {}

鍑€熻繖涓€鐗规€э紝鍙互鑸掗€傚湴榛樿鍚敤鏇村璇婃柇锛堝嵆 `W=` 绾у埆涔嬪锛夈€傜壒鍒槸閭ｄ簺鍙兘鏈変竴浜涜鎶ワ紝浣嗛櫎姝や箣澶栦繚鎸佸惎鐢ㄤ互鎹曡幏娼滃湪閿欒鐩稿綋鏈夌敤鐨勮瘖鏂€?
闄ゆ涔嬪锛孯ust 鎻愪緵浜?`expect` 灞炴€э紝灏嗗叾鏇磋繘涓€姝ャ€傚畠浣垮緱濡傛灉璀﹀憡鏈浜х敓锛岀紪璇戝櫒浼氬彂鍑鸿鍛娿€備緥濡傦紝浠ヤ笅鍐呭灏嗙‘淇濆綋 `f()` 鍦ㄦ煇澶勮璋冪敤鏃讹紝鎴戜滑灏嗕笉寰椾笉绉婚櫎璇ュ睘鎬э細

	#[expect(dead_code)]
	fn f() {}

```
	warning: this lint expectation is unfulfilled
	 --> x.rs:3:10
	  |
	3 | #[expect(dead_code)]
	  |          ^^^^^^^^^
	  |
	  = note: `#[warn(unfulfilled_lint_expectations)]` on by default
```

杩欐剰鍛崇潃 `expect`\ s 鍦ㄤ笉鍐嶉渶瑕佹椂涓嶄細琚仐蹇橈紝杩欏彲鑳藉彂鐢熷湪澶氱鎯呭喌涓嬶紝渚嬪锛?
- 鍦ㄥ紑鍙戣繃绋嬩腑娣诲姞鐨勪复鏃跺睘鎬с€?
- 缂栬瘧鍣ㄣ€丆lippy 鎴栬嚜瀹氫箟宸ュ叿涓?lint 鐨勬敼杩涳紝鍙兘浼氱Щ闄や竴涓鎶ャ€?
- 褰撹 lint 涓嶅啀闇€瑕侊紝鍥犱负棰勬湡瀹冧細鍦ㄦ煇涓椂鍒昏绉婚櫎锛屼緥濡備笂闈㈢殑 `dead_code` 渚嬪瓙銆?
瀹冭繕鎻愰珮浜嗗墿浣?`allow`\ s 鐨勫彲瑙佹€э紝骞堕檷浣庝簡璇敤涓€涓殑鍙兘鎬с€?
鍥犳锛岄櫎闈炰笅鍒楁儏鍐碉紝鍚﹀垯浼樺厛浣跨敤 `expect` 鑰岄潪 `allow`锛?
- 鏉′欢缂栬瘧鍦ㄦ煇浜涙儏鍐佃€岄潪鍏朵粬鎯呭喌涓嬭Е鍙戣鍛娿€?
  濡傛灉瑙﹀彂璀﹀憡锛堟垨涓嶈Е鍙戣鍛婏級鐨勬儏鍐电浉瀵逛簬鎬绘暟鍙湁灏戞暟锛岄偅涔堝彲浠ヨ€冭檻浣跨敤鏉′欢 `expect`锛堝嵆 `cfg_attr(..., expect(...))`锛夈€傚惁鍒欙紝鐩存帴浣跨敤 `allow` 鍙兘鏇寸畝鍗曘€?
- 鍦ㄥ畯鍐呴儴锛屽綋涓嶅悓鐨勮皟鐢ㄥ彲鑳界敓鎴愬湪鏌愪簺鎯呭喌鑰岄潪鍏朵粬鎯呭喌涓嬭Е鍙戣鍛婄殑鎵╁睍浠ｇ爜鏃躲€?
- 褰撲唬鐮佸彲鑳藉洜鏌愪簺鏋舵瀯鑰岄潪鍏朵粬鏋舵瀯瑙﹀彂璀﹀憡鏃讹紝渚嬪鍚?C FFI 绫诲瀷杩涜鐨?`as` 杞崲銆?
浣滀负涓€涓洿瀹屾暣鐨勪緥瀛愶紝鑰冭檻杩欎釜绋嬪簭锛?
	fn g() {}

	fn main() {
	    #[cfg(CONFIG_X)]
	    g();
	}

杩欓噷锛屽鏋滄湭璁剧疆 `CONFIG_X`锛屽嚱鏁?`g()` 灏辨槸姝讳唬鐮併€傛垜浠彲浠ュ湪杩欓噷浣跨敤 `expect` 鍚楋紵

	#[expect(dead_code)]
	fn g() {}

	fn main() {
	    #[cfg(CONFIG_X)]
	    g();
	}

濡傛灉鍦ㄨ缃簡 `CONFIG_X` 鐨勬儏鍐典笅锛岃繖浼氬彂鍑轰竴涓?lint锛屽洜涓哄湪璇ラ厤缃腑瀹冧笉鏄浠ｇ爜銆傚洜姝わ紝鍦ㄨ繖鏍风殑鎯呭喌涓紝鎴戜滑涓嶈兘鍘熸牱浣跨敤 `expect`銆?
涓€涓畝鍗曠殑鍔炴硶鏄娇鐢?`allow`锛?
	#[allow(dead_code)]
	fn g() {}

	fn main() {
	    #[cfg(CONFIG_X)]
	    g();
	}

鍙︿竴绉嶉€夋嫨鏄娇鐢ㄦ潯浠?`expect`锛?
	#[cfg_attr(not(CONFIG_X), expect(dead_code))]
	fn g() {}

	fn main() {
	    #[cfg(CONFIG_X)]
	    g();
	}

杩欏皢纭繚濡傛灉鏈変汉鍦ㄦ煇澶勫紩鍏ヤ簡瀵?`g()` 鐨勫彟涓€涓皟鐢紙渚嬪鏃犳潯浠跺湴锛夛紝閭ｄ箞灏辫兘琚彂鐜板畠涓嶅啀鏄浠ｇ爜銆備笉杩囷紝`cfg_attr` 姣旂畝鍗曠殑 `allow` 鏇村鏉傘€?
鍥犳锛屽綋娑夊強瓒呰繃涓€涓や釜閰嶇疆锛屾垨鑰呰 lint 鍙兘鍥犻潪灞€閮ㄥ彉鏇达紙濡?`dead_code`锛夎€岃瑙﹀彂鏃讹紝浣跨敤鏉′欢 `expect`\ s 鍙兘骞朵笉鍊煎緱銆?
鏈夊叧 Rust 涓瘖鏂俊鎭殑鏇村淇℃伅锛岃鍙傞槄锛?
	https://doc.rust-lang.org/stable/reference/attributes/diagnostics.html

### 閿欒澶勭悊


鏈夊叧 Linux 涓撶敤 Rust 閿欒澶勭悊鐨勪竴浜涜儗鏅拰鎸囧崡锛岃鍙傞槄锛?
	https://rust.docs.kernel.org/kernel/error/type.Result.html#error-codes-in-c-and-rust
