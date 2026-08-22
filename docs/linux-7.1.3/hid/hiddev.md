## Care feeding 您的 Human 接口 设备


## Introduction


此外 the 正常 输入 类型 HID 设备, USB uses the
human 接口 设备 协议 用于 things really human
interfaces, 具有 similar sorts communication needs. The two big
示例 用于 电源 设备 (especially uninterruptible 电源
supplies) 监视control higher end monitors.

支持 这些 disparate requirements, the Linux USB 系统 提供
HID 事件 two separate interfaces:
- the 输入 子系 converts HID 事件 进入 正常 输入
设备 interfaces (例如 键盘, 鼠标 joystick) 一
normalised 事件 接口 - 参见 Documentation/输入/输入.rst
- the hiddev 接口, 提供 fairly raw HID 事件

The 数据 flow 用于 一HID 事件 produced 一设备 something 类似
```

 usb.c ---> hid-core.c  ----> hid-input.c ----> [keyboard/mouse/joystick/event]
                         |
                         |
                          --> hiddev.c ----> POWER / MONITOR CONTROL

```
此外, 其他 子系(除…之USB) potentially feed
事件 进入 the 输入 子系 这些 具有 effect the HID
设备 接口.

## 使用 the HID 设备 接口


The hiddev 接口 一char 接口 使用 the 正常 USB 主要,
the 次要 numbers starting 96 finishing 111. 因此,
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
因此 point 您的 hiddev compliant user-space program the correct
接口 用于 您的 设备, 全部 just works.

Assuming 具有 一hiddev compliant user-space program, 
course. 需写入 one, 读取 


## The HIDDEV API


description 应当 读取 conjunction the HID
specification, freely 可用 来自 https://www.usb.org, 
conveniently linked 鐨?http://www.linux-usb.org.

The hiddev API uses 一读取() 接口, 一set ioctl() calls.

HID 设备 exchange 数据 the host computer 使用 数据
bundles called "reports".  每个 report divided 进入 "字段",
每个 具有 one 更多 "usages".  the hid-core,
每个 one 这些 usages 具有 一单个 signed 32-

### 读取():


这是 the 事件 接口.  the HID 设备's 状changes,
performs 一中断 transfer containing 一report 包含
the changed 鍊?  The hid-core.c 妯″潡 parses the report, 鍜。
returns hiddev.c the 各个 usages 具有 changed 之内
the report.  基本 模式, the hiddev make 这些 各个
```

       struct hiddev_event {
           unsigned hid;
           signed int value;
       };

```
containing the HID usage identifier 用于 the 状changed, 
the 曾是 changed  注意 the 结构定义
之内 <linux/hiddev.h>, along 一其他 useful #defines 
结构  The HID usage identifier 一composite the HID usage
椤?shifted 鍒?the 16 high order 浣?ORed 涓?the usage code.  The
behavior the 读取() 函数 modified 使用 the HIDIOCSFLAG
ioctl() 描述 下文.


### ioctl():


这是 the control 接口. 存在 一数字 controls:

HIDIOCGVERSION
  - int (读取)

 Gets the 版本 code 超出 the hiddev 驱动.

HIDIOCAPPLICATION
  - (none)

ioctl call returns the HID 应用程序 usage associated the
HID 设备. The third 参数 ioctl() specifies 应用程序
索引 get. 这是 useful the 设备 具有 多于 one
应用程序 collection. the 索引 invalid (greater equal 
the 数字 应用程序 collections 设备 具有) the ioctl
returns -1. 您可find out beforehand 如何 许多 应用程序
collections the 设备 具有 来自 the num_applications 字段 来自 the
hiddev_devinfo 结构

HIDIOCGCOLLECTIONINFO
  - 结构hiddev_collection_info (读取/写入)

returns 一superset the information 上文, providing 
应用程序 collections, 全部 the collections the 设备 具有.  
涔?returns the level the collection lives 鍦?the hierarchy.
The 用户 passes 一hiddev_collection_info 结构the 索引
字段 set the 索引 应当 returned.  The ioctl fills 
the 其他 字段.  the 索引 larger the 最collection
索引, the ioctl returns -1 sets errno -EINVAL.

HIDIOCGDEVINFO
  - 结构hiddev_devinfo (读取)

Gets 一hiddev_devinfo 结构describes the 设备.

HIDIOCGSTRING
  - 结构hiddev_字符串_描述(读取/写入)

Gets 一字符描述来自 the 设备. The caller 必须 fill the
"索引" 字段 indicate 描述应当 returned.

HIDIOCINITREPORT
  - (none)

Instructs the 内核 retrieve 全部 输入 特report 
来自 the 设备. point, 全部 the usage 结构包含
电流 用于 the 设备, maintain 作为 the 设备
changes.  注意 the 使用 ioctl unnecessary 一般而言,
since 稍后 kernels automatically initialize the reports 来自 the
设备 attach time.

HIDIOCGNAME
  - 字符(variable 长度)

Gets the 设备 name

HIDIOCGREPORT
  - 结构hiddev_report_info (写入)

Instructs the 内核 get 一特输入 report 来自 the 设备,
为了 selectively 更新 the usage 结构(相比之下 
INITREPORT).

HIDIOCSREPORT
  - 结构hiddev_report_info (写入)

Instructs the 内核 send 一report the 设备. report 
filled the 用户 through HIDIOCSUSAGE calls (下文) fill 
各个 usage the report 之前 sending the report full
the 设备.

HIDIOCGREPORTINFO
  - 结构hiddev_report_info (读取/写入)

Fills 一hiddev_report_info 结构用于 the 用户. The report 
looked up 类型 (输入, 输出 特 id, 因此 这些 字段
必须 filled the 用户. The ID absolute -- the actual
report id 作为 reported the 设备 -- relative --
HID_REPORT_ID_第一 用于 the 第一 report, (HID_REPORT_ID_接下|
report_id) 用于 the 接下report 之后 report_id. 一priori
information 关于 report ids, the right way 使用 ioctl 
使用 the relative IDs 上文 enumerate the valid IDs. The ioctl
returns non-zero 存在 更多 接下ID. The real report ID 
filled 进入 the returned hiddev_report_info 结构

HIDIOCGFIELDINFO
  - 结构hiddev_字段_info (读取/写入)

Returns the 字段 information associated 一report 一
hiddev_字段_info 结构 The 用户 必须 fill report_id 
report_类型 结构 作为 上文. The 字段_索引 应当 
filled  应当 一数字 来自 0 maxfield-1, 作为
returned 来自 一前一HIDIOCGREPORTINFO call.

HIDIOCGUCODE
  - 结构hiddev_usage_ref (读取/写入)

Returns the usage_code 一hiddev_usage_ref 结构 given 
report 类型, report id, 字段 索引, 索引 之内 the
字段 具有 已经 已经 filled 进入 the 结构

HIDIOCGUSAGE
  - 结构hiddev_usage_ref (读取/写入)

Returns the 一usage 一hiddev_usage_ref 结构 The
usage retrieved specified 作为 上文, the 用户 
choose fill the report_类型 字段 specify the report_id 作为
HID_REPORT_ID_未知. case, the hiddev_usage_ref 
filled the report 字段 information associated 
usage 鑻，瀹冩槸 found.

HIDIOCSUSAGE
  - 结构hiddev_usage_ref (写入)

Sets the 一usage 一输出 report.  The 用户 fills 
the hiddev_usage_ref 结构作为 上文, additionally fills 
the 字段.

HIDIOGCOLLECTIONINDEX
  - 结构hiddev_usage_ref (写入)

Returns the collection 索引 associated usage.  
indicates 何处 the collection hierarchy usage sits.

HIDIOCGFLAG
  - int (读取)
HIDIOCSFLAG
  - int (写入)

杩欎簺 鎿嶄綔 respectively inspect 鍜?replace the 妯″紡 鏍囧織
influence the 读取() call 上文.  The 标志 作为 follows:

    HIDDEV_标志_UREF
      - 读取() calls 现在 return
        结构hiddev_usage_ref 而非 结构hiddev_事件.
        这是 一larger 结构 situations 何处 the
        设备 具有 多于 one usage reports the
        相同 usage code, 模式 serves resolve 此类
        ambiguity.

    HIDDEV_标志_REPORT
      - 标志 使用 conjunction
        HIDDEV_标志_UREF.  标志 set, the 设备
        sends 一report, 一结构hiddev_usage_ref returned
        读取() filled the report_类型 report_id, 
        字段_索引 set 字段_索引_NONE.  serves 作为
        额外 notification the 设备 具有 sent 一report.
