
## 蹇€熷紑濮嬶紙Quick Start锛?


鏈枃妗ｆ弿杩板浣曞紑濮嬩娇鐢?Rust 杩涜鍐呮牳寮€鍙戙€?

鏈夊嚑绉嶆柟寮忓彲浠ュ畨瑁呭唴鏍稿紑鍙戞墍闇€鐨?Rust 宸ュ叿閾俱€備竴绉嶇畝鍗曠殑鏂规硶鏄娇鐢ㄤ綘鐨?Linux 鍙戣鐗?
鎻愪緵鐨勮蒋浠跺寘锛堝鏋滈€傜敤锛夆€斺€斾笅闈㈢涓€鑺傝В閲婁簡杩欑鏂瑰紡銆傝繖绉嶆柟寮忕殑涓€涓紭鐐规槸锛屽彂琛岀増閫氬父
浼氫娇 Rust 鎵€浣跨敤鐨?LLVM 涓?Clang 鐩稿尮閰嶃€?

鍙︿竴绉嶆柟寮忔槸浣跨敤 `kernel.org <https://kernel.org/pub/tools/llvm/rust/>`_ 涓婃彁渚涚殑
棰勬瀯寤虹ǔ瀹氱増 LLVM+Rust銆傝繖浜涙槸涓庘€滆幏鍙?LLVM鈥濓紙Getting LLVM <getting_llvm>锛変腑鐩稿悓鐨?
绮剧畝涓斿揩閫熺殑 LLVM 宸ュ叿閾撅紝鍙槸棰濆鍔犲叆浜?Linux 鐨?Rust 鎵€鏀寔鐨?Rust 鐗堟湰銆傛彁渚涗簡涓ょ粍锛?
鈥渓atest LLVM鈥?鍜?鈥渕atching LLVM鈥濓紙鏇村淇℃伅璇峰弬闃呰閾炬帴锛夈€?

姝ゅ锛屾帴涓嬫潵鐨勪袱涓?鈥淩equirements鈥濓紙闇€姹傦級灏忚妭鍒嗗埆瑙ｉ噴浜嗗悇涓粍浠朵互鍙婂浣曢€氳繃 `rustup`銆?
Rust 鐨勭嫭绔嬪畨瑁呯▼搴忥紝鍜?鎴栬嚜琛屾瀯寤烘潵瀹夎瀹冧滑銆?

鏂囨。鐨勫叾浣欓儴鍒嗚В閲婁簡濡備綍涓婃墜鐨勫叾浠栨柟闈€?

### 鍙戣鐗?


######## Arch Linux


Arch Linux 鎻愪緵杈冩柊鐨?Rust 鐗堟湰锛屽洜姝ら€氬父鍙互鐩存帴浣跨敤
```

	pacman -S rust rust-src rust-bindgen


```
######## Debian


Debian 13锛圱rixie锛夛紝浠ュ強 Testing 鍜?Debian Unstable锛圫id锛夋彁渚涜緝鏂扮殑
```
	apt install rustc rust-src bindgen rustfmt rust-clippy


```
######## Fedora Linux


Fedora Linux 鎻愪緵杈冩柊鐨?Rust 鐗堟湰锛屽洜姝ら€氬父鍙互鐩存帴浣跨敤
```
	dnf install rust rust-src bindgen-cli rustfmt clippy


```
######## Gentoo Linux


Gentoo Linux 鎻愪緵杈冩柊鐨?Rust 鐗堟湰锛屽洜姝ら€氬父鍙互鐩存帴浣跨敤
```
	USE='rust-src rustfmt clippy' emerge dev-lang/rust dev-util/bindgen
```
`LIBCLANG_PATH` 鍙兘闇€瑕佽缃€?

######## Nix


Nix 鎻愪緵杈冩柊鐨?Rust 鐗堟湰锛屽洜姝ら€氬父鍙互鐩存帴浣跨敤
```
	{ pkgs ? import <nixpkgs> {} }:
	pkgs.mkShell {
	  nativeBuildInputs = with pkgs; [ rustc rust-bindgen rustfmt clippy ];
	  RUST_LIB_SRC = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
	}


```
######## openSUSE


openSUSE Slowroll 鍜?openSUSE Tumbleweed 鎻愪緵杈冩柊鐨?Rust 鐗堟湰锛屽洜姝?
```
	zypper install rust rust-src rust-bindgen clang


```
######## Ubuntu


Ubuntu 25.10 鍜?26.04 LTS 鎻愪緵杈冩柊鐨?Rust 鐗堟湰锛屽洜姝?
```
	apt install rustc rust-src bindgen rustfmt rust-clippy
```
```
	RUST_LIB_SRC=/usr/src/rustc-$(rustc --version | cut -d' ' -f2)/library
```
涓烘柟渚胯捣瑙侊紝`RUST_LIB_SRC` 鍙互瀵煎嚭鍒板叏灞€鐜涓€?

#### 24.04 LTS 鍙婃洿鏃х増鏈?


铏界劧 Ubuntu 24.04 LTS 鍙婃洿鏃х増鏈粛鎻愪緵杈冩柊鐨?Rust 鐗堟湰锛屼絾瀹冧滑闇€瑕佽缃竴浜涢澶栫殑
閰嶇疆锛屼娇鐢?
```
	apt install rustc-1.85 rust-1.85-src bindgen-0.71 rustfmt-1.85 \
		rust-1.85-clippy
	ln -s /usr/lib/rust-1.85/bin/rustfmt /usr/bin/rustfmt-1.85
	ln -s /usr/lib/rust-1.85/bin/clippy-driver /usr/bin/clippy-driver-1.85
```
杩欎簺杞欢鍖呴兘娌℃湁灏嗗畠浠殑宸ュ叿璁句负榛樿锛屽洜姝ら渶瑕?
```
	make LLVM=1 RUSTC=rustc-1.85 RUSTDOC=rustdoc-1.85 RUSTFMT=rustfmt-1.85 \
		CLIPPY_DRIVER=clippy-driver-1.85 BINDGEN=bindgen-0.71
```
鎴栬€咃紝淇敼 `PATH` 鍙橀噺锛屽皢 Rust 1.85 鐨勪簩杩涘埗鏂囦欢鏀惧湪鍓嶉潰
```
	PATH=/usr/lib/rust-1.85/bin:$PATH
	update-alternatives --install /usr/bin/bindgen bindgen \
		/usr/bin/bindgen-0.71 100
	update-alternatives --set bindgen /usr/bin/bindgen-0.71
```
```
	RUST_LIB_SRC=/usr/src/rustc-$(rustc-1.85 --version | cut -d' ' -f2)/library
```
涓烘柟渚胯捣瑙侊紝`RUST_LIB_SRC` 鍙互瀵煎嚭鍒板叏灞€鐜涓€?

姝ゅ锛宍bindgen-0.71` 鍦ㄨ緝鏂扮殑鐗堟湰锛?4.04 LTS锛変腑鍙敤锛屼絾鍦ㄨ緝鏃х殑鐗堟湰锛?0.04 LTS 鍜?
22.04 LTS锛変腑鍙兘涓嶅彲鐢紝鍥犳 `bindgen` 鍙兘闇€瑕佹墜鍔ㄦ瀯寤猴紙璇峰弬闃呬笅鏂囷級銆?

### 闇€姹傦細鏋勫缓


鏈妭瑙ｉ噴濡備綍鑾峰彇鏋勫缓鎵€闇€鐨勫伐鍏枫€?

瑕佽交鏉炬鏌ユ槸鍚︽弧瓒宠姹傦紝鍙繍琛屼互涓嬬洰鏍?
```
	make LLVM=1 rustavailable
```
杩欎細瑙﹀彂涓?Kconfig 鐩稿悓鐨勯€昏緫锛屼互鍒ゆ柇鏄惁闇€瑕佸惎鐢?`RUST_IS_AVAILABLE`锛涘鏋滀笉鏄紝
瀹冭繕浼氳В閲婂師鍥犮€?

######## rustc


闇€瑕佷娇鐢ㄨ緝鏂扮増鏈殑 Rust 缂栬瘧鍣ㄣ€?

濡傛灉浣跨敤 `rustup`锛岃繘鍏ュ唴鏍告瀯寤虹洰褰曪紙鎴栧 `set` 瀛愬懡浠や娇鐢?`--path=<build-dir>`
鍙傛暟锛夊苟杩愯
```
	rustup override set stable
```
杩欎細灏嗕綘鐨勫伐浣滅洰褰曢厤缃负浣跨敤缁欏畾鐗堟湰鐨?`rustc`锛岃€屼笉浼氬奖鍝嶄綘鐨勯粯璁ゅ伐鍏烽摼銆?

娉ㄦ剰锛岃瑕嗙洊閫傜敤浜庡綋鍓嶅伐浣滅洰褰曪紙鍙婂叾瀛愮洰褰曪級銆?

濡傛灉涓嶄娇鐢?`rustup`锛屽彲浠庝互涓嬪湴鍧€鑾峰彇鐙珛瀹夎绋嬪簭锛?

	https://forge.rust-lang.org/infra/other-installation-methods.html#standalone

######## Rust 鏍囧噯搴撴簮鐮?


闇€瑕?Rust 鏍囧噯搴撴簮鐮侊紝鍥犱负鏋勫缓绯荤粺浼氬 `core` 杩涜浜ゅ弶缂栬瘧銆?

```
	rustup component add rust-src
```
缁勪欢鏄寜宸ュ叿閾惧畨瑁呯殑锛屽洜姝や互鍚庡崌绾?Rust 缂栬瘧鍣ㄧ増鏈渶瑕侀噸鏂版坊鍔犺缁勪欢銆?

鍚﹀垯锛屽鏋滀娇鐢ㄧ嫭绔嬪畨瑁呯▼搴忥紝Rust 婧愮爜鏍戝彲浠?
```
	curl -L "https://static.rust-lang.org/dist/rust-src-$(rustc --version | cut -d' ' -f2).tar.gz" |
		tar -xzf - -C "$(rustc --print sysroot)/lib" \
		"rust-src-$(rustc --version | cut -d' ' -f2)/rust-src/lib/" \
		--strip-components=3
```
鍦ㄨ繖绉嶆儏鍐典笅锛屼互鍚庡崌绾?Rust 缂栬瘧鍣ㄧ増鏈渶瑕佹墜鍔ㄦ洿鏂版簮鐮佹爲锛堟柟娉曟槸鍏堝垹闄?
``$(rustc --print sysroot)/lib/rustlib/src/rust``锛岀劧鍚庨噸鏂拌繍琛屼笂杩板懡浠わ級銆?

######## libclang


`libclang`锛圠LVM 鐨勪竴閮ㄥ垎锛夎 `bindgen` 鐢ㄦ潵鐞嗚В鍐呮牳涓殑 C 浠ｇ爜锛岃繖鎰忓懗鐫€闇€瑕佸畨瑁?
LLVM锛涘氨鍍忎娇鐢?`LLVM=1` 缂栬瘧鍐呮牳鏃朵竴鏍枫€?

Linux 鍙戣鐗堝緢鍙兘鎻愪緵鍚堥€傜殑鐗堟湰锛屽洜姝ゆ渶濂藉厛妫€鏌ャ€?

涔熸湁涓€浜涢€傜敤浜庤嫢骞茬郴缁熷拰鏋舵瀯鐨勪簩杩涘埗鏂囦欢涓婁紶鍦細

	https://releases.llvm.org/download.html

鍚﹀垯锛屾瀯寤?LLVM 鐩稿綋鑰楁椂锛屼絾杩囩▼骞朵笉澶嶆潅锛?

	https://llvm.org/docs/GettingStarted.html#getting-the-source-code-and-building-llvm

鏇村淇℃伅浠ュ強鑾峰彇棰勬瀯寤虹増鏈拰鍙戣鐗堣蒋浠跺寘鐨勫叾浠栨柟寮忥紝璇峰弬闃?
Documentation/kbuild/llvm.rst銆?

######## bindgen


鍒板唴鏍?C 渚х殑缁戝畾鏄湪鏋勫缓鏃朵娇鐢?`bindgen` 宸ュ叿鐢熸垚鐨勩€?

渚嬪閫氳繃浠ヤ笅鏂瑰紡瀹夎锛堟敞鎰忥紝杩欎細涓嬭浇骞舵瀯寤鸿宸ュ叿
```
	cargo install --locked bindgen-cli
```
`bindgen` 浣跨敤 `clang-sys` crate 鏉ユ煡鎵惧悎閫傜殑 `libclang`锛堝彲鑳介潤鎬侀摼鎺ャ€佸姩鎬侀摼鎺ユ垨
鍦ㄨ繍琛屾椂鍔犺浇锛夈€傞粯璁ゆ儏鍐典笅锛屼笂闈㈢殑 `cargo` 鍛戒护浼氱敓鎴愪竴涓湪杩愯鏃跺姞杞?`libclang` 鐨?
`bindgen` 浜岃繘鍒舵枃浠躲€傚鏋滄壘涓嶅埌锛堟垨鑰呭簲璇ヤ娇鐢ㄤ笌鎵惧埌鐨勪笉鍚岀殑 `libclang`锛夛紝鍙互璋冩暣璇?
杩囩▼锛屼緥濡備娇鐢?`LIBCLANG_PATH` 鐜鍙橀噺銆傝鎯呰鍙傞槄 `clang-sys` 鐨勬枃妗ｏ細

	https://github.com/KyleMayes/clang-sys#linking

	https://github.com/KyleMayes/clang-sys#environment-variables

### 闇€姹傦細寮€鍙?


鏈妭瑙ｉ噴濡備綍鑾峰彇寮€鍙戞墍闇€鐨勫伐鍏枫€備篃灏辨槸璇达紝浠呭湪鏋勫缓鍐呮牳鏃跺苟涓嶉渶瑕佽繖浜涘伐鍏枫€?

######## rustfmt


`rustfmt` 宸ュ叿鐢ㄤ簬鑷姩鏍煎紡鍖栨墍鏈?Rust 鍐呮牳浠ｇ爜锛屽寘鎷敓鎴愮殑 C 缁戝畾锛堣鎯呰鍙傞槄
coding-guidelines.rst锛夈€?

濡傛灉浣跨敤 `rustup`锛屽叾 `default` profile 宸茬粡瀹夎浜嗚宸ュ叿锛屽洜姝ゆ棤闇€浠讳綍鎿嶄綔銆傚鏋滀娇鐢?
鍏朵粬 profile锛屽垯
```
	rustup component add rustfmt
```
鐙珛瀹夎绋嬪簭涔熼殢闄?`rustfmt`銆?

######## clippy


`clippy` 鏄竴涓?Rust 鐨?linter銆傝繍琛屽畠鍙互鎻愪緵 Rust 浠ｇ爜鐨勯澶栬鍛娿€傚彲浠ラ€氳繃鍚?`make`
浼犲叆 `CLIPPY=1` 鏉ヨ繍琛岋紙璇︽儏璇峰弬闃?general-information.rst锛夈€?

濡傛灉浣跨敤 `rustup`锛屽叾 `default` profile 宸茬粡瀹夎浜嗚宸ュ叿锛屽洜姝ゆ棤闇€浠讳綍鎿嶄綔銆傚鏋滀娇鐢?
鍏朵粬 profile锛屽垯
```
	rustup component add clippy
```
鐙珛瀹夎绋嬪簭涔熼殢闄?`clippy`銆?

######## rustdoc


`rustdoc` 鏄?Rust 鐨勬枃妗ｅ伐鍏枫€傚畠涓?Rust 浠ｇ爜鐢熸垚缇庤鐨?HTML 鏂囨。锛堣鎯呰鍙傞槄
general-information.rst锛夈€?

`rustdoc` 涔熺敤浜庢祴璇曟湁鏂囨。鐨?Rust 浠ｇ爜涓墍鎻愪緵鐨勭ず渚嬶紙绉颁负 doctests 鎴栨枃妗ｆ祴璇曪級銆?
`rusttest` Make 鐩爣浣跨敤浜嗚繖涓€鐗规€с€?

濡傛灉浣跨敤 `rustup`锛屾墍鏈?profile 閮藉凡缁忓畨瑁呬簡璇ュ伐鍏凤紝鍥犳鏃犻渶浠讳綍鎿嶄綔銆?

鐙珛瀹夎绋嬪簭涔熼殢闄?`rustdoc`銆?

######## rust-analyzer


`rust-analyzer <https://rust-analyzer.github.io/>`_ 璇█鏈嶅姟鍣ㄥ彲浠ヤ笌璁稿缂栬緫鍣ㄤ竴璧蜂娇鐢紝
浠ュ疄鐜拌娉曢珮浜€佽ˉ鍏ㄣ€佽烦杞埌瀹氫箟浠ュ強鍏朵粬鍔熻兘銆?

`rust-analyzer` 闇€瑕佷竴涓厤缃枃浠?`rust-project.json`锛屽畠
```
	make LLVM=1 rust-analyzer


```
### 閰嶇疆


闇€瑕佸湪 `General setup` 鑿滃崟涓惎鐢?`Rust support`锛坄CONFIG_RUST`锛夈€傚彧鏈夊綋鎵惧埌鍚堥€傜殑
Rust 宸ュ叿閾撅紙瑙佷笂鏂囷級涓旀弧瓒冲叾浠栬姹傛椂锛岃閫夐」鎵嶄細鏄剧ず銆傚弽杩囨潵锛岃繖浼氫娇渚濊禆浜?Rust 鐨?
鍏朵綑閫夐」鍙銆?

```
	Kernel hacking
	    -> Sample kernel code
	        -> Rust samples
```
骞跺惎鐢ㄤ竴浜涚ず渚嬫ā鍧楋紝鍙互鏄唴寤烘垨鍙綔涓烘ā鍧楀姞杞姐€?

### 鏋勫缓


浣跨敤瀹屾暣鐨?LLVM 宸ュ叿閾炬瀯寤哄唴鏍告槸鍙楁敮鎸佹渶濂界殑閰嶇疆
```
	make LLVM=1
```
浣跨敤 GCC 瀵规煇浜涢厤缃篃鏈夋晥锛屼絾鐩墠闈炲父瀹為獙鎬с€?

### 娣卞叆鎺㈢┒锛圚acking锛?


瑕佹繁鍏ヤ簡瑙ｏ紝鍙互鏌ョ湅 `samples/rust/` 涓嬬殑绀轰緥浠ｇ爜銆?`rust/` 涓嬬殑 Rust 鏀寔浠ｇ爜锛屼互鍙?
`Kernel hacking` 涓嬬殑 `Rust hacking` 鑿滃崟銆?
