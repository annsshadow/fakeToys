
##  drm/Panfrost Mali 驱动


## Panfrost DRM 客户端使用统计实现


drm/Panfrost 驱动实现了 DRM 客户端使用统计规范，如
drm-client-usage-stats 中所述。

以下示例展示了已实现的键值对以及当前
所有可能的格式选项：

```
      pos:    0
      flags:  02400002
      mnt_id: 27
      ino:    531
      drm-driver:     panfrost
      drm-client-id:  14
      drm-engine-fragment:    1846584880 ns
      drm-cycles-fragment:    1424359409
      drm-maxfreq-fragment:   799999987 Hz
      drm-curfreq-fragment:   799999987 Hz
      drm-engine-vertex-tiler:        71932239 ns
      drm-cycles-vertex-tiler:        52617357
      drm-maxfreq-vertex-tiler:       799999987 Hz
      drm-curfreq-vertex-tiler:       799999987 Hz
      drm-total-memory:       290 MiB
      drm-shared-memory:      0 MiB
      drm-active-memory:      226 MiB
      drm-resident-memory:    36496 KiB
      drm-purgeable-memory:   128 KiB

```
可能的 `drm-engine-` 键名为：`fragment`，以及 `vertex-tiler`。
`drm-curfreq-` 值表示该引擎当前的运行频率。

用户必须注意，出于省电考虑，引擎和周期采样默认是禁用的，
`fdinfo` 用户和基准测试应用程序在查询 fdinfo 文件时
必须确保切换任务的性能分析状态：

```
    echo <N> > /sys/bus/platform/drivers/panfrost/[a-f0-9]*.gpu/profiling

```
其中 `N` 为 `0` 或 `1`，取决于期望的启用状态。
