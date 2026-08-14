
##  AMDGPU 进程隔离


AMDGPU 驱动包含一个特性，可在图形引擎上启用自动进程隔离。该特性对图形引擎的访问进行串行化，并添加一个 cleaner shader，在各作业之间清除局部数据存储（LDS）与通用寄存器（GPR）。当启用此特性时，所有使用 GPU 的进程（包括图形与计算工作负载）都会被串行化。在支持可分区图形引擎的 GPU 上，此特性可以按分区启用。

此外，还提供了一个接口，可在 GPU 使用完毕后手动运行 cleaner shader。这在某些用例中可能更可取，例如单用户系统中，登录管理器在用户登出时触发 cleaner shader。

## 进程隔离


`run_cleaner_shader` 与 `enforce_isolation` sysfs 接口分别允许用户手动执行 cleaner shader 并控制进程隔离特性。

### 分区处理


sysfs 中的 `enforce_isolation` 文件可用于启用进程隔离以及在进程间自动清理 shader。在支持图形引擎分区的 GPU 上，可以按分区启用。分区及其当前设置（0 禁用，1 启用）可从 sysfs 读取。在不支持图形引擎分区的 GPU 上，将只存在一个分区。向分区位置写入 1 启用强制隔离，写入 0 禁用它。

在多分区 GPU 上启用强制隔离的示例：


    $ echo 1 0 1 0 > /sys/class/drm/card0/device/enforce_isolation
    $ cat /sys/class/drm/card0/device/enforce_isolation
    1 0 1 0

输出表明强制隔离在第零个与第二个分区上启用，在第一个与第三个分区上禁用。

对于单分区或不支持分区的设备，将只有一个元素：


    $ echo 1 > /sys/class/drm/card0/device/enforce_isolation
    $ cat /sys/class/drm/card0/device/enforce_isolation
    1

## Cleaner Shader 执行


驱动可以触发一个 cleaner shader 来清理图形引擎上的 LDS 与 GPR 状态。当启用进程隔离时，这会在进程间自动发生。此外，还有一个 sysfs 文件用于手动触发 cleaner shader 执行。

要手动触发 cleaner shader 的执行，向 `run_cleaner_shader` sysfs 文件写入 `0`：


    $ echo 0 > /sys/class/drm/card0/device/run_cleaner_shader

对于多分区设备，你可以在触发 cleaner shader 时指定分区索引：


    $ echo 0 > /sys/class/drm/card0/device/run_cleaner_shader # 对于分区 0
    $ echo 1 > /sys/class/drm/card0/device/run_cleaner_shader # 对于分区 1
    $ echo 2 > /sys/class/drm/card0/device/run_cleaner_shader # 对于分区 2
    # ... 依此类推，每个分区

此命令启动 cleaner shader，它将在 GPU 上调度任何新任务之前运行并完成。
