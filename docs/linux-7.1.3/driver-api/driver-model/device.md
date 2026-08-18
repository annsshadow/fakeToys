## 鍩烘湰璁惧缁撴瀯浣?

璇峰弬闃?struct device 鐨?kerneldoc銆?

#### 缂栫▼鎺ュ彛

鍙戠幇璇ヨ澶囩殑鎬荤嚎椹卞姩浣跨敤姝ゆ帴鍙ｆ潵娉ㄥ唽

```
  int device_register(struct device * dev);

```
鎬荤嚎搴斿綋鍒濆鍖栦互涓嬪瓧娈碉細

    - parent
    - name
    - bus_id
    - bus

褰撳紩鐢ㄨ鏁伴檷涓轰互涓嬪€兼椂锛岃澶囧皢浠庢牳蹇冧腑绉婚櫎

```
  struct device * get_device(struct device * dev);
  void put_device(struct device * dev);

```
濡傛灉寮曠敤璁℃暟杩樹笉鏄?0锛堝嵆璁惧姝ｅ湪琚Щ闄ょ殑杩囩▼涓級锛実et_device() 灏嗚繑鍥?浼犲叆鐨?struct device 鎸囬拡銆?
```
  void lock_device(struct device * dev);
  void unlock_device(struct device * dev);

```
#### 灞炴€?

```
  struct device_attribute {
	struct attribute	attr;
	ssize_t (*show)(struct device *dev, struct device_attribute *attr,
			char *buf);
	ssize_t (*store)(struct device *dev, struct device_attribute *attr,
			 const char *buf, size_t count);
  };

```
璁惧鐨勫睘鎬у彲浠ョ敱璁惧椹卞姩閫氳繃 sysfs 瀵煎嚭銆?
璇峰弬闃?Documentation/filesystems/sysfs.rst 浠ヤ簡瑙ｆ洿澶氬叧浜?sysfs
宸ヤ綔鍘熺悊鐨勪俊鎭€?
濡?Documentation/core-api/kobject.rst 鎵€瑙ｉ噴锛岃澶囧睘鎬у繀椤诲湪鐢熸垚
KOBJ_ADD uevent 涔嬪墠鍒涘缓銆傚疄鐜拌繖涓€鐐圭殑鍞竴鏂瑰紡鏄畾涔変竴涓睘鎬х粍銆?
```
  #define DEVICE_ATTR(name,mode,show,store)

```
```
  static DEVICE_ATTR(type, 0444, type_show, NULL);
  static DEVICE_ATTR(power, 0644, power_show, power_store);

```
瀵逛簬 mode 鐨勫父瑙佸彇鍊硷紝鎻愪緵浜嗚緟鍔╁畯锛屽洜姝や笂杩扮ず渚嬪彲浠ユ敼鍐欎负

```
  static DEVICE_ATTR_RO(type);
  static DEVICE_ATTR_RW(power);

```
杩欎細澹版槑涓や釜绫诲瀷涓?struct device_attribute 鐨勭粨鏋勪綋锛屽悕绉板垎鍒负
'dev_attr_type' 鍜?'dev_attr_power'銆傝繖涓や釜灞炴€у彲浠ラ€氳繃

```
  static struct attribute *dev_attrs[] = {
	&dev_attr_type.attr,
	&dev_attr_power.attr,
	NULL,
  };

  static struct attribute_group dev_group = {
	.attrs = dev_attrs,
  };

  static const struct attribute_group *dev_groups[] = {
	&dev_group,
	NULL,
  };

```
瀵逛簬鍗曚竴缁勭殑甯歌鎯呭喌锛屾彁渚涗簡涓€涓緟鍔╁畯锛屽洜姝や笂杩颁唬鐮佸彲浠ユ敼鍐欎负

```
  ATTRIBUTE_GROUPS(dev);

```
闅忓悗鍙互閫氳繃灏嗕互涓嬫寚閽堝叧鑱斿埌璁惧鏉ュ皢璇ョ粍鏁扮粍涓庤澶囧叧鑱旓細

```
        dev->groups = dev_groups;
        device_register(dev);

```
device_register() 鍑芥暟灏嗕娇鐢?'groups' 鎸囬拡鏉ュ垱寤鸿澶囧睘鎬э紝鑰?device_unregister() 鍑芥暟灏嗕娇鐢ㄨ鎸囬拡鏉ョЩ闄よ澶囧睘鎬с€?
璀﹀憡锛氳櫧鐒跺唴鏍稿厑璁稿湪浠绘剰鏃跺埢瀵硅澶囪皟鐢?device_create_file() 鍜?device_remove_file()锛屼絾鐢ㄦ埛绌洪棿瀵瑰睘鎬х殑鍒涘缓鏃舵満鏈変弗鏍肩殑棰勬湡銆傚綋
涓€涓柊璁惧鍦ㄥ唴鏍镐腑娉ㄥ唽鏃讹紝浼氱敓鎴愪竴涓?uevent 鏉ラ€氱煡鐢ㄦ埛绌洪棿锛堝 udev锛?鏈変竴涓柊璁惧鍙敤銆傚鏋滃湪璁惧娉ㄥ唽涔嬪悗鎵嶆坊鍔犲睘鎬э紝閭ｄ箞鐢ㄦ埛绌洪棿灏嗕笉浼?鏀跺埌閫氱煡锛屼篃灏变笉浼氱煡閬撹繖浜涙柊灞炴€с€?
杩欏浜庨渶瑕佸湪椹卞姩鎺㈡祴鏃朵负璁惧鍙戝竷棰濆灞炴€х殑璁惧椹卞姩鍗佸垎閲嶈銆傚鏋?璁惧椹卞姩鍙槸瀵瑰叾浼犲叆鐨勮澶囩粨鏋勪綋璋冪敤 device_create_file()锛岄偅涔?鐢ㄦ埛绌洪棿灏嗘案杩滄敹涓嶅埌鏂板睘鎬х殑閫氱煡銆?