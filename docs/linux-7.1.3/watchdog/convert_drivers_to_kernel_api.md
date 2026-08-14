## 灏嗘棫鐨勭湅闂ㄧ嫍锛坵atchdog锛夐┍鍔ㄨ浆鎹㈠埌鐪嬮棬鐙楁鏋?

浣滆€咃細Wolfram Sang <wsa@kernel.org>

闅忕潃鐪嬮棬鐙楁鏋讹紙watchdog framework锛夎繘鍏ュ唴鏍革紝杩囧幓姣忎釜椹卞姩閮借嚜琛屽疄鐜?API 鐨勫眬闈㈠凡缁忔敼鍙樸€傚浠婏紝妗嗘灦宸插皢鍏叡缁勪欢鎶藉彇鍑烘潵锛岄┍鍔ㄥ緱浠ョ簿绠€锛岀敤鎴峰彲浠ョ洿鎺ヤ娇鐢ㄦ鏋躲€傛湰鏂囨。灏嗘寚瀵间綘瀹屾垚杩欎竴杞崲宸ヤ綔锛屾弿杩板繀瑕佺殑姝ラ浠ュ強闇€瑕佺暀鎰忕殑鍦版柟銆?

### 绉婚櫎 file_operations 缁撴瀯浣?

鏃х殑椹卞姩浼氬畾涔夎嚜宸辩殑 file_operations 鎿嶄綔锛屼緥濡?open()銆亀rite() 绛夛紝鑰岀幇鍦ㄨ繖浜涘ぇ澶氱敱妗嗘灦澶勭悊锛屾鏋跺彧鍦ㄩ渶瑕佹椂璋冪敤椹卞姩銆傚洜姝わ紝涓€鑸€岃█锛?file_operations' 缁撴瀯浣撳強鍏剁浉鍏冲嚱鏁板彲浠ョЩ闄わ紝鍙湁鏋佸皯鏁伴┍鍔ㄧ壒瀹氱殑缁嗚妭闇€瑕佺Щ鍒扮浉搴斿嚱鏁颁腑銆備笅闈㈡杩板悇鍑芥暟鍙兘闇€瑕佽繘琛岀殑鎿嶄綔锛?

- open锛氭墍鏈夋秹鍙婅祫婧愮鐞嗭紙鏂囦欢鎵撳紑妫€鏌ャ€乵agic close 鐨勫噯澶囧伐浣滐級鐨勫唴瀹圭洿鎺ュ垹闄ゅ嵆鍙€傝澶囩壒瀹氱殑閮ㄥ垎闇€瑕佺Щ鍒伴┍鍔ㄧ殑 start 鍑芥暟涓€傚浜庢煇浜涢┍鍔紝start 鍑芥暟鍚屾椂涔熷厖褰?ping 鍑芥暟銆傚鏋滈渶瑕?start/stop 淇濇寔骞宠　锛堝挨鍏舵槸娑夊強鏃堕挓鏃讹級锛屾渶濂藉皢鍏堕噸鏋勪负鐙珛鐨?start 鍑芥暟銆?
- release锛氫笌 open 鐩稿悓鐨勬彁绀洪€傜敤銆?
- write锛氱洿鎺ュ垹闄ゅ嵆鍙紝妗嗘灦浼氳礋璐ｅ畾涔夊ソ鐨勮涓猴紝鍗冲鐞嗗啓鍏?magic 瀛楃锛?V'锛夌殑 ping 鎿嶄綔銆?
- ioctl锛氶┍鍔ㄤ粛鐒跺厑璁告墿灞?IOCTL 鎺ュ彛锛屼絾鏈€甯歌鐨勯偅浜涘凡鐢辨鏋跺鐞嗭紝鍙渶椹卞姩鎻愪緵鍗忓姪锛?

  WDIOC_GETSUPPORT锛氳繑鍥為┍鍔ㄥ繀椤绘彁渚涚殑 watchdog_info 缁撴瀯浣撱€?
  WDIOC_GETSTATUS锛氶渶瑕佸畾涔?status 鍥炶皟锛屽惁鍒欒繑鍥?0銆?
  WDIOC_GETBOOTSTATUS锛氶渶瑕佹纭缃?bootstatus 鎴愬憳銆傝纭繚涓嶈鍥犫€滀笉鍐嶆敮鎸佲€濊€岄敊璇湴灏?0 鍐欏叆鍏朵腑锛?
  WDIOC_SETOPTIONS锛氶渶瑕佸仛涓€浜涘噯澶囧伐浣溿€?
  WDIOC_KEEPALIVE锛氬闇€瑕侊紝watchdog_info 蹇呴』璁剧疆 WDIOF_KEEPALIVEPING 鏍囧織銆?
  WDIOC_SETTIMEOUT锛歸atchdog_info 闇€瑕佽缃?WDIOF_SETTIMEOUT 鏍囧織锛屽苟瀹氫箟 set_timeout 鍥炶皟銆傛牳蹇冨眰浼氳繘琛岃寖鍥存鏌ワ紝骞惰姹傝缃?min_timeout 涓?max_timeout銆傝鍥炶皟鏄彲閫夌殑銆?
  WDIOC_GETTIMEOUT锛氶渶瑕佸仛涓€浜涘噯澶囧伐浣溿€?
  WDIOC_GETTIMELEFT锛氶渶瑕佸畾涔?get_timeleft() 鍥炶皟锛屽惁鍒欒繑鍥?EOPNOTSUPP銆?

閫氳繃 ioctl 鍥炶皟鏉ュ鐞嗛偅浜涙鏋舵湭鎻愪緵鐨?IOCTL銆傞渶瑕佹敞鎰忕殑鏄紝璇ユ満鍒朵富瑕侀潰鍚戠Щ妞嶆棫椹卞姩锛涙柊鐨勯┍鍔ㄤ笉搴斿彂鏄庣鏈夌殑 IOCTL銆傜鏈?IOCTL 浼氳浼樺厛澶勭悊銆傚鏋滃洖璋冭繑鍥?-ENOIOCTLCMD锛屾鏋朵篃浼氬皾璇曞鐞嗚 IOCTL銆傚嚭鐜伴敊璇椂鐩存帴杩斿洖缁欑敤鎴峰嵆鍙€?

```
  -static const struct file_operations s3c2410wdt_fops = {
  -       .owner          = THIS_MODULE,
  -       .write          = s3c2410wdt_write,
  -       .unlocked_ioctl = s3c2410wdt_ioctl,
  -       .open           = s3c2410wdt_open,
  -       .release        = s3c2410wdt_release,
  -};
```

妫€鏌ュ悇鍑芥暟锛屽皢璁惧鐗瑰畾鐨勫唴瀹逛繚鐣欎笅鏉ヤ緵鍚庣画閲嶆瀯锛屽叾浣欓儴鍒嗗垹闄ゃ€?

### 绉婚櫎 miscdevice

鐢变簬 file_operations 宸茬Щ闄わ紝鐜板湪涔熷簲绉婚櫎 'miscdevice' 缁撴瀯浣撱€傛鏋朵細鍦?watchdog_dev_register() 琚皟鐢ㄦ椂鑷姩鍒涘缓璁惧锛?

```
  -static struct miscdevice s3c2410wdt_miscdev = {
  -       .minor          = WATCHDOG_MINOR,
  -       .name           = "watchdog",
  -       .fops           = &s3c2410wdt_fops,
  -};
```

### 绉婚櫎杩囨椂鐨?include 涓庡畾涔?

缁忚繃涓婅堪绠€鍖栧悗锛屽皯鏁板畾涔夌幇鍦ㄥ彲鑳藉凡涓嶅啀浣跨敤锛屽彲浠ョЩ闄わ細

```
  - #include <linux/fs.h>
  - #include <linux/miscdevice.h> (if MODULE_ALIAS_MISCDEV is not used)
  - #include <linux/uaccess.h> (if no custom IOCTLs are used)
```

### 娣诲姞 watchdog 鎿嶄綔

鍙互鍦?'watchdog_ops' 缁撴瀯浣撲腑瀹氫箟鍙敤鐨勫洖璋冿紝鍏惰缁嗚鏄庤 'watchdog-鍐呮牳-鎺ュ彛.txt'銆傞櫎 start() 涓?owner 蹇呴』璁剧疆澶栵紝鍏朵綑鍧囦负鍙€夈€備綘鍙互寰堝鏄撳湴鍦ㄦ棫椹卞姩涓壘鍒板搴旂殑鍑芥暟銆傝娉ㄦ剰锛岀幇鍦ㄥ嚱鏁颁細鏀跺埌鎸囧悜 watchdog_device 鐨勬寚閽堜綔涓哄弬鏁帮紝鍥犳鍙兘闇€瑕佷慨鏀瑰嚱鏁扮鍚嶃€傚ぇ澶氭暟鎯呭喌涓嬶紝杩欑被鏀瑰姩鍙槸鍥犱负鐩存帴杩涜浜嗙‖浠惰闂€傝澶囩壒瀹氱殑浠ｇ爜鐣欏湪鍚勬楠や腑锛岃閲嶆瀯涓哄洖璋冦€?

```
  +static struct watchdog_ops s3c2410wdt_ops = {
  +       .owner = THIS_MODULE,
  +       .start = s3c2410wdt_start,
  +       .stop = s3c2410wdt_stop,
  +       .ping = s3c2410wdt_keepalive,
  +       .set_timeout = s3c2410wdt_set_heartbeat,
  +};
```

```
  -static void s3c2410wdt_keepalive(void)
  +static int s3c2410wdt_keepalive(struct watchdog_device *wdd)
   {
  ...
  +
  +       return 0;
   }

  ...

  -       s3c2410wdt_keepalive();
  +       s3c2410wdt_keepalive(&s3c2410_wdd);
```

### 娣诲姞 watchdog 璁惧

鐜板湪闇€瑕佸垱寤?'watchdog_device' 缁撴瀯浣擄紝骞跺～鍏呮鏋舵墍闇€鐨勫繀瑕佷俊鎭€傝缁撴瀯浣撳湪 'watchdog-鍐呮牳-鎺ュ彛.txt' 涓湁璇︾粏璇存槑銆傚繀椤讳紶鍏ユ柊寤虹殑 watchdog_ops 浠ュ強 watchdog_info 缁撴瀯浣撱€傞€氬父锛屾棫椹卞姩浼氫娇鐢ㄩ潤鎬佸彉閲忔潵璁板綍 bootstatus銆乼imeout 绛変俊鎭紝鐜板湪搴旀敼鐢?watchdog_device 鐨勫搴旀垚鍛樸€傝娉ㄦ剰锛宼imeout 鍊间负 unsigned int 绫诲瀷锛涘鏋滈┍鍔ㄥ師鏉ヤ娇鐢?signed int锛屼篃闇€瑕佷竴骞惰浆鎹€?

```
  +static struct watchdog_device s3c2410_wdd = {
  +       .info = &s3c2410_wdt_ident,
  +       .ops = &s3c2410wdt_ops,
  +};
```

### 澶勭悊 'nowayout' 鐗规€?

灏戞暟椹卞姩闈欐€佸湴浣跨敤 nowayout锛屽嵆鐢辨ā鍧楀弬鏁?CONFIG_WATCHDOG_NOWAYOUT 鍐冲畾璇ョ壒鎬ф槸鍚﹀惎鐢ㄣ€傞渶瑕佸皢鍏惰浆鎹负瀵圭姸鎬佸彉閲忕殑鍒濆鍖栵細

```
        .status = WATCHDOG_NOWAYOUT_INIT_STATUS,
```

涓嶈繃锛屽ぇ澶氭暟椹卞姩鍏佽鍦ㄨ繍琛屾椂閰嶇疆 nowayout锛岄€氬父濡備笅锛?

```
	watchdog_set_nowayout(&s3c2410_wdd, nowayout);
```

妯″潡鍙傛暟鏈韩闇€瑕佷繚鐣欙紝浣嗕笌 nowayout 鐩稿叧鐨勫叾浣欎唬鐮侊紙寰堝彲鑳戒綅浜?open()銆乺elease()銆亀rite() 涓級閮藉彲浠ュ垹闄ゃ€?

### 娉ㄥ唽 watchdog 璁惧

灏?misc_register(&miscdev) 鏇挎崲涓?watchdog_register_device(&watchdog_dev)銆傝纭繚妫€鏌ヨ繑鍥炲€煎苟缁欏嚭閿欒娑堟伅锛堝鏈夛級锛?

```
  -       ret = misc_register(&s3c2410wdt_miscdev);
  +       ret = watchdog_register_device(&s3c2410_wdd);

  ...

  -       misc_deregister(&s3c2410wdt_miscdev);
  +       watchdog_unregister_device(&s3c2410_wdd);
```

### 鏇存柊 Kconfig 椤?

璇ラ┍鍔ㄧ幇鍦ㄩ渶瑕侀€夋嫨 WATCHDOG_CORE锛?

- 閫夋嫨 WATCHDOG_CORE

### 鍒涘缓琛ヤ竵骞跺彂閫佷笂娓?

鍦ㄥ彂閫佽ˉ涓佸墠锛岃鍔″繀闃呰 鏂囨。/杩涚▼/submitting-patches.rst锛屽苟鍙戦€佸埌 linux-watchdog@vger.kernel.org銆傛湡寰呬綘鐨勮础鐚?:)
