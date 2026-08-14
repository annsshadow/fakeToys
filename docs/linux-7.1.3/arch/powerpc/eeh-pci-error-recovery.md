## PCI 鎬荤嚎 EEH 閿欒鎭㈠


Linas Vepstas <linas@austin.ibm.com>

2005 骞?1 鏈?12 鏃?

### 姒傝堪锛?
鍩轰簬 IBM POWER 鐨?pSeries 鍜?iSeries 璁＄畻鏈哄寘鍚?PCI 鎬荤嚎鎺у埗鍣ㄨ姱鐗囷紝杩欎簺鑺墖鍏锋湁
妫€娴嬪拰鎶ュ憡鍚勭 PCI 鎬荤嚎閿欒鏉′欢鐨勬墿灞曡兘鍔涖€傝繖浜涚壒鎬ц缁熺О涓衡€淓EH鈥濓紝鍗斥€淓nhanced
Error Handling锛堝寮洪敊璇鐞嗭級鈥濄€侲EH 纭欢鐗规€у厑璁告竻闄?PCI 鎬荤嚎閿欒骞垛€滈噸鍚€漃CI
鍗★紝鑰屾棤闇€閲嶅惎鎿嶄綔绯荤粺銆?
杩欎笌浼犵粺鐨?PCI 閿欒澶勭悊褰㈡垚瀵规瘮锛屽湪浼犵粺鏂瑰紡涓紝PCI 鑺墖鐩存帴杩炲埌 CPU锛岄敊璇細瀵艰嚧
CPU 鐨勬満鍣ㄦ鏌?check-stop 鐘舵€侊紝瀹屽叏鍋滄 CPU銆傚彟涓€绉嶁€滀紶缁熲€濇妧鏈槸蹇界暐姝ょ被閿欒锛?杩欏彲鑳藉鑷寸敤鎴锋暟鎹垨鍐呮牳鏁版嵁鎹熷潖銆侀€傞厤鍣ㄦ寕璧?鏃犲搷搴旓紝鎴栫郴缁熷穿婧?姝婚攣銆傚洜姝わ紝EEH
鑳屽悗鐨勭悊蹇垫槸锛氭搷浣滅郴缁熷彲浠ラ€氳繃鍏嶅彈 PCI 閿欒鐨勫奖鍝嶈€屽彉寰楁洿鍙潬銆佹洿鍋ュ．锛屽苟璧嬩簣 OS
鈥滈噸鍚€?鎭㈠鍗曚釜 PCI 璁惧鐨勮兘鍔涖€?
鍩轰簬 PCI-E 瑙勮寖鐨勫叾浠栧巶鍟嗙殑鏈潵绯荤粺鍙兘鍖呭惈绫讳技鐗规€с€?

### EEH 閿欒鐨勬垚鍥?
EEH 鏈€鍒濊璁＄敤浜庨槻鑼冪‖浠舵晠闅滐紝渚嬪 PCI 鍗″洜鐑€佹箍搴︺€佺伆灏樸€佹尟鍔ㄥ拰涓嶈壇鐢垫皵杩炴帴
鑰屾崯鍧忋€傚湪鈥滅幇瀹炵敓娲烩€濅腑鐪嬪埌鐨勭粷澶у鏁?EEH 閿欒鏄敱浜?PCI 鍗℃彃鎺ヤ笉鑹紝鎴栬€咃紙涓嶅垢鍦?鐩稿綋甯歌锛夌敱浜庤澶囬┍鍔?bug銆佽澶囧浐浠?bug锛屼互鍙婃湁鏃?PCI 鍗＄‖浠?bug 閫犳垚鐨勩€?
鏈€甯歌鐨勮蒋浠?bug 鏄紝瀵艰嚧璁惧灏濊瘯 DMA 鍒扮郴缁熷唴瀛樹腑鏈棰勭暀缁欒鍗¤繘琛?DMA 璁块棶鐨?浣嶇疆銆傝繖鏄竴涓己澶х殑鐗规€э紝鍥犱负瀹冮槻姝簡鍘熸湰浼氱敱閿欒 DMA 閫犳垚鐨勯潤榛樺唴瀛樻崯鍧忋€傝繃鍘?鍑犲勾涓紝宸查€氳繃姝ゆ柟寮忓彂鐜板苟淇浜嗗涓澶囬┍鍔?bug銆侲EH 閿欒鐨勫叾浠栧彲鑳藉師鍥犲寘鎷暟鎹垨
鍦板潃绾垮鍋舵牎楠岄敊璇紙渚嬪锛岀敱浜庡崱鎻掓帴涓嶈壇瀵艰嚧鐨勭數姘旇繛鎺ヤ笉鑹級锛屼互鍙?PCI-X
split-completion 閿欒锛堢敱浜庤蒋浠躲€佽澶囧浐浠舵垨璁惧 PCI 纭欢 bug锛夈€傜粷澶у鏁扳€滅湡姝ｇ殑
纭欢鏁呴殰鈥濆彲浠ラ€氳繃鐗╃悊鎷斿嚭骞堕噸鏂版彃濂?PCI 鍗℃潵娌绘剤銆?

### 妫€娴嬩笌鎭㈠

鍦ㄦ帴涓嬫潵鐨勮璁轰腑锛屽皢缁欏嚭濡備綍妫€娴嬪拰浠?EEH 閿欒涓仮澶嶇殑涓€鑸杩般€傞殢鍚庢杩?Linux
鍐呮牳涓綋鍓嶅疄鐜版槸濡備綍鍋氱殑銆傚疄闄呭疄鐜板彲鑳戒細鍙戠敓鍙樺寲锛屼竴浜涚粏鑺備粛鍦ㄨ璁轰腑銆傚鏋滄垨褰撳叾浠?鏋舵瀯瀹炵幇绫讳技鍔熻兘鏃讹紝杩欎簺涔熷彲鑳戒細鍙楀埌褰卞搷銆?
褰?PCI 涓绘ˉ锛圥HB锛屽嵆杩炴帴 PCI 鎬荤嚎涓庣郴缁?CPU 鐢靛瓙澶嶅悎浣撶殑鎬荤嚎鎺у埗鍣級妫€娴嬪埌 PCI
閿欒鏉′欢鏃讹紝瀹冧細鈥滈殧绂烩€濆彈褰卞搷鐨?PCI 鍗°€傞殧绂讳細闃诲鎵€鏈夊啓鎿嶄綔锛堟棤璁烘槸浠庣郴缁熷埌鍗★紝
杩樻槸浠庡崱鍒扮郴缁燂級锛屽苟涓斾細瀵艰嚧鎵€鏈夎鎿嶄綔杩斿洖鍏?ff锛堝浜?8/16/32 浣嶈锛屽垎鍒负
0xff銆?xffff銆?xffffffff锛夈€傞€夋嫨杩欎釜鍊兼槸鍥犱负瀹冧笌璁惧琚墿鐞嗕粠鎻掓Ы鎷斿嚭鏃朵細寰楀埌鐨勫€?鐩稿悓銆傝繖鍖呮嫭瀵?PCI 鍐呭瓨銆両/O 绌洪棿鍜?PCI 閰嶇疆绌洪棿鐨勮闂€備絾鏄紝涓柇浠嶄細缁х画鎶曢€掋€?
妫€娴嬪拰鎭㈠鏄湪 ppc64 鍥轰欢鐨勮緟鍔╀笅杩涜鐨勩€侺inux 鍐呮牳涓繘鍏ュ浐浠剁殑缂栫▼鎺ュ彛琚О涓?RTAS锛圧un-Time Abstraction Services锛岃繍琛屾椂鎶借薄鏈嶅姟锛夈€侺inux 鍐呮牳涓嶏紙涓嶅簲锛夌洿鎺?璁块棶 PCI 鑺墖缁勪腑鐨?EEH 鍔熻兘锛屼富瑕佹槸鍥犱负甯傚満涓婃湁璁稿涓嶅悓鐨勮姱鐗囩粍锛屽悇鑷叿鏈変笉鍚岀殑
鎺ュ彛鍜屾€櫀銆傚浐浠舵彁渚涗簡涓€涓粺涓€鐨勬娊璞″眰锛屽彲涓庢墍鏈?pSeries 鍜?iSeries 纭欢閰嶅悎宸ヤ綔
锛堝苟涓斿悜鍓嶅吋瀹癸級銆?
濡傛灉 OS 鎴栬澶囬┍鍔ㄦ€€鐤戞煇涓?PCI 鎻掓Ы宸茶 EEH 闅旂锛屽畠鍙互鍙戣捣涓€涓浐浠惰皟鐢ㄦ潵纭鏄惁
濡傛銆傚鏋滄槸锛岄偅涔堣澶囬┍鍔ㄥ簲浣胯嚜宸辫繘鍏ヤ竴鑷寸姸鎬侊紙閴翠簬瀹冩棤娉曞畬鎴愪换浣曟寕璧风殑宸ヤ綔锛夊苟寮€濮?鎭㈠璇ュ崱銆傛仮澶嶉€氬父鍖呮嫭澶嶄綅 PCI 璁惧锛堝皢 PCI #RST 绾挎媺楂樹袱绉掞級锛岀劧鍚庤缃澶囬厤缃┖闂?锛堝熀鍦板潃瀵勫瓨鍣紙BAR锛夈€佸欢杩熷畾鏃跺櫒銆乧ache 琛屽ぇ灏忋€佷腑鏂嚎绛夛級銆傞殢鍚庢槸璁惧椹卞姩鐨勯噸鏂?鍒濆鍖栥€傚湪鏈€鍧忔儏鍐典笅锛屽彲浠ュ垏鎹㈠崱鐨勭數婧愶紝鑷冲皯鍦ㄦ敮鎸佺儹鎻掓嫈鐨勬彃妲戒笂鍙互銆傚師鍒欎笂锛岃繙鍦?璁惧椹卞姩涔嬩笂鐨勫眰鍙兘涓嶉渶瑕佺煡閬?PCI 鍗″凡閫氳繃杩欑鏂瑰紡鈥滈噸鍚€濓紱鐞嗘兂鎯呭喌涓嬶紝鍦ㄥ崱琚浣?鏈熼棿锛屼互澶綉/纾佺洏/USB I/O 鏈€澶氬簲鍑虹幇涓€娆℃殏鍋溿€?
濡傛灉鍗″湪涓夋鎴栧洓娆″浣嶅悗浠嶆棤娉曟仮澶嶏紝鍐呮牳/璁惧椹卞姩搴斿亣瀹氭渶鍧忔儏鍐碉紝鍗冲崱宸插畬鍏ㄦ崯鍧忥紝骞?灏嗘閿欒鎶ュ憡缁欑郴缁熺鐞嗗憳銆傛澶栵紝閿欒淇℃伅閫氳繃 RTAS 浠ュ強閫氳繃 syslogd
锛?var/log/messages锛夋姤鍛婏紝浠ユ彁閱掔郴缁熺鐞嗗憳鍏充簬 PCI 澶嶄綅銆傚鐞嗘晠闅滈€傞厤鍣ㄧ殑姝ｇ‘鏂规硶
鏄娇鐢ㄦ爣鍑?PCI 鐑彃鎷斿伐鍏风Щ闄ゅ苟鏇存崲鎹熷潖鐨勫崱銆?

### 褰撳墠 PPC64 Linux EEH 瀹炵幇

鐩墠锛屽凡缁忓疄鐜颁簡涓€涓€氱敤鐨?EEH 鎭㈠鏈哄埗锛屽洜姝ゅ崟涓澶囬┍鍔ㄦ棤闇€淇敼鍗冲彲鏀寔 EEH
鎭㈠銆傝繖涓€氱敤鏈哄埗鍊熷姪 PCI 鐑彃鎷斿熀纭€璁炬柦锛屽苟閫氳繃 userspace/udev 鍩虹璁炬柦灏嗕簨浠跺悜涓?浼犻€掋€備互涓嬫槸鍏跺疄鐜版柟寮忕殑璇︾粏鎻忚堪銆?
EEH 蹇呴』鍦ㄥ紩瀵艰繃绋嬫棭鏈熴€佷互鍙婂湪 PCI 鎻掓Ы琚儹鎻掓嫈鏃讹紝鍦?PHB 涓惎鐢ㄣ€傚墠鑰呯敱
arch/powerpc/platforms/pseries/eeh.c 涓殑 eeh_init() 鎵ц锛屽悗鑰呯敱
drivers/pci/hotplug/pSeries_pci.c 璋冪敤 eeh.c 浠ｇ爜鏉ユ墽琛屻€侲EH 蹇呴』鍦?PCI 鎵弿璁惧
涔嬪墠鍚敤銆傚綋鍓嶇殑 Power5 纭欢鍦?EEH 鏈惎鐢ㄦ椂鏃犳硶宸ヤ綔锛涘敖绠¤緝鏃х殑 Power4 鍙互鍦ㄥ叾
绂佺敤鏃惰繍琛屻€傚疄闄呬笂锛孍EH 宸叉棤娉曞啀鍏抽棴銆侾CI 璁惧**蹇呴』**鍦?EEH 浠ｇ爜涓敞鍐岋紱EEH
浠ｇ爜闇€瑕佺煡閬?PCI 璁惧鐨?I/O 鍦板潃鑼冨洿锛屼互渚挎娴嬮敊璇€傜粰瀹氫换鎰忓湴鍧€锛屼緥绋?pci_get_device_by_addr() 灏嗘壘鍒颁笌璇ュ湴鍧€鍏宠仈鐨?pci 璁惧锛堝鏋滄湁锛夈€?
榛樿鐨?arch/powerpc/include/asm/io.h 瀹?readb()銆乮nb()銆乮nsb() 绛夊寘鍚竴椤规鏌ワ紝
鐢ㄤ簬鏌ョ湅 i/o 璇绘槸鍚﹁繑鍥炰簡鍏?0xff銆傚鏋滄槸锛屽畠浠細璋冪敤 eeh_dn_check_failure()锛屽悗鑰?鍐嶈闂浐浠讹細鍏?ff 鍊兼槸鍚︽槸鐪熸 EEH 閿欒鐨勬爣蹇椼€傚鏋滀笉鏄紝鍒欏儚姝ｅ父涓€鏍风户缁鐞嗐€傝繖浜?璇姤鎴栤€滃亣闃虫€р€濈殑鎬绘暟鍙互鍦?/proc/ppc64/eeh 涓湅鍒帮紙鍙兘浼氬彉鏇达級銆傞€氬父锛屽嚑涔庢墍鏈夎繖浜涢兘
鍙戠敓鍦ㄥ紩瀵兼湡闂存壂鎻?PCI 鎬荤嚎鏃讹紝姝ゆ椂澶ч噺 0xff 璇绘槸鎬荤嚎鎵弿杩囩▼鐨勪竴閮ㄥ垎銆?
濡傛灉妫€娴嬪埌鍐荤粨鐨勬彃妲斤紝arch/powerpc/platforms/pseries/eeh.c 涓殑浠ｇ爜浼氬悜 syslog
锛?var/log/messages锛夋墦鍗颁竴涓爤璺熻釜銆傝繖涓爤璺熻釜瀵硅澶囬┍鍔ㄤ綔鑰呴潪甯告湁鐢紝鐢ㄤ簬鎵惧嚭鍦?浣曞妫€娴嬪埌 EEH 閿欒锛屽洜涓洪敊璇湰韬€氬父鍙戠敓鍦ㄧ◢鏃╀箣鍓嶃€?
鎺ヤ笅鏉ワ紝瀹冧娇鐢?Linux 鍐呮牳鐨?notifier 閾?宸ヤ綔闃熷垪鏈哄埗锛屽厑璁镐换浣曠浉鍏虫柟浜嗚В璇ユ晠闅溿€傝澶?椹卞姩鎴栧唴鏍哥殑鍏朵粬閮ㄥ垎鍙互浣跨敤 `eeh_register_notifier(struct notifier_block *)` 鏉?浜嗚В EEH 浜嬩欢銆傝浜嬩欢灏嗗寘鍚寚鍚?pci 璁惧銆佽澶囪妭鐐瑰拰涓€浜涚姸鎬佷俊鎭殑鎸囬拡銆備簨浠剁殑鎺ユ敹鑰?鍙互鈥滀负鎵€娆蹭负鈥濓紱榛樿澶勭悊绋嬪簭灏嗗湪鏈妭杩涗竴姝ユ弿杩般€?
涓轰簡鍗忓姪璁惧鎭㈠锛宔eh.c 瀵煎嚭浜嗕互涓嬪嚱鏁帮細

rtas_set_slot_reset()
   灏?PCI #RST 绾挎媺楂?1/8 绉?rtas_configure_bridge()
   璇锋眰鍥轰欢閰嶇疆浣嶄簬 pci 鎻掓Ы鎷撴墤涔嬩笅鐨勪换浣?PCI 妗ャ€?eeh_save_bars() 鍜?eeh_restore_bars()锛?   淇濆瓨鍜屾仮澶嶈澶囧強鍏朵笅浠讳綍璁惧鐨?PCI 閰嶇疆绌洪棿淇℃伅銆?

EEH notifier_block 浜嬩欢鐨勫鐞嗙▼搴忓湪 drivers/pci/hotplug/pSeries_pci.c 涓疄鐜帮紝鍚嶄负
handle_eeh_events()銆傚畠淇濆瓨璁惧 BAR锛岀劧鍚庤皟鐢?rpaphp_unconfig_pci_adapter()銆傛渶鍚庝竴
娆¤皟鐢ㄤ細瀵艰嚧璇ュ崱鐨勭殑璁惧椹卞姩鍋滄锛屼粠鑰屽悜鐢ㄦ埛绌洪棿鍙戝嚭 uevent銆傝繖浼氳Е鍙戠敤鎴风┖闂磋剼鏈紝
鍙兘鍙戝嚭璇稿浠ュお缃戝崱鐨勨€渋fdown eth0鈥濅箣绫荤殑鍛戒护锛岀瓑绛夈€傜劧鍚庤澶勭悊绋嬪簭浼戠湢 5 绉掞紝甯屾湜
缁欑敤鎴风┖闂磋剼鏈冻澶熺殑鏃堕棿瀹屾垚銆傛帴鐫€瀹冨浣?PCI 鍗★紝閲嶆柊閰嶇疆璁惧 BAR 浠ュ強鍏朵笅鐨勪换浣曟ˉ銆?鐒跺悗瀹冭皟鐢?rpaphp_enable_pci_slot()锛岃繖浼氶噸鏂板惎鍔ㄨ澶囬┍鍔ㄥ苟瑙﹀彂鏇村鐢ㄦ埛绌洪棿浜嬩欢
锛堜緥濡傦紝瀵逛互澶綉鍗¤皟鐢ㄢ€渋fup eth0鈥濓級銆?

### 璁惧鍏抽棴涓庣敤鎴风┖闂翠簨浠?
鏈妭璁板綍褰?pci 鎻掓Ы琚彇娑堥厤缃椂鍙戠敓鐨勪簨鎯咃紝閲嶇偣鍏虫敞璁惧椹卞姩濡備綍琚叧闂紝浠ュ強浜嬩欢
濡備綍鎶曢€掔粰鐢ㄦ埛绌洪棿鑴氭湰銆?
浠ヤ笅鏄鑷村湪 EEH 澶嶄綅绗竴闃舵璋冪敤璁惧椹卞姩 close 鍑芥暟鐨勪竴绯诲垪浜嬩欢绀轰緥銆?```

    rpa_php_unconfig_pci_adapter (struct slot *)  // in rpaphp_pci.c
    {
      calls
      pci_remove_bus_device (struct pci_dev *) // in /drivers/pci/remove.c
      {
        calls
        pci_destroy_dev (struct pci_dev *)
        {
          calls
          device_unregister (&dev->dev) // in /drivers/base/core.c
          {
            calls
            device_del (struct device *)
            {
              calls
              bus_remove_device() // in /drivers/base/bus.c
              {
                calls
                device_release_driver()
                {
                  calls
                  struct device_driver->remove() which is just
                  pci_device_remove()  // in /drivers/pci/pci_driver.c
                  {
                    calls
                    struct pci_driver->remove() which is just
                    pcnet32_remove_one() // in /drivers/net/pcnet32.c
                    {
                      calls
                      unregister_netdev() // in /net/core/dev.c
                      {
                        calls
                        dev_close()  // in /net/core/dev.c
                        {
                           calls dev->stop();
                           which is just pcnet32_close() // in pcnet32.c
                           {
                             which does what you wanted
                             to stop the device
                           }
                        }
                     }
                   which
                   frees pcnet32 device driver memory
                }
     }}}}}}

```
鍦?drivers/pci/pci_driver.c 涓紝struct device_driver->remove() 灏辨槸
pci_device_remove()锛屽畠璋冪敤 struct pci_driver->remove()锛屽嵆
pcnet32_remove_one()锛屽悗鑰呰皟鐢?unregister_netdev()锛堝湪 net/core/dev.c锛夛紝鍚庤€呰皟鐢?dev_close()锛堝湪 net/core/dev.c锛夛紝鍚庤€呰皟鐢?dev->stop()锛屽嵆 pcnet32_close()锛岀劧鍚?鎵ц閫傚綋鐨勫叧闂搷浣溿€?
---

浠ヤ笅鏄彂閫佺粰鐢ㄦ埛绌洪棿鐨勪簨浠剁殑绫讳技鏍堣窡韪?```

  rpa_php_unconfig_pci_adapter() {             // in rpaphp_pci.c
    calls
    pci_remove_bus_device (struct pci_dev *) { // in /drivers/pci/remove.c
      calls
      pci_destroy_dev (struct pci_dev *) {
        calls
        device_unregister (&dev->dev) {        // in /drivers/base/core.c
          calls
          device_del(struct device * dev) {    // in /drivers/base/core.c
            calls
            kobject_del() {                    //in /libs/kobject.c
              calls
              kobject_uevent() {               // in /libs/kobject.c
                calls
                kset_uevent() {                // in /lib/kobject.c
                  calls
                  kset->uevent_ops->uevent()   // which is really just
                  a call to
                  dev_uevent() {               // in /drivers/base/core.c
                    calls
                    dev->bus->uevent() which is really just a call to
                    pci_uevent () {            // in drivers/pci/hotplug.c
                      which prints device name, etc....
                   }
                 }
                 then kobject_uevent() sends a netlink uevent to userspace
                 --> userspace uevent
                 (during early boot, nobody listens to netlink events and
                 kobject_uevent() executes uevent_helper[], which runs the
                 event process /sbin/hotplug)
             }
           }
           kobject_del() then calls sysfs_remove_dir(), which would
           trigger any user-space daemon that was watching /sysfs,
           and notice the delete event.


```
### 褰撳墠璁捐鐨勪紭鐐逛笌缂虹偣

褰撳墠鐨?EEH 杞欢鎭㈠璁捐瀛樺湪鑻ュ共闂锛屽彲鑳戒細鍦ㄦ湭鏉ョ殑淇涓В鍐炽€備絾棣栧厛瑕佹敞鎰忥紝褰撳墠
璁捐鐨勪竴澶т紭鐐规槸鏃犻渶瀵瑰崟涓澶囬┍鍔ㄥ仛浠讳綍淇敼锛屽洜姝ゅ綋鍓嶈璁＄殑瑕嗙洊闈㈠緢骞裤€傝璁捐鏈€澶х殑
缂虹偣鏄畠鍙兘鎵撴壈閭ｄ簺鏈笉闇€瑕佽鎵撴壈鐨勭綉缁滃畧鎶よ繘绋嬪拰鏂囦欢绯荤粺銆?
- 涓€涓皬鎶辨€ㄦ槸锛屽浣嶇綉鍗′細瀵艰嚧鐢ㄦ埛绌洪棿鑳岄潬鑳岀殑 ifdown/ifup 鍡濆棟澹帮紝鍙兘鎵撴壈閭ｄ簺
   鏈笉闇€瑕佺煡閬?pci 鍗℃鍦ㄩ噸鍚殑缃戠粶瀹堟姢杩涚▼銆?
- 涓€涓洿涓ラ噸鐨勬媴蹇ф槸锛屽悓鏍风殑澶嶄綅瀵逛簬 SCSI 璁惧浼氬鑷存寕杞界殑鏂囦欢绯荤粺闄峰叆娣蜂贡銆傝剼鏈?   鏃犳硶鍦ㄤ簨鍚庡嵏杞芥枃浠剁郴缁熻€屼笉鍒锋柊鎸傝捣鐨勭紦鍐插尯锛屼絾杩欐槸涓嶅彲鑳界殑锛屽洜涓?I/O 宸茬粡鍋滄銆?   鍥犳锛岀悊鎯虫儏鍐典笅锛屽浣嶅簲璇ュ彂鐢熷湪鍧楀眰鎴栨洿浣庡眰锛岃繖鏍锋枃浠剁郴缁熷氨涓嶄細琚墦鎵般€?
   Ext3fs 浼间箮鍏锋湁瀹瑰繊鎬э紝浼氶噸璇曡/鍐欑洿鍒版垚鍔熴€備袱鑰呭湪姝ゅ満鏅笅閮藉彧缁忚繃浜嗚交搴︽祴璇曘€?
   SCSI-generic 瀛愮郴缁熷凡缁忓唴缃簡鎵ц SCSI 璁惧澶嶄綅銆丼CSI 鎬荤嚎澶嶄綅鍜?SCSI 涓绘€荤嚎
   閫傞厤鍣紙HBA锛夊浣嶇殑浠ｇ爜銆傚鏋?SCSI 鍛戒护澶辫触锛岃繖浜涗細琚骇鑱旀垚涓€绯诲垪灏濊瘯鐨勫浣嶃€傝繖浜?   瀹屽叏瀵瑰潡灞傞殣钘忋€傚皢 EEH 澶嶄綅娣诲姞鍒拌繖涓€绯诲垪浜嬩欢涓槸鍗佸垎鑷劧鐨勩€?
- 濡傛灉鏍硅澶囧彂鐢?SCSI 閿欒锛屼竴鍒囬兘灏嗕涪澶憋紝闄ら潪绯荤粺绠＄悊鍛樻湁鍏堣涔嬫槑灏?/bin銆?sbin銆?   /etc銆?var 绛夋斁鍦?ramdisk/tmpfs 涓繍琛屻€?

### 缁撹

姝ｅ湪鍙栧緱杩涘睍鈥︹€?