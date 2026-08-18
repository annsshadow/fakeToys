## FPGA Manager


### Overview


FPGA manager 鏍稿績瀵煎嚭涓€缁勭敤浜庨€氳繃闀滃儚瀵?FPGA 杩涜缂栫▼鐨勫嚱鏁般€傝 API 涓庡巶鍟嗘棤鍏炽€傛墍鏈夊巶鍟嗙浉鍏崇殑缁嗚妭閮介殣钘忓湪搴曞眰椹卞姩涓紝璇ラ┍鍔ㄥ悜鏍稿績娉ㄥ唽涓€缁?ops銆侳PGA 闀滃儚鏁版嵁鏈韩鏄笌鍘傚晢寮虹浉鍏崇殑锛屼絾鍦ㄦ垜浠繖閲屽畠鍙槸浜岃繘鍒舵暟鎹€侳PGA manager 鏍稿績涓嶄細瑙ｆ瀽瀹冦€?
寰呯紪绋嬬殑 FPGA 闀滃儚鍙互浣嶄簬鍒嗘暎/鑱氶泦锛坰catter gather锛夊垪琛ㄣ€佸崟涓繛缁紦鍐插尯鎴栧浐浠舵枃浠朵腑銆傜敱浜庡簲褰撻伩鍏嶄负缂撳啿鍖哄垎閰嶈繛缁殑鍐呮牳鍐呭瓨锛屽洜姝ゅ缓璁敤鎴峰敖鍙兘鏀圭敤鍒嗘暎/鑱氶泦鍒楄〃銆?
缂栫▼闀滃儚鐨勫叿浣撳弬鏁扮敱涓€涓粨鏋勪綋锛坰truct fpga_image_info锛夌粰鍑恒€傝缁撴瀯浣撳寘鍚濡傛寚鍚?FPGA 闀滃儚鐨勬寚閽堬紝浠ュ強闀滃儚鐗规湁鐨勫弬鏁帮紙渚嬪璇ラ暅鍍忔槸閽堝瀹屾暣杩樻槸閮ㄥ垎閲嶉厤缃€屾瀯寤虹殑锛夈€?
### How to support a new FPGA device


瑕佹柊澧炰竴涓?FPGA manager锛岄渶缂栧啓涓€涓疄鐜颁簡鏌愮粍 ops 鐨勯┍鍔ㄣ€傚叾 probe 鍑芥暟璋冪敤 `fpga_mgr_register()` 鎴?`fpga_mgr_register_full()`锛?```

	static const struct fpga_manager_ops socfpga_fpga_ops = {
		.write_init = socfpga_fpga_ops_configure_init,
		.write = socfpga_fpga_ops_configure_write,
		.write_complete = socfpga_fpga_ops_configure_complete,
		.state = socfpga_fpga_ops_state,
	};

	static int socfpga_fpga_probe(struct platform_device *pdev)
	{
		struct device *dev = &pdev->dev;
		struct socfpga_fpga_priv *priv;
		struct fpga_manager *mgr;
		int ret;

		priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
		if (!priv)
			return -ENOMEM;

		/*
		 * do ioremaps, get interrupts, etc. and save
		 * them in priv
		 */

		mgr = fpga_mgr_register(dev, "Altera SOCFPGA FPGA Manager",
					&socfpga_fpga_ops, priv);
		if (IS_ERR(mgr))
			return PTR_ERR(mgr);

		platform_set_drvdata(pdev, mgr);

		return 0;
	}

	static int socfpga_fpga_remove(struct platform_device *pdev)
	{
		struct fpga_manager *mgr = platform_get_drvdata(pdev);

		fpga_mgr_unregister(mgr);

		return 0;
	}

```
鍙﹀锛宲robe 鍑芥暟涔熷彲浠ヨ皟鐢ㄦ煇涓祫婧愭墭绠★紙resource managed锛夌殑娉ㄥ唽鍑芥暟 `devm_fpga_mgr_register()` 鎴?`devm_fpga_mgr_register_full()`銆備娇鐢ㄨ繖浜涘嚱鏁版椂鍙傛暟璇硶鐩稿悓锛屼絾搴斿綋鍘绘帀瀵?`fpga_mgr_unregister()` 鐨勮皟鐢ㄣ€傚湪涓婇潰鐨勪緥瀛愪腑锛宍socfpga_fpga_remove()` 鍑芥暟灏变笉鍐嶉渶瑕佷簡銆?
ops 灏嗗疄鐜伴拡瀵硅鐗瑰畾 FPGA 杩涜缂栫▼搴忓垪鎵€闇€鐨勫悇绉嶈澶囩浉鍏崇殑瀵勫瓨鍣ㄥ啓鍏ャ€傝繖浜?ops 鍦ㄦ垚鍔熸椂杩斿洖 0锛屽惁鍒欒繑鍥炶礋鐨勯敊璇爜銆?
```
 1. .parse_header (optional, may be called once or multiple times)
 2. .write_init
 3. .write or .write_sg (may be called once or multiple times)
 4. .write_complete

```
`.parse_header` 鍑芥暟浼氭妸 header_size 鍜?data_size 璁剧疆鍒?struct fpga_image_info 涓€傚湪璋冪敤 parse_header 涔嬪墠锛宧eader_size 鐢?initial_header_size 鍒濆鍖栥€傚鏋?fpga_manager_ops 鐨?skip_header 鏍囧織涓虹湡锛屽垯 `.write` 鍑芥暟灏嗚幏寰椾粠寮€澶翠綅缃?header_size 鍋忕Щ澶勫紑濮嬬殑闀滃儚缂撳啿鍖恒€傚鏋滆缃簡 data_size锛宍.write` 鍑芥暟灏嗚幏寰?data_size 瀛楄妭鐨勯暅鍍忕紦鍐插尯锛屽惁鍒?`.write` 灏嗚幏寰楃洿鍒伴暅鍍忕紦鍐插尯鏈熬鐨勬暟鎹€傝繖涓嶄細褰卞搷 `.write_sg`锛宍.write_sg` 浠嶇劧浠?sg_table 褰㈠紡鑾峰緱鏁翠釜闀滃儚銆傚鏋?FPGA 闀滃儚宸茶鏄犲皠涓哄崟涓繛缁紦鍐插尯锛屽垯鏁翠釜缂撳啿鍖轰細琚紶鍏?`.parse_header`銆傚鏋滈暅鍍忎互鍒嗘暎/鑱氶泦褰㈠紡瀛樺湪锛屾牳蹇冧唬鐮佷細鍦ㄧ涓€娆¤皟鐢?`.parse_header` 涔嬪墠鑷冲皯缂撳啿 `.initial_header_size` 澶у皬锛屽鏋滀笉澶燂紝`.parse_header` 搴旀妸鏈熸湜鐨勫ぇ灏忓啓鍏?info->header_size 骞惰繑鍥?-EAGAIN锛岄殢鍚庝細甯︾潃鏇村ぇ鐨勯暅鍍忕紦鍐插尯閮ㄥ垎鍐嶆琚皟鐢ㄣ€?
`.write_init` 鍑芥暟鐢ㄤ簬璁?FPGA 鍑嗗濂芥帴鏀堕暅鍍忔暟鎹€備紶鍏?`.write_init` 鐨勭紦鍐插尯鑷冲皯闀?info->header_size 瀛楄妭锛涘鏋滄暣涓瘮鐗规祦涓嶈兘绔嬪嵆鍙敤锛屾牳蹇冧唬鐮佷細鍦ㄥ紑濮嬩箣鍓嶈嚦灏戠紦鍐茶繖涔堝銆?
`.write` 鍑芥暟鍚?FPGA 鍐欏叆涓€涓紦鍐插尯銆傝缂撳啿鍖哄彲鑳藉寘鍚暣涓?FPGA 闀滃儚锛屼篃鍙兘鍙槸 FPGA 闀滃儚鐨勪竴灏忔銆傚湪鍚庝竴绉嶆儏鍐典笅锛岃鍑芥暟浼氳澶氭璋冪敤浠ュ啓鍏ヨ繛缁殑鐗囨銆傛鎺ュ彛閫傚悎浣跨敤 PIO 鐨勯┍鍔ㄣ€?
`.write_sg` 鐗堟湰鐨勮涓轰笌 `.write` 鐩稿悓锛屽彧鏄緭鍏ユ槸涓€涓?sg_table 鍒嗘暎鍒楄〃銆傛鎺ュ彛閫傚悎浣跨敤 DMA 鐨勯┍鍔ㄣ€?
`.write_complete` 鍑芥暟鍦ㄦ墍鏈夐暅鍍忓啓鍏ュ畬鎴愬悗琚皟鐢紝鐢ㄤ簬灏?FPGA 缃叆宸ヤ綔妯″紡銆?
ops 杩樺寘鍚竴涓?`.state` 鍑芥暟锛岀敤浜庣‘瀹?FPGA 鎵€澶勭殑鐘舵€佸苟杩斿洖 enum fpga_mgr_states 绫诲瀷鐨勪唬鐮併€傚畠涓嶄細瀵艰嚧鐘舵€佸彂鐢熸敼鍙樸€?
### API for implementing a new FPGA Manager driver


- `fpga_mgr_states` -  :c`fpga_manager->state` 鐨勫彇鍊笺€?- struct fpga_manager -  FPGA manager 缁撴瀯浣?- struct fpga_manager_ops -  搴曞眰 FPGA manager 椹卞姩 ops
- struct fpga_manager_info -  fpga_mgr_register_full() 鐨勫弬鏁扮粨鏋勪綋
- __fpga_mgr_register_full() -  浣跨敤 fpga_mgr_info 缁撴瀯浣撳垱寤哄苟娉ㄥ唽涓€涓?FPGA manager锛屼互鎻愪緵鏈€澶х伒娲诲害鐨勯€夐」
- __fpga_mgr_register() -  浣跨敤鏍囧噯鍙傛暟鍒涘缓骞舵敞鍐屼竴涓?FPGA manager
- __devm_fpga_mgr_register_full() -  __fpga_mgr_register_full() 鐨勮祫婧愭墭绠＄増鏈?- __devm_fpga_mgr_register() -  __fpga_mgr_register() 鐨勮祫婧愭墭绠＄増鏈?- fpga_mgr_unregister() -  娉ㄩ攢涓€涓?FPGA manager

杈呭姪瀹?`fpga_mgr_register_full()`銆乣fpga_mgr_register()`銆乣devm_fpga_mgr_register_full()` 鍜?`devm_fpga_mgr_register()` 鍙敤浜庣畝鍖栨敞鍐岃繃绋嬨€?
   :functions: fpga_mgr_states

   :functions: fpga_manager

   :functions: fpga_manager_ops

   :functions: fpga_manager_info

   :functions: __fpga_mgr_register_full

   :functions: __fpga_mgr_register

   :functions: __devm_fpga_mgr_register_full

   :functions: __devm_fpga_mgr_register

   :functions: fpga_mgr_unregister
