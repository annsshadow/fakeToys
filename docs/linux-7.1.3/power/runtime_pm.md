## I/O 璁惧鐨勮繍琛屾椂鐢垫簮绠＄悊妗嗘灦


(C) 2009-2011 Rafael J. Wysocki <rjw@sisk.pl>, Novell Inc.

(C) 2010 Alan Stern <stern@rowland.harvard.edu>

(C) 2014 Intel Corp., Rafael J. Wysocki <rafael.j.wysocki@intel.com>

## 1. 寮曡█


I/O 璁惧鐨勮繍琛屾椂鐢垫簮绠＄悊锛坮untime PM锛夋敮鎸佺敱鐢垫簮绠＄悊鏍稿績锛圥M core锛夊湪浠ヤ笅灞傞潰鎻愪緵锛?

- 鐢垫簮绠＄悊宸ヤ綔闃熷垪 pm_wq锛屾€荤嚎绫诲瀷鍜岃澶囬┍鍔ㄥ彲浠ュ湪鍏朵腑鏀惧叆瀹冧滑涓?PM 鐩稿叧鐨勫伐浣滈」銆傚己鐑堝缓璁?pm_wq 鐢ㄤ簬鎺掗槦鎵€鏈変笌杩愯鏃?PM 鐩稿叧鐨勫伐浣滈」锛屽洜涓鸿繖浣垮緱瀹冧滑鑳藉涓庣郴缁熻寖鍥寸殑鐢垫簮鐘舵€佸垏鎹紙鎸傝捣鍒?RAM銆佷紤鐪犱互鍙婁粠绯荤粺鐫＄湢鐘舵€佹仮澶嶏級淇濇寔鍚屾銆俻m_wq 鍦?include/linux/pm_runtime.h 涓０鏄庯紝鍦?kernel/power/main.c 涓畾涔夈€?

- 'struct device' 鐨?'power' 鎴愬憳锛堝叾绫诲瀷涓?'struct dev_pm_info'锛屽畾涔夊湪 include/linux/pm.h 涓級涓殑鑻ュ共杩愯鏃?PM 瀛楁锛屽彲鐢ㄤ簬鎶婅繍琛屾椂 PM 鎿嶄綔褰兼鍚屾銆?

- 'struct dev_pm_ops'锛堝畾涔夊湪 include/linux/pm.h 涓級涓殑涓変釜璁惧杩愯鏃?PM 鍥炶皟銆?

- 涓€缁勫畾涔夊湪 drivers/base/power/runtime.c 涓殑杈呭姪鍑芥暟锛屽彲鐢ㄤ簬鎵ц杩愯鏃?PM 鎿嶄綔锛岃€岃繖浜涙搷浣滀箣闂寸殑鍚屾鐢?PM core 璐熻矗銆傞紦鍔辨€荤嚎绫诲瀷鍜岃澶囬┍鍔ㄤ娇鐢ㄨ繖浜涘嚱鏁般€?

'struct dev_pm_ops' 涓殑杩愯鏃?PM 鍥炶皟銆?struct dev_pm_info' 鐨勮澶囪繍琛屾椂 PM 瀛楁浠ュ強涓鸿繍琛屾椂 PM 鎻愪緵鐨勬牳蹇冭緟鍔╁嚱鏁板皢鍦ㄤ笅闈㈡弿杩般€?

## 2. 璁惧杩愯鏃?PM 鍥炶皟


```
  struct dev_pm_ops {
	...
	int (*runtime_suspend)(struct device *dev);
	int (*runtime_resume)(struct device *dev);
	int (*runtime_idle)(struct device *dev);
	...
  };

```
PM core 涓鸿璁惧鐨勫瓙绯荤粺鎵ц ->runtime_suspend()銆?>runtime_resume() 鍜?->runtime_idle() 鍥炶皟锛屽瓙绯荤粺鍙兘鏄互涓嬩箣涓€锛?

  1. 璁惧鐨?PM 鍩燂紙PM domain锛夛紝濡傛灉璁惧鐨?PM 鍩熷璞?dev->pm_domain 瀛樺湪銆?

  2. 璁惧鐨勮澶囩被鍨嬶紙device type锛夛紝濡傛灉 dev->type 鍜?dev->type->pm 閮藉瓨鍦ㄣ€?

  3. 璁惧鐨勮澶囩被锛坉evice class锛夛紝濡傛灉 dev->class 鍜?dev->class->pm 閮藉瓨鍦ㄣ€?

  4. 璁惧鐨勬€荤嚎绫诲瀷锛坆us type锛夛紝濡傛灉 dev->bus 鍜?dev->bus->pm 閮藉瓨鍦ㄣ€?

濡傛灉搴旂敤涓婅堪瑙勫垯鎵€閫夋嫨鐨勫瓙绯荤粺娌℃湁鎻愪緵鐩稿叧鐨勫洖璋冿紝PM core 灏嗙洿鎺ヨ皟鐢ㄥ瓨鍌ㄥ湪 dev->driver->pm 涓搴旂殑椹卞姩鍥炶皟锛堝鏋滃瓨鍦ㄧ殑璇濓級銆?

PM core 鎬绘槸鎸変笂闈㈢粰鍑虹殑椤哄簭妫€鏌ヤ娇鐢ㄥ摢涓洖璋冿紝鍥犳鍥炶皟鐨勪紭鍏堢骇浠庨珮鍒颁綆涓猴細PM 鍩熴€佽澶囩被鍨嬨€佺被鍜屾€荤嚎绫诲瀷銆傛澶栵紝楂樹紭鍏堢骇鐨勫洖璋冩€绘槸浼樺厛浜庝綆浼樺厛绾х殑鍥炶皟銆侾M 鍩熴€佹€荤嚎绫诲瀷銆佽澶囩被鍨嬪拰绫诲洖璋冨湪涓嬮潰琚О涓哄瓙绯荤粺绾э紙subsystem-level锛夊洖璋冦€?

榛樿鎯呭喌涓嬶紝鍥炶皟鎬绘槸鍦ㄤ腑鏂娇鑳界殑杩涚▼涓婁笅鏂囦腑琚皟鐢ㄣ€備笉杩囷紝pm_runtime_irq_safe() 杈呭姪鍑芥暟鍙敤鏉ュ憡璇?PM core锛屽湪涓柇绂佺敤銆佸師瀛愪笂涓嬫枃涓繍琛岃缁欏畾璁惧鐨?->runtime_suspend()銆?>runtime_resume() 鍜?->runtime_idle() 鍥炶皟鏄畨鍏ㄧ殑銆傝繖鎰忓懗鐫€鐩稿叧鐨勫洖璋冧緥绋嬩笉鑳介樆濉炴垨鐫＄湢锛屼絾涔熸剰鍛崇潃绗?4 鑺傛湯灏惧垪鍑虹殑鍚屾杈呭姪鍑芥暟鍙敤浜庤璁惧鐨勪腑鏂鐞嗙▼搴忎腑锛屾垨涓€鑸€岃█鍦ㄥ師瀛愪笂涓嬫枃涓娇鐢ㄣ€?

瀛愮郴缁熺骇鎸傝捣鍥炶皟锛堝鏋滃瓨鍦級_瀹屽叏_ _璐熻矗_ 浠ラ€傚綋鐨勬柟寮忓鐞嗚澶囩殑鎸傝捣锛屽叾涓彲浠ワ紙浣嗕笉闇€瑕侊級鍖呮嫭鎵ц璁惧椹卞姩鑷韩鐨?->runtime_suspend() 鍥炶皟锛堜粠 PM core 鐨勮搴︾湅锛屽彧瑕佸瓙绯荤粺绾ф寕璧峰洖璋冪煡閬撳浣曞鐞嗚璁惧锛屽氨鏃犻渶鍦ㄨ澶囬┍鍔ㄤ腑瀹炵幇 ->runtime_suspend() 鍥炶皟锛夈€?

  - 涓€鏃﹀瓙绯荤粺绾ф寕璧峰洖璋冿紙鎴栬€呭鏋滅洿鎺ヨ皟鐢ㄥ垯涓洪┍鍔ㄦ寕璧峰洖璋冿級涓虹粰瀹氳澶囨垚鍔熷畬鎴愶紝PM core 灏辫涓鸿璁惧宸茶鎸傝捣锛岃繖鏈繀鎰忓懗鐫€瀹冨凡琚疆浜庝綆鍔熻€楃姸鎬併€備笉杩囷紝瀹冨簲褰撴剰鍛崇潃鍦ㄤ负鍏舵墽琛岄€傚綋鐨勬仮澶嶅洖璋冧箣鍓嶏紝璇ヨ澶囧皢涓嶅鐞嗘暟鎹€佷篃涓嶄笌 CPU 鍜?RAM 閫氫俊銆傛寕璧峰洖璋冩垚鍔熸墽琛屽悗锛岃澶囩殑杩愯鏃?PM 鐘舵€佷负 'suspended'銆?

  - 濡傛灉鎸傝捣鍥炶皟杩斿洖 -EBUSY 鎴?-EAGAIN锛岃澶囩殑杩愯鏃?PM 鐘舵€佷繚鎸佷负 'active'锛岃繖鎰忓懗鐫€涔嬪悗璇ヨ澶?_蹇呴』_ 瀹屽叏鍙搷浣溿€?

  - 濡傛灉鎸傝捣鍥炶皟杩斿洖涓€涓笉鍚屼簬 -EBUSY 鍜?-EAGAIN 鐨勯敊璇爜锛孭M core 灏嗘瑙嗕负鑷村懡閿欒锛屽苟鎷掔粷杩愯绗?4 鑺傛弿杩扮殑杈呭姪鍑芥暟锛岀洿鍒板叾鐘舵€佽鐩存帴璁剧疆涓?'active' 鎴?'suspended'锛圥M core 涓烘鎻愪緵浜嗙壒娈婄殑杈呭姪鍑芥暟锛夈€?

鐗瑰埆鍦帮紝濡傛灉椹卞姩闇€瑕佽繙绋嬪敜閱掕兘鍔涳紙鍗冲厑璁歌澶囪姹傛敼鍙樺叾鐢垫簮鐘舵€佺殑纭欢鏈哄埗锛屼緥濡?PCI PME锛夋墠鑳芥甯稿伐浣滐紝鑰?device_can_wakeup() 瀵硅璁惧杩斿洖 'false'锛岄偅涔?->runtime_suspend() 搴斿綋杩斿洖 -EBUSY銆傚彟涓€鏂归潰锛屽鏋?device_can_wakeup() 瀵硅璁惧杩斿洖 'true'锛屽苟涓斿湪鎵ц鎸傝捣鍥炶皟鏈熼棿璁惧琚疆浜庝綆鍔熻€楃姸鎬侊紝鍒欓鏈熷皢涓鸿璁惧鍚敤杩滅▼鍞ら啋銆備竴鑸€岃█锛屽浜庡湪杩愯鏃惰缃簬浣庡姛鑰楃姸鎬佺殑鎵€鏈夎緭鍏ヨ澶囷紝閮藉簲鍚敤杩滅▼鍞ら啋銆?

瀛愮郴缁熺骇鎭㈠鍥炶皟锛堝鏋滃瓨鍦級**瀹屽叏璐熻矗** 浠ラ€傚綋鐨勬柟寮忓鐞嗚澶囩殑鎭㈠锛屽叾涓彲浠ワ紙浣嗕笉闇€瑕侊級鍖呮嫭鎵ц璁惧椹卞姩鑷韩鐨?->runtime_resume() 鍥炶皟锛堜粠 PM core 鐨勮搴︾湅锛屽彧瑕佸瓙绯荤粺绾ф仮澶嶅洖璋冪煡閬撳浣曞鐞嗚璁惧锛屽氨鏃犻渶鍦ㄨ澶囬┍鍔ㄤ腑瀹炵幇 ->runtime_resume() 鍥炶皟锛夈€?

  - 涓€鏃﹀瓙绯荤粺绾ф仮澶嶅洖璋冿紙鎴栬€呭鏋滅洿鎺ヨ皟鐢ㄥ垯涓洪┍鍔ㄦ仮澶嶅洖璋冿級鎴愬姛瀹屾垚锛孭M core 灏辫涓鸿璁惧瀹屽叏鍙搷浣滐紝杩欐剰鍛崇潃璇ヨ澶?_蹇呴』_ 鑳藉鎸夐渶瀹屾垚 I/O 鎿嶄綔銆傛鏃惰澶囩殑杩愯鏃?PM 鐘舵€佷负 'active'銆?

  - 濡傛灉鎭㈠鍥炶皟杩斿洖閿欒鐮侊紝PM core 灏嗘瑙嗕负鑷村懡閿欒锛屽苟鎷掔粷杩愯绗?4 鑺傛弿杩扮殑杈呭姪鍑芥暟锛岀洿鍒板叾鐘舵€佽鐩存帴璁剧疆涓?'active' 鎴?'suspended'锛堥€氳繃 PM core 涓烘鎻愪緵鐨勭壒娈婅緟鍔╁嚱鏁帮級銆?

绌洪棽锛坕dle锛夊洖璋冿紙濡傛灉瀛樺湪鍒欎负瀛愮郴缁熺骇鐨勶紝鍚﹀垯涓洪┍鍔ㄧ殑锛夊湪璁惧鐪嬩笂鍘荤┖闂叉椂鐢?PM core 鎵ц锛岃繖鐢变袱涓鏁板櫒鍚?PM core 鎸囩ず锛氳澶囩殑浣跨敤璁℃暟鍣紙usage counter锛夊拰璁惧"active"瀛愯澶囩殑璁℃暟鍣ㄣ€?

  - 濡傛灉浣跨敤 PM core 鎻愪緵鐨勮緟鍔╁嚱鏁颁娇鍏朵腑浠讳竴璁℃暟鍣ㄥ噺灏忥紝涓旂粨鏋滀负闆讹紝鍒欎細妫€鏌ュ彟涓€涓鏁板櫒銆傚鏋滆璁℃暟鍣ㄤ篃绛変簬闆讹紝PM core 灏变互璇ヨ澶囦负鍙傛暟鎵ц绌洪棽鍥炶皟銆?

绌洪棽鍥炶皟鎵ц鐨勬搷浣滃畬鍏ㄥ彇鍐充簬鐩稿叧鐨勫瓙绯荤粺锛堟垨椹卞姩锛夛紝浣嗛鏈熶笖鎺ㄨ崘鐨勬搷浣滄槸妫€鏌ヨ澶囨槸鍚﹀彲浠ヨ鎸傝捣锛堝嵆鎸傝捣璇ヨ澶囨墍闇€鐨勫叏閮ㄦ潯浠舵槸鍚﹂兘婊¤冻锛夛紝骞跺湪杩欑鎯呭喌涓嬩负璁惧鎺掗槦涓€涓寕璧疯姹傘€傚鏋滄病鏈夌┖闂插洖璋冿紝鎴栬€呭洖璋冭繑鍥?0锛岄偅涔?PM core 灏嗗皾璇曞璁惧鎵ц杩愯鏃舵寕璧凤紝鍚屾椂涔熷皧閲嶉厤缃负鑷姩鎸傝捣锛坅utosuspend锛夌殑璁惧銆傛湰璐ㄤ笂杩欐剰鍛崇潃璋冪敤 pm_runtime_autosuspend()銆備负闃叉杩欎竴鐐癸紙渚嬪锛屽鏋滃洖璋冧緥绋嬪凡缁忓惎鍔ㄤ簡涓€涓欢杩熸寕璧凤級锛岃渚嬬▼蹇呴』杩斿洖涓€涓潪闆跺€笺€傝礋鐨勯敊璇繑鍥炵爜浼氳 PM core 蹇界暐銆?

PM core 鎻愪緵鐨勮緟鍔╁嚱鏁帮紙鍦ㄧ 4 鑺傛弿杩帮級淇濊瘉閽堝涓€涓澶囩殑杩愯鏃?PM 鍥炶皟婊¤冻浠ヤ笅绾︽潫锛?

(1) 鍥炶皟涔嬮棿浜掓枼锛堜緥濡傦紝绂佹涓?->runtime_resume() 鎴栧悓涓€璁惧鐨勫彟涓€涓?->runtime_suspend() 瀹炰緥骞惰鎵ц ->runtime_suspend()锛夛紝鍞竴鐨勪緥澶栨槸 ->runtime_suspend() 鎴?->runtime_resume() 鍙互涓?->runtime_idle() 骞惰鎵ц锛堝敖绠″湪涓鸿鍚屼竴璁惧鎵ц浠讳綍鍏朵粬鍥炶皟鏃讹紝涓嶄細鍚姩 ->runtime_idle()锛夈€?

(2) ->runtime_idle() 鍜?->runtime_suspend() 鍙兘瀵?'active' 璁惧鎵ц锛堝嵆 PM core 鍙細涓鸿繍琛屾椂 PM 鐘舵€佷负 'active' 鐨勮澶囨墽琛?->runtime_idle() 鎴?->runtime_suspend()锛夈€?

(3) ->runtime_idle() 鍜?->runtime_suspend() 鍙兘瀵逛娇鐢ㄨ鏁板櫒绛変簬闆?_骞朵笖_ 鍏?active"瀛愯澶囪鏁板櫒绛変簬闆躲€佹垨鍏?'power.ignore_children' 鏍囧織琚疆浣嶇殑璁惧鎵ц銆?

(4) ->runtime_resume() 鍙兘瀵?'suspended' 璁惧鎵ц锛堝嵆 PM core 鍙細涓鸿繍琛屾椂 PM 鐘舵€佷负 'suspended' 鐨勮澶囨墽琛?->runtime_resume()锛夈€?

姝ゅ锛孭M core 鎻愪緵鐨勮緟鍔╁嚱鏁伴伒寰互涓嬭鍒欙細

  - 濡傛灉 ->runtime_suspend() 鍗冲皢琚墽琛岋紝鎴栬€呮湁涓€涓緟鎵ц鐨勮姹傝鎵ц瀹冿紝鍒欎笉浼氫负鍚屼竴璁惧鎵ц ->runtime_idle()銆?

  - 涓€涓墽琛屾垨璋冨害 ->runtime_suspend() 鎵ц鐨勮姹傦紝灏嗗彇娑堝悓涓€璁惧鐨勪换浣曞緟鎵ц鐨?->runtime_idle() 鎵ц璇锋眰銆?

  - 濡傛灉 ->runtime_resume() 鍗冲皢琚墽琛岋紝鎴栬€呮湁涓€涓緟鎵ц鐨勮姹傝鎵ц瀹冿紝鍒欎笉浼氫负鍚屼竴璁惧鎵ц鍏朵粬鍥炶皟銆?

  - 涓€涓墽琛?->runtime_resume() 鐨勮姹傚皢鍙栨秷鍚屼竴璁惧鐨勪换浣曞叾浠栧洖璋冪殑寰呮墽琛屾垨宸茶皟搴︾殑璇锋眰锛屽凡璋冨害鐨勮嚜鍔ㄦ寕璧烽櫎澶栥€?

## 3. 杩愯鏃?PM 璁惧瀛楁


'struct dev_pm_info'锛堝畾涔夊湪 include/linux/pm.h 涓級涓瓨鍦ㄤ互涓嬭澶囪繍琛屾椂 PM 瀛楁锛?

  `struct timer_list suspend_timer;`
    - 鐢ㄤ簬璋冨害锛堝欢杩燂級鎸傝捣鍜岃嚜鍔ㄦ寕璧疯姹傜殑瀹氭椂鍣?

  `unsigned long timer_expires;`
    - 瀹氭椂鍣ㄥ埌鏈熸椂闂达紝浠?jiffies 璁★紙濡傛灉姝ゅ€间笉鍚屼簬闆讹紝鍒欏畾鏃跺櫒姝ｅ湪杩愯锛屽苟灏嗗湪璇ユ椂鍒诲埌鏈燂紝鍚﹀垯瀹氭椂鍣ㄦ湭杩愯锛?

  `struct work_struct work;`
    - 鐢ㄤ簬鎺掗槦璇锋眰锛堝嵆 pm_wq 涓殑宸ヤ綔椤癸級鐨勫伐浣滅粨鏋?

  `wait_queue_head_t wait_queue;`
    - 濡傛灉鏈変换浣曡緟鍔╁嚱鏁伴渶瑕佺瓑寰呭彟涓€涓畬鎴愭椂鎵€浣跨敤鐨勭瓑寰呴槦鍒?

  `spinlock_t lock;`
    - 鐢ㄤ簬鍚屾鐨勮嚜鏃嬮攣

  `atomic_t usage_count;`
    - 璁惧鐨勪娇鐢ㄨ鏁板櫒

  `atomic_t child_count;`
    - 璁惧鐨?'active' 瀛愯澶囪鏁?

  `unsigned int ignore_children;`
    - 濡傛灉缃綅锛屽垯蹇界暐 child_count 鐨勫€硷紙浣嗕粛浼氭洿鏂帮級

  `unsigned int disable_depth;`
    - 鐢ㄤ簬绂佺敤杈呭姪鍑芥暟锛堝鏋滄鍊间负闆跺垯瀹冧滑姝ｅ父宸ヤ綔锛夛紱鍏跺垵濮嬪€间负 1锛堝嵆鎵€鏈夎澶囩殑杩愯鏃?PM 鍒濆鏄鐢ㄧ殑锛?

  `int runtime_error;`
    - 濡傛灉缃綅锛屽垯鍙戠敓杩囪嚧鍛介敊璇紙鏌愪釜鍥炶皟杩斿洖浜嗙 2 鑺傛弿杩扮殑閿欒鐮侊級锛屽洜姝ゅ湪娓呴櫎姝ゆ爣蹇椾箣鍓嶈緟鍔╁嚱鏁颁笉浼氬伐浣滐紱杩欐槸澶辫触鐨勫洖璋冭繑鍥炵殑閿欒鐮?

  `unsigned int idle_notification;`
    - 濡傛灉缃綅锛屽垯 ->runtime_idle() 姝ｅ湪鎵ц

  `unsigned int request_pending;`
    - 濡傛灉缃綅锛屽垯鏈変竴涓緟澶勭悊鐨勮姹傦紙鍗充竴涓帓闃熻繘鍏?pm_wq 鐨勫伐浣滈」锛?

  `enum rpm_request request;`
    - 寰呭鐞嗚姹傜殑绫诲瀷锛坮equest_pending 缃綅鏃舵湁鏁堬級

  `unsigned int deferred_resume;`
    - 濡傛灉鍦ㄦ墽琛岃璁惧鐨?->runtime_suspend() 鏃?->runtime_resume() 鍗冲皢杩愯锛屼笖绛夊緟鎸傝捣瀹屾垚涓嶅垏瀹為檯锛屽垯缃綅锛涙剰涓?涓€鏃︿綘鎸傝捣灏卞惎鍔ㄦ仮澶?

  `enum rpm_status runtime_status;`
    - 璁惧鐨勮繍琛屾椂 PM 鐘舵€侊紱姝ゅ瓧娈电殑鍒濆鍊间负 RPM_SUSPENDED锛岃繖鎰忓懗鐫€鏃犺鍏剁湡瀹炵‖浠剁姸鎬佸浣曪紝姣忎釜璁惧鍦ㄥ垵濮嬫椂閮借 PM core 瑙嗕负 'suspended'

  `enum rpm_status last_status;`
    - 鍦ㄤ负璁惧绂佺敤杩愯鏃?PM 涔嬪墠鎹曡幏鐨勮澶囨渶鍚庝竴娆¤繍琛屾椂 PM 鐘舵€侊紙鍦ㄥ垵濮嬫椂浠ュ強 disable_depth 涓?0 鏃舵棤鏁堬級

  `unsigned int runtime_auto;`
    - 濡傛灉缃綅锛岃〃绀虹敤鎴风┖闂村凡鍏佽璁惧椹卞姩閫氳繃 /sys/devices/.../power/control `interface;` 鍦ㄨ繍琛屾椂瀵硅澶囪繘琛岀數婧愮鐞嗭紱瀹冨彧鑳藉€熷姪 pm_runtime_allow() 鍜?pm_runtime_forbid() 杈呭姪鍑芥暟淇敼

  `unsigned int no_callbacks;`
    - 琛ㄧず璁惧涓嶄娇鐢ㄨ繍琛屾椂 PM 鍥炶皟锛堣绗?8 鑺傦級锛涘畠鍙兘鐢?pm_runtime_no_callbacks() 杈呭姪鍑芥暟淇敼

  `unsigned int irq_safe;`
    - 琛ㄧず ->runtime_suspend() 鍜?->runtime_resume() 鍥炶皟灏嗗湪鎸佹湁鑷棆閿佷笖涓柇绂佺敤鐨勬儏鍐典笅琚皟鐢?

  `unsigned int use_autosuspend;`
    - 琛ㄧず璁惧鐨勯┍鍔ㄦ敮鎸佸欢杩熻嚜鍔ㄦ寕璧凤紙瑙佺 9 鑺傦級锛涘畠鍙兘鐢?pm_runtime{_dont}_use_autosuspend() 杈呭姪鍑芥暟淇敼

  `unsigned int timer_autosuspends;`
    - 琛ㄧず PM core 搴斿湪瀹氭椂鍣ㄥ埌鏈熸椂灏濊瘯鎵ц鑷姩鎸傝捣锛岃€屼笉鏄櫘閫氭寕璧?

  `int autosuspend_delay;`
    - 鐢ㄤ簬鑷姩鎸傝捣鐨勫欢杩熸椂闂达紙浠ユ绉掕锛?

  `unsigned long last_busy;`
    - pm_runtime_mark_last_busy() 杈呭姪鍑芥暟鏈€鍚庝竴娆′负璇ヨ澶囪璋冪敤鐨勬椂闂达紙浠?jiffies 璁★級锛涚敤浜庤绠楄嚜鍔ㄦ寕璧风殑闈炴椿鍔ㄦ椂闂存

浠ヤ笂鎵€鏈夊瓧娈甸兘鏄?'struct device' 鐨?'power' 鎴愬憳鐨勬垚鍛樸€?

## 4. 杩愯鏃?PM 璁惧杈呭姪鍑芥暟


浠ヤ笅杩愯鏃?PM 杈呭姪鍑芥暟瀹氫箟鍦?drivers/base/power/runtime.c 鍜?include/linux/pm_runtime.h 涓細

  `void pm_runtime_init(struct device *dev);`
    - 鍒濆鍖?'struct dev_pm_info' 涓殑璁惧杩愯鏃?PM 瀛楁

  `void pm_runtime_remove(struct device *dev);`
    - 纭繚鍦ㄤ粠璁惧灞傛缁撴瀯涓Щ闄よ璁惧鍚庯紝璇ヨ澶囩殑杩愯鏃?PM 灏嗚绂佺敤

  `int pm_runtime_idle(struct device *dev);`
    - 鎵ц璇ヨ澶囩殑瀛愮郴缁熺骇绌洪棽鍥炶皟锛涘け璐ユ椂杩斿洖閿欒鐮侊紝鍏朵腑 -EINPROGRESS 琛ㄧず ->runtime_idle() 宸插湪鎵ц锛涘鏋滄病鏈夊洖璋冩垨鍥炶皟杩斿洖 0锛屽垯杩愯 pm_runtime_autosuspend(dev) 骞惰繑鍥炲叾缁撴灉

  `int pm_runtime_suspend(struct device *dev);`
    - 鎵ц璇ヨ澶囩殑瀛愮郴缁熺骇鎸傝捣鍥炶皟锛涙垚鍔熸椂杩斿洖 0锛屽鏋滆澶囩殑杩愯鏃?PM 鐘舵€佸凡缁忔槸 'suspended' 鍒欒繑鍥?1锛屽け璐ユ椂杩斿洖閿欒鐮侊紝鍏朵腑 -EAGAIN 鎴?-EBUSY 琛ㄧず灏嗘潵灏濊瘯鎸傝捣璇ヨ澶囨槸瀹夊叏鐨勶紝-EACCES 琛ㄧず 'power.disable_depth' 涓嶅悓浜?0

  `int pm_runtime_autosuspend(struct device *dev);`
    - 涓?pm_runtime_suspend() 鐩稿悓锛屽彧涓嶈繃浼氳皟鐢?pm_runtime_mark_last_busy() 骞跺湪閫傚綋鏃堕棿璋冨害涓€娆¤嚜鍔ㄦ寕璧凤紝骞惰繑鍥?0

  `int pm_runtime_resume(struct device *dev);`
    - 鎵ц璇ヨ澶囩殑瀛愮郴缁熺骇鎭㈠鍥炶皟锛涙垚鍔熸椂杩斿洖 0锛屽鏋滆澶囩殑杩愯鏃?PM 鐘舵€佸凡缁忔槸 'active'锛堜篃鍖呮嫭 'power.disable_depth' 闈為浂銆佷絾鐘舵€佸湪浠?0 鍙樹负 1 鏃舵槸 'active' 鐨勬儏鍐碉級鍒欒繑鍥?1锛屽け璐ユ椂杩斿洖閿欒鐮侊紝鍏朵腑 -EAGAIN 琛ㄧず灏嗘潵灏濊瘯鎭㈠璇ヨ澶囧彲鑳芥槸瀹夊叏鐨勶紝浣嗚繕搴旈澶栨鏌?'power.runtime_error'锛?EACCES 琛ㄧず鍥犱负 'power.disable_depth' 涓嶅悓浜?0 鑰屾棤娉曡繍琛岃鍥炶皟

  `int pm_runtime_resume_and_get(struct device *dev);`
    - 杩愯 pm_runtime_resume(dev)锛屽鏋滄垚鍔熷垯閫掑璁惧鐨勪娇鐢ㄨ鏁板櫒锛涙垚鍔熸椂杩斿洖 0锛堟棤璁鸿澶囩殑杩愯鏃?PM 鐘舵€佹槸鍚﹀凡缁忔槸 'active'锛夛紝澶辫触鏃惰繑鍥?pm_runtime_resume() 鐨勯敊璇爜銆?

  `int pm_request_idle(struct device *dev);`
    - 鎻愪氦涓€涓墽琛岃璁惧瀛愮郴缁熺骇绌洪棽鍥炶皟鐨勮姹傦紙璇ヨ姹傜敱 pm_wq 涓殑涓€涓伐浣滈」琛ㄧず锛夛紱鎴愬姛鏃惰繑鍥?0锛屽鏋滆姹傛湭琚帓闃熷垯杩斿洖閿欒鐮?

  `int pm_request_autosuspend(struct device *dev);`
    - 璋冪敤 pm_runtime_mark_last_busy()锛屽苟鍦ㄨ嚜鍔ㄦ寕璧峰欢杩熷埌鏈熸椂璋冨害璇ヨ澶囧瓙绯荤粺绾ф寕璧峰洖璋冪殑鎵ц

  `int pm_schedule_suspend(struct device *dev, unsigned int delay);`
    - 灏嗘潵璋冨害璇ヨ澶囧瓙绯荤粺绾ф寕璧峰洖璋冪殑鎵ц锛屽叾涓?'delay' 鏄湪 pm_wq 涓帓闃熸寕璧峰伐浣滈」涔嬪墠绛夊緟鐨勬椂闂达紝浠ユ绉掕锛堝鏋?'delay' 涓洪浂锛屽垯绔嬪嵆鎺掗槦宸ヤ綔椤癸級锛涙垚鍔熸椂杩斿洖 0锛屽鏋滆澶囩殑 PM 杩愯鏃剁姸鎬佸凡缁忔槸 'suspended' 鍒欒繑鍥?1锛屽鏋滆姹傛湭琚皟搴︼紙鎴栬€呭湪 'delay' 涓?0 鏃舵湭琚帓闃燂級鍒欒繑鍥為敊璇爜锛涘鏋?->runtime_suspend() 鐨勬墽琛屽凡缁忚璋冨害涓斿皻鏈埌鏈燂紝鍒?'delay' 鐨勬柊鍊煎皢鐢ㄤ綔绛夊緟鏃堕棿

  `int pm_request_resume(struct device *dev);`
    - 鎻愪氦涓€涓墽琛岃璁惧瀛愮郴缁熺骇鎭㈠鍥炶皟鐨勮姹傦紙璇ヨ姹傜敱 pm_wq 涓殑涓€涓伐浣滈」琛ㄧず锛夛紱鎴愬姛鏃惰繑鍥?0锛屽鏋滆澶囩殑杩愯鏃?PM 鐘舵€佸凡缁忔槸 'active' 鍒欒繑鍥?1锛屽鏋滆姹傛湭琚帓闃熷垯杩斿洖閿欒鐮?

  `void pm_runtime_get_noresume(struct device *dev);`
    - 閫掑璁惧鐨勪娇鐢ㄨ鏁板櫒

  `int pm_runtime_get(struct device *dev);`
    - 閫掑璁惧鐨勪娇鐢ㄨ鏁板櫒锛岃繍琛?pm_request_resume(dev) 骞惰繑鍥炲叾缁撴灉

  `int pm_runtime_get_sync(struct device *dev);`
    - 閫掑璁惧鐨勪娇鐢ㄨ鏁板櫒锛岃繍琛?pm_runtime_resume(dev) 骞惰繑鍥炲叾缁撴灉锛?
      娉ㄦ剰瀹冨湪鍑洪敊鏃朵笉浼氶€掑噺璁惧鐨勪娇鐢ㄨ鏁板櫒锛屽洜姝よ€冭檻浣跨敤 pm_runtime_resume_and_get() 浠ｆ浛瀹冿紝鐗瑰埆鏄湪鍏惰繑鍥炲€艰璋冪敤鑰呮鏌ョ殑鎯呭喌涓嬶紝鍥犱负杩欏緢鍙兘浜х敓鏇存竻鏅扮殑浠ｇ爜銆?

  `int pm_runtime_get_if_in_use(struct device *dev);`
    - 濡傛灉 'power.disable_depth' 闈為浂鍒欒繑鍥?-EINVAL锛涘惁鍒欙紝濡傛灉杩愯鏃?PM 鐘舵€佷负 RPM_ACTIVE 涓旇繍琛屾椂 PM 浣跨敤璁℃暟鍣ㄩ潪闆讹紝鍒欓€掑璁℃暟鍣ㄥ苟杩斿洖 1锛涘惁鍒欏湪涓嶆敼鍙樿鏁板櫒鐨勬儏鍐典笅杩斿洖 0

  `int pm_runtime_get_if_active(struct device *dev);`
    - 濡傛灉 'power.disable_depth' 闈為浂鍒欒繑鍥?-EINVAL锛涘惁鍒欙紝濡傛灉杩愯鏃?PM 鐘舵€佷负 RPM_ACTIVE锛屽垯閫掑璁℃暟鍣ㄥ苟杩斿洖 1锛涘惁鍒欏湪涓嶆敼鍙樿鏁板櫒鐨勬儏鍐典笅杩斿洖 0

  `void pm_runtime_put_noidle(struct device *dev);`
    - 閫掑噺璁惧鐨勪娇鐢ㄨ鏁板櫒

  `int pm_runtime_put(struct device *dev);`
    - 閫掑噺璁惧鐨勪娇鐢ㄨ鏁板櫒锛涘鏋滅粨鏋滀负 0锛屽垯杩愯 pm_request_idle(dev) 骞惰繑鍥炲叾缁撴灉

  `int pm_runtime_put_autosuspend(struct device *dev);`
    - 灏?power.last_busy 瀛楁璁句负褰撳墠鏃堕棿骞堕€掑噺璁惧鐨勪娇鐢ㄨ鏁板櫒锛涘鏋滅粨鏋滀负 0锛屽垯杩愯 pm_request_autosuspend(dev) 骞惰繑鍥炲叾缁撴灉

  `int __pm_runtime_put_autosuspend(struct device *dev);`
    - 閫掑噺璁惧鐨勪娇鐢ㄨ鏁板櫒锛涘鏋滅粨鏋滀负 0锛屽垯杩愯 pm_request_autosuspend(dev) 骞惰繑鍥炲叾缁撴灉

  `int pm_runtime_put_sync(struct device *dev);`
    - 閫掑噺璁惧鐨勪娇鐢ㄨ鏁板櫒锛涘鏋滅粨鏋滀负 0锛屽垯杩愯 pm_runtime_idle(dev) 骞惰繑鍥炲叾缁撴灉

  `int pm_runtime_put_sync_suspend(struct device *dev);`
    - 閫掑噺璁惧鐨勪娇鐢ㄨ鏁板櫒锛涘鏋滅粨鏋滀负 0锛屽垯杩愯 pm_runtime_suspend(dev) 骞惰繑鍥炲叾缁撴灉

  `int pm_runtime_put_sync_autosuspend(struct device *dev);`
    - 灏?power.last_busy 瀛楁璁句负褰撳墠鏃堕棿骞堕€掑噺璁惧鐨勪娇鐢ㄨ鏁板櫒锛涘鏋滅粨鏋滀负 0锛屽垯杩愯 pm_runtime_autosuspend(dev) 骞惰繑鍥炲叾缁撴灉

  `void pm_runtime_enable(struct device *dev);`
    - 閫掑噺璁惧鐨?'power.disable_depth' 瀛楁锛涘鏋滆瀛楁绛変簬闆讹紝鍒欒繍琛屾椂 PM 杈呭姪鍑芥暟鍙互鎵ц绗?2 鑺傛弿杩扮殑瀛愮郴缁熺骇鍥炶皟

  `int pm_runtime_disable(struct device *dev);`
    - 閫掑璁惧鐨?'power.disable_depth' 瀛楁锛堝鏋滆瀛楁涔嬪墠涓洪浂锛岃繖闃叉涓鸿璁惧杩愯瀛愮郴缁熺骇杩愯鏃?PM 鍥炶皟锛夛紝纭繚璁惧涓婃墍鏈夊緟澶勭悊鐨勮繍琛屾椂 PM 鎿嶄綔瑕佷箞宸插畬鎴愯涔堝凡鍙栨秷锛涘鏋滄湁寰呭鐞嗙殑鎭㈠璇锋眰涓旀湁蹇呰鎵ц璇ヨ澶囩殑瀛愮郴缁熺骇鎭㈠鍥炶皟浠ユ弧瓒宠璇锋眰锛屽垯杩斿洖 1锛屽惁鍒欒繑鍥?0

  `void pm_runtime_barrier(struct device *dev);`
    - 妫€鏌ユ槸鍚︽湁寰呭鐞嗙殑鎭㈠璇锋眰锛屽苟鍦ㄨ繖绉嶆儏鍐典笅锛堝悓姝ュ湴锛夋仮澶嶅畠锛屽彇娑堝叧浜庡畠鐨勪换浣曞叾浠栧緟澶勭悊杩愯鏃?PM 璇锋眰锛屽苟绛夊緟鍏朵笂鎵€鏈夋鍦ㄨ繘琛岀殑杩愯鏃?PM 鎿嶄綔瀹屾垚

  `void pm_suspend_ignore_children(struct device *dev, bool enable);`
    - 璁剧疆/娓呴櫎璁惧鐨?power.ignore_children 鏍囧織

  `int pm_runtime_set_active(struct device *dev);`
    - 娓呴櫎璁惧鐨?'power.runtime_error' 鏍囧織锛屽皢璁惧鐨勮繍琛屾椂 PM 鐘舵€佽涓?'active'锛屽苟閫傚綋鍦版洿鏂板叾鐖惰澶囩殑 'active' 瀛愯澶囪鏁板櫒锛堝彧鏈夊湪 'power.runtime_error' 琚疆浣嶆垨 'power.disable_depth' 澶т簬闆舵椂鎵嶅厑璁镐娇鐢ㄦ鍑芥暟锛夛紱濡傛灉璁惧鏈変竴涓埗璁惧涓嶆槸 'active' 涓斿叾 'power.ignore_children' 鏍囧織鏈疆浣嶏紝鍒欏畠灏嗗け璐ュ苟杩斿洖閿欒鐮?

  `void pm_runtime_set_suspended(struct device *dev);`
    - 娓呴櫎璁惧鐨?'power.runtime_error' 鏍囧織锛屽皢璁惧鐨勮繍琛屾椂 PM 鐘舵€佽涓?'suspended'锛屽苟閫傚綋鍦版洿鏂板叾鐖惰澶囩殑 'active' 瀛愯澶囪鏁板櫒锛堝彧鏈夊湪 'power.runtime_error' 琚疆浣嶆垨 'power.disable_depth' 澶т簬闆舵椂鎵嶅厑璁镐娇鐢ㄦ鍑芥暟锛?

  `bool pm_runtime_active(struct device *dev);`
    - 濡傛灉璁惧鐨勮繍琛屾椂 PM 鐘舵€佷负 'active' 鎴栧叾 'power.disable_depth' 瀛楁涓嶇瓑浜庨浂锛屽垯杩斿洖 true锛屽惁鍒欒繑鍥?false

  `bool pm_runtime_suspended(struct device *dev);`
    - 濡傛灉璁惧鐨勮繍琛屾椂 PM 鐘舵€佷负 'suspended' 涓斿叾 'power.disable_depth' 瀛楁绛変簬闆讹紝鍒欒繑鍥?true锛屽惁鍒欒繑鍥?false

  `bool pm_runtime_status_suspended(struct device *dev);`
    - 濡傛灉璁惧鐨勮繍琛屾椂 PM 鐘舵€佷负 'suspended'锛屽垯杩斿洖 true

  `void pm_runtime_no_callbacks(struct device *dev);`
    - 涓鸿澶囪缃?power.no_callbacks 鏍囧織锛屽苟浠?/sys/devices/.../power 绉婚櫎杩愯鏃?PM 灞炴€э紙鎴栧湪璁惧娉ㄥ唽鏃堕樆姝㈠畠浠娣诲姞锛?

  `void pm_runtime_irq_safe(struct device *dev);`
    - 涓鸿澶囪缃?power.irq_safe 鏍囧織锛屼娇寰楄繍琛屾椂 PM 鍥炶皟鍦ㄤ腑鏂鐢ㄧ殑鎯呭喌涓嬭璋冪敤

  `bool pm_runtime_is_irq_safe(struct device *dev);`
    - 濡傛灉涓鸿澶囪缃簡 power.irq_safe 鏍囧織锛堜娇杩愯鏃?PM 鍥炶皟鍦ㄤ腑鏂鐢ㄧ殑鎯呭喌涓嬭璋冪敤锛夛紝鍒欒繑鍥?true

  `void pm_runtime_mark_last_busy(struct device *dev);`
    - 灏?power.last_busy 瀛楁璁句负褰撳墠鏃堕棿

  `void pm_runtime_use_autosuspend(struct device *dev);`
    - 璁剧疆 power.use_autosuspend 鏍囧織锛屽惎鐢ㄨ嚜鍔ㄦ寕璧峰欢杩燂紱濡傛灉璇ユ爣蹇椾箣鍓嶈娓呴櫎涓?power.autosuspend_delay 涓鸿礋锛屽垯璋冪敤 pm_runtime_get_sync

  `void pm_runtime_dont_use_autosuspend(struct device *dev);`
    - 娓呴櫎 power.use_autosuspend 鏍囧織锛岀鐢ㄨ嚜鍔ㄦ寕璧峰欢杩燂紱濡傛灉璇ユ爣蹇椾箣鍓嶈缃綅涓?power.autosuspend_delay 涓鸿礋锛屽垯閫掑噺璁惧鐨勪娇鐢ㄨ鏁板櫒锛涜皟鐢?pm_runtime_idle

  `void pm_runtime_set_autosuspend_delay(struct device *dev, int delay);`
    - 灏?power.autosuspend_delay 鐨勫€艰涓?'delay'锛堜互姣琛ㄧず锛夛紱濡傛灉 'delay' 涓鸿礋鍒欓樆姝㈣繍琛屾椂鎸傝捣锛涘鏋?power.use_autosuspend 琚疆浣嶏紝鍒欐牴鎹?power.autosuspend_delay 鏄惁琚敼涓鸿礋鍊兼垨鏀逛负绂诲紑璐熷€硷紝璋冪敤 pm_runtime_get_sync 鎴栭€掑噺璁惧鐨勪娇鐢ㄨ鏁板櫒骞惰皟鐢?pm_runtime_idle锛涘鏋?power.use_autosuspend 琚竻闄わ紝鍒欒皟鐢?pm_runtime_idle

  `unsigned long pm_runtime_autosuspend_expiration(struct device *dev);`
    - 鍩轰簬 power.last_busy 鍜?power.autosuspend_delay 璁＄畻褰撳墠鑷姩鎸傝捣寤惰繜鏃舵鍒版湡鐨勬椂闂达紱濡傛灉寤惰繜鏃堕棿涓?1000 ms 鎴栨洿澶э紝鍒欏埌鏈熸椂闂村悜涓婂彇鏁村埌鏈€杩戠殑绉掞紱濡傛灉寤惰繜鏃舵宸茬粡鍒版湡鎴?power.use_autosuspend 鏈璁剧疆锛屽垯杩斿洖 0锛屽惁鍒欎互 jiffies 杩斿洖鍒版湡鏃堕棿

鍙互鍦ㄤ腑鏂笂涓嬫枃涓畨鍏ㄦ墽琛屼互涓嬭緟鍔╁嚱鏁帮細

- pm_request_idle()
- pm_request_autosuspend()
- pm_schedule_suspend()
- pm_request_resume()
- pm_runtime_get_noresume()
- pm_runtime_get()
- pm_runtime_put_noidle()
- pm_runtime_put()
- pm_runtime_put_autosuspend()
- __pm_runtime_put_autosuspend()
- pm_runtime_enable()
- pm_suspend_ignore_children()
- pm_runtime_set_active()
- pm_runtime_set_suspended()
- pm_runtime_suspended()
- pm_runtime_mark_last_busy()
- pm_runtime_autosuspend_expiration()

濡傛灉宸茬粡涓鸿澶囪皟鐢ㄤ簡 pm_runtime_irq_safe()锛屽垯浠ヤ笅杈呭姪鍑芥暟涔熷彲浠ュ湪涓柇涓婁笅鏂囦腑浣跨敤锛?

- pm_runtime_idle()
- pm_runtime_suspend()
- pm_runtime_autosuspend()
- pm_runtime_resume()
- pm_runtime_get_sync()
- pm_runtime_put_sync()
- pm_runtime_put_sync_suspend()
- pm_runtime_put_sync_autosuspend()

## 5. 杩愯鏃?PM 鍒濆鍖栥€佽澶囨帰娴嬩笌绉婚櫎


鏈€鍒濓紝鎵€鏈夎澶囩殑杩愯鏃?PM 閮芥槸绂佺敤鐨勶紝杩欐剰鍛崇潃鍦ㄧ 4 鑺傛弿杩扮殑澶у鏁拌繍琛屾椂 PM 杈呭姪鍑芥暟鍦ㄤ负璁惧璋冪敤 pm_runtime_enable() 涔嬪墠閮藉皢杩斿洖 -EAGAIN銆?

闄ゆ涔嬪锛屾墍鏈夎澶囩殑鍒濆杩愯鏃?PM 鐘舵€佷负 'suspended'锛屼絾杩欐湭蹇呭弽鏄犺澶囩殑瀹為檯鐗╃悊鐘舵€併€傚洜姝わ紝濡傛灉璁惧鍒濆鏄椿璺冪殑锛堝嵆瀹冭兘澶熷鐞?I/O锛夛紝鍒欏湪鍏惰皟鐢?pm_runtime_enable() 涔嬪墠锛屽繀椤诲€熷姪 pm_runtime_set_active() 灏嗗叾杩愯鏃?PM 鐘舵€佹敼涓?'active'銆?

鐒惰€岋紝濡傛灉璁惧鏈夌埗璁惧涓旂埗璁惧鐨勮繍琛屾椂 PM 鏄惎鐢ㄧ殑锛屽垯涓鸿澶囪皟鐢?pm_runtime_set_active() 浼氬奖鍝嶇埗璁惧锛岄櫎闈炵埗璁惧鐨?'power.ignore_children' 鏍囧織琚疆浣嶃€傚嵆鍦ㄩ偅绉嶆儏鍐典笅锛屽彧瑕佸瓙璁惧鐨勭姸鎬佹槸 'active'锛屽嵆浣垮瓙璁惧鐨勮繍琛屾椂 PM 浠嶈绂佺敤锛堝嵆灏氭湭涓哄瓙璁惧璋冪敤 pm_runtime_enable() 鎴栧凡涓哄叾璋冪敤 pm_runtime_disable()锛夛紝鐖惰澶囦篃鏃犳硶鍦ㄨ繍琛屾椂鎸傝捣锛堜娇鐢?PM core 鐨勮緟鍔╁嚱鏁帮級銆傚嚭浜庤繖涓師鍥狅紝涓€鏃︿负璁惧璋冪敤浜?pm_runtime_set_active()锛屽氨搴斿敖蹇悎鐞嗗湴涓哄叾璋冪敤 pm_runtime_enable()锛屾垨鑰呭€熷姪 pm_runtime_set_suspended() 灏嗗叾杩愯鏃?PM 鐘舵€佹敼鍥?'suspended'銆?

濡傛灉璁惧鐨勯粯璁ゅ垵濮嬭繍琛屾椂 PM 鐘舵€侊紙鍗?'suspended'锛夊弽鏄犱簡璁惧鐨勫疄闄呯姸鎬侊紝鍏舵€荤嚎绫诲瀷鎴栭┍鍔ㄧ殑 ->probe() 鍥炶皟寰堝彲鑳介渶瑕佷娇鐢ㄧ 4 鑺傛弿杩扮殑 PM core 鐨勬煇涓緟鍔╁嚱鏁版潵鍞ら啋瀹冦€傚湪閭ｇ鎯呭喌涓嬶紝搴斾娇鐢?pm_runtime_resume()銆傚綋鐒讹紝涓烘鐩殑锛岃澶囩殑杩愯鏃?PM 蹇呴』鏇存棭閫氳繃璋冪敤 pm_runtime_enable() 鏉ュ惎鐢ㄣ€?

娉ㄦ剰锛屽鏋滆澶囧彲鑳藉湪鎺㈡祴锛坧robe锛夋湡闂存墽琛?pm_runtime 璋冪敤锛堜緥濡傚鏋滃畠娉ㄥ唽鍒颁竴涓彲鑳戒細鍥炶皟鐨勫瓙绯荤粺锛夛紝閭ｄ箞鎴愬浣跨敤 pm_runtime_get_sync() 涓?pm_runtime_put() 璋冪敤鏄悎閫傜殑锛屼互纭繚璁惧鍦ㄦ帰娴嬫湡闂翠笉浼氳鏀惧洖鐫＄湢銆傝繖鍙兘鍙戠敓鍦ㄨ濡傜綉缁滆澶囧眰杩欐牱鐨勭郴缁熶腑銆?

鍦?->probe() 瀹屾垚鍚庢寕璧疯澶囧彲鑳芥槸鍙彇鐨勩€傚洜姝ら┍鍔ㄦ牳蹇冧娇鐢ㄥ紓姝ョ殑 pm_request_idle() 鏉ユ彁浜や竴涓湪璇ユ椂鍒绘墽琛岃澶囧瓙绯荤粺绾х┖闂插洖璋冪殑璇锋眰銆傚埄鐢ㄤ簡杩愯鏃惰嚜鍔ㄦ寕璧风壒鎬х殑椹卞姩鍙兘鎯冲湪浠?->probe() 杩斿洖涔嬪墠鏇存柊鏈€鍚庣殑 busy 鏍囪銆?

姝ゅ锛岄┍鍔ㄦ牳蹇冮槻姝㈣繍琛屾椂 PM 鍥炶皟涓?__device_release_driver() 涓殑鎬荤嚎閫氱煡锛坣otifier锛夊洖璋冪珵浜夛紝杩欐槸蹇呰鐨勶紝鍥犱负鏌愪簺瀛愮郴缁熶娇鐢ㄨ閫氱煡鏉ユ墽琛屽奖鍝嶈繍琛屾椂 PM 鍔熻兘鐨勬搷浣溿€傚畠閫氳繃鍦?driver_sysfs_remove() 鍜?BUS_NOTIFY_UNBIND_DRIVER 閫氱煡涔嬪墠璋冪敤 pm_runtime_get_sync() 鏉ュ疄鐜般€傝繖浼氬湪璁惧澶勪簬鎸傝捣鐘舵€佹椂鎭㈠瀹冿紝骞堕槻姝㈠湪杩欎簺渚嬬▼鎵ц鏈熼棿瀹冭鍐嶆鎸傝捣銆?

涓轰簡鍏佽鎬荤嚎绫诲瀷鍜岄┍鍔ㄩ€氳繃浠庡畠浠殑 ->remove() 渚嬬▼璋冪敤 pm_runtime_suspend() 灏嗚澶囩疆浜庢寕璧风姸鎬侊紝椹卞姩鏍稿績鍦?__device_release_driver() 涓繍琛?BUS_NOTIFY_UNBIND_DRIVER 閫氱煡涔嬪悗鎵ц pm_runtime_put_sync()銆傝繖瑕佹眰鎬荤嚎绫诲瀷鍜岄┍鍔ㄨ瀹冧滑鐨?->remove() 鍥炶皟鐩存帴閬垮厤涓庤繍琛屾椂 PM 绔炰簤锛屼絾瀹冧篃鍏佽鍦ㄧЩ闄ら┍鍔ㄦ湡闂存洿鐏垫椿鍦板鐞嗚澶囥€?

椹卞姩鍦?->remove() 鍥炶皟涓簲鎾ら攢鍦?->probe() 涓仛鐨勮繍琛屾椂 PM 鏇存敼銆傞€氬父杩欐剰鍛崇潃璋冪敤 pm_runtime_disable()銆乸m_runtime_dont_use_autosuspend() 绛夈€?

鐢ㄦ埛绌洪棿鍙互閫氳繃灏嗚澶?/sys/devices/.../power/control 灞炴€х殑鍊兼敼涓?"on" 鏉ユ湁鏁堢姝㈣澶囩殑椹卞姩鍦ㄨ繍琛屾椂瀵瑰叾杩涜鐢垫簮绠＄悊锛岃繖浼氬鑷磋皟鐢?pm_runtime_forbid()銆傚師鍒欎笂锛岄┍鍔ㄤ篃鍙互鍒╃敤姝ゆ満鍒跺湪鐢ㄦ埛绌洪棿鎵撳紑瀹冧箣鍓嶆湁鏁堝叧闂澶囩殑杩愯鏃剁數婧愮鐞嗐€傚嵆锛屽湪鍒濆鍖栨湡闂撮┍鍔ㄥ彲浠ョ‘淇濊澶囩殑杩愯鏃?PM 鐘舵€佷负 'active' 骞惰皟鐢?pm_runtime_forbid()銆備笉杩囧簲褰撴敞鎰忥紝濡傛灉鐢ㄦ埛绌洪棿宸茬粡鏈夋剰灏?/sys/devices/.../power/control 鐨勫€兼敼涓?"auto" 浠ュ厑璁搁┍鍔ㄥ湪杩愯鏃跺璁惧杩涜鐢垫簮绠＄悊锛岄┍鍔ㄤ互杩欑鏂瑰紡浣跨敤 pm_runtime_forbid() 鍙兘浼氳鐢ㄦ埛绌洪棿鍥版儜銆?

## 6. 杩愯鏃?PM 涓庣郴缁熺潯鐪?


杩愯鏃?PM 涓庣郴缁熺潯鐪狅紙鍗崇郴缁熸寕璧峰拰浼戠湢锛屼篃绉颁负鎸傝捣鍒?RAM 鍜屾寕璧峰埌纾佺洏锛変互鍑犵鏂瑰紡鐩镐簰浜や簰銆傚鏋滅郴缁熺潯鐪犲紑濮嬫椂璁惧鏄椿璺冪殑锛屽垯涓€鍒囬兘寰堢洿鎺ャ€備絾濡傛灉璁惧宸茬粡琚寕璧凤紝浼氬彂鐢熶粈涔堝憿锛?

璁惧瀵硅繍琛屾椂 PM 鍜岀郴缁熺潯鐪犲彲鑳芥湁涓嶅悓鐨勫敜閱掕缃€備緥濡傦紝杩滅▼鍞ら啋鍙兘瀵硅繍琛屾椂鎸傝捣鍚敤锛屼絾瀵圭郴缁熺潯鐪犵姝紙device_may_wakeup(dev) 杩斿洖 'false'锛夈€傚綋杩欑鎯呭喌鍙戠敓鏃讹紝瀛愮郴缁熺骇绯荤粺鎸傝捣鍥炶皟璐熻矗鏀瑰彉璁惧鐨勫敜閱掕缃紙瀹冨彲浠ユ妸杩欎欢浜嬬暀缁欒澶囬┍鍔ㄧ殑绯荤粺鎸傝捣渚嬬▼锛夈€備负姝ゅ彲鑳芥湁蹇呰鍏堟仮澶嶈澶囧啀灏嗗叾鎸傝捣銆傚鏋滈┍鍔ㄥ杩愯鏃舵寕璧峰拰绯荤粺鐫＄湢浣跨敤涓嶅悓鐨勭數婧愮骇鍒垨鍏朵粬璁剧疆锛屾儏鍐典篃鏄姝ゃ€?

鍦ㄧ郴缁熸仮澶嶆湡闂达紝鏈€绠€鍗曠殑鏂规硶鏄妸鎵€鏈夎澶囬兘鎭㈠鍒板叏鍔熺巼锛屽嵆浣垮畠浠湪绯荤粺鐫＄湢寮€濮嬩箣鍓嶅氨宸茬粡琚寕璧枫€傝繖鏍峰仛鏈夊嚑涓師鍥狅紝鍖呮嫭锛?

  - 璁惧鍙兘闇€瑕佸垏鎹㈢數婧愮骇鍒€佸敜閱掕缃瓑銆?

  - 鍥轰欢鍙兘涓㈠け浜嗚繙绋嬪敜閱掍簨浠躲€?

  - 璁惧鐨勫瓙璁惧鍙兘闇€瑕佽澶囧浜庡叏鍔熺巼鎵嶈兘鎭㈠瀹冧滑鑷繁銆?

  - 椹卞姩瀵硅澶囩姸鎬佺殑璁ょ煡鍙兘涓庤澶囩殑鐗╃悊鐘舵€佷笉涓€鑷淬€傝繖鍦ㄤ粠浼戠湢鎭㈠鏈熼棿鍙兘鍙戠敓銆?

  - 璁惧鍙兘闇€瑕佽閲嶇疆銆?

  - 鍗充娇璁惧宸茶鎸傝捣锛屽鏋滃叾浣跨敤璁℃暟鍣?> 0锛岄偅涔堝緢鍙兘涓嶄箙涔嬪悗瀹冧篃闇€瑕佷竴娆¤繍琛屾椂鎭㈠銆?

濡傛灉璁惧鍦ㄧ郴缁熺潯鐪犲紑濮嬩箣鍓嶅凡琚寕璧凤紝骞跺湪鎭㈠鏈熼棿琚仮澶嶅埌鍏ㄥ姛鐜囷紝閭ｄ箞瀹冪殑杩愯鏃?PM 鐘舵€佸皢蹇呴』鏇存柊浠ュ弽鏄犵郴缁熺潯鐪犲悗鐨勫疄闄呯姸鎬併€傚仛娉曟槸锛?

  - pm_runtime_disable(dev);
  - pm_runtime_set_active(dev);
  - pm_runtime_enable(dev);

PM core 鎬绘槸鍦ㄨ皟鐢?->suspend() 鍥炶皟涔嬪墠閫掑杩愯鏃朵娇鐢ㄨ鏁板櫒锛屽苟鍦ㄨ皟鐢?->resume() 鍥炶皟涔嬪悗閫掑噺瀹冦€傚洜姝ゅ儚杩欐牱涓存椂绂佺敤杩愯鏃?PM 涓嶄細瀵艰嚧浠讳綍杩愯鏃舵寕璧峰皾璇曡姘镐箙涓㈠け銆傚鏋滀娇鐢ㄨ鏁板湪 ->resume() 鍥炶皟杩斿洖鍚庡彉涓洪浂锛?>runtime_idle() 鍥炶皟灏嗙収甯歌璋冪敤銆?

鐒惰€岋紝鍦ㄦ煇浜涚郴缁熶笂锛岀郴缁熺潯鐪犱笉鏄€氳繃鍏ㄥ眬鍥轰欢鎴栫‖浠舵搷浣滆繘鍏ョ殑銆傜浉鍙嶏紝鎵€鏈夌‖浠剁粍浠堕兘鐢卞唴鏍镐互鍗忚皟鐨勬柟寮忕洿鎺ョ疆浜庝綆鍔熻€楃姸鎬併€傜劧鍚庯紝绯荤粺鐫＄湢鐘舵€佸疄闄呬笂婧愪簬纭欢缁勪欢鏈€缁堟墍澶勭殑鐘舵€侊紝骞朵笖绯荤粺浠庤鐘舵€佽纭欢涓柇鎴栫被浼兼満鍒讹紙瀹屽叏澶勪簬鍐呮牳鎺у埗涔嬩笅锛夊敜閱掋€傜粨鏋滐紝鍐呮牳浠庝笉浜ゅ嚭鎺у埗鏉冿紝骞朵笖鎭㈠鏈熼棿鎵€鏈夎澶囩殑鐘舵€佸畠閮界簿纭煡鏅撱€傚鏋滄槸杩欑鎯呭喌锛屽苟涓斾笂闈㈠垪鍑虹殑鎯呭舰閮戒笉鍙戠敓锛堢壒鍒槸锛屽鏋滅郴缁熶笉鏄粠浼戠湢鍞ら啋锛夛紝閭ｄ箞鎶婂湪绯荤粺鐫＄湢寮€濮嬩箣鍓嶅凡琚寕璧风殑璁惧鐣欏湪鎸傝捣鐘舵€佷腑鍙兘鏇撮珮鏁堛€?

涓烘锛孭M core 鎻愪緵浜嗕竴绉嶆満鍒讹紝鍏佽璁惧灞傛缁撴瀯鐨勪笉鍚屽眰绾т箣闂磋繘琛屾煇绉嶅崗璋冦€傚嵆锛屽鏋滅郴缁熸寕璧风殑 .prepare() 鍥炶皟涓烘煇璁惧杩斿洖涓€涓鏁帮紝杩欒〃绀哄悜 PM core 琛ㄦ槑璇ヨ澶囩湅涓婂幓澶勪簬杩愯鏃舵寕璧风姸鎬佷笖鍏剁姸鎬佽壇濂斤紝鍥犳鍙瀹冪殑鎵€鏈夊悗浠ｄ篃鐣欏湪杩愯鏃舵寕璧风姸鎬侊紝灏卞彲浠ヨ瀹冪暀鍦ㄨ繍琛屾椂鎸傝捣銆傚鏋滃彂鐢熻繖绉嶆儏鍐碉紝PM core 灏嗕笉浼氫负鎵€鏈夎繖浜涜澶囨墽琛屼换浣曠郴缁熸寕璧峰拰鎭㈠鍥炶皟锛岄櫎浜?.complete() 鍥炶皟锛屽畠闅忓悗瀹屽叏璐熻矗浠ラ€傚綋鐨勬柟寮忓鐞嗚璁惧銆傝繖浠呴€傜敤浜庝笌浼戠湢鏃犲叧鐨勭郴缁熸寕璧疯浆鎹紙鏇村淇℃伅鍙傝 Documentation/driver-api/pm/devices.rst锛夈€?

PM core 閫氳繃鎵ц浠ヤ笅鎿嶄綔锛屽敖鏈€澶у姫鍔涢檷浣庤繍琛屾椂 PM 涓庣郴缁熸寕璧?鎭㈠锛堜互鍙婁紤鐪狅級鍥炶皟涔嬮棿绔炰簤鏉′欢鐨勫彲鑳芥€э細

  - 鍦ㄧ郴缁熸寕璧锋湡闂达紝姝ｅソ鍦ㄦ墽琛屾煇璁惧鐨勫瓙绯荤粺绾?.prepare() 鍥炶皟涔嬪墠锛屽鍏惰皟鐢?pm_runtime_get_noresume()锛屽苟涓旀濂藉湪鎵ц鍏跺瓙绯荤粺绾?.suspend() 鍥炶皟涔嬪墠锛屽鍏惰皟鐢?pm_runtime_barrier()銆傞櫎姝や箣澶栵紝PM core 姝ｅソ鍦ㄦ墽琛屽叾瀛愮郴缁熺骇 .suspend_late() 鍥炶皟涔嬪墠锛屼负姣忓彴璁惧绂佺敤杩愯鏃?PM銆?

  - 鍦ㄧ郴缁熸仮澶嶆湡闂达紝姝ｅソ鍦ㄦ墽琛屽叾瀛愮郴缁熺骇 .resume_early() 鍥炶皟涔嬪悗锛屽鍏惰皟鐢?pm_runtime_enable()锛屽苟姝ｅソ鍦ㄦ墽琛屽叾瀛愮郴缁熺骇 .complete() 鍥炶皟涔嬪悗锛屽鍏惰皟鐢?pm_runtime_put()銆?

## 7. 閫氱敤瀛愮郴缁熷洖璋?


瀛愮郴缁熷彲鑳藉笇鏈涢€氳繃浣跨敤 PM core 鎻愪緵鐨勪竴缁勯€氱敤鐢垫簮绠＄悊鍥炶皟鏉ヨ妭鐪佷唬鐮佺┖闂达紝杩欎簺鍥炶皟瀹氫箟鍦?driver/base/power/generic_ops.c 涓細

  `int pm_generic_runtime_suspend(struct device *dev);`
    - 璋冪敤姝よ澶囩殑椹卞姩鎻愪緵鐨?->runtime_suspend() 鍥炶皟骞惰繑鍥炲叾缁撴灉锛屽鏋滄湭瀹氫箟鍒欒繑鍥?0

  `int pm_generic_runtime_resume(struct device *dev);`
    - 璋冪敤姝よ澶囩殑椹卞姩鎻愪緵鐨?->runtime_resume() 鍥炶皟骞惰繑鍥炲叾缁撴灉锛屽鏋滄湭瀹氫箟鍒欒繑鍥?0

  `int pm_generic_suspend(struct device *dev);`
    - 濡傛灉璁惧灏氭湭鍦ㄨ繍琛屾椂琚寕璧凤紝璋冪敤鍏堕┍鍔ㄦ彁渚涚殑 ->suspend() 鍥炶皟骞惰繑鍥炲叾缁撴灉锛屽鏋滄湭瀹氫箟鍒欒繑鍥?0

  `int pm_generic_suspend_noirq(struct device *dev);`
    - 濡傛灉 pm_runtime_suspended(dev) 杩斿洖 "false"锛岃皟鐢ㄨ澶囬┍鍔ㄦ彁渚涚殑 ->suspend_noirq() 鍥炶皟骞惰繑鍥炲叾缁撴灉锛屽鏋滄湭瀹氫箟鍒欒繑鍥?0

  `int pm_generic_resume(struct device *dev);`
    - 璋冪敤姝よ澶囩殑椹卞姩鎻愪緵鐨?->resume() 鍥炶皟锛屽鏋滄垚鍔燂紝鍒欏皢璁惧鐨勮繍琛屾椂 PM 鐘舵€佹敼涓?'active'

  `int pm_generic_resume_noirq(struct device *dev);`
    - 璋冪敤姝よ澶囩殑椹卞姩鎻愪緵鐨?->resume_noirq() 鍥炶皟

  `int pm_generic_freeze(struct device *dev);`
    - 濡傛灉璁惧灏氭湭鍦ㄨ繍琛屾椂琚寕璧凤紝璋冪敤鍏堕┍鍔ㄦ彁渚涚殑 ->freeze() 鍥炶皟骞惰繑鍥炲叾缁撴灉锛屽鏋滄湭瀹氫箟鍒欒繑鍥?0

  `int pm_generic_freeze_noirq(struct device *dev);`
    - 濡傛灉 pm_runtime_suspended(dev) 杩斿洖 "false"锛岃皟鐢ㄨ澶囬┍鍔ㄦ彁渚涚殑 ->freeze_noirq() 鍥炶皟骞惰繑鍥炲叾缁撴灉锛屽鏋滄湭瀹氫箟鍒欒繑鍥?0

  `int pm_generic_thaw(struct device *dev);`
    - 濡傛灉璁惧灏氭湭鍦ㄨ繍琛屾椂琚寕璧凤紝璋冪敤鍏堕┍鍔ㄦ彁渚涚殑 ->thaw() 鍥炶皟骞惰繑鍥炲叾缁撴灉锛屽鏋滄湭瀹氫箟鍒欒繑鍥?0

  `int pm_generic_thaw_noirq(struct device *dev);`
    - 濡傛灉 pm_runtime_suspended(dev) 杩斿洖 "false"锛岃皟鐢ㄨ澶囬┍鍔ㄦ彁渚涚殑 ->thaw_noirq() 鍥炶皟骞惰繑鍥炲叾缁撴灉锛屽鏋滄湭瀹氫箟鍒欒繑鍥?0

  `int pm_generic_poweroff(struct device *dev);`
    - 濡傛灉璁惧灏氭湭鍦ㄨ繍琛屾椂琚寕璧凤紝璋冪敤鍏堕┍鍔ㄦ彁渚涚殑 ->poweroff() 鍥炶皟骞惰繑鍥炲叾缁撴灉锛屽鏋滄湭瀹氫箟鍒欒繑鍥?0

  `int pm_generic_poweroff_noirq(struct device *dev);`
    - 濡傛灉 pm_runtime_suspended(dev) 杩斿洖 "false"锛岃繍琛岃澶囬┍鍔ㄦ彁渚涚殑 ->poweroff_noirq() 鍥炶皟骞惰繑鍥炲叾缁撴灉锛屽鏋滄湭瀹氫箟鍒欒繑鍥?0

  `int pm_generic_restore(struct device *dev);`
    - 璋冪敤姝よ澶囩殑椹卞姩鎻愪緵鐨?->restore() 鍥炶皟锛屽鏋滄垚鍔燂紝鍒欏皢璁惧鐨勮繍琛屾椂 PM 鐘舵€佹敼涓?'active'

  `int pm_generic_restore_noirq(struct device *dev);`
    - 璋冪敤姝よ澶囩殑椹卞姩鎻愪緵鐨?->restore_noirq() 鍥炶皟

杩欎簺鍑芥暟鏄?PM core 鍦ㄦ湭鎻愪緵鑷繁鐨勫洖璋冩椂浣跨敤榛樿鍊硷紝鐢ㄤ簬瀛愮郴缁熺骇 dev_pm_ops 缁撴瀯涓殑 ->runtime_idle()銆?>runtime_suspend()銆?>runtime_resume()銆?>suspend()銆?>suspend_noirq()銆?>resume()銆?>resume_noirq()銆?>freeze()銆?>freeze_noirq()銆?>thaw()銆?>thaw_noirq()銆?>poweroff()銆?>poweroff_noirq()銆?>restore()銆?>restore_noirq()銆?

甯屾湜浣跨敤鍚屼竴涓嚱鏁颁綔涓虹郴缁熸寕璧枫€佸喕缁撱€佹柇鐢靛拰杩愯鏃舵寕璧峰洖璋冿紝绫讳技鍦扮敤浜庣郴缁熸仮澶嶃€佽В鍐汇€佹仮澶嶅拰杩愯鏃舵仮澶嶇殑鐨勮澶囬┍鍔紝鍙互鍊熷姪 include/linux/pm_runtime.h 涓畾涔夌殑 DEFINE_RUNTIME_DEV_PM_OPS()锛堝彲鑳藉皢鍏舵渶鍚庝竴涓弬鏁拌涓?NULL锛夋潵瀹炵幇绫讳技鐨勮涓恒€?

## 8. "鏃犲洖璋?璁惧


鏌愪簺"璁惧"鍙槸鍏剁埗璁惧鐨勯€昏緫瀛愯澶囷紝鏃犳硶鑷杩涜鐢垫簮绠＄悊銆傦紙鍏稿瀷鐨勪緥瀛愭槸 USB 鎺ュ彛銆傛暣涓?USB 璁惧鍙互杩涘叆浣庡姛鑰楁ā寮忔垨鍙戦€佸敜閱掕姹傦紝浣嗗浜庡崟涓帴鍙ｈ繖涓よ€呴兘涓嶅彲鑳姐€傦級杩欎簺璁惧鐨勯┍鍔ㄤ笉闇€瑕佽繍琛屾椂 PM 鍥炶皟锛涘鏋滃洖璋冨瓨鍦紝->runtime_suspend() 鍜?->runtime_resume() 灏嗘€绘槸杩斿洖 0 鑰屼笉鍋氫换浣曞叾浠栦簨鎯咃紝->runtime_idle() 灏嗘€绘槸璋冪敤 pm_runtime_suspend()銆?

瀛愮郴缁熷彲浠ラ€氳繃璋冪敤 pm_runtime_no_callbacks() 鏉ュ憡鐭?PM core 杩欎簺璁惧銆傝繖搴旇鍦ㄨ澶囩粨鏋勮鍒濆鍖栦箣鍚庛€佹敞鍐屼箣鍓嶅畬鎴愶紙灏界鍦ㄨ澶囨敞鍐屼箣鍚庡仛涔熷彲浠ワ級銆傝渚嬬▼灏嗚缃澶囩殑 power.no_callbacks 鏍囧織锛屽苟闃绘鍒涘缓闈炶皟璇曠敤鐨勮繍琛屾椂 PM sysfs 灞炴€с€?

褰?power.no_callbacks 琚疆浣嶆椂锛孭M core 灏嗕笉浼氳皟鐢?->runtime_idle()銆?>runtime_suspend() 鎴?->runtime_resume() 鍥炶皟銆傜浉鍙嶏紝瀹冨皢鍋囧畾鎸傝捣鍜屾仮澶嶆€绘槸鎴愬姛锛屽苟涓旂┖闂茶澶囧簲褰撹鎸傝捣銆?

鍥犳锛孭M core 灏嗘案杩滀笉浼氱洿鎺ラ€氱煡璁惧鐨勫瓙绯荤粺鎴栭┍鍔ㄦ湁鍏宠繍琛屾椂鐢垫簮鐨勫彉鍖栥€傜浉鍙嶏紝璁惧鐨勭埗璁惧鐨勯┍鍔ㄥ繀椤昏礋璐ｅ湪鐖惰澶囩殑鐢垫簮鐘舵€佹敼鍙樻椂閫氱煡璁惧鐨勯┍鍔ㄣ€?

娉ㄦ剰锛屽湪鏌愪簺鎯呭喌涓嬶紝瀛愮郴缁?椹卞姩鍙兘涓嶅笇鏈涗负鍏惰澶囪皟鐢?pm_runtime_no_callbacks()銆傝繖鍙兘鏄洜涓洪渶瑕佸疄鐜拌繍琛屾椂 PM 鍥炶皟鐨勪竴涓瓙闆嗐€佷竴涓钩鍙扮浉鍏崇殑 PM 鍩熷彲鑳介檮鍔犲埌璇ヨ澶囷紝鎴栬€呰璁惧鏄€氳繃渚涘簲鑰呰澶囬摼鎺ワ紙supplier device link锛夎繘琛岀數婧愮鐞嗐€傚嚭浜庤繖浜涘師鍥狅紝骞朵负浜嗛伩鍏嶅瓙绯荤粺/椹卞姩涓殑鏍锋澘浠ｇ爜锛孭M core 鍏佽杩愯鏃?PM 鍥炶皟涓嶈璧嬪€笺€傛洿鍑嗙‘鍦拌锛屽鏋滄煇涓洖璋冩寚閽堜负 NULL锛孭M core 灏嗚〃鐜板緱濂藉儚瀛樺湪涓€涓洖璋冨苟涓斿畠杩斿洖浜?0銆?

## 9. 鑷姩鎸傝捣锛屾垨鑷姩寤惰繜鐨勬寕璧?


鏀瑰彉璁惧鐨勭數婧愮姸鎬佸苟闈炴病鏈変唬浠凤紱瀹冮渶瑕佹椂闂村拰鑳介噺銆傚彧鏈夊綋鏈夌悊鐢辫涓鸿澶囧皢鍦ㄨ鐘舵€佸仠鐣欑浉褰撻暱涓€娈垫椂闂存椂锛屾墠搴斿皢鍏剁疆浜庝綆鍔熻€楃姸鎬併€備竴绉嶅父瑙佺殑鍚彂寮忔柟娉曡涓猴紝涓€娈垫椂闂村唴鏈浣跨敤鐨勮澶囧緢鍙兘淇濇寔鏈娇鐢紱閬靛惊杩欎竴寤鸿锛岄┍鍔ㄤ笉搴斿厑璁歌澶囧湪杩愯鏃惰鎸傝捣锛岀洿鍒板畠浠凡缁忛潪娲诲姩浜嗘煇涓渶鐭椂闂存銆傚嵆浣胯鍚彂寮忔柟娉曟渶缁堜笉鏄渶浼樼殑锛屽畠浠嶇劧鑳介槻姝㈣澶囧湪浣庡姛鑰楀拰鍏ㄥ姛鐜囩姸鎬佷箣闂?寮硅烦"寰楀お蹇€?

鏈"autosuspend"锛堣嚜鍔ㄦ寕璧凤級鏄竴涓巻鍙查仐鐣欑墿銆傚畠骞朵笉鎰忓懗鐫€璁惧琚嚜鍔ㄦ寕璧凤紙瀛愮郴缁熸垨椹卞姩浠嶇劧蹇呴』璋冪敤閫傚綋鐨?PM 渚嬬▼锛夛紱鑰屾槸鎰忓懗鐫€杩愯鏃舵寕璧峰皢鑷姩寤惰繜锛岀洿鍒版湡鏈涚殑闈炴椿鍔ㄦ椂娈靛凡缁忚繃鍘汇€?

闈炴椿鍔ㄦ槸鏍规嵁 power.last_busy 瀛楁纭畾鐨勩€傛湡鏈涚殑闈炴椿鍔ㄦ椂娈甸暱搴︽槸涓€涓瓥鐣ラ棶棰樸€傚瓙绯荤粺鍙互閫氳繃璋冪敤 pm_runtime_set_autosuspend_delay() 鍒濆璁剧疆杩欎釜闀垮害锛屼絾鍦ㄨ澶囨敞鍐屼箣鍚庯紝璇ラ暱搴﹀簲鐢辩敤鎴风┖闂翠娇鐢?/sys/devices/.../power/autosuspend_delay_ms 灞炴€ф潵鎺у埗銆?

涓轰簡浣跨敤鑷姩鎸傝捣锛屽瓙绯荤粺鎴栭┍鍔ㄥ繀椤昏皟鐢?pm_runtime_use_autosuspend()锛堟渶濂藉湪娉ㄥ唽璁惧涔嬪墠锛夛紝姝ゅ悗瀹冧滑搴斿綋浣跨敤鍚勭 `*_autosuspend()` 杈呭姪鍑芥暟

```
	Instead of: pm_runtime_suspend    use: pm_runtime_autosuspend;
	Instead of: pm_schedule_suspend   use: pm_request_autosuspend;
	Instead of: pm_runtime_put        use: pm_runtime_put_autosuspend;
	Instead of: pm_runtime_put_sync   use: pm_runtime_put_sync_autosuspend.

```
椹卞姩涔熷彲浠ョ户缁娇鐢ㄩ潪鑷姩鎸傝捣鐨勮緟鍔╁嚱鏁帮紱瀹冧滑鐨勮涓哄皢姝ｅ父锛岃繖鎰忓懗鐫€鏈夋椂浼氳€冭檻鑷姩鎸傝捣寤惰繜锛堣 pm_runtime_idle锛夈€傝繖浜涘嚱鏁扮殑鑷姩鎸傝捣鍙樹綋涔熶細璋冪敤 pm_runtime_mark_last_busy()銆?

鍦ㄦ煇浜涙儏鍐典笅锛岄┍鍔ㄦ垨瀛愮郴缁熷彲鑳芥兂闃绘璁惧绔嬪嵆鑷姩鎸傝捣锛屽嵆浣夸娇鐢ㄨ鏁板櫒涓洪浂涓旇嚜鍔ㄦ寕璧峰欢杩熸椂闂村凡缁忓埌鏈熴€傚鏋?->runtime_suspend() 鍥炶皟杩斿洖 -EAGAIN 鎴?-EBUSY锛屽苟涓斾笅涓€娆¤嚜鍔ㄦ寕璧峰欢杩熷埌鏈熸椂闂村湪鏈潵锛堝氨鍍忓洖璋冭皟鐢ㄤ簡 pm_runtime_mark_last_busy() 鏃堕€氬父閭ｆ牱锛夛紝PM core 灏嗚嚜鍔ㄩ噸鏂拌皟搴﹁嚜鍔ㄦ寕璧枫€?>runtime_suspend() 鍥炶皟鑷韩涓嶈兘鍋氳繖涓噸鏂拌皟搴︼紝鍥犱负鍦ㄨ澶囨寕璧锋湡闂达紙鍗冲洖璋冭繍琛屾椂锛変换浣曠被鍨嬬殑鎸傝捣璇锋眰閮戒笉浼氳鎺ュ彈銆?

璇ュ疄鐜伴潪甯搁€傚悎鍦ㄤ腑鏂笂涓嬫枃涓紓姝ヤ娇鐢ㄣ€傜劧鑰岃繖绉嶄娇鐢ㄤ笉鍙伩鍏嶅湴娑夊強绔炰簤锛屽洜涓?PM core 鏃犳硶灏?->runtime_suspend() 鍥炶皟涓?I/O 璇锋眰鐨勫埌杈惧悓姝ャ€傝繖绉嶅悓姝ュ繀椤荤敱椹卞姩浣跨敤鍏剁鏈夐攣鏉ュ鐞嗐€?

```
	foo_read_or_write(struct foo_priv *foo, void *data)
	{
		lock(&foo->private_lock);
		add_request_to_io_queue(foo, data);
		if (foo->num_pending_requests++ == 0)
			pm_runtime_get(&foo->dev);
		if (!foo->is_suspended)
			foo_process_next_request(foo);
		unlock(&foo->private_lock);
	}

	foo_io_completion(struct foo_priv *foo, void *req)
	{
		lock(&foo->private_lock);
		if (--foo->num_pending_requests == 0)
			pm_runtime_put_autosuspend(&foo->dev);
		else
			foo_process_next_request(foo);
		unlock(&foo->private_lock);
		/* Send req result back to the user ... */
	}

	int foo_runtime_suspend(struct device *dev)
	{
		struct foo_priv foo = container_of(dev, ...);
		int ret = 0;

		lock(&foo->private_lock);
		if (foo->num_pending_requests > 0) {
			ret = -EBUSY;
		} else {
			/* ... suspend the device ... */
			foo->is_suspended = 1;
		}
		unlock(&foo->private_lock);
		return ret;
	}

	int foo_runtime_resume(struct device *dev)
	{
		struct foo_priv foo = container_of(dev, ...);

		lock(&foo->private_lock);
		/* ... resume the device ... */
		foo->is_suspended = 0;
		pm_runtime_mark_last_busy(&foo->dev);
		if (foo->num_pending_requests > 0)
			foo_process_next_request(foo);
		unlock(&foo->private_lock);
		return 0;
	}

```
瑕佺偣鏄紝鍦?foo_io_completion() 璇锋眰鑷姩鎸傝捣涔嬪悗锛宖oo_runtime_suspend() 鍥炶皟鍙兘涓?foo_read_or_write() 绔炰簤銆傚洜姝?foo_runtime_suspend() 蹇呴』鍦ㄥ厑璁告寕璧风户缁繘琛屼箣鍓嶏紙鎸佹湁绉佹湁閿佹椂锛夋鏌ユ槸鍚︽湁浠讳綍寰呭鐞嗙殑 I/O 璇锋眰銆?

姝ゅ锛宲ower.autosuspend_delay 瀛楁鍙互闅忔椂琚敤鎴风┖闂存敼鍙樸€傚鏋滈┍鍔ㄥ叧蹇冭繖涓€鐐癸紝瀹冨彲浠ュ湪鎸佹湁鑷韩绉佹湁閿佺殑鎯呭喌涓嬶紝浠?->runtime_suspend() 鍥炶皟鍐呰皟鐢?pm_runtime_autosuspend_expiration()銆傚鏋滃嚱鏁拌繑鍥炰竴涓潪闆跺€硷紝鍒欏欢杩熷皻鏈埌鏈燂紝璇ュ洖璋冨簲褰撹繑鍥?-EAGAIN銆?
