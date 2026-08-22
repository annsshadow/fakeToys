
## Debugging and tracing in the media subsystem


本文档作为起点和查阅手册，用于在 media 子系统中调试设备驱动，以及从用户空间调试这些驱动
    :depth: 3

### General debugging advice


一般性建议请参阅 :doc:`通用建议文档 </process/debugging/index>`
以下各节向你展示一些可用的工具
### dev_debug module parameter


每个视频设备都提供一`dev_debug` 参数，可用于获取
```

  # cat /sys/class/video4linux/video3/name
  rkvdec
  # echo 0xff > /sys/class/video4linux/video3/dev_debug
  # dmesg -wH
  [...] videodev: v4l2_open: video3: open (0)
  [  +0.000036] video3: VIDIOC_QUERYCAP: driver=rkvdec, card=rkvdec,
  bus=platform:rkvdec, version=0x00060900, capabilities=0x84204000,
  device_caps=0x04204000

```
完整文档请参:ref:`driver-api/media/v4l2-dev:video device debugging`

### dev_dbg() / v4l2_dbg()


两个特定于设备和 v4l2 子系统的调试打印语句，除非它们对调查具有长期价值，否则不要把它们加入你的最终提交中
概览请参process/debugging/driver_development_debugging_guide:printk() & friends 指南
- 两者区别？

  - v4l2_dbg() 底层使用 v4l2_printk()，后者进一步直接使printk()，因此无法被 dynamic debug 定位
  - dev_dbg() 可以dynamic debug 定位
  - v4l2_dbg() media 子系统有更特定的前缀格式，dev_dbg 只高亮显示驱动名和日志位
### Dynamic debug


一种根据你的需要裁剪调试输出的方法
一般性建议请参阅 process/debugging/userspace_debugging_guide:dynamic debug 指南
```

  $ alias ddcmd='echo $* > /proc/dynamic_debug/control'
  $ ddcmd '-p; file v4l2-h264.c +p'
  $ grep =p /proc/dynamic_debug/control
   drivers/media/v4l2-core/v4l2-h264.c:372 [v4l2_h264]print_ref_list_b =p
   "ref_pic_list_b%u (cur_poc %u%c) %s"
   drivers/media/v4l2-core/v4l2-h264.c:333 [v4l2_h264]print_ref_list_p =p
   "ref_pic_list_p (cur_poc %u%c) %s\n"

```
### Ftrace


一种可以追踪静态预定义事件、函数调用等的内核内tracer。对于在不修改内核的情况下调试问题以及理解子系统的行为非常有用
一般性建议请参阅 process/debugging/userspace_debugging_guide:ftrace 指南
### DebugFS


该工具允许你把驱动的内部值转储或修改到自定义文件系统中的文件里
一般性建议请参阅 process/debugging/driver_development_debugging_guide:debugfs 指南
### Perf & alternatives


用于在运行中的系统上测量各种统计信息以诊断问题的工具
一般性建议请参阅 process/debugging/userspace_debugging_guide:perf & alternatives 指南
media 设备示例
收集解码任务的统计数据：（此示例在带rkvdec 编解码器驱动、使`fluster test suite
```

  perf stat -d python3 fluster.py run -d GStreamer-H.264-V4L2SL-Gst1.0 -ts
  JVT-AVC_V1 -tv AUD_MW_E -j1
  ...
  Performance counter stats for 'python3 fluster.py run -d
  GStreamer-H.264-V4L2SL-Gst1.0 -ts JVT-AVC_V1 -tv AUD_MW_E -j1 -v':

         7794.23 msec task-clock:u                     #    0.697 CPUs utilized
               0      context-switches:u               #    0.000 /sec
               0      cpu-migrations:u                 #    0.000 /sec
           11901      page-faults:u                    #    1.527 K/sec
       882671556      cycles:u                         #    0.113 GHz                         (95.79%)
       711708695      instructions:u                   #    0.81  insn per cycle              (95.79%)
        10581935      branches:u                       #    1.358 M/sec                       (15.13%)
         6871144      branch-misses:u                  #   64.93% of all branches             (95.79%)
       281716547      L1-dcache-loads:u                #   36.144 M/sec                       (95.79%)
         9019581      L1-dcache-load-misses:u          #    3.20% of all L1-dcache accesses   (95.79%)
 <not supported>      LLC-loads:u
 <not supported>      LLC-load-misses:u

    11.180830431 seconds time elapsed

     1.502318000 seconds user
     6.377221000 seconds sys

```
可用事件和指标取决于你所运行的系统
### Error checking & panic analysis


各种内核配置选项，以增强 Linux 内核的错误检测能力，代价是降低性能
一般性建议请参阅 :ref:`process/debugging/driver_development_debugging_guide:kasan, ubsan, lockdep and other error checkers` 指南
### Driver verification with v4l2-compliance


为了验证驱动是否遵循 v4l2 API，使用工v4l2-compliance，它`v4l_utils <https://git.linuxtv.org/v4l-utils.git>`__ 的一部分，后者是一套用media 子系统的用户空间工具
```

  v4l2-compliance -M /dev/mediaX --verbose

```
你也可以mediaX 引用的所有设备运行完整的合规性检```

  v4l2-compliance -m /dev/mediaX

```
### Debugging problems with receiving video


在驱动中实现 vidioc_log_status：这可以把当前状态记录到内核日志。它v4l2-ctl --log-status 调用。对于调试接收视频（TV/S-Video/HDMI 等）的问题非常有用，因为视频信号是外部的（因此不可预测）。对于摄像头传感器输入用处较小，因为你可以控制摄像头传感器的行为
```

  .vidioc_log_status  = v4l2_ctrl_log_status,

```
但你也可以创建自己的回调，以创建自定义的状态日志
你可以在 cobalt 驱动中找到一个示例（`drivers/media/pci/cobalt/cobalt-v4l2.c <https://elixir.bootlin.com/linux/v6.11.6/source/drivers/media/pci/cobalt/cobalt-v4l2.c#L567>`__）
**Copyright** 漏2024 : Collabora
