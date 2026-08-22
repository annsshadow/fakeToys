
##  drm/vkms 虚拟内核模式设置（Virtual Kernel Modesetting

   :doc: vkms (Virtual Kernel Modesetting)

## 设置


VKMS 驱动可以通过以下步骤进行设置
```
  lsmod | grep vkms

```
这应当会列出 VKMS 驱动。如果没有任何输出，那么你需要启用和/或加VKMS 驱动
确保 VKMS 驱动已在你的
```
  make nconfig

  Go to `Device Drivers> Graphics support`

  Enable `Virtual KMS (EXPERIMENTAL)`

```
编译并构建内核，以使更改生效```

  sudo modprobe vkms

```
现在运行 lsmod 命令，VKMS 驱动将会出现在列表中。你也可以观察到驱动dmesg 日志中被加载
VKMS 驱动具有用于模拟不同类型硬件的可选特性，它们作为模块选项暴露出来。你可以使用 `modinfo` 命令
```
  modinfo vkms

```
模块选项在测试时很有帮助，并且可以在加载 vkms 时启用模块。例如，要加载启用了光标vkms```
  sudo modprobe vkms enable_cursor=1

```
```
  sudo modprobe -r vkms

```
## 通过 Configfs 配置


可以通过 configfs 创建并配置多VKMS 实例
```
  sudo mount -t configfs none /config
  sudo modprobe vkms

```
一VKMS 被加载，`/config/vkms` 会自动创建。每个目```
  sudo mkdir /config/vkms/my-vkms

```
```
  cat /config/vkms/my-vkms/enabled
  0

```
```
  tree /config/vkms/my-vkms
  鈹溾攢鈹€ connectors
  鈹溾攢鈹€ crtcs
  鈹溾攢鈹€ enabled
  鈹溾攢鈹€ encoders
  鈹斺攢鈹€ planes

```
要向显示流水线添加项目，在可用路径下创建一个或多个目录
```
  sudo mkdir /config/vkms/my-vkms/planes/plane0

```
平面（Plane）有 1 个可配置属性：

- type：平面类型：0 叠加层（overlay），1 主平面（primary），2 光标（cursor）（与平面的 "type" 属性暴露的值相同）

```
  sudo mkdir /config/vkms/my-vkms/crtcs/crtc0

```
CRTC 1 个可配置属性：

- writeback：通过写入 1 0 来启用或禁用回写（writeback）连接器支持

```
  sudo mkdir /config/vkms/my-vkms/encoders/encoder0

```
```
  sudo mkdir /config/vkms/my-vkms/connectors/connector0

```
连接器（Connector）有 1 个可配置属性：

- status：连接状态：1 已连接，2 已断开 未知（与连接器的 "status" 属性暴露的值相同）

```
  sudo ln -s /config/vkms/my-vkms/crtcs/crtc0 /config/vkms/my-vkms/planes/plane0/possible_crtcs
  sudo ln -s /config/vkms/my-vkms/crtcs/crtc0 /config/vkms/my-vkms/encoders/encoder0/possible_crtcs
  sudo ln -s /config/vkms/my-vkms/encoders/encoder0 /config/vkms/my-vkms/connectors/connector0/possible_encoders

```
```
  echo "1" | sudo tee /config/vkms/my-vkms/planes/plane0/type

```
```
  echo "1" | sudo tee /config/vkms/my-vkms/enabled

```
```
  echo "0" | sudo tee /config/vkms/my-vkms/enabled

```
```
  sudo rm /config/vkms/my-vkms/planes/*/possible_crtcs/*
  sudo rm /config/vkms/my-vkms/encoders/*/possible_crtcs/*
  sudo rm /config/vkms/my-vkms/connectors/*/possible_encoders/*
  sudo rmdir /config/vkms/my-vkms/planes/*
  sudo rmdir /config/vkms/my-vkms/crtcs/*
  sudo rmdir /config/vkms/my-vkms/encoders/*
  sudo rmdir /config/vkms/my-vkms/connectors/*
  sudo rmdir /config/vkms/my-vkms

```
## 使用 IGT 测试


IGT GPU Tools 是一个专门用DRM 驱动调试和开发的测试套件
IGT 工具可以`here <https://gitlab.freedesktop.org/drm/igt-gpu-tools>`_ 安装
测试需要在没有合成器（compositor）的情况下运行，所以你需要切换到文本
```
  sudo systemctl isolate multi-user.target

```
```
  sudo systemctl isolate graphical.target

```
一旦进入纯文本模式，你就可以使IGT_FORCE_DRIVER 变量来指定想要测试的驱动的设备过滤器来运行测试
IGT_FORCE_DRIVER 也可以与 run-tests.sh 脚本一起使用来运行
```
  sudo IGT_FORCE_DRIVER="vkms" ./build/tests/<name of test>
  sudo IGT_FORCE_DRIVER="vkms" ./scripts/run-tests.sh -t <name of test>

```
例如，要测试回写（writeback）库的功能，
```
  sudo IGT_FORCE_DRIVER="vkms" ./build/tests/kms_writeback
  sudo IGT_FORCE_DRIVER="vkms" ./scripts/run-tests.sh -t kms_writeback

```
```
  sudo IGT_FORCE_DRIVER="vkms" ./build/tests/kms_flip --run-subtest basic-plain-flip

```
## 使用 KUnit 测试


KUnit（内核单元测试框架）Linux 内核中的单元测试提供了一个通用框架
更多信息../dev-tools/kunit/index.rst
```
  tools/testing/kunit/kunit.py run --kunitconfig=drivers/gpu/drm/vkms/tests

```
## TODO


如果你想做下面列出的任何一项，请与 VKMS 维护者分享你的兴趣
### 改进 IGT 支持


调试
- kms_plane：一些测试用例因为捕CRC 超时而失败；

虚拟硬件（无 vblank）模式：

- VKMS 已经支持通过 hrtimer 模拟vblank，可以用 kms_flip 测试来验证；在某种程度上，可以说 VKMS 已经模拟了真实硬件的 vblank。不过，我们也有不支vblank 中断、并立即完成 page_flip 事件的虚拟硬件；在这种情况下，合成器开发者可能会在虚拟硬件上陷入忙循环。在 VKMS 中支持虚拟硬件行为会很有用，因为这可以帮助合成器开发者在多种场景下测试他们的特性
### 添加平面特

有很多平面特性我们可以增加支持：

- 添加背景KMS 属性[适合入门]
- 缩放（Scaling）
- 额外的缓冲区格式。低/高位深（bpp）的 RGB 格式会很有意思[适合入门]
- 异步更新（目前仅能使用旧cursor api 在光标平面上实现）
对于所有这些，我们也希望审igt 测试覆盖率，并确保所有相关的 igt 测试用例vkms 上正常工作。它们是实习项目的不错选择
### 运行时配

我们希望能够重新配置 vkms 实例，而无需通过 configfs 重新加载模块。使测试用例
- 动态热插拔/热移除连接器（以便能够测试合成器DP MST 的处理）
- 更改输出配置：插拔屏幕、更EDID、允许更改刷新率
### 回写支持


- 回写CRC 捕获操作共享 composer_enabled 布尔值的使用以确vblank。可能当这些操作一起工作时，composer_enabled 需要对 composer 状态做引用计数才能正常工作。[适合入门]

- 增加对克隆回写输出的支持，以及相关的测试用例（在 IGT kms_writeback 中使用克隆输出）
- 作为一v4l 设备。这对于在特vkms 配置上调试合成器很有用，以便开发者看到真正发生的情况
### 输出特

- 可变刷新freesync 支持。这可能 Prime 缓冲区共享支持，以便我们可以使用 vgem fence 在测试中模拟渲染。还需要支持指EDID
- 增加link status 的支持，以便合成器可以在例如 Display Port 链路出问题时验证它们的运行时回退方案
### CRC API 改进


- 优化 CRC 计算 `compute_crc()` 和平面混`blend()`

### 使用 eBPF 进行原子检

原子（Atomic）驱动有许多限制，这些限制并未以任何显式形式（例如通过可能的属性值）暴露给用户空间。用户空间只能通过 atomic IOCTL（可能使TEST_ONLY 标志）来查询这些限制。试图为所有这些限制添加可配置代码，以便合成器能够针对它们被测试，将是一项相当徒劳的工作。相反，我们可以增加eBPF 的支持来验证任何类型的原子状态，并实现一个包含不同限制的库
这需要一大批特性（平面合成、多输出……）已经启用才有意义