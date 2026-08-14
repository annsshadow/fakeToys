## Care 和 feeding 的 您的 Human 接口 设备


## Introduction


此外 到 the 正常 输入 类型 HID 设备, USB 也 uses the
human 接口 设备 协议 用于 things 该 是 不 really human
interfaces, 但 具有 similar sorts 的 communication needs. The two big
示例 用于 此 是 电源 设备 (especially uninterruptible 电源
supplies) 和 监视器 control 在 higher end monitors.

到 支持 这些 disparate requirements, the Linux USB 系统 提供
HID 事件 到 two separate interfaces:
- the 输入 子系统, 其 converts HID 事件 进入 正常 输入
设备 interfaces (例如 键盘, 鼠标 和 joystick) 和 一个
normalised 事件 接口 - 参见 Documentation/输入/输入.rst
- the hiddev 接口, 其 提供 fairly raw HID 事件

The 数据 flow 用于 一个 HID 事件 produced 由 一个 设备 是 something 类似
```

 usb.c ---> hid-core.c  ----> hid-input.c ----> [keyboard/mouse/joystick/event]
                         |
                         |
                          --> hiddev.c ----> POWER / MONITOR CONTROL

```
此外, 其他 子系统 (除…之外 USB) 可 potentially feed
事件 进入 the 输入 子系统, 但 这些 具有 无 effect 在 the HID
设备 接口.

## 使用 the HID 设备 接口


The hiddev 接口 是 一个 char 接口 使用 the 正常 USB 主要,
与 the 次要 numbers starting 在 96 和 finishing 在 111. 因此,
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
因此 您 point 您的 hiddev compliant user-space program 在 the correct
接口 用于 您的 设备, 和 它 全部 just works.

Assuming 该 您 具有 一个 hiddev compliant user-space program, 的
course. 若 您 需要 到 写入 one, 读取 在.


## The HIDDEV API


此 description 应当 为 读取 在 conjunction 与 the HID
specification, freely 可用 来自 https://www.usb.org, 和
conveniently linked 的 http://www.linux-usb.org.

The hiddev API uses 一个 读取() 接口, 和 一个 set 的 ioctl() calls.

HID 设备 exchange 数据 与 the host computer 使用 数据
bundles called "reports".  每个 report 是 divided 进入 "字段",
每个 的 其 可 具有 one 或 更多 "usages".  在 the hid-core,
每个 one 的 这些 usages 具有 一个 单个 signed 32-位 值.

### 读取():


这是 the 事件 接口.  当 the HID 设备's 状态 changes,
它 performs 一个 中断 transfer containing 一个 report 其 包含
the changed 值.  The hid-core.c 模块 parses the report, 和
returns 到 hiddev.c the 各个 usages 该 具有 changed 之内
the report.  在 其 基本 模式, the hiddev 将 make 这些 各个
```

       struct hiddev_event {
           unsigned hid;
           signed int value;
       };

```
containing the HID usage identifier 用于 the 状态 该 changed, 和
the 值 该 它 曾是 changed 到. 注意 该 the 结构体 是 定义
之内 <linux/hiddev.h>, along 与 一些 其他 useful #defines 和
结构体.  The HID usage identifier 是 一个 composite 的 the HID usage
页 shifted 到 the 16 high order 位 ORed 与 the usage code.  The
behavior 的 the 读取() 函数 可 为 modified 使用 the HIDIOCSFLAG
ioctl() 描述 下文.


### ioctl():


这是 the control 接口. 存在 一个 数字 的 controls:

HIDIOCGVERSION
  - int (读取)

 Gets the 版本 code 超出 the hiddev 驱动.

HIDIOCAPPLICATION
  - (none)

此 ioctl call returns the HID 应用程序 usage associated 与 the
HID 设备. The third 参数 到 ioctl() specifies 其 应用程序
索引 到 get. 这是 useful 当 the 设备 具有 多于 one
应用程序 collection. 若 the 索引 是 invalid (greater 或 equal 到
the 数字 的 应用程序 collections 此 设备 具有) the ioctl
returns -1. 您可以 find out beforehand 如何 许多 应用程序
collections the 设备 具有 来自 the num_applications 字段 来自 the
hiddev_devinfo 结构体.

HIDIOCGCOLLECTIONINFO
  - 结构体 hiddev_collection_info (读取/写入)

此 returns 一个 superset 的 the information 上文, providing 不 仅
应用程序 collections, 但 全部 the collections the 设备 具有.  它
也 returns the level the collection lives 在 the hierarchy.
The 用户 passes 在 一个 hiddev_collection_info 结构体 与 the 索引
字段 set 到 the 索引 该 应当 为 returned.  The ioctl fills 在
the 其他 字段.  若 the 索引 是 larger 比 the 最后 collection
索引, the ioctl returns -1 和 sets errno 到 -EINVAL.

HIDIOCGDEVINFO
  - 结构体 hiddev_devinfo (读取)

Gets 一个 hiddev_devinfo 结构体 其 describes the 设备.

HIDIOCGSTRING
  - 结构体 hiddev_字符串_描述符 (读取/写入)

Gets 一个 字符串 描述符 来自 the 设备. The caller 必须 fill 在 the
"索引" 字段 到 indicate 其 描述符 应当 为 returned.

HIDIOCINITREPORT
  - (none)

Instructs the 内核 到 retrieve 全部 输入 和 特性 report 值
来自 the 设备. 在 此 point, 全部 the usage 结构体 将 包含
电流 值 用于 the 设备, 和 将 maintain 它 作为 the 设备
changes.  注意 该 the 使用 的 此 ioctl 是 unnecessary 一般而言,
since 稍后 kernels automatically initialize the reports 来自 the
设备 在 attach time.

HIDIOCGNAME
  - 字符串 (variable 长度)

Gets the 设备 name

HIDIOCGREPORT
  - 结构体 hiddev_report_info (写入)

Instructs the 内核 到 get 一个 特性 或 输入 report 来自 the 设备,
为了 selectively 更新 the usage 结构体 (相比之下 到
INITREPORT).

HIDIOCSREPORT
  - 结构体 hiddev_report_info (写入)

Instructs the 内核 到 send 一个 report 到 the 设备. 此 report 可
为 filled 在 由 the 用户 through HIDIOCSUSAGE calls (下文) 到 fill 在
各个 usage 值 在 the report 之前 sending the report 在 full
到 the 设备.

HIDIOCGREPORTINFO
  - 结构体 hiddev_report_info (读取/写入)

Fills 在 一个 hiddev_report_info 结构体 用于 the 用户. The report 是
looked up 由 类型 (输入, 输出 或 特性) 和 id, 因此 这些 字段
必须 为 filled 在 由 the 用户. The ID 可 为 absolute -- the actual
report id 作为 reported 由 the 设备 -- 或 relative --
HID_REPORT_ID_第一 用于 the 第一 report, 和 (HID_REPORT_ID_接下来 |
report_id) 用于 the 接下来 report 之后 report_id. 无 一个 priori
information 关于 report ids, the right way 到 使用 此 ioctl 是 到
使用 the relative IDs 上文 到 enumerate the valid IDs. The ioctl
returns non-zero 当 存在 无 更多 接下来 ID. The real report ID 是
filled 进入 the returned hiddev_report_info 结构体.

HIDIOCGFIELDINFO
  - 结构体 hiddev_字段_info (读取/写入)

Returns the 字段 information associated 与 一个 report 在 一个
hiddev_字段_info 结构体. The 用户 必须 fill 在 report_id 和
report_类型 在 此 结构体, 作为 上文. The 字段_索引 应当 也
为 filled 在, 其 应当 为 一个 数字 来自 0 和 maxfield-1, 作为
returned 来自 一个 前一个 HIDIOCGREPORTINFO call.

HIDIOCGUCODE
  - 结构体 hiddev_usage_ref (读取/写入)

Returns the usage_code 在 一个 hiddev_usage_ref 结构体, given 该
其 report 类型, report id, 字段 索引, 和 索引 之内 the
字段 具有 已经 已经 filled 进入 the 结构体.

HIDIOCGUSAGE
  - 结构体 hiddev_usage_ref (读取/写入)

Returns the 值 的 一个 usage 在 一个 hiddev_usage_ref 结构体. The
usage 到 为 retrieved 可 为 specified 作为 上文, 或 the 用户 可
choose 到 fill 在 the report_类型 字段 和 specify the report_id 作为
HID_REPORT_ID_未知. 在 此 case, the hiddev_usage_ref 将 为
filled 在 与 the report 和 字段 information associated 与 此
usage 若 它是 found.

HIDIOCSUSAGE
  - 结构体 hiddev_usage_ref (写入)

Sets the 值 的 一个 usage 在 一个 输出 report.  The 用户 fills 在
the hiddev_usage_ref 结构体 作为 上文, 但 additionally fills 在
the 值 字段.

HIDIOGCOLLECTIONINDEX
  - 结构体 hiddev_usage_ref (写入)

Returns the collection 索引 associated 与 此 usage.  此
indicates 何处 在 the collection hierarchy 此 usage sits.

HIDIOCGFLAG
  - int (读取)
HIDIOCSFLAG
  - int (写入)

这些 操作 respectively inspect 和 replace the 模式 标志
该 influence the 读取() call 上文.  The 标志 是 作为 follows:

    HIDDEV_标志_UREF
      - 读取() calls 将 现在 return
        结构体 hiddev_usage_ref 而非 结构体 hiddev_事件.
        这是 一个 larger 结构体, 但 在 situations 何处 the
        设备 具有 多于 one usage 在 其 reports 与 the
        相同 usage code, 此 模式 serves 到 resolve 此类
        ambiguity.

    HIDDEV_标志_REPORT
      - 此 标志 可 仅 为 使用 在 conjunction
        与 HIDDEV_标志_UREF.  与 此 标志 set, 当 the 设备
        sends 一个 report, 一个 结构体 hiddev_usage_ref 将 为 returned
        到 读取() filled 在 与 the report_类型 和 report_id, 但
        与 字段_索引 set 到 字段_索引_NONE.  此 serves 作为
        额外 notification 当 the 设备 具有 sent 一个 report.
