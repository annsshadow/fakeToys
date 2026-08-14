## 閫氱敤鏃堕挓妗嗘灦锛圕ommon Clk Framework锛?
:Author: Mike Turquette <mturquette@ti.com>

鏈枃妗ｅ姏姹傝В閲婇€氱敤鏃堕挓锛坈ommon clk锛夋鏋剁殑缁嗚妭锛屼互鍙婂浣曞皢涓€涓钩鍙扮Щ妞嶅埌璇ユ鏋朵笂銆?瀹冪洰鍓嶈繕涓嶆槸瀵?include/linux/clk.h 涓椂閽?API 鐨勮缁嗚В閲婏紝浣嗕篃璁稿皢鏉ヤ細鍖呭惈杩欎簺淇℃伅銆?
## 绠€浠嬩笌鎺ュ彛鍒掑垎

閫氱敤鏃堕挓妗嗘灦鏄竴涓敤浜庢帶鍒跺綋浠婂悇绉嶈澶囦笂鍙敤鏃堕挓鑺傜偣鐨勬帴鍙ｃ€傚畠鍙互琛ㄧ幇涓烘椂閽熼棬鎺с€?閫熺巼璋冭妭銆佸璺鐢ㄦ垨鍏朵粬鎿嶄綔銆傝妗嗘灦閫氳繃 CONFIG_COMMON_CLK 閫夐」鍚敤銆?
鎺ュ彛鏈韩琚垎涓轰袱鍗婏紝鍚勮嚜灞忚斀浜嗗彟涓€鍗婄殑瀹炵幇缁嗚妭銆傞鍏堟槸 struct clk 鐨勫叕鍏卞畾涔夛紝瀹冪粺涓€
浜嗘鏋跺眰闈㈢殑璁拌处锛坅ccounting锛変笌鍩虹璁炬柦鈥斺€旇繖浜涘湪浼犵粺涓婅鍚勭涓嶅悓鐨勫钩鍙伴噸澶嶅疄鐜般€傚叾娆?鏄?clk.h API 鐨勫叕鍏卞疄鐜帮紝瀹氫箟浜?drivers/clk/clk.c銆傛渶鍚庢槸 struct clk_ops锛屽叾鎿嶄綔鐢?鏃堕挓 API 鐨勫疄鐜版潵璋冪敤銆?
鎺ュ彛鐨勫悗鍗婇儴鍒嗙敱娉ㄥ唽鍒?struct clk_ops 鐨勭‖浠剁浉鍏冲洖璋冨嚱鏁帮紝浠ュ強涓哄缓妯＄壒瀹氭椂閽熸墍闇€鐨勭浉搴?纭欢鐩稿叧缁撴瀯缁勬垚銆傚湪鏈枃妗ｇ殑浣欎笅閮ㄥ垎涓紝浠讳綍瀵?struct clk_ops 涓洖璋冿紙渚嬪 .enable 鎴?.set_rate锛夌殑寮曠敤锛岄兘鎸囪浠ｇ爜鐨勭‖浠剁浉鍏冲疄鐜般€傜被浼煎湴锛屽 struct clk_foo 鐨勫紩鐢ㄥ彧鏄
鍋囨兂鐨勨€渇oo鈥濈‖浠剁殑纭欢鐩稿叧閮ㄥ垎瀹炵幇鐨勪竴绉嶇畝渚跨畝鍐欍€?
灏嗘帴鍙ｇ殑涓ゅ崐鑱旂郴鍦ㄤ竴璧风殑鏄?struct clk_hw锛屽畠瀹氫箟鍦?struct clk_foo 涓紝骞惰 struct
clk_core 涓殑鎸囬拡鎵€鎸囧悜銆傝繖鏍峰彲浠ユ柟渚垮湴鍦ㄩ€氱敤鏃堕挓鎺ュ彛涓や釜鐙珛鐨勪竴鍗婁箣闂磋繘琛屽鑸€?
## 鍏叡鏁版嵁缁撴瀯涓?API

涓嬮潰鏄潵鑷?include/linux/clk-provider.h 鐨勯€氱敤 struct clk_core 瀹氫箟锛?
```
	struct clk_core {
		const char		*name;
		const struct clk_ops	*ops;
		struct clk_hw		*hw;
		struct module		*owner;
		struct clk_core		*parent;
		const char		**parent_names;
		struct clk_core		**parents;
		u8			num_parents;
		u8			new_parent_index;
		...
	};

```
涓婅堪鎴愬憳鏋勬垚浜嗘椂閽熸爲鎷撴墤鐨勬牳蹇冦€傛椂閽?API 鏈韩瀹氫箟浜嗗涓潰鍚戦┍鍔ㄧ殑鍑芥暟锛岃繖浜涘嚱鏁版搷浣?struct clk銆傝 API 鍦?include/linux/clk.h 涓湁鏂囨。璇存槑銆?
浣跨敤閫氱敤 struct clk_core 鐨勫钩鍙板拰璁惧锛屽埄鐢?struct clk_core 涓殑 struct clk_ops 鎸囬拡
鏉ユ墽琛岀‖浠剁浉鍏崇殑閮ㄥ垎锛屼緥濡傦細

```
	struct clk_ops {
		int		(*prepare)(struct clk_hw *hw);
		void		(*unprepare)(struct clk_hw *hw);
		int		(*is_prepared)(struct clk_hw *hw);
		void		(*unprepare_unused)(struct clk_hw *hw);
		int		(*enable)(struct clk_hw *hw);
		void		(*disable)(struct clk_hw *hw);
		int		(*is_enabled)(struct clk_hw *hw);
		void		(*disable_unused)(struct clk_hw *hw);
		unsigned long	(*recalc_rate)(struct clk_hw *hw,
						unsigned long parent_rate);
		int		(*determine_rate)(struct clk_hw *hw,
						  struct clk_rate_request *req);
		int		(*set_parent)(struct clk_hw *hw, u8 index);
		u8		(*get_parent)(struct clk_hw *hw);
		int		(*set_rate)(struct clk_hw *hw,
					    unsigned long rate,
					    unsigned long parent_rate);
		int		(*set_rate_and_parent)(struct clk_hw *hw,
					    unsigned long rate,
					    unsigned long parent_rate,
					    u8 index);
		unsigned long	(*recalc_accuracy)(struct clk_hw *hw,
						unsigned long parent_accuracy);
		int		(*get_phase)(struct clk_hw *hw);
		int		(*set_phase)(struct clk_hw *hw, int degrees);
		void		(*init)(struct clk_hw *hw);
		void		(*debug_init)(struct clk_hw *hw,
					      struct dentry *dentry);
	};

```
## 纭欢鏃堕挓瀹炵幇

閫氱敤 struct clk_core 鐨勫己澶т箣澶勫湪浜庡畠鐨?.ops 鍜?.hw 鎸囬拡锛屽畠浠皢 struct clk 鐨勭粏鑺備笌
纭欢鐩稿叧閮ㄥ垎鐩镐簰鎶借薄寮€鏉ワ紝鍙嶄箣浜︾劧銆備负浜嗚鏄庤繖涓€鐐癸紝璇疯€冭檻濡備笅绠€鍗曠殑鍙棬鎺ф椂閽熷疄鐜帮細

```
	struct clk_gate {
		struct clk_hw	hw;
		void __iomem    *reg;
		u8              bit_idx;
		...
	};

```
struct clk_gate 鍖呭惈 struct clk_hw hw锛屼互鍙婂叧浜庡摢涓瘎瀛樺櫒鍜屽摢涓€浣嶆帶鍒惰鏃堕挓闂ㄦ帶鐨勭‖浠?鐩稿叧鐭ヨ瘑銆傝繖閲屼笉闇€瑕佷换浣曞叧浜庢椂閽熸嫇鎵戞垨璁拌处锛堝 enable_count 鎴?notifier_count锛夌殑淇℃伅锛?杩欎簺鍏ㄩ儴鐢遍€氱敤妗嗘灦浠ｇ爜鍜?struct clk_core 澶勭悊銆?
```
	struct clk *clk;
	clk = clk_get(NULL, "my_gateable_clk");

	clk_prepare(clk);
	clk_enable(clk);

```
```
	clk_enable(clk);
		clk->ops->enable(clk->hw);
		[resolves to...]
			clk_gate_enable(hw);
			[resolves struct clk gate with to_clk_gate(hw)]
				clk_gate_set_bit(gate);

```
```
	static void clk_gate_set_bit(struct clk_gate *gate)
	{
		u32 reg;

		reg = __raw_readl(gate->reg);
		reg |= BIT(gate->bit_idx);
		writel(reg, gate->reg);
	}

```
```
	#define to_clk_gate(_hw) container_of(_hw, struct clk_gate, hw)

```
杩欑鎶借薄妯″紡琚敤浜庢瘡涓€绉嶆椂閽熺‖浠剁殑琛ㄧず銆?
## 鏀寔浣犺嚜宸辩殑鏃堕挓纭欢

褰撲负鏂板瀷鏃堕挓瀹炵幇鏀寔鏃讹紝鍙渶寮曞叆锛?
```
	#include <linux/clk-provider.h>

```
瑕佷负浣犵殑骞冲彴鏋勯€犱竴涓椂閽熺‖浠剁粨鏋勶紝浣犲繀椤诲畾涔夛細

```
	struct clk_foo {
		struct clk_hw hw;
		... hardware specific data goes here ...
	};

```
涓轰簡鍒╃敤浣犵殑鏁版嵁锛屼綘闇€瑕佹敮鎸佹湁鏁堢殑鎿嶄綔锛?
```
	struct clk_ops clk_foo_ops = {
		.enable		= &clk_foo_enable,
		.disable	= &clk_foo_disable,
	};

```
```
	#define to_clk_foo(_hw) container_of(_hw, struct clk_foo, hw)

	int clk_foo_enable(struct clk_hw *hw)
	{
		struct clk_foo *foo;

		foo = to_clk_foo(hw);

		... perform magic on foo ...

		return 0;
	};

```
涓嬮潰鏄竴寮犵煩闃碉紝璇︾粏璇存槑鏍规嵁鏃堕挓鐨勭‖浠惰兘鍔涘摢浜?clk_ops 鏄繀闇€鐨勩€傛爣璁颁负鈥測鈥濈殑鍗曞厓鏍?琛ㄧず蹇呴渶锛涙爣璁颁负鈥渘鈥濈殑鍗曞厓鏍艰〃绀鸿鍥炶皟瑕佷箞鏃犳晥锛岃涔堜笉闇€瑕佸寘鍚€傜┖鐧藉崟鍏冩牸琛ㄧず鍙€夛紝
鎴栧繀椤绘牴鎹叿浣撴儏鍐佃瘎浼般€?
   +----------------+------+-------------+---------------+-------------+------+
   |                | gate | change rate | single parent | multiplexer | root |
   +================+======+=============+===============+=============+======+
   |.prepare        |      |             |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   |.unprepare      |      |             |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   +----------------+------+-------------+---------------+-------------+------+
   |.enable         | y    |             |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   |.disable        | y    |             |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   |.is_enabled     | y    |             |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   +----------------+------+-------------+---------------+-------------+------+
   |.recalc_rate    |      | y           |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   |.determine_rate |      | y           |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   |.set_rate       |      | y           |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   +----------------+------+-------------+---------------+-------------+------+
   |.set_parent     |      |             | n             | y           | n    |
   +----------------+------+-------------+---------------+-------------+------+
   |.get_parent     |      |             | n             | y           | n    |
   +----------------+------+-------------+---------------+-------------+------+
   +----------------+------+-------------+---------------+-------------+------+
   |.recalc_accuracy|      |             |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   +----------------+------+-------------+---------------+-------------+------+
   |.init           |      |             |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+

鏈€鍚庯紝浣跨敤纭欢鐩稿叧鐨勬敞鍐屽嚱鏁板湪杩愯鏃舵敞鍐屼綘鐨勬椂閽熴€傝鍑芥暟鍙槸濉厖 struct clk_foo 鐨?鏁版嵁锛岀劧鍚庡皢閫氱敤 struct clk 鍙傛暟浼犻€掔粰妗嗘灦锛屼緥濡傦細

```
	clk_register(...)

```
鐩稿叧绀轰緥璇峰弬瑙?`drivers/clk/clk-*.c` 涓殑鍩烘湰鏃堕挓绫诲瀷銆?
## 绂佹瀵规湭浣跨敤鏃堕挓杩涜闂ㄦ帶

鍦ㄥ紑鍙戣繃绋嬩腑锛屾湁鏃惰兘澶熺粫杩囬粯璁ゅ鏈娇鐢ㄦ椂閽熺殑绂佺敤浼氬緢鏈夌敤銆備緥濡傦紝濡傛灉椹卞姩娌℃湁姝ｇ‘鍦?鍚敤鏃堕挓锛岃€屾槸渚濊禆瀹冧滑浠?bootloader 璧峰氨澶勪簬寮€鍚姸鎬侊紝閭ｄ箞缁曡繃绂佺敤鎰忓懗鐫€鍦ㄨ闂琚?瑙ｅ喅涔嬪墠椹卞姩浠嶈兘姝ｅ父宸ヤ綔銆?
浣犲彲浠ラ€氳繃鍦ㄥ唴鏍稿惎鍔ㄦ椂浣跨敤浠ヤ笅鍙傛暟鏉ユ煡鐪嬪摢浜涙椂閽熷凡琚鐢細

```
 tp_printk trace_event=clk:clk_disable

```
瑕佺粫杩囪繖绉嶇鐢紝璇峰湪浼犵粰鍐呮牳鐨?bootargs 涓寘鍚?"clk_ignore_unused"銆?
## 閿?
閫氱敤鏃堕挓妗嗘灦浣跨敤涓ゆ妸鍏ㄥ眬閿侊細prepare 閿佸拰 enable 閿併€?
enable 閿佹槸涓€鎶婅嚜鏃嬮攣锛屽湪瀵?.enable銆?disable 鎿嶄綔鐨勮皟鐢ㄦ湡闂存寔鏈夈€傚洜姝よ繖浜涙搷浣滀笉鍏佽
鐫＄湢锛屽苟涓斿 clk_enable()銆乧lk_disable() API 鍑芥暟鐨勮皟鐢ㄥ厑璁稿湪鍘熷瓙涓婁笅鏂囦腑杩涜銆?
瀵逛簬 clk_is_enabled() API锛屽畠鍚屾牱琚璁′负鍏佽鍦ㄥ師瀛愪笂涓嬫枃涓娇鐢ㄣ€傜劧鑰岋紝鍦ㄦ鏋舵牳蹇冧腑
鎸佹湁 enable 閿佸叾瀹炲苟娌℃湁澶ぇ鎰忎箟锛岄櫎闈炰綘鎯冲湪鎸佹湁璇ラ攣鐨勫悓鏃跺埄鐢ㄥ惎鐢ㄧ姸鎬佺殑淇℃伅鍋氬叾浠?浜嬫儏銆傚惁鍒欙紝鏌ョ湅鏌愪釜鏃堕挓鏄惁鍚敤鍙槸瀵瑰惎鐢ㄧ姸鎬佺殑涓€娆℃€ц鍙栵紝鑰屽湪鍑芥暟杩斿洖鍚庤鐘舵€佸緢鍙兘
绔嬪埢灏变細鏀瑰彉锛堝洜涓洪攣宸茶閲婃斁锛夈€傚洜姝わ紝璇?API 鐨勭敤鎴烽渶瑕佽嚜琛屽皢璇ョ姸鎬佺殑璇诲彇涓庡叾鐢ㄩ€旇繘琛?鍚屾锛屼互纭繚鍚敤鐘舵€佸湪姝ゆ湡闂翠笉浼氬彂鐢熷彉鍖栥€?
prepare 閿佹槸涓€鎶婁簰鏂ヤ綋锛坢utex锛夛紝鍦ㄥ鎵€鏈夊叾浠栨搷浣滅殑璋冪敤鏈熼棿鎸佹湁銆傛墍鏈夎繖浜涙搷浣滈兘鍏佽
鐫＄湢锛屽洜姝ゅ鐩稿簲 API 鍑芥暟鐨勮皟鐢ㄤ笉鍏佽鍦ㄥ師瀛愪笂涓嬫枃涓繘琛屻€?
浠庡姞閿佺殑瑙掑害鐪嬶紝杩欏疄闄呬笂灏嗘搷浣滃垎鎴愪簡涓ょ粍銆?
椹卞姩涓嶉渶瑕佹墜鍔ㄤ繚鎶や竴缁勬搷浣滃唴閮ㄥ叡浜殑璧勬簮锛屾棤璁鸿繖浜涜祫婧愭槸鍚﹁澶氫釜鏃堕挓鍏变韩銆傜劧鑰岋紝瀵逛簬
琚袱缁勬搷浣滃叡浜殑璧勬簮鐨勮闂紝闇€瑕佺敱椹卞姩鏉ヤ繚鎶ゃ€傛绫昏祫婧愮殑涓€涓緥瀛愭槸鍚屾椂鎺у埗鏃堕挓閫熺巼鍜?鏃堕挓鍚敤/绂佺敤鐘舵€佺殑瀵勫瓨鍣ㄣ€?
鏃堕挓妗嗘灦鏄彲閲嶅叆鐨勶紝鍗抽┍鍔ㄥ厑璁稿湪鍏舵椂閽熸搷浣滅殑瀹炵幇鍐呴儴璋冪敤鏃堕挓妗嗘灦鍑芥暟銆備緥濡傦紝杩欏彲鑳藉鑷?涓€涓椂閽熺殑 .set_rate 鎿嶄綔鍦ㄥ彟涓€涓椂閽熺殑 .set_rate 鎿嶄綔鍐呴儴琚皟鐢ㄣ€傞┍鍔ㄥ疄鐜颁腑蹇呴』鑰冭檻
杩欑鎯呭喌锛屼笉杩囨鏃剁殑浠ｇ爜娴侀€氬父鐢遍┍鍔ㄦ帶鍒躲€?
璇锋敞鎰忥紝褰撻€氱敤鏃堕挓妗嗘灦涔嬪鐨勪唬鐮侀渶瑕佽闂椂閽熸搷浣滄墍浣跨敤鐨勮祫婧愭椂锛屼篃蹇呴』鑰冭檻鍔犻攣闂銆?杩欒瑙嗕负瓒呭嚭鏈枃妗ｈ寖鍥淬€?