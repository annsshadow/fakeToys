## 显示核心调试工具（Display Core Debug tools）

本节将为你提供从显示角度调试 amdgpu 驱动的有用信息。本页介绍了一些调试机制与流程，
以帮助你判断某些问题是否与显示代码相关。

## 缩小显示问题的范围（Narrow down display issues）

由于显示是驱动的可视化组件，用户经常把由其它组件引起的问题当作显示问题来上报。本节
帮助用户判断某个具体问题是否由显示组件还是驱动中的其它部分所引起。

### DC 相关的 dmesg 重要信息（DC dmesg important messages）

dmesg 日志是要检查的第一手信息来源，amdgpu 充分利用了这一特性，记录了一些有价值的信息。
在查找与 amdgpu 相关的问题时，请记住驱动的每一个组件（例如 smu、PSP、dm 等）是逐个加载的，
这些信息都能在 dmesg 日志中找到。从这个意义上说，请查找如下部分：

```

  [    4.254295] [drm] initializing kernel modesetting (IP DISCOVERY 0x1002:0x744C 0x1002:0x0E3B 0xC8).
  [    4.254718] [drm] register mmio base: 0xFCB00000
  [    4.254918] [drm] register mmio size: 1048576
  [    4.260095] [drm] add ip block number 0 <soc21_common>
  [    4.260318] [drm] add ip block number 1 <gmc_v11_0>
  [    4.260510] [drm] add ip block number 2 <ih_v6_0>
  [    4.260696] [drm] add ip block number 3 <psp>
  [    4.260878] [drm] add ip block number 4 <smu>
  [    4.261057] [drm] add ip block number 5 <dm>
  [    4.261231] [drm] add ip block number 6 <gfx_v11_0>
  [    4.261402] [drm] add ip block number 7 <sdma_v6_0>
  [    4.261568] [drm] add ip block number 8 <vcn_v4_0>
  [    4.261729] [drm] add ip block number 9 <jpeg_v4_0>
  [    4.261887] [drm] add ip block number 10 <mes_v11_0>

```
从上面的示例可以看到报告 `<dm>`（**Display Manager**，显示管理器）已加载的那一行，这意味着
显示可能是问题的一部分。如果你没有看到这一行，说明在 amdgpu 加载显示组件之前，某个其他部分
可能已经失败，表明我们遇到的问题并非显示问题。

在确认 DM 已正确加载之后，你可以检查正在使用的硬件所对应的显示版本，它可以从 dmesg 中获取：
```

  dmesg | grep -i 'display core'

```
```

  [    4.655828] [drm] Display Core v3.2.285 initialized on DCN 3.2

```
这条信息包含两个关键信息：

- **DC 版本（例如 v3.2.285）**：显示开发者每周都会发布一个新的 DC 版本，当用户/开发者必须
  根据经过测试的显示代码版本来寻找好点与坏点的差异时，这个信息会很有用。请记住在
  Display Core <amdgpu-display-core> 页面中提到，每周都会用 IGT 和手动测试对新显示补丁进行
  充分测试。
- **DCN 版本（例如 DCN 3.2）**：DCN 模块与硬件代次相关，DCN 版本反映了驱动当前运行的硬件代次。
  这条信息有助于缩小代码调试范围，因为每个 DCN 版本在 DC 目录下都有各自对应的 DCN 组件文件
  （从示例来看，开发者可能希望重点关注带有 dcn32 标签的文件/文件夹/函数/结构体）。不过请注意，
  DC 会在不同的 DCN 版本之间复用代码；例如，某个 DCN 中设置的部分回调与另一个 DCN 中的回调是相同的。
  总之，请把 DCN 版本仅作为参考。

```

  dmesg  | grep -i 'ATOM BIOS'

```
```

  [    4.274534] amdgpu: ATOM BIOS: 113-D7020100-102

```
这类信息在报告问题时很有用。

### 避免加载显示核心（Avoid loading display core）

有时，可能难以判断驱动中的哪一部分引发了问题；如果你怀疑显示并非问题所在，并且你的
缺陷场景较为简单（例如某些桌面环境配置），你可以尝试从方程中移除显示组件。首先，你需要
识别 `dm` 的 ID：
```

  [    4.254295] [drm] initializing kernel modesetting (IP DISCOVERY 0x1002:0x744C 0x1002:0x0E3B 0xC8).
  [..]
  [    4.260095] [drm] add ip block number 0 <soc21_common>
  [    4.260318] [drm] add ip block number 1 <gmc_v11_0>
  [..]
  [    4.261057] [drm] add ip block number 5 <dm>

```
从上面的示例可以看出，对于这个特定硬件，`dm` 的 id 为 5。接下来，你需要运行如下二进制运算
来识别 IP 块：
```

  0xffffffff & ~(1 << [DM ID])

```
```

 0xffffffff & ~(1 << 5) = 0xffffffdf

```
最后，要禁用 DC，只需要在你的配置中设置如下参数：
```

 amdgpu.ip_block_mask = 0xffffffdf

```
如果你能在禁用 DC 的情况下启动系统并且仍然能看到该问题，说明你可以把 DC 排除在原因之外。
但如果缺陷消失了，你仍然需要把 DC 视为问题的一部分，并继续缩小范围。在某些场景下，禁用 DC
是不可能的，因为可能需要用到显示组件才能复现该问题（例如玩游戏）。

**注意：这很可能会导致没有显示输出。**

### 显示闪烁（Display flickering）

显示闪烁可能有多种原因，其中一种原因是 GPU 供电不足或 DPM 开关存在问题。一个不错的通用
初步检查是：
```

   bash -c "echo high > /sys/class/drm/card0/device/power_dpm_force_performance_level"

```
上述命令将 GPU/APU 设置为使用允许的最大功耗，从而禁用 DPM 开关。如果强制把 DPM 级别调到
高电平并不能修复问题，那么该问题与电源管理相关的可能性就较小。如果问题消失了，则很有可能是
其它组件牵涉其中，并且不应忽视显示部分，因为这可能是一个 DPM 问题。从显示角度看，如果
提高功耗修复了问题，那么值得去调试该特定配置下所使用的时钟配置与 pipe split 策略。

### 显示残影/瑕疵（Display artifacts）

用户可能会看到一些屏幕瑕疵，可分为两种不同类型：局部瑕疵与全局瑕疵。局部瑕疵发生在某些
特定区域，例如 UI 窗口角落附近；如果你看到这类问题，有相当大的概率是你遇到了用户空间问题，
很可能是 Mesa 或类似软件。全局瑕疵通常发生在整个屏幕上。它们可能是由驱动层面对显示参数的
配置错误引起的，但用户空间也可能导致此问题。识别问题根源的一种方法是，在问题发生时
截屏或进行桌面视频采集；在查看截屏/视频录制后，如果你没有看到任何瑕疵，说明问题很可能
出在驱动一侧。如果你在采集到的数据中仍然能看到问题，那这是一个很可能发生在渲染阶段、
只是显示代码拿到了已损坏的帧缓冲的问题。

## 禁用/启用特定功能（Disabling/Enabling specific features）

DC 有一个名为 `dc_debug_options` 的结构体，它由所有 DCE/DCN 组件根据特定硬件特性静态初始化。
该结构通常有助于初启（bring-up）阶段，因为开发者可以从许多被禁用的功能开始，再逐个启用它们。
这也是一个重要的调试特性，因为用户在调试特定问题时可以修改它。

例如，独显（dGPU）用户有时会发现屏幕某个特定位置出现一条横向的闪烁带。这可能是 Sub-Viewport
问题的迹象；在用户识别出目标 DCN 之后，他们可以把静态初始化的 `dc_debug_options` 中的
`force_disable_subvp` 字段设置为 true，看看问题是否得到修复。同理，用户/开发者也可以尝试
关闭 `fams2_config` 和 `enable_single_display_2to1_odm_policy`。总之，`dc_debug_options`
是一种有趣的定位问题的方式。

## DC 可视化确认（DC Visual Confirmation）

显示核心提供了一个名为"可视化确认"（visual confirmation）的特性，它是在扫描输出（scanout）
时由驱动添加的一组色条，用于传达某些特定信息：
```

  echo <N> > /sys/kernel/debug/dri/0/amdgpu_dm_visual_confirm

```
其中 `N` 是开发者想要启用的某些特定场景对应的整数，你将在下面的小节中看到其中一些调试用例。

### 多平面调试（Multiple Planes Debug）

如果你想要在某个特定的用户空间应用程序中启用或调试多个平面（plane），可以利用一个名为
visual confirm 的调试特性：
```

  echo 1 > /sys/kernel/debug/dri/0/amdgpu_dm_visual_confirm

```
你需要重新加载你的 GUI 才能看到可视化确认。当平面配置发生变化或发生一次完整更新时，屏幕上
正在绘制的每个硬件平面底部都会出现一条彩色条。

- 颜色表示格式 —— 例如，红色是 AR24，绿色是 NV12
- 条的高度表示平面的索引
- 如果出现两条高度不同、覆盖同一平面的色条，则可以看到发生了 pipe split

考虑视频播放场景：视频在一个特定平面中播放，而桌面绘制在另一个平面中。根据 pipe split 配置
的不同，视频平面底部应出现一条或两条绿色条。

- 应该**不**出现任何视觉损坏
- 应该**不**出现任何下溢（underflow）或屏幕闪烁
- 应该**不**出现任何黑屏
- 应该**不**出现任何光标损坏
- 多平面在窗口切换或缩放期间**可能**被短暂禁用，但操作完成后应当恢复

### Pipe Split 调试（Pipe Split Debug）

有时我们需要调试 DCN 是否正确地进行了 pipe split，可视化确认在这种情况下也很方便。与 MPO
的情况类似，你可以使用：
```

  echo 1 > /sys/kernel/debug/dri/0/amdgpu_dm_visual_confirm

```
在这种情况下，如果你有 pipe split，你会在显示底部看到一条覆盖整个显示宽度的细小红色条，
以及另一条覆盖第二个 pipe 的条。换句话说，你会在第二个 pipe 中看到一个略高的条。

## DTN 调试（DTN Debug）

DC（DCN）提供了一个详尽的日志，会导出我们硬件配置的诸多细节。通过 debugfs，你可以采集这些
状态值：
```

  cat /sys/kernel/debug/dri/0/amdgpu_dm_dtn_log

```
由于该日志会随着 DCN 状态同步更新，你还可以执行如下命令来跟踪：
```

  sudo watch -d cat /sys/kernel/debug/dri/0/amdgpu_dm_dtn_log

```
在报告与 DC 相关的缺陷时，请考虑在复现缺陷前后都附上该日志。

## 采集固件信息（Collect Firmware information）

在报告问题时，拥有固件信息很重要，因为它有助于调试。要获取所有固件信息，可以执行：
```

  cat /sys/kernel/debug/dri/0/amdgpu_firmware_info

```
从显示角度看，请关注 DMCU 与 DMCUB 的固件。

## DMUB 固件调试（DMUB Firmware Debug）

有时，dmesg 日志信息并不够用。当某个特性主要在 DMUB 固件中实现时尤其如此。在这种情况下，
问题出现时我们在 dmesg 中看到的只是一些通用的超时错误。因此，为了获得更相关的信息，我们
可以通过在 `amdgpu_dm_dmub_trace_mask` 中启用相应位来跟踪 DMUB 命令。

目前，我们支持跟踪以下分组：

### 跟踪分组（Trace Groups）

   :header-rows: 1
   :widths: 1, 1
   :file: ./trace-groups-table.csv

**注意：并非所有 ASIC 都支持列出的全部跟踪分组**

```

  # echo 0x8020 > /sys/kernel/debug/dri/0/amdgpu_dm_dmub_trace_mask

```
然后，你需要启用将跟踪事件记录到缓冲区的功能，可以执行：
```

  # echo 1 > /sys/kernel/debug/dri/0/amdgpu_dm_dmcub_trace_event_en

```
最后，在你能够复现想要调试的问题之后，执行：
```

  # echo 0 > /sys/kernel/debug/dri/0/amdgpu_dm_dmcub_trace_event_en
  # cat /sys/kernel/debug/dri/0/amdgpu_dm_dmub_tracebuffer

```
因此，在报告与 PSR 和 ABM 等特性相关的缺陷时，请考虑在复现问题之前在掩码中启用相应的位，
并在你创建的任何缺陷报告中附上从跟踪缓冲区中获取的日志。
