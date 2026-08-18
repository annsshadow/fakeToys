
## sysfs 鈥斺€?鐢ㄤ簬瀵煎嚭鍐呮牳瀵硅薄鐨勬枃浠剁郴缁?

Patrick Mochel	<mochel@osdl.org>

Mike Murphy <mamurph@cs.clemson.edu>

:Revised:    16 August 2011
:Original:   10 January 2003


#### 瀹冩槸浠€涔?

sysfs 鏄竴涓熀浜?RAM 鐨勬枃浠剁郴缁燂紝鏈€鍒濆熀浜?ramfs銆傚畠鎻愪緵浜嗕竴绉嶅皢鍐呮牳鏁版嵁缁撴瀯銆佸叾灞炴€т互鍙婂畠浠箣闂寸殑閾炬帴瀵煎嚭鍒扮敤鎴风┖闂寸殑鏂瑰紡銆?
sysfs 鏈川涓婁笌 kobject 鍩虹璁炬柦缁戝畾銆傛湁鍏?kobject 鎺ュ彛鐨勬洿澶氫俊鎭紝璇烽槄璇?Documentation/core-api/kobject.rst銆?

#### 浣跨敤 sysfs


濡傛灉瀹氫箟浜?CONFIG_SYSFS锛宻ysfs 鎬绘槸琚紪璇戣繘鍐呮牳銆備綘鍙互閫氳繃浠ヤ笅鏂瑰紡璁块棶

```

    mount -t sysfs sysfs /sys


```
#### 鐩綍鐨勫垱寤?

瀵逛簬绯荤粺涓敞鍐岀殑姣忎釜 kobject锛岄兘浼氬湪 sysfs 涓负鍏跺垱寤轰竴涓洰褰曘€傝鐩綍浣滀负 kobject 鐖剁洰褰曠殑瀛愮洰褰曞垱寤猴紝浠庤€屽悜鐢ㄦ埛绌洪棿琛ㄨ揪鍐呴儴瀵硅薄灞傜骇銆俿ysfs 涓殑椤跺眰鐩綍琛ㄧず瀵硅薄灞傜骇鐨勫叡鍚岀鍏堬紱鍗冲璞℃墍灞炵殑瀛愮郴缁熴€?
sysfs 鍦ㄤ笌璇ョ洰褰曞叧鑱旂殑 kernfs_node 瀵硅薄鍐呴儴瀛樺偍涓€涓寚鍚戝疄鐜拌鐩綍鐨?kobject 鐨勬寚閽堛€傝繃鍘伙紝sysfs 鍦ㄦ枃浠舵墦寮€鎴栧叧闂椂鐩存帴浣跨敤姝?kobject 鎸囬拡瀵?kobject 杩涜寮曠敤璁℃暟銆傚湪褰撳墠 sysfs 瀹炵幇涓紝kobject 寮曠敤璁℃暟浠呯敱 sysfs_schedule_callback() 鍑芥暟鐩存帴淇敼銆?

#### 灞炴€?

鍙互浠ユ枃浠剁郴缁熶腑鏅€氭枃浠剁殑褰㈠紡涓?kobject 瀵煎嚭灞炴€с€俿ysfs 灏嗘枃浠?I/O 鎿嶄綔杞彂缁欎负灞炴€у畾涔夌殑鏂规硶锛屼粠鑰屾彁渚涗竴绉嶈鍐欏唴鏍稿睘鎬х殑鎵嬫銆?
灞炴€у簲涓?ASCII 鏂囨湰鏂囦欢锛屾渶濂芥瘡涓枃浠跺彧鏈変竴涓€笺€傞渶瑕佹敞鎰忕殑鏄紝姣忎釜鏂囦欢鍙寘鍚竴涓€煎彲鑳芥晥鐜囦笉楂橈紝鍥犳琛ㄨ揪鍚屼竴绫诲瀷鐨勬暟鍊兼暟缁勫湪绀句細灞傞潰涓婃槸鍙帴鍙楃殑銆?
娣峰悎绫诲瀷銆佽〃杈惧琛屾暟鎹互鍙婅姳鍝ㄥ湴鏍煎紡鍖栨暟鎹槸琚己鐑堝弽瀵圭殑銆傚仛杩欎簺浜嬫儏鍙兘浼氳浣犲綋浼楀嚭涓戯紝骞朵笖浣犵殑浠ｇ爜浼氬湪鏈€氱煡鐨勬儏鍐典笅琚噸鍐欍€?
```

    struct attribute {
	    char                    *name;
	    struct module           *owner;
	    umode_t                 mode;
    };


    int sysfs_create_file(struct kobject * kobj, const struct attribute * attr);
    void sysfs_remove_file(struct kobject * kobj, const struct attribute * attr);


```
涓€涓８灞炴€т笉鍖呭惈璇绘垨鍐欏睘鎬у€肩殑鎵嬫銆傞紦鍔卞瓙绯荤粺瀹氫箟鑷繁鐨勫睘鎬х粨鏋勪互鍙婄敤浜庝负鐗瑰畾瀵硅薄绫诲瀷娣诲姞鍜岀Щ闄ゅ睘鎬х殑鍖呰鍑芥暟銆?
```

    struct device_attribute {
	    struct attribute	attr;
	    ssize_t (*show)(struct device *dev, struct device_attribute *attr,
			    char *buf);
	    ssize_t (*store)(struct device *dev, struct device_attribute *attr,
			    const char *buf, size_t count);
    };

    int device_create_file(struct device *, const struct device_attribute *);
    void device_remove_file(struct device *, const struct device_attribute *);

```
```

    #define DEVICE_ATTR(_name, _mode, _show, _store) \
    struct device_attribute dev_attr_##_name = __ATTR(_name, _mode, _show, _store)

```
```

    static DEVICE_ATTR(foo, S_IWUSR | S_IRUGO, show_foo, store_foo);

```
```

    static struct device_attribute dev_attr_foo = {
	    .attr = {
		    .name = "foo",
		    .mode = S_IWUSR | S_IRUGO,
	    },
	    .show = show_foo,
	    .store = store_foo,
    };

```
娉ㄦ剰锛屽 include/linux/sysfs.h 涓墍杩帮紝鈥淥THER_WRITABLE锛熼€氬父琚涓烘槸涓潖涓绘剰銆傗€濆洜姝ゅ皾璇曞皢 sysfs 鏂囦欢璁剧疆涓哄鎵€鏈変汉閮藉彲鍐欎細澶辫触锛屽苟鍥為€€涓哄鈥淥thers鈥濈殑鍙妯″紡銆?
瀵逛簬甯歌鎯呭喌锛宻ysfs.h 鎻愪緵浜嗕究鍒╁畯锛屼娇瀹氫箟灞炴€ф洿瀹规槗锛屽悓鏃惰浠ｇ爜鏇寸畝娲佹槗璇汇€備笂杩版儏鍐靛彲绠€鍐欎负锛?
static struct device_attribute dev_attr_foo = __ATTR_RW(foo);

鍙敤浜庡畾涔変綘鍖呰鍑芥暟鐨勮緟鍔╁畯鍒楄〃濡備笅锛?
__ATTR_RO(name)锛?		 鍋囪榛樿 name_show 涓旀ā寮忎负 0444
__ATTR_WO(name)锛?		 鍋囪鍙湁 name_store锛屽苟闄愬埗涓烘ā寮?0200锛屽嵆浠?root 鍙啓銆?__ATTR_RO_MODE(name, mode)锛?	         鐢ㄤ簬鏇翠弗鏍肩殑鍙璁块棶锛涚洰鍓嶅敮涓€鐢ㄤ緥鏄?EFI 绯荤粺璧勬簮琛?	         锛堣 drivers/firmware/efi/esrt.c锛?__ATTR_RW(name)锛?	         鍋囪榛樿 name_show銆乶ame_store锛屽苟灏嗘ā寮忚涓?0644銆?__ATTR_NULL锛?	         灏嗗悕绉拌涓?NULL锛岀敤浣滃垪琛ㄧ粨鏉熸寚绀虹锛堣锛歬ernel/workqueue.c锛?
#### 瀛愮郴缁熺壒瀹氱殑鍥炶皟


褰撳瓙绯荤粺瀹氫箟鏂扮殑灞炴€х被鍨嬫椂锛屽畠蹇呴』瀹炵幇涓€缁?sysfs 鎿嶄綔锛岀敤浜庡皢璇?鍐欒皟鐢ㄨ浆鍙戠粰

```

    struct sysfs_ops {
	    ssize_t (*show)(struct kobject *, struct attribute *, char *);
	    ssize_t (*store)(struct kobject *, struct attribute *, const char *, size_t);
    };

```
[ 瀛愮郴缁熷簲褰撳凡缁忓畾涔変簡涓€涓?struct kobj_type 浣滀负璇ョ被鍨嬬殑鎻忚堪绗︼紝sysfs_ops 鎸囬拡灏卞瓨鍌ㄤ簬姝ゃ€傛洿澶氫俊鎭鍙傝 kobject 鏂囨。銆?]

褰撴枃浠惰璇绘垨鍐欐椂锛宻ysfs 浼氳皟鐢ㄨ绫诲瀷鐨勯€傚綋鏂规硶銆傝鏂规硶闅忓悗灏嗛€氱敤鐨?struct kobject 涓?struct attribute 鎸囬拡杞崲涓洪€傚綋鐨勬寚閽堢被鍨嬶紝骞惰皟鐢ㄥ叧鑱旂殑鏂规硶銆?
```

    #define to_dev_attr(_attr) container_of(_attr, struct device_attribute, attr)

    static ssize_t dev_attr_show(struct kobject *kobj, struct attribute *attr,
				char *buf)
    {
	    struct device_attribute *dev_attr = to_dev_attr(attr);
	    struct device *dev = kobj_to_dev(kobj);
	    ssize_t ret = -EIO;

	    if (dev_attr->show)
		    ret = dev_attr->show(dev, dev_attr, buf);
	    if (ret >= (ssize_t)PAGE_SIZE) {
		    printk("dev_attr_show: %pS returned bad count\n",
				    dev_attr->show);
	    }
	    return ret;
    }



```
#### 璇?鍐欏睘鎬ф暟鎹?

瑕佽鎴栧啓灞炴€э紝蹇呴』鍦ㄥ０鏄庡睘鎬ф椂鎸囧畾 show() 鎴?store() 鏂规硶銆傛柟娉曠被鍨嬪簲濡備笅

```

    ssize_t (*show)(struct device *dev, struct device_attribute *attr, char *buf);
    ssize_t (*store)(struct device *dev, struct device_attribute *attr,
		    const char *buf, size_t count);

```
鎹㈣█涔嬶紝瀹冧滑搴斿彧鎺ュ彈涓€涓璞°€佷竴涓睘鎬у拰涓€涓紦鍐插尯浣滀负鍙傛暟銆?
sysfs 鍒嗛厤涓€涓ぇ灏忎负 (PAGE_SIZE) 鐨勭紦鍐插尯骞朵紶閫掔粰鏂规硶銆俿ysfs 瀵规瘡娆¤鎴栧啓鍙皟鐢ㄨ鏂规硶涓€娆°€傝繖寮哄埗鏂规硶瀹炵幇閬靛惊浠ヤ笅琛屼负锛?
- 鍦?read(2) 鏃讹紝show() 鏂规硶搴斿～婊℃暣涓紦鍐插尯銆傚洖鎯充竴涓嬶紝涓€涓睘鎬у彧搴斿鍑轰竴涓€兼垨涓€缁勭浉浼煎€硷紝鍥犳杩欎笉搴斿お鏄傝吹銆傝繖鍏佽鐢ㄦ埛绌洪棿杩涜閮ㄥ垎璇诲彇锛屽苟鍦ㄦ暣涓枃浠朵笂闅忔剰鍓嶅悜瀹氫綅銆傚鏋滅敤鎴风┖闂村畾浣嶅洖闆舵垨鐢ㄥ亸绉婚噺 '0' 杩涜 pread(2)锛宻how() 鏂规硶浼氳鍐嶆璋冪敤锛堥噸鏂拌濉級浠ュ～鍏呯紦鍐插尯銆?
  杩欏厑璁哥敤鎴风┖闂磋繘琛岄儴鍒嗚鍙栧苟鍦ㄦ暣涓枃浠朵笂闅忔剰鍓嶅悜瀹氫綅銆傚鏋滅敤鎴风┖闂村畾浣嶅洖闆舵垨鐢ㄥ亸绉婚噺 '0' 杩涜 pread(2)锛宻how() 鏂规硶浼氳鍐嶆璋冪敤锛岄噸鏂拌濉紝浠ュ～鍏呯紦鍐插尯銆?
- 鍦?write(2) 鏃讹紝sysfs 鏈熸湜鍦ㄧ涓€娆″啓鍏ユ椂浼犻€掓暣涓紦鍐插尯銆俿ysfs 闅忓悗灏嗘暣涓紦鍐插尯浼犻€掔粰 store() 鏂规硶銆傚湪 store 鏃讹紝鏁版嵁涔嬪悗浼氭坊鍔犱竴涓粓姝㈢┖瀛楃銆傝繖浣垮緱 sysfs_streq() 绛夊嚱鏁板彲浠ュ畨鍏ㄤ娇鐢ㄣ€?
  鍐欏叆 sysfs 鏂囦欢鏃讹紝鐢ㄦ埛绌洪棿杩涚▼搴旈鍏堣鍙栨暣涓枃浠讹紝淇敼鍏跺笇鏈涙敼鍙樼殑鍊硷紝鐒跺悗灏嗘暣涓紦鍐插尯鍐欏洖銆?
  灞炴€х殑鏂规硶瀹炵幇鍦ㄨ鍐欏€兼椂搴斿湪鐩稿悓鐨勭紦鍐插尯涓婃搷浣溿€?
鍏朵粬娉ㄦ剰浜嬮」锛?
- 鍐欏叆浼氬鑷?show() 鏂规硶琚噸鏂拌濉紝鏃犺褰撳墠鏂囦欢浣嶇疆濡備綍銆?
- 缂撳啿鍖洪暱搴﹀缁堜负 PAGE_SIZE 瀛楄妭銆傚湪 x86 涓婏紝杩欐槸 4096銆?
- show() 鏂规硶搴旇繑鍥炴墦鍗板埌缂撳啿鍖轰腑鐨勫瓧鑺傛暟銆?
- show() 鏂规硶鐨勬柊瀹炵幇鍦ㄦ牸寮忓寲瑕佽繑鍥炵粰鐢ㄦ埛绌洪棿鐨勫€兼椂锛屽簲鍙娇鐢?sysfs_emit() 鎴?sysfs_emit_at()銆?
- store() 搴旇繑鍥炰粠缂撳啿鍖轰腑浣跨敤鐨勫瓧鑺傛暟銆傚鏋滄暣涓紦鍐插尯閮藉凡琚娇鐢紝鐩存帴杩斿洖 count 鍙傛暟鍗冲彲銆?
- show() 鎴?store() 鎬绘槸鍙互杩斿洖閿欒銆傚鏋滀紶鍏ヤ簡閿欒鐨勫€硷紝鍔″繀杩斿洖涓€涓敊璇€?
- 浼犻€掔粰鏂规硶鐨勫璞′細閫氳繃 sysfs 瀵瑰叾鍐呭祵瀵硅薄鐨勫紩鐢ㄨ鏁拌鍥哄畾鍦ㄥ唴瀛樹腑銆傜劧鑰岋紝璇ュ璞℃墍浠ｈ〃鐨勭墿鐞嗗疄浣擄紙渚嬪璁惧锛夊彲鑳藉苟涓嶅瓨鍦ㄣ€傚鏈夊繀瑕侊紝鍔″繀鏈夊姙娉曞姝よ繘琛屾鏌ャ€?
```

    static ssize_t show_name(struct device *dev, struct device_attribute *attr,
			    char *buf)
    {
	    return sysfs_emit(buf, "%s\n", dev->name);
    }

    static ssize_t store_name(struct device *dev, struct device_attribute *attr,
			    const char *buf, size_t count)
    {
	    snprintf(dev->name, sizeof(dev->name), "%.*s",
		    (int)min(count, sizeof(dev->name) - 1), buf);
	    return count;
    }

    static DEVICE_ATTR(name, S_IRUGO, show_name, store_name);


```
锛堟敞鎰忥紝鐪熷疄瀹炵幇涓嶅厑璁哥敤鎴风┖闂磋缃澶囩殑鍚嶇О銆傦級


#### 椤跺眰鐩綍甯冨眬


sysfs 鐨勭洰褰曟帓鍒楀睍鐜颁簡鍐呮牳鏁版嵁缁撴瀯涔嬮棿鐨勫叧绯汇€?
```

    block/
    bus/
    class/
    dev/
    devices/
    firmware/
    fs/
    hypervisor/
    kernel/
    module/
    power/

```
devices/ 鍖呭惈璁惧鏍戝湪鏂囦欢绯荤粺涓殑琛ㄧず銆傚畠鐩存帴鏄犲皠鍒板唴閮ㄥ唴鏍歌澶囨爲锛屽嵆 struct device 鐨勫眰绾х粨鏋勩€?
bus/ 鍖呭惈绯荤粺涓悇绉嶆€荤嚎绫诲瀷鐨勬墎骞崇洰褰曞竷灞€锛?
```

	devices/
	drivers/

```
devices/ 鍖呭惈涓虹郴缁熶腑鍙戠幇鐨勬瘡涓澶囨墍寤虹珛鐨勬寚鍚戝叾鍦?/sys/devices 涓嬬洰褰曠殑绗﹀彿閾炬帴銆?
drivers/ 鍖呭惈涓鸿鐗瑰畾鎬荤嚎涓婄殑璁惧鎵€鍔犺浇鐨勬瘡涓澶囬┍鍔ㄥ搴斾竴涓洰褰曪紙杩欏亣璁鹃┍鍔ㄤ笉浼氳法瓒婂绉嶆€荤嚎绫诲瀷锛夈€?
fs/ 鍖呭惈鏌愪簺鏂囦欢绯荤粺鐨勭洰褰曘€傜洰鍓嶆瘡涓笇鏈涘鍑哄睘鎬х殑鏂囦欢绯荤粺蹇呴』鍦?fs/ 涓嬪垱寤鸿嚜宸辩殑灞傜骇锛堢ず渚嬭 fuse/fuse.rst锛夈€?
module/ 鍖呭惈鎵€鏈夊凡鍔犺浇绯荤粺妯″潡锛堝寘鎷唴寤轰笌鍙姞杞芥ā鍧楋級鐨勫弬鏁板€间笌鐘舵€佷俊鎭€?
dev/ 鍖呭惈涓や釜鐩綍锛歝har/ 涓?block/銆傝繖涓や釜鐩綍鍐呮湁鍚嶄负 <major>:<minor> 鐨勭鍙烽摼鎺ャ€傝繖浜涚鍙烽摼鎺ユ寚鍚戞瘡涓澶囧湪 /sys/devices 涓嬬殑鐩綍銆?sys/dev 鎻愪緵浜嗕竴绉嶄粠 stat(2) 鎿嶄綔缁撴灉蹇€熸煡鎵捐澶?sysfs 鎺ュ彛鐨勬柟寮忋€?
鏈夊叧椹卞姩妯″瀷鐗瑰畾鐗规€х殑鏇村淇℃伅鍙湪 Documentation/driver-api/driver-model/ 涓壘鍒般€?
block/ 鍖呭惈鎸囧悜绯荤粺涓婂彂鐜扮殑鎵€鏈夊潡璁惧鐨勭鍙烽摼鎺ャ€傝繖浜涚鍙烽摼鎺ユ寚鍚?/sys/devices 涓嬬殑鐩綍銆?
class/ 鍖呭惈鎸夊姛鑳界被鍨嬪垎缁勭殑姣忎釜璁惧绫荤殑鐩綍銆俢lass/ 涓殑姣忎釜鐩綍鍖呭惈鎸囧悜 /sys/devices 鐩綍涓澶囩殑绗﹀彿閾炬帴銆?
firmware/ 鍖呭惈绯荤粺鍥轰欢鏁版嵁涓庨厤缃紝渚嬪鍥轰欢琛ㄣ€丄CPI 淇℃伅涓庤澶囨爲鏁版嵁銆?
hypervisor/ 鍖呭惈铏氭嫙鍖栧钩鍙颁俊鎭紝骞舵彁渚涘埌搴曞眰 hypervisor 鐨勬帴鍙ｃ€備粎鍦ㄨ繍琛屼簬铏氭嫙鏈轰笂鏃跺瓨鍦ㄣ€?
kernel/ 鍖呭惈杩愯鏃跺唴鏍稿弬鏁般€侀厤缃缃笌鐘舵€併€?
power/ 鍖呭惈鐢垫簮绠＄悊瀛愮郴缁熶俊鎭紝鍖呮嫭鐫＄湢鐘舵€併€佹寕璧?鎭㈠鑳藉姏浠ュ強绛栫暐銆?

#### 褰撳墠鐨勬帴鍙?

sysfs 褰撳墠瀛樺湪浠ヤ笅鎺ュ彛灞傘€?

### devices (include/linux/device.h)

```

    struct device_attribute {
	    struct attribute	attr;
	    ssize_t (*show)(struct device *dev, struct device_attribute *attr,
			    char *buf);
	    ssize_t (*store)(struct device *dev, struct device_attribute *attr,
			    const char *buf, size_t count);
    };

```
```

    DEVICE_ATTR(_name, _mode, _show, _store);

```
```

    int device_create_file(struct device *dev, const struct device_attribute * attr);
    void device_remove_file(struct device *dev, const struct device_attribute * attr);


```
### bus drivers (include/linux/device.h)

```

    struct bus_attribute {
	    struct attribute        attr;
	    ssize_t (*show)(const struct bus_type *, char * buf);
	    ssize_t (*store)(const struct bus_type *, const char * buf, size_t count);
    };

```
```

    static BUS_ATTR_RW(name);
    static BUS_ATTR_RO(name);
    static BUS_ATTR_WO(name);

```
```

    int bus_create_file(struct bus_type *, struct bus_attribute *);
    void bus_remove_file(struct bus_type *, struct bus_attribute *);


```
### device drivers (include/linux/device.h)


```

    struct driver_attribute {
	    struct attribute        attr;
	    ssize_t (*show)(struct device_driver *, char * buf);
	    ssize_t (*store)(struct device_driver *, const char * buf,
			    size_t count);
    };

```
```

    DRIVER_ATTR_RO(_name)
    DRIVER_ATTR_RW(_name)

```
```

    int driver_create_file(struct device_driver *, const struct driver_attribute *);
    void driver_remove_file(struct device_driver *, const struct driver_attribute *);


```
#### 鏂囨。


sysfs 鐩綍缁撴瀯浠ュ強姣忎釜鐩綍涓殑灞炴€у畾涔変簡鍐呮牳涓庣敤鎴风┖闂翠箣闂寸殑 ABI銆傚浜庝换浣?ABI 鑰岃█锛岃 ABI 淇濇寔绋冲畾骞跺緱鍒板Ε鍠勬枃妗ｅ寲閮藉緢閲嶈銆傛墍鏈夋柊鐨?sysfs 灞炴€у繀椤诲湪 Documentation/ABI 涓褰曘€傛洿澶氫俊鎭彟瑙?Documentation/ABI/README銆?