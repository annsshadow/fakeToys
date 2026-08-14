## mtouchusb 驱动


## 变更


- 0.3 - 基于 freecode 上原触摸屏驱动的 scanner 与 INSTALL 创建
  （http://freecode.com/projects/3mtouchscreendriver）
- 针对 linux-2.4.18 修改，随后是 2.4.19

- 0.5 - 在 2.6.3 中使用 Linux Input 完全重写
  遗憾的是目前不支持校准

- 1.4 - 为支持 EXII 5000UC 做了多处修改并进行了清理
  将复位从标准 USB 设备复位改为厂商复位
  将从主机发送的数据由补偿坐标改为原始坐标
  去掉了 vendor/product 模块参数
  使用 EXII-5010UC 进行了多次成功测试

## 支持的硬件


```

        All controllers have the Vendor: 0x0596 & Product: 0x0001


        Controller Description          Part Number
        ------------------------------------------------------

        USB Capacitive - Pearl Case     14-205  (Discontinued)
        USB Capacitive - Black Case     14-124  (Discontinued)
        USB Capacitive - No Case        14-206  (Discontinued)

        USB Capacitive - Pearl Case     EXII-5010UC
        USB Capacitive - Black Case     EXII-5030UC
        USB Capacitive - No Case        EXII-5050UC

```
## 驱动说明


安装很简单，你只需将 Linux Input、Linux USB 以及本驱动加入内核即可。该驱动也可以选择编译为模块。

本驱动似乎是可能的两个 Linux USB Input 触摸屏驱动之一。尽管 3M 提供了可供下载的仅二进制驱动，我仍坚持更新本驱动，因为我想将触摸屏用于使用 QTEmbedded、DirectFB 等的嵌入式应用。因此我认为合乎逻辑的选择是使用 Linux Input。

目前无法通过本驱动校准设备。即使设备可以被校准，驱动也是从控制器拉取原始坐标数据。这意味着校准必须在用户空间中进行。

控制器屏幕分辨率现在 X 和 Y 均为 0 到 16384，报告的是原始触摸数据。这对新旧电容式 USB 控制器都是相同的。

也许将来会在 evdev 中放置一个抽象函数，以便从用户空间请求校准、复位和厂商信息之类的通用功能（驱动将处理厂商特定的任务）。

## 待办


一旦/如果可用，重新实现一个控制 urb 以处理与设备的往返请求，例如校准等。

## 免责声明


我不是 MicroTouch/3M 的员工，也从未是。3M 不支持本驱动！如果你只想要在 X 内受支持的触摸驱动，请前往：

http://www.3m.com/3MTouchSystems/

## 致谢


非常感谢 3M Touch Systems 提供用于测试的 EXII-5010UC 控制器！
