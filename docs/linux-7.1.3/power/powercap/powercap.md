## 功耗封顶（Power Capping）框架


功耗封顶框架在内核与用户空间之间提供了一致的接口，使得功耗封顶驱动能够以统一的方式将相关设置暴露给用户空间。

## 术语


该框架通过 sysfs 以对象树的形式将功耗封顶设备暴露给用户空间。树根层级的对象代表“控制类型（control types）”，对应于不同的功耗封顶方法。例如，intel-rapl 控制类型代表 Intel 的“运行平均功率限制”（Running Average Power Limit，RAPL）技术，而 idle-injection 控制类型则对应于使用空闲注入（idle injection）来控制功耗。

功耗区（power zone）代表系统中可以进行控制和监控的不同部分，使用其所归属的控制类型所确定的功耗封顶方法。它们各自包含用于监控功耗的属性，以及以功耗约束（power constraint）形式表示的控件。如果由不同功耗区所代表的系统各部分之间存在层次关系（即一个较大的部分由多个较小的、各自拥有独立功耗控制的部分组成），那么这些功耗区也可以组织成层次结构：一个父功耗区包含多个子区，以此类推，以反映系统的功耗控制拓扑。在这种情况下，可以通过父功耗区将功耗封顶同时应用于一组设备；如果需要更细粒度的控制，则可以通过子区来施加。

```
  /sys/devices/virtual/powercap
  └──intel-rapl
      ├──intel-rapl:0
      │   ├──constraint_0_name
      │   ├──constraint_0_power_limit_uw
      │   ├──constraint_0_time_window_us
      │   ├──constraint_1_name
      │   ├──constraint_1_power_limit_uw
      │   ├──constraint_1_time_window_us
      │   ├──device -> ../../intel-rapl
      │   ├──energy_uj
      │   ├──intel-rapl:0:0
      │   │   ├──constraint_0_name
      │   │   ├──constraint_0_power_limit_uw
      │   │   ├──constraint_0_time_window_us
      │   │   ├──constraint_1_name
      │   │   ├──constraint_1_power_limit_uw
      │   │   ├──constraint_1_time_window_us
      │   │   ├──device -> ../../intel-rapl:0
      │   │   ├──energy_uj
      │   │   ├──max_energy_range_uj
      │   │   ├──name
      │   │   ├──enabled
      │   │   ├──power
      │   │   │   ├──async
      │   │   │   []
      │   │   ├──subsystem -> ../../../../../../class/power_cap
      │   │   └──uevent
      │   ├──intel-rapl:0:1
      │   │   ├──constraint_0_name
      │   │   ├──constraint_0_power_limit_uw
      │   │   ├──constraint_0_time_window_us
      │   │   ├──constraint_1_name
      │   │   ├──constraint_1_power_limit_uw
      │   │   ├──constraint_1_time_window_us
      │   │   ├──device -> ../../intel-rapl:0
      │   │   ├──energy_uj
      │   │   ├──max_energy_range_uj
      │   │   ├──name
      │   │   ├──enabled
      │   │   ├──power
      │   │   │   ├──async
      │   │   │   []
      │   │   ├──subsystem -> ../../../../../../class/power_cap
      │   │   └──uevent
      │   ├──max_energy_range_uj
      │   ├──max_power_range_uw
      │   ├──name
      │   ├──enabled
      │   ├──power
      │   │   ├──async
      │   │   []
      │   ├──subsystem -> ../../../../../class/power_cap
      │   ├──enabled
      │   ├──uevent
      ├──intel-rapl:1
      │   ├──constraint_0_name
      │   ├──constraint_0_power_limit_uw
      │   ├──constraint_0_time_window_us
      │   ├──constraint_1_name
      │   ├──constraint_1_power_limit_uw
      │   ├──constraint_1_time_window_us
      │   ├──device -> ../../intel-rapl
      │   ├──energy_uj
      │   ├──intel-rapl:1:0
      │   │   ├──constraint_0_name
      │   │   ├──constraint_0_power_limit_uw
      │   │   ├──constraint_0_time_window_us
      │   │   ├──constraint_1_name
      │   │   ├──constraint_1_power_limit_uw
      │   │   ├──constraint_1_time_window_us
      │   │   ├──device -> ../../intel-rapl:1
      │   │   ├──energy_uj
      │   │   ├──max_energy_range_uj
      │   │   ├──name
      │   │   ├──enabled
      │   │   ├──power
      │   │   │   ├──async
      │   │   │   []
      │   │   ├──subsystem -> ../../../../../../class/power_cap
      │   │   └──uevent
      │   ├──intel-rapl:1:1
      │   │   ├──constraint_0_name
      │   │   ├──constraint_0_power_limit_uw
      │   │   ├──constraint_0_time_window_us
      │   │   ├──constraint_1_name
      │   │   ├──constraint_1_power_limit_uw
      │   │   ├──constraint_1_time_window_us
      │   │   ├──device -> ../../intel-rapl:1
      │   │   ├──energy_uj
      │   │   ├──max_energy_range_uj
      │   │   ├──name
      │   │   ├──enabled
      │   │   ├──power
      │   │   │   ├──async
      │   │   │   []
      │   │   ├──subsystem -> ../../../../../../class/power_cap
      │   │   └──uevent
      │   ├──max_energy_range_uj
      │   ├──max_power_range_uw
      │   ├──name
      │   ├──enabled
      │   ├──power
      │   │   ├──async
      │   │   []
      │   ├──subsystem -> ../../../../../class/power_cap
      │   ├──uevent
      ├──power
      │   ├──async
      │   []
      ├──subsystem -> ../../../../class/power_cap
      ├──enabled
      └──uevent
```

上述示例展示了使用 Intel® IA-64 与 IA-32 处理器架构中可用的 Intel RAPL 技术的情况。其中有一个名为 intel-rapl 的控制类型，它包含两个功耗区 intel-rapl:0 与 intel-rapl:1，代表 CPU 封装（package）。每个功耗区又包含两个子区 intel-rapl:j:0 与 intel-rapl:j:1（j = 0, 1），分别代表该 CPU 封装的“核心（core）”与“非核心（uncore）”部分。所有的区与子区都包含能耗监控属性（energy_uj、max_energy_range_uj）以及约束属性（constraint_*），用以施加控制（“封装（package）”功耗区中的约束作用于整个 CPU 封装，而子区约束只分别作用于该封装各自的部分）。由于 Intel RAPL 不提供瞬时功率值，因此没有 power_uw 属性。

此外，每个功耗区还包含一个 name 属性，用于标识该区所代表的系统部分。

```
	cat /sys/class/power_cap/intel-rapl/intel-rapl:0/name
```

### package-0


根据功耗区的不同，Intel RAPL 技术允许对各个功耗区施加一个或多个约束，例如短期、长期以及峰值功率，并带有不同的时间窗口。
所有的区都包含代表约束名称、功率限制以及时间窗口大小的属性。注意，时间窗口不适用于峰值功率。这里的 constraint_j_* 属性对应于第 j 个约束（j = 0,1,2）。

```
	constraint_0_name
	constraint_0_power_limit_uw
	constraint_0_time_window_us
	constraint_1_name
	constraint_1_power_limit_uw
	constraint_1_time_window_us
	constraint_2_name
	constraint_2_power_limit_uw
	constraint_2_time_window_us
```

## 功耗区属性


### 监控属性


energy_uj (rw)
	当前能耗计数器，单位为微焦（micro joules）。写入 “0” 以重置。
	如果计数器无法重置，则该属性为只读。

max_energy_range_uj (ro)
	上述能耗计数器的范围，单位为微焦。

power_uw (ro)
	当前功率，单位为微瓦。

max_power_range_uw (ro)
	上述功率值的范围，单位为微瓦。

name (ro)
	本功耗区的名称。

某些域可能同时具有功率范围与能耗计数器范围；不过，二者中只有一个是必须的。

### 约束


constraint_X_power_limit_uw (rw)
	功率限制，单位为微瓦，应适用于由 “constraint_X_time_window_us” 指定的
	时间窗口。

constraint_X_time_window_us (rw)
	时间窗口，单位为微秒。

constraint_X_name (ro)
	约束的可选名称。

constraint_X_max_power_uw (ro)
	允许的最大功率，单位为微瓦。

constraint_X_min_power_uw (ro)
	允许的最小功率，单位为微瓦。

constraint_X_max_time_window_us (ro)
	允许的最大时间窗口，单位为微秒。

constraint_X_min_time_window_us (ro)
	允许的最小时间窗口，单位为微秒。

除 power_limit_uw 与 time_window_us 外，其余字段均为可选。

### 通用区与控制类型属性


enabled (rw)：在区级别或使用某个控制类型对所有区启用/禁用控制。

## 功耗封顶客户端驱动接口


API 概要：

调用 powercap_register_control_type() 注册控制类型对象。
调用 powercap_register_zone() 注册一个功耗区（在某个给定的控制类型下），
既可以作为顶层功耗区，也可以作为先前注册的另一个功耗区的子区。
在调用 powercap_register_zone() 注册某个功耗区之前，必须先定义该区中
约束的数量以及相应的回调函数。

要释放一个功耗区，调用 powercap_unregister_zone()。
要释放一个控制类型对象，调用 powercap_unregister_control_type()。
详细的 API 可以通过对 include/linux/powercap.h 使用 kernel-doc 生成。
