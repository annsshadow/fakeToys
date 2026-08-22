
## drm/Panthor CSF 驱动


## Panthor DRM 客户端使用统计实

drm/Panthor 驱动实现DRM 客户端使用统计规范，drm-client-usage-stats 中所述
展示所实现的键值对以及当前可能格式选项的完整性的输出示例
```
     pos:    0
     flags:  02400002
     mnt_id: 29
     ino:    491
     drm-driver:     panthor
     drm-client-id:  10
     drm-engine-panthor:     111110952750 ns
     drm-cycles-panthor:     94439687187
     drm-maxfreq-panthor:    1000000000 Hz
     drm-curfreq-panthor:    1000000000 Hz
     panthor-resident-memory:        10396 KiB
     panthor-active-memory:  10396 KiB
     drm-total-memory:       16480 KiB
     drm-shared-memory:      0
     drm-active-memory:      16200 KiB
     drm-resident-memory:    16480 KiB
     drm-purgeable-memory:   0

```
可能`drm-engine-` 键名有：`panthor``drm-curfreq-` 值表示该引擎当前的运行频率
用户必须记住，出于省电考虑，引擎和周期采样默认是禁用的。`fdinfo` 用户以及查询 fdinfo 文件的基准测试应用程序必须确保切换作业的性能分析状态：
```

    echo <N> > /sys/bus/platform/drivers/panthor/[a-f0-9]*.gpu/profiling

```
其中 `N` 是一个位掩码，其中周期和时间戳采样分别由第一位和第二位启用
可能`panthor-*-memory` 键有：`active` `resident`这些值表示内部由驱动拥有shmem BO 的大小，这些 BO 没有通过 DRM handle 暴露给用户空间，例如队列环形缓冲区、同步对象数组和堆块。因为它们都在创建时分配并固定，所以只需 `panthor-resident-memory` 即可说明它们的大小。`panthor-active-memory` 显示当前正被 GPU 调度执行、与 VM 和组关联的驱BO 的大小