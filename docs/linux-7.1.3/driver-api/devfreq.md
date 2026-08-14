
## 设备频率调节（Device Frequency Scaling）


### 简介


该框架为任意设备上的动态电压与频率调节（DVFS）提供标准的内核接口。

它通过 sysfs 文件暴露用于调整频率的控制接口，类似于 cpufreq 子系统。

对于可以测量当前使用情况的设备，其频率可由调节器（governor）自动调整。

### API


设备驱动需要初始化一个 `devfreq_profile` 并调用 `devfreq_add_device` 函数来创建一个 `devfreq` 实例。

        :export:
        :export:
