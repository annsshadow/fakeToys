
## NVDIMM 运行时固件激

一些持久内存设备在设备/“DIMM”本地运行固件，以执行媒体管理、容量配置以及健康监控等任务更新该固件的过程通常涉及重启，因为这会影响正在进行的内存事务。然而，重启具有破坏性，至少
Intel ACPI DSM 规范 [^1^] 所描述Intel 持久内存平台实现增加了在运行时激活固件的支持
libnvdimm 中实现了一个原生的 sysfs 接口，以允许平台通告并控制其本地的运行时固件激活能力
libnvdimm 的总线对象 ndbusX 实现了一ndbusX/firmware/activate 属性，'idle'armed''overflow'busy' 之一显示固件激活的状态
- idle（空闲）  没有设备被设武装（armed）以激活固
- armed（已武装）：
  至少有一个设备已被武
- busy（忙碌）  busy 状态下，已武装的设备正在转idle 并完成一个激活周期
- overflow（溢出）  如果平台存在执行激活所需的增量工作的概念，则可能出现为激活武装了过多 DIMM 的情况。在  场景下，'overflow' 状态表明固件激活有可能超时
'ndbusX/firmware/activate' 属性可以被写入 'live' 'quiesce' 值quiesce' 值会触发内核
在相当于 hibernation（休眠）'freeze' 状态的环境中运行固件激活，此时驱动与应用程序会被通知
停止对系统内存的修改live' 值则尝试不进行这一休眠周期来完成固件激活。如果未检测到任何
固件激活能力，'ndbusX/firmware/activate' 属性会被完全省略
另一个属'ndbusX/firmware/capability' 指示 'live' 'quiesce' 值，其中 'live' 表示固件
更新不需要或不会对系统施加任何静默（quiesce）期quiesce' 能力值表示固件确实期望并为内控制器注入一个静默期，但 'live' 仍可被写'ndbusX/firmware/activate' 作为覆盖，以承担固件
更新与在途设备及应用活动相互竞争的风险。如果未检测到任何固件激活能力，'ndbusX/firmware/capability' 属性会被完全省略
libnvdimm 的内存设DIMM 对象 nmemX 实现'nmemX/firmware/activate' 'nmemX/firmware/result'
属性，以传达每设备的固件激活状态。与 'ndbusX/firmware/activate' 属性类似，'nmemX/firmware/activate'
属性指'idle'armed' 'busy'。当系统准备好激活固件、固件已暂存且状态被设为 armed、并
触发 'ndbusX/firmware/activate' 时，状态从 'armed' 转变'idle'。在该激活事件之后，
nmemX/firmware/result 属性以以下之一反映上一次激活的状态：

- none（无）：
  自上一次设备复位以来，未触发任何运行时激
- success（成功）  上一次运行时激活成功完成
- fail（失败）  上一次运行时激活因设备特定的原因而失败
- not_staged（未暂存）：
  上一次运行时激活失败，原因是固件镜像未被暂存而导致顺序错误
- need_reset（需要复位）  运行时固件激活失败，但固件仍可通过重启系统的传统方法激活
[^1^]: https://docs.pmem.io/persistent-memory/
