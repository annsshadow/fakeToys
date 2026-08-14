## 灏嗛┍鍔ㄧЩ妞嶅埌鏂扮殑椹卞姩妯″瀷


Patrick Mochel

2003 骞?1 鏈?7 鏃?

姒傝堪锛圤verview锛?
璇峰弬闃?`Documentation/driver-api/driver-model/*.rst`锛屼簡瑙ｅ悇绉嶉┍鍔ㄧ被鍨嬪拰姒傚康鐨勫畾涔夈€?
灏嗚澶囬┍鍔ㄧЩ妞嶅埌鏂版ā鍨嬬殑澶ч儴鍒嗗伐浣滃彂鐢熷湪鎬荤嚎椹卞姩灞傘€傝繖鏄湁鎰忎负涔嬶紝浠ュ敖閲忓噺灏忓鍐呮牳椹卞姩鐨勮礋闈㈠奖鍝嶏紝骞跺厑璁告€荤嚎椹卞姩閫愭杩囨浮銆?
绠€鑰岃█涔嬶紝椹卞姩妯″瀷鐢变竴缁勫彲浠ュ祵鍏ュ埌鏇村ぇ鐨勩€佺壒瀹氫簬鎬荤嚎鐨勫璞′腑鐨勫璞＄粍鎴愩€傝繖浜涢€氱敤瀵硅薄涓殑瀛楁鍙互鍙栦唬鐗瑰畾浜庢€荤嚎鐨勫璞′腑鐨勫瓧娈点€?
閫氱敤瀵硅薄蹇呴』鍚戦┍鍔ㄦā鍨嬫牳蹇冩敞鍐屻€傝繖鏍峰仛涔嬪悗锛屽畠浠細閫氳繃 sysfs 鏂囦欢绯荤粺瀵煎嚭銆俿ysfs 鍙互閫氳繃
```

	# mount -t sysfs sysfs /sys



```
杩欎釜杩囩▼锛圱he Process锛?
姝ラ 0锛氶槄璇?include/linux/device.h锛屼簡瑙ｅ璞″拰鍑芥暟鐨勫畾涔夈€?
姝ラ 1锛氭敞鍐屾€荤嚎椹卞姩銆?

```
    struct bus_type pci_bus_type = {
          .name           = "pci",
    };


```
- 娉ㄥ唽鎬荤嚎绫诲瀷銆?
  杩欏簲褰撳湪鎬荤嚎绫诲瀷鐨勫垵濮嬪寲鍑芥暟涓畬鎴愶紝
```

    static int __init pci_driver_init(void)
    {
            return bus_register(&pci_bus_type);
    }

    subsys_initcall(pci_driver_init);


  The bus type may be unregistered (if the bus driver may be compiled
  as a module) by doing::

     bus_unregister(&pci_bus_type);


```
- 瀵煎嚭鎬荤嚎绫诲瀷渚涘叾浠栦唬鐮佷娇鐢ㄣ€?
  鍏跺畠浠ｇ爜鍙兘甯屾湜寮曠敤璇ユ€荤嚎绫诲瀷锛屽洜姝ゅ簲鍦ㄥ叡浜ご鏂囦欢涓０鏄庡畠骞跺鍑鸿绗﹀彿銆?
```

  extern struct bus_type pci_bus_type;


```
```

  EXPORT_SYMBOL(pci_bus_type);



```
- 杩欏皢瀵艰嚧璇ユ€荤嚎鍑虹幇鍦?/sys/bus/pci/ 涓嬶紝鍖呭惈涓や釜
```

    # tree -d /sys/bus/pci/
    /sys/bus/pci/
    |-- devices
    `-- drivers



```
姝ラ 2锛氭敞鍐岃澶囥€?
struct device 琛ㄧず鍗曚釜璁惧銆傚畠涓昏鍖呭惈鎻忚堪璇ヨ澶囦笌鍏跺畠瀹炰綋涔嬮棿鍏崇郴鐨勫厓鏁版嵁銆?

```

    struct pci_dev {
           ...
           struct  device  dev;            /* Generic device interface */
           ...
    };

  It is recommended that the generic device not be the first item in
  the struct to discourage programmers from doing mindless casts
  between the object types. Instead macros, or inline functions,
  should be created to convert from the generic object type::

    #define to_pci_dev(n) container_of(n, struct pci_dev, dev)

    or

    static inline struct pci_dev * to_pci_dev(struct kobject * kobj)
    {
	return container_of(n, struct pci_dev, dev);
    }

  This allows the compiler to verify type-safety of the operations
  that are performed (which is Good).


```
- 鍦ㄦ敞鍐屾椂鍒濆鍖栬澶囥€?
  褰撹澶囪鍙戠幇鎴栧悜鎬荤嚎绫诲瀷娉ㄥ唽鏃讹紝鎬荤嚎椹卞姩搴斿綋鍒濆鍖栭€氱敤璁惧銆傛渶闇€瑕佸垵濮嬪寲鐨勫瓧娈垫槸 bus_id銆乸arent 鍜?bus銆?
  bus_id 鏄竴涓?ASCII 瀛楃涓诧紝鍖呭惈璁惧鍦ㄨ鎬荤嚎涓婄殑鍦板潃銆傝瀛楃涓茬殑鏍煎紡鏄壒瀹氫簬鎬荤嚎鐨勩€傝繖瀵逛簬鍦?sysfs 涓〃绀鸿澶囨槸蹇呰鐨勩€?
  parent 鏄澶囩殑鐗╃悊鐖惰澶囥€傛€荤嚎椹卞姩姝ｇ‘璁剧疆璇ュ瓧娈甸潪甯搁噸瑕併€?
  椹卞姩妯″瀷缁存姢涓€涓湁搴忕殑璁惧鍒楄〃锛岀敤浜庣數婧愮鐞嗐€傝鍒楄〃蹇呴』鏄湁搴忕殑锛屼互淇濊瘉璁惧鍦ㄥ叾鐗╃悊鐖惰澶囦箣鍓嶈鍏抽棴锛屽弽涔嬩害鐒躲€傝鍒楄〃鐨勯『搴忕敱宸叉敞鍐岃澶囩殑 parent 鍐冲畾銆?
  姝ゅ锛岃澶囩殑 sysfs 鐩綍鐨勪綅缃彇鍐充簬璁惧鐨?parent銆俿ysfs 瀵煎嚭涓€涓暅鍍忚澶囧眰娆＄殑鐩綍缁撴瀯銆傚噯纭湴璁剧疆 parent 鍙互淇濊瘉 sysfs 鍑嗙‘鍦拌〃绀鸿繖涓眰娆°€?
  璁惧鐨?bus 瀛楁鏄竴涓寚鍚戣璁惧鎵€灞炴€荤嚎绫诲瀷鐨勬寚閽堛€傚畠搴斿綋琚缃负涔嬪墠宸插０鏄庡苟鍒濆鍖栫殑 bus_type銆?
  鍙€夊湴锛屾€荤嚎椹卞姩鍙互璁剧疆璁惧鐨?name 鍜?release 瀛楁銆?
  name 瀛楁鏄竴涓弿杩拌璁惧鐨?ASCII 瀛楃涓诧紝渚嬪

     "ATI Technologies Inc Radeon QD"

  release 瀛楁鏄竴涓洖璋冨嚱鏁帮紝褰撹澶囧凡琚Щ闄ゃ€佷笖瀵瑰畠鐨勬墍鏈夊紩鐢ㄩ兘宸茶閲婃斁鏃讹紝椹卞姩妯″瀷鏍稿績浼氳皟鐢ㄥ畠銆傜◢鍚庡皢瀵规鍋氭洿澶氳鏄庛€?

- 娉ㄥ唽璁惧銆?
  涓€鏃﹂€氱敤璁惧琚垵濮嬪寲锛屽氨鍙互娉ㄥ唽瀹?```

       device_register(&dev->dev);

  It can later be unregistered by doing::

       device_unregister(&dev->dev);

  This should happen on buses that support hotpluggable devices.
  If a bus driver unregisters a device, it should not immediately free
  it. It should instead wait for the driver model core to call the
  device's release method, then free the bus-specific object.
  (There may be other code that is currently referencing the device
  structure, and it would be rude to free the device while that is
  happening).


  褰撹澶囪娉ㄥ唽鏃讹紝浼氬湪 sysfs 涓垱寤轰竴涓洰褰曘€俿ysfs 涓殑 PCI 鏍戝舰濡傦細锛?
    /sys/devices/pci0/
    |-- 00:00.0
    |-- 00:01.0
    |   `-- 01:00.0
    |-- 00:02.0
    |   `-- 02:1f.0
    |       `-- 03:00.0
    |-- 00:1e.0
    |   `-- 04:04.0
    |-- 00:1f.0
    |-- 00:1f.1
    |   |-- ide0
    |   |   |-- 0.0
    |   |   `-- 0.1
    |   `-- ide1
    |       `-- 1.0
    |-- 00:1f.2
    |-- 00:1f.3
    `-- 00:1f.5

  姝ゅ锛屽湪鎬荤嚎鐨?'devices' 鐩綍涓細鍒涘缓鎸囧悜璁惧鐗╃悊灞傛鐩綍鐨勭鍙烽摼鎺ワ細锛?
    /sys/bus/pci/devices/
    |-- 00:00.0 -> ../../../devices/pci0/00:00.0
    |-- 00:01.0 -> ../../../devices/pci0/00:01.0
    |-- 00:02.0 -> ../../../devices/pci0/00:02.0
    |-- 00:1e.0 -> ../../../devices/pci0/00:1e.0
    |-- 00:1f.0 -> ../../../devices/pci0/00:1f.0
    |-- 00:1f.1 -> ../../../devices/pci0/00:1f.1
    |-- 00:1f.2 -> ../../../devices/pci0/00:1f.2
    |-- 00:1f.3 -> ../../../devices/pci0/00:1f.3
    |-- 00:1f.5 -> ../../../devices/pci0/00:1f.5
    |-- 01:00.0 -> ../../../devices/pci0/00:01.0/01:00.0
    |-- 02:1f.0 -> ../../../devices/pci0/00:02.0/02:1f.0
    |-- 03:00.0 -> ../../../devices/pci0/00:02.0/02:1f.0/03:00.0
    `-- 04:04.0 -> ../../../devices/pci0/00:1e.0/04:04.0



```
姝ラ 3锛氭敞鍐岄┍鍔ㄣ€?
struct device_driver 鏄竴涓畝鍗曠殑椹卞姩缁撴瀯锛屽寘鍚竴缁勯┍鍔ㄦā鍨嬫牳蹇冨彲鑳戒細璋冪敤鐨勬搷浣溿€?

- 鍦ㄧ壒瀹氫簬鎬荤嚎鐨勯┍鍔ㄤ腑宓屽叆涓€涓?struct device_driver銆?
```

    struct pci_driver {
           ...
           struct device_driver    driver;
    };


```
- 鍒濆鍖栭€氱敤椹卞姩缁撴瀯銆?
  褰撻┍鍔ㄥ悜鎬荤嚎娉ㄥ唽鏃讹紙渚嬪璋冪敤 pci_register_driver()锛夛紝鍒濆鍖栭┍鍔ㄧ殑蹇呰瀛楁锛歯ame 鍜?bus 瀛楁銆?

- 娉ㄥ唽椹卞姩銆?
```

	driver_register(&drv->driver);

  to register the driver with the core.

  When the driver is unregistered from the bus, unregister it from the
  core by doing::

        driver_unregister(&drv->driver);

  Note that this will block until all references to the driver have
  gone away. Normally, there will not be any.


```
- sysfs 琛ㄧず銆?
  椹卞姩閫氳繃 sysfs 鍦ㄥ叾鎬荤嚎鐨?'drivers' 鐩綍涓鍑恒€?```

    /sys/bus/pci/drivers/
    |-- 3c59x
    |-- Ensoniq AudioPCI
    |-- agpgart-amdk7
    |-- e100
    `-- serial


```
姝ラ 4锛氫负椹卞姩瀹氫箟閫氱敤鏂规硶銆?
struct device_driver 瀹氫箟浜嗕竴缁勯┍鍔ㄦā鍨嬫牳蹇冧細璋冪敤鐨勬搷浣溿€傝繖浜涙搷浣滀腑鐨勫ぇ澶氭暟鍙兘涓庢€荤嚎宸茬粡涓洪┍鍔ㄥ畾涔夌殑鎿嶇被浼硷紝浣嗗弬鏁颁笉鍚屻€?
寮哄埗璁╂€荤嚎涓婄殑姣忎竴涓┍鍔ㄥ悓鏃跺皢瀹冧滑鑷繁鐨勯┍鍔ㄨ浆鎹负閫氱敤鏍煎紡锛屼細鏄洶闅句笖绻佺悙鐨勩€傜浉鍙嶏紝鎬荤嚎椹卞姩搴斿綋瀹氫箟閫氱敤鏂规硶鐨勫崟涓€瀹炰緥锛岀敱
```


  static int pci_device_remove(struct device * dev)
  {
          struct pci_dev * pci_dev = to_pci_dev(dev);
          struct pci_driver * drv = pci_dev->driver;

          if (drv) {
                  if (drv->remove)
                          drv->remove(pci_dev);
                  pci_dev->driver = NULL;
          }
          return 0;
  }


```
閫氱敤椹卞姩搴斿綋鐢ㄨ繖浜涙柟娉曞垵濮嬪寲锛岀劧鍚?```

        /* initialize common driver fields */
        drv->driver.name = drv->name;
        drv->driver.bus = &pci_bus_type;
        drv->driver.probe = pci_device_probe;
        drv->driver.resume = pci_device_resume;
        drv->driver.suspend = pci_device_suspend;
        drv->driver.remove = pci_device_remove;

        /* register with core */
        driver_register(&drv->driver);


```
鐞嗘兂鎯呭喌涓嬶紝鎬荤嚎鍙簲鍦ㄨ繖浜涘瓧娈靛皻鏈璁剧疆鏃舵墠鍒濆鍖栧畠浠€傝繖鏍峰厑璁搁┍鍔ㄥ疄鐜板畠浠嚜宸辩殑閫氱敤鏂规硶銆?

姝ラ 5锛氭敮鎸侀€氱敤椹卞姩缁戝畾銆?
璇ユā鍨嬪亣璁捐澶囨垨椹卞姩鍙互鍦ㄤ换浣曟椂鍒诲姩鎬佸湴娉ㄥ唽鍒版€荤嚎涓娿€傚綋娉ㄥ唽鍙戠敓鏃讹紝璁惧蹇呴』缁戝畾鍒颁竴涓┍鍔紝鎴栬€呴┍鍔ㄥ繀椤荤粦瀹氬埌瀹冩墍鏀寔鐨勬墍鏈夎澶囥€?
椹卞姩閫氬父鍖呭惈涓€涓畠鎵€鏀寔鐨勮澶?ID 鍒楄〃銆傛€荤嚎椹卞姩灏嗚繖浜?ID 涓庢敞鍐屽埌瀹冧笂闈㈢殑璁惧鐨?ID 杩涜姣旇緝銆傝澶?ID 鐨勬牸寮忥紝浠ュ強姣旇緝瀹冧滑鐨勮涔夛紝鏄壒瀹氫簬鎬荤嚎鐨勶紝鍥犳閫氱敤妯″瀷骞朵笉璇曞浘瀵瑰畠浠繘琛屾硾鍖栥€?
鐩稿弽锛屾€荤嚎鍙互鍦?struct bus_type 涓彁渚涗竴涓柟娉曪紝鐢?```

  int (*match)(struct device * dev, struct device_driver * drv);

```
濡傛灉椹卞姩鏀寔璇ヨ澶囷紝match 搴斿綋杩斿洖涓€涓鍊硷紝鍚﹀垯杩斿洖 0銆傚鏋滄棤娉曠‘瀹氱粰瀹氶┍鍔ㄦ槸鍚︽敮鎸佽璁惧锛屽畠涔熷彲浠ヨ繑鍥為敊璇爜锛堜緥濡?-EPROBE_DEFER锛夈€?
褰撹澶囪娉ㄥ唽鏃讹紝浼氶亶鍘嗘€荤嚎鐨勯┍鍔ㄥ垪琛ㄣ€傚姣忎釜椹卞姩璋冪敤 bus->match()锛岀洿鍒版壘鍒板尮閰嶃€?
褰撻┍鍔ㄨ娉ㄥ唽鏃讹紝浼氶亶鍘嗘€荤嚎鐨勮澶囧垪琛ㄣ€傚姣忎竴涓皻鏈鏌愪釜椹卞姩璁ら鐨勮澶囪皟鐢?bus->match()銆?
褰撲竴涓澶囨垚鍔熷湴缁戝畾鍒颁竴涓┍鍔ㄦ椂锛屼細璁剧疆 device->driver锛屽皢璇ヨ澶囨坊鍔犲埌璇ラ┍鍔ㄧ殑姣忛┍鍔ㄨ澶囧垪琛ㄤ腑锛屽苟鍦ㄨ椹卞姩鐨?sysfs 鐩綍涓垱寤轰竴涓寚鍚?```

  /sys/bus/pci/drivers/
  |-- 3c59x
  |   `-- 00:0b.0 -> ../../../../devices/pci0/00:0b.0
  |-- Ensoniq AudioPCI
  |-- agpgart-amdk7
  |   `-- 00:00.0 -> ../../../../devices/pci0/00:00.0
  |-- e100
  |   `-- 00:0c.0 -> ../../../../devices/pci0/00:0c.0
  `-- serial


```
杩欑椹卞姩缁戝畾搴斿綋鍙栦唬鎬荤嚎褰撳墠浣跨敤鐨勭幇鏈夐┍鍔ㄧ粦瀹氭満鍒躲€?

姝ラ 6锛氭彁渚涚儹鎻掓嫈鍥炶皟銆?
姣忓綋涓€涓澶囪娉ㄥ唽鍒伴┍鍔ㄦā鍨嬫牳蹇冩椂锛岀敤鎴风┖闂寸▼搴?/sbin/hotplug 浼氳璋冪敤锛屼互閫氱煡鐢ㄦ埛绌洪棿銆傜敤鎴峰彲浠ュ畾涔夊湪璁惧鎴栨彃鍏?绉婚櫎鏃惰鎵ц鐨勫姩浣溿€?
椹卞姩妯″瀷鏍稿績閫氳繃鐜鍙橀噺鍚戠敤鎴风┖闂翠紶閫掕嫢骞插弬鏁帮紝鍖呮嫭

- ACTION锛氳缃负 'add' 鎴?'remove'
- DEVPATH锛氳缃负璁惧鍦?sysfs 涓殑鐗╃悊璺緞銆?
鎬荤嚎椹卞姩涔熷彲浠ユ彁渚涢澶栫殑鍙傛暟渚涚敤鎴风┖闂翠娇鐢ㄣ€備负姝わ紝鎬荤嚎蹇呴』鍦?```

     int (*hotplug) (struct device *dev, char **envp,
                     int num_envp, char *buffer, int buffer_size);

```
涓疄鐜?'hotplug' 鏂规硶銆傝繖浼氬湪 /sbin/hotplug 鎵ц涔嬪墠绔嬪嵆琚皟鐢ㄣ€?

姝ラ 7锛氭竻鐞嗘€荤嚎椹卞姩銆?
閫氱敤鐨?bus銆乨evice 鍜?driver 缁撴瀯鎻愪緵浜嗚嫢骞插瓧娈碉紝鍙互鍙栦唬鎬荤嚎椹卞姩绉佷笅瀹氫箟鐨勯偅浜涘瓧娈点€?
- 璁惧鍒楄〃銆?
struct bus_type 鍖呭惈涓€涓敞鍐屽埌璇ユ€荤嚎绫诲瀷鐨勬墍鏈夎澶囩殑鍒楄〃銆傝繖鍖呮嫭璇ユ€荤嚎绫诲瀷鎵€鏈夊疄渚嬩笂鐨勬墍鏈夎澶囥€傛€荤嚎浣跨敤鐨勫唴閮ㄥ垪琛ㄥ彲浠ヨ绉婚櫎锛岃浆鑰屼娇鐢ㄨ繖涓€涓€?
```

  int bus_for_each_dev(struct bus_type * bus, struct device * start,
                       void * data, int (*fn)(struct device *, void *));


```
- 椹卞姩鍒楄〃銆?
struct bus_type 杩樺寘鍚竴涓敞鍐屽埌瀹冪殑鎵€鏈夐┍鍔ㄧ殑鍒楄〃銆傛€荤嚎椹卞姩缁存姢鐨勯┍鍔ㄥ唴閮ㄥ垪琛ㄥ彲浠ヨ绉婚櫎锛岃浆鑰屼娇鐢ㄩ€氱敤鐨勯偅涓€涓€?
```

  int bus_for_each_drv(struct bus_type * bus, struct device_driver * start,
                       void * data, int (*fn)(struct device_driver *, void *));


```
鏇村鐩稿叧淇℃伅璇峰弬闃?drivers/base/bus.c銆?

- rwsem銆?
struct bus_type 鍖呭惈涓€涓?rwsem锛岀敤浜庝繚鎶ゅ璁惧鍜岄┍鍔ㄥ垪琛ㄧ殑鎵€鏈夋牳蹇冭闂€傛€荤嚎椹卞姩鍙互鍦ㄥ唴閮ㄤ娇鐢ㄥ畠锛屽苟涓斿湪璁块棶鎬荤嚎缁存姢鐨勮澶囨垨椹卞姩鍒楄〃鏃跺簲褰撲娇鐢ㄥ畠銆?

- 璁惧鍜岄┍鍔ㄥ瓧娈点€?
struct device 鍜?struct device_driver 涓殑鏌愪簺瀛楁涓庤繖浜涘璞＄殑鐗瑰畾浜庢€荤嚎鐨勮〃绀轰腑鐨勫瓧娈甸噸澶嶃€傚彲浠ラ殢鎰忕Щ闄ょ壒瀹氫簬鎬荤嚎鐨勫瓧娈碉紝杞€屼娇鐢ㄩ€氱敤瀛楁銆備笉杩囪娉ㄦ剰锛岃繖寰堝彲鑳芥剰鍛崇潃瑕佷慨澶嶆墍鏈夊紩鐢ㄤ簡杩欎簺鐗瑰畾浜庢€荤嚎鐨勫瓧娈电殑椹卞姩锛堝敖绠¤繖浜涘簲璇ラ兘鍙槸涓€琛屾敼鍔級銆?