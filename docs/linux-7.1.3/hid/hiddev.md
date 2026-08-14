## Care 鍜?feeding 鐨?鎮ㄧ殑 Human 鎺ュ彛 璁惧


## Introduction


姝ゅ 鍒?the 姝ｅ父 杈撳叆 绫诲瀷 HID 璁惧, USB 涔?uses the
human 鎺ュ彛 璁惧 鍗忚 鐢ㄤ簬 things 璇?鏄?涓?really human
interfaces, 浣?鍏锋湁 similar sorts 鐨?communication needs. The two big
绀轰緥 鐢ㄤ簬 姝?鏄?鐢垫簮 璁惧 (especially uninterruptible 鐢垫簮
supplies) 鍜?鐩戣鍣?control 鍦?higher end monitors.

鍒?鏀寔 杩欎簺 disparate requirements, the Linux USB 绯荤粺 鎻愪緵
HID 浜嬩欢 鍒?two separate interfaces:
- the 杈撳叆 瀛愮郴缁? 鍏?converts HID 浜嬩欢 杩涘叆 姝ｅ父 杈撳叆
璁惧 interfaces (渚嬪 閿洏, 榧犳爣 鍜?joystick) 鍜?涓€涓?
normalised 浜嬩欢 鎺ュ彛 - 鍙傝 Documentation/杈撳叆/杈撳叆.rst
- the hiddev 鎺ュ彛, 鍏?鎻愪緵 fairly raw HID 浜嬩欢

The 鏁版嵁 flow 鐢ㄤ簬 涓€涓?HID 浜嬩欢 produced 鐢?涓€涓?璁惧 鏄?something 绫讳技
```

 usb.c ---> hid-core.c  ----> hid-input.c ----> [keyboard/mouse/joystick/event]
                         |
                         |
                          --> hiddev.c ----> POWER / MONITOR CONTROL

```
姝ゅ, 鍏朵粬 瀛愮郴缁?(闄も€︿箣澶?USB) 鍙?potentially feed
浜嬩欢 杩涘叆 the 杈撳叆 瀛愮郴缁? 浣?杩欎簺 鍏锋湁 鏃?effect 鍦?the HID
璁惧 鎺ュ彛.

## 浣跨敤 the HID 璁惧 鎺ュ彛


The hiddev 鎺ュ彛 鏄?涓€涓?char 鎺ュ彛 浣跨敤 the 姝ｅ父 USB 涓昏,
涓?the 娆¤ numbers starting 鍦?96 鍜?finishing 鍦?111. 鍥犳,
```

	mknod /dev/usb/hiddev0 c 180 96
	mknod /dev/usb/hiddev1 c 180 97
	mknod /dev/usb/hiddev2 c 180 98
	mknod /dev/usb/hiddev3 c 180 99
	mknod /dev/usb/hiddev4 c 180 100
	mknod /dev/usb/hiddev5 c 180 101
	mknod /dev/usb/hiddev6 c 180 102
	mknod /dev/usb/hiddev7 c 180 103
	mknod /dev/usb/hiddev8 c 180 104
	mknod /dev/usb/hiddev9 c 180 105
	mknod /dev/usb/hiddev10 c 180 106
	mknod /dev/usb/hiddev11 c 180 107
	mknod /dev/usb/hiddev12 c 180 108
	mknod /dev/usb/hiddev13 c 180 109
	mknod /dev/usb/hiddev14 c 180 110
	mknod /dev/usb/hiddev15 c 180 111

```
鍥犳 鎮?point 鎮ㄧ殑 hiddev compliant user-space program 鍦?the correct
鎺ュ彛 鐢ㄤ簬 鎮ㄧ殑 璁惧, 鍜?瀹?鍏ㄩ儴 just works.

Assuming 璇?鎮?鍏锋湁 涓€涓?hiddev compliant user-space program, 鐨?
course. 鑻?鎮?闇€瑕?鍒?鍐欏叆 one, 璇诲彇 鍦?


## The HIDDEV API


姝?description 搴斿綋 涓?璇诲彇 鍦?conjunction 涓?the HID
specification, freely 鍙敤 鏉ヨ嚜 https://www.usb.org, 鍜?
conveniently linked 鐨?http://www.linux-usb.org.

The hiddev API uses 涓€涓?璇诲彇() 鎺ュ彛, 鍜?涓€涓?set 鐨?ioctl() calls.

HID 璁惧 exchange 鏁版嵁 涓?the host computer 浣跨敤 鏁版嵁
bundles called "reports".  姣忎釜 report 鏄?divided 杩涘叆 "瀛楁",
姣忎釜 鐨?鍏?鍙?鍏锋湁 one 鎴?鏇村 "usages".  鍦?the hid-core,
姣忎釜 one 鐨?杩欎簺 usages 鍏锋湁 涓€涓?鍗曚釜 signed 32-浣?鍊?

### 璇诲彇():


杩欐槸 the 浜嬩欢 鎺ュ彛.  褰?the HID 璁惧's 鐘舵€?changes,
瀹?performs 涓€涓?涓柇 transfer containing 涓€涓?report 鍏?鍖呭惈
the changed 鍊?  The hid-core.c 妯″潡 parses the report, 鍜?
returns 鍒?hiddev.c the 鍚勪釜 usages 璇?鍏锋湁 changed 涔嬪唴
the report.  鍦?鍏?鍩烘湰 妯″紡, the hiddev 灏?make 杩欎簺 鍚勪釜
```

       struct hiddev_event {
           unsigned hid;
           signed int value;
       };

```
containing the HID usage identifier 鐢ㄤ簬 the 鐘舵€?璇?changed, 鍜?
the 鍊?璇?瀹?鏇炬槸 changed 鍒? 娉ㄦ剰 璇?the 缁撴瀯浣?鏄?瀹氫箟
涔嬪唴 <linux/hiddev.h>, along 涓?涓€浜?鍏朵粬 useful #defines 鍜?
缁撴瀯浣?  The HID usage identifier 鏄?涓€涓?composite 鐨?the HID usage
椤?shifted 鍒?the 16 high order 浣?ORed 涓?the usage code.  The
behavior 鐨?the 璇诲彇() 鍑芥暟 鍙?涓?modified 浣跨敤 the HIDIOCSFLAG
ioctl() 鎻忚堪 涓嬫枃.


### ioctl():


杩欐槸 the control 鎺ュ彛. 瀛樺湪 涓€涓?鏁板瓧 鐨?controls:

HIDIOCGVERSION
  - int (璇诲彇)

 Gets the 鐗堟湰 code 瓒呭嚭 the hiddev 椹卞姩.

HIDIOCAPPLICATION
  - (none)

姝?ioctl call returns the HID 搴旂敤绋嬪簭 usage associated 涓?the
HID 璁惧. The third 鍙傛暟 鍒?ioctl() specifies 鍏?搴旂敤绋嬪簭
绱㈠紩 鍒?get. 杩欐槸 useful 褰?the 璁惧 鍏锋湁 澶氫簬 one
搴旂敤绋嬪簭 collection. 鑻?the 绱㈠紩 鏄?invalid (greater 鎴?equal 鍒?
the 鏁板瓧 鐨?搴旂敤绋嬪簭 collections 姝?璁惧 鍏锋湁) the ioctl
returns -1. 鎮ㄥ彲浠?find out beforehand 濡備綍 璁稿 搴旂敤绋嬪簭
collections the 璁惧 鍏锋湁 鏉ヨ嚜 the num_applications 瀛楁 鏉ヨ嚜 the
hiddev_devinfo 缁撴瀯浣?

HIDIOCGCOLLECTIONINFO
  - 缁撴瀯浣?hiddev_collection_info (璇诲彇/鍐欏叆)

姝?returns 涓€涓?superset 鐨?the information 涓婃枃, providing 涓?浠?
搴旂敤绋嬪簭 collections, 浣?鍏ㄩ儴 the collections the 璁惧 鍏锋湁.  瀹?
涔?returns the level the collection lives 鍦?the hierarchy.
The 鐢ㄦ埛 passes 鍦?涓€涓?hiddev_collection_info 缁撴瀯浣?涓?the 绱㈠紩
瀛楁 set 鍒?the 绱㈠紩 璇?搴斿綋 涓?returned.  The ioctl fills 鍦?
the 鍏朵粬 瀛楁.  鑻?the 绱㈠紩 鏄?larger 姣?the 鏈€鍚?collection
绱㈠紩, the ioctl returns -1 鍜?sets errno 鍒?-EINVAL.

HIDIOCGDEVINFO
  - 缁撴瀯浣?hiddev_devinfo (璇诲彇)

Gets 涓€涓?hiddev_devinfo 缁撴瀯浣?鍏?describes the 璁惧.

HIDIOCGSTRING
  - 缁撴瀯浣?hiddev_瀛楃涓瞋鎻忚堪绗?(璇诲彇/鍐欏叆)

Gets 涓€涓?瀛楃涓?鎻忚堪绗?鏉ヨ嚜 the 璁惧. The caller 蹇呴』 fill 鍦?the
"绱㈠紩" 瀛楁 鍒?indicate 鍏?鎻忚堪绗?搴斿綋 涓?returned.

HIDIOCINITREPORT
  - (none)

Instructs the 鍐呮牳 鍒?retrieve 鍏ㄩ儴 杈撳叆 鍜?鐗规€?report 鍊?
鏉ヨ嚜 the 璁惧. 鍦?姝?point, 鍏ㄩ儴 the usage 缁撴瀯浣?灏?鍖呭惈
鐢垫祦 鍊?鐢ㄤ簬 the 璁惧, 鍜?灏?maintain 瀹?浣滀负 the 璁惧
changes.  娉ㄦ剰 璇?the 浣跨敤 鐨?姝?ioctl 鏄?unnecessary 涓€鑸€岃█,
since 绋嶅悗 kernels automatically initialize the reports 鏉ヨ嚜 the
璁惧 鍦?attach time.

HIDIOCGNAME
  - 瀛楃涓?(variable 闀垮害)

Gets the 璁惧 name

HIDIOCGREPORT
  - 缁撴瀯浣?hiddev_report_info (鍐欏叆)

Instructs the 鍐呮牳 鍒?get 涓€涓?鐗规€?鎴?杈撳叆 report 鏉ヨ嚜 the 璁惧,
涓轰簡 selectively 鏇存柊 the usage 缁撴瀯浣?(鐩告瘮涔嬩笅 鍒?
INITREPORT).

HIDIOCSREPORT
  - 缁撴瀯浣?hiddev_report_info (鍐欏叆)

Instructs the 鍐呮牳 鍒?send 涓€涓?report 鍒?the 璁惧. 姝?report 鍙?
涓?filled 鍦?鐢?the 鐢ㄦ埛 through HIDIOCSUSAGE calls (涓嬫枃) 鍒?fill 鍦?
鍚勪釜 usage 鍊?鍦?the report 涔嬪墠 sending the report 鍦?full
鍒?the 璁惧.

HIDIOCGREPORTINFO
  - 缁撴瀯浣?hiddev_report_info (璇诲彇/鍐欏叆)

Fills 鍦?涓€涓?hiddev_report_info 缁撴瀯浣?鐢ㄤ簬 the 鐢ㄦ埛. The report 鏄?
looked up 鐢?绫诲瀷 (杈撳叆, 杈撳嚭 鎴?鐗规€? 鍜?id, 鍥犳 杩欎簺 瀛楁
蹇呴』 涓?filled 鍦?鐢?the 鐢ㄦ埛. The ID 鍙?涓?absolute -- the actual
report id 浣滀负 reported 鐢?the 璁惧 -- 鎴?relative --
HID_REPORT_ID_绗竴 鐢ㄤ簬 the 绗竴 report, 鍜?(HID_REPORT_ID_鎺ヤ笅鏉?|
report_id) 鐢ㄤ簬 the 鎺ヤ笅鏉?report 涔嬪悗 report_id. 鏃?涓€涓?priori
information 鍏充簬 report ids, the right way 鍒?浣跨敤 姝?ioctl 鏄?鍒?
浣跨敤 the relative IDs 涓婃枃 鍒?enumerate the valid IDs. The ioctl
returns non-zero 褰?瀛樺湪 鏃?鏇村 鎺ヤ笅鏉?ID. The real report ID 鏄?
filled 杩涘叆 the returned hiddev_report_info 缁撴瀯浣?

HIDIOCGFIELDINFO
  - 缁撴瀯浣?hiddev_瀛楁_info (璇诲彇/鍐欏叆)

Returns the 瀛楁 information associated 涓?涓€涓?report 鍦?涓€涓?
hiddev_瀛楁_info 缁撴瀯浣? The 鐢ㄦ埛 蹇呴』 fill 鍦?report_id 鍜?
report_绫诲瀷 鍦?姝?缁撴瀯浣? 浣滀负 涓婃枃. The 瀛楁_绱㈠紩 搴斿綋 涔?
涓?filled 鍦? 鍏?搴斿綋 涓?涓€涓?鏁板瓧 鏉ヨ嚜 0 鍜?maxfield-1, 浣滀负
returned 鏉ヨ嚜 涓€涓?鍓嶄竴涓?HIDIOCGREPORTINFO call.

HIDIOCGUCODE
  - 缁撴瀯浣?hiddev_usage_ref (璇诲彇/鍐欏叆)

Returns the usage_code 鍦?涓€涓?hiddev_usage_ref 缁撴瀯浣? given 璇?
鍏?report 绫诲瀷, report id, 瀛楁 绱㈠紩, 鍜?绱㈠紩 涔嬪唴 the
瀛楁 鍏锋湁 宸茬粡 宸茬粡 filled 杩涘叆 the 缁撴瀯浣?

HIDIOCGUSAGE
  - 缁撴瀯浣?hiddev_usage_ref (璇诲彇/鍐欏叆)

Returns the 鍊?鐨?涓€涓?usage 鍦?涓€涓?hiddev_usage_ref 缁撴瀯浣? The
usage 鍒?涓?retrieved 鍙?涓?specified 浣滀负 涓婃枃, 鎴?the 鐢ㄦ埛 鍙?
choose 鍒?fill 鍦?the report_绫诲瀷 瀛楁 鍜?specify the report_id 浣滀负
HID_REPORT_ID_鏈煡. 鍦?姝?case, the hiddev_usage_ref 灏?涓?
filled 鍦?涓?the report 鍜?瀛楁 information associated 涓?姝?
usage 鑻?瀹冩槸 found.

HIDIOCSUSAGE
  - 缁撴瀯浣?hiddev_usage_ref (鍐欏叆)

Sets the 鍊?鐨?涓€涓?usage 鍦?涓€涓?杈撳嚭 report.  The 鐢ㄦ埛 fills 鍦?
the hiddev_usage_ref 缁撴瀯浣?浣滀负 涓婃枃, 浣?additionally fills 鍦?
the 鍊?瀛楁.

HIDIOGCOLLECTIONINDEX
  - 缁撴瀯浣?hiddev_usage_ref (鍐欏叆)

Returns the collection 绱㈠紩 associated 涓?姝?usage.  姝?
indicates 浣曞 鍦?the collection hierarchy 姝?usage sits.

HIDIOCGFLAG
  - int (璇诲彇)
HIDIOCSFLAG
  - int (鍐欏叆)

杩欎簺 鎿嶄綔 respectively inspect 鍜?replace the 妯″紡 鏍囧織
璇?influence the 璇诲彇() call 涓婃枃.  The 鏍囧織 鏄?浣滀负 follows:

    HIDDEV_鏍囧織_UREF
      - 璇诲彇() calls 灏?鐜板湪 return
        缁撴瀯浣?hiddev_usage_ref 鑰岄潪 缁撴瀯浣?hiddev_浜嬩欢.
        杩欐槸 涓€涓?larger 缁撴瀯浣? 浣?鍦?situations 浣曞 the
        璁惧 鍏锋湁 澶氫簬 one usage 鍦?鍏?reports 涓?the
        鐩稿悓 usage code, 姝?妯″紡 serves 鍒?resolve 姝ょ被
        ambiguity.

    HIDDEV_鏍囧織_REPORT
      - 姝?鏍囧織 鍙?浠?涓?浣跨敤 鍦?conjunction
        涓?HIDDEV_鏍囧織_UREF.  涓?姝?鏍囧織 set, 褰?the 璁惧
        sends 涓€涓?report, 涓€涓?缁撴瀯浣?hiddev_usage_ref 灏?涓?returned
        鍒?璇诲彇() filled 鍦?涓?the report_绫诲瀷 鍜?report_id, 浣?
        涓?瀛楁_绱㈠紩 set 鍒?瀛楁_绱㈠紩_NONE.  姝?serves 浣滀负
        棰濆 notification 褰?the 璁惧 鍏锋湁 sent 涓€涓?report.
