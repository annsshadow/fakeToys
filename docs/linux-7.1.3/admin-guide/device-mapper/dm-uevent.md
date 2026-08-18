## device-mapper uevent锛堣澶囨槧灏勫櫒 uevent锛?

device-mapper uevent 浠ｇ爜涓?device-mapper 澧炲姞浜嗗垱寤哄苟鍙戦€?kobject uevent锛坲event锛夌殑鑳藉姏銆傛鍓?device-mapper 浜嬩欢浠呴€氳繃 ioctl 鎺ュ彛鍙敤銆倁event 鎺ュ彛鐨勪紭鍔垮湪浜庯紝浜嬩欢鍖呭惈鐜灞炴€э紝涓轰簨浠舵彁渚涗簡鏇村涓婁笅鏂囷紝浠庤€屾棤闇€鍦ㄦ敹鍒颁簨浠跺悗鍐嶅幓鏌ヨ device-mapper 璁惧鐨勭姸鎬併€?
鐩墠 device-mapper 浜嬩欢鏈変袱涓嚱鏁般€傜涓€涓嚱鏁?```

  void dm_path_uevent(enum dm_uevent_type event_type, struct dm_target *ti,
                      const char *path, unsigned nr_valid_paths)

  void dm_send_uevents(struct list_head *events, struct kobject *kobj)


```
娣诲姞鍒?uevent 鐜鐨勫彉閲忔湁锛?
### 鍙橀噺鍚嶏細DM_TARGET

:Uevent Action(s): KOBJ_CHANGE
:Type: string
:Description:
:Value: 浜х敓璇ヤ簨浠剁殑 device-mapper 鐩爣鐨勫悕绉般€?
### 鍙橀噺鍚嶏細DM_ACTION

:Uevent Action(s): KOBJ_CHANGE
:Type: string
:Description:
:Value: 瀵艰嚧璇?uevent 鍔ㄤ綔鐨?device-mapper 鐗瑰畾鍔ㄤ綔銆?	PATH_FAILED - 涓€鏉¤矾寰勫凡澶辫触锛?	PATH_REINSTATED - 涓€鏉¤矾寰勫凡琚仮澶嶃€?
### 鍙橀噺鍚嶏細DM_SEQNUM

:Uevent Action(s): KOBJ_CHANGE
:Type: unsigned integer
:Description: 璇ョ壒瀹?device-mapper 璁惧鐨勫簭鍒楀彿銆?:Value: 鏈夋晥鐨勬棤绗﹀彿鏁存暟鑼冨洿銆?
### 鍙橀噺鍚嶏細DM_PATH

:Uevent Action(s): KOBJ_CHANGE
:Type: string
:Description: 涓庢湰娆′簨浠剁浉鍏崇殑璺緞璁惧鐨勪富璁惧鍙峰拰娆¤澶囧彿銆?:Value: 褰㈠ "Major:Minor" 鐨勮矾寰勫悕銆?
### 鍙橀噺鍚嶏細DM_NR_VALID_PATHS

:Uevent Action(s): KOBJ_CHANGE
:Type: unsigned integer
:Description:
:Value: 鏈夋晥鐨勬棤绗﹀彿鏁存暟鑼冨洿銆?
### 鍙橀噺鍚嶏細DM_NAME

:Uevent Action(s): KOBJ_CHANGE
:Type: string
:Description: device-mapper 璁惧鐨勫悕绉般€?:Value: 鍚嶇О

### 鍙橀噺鍚嶏細DM_UUID

:Uevent Action(s): KOBJ_CHANGE
:Type: string
:Description: device-mapper 璁惧鐨?UUID銆?:Value: UUID銆傦紙濡傛灉娌℃湁鍒欎负绌哄瓧绗︿覆銆傦級

涓嬮潰鏄敱 udevmonitor 鎹曡幏鐨勬墍鐢熸垚 uevent 鐨勭ず渚?
```

	UEVENT[1192521009.711215] change@/block/dm-3
	ACTION=change
	DEVPATH=/block/dm-3
	SUBSYSTEM=block
	DM_TARGET=multipath
	DM_ACTION=PATH_FAILED
	DM_SEQNUM=1
	DM_PATH=8:32
	DM_NR_VALID_PATHS=0
	DM_NAME=mpath2
	DM_UUID=mpath-35333333000002328
	MINOR=3
	MAJOR=253
	SEQNUM=1130

```
```

	UEVENT[1192521132.989927] change@/block/dm-3
	ACTION=change
	DEVPATH=/block/dm-3
	SUBSYSTEM=block
	DM_TARGET=multipath
	DM_ACTION=PATH_REINSTATED
	DM_SEQNUM=2
	DM_PATH=8:32
	DM_NR_VALID_PATHS=1
	DM_NAME=mpath2
	DM_UUID=mpath-35333333000002328
	MINOR=3
	MAJOR=253
	SEQNUM=1131

```
