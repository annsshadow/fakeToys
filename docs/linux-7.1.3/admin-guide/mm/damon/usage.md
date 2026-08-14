
## 详细用法


DAMON 为不同用户提供了以下接口。

- **专用 DAMON 模块。** This <damon_modules_special_purpose> 面向那些使用专用 DAMON 用途来构建、分发和/或管理内核的人。使用它，用户可以以简单的方式在构建、启动或运行时针对给定目的使用 DAMON 的主要特性。
- **DAMON 用户空间工具。** This <https://github.com/damonitor/damo>_ 面向系统管理员等希望获得开箱即用、人性化的接口的特权用户。使用它，用户可以以人性化的方式使用 DAMON 的主要特性。不过，它可能没有针对特殊场景进行高度优化。更多细节，请参考其 usage document <https://github.com/damonitor/damo/blob/next/USAGE.md>_。
- **sysfs 接口。** This <sysfs_interface> 面向希望对 DAMON 进行更优化使用的特权用户空间程序员。使用它，用户可以通过读写特殊的 sysfs 文件来使用 DAMON 的主要特性。因此，你可以编写并使用你个性化的 DAMON sysfs 包装程序来替你读写这些 sysfs 文件。DAMON user space tool <https://github.com/damonitor/damo>_ 就是此类程序的一个例子。
- **内核空间编程接口。** [This </mm/damon/api>](This </mm/damon/api>) 面向内核空间程序员。使用它，用户可以通过为你编写内核空间 DAMON 应用程序，最灵活高效地利用 DAMON 的每一项特性。你甚至可以扩展 DAMON 以支持各种地址空间。更多细节，请参考 interface document </mm/damon/api>](document </mm/damon/api>)。


## sysfs 接口


DAMON sysfs 接口在定义了 `CONFIG_DAMON_SYSFS` 时构建。它在自己的 sysfs 目录 `<sysfs>/kernel/mm/damon/` 下创建多个目录和文件。你可以通过读写该目录下的文件来控制 DAMON。

作为一个简短示例，用户可以监控给定进程的虚拟地址空间，方法如下：

```

    # cd /sys/kernel/mm/damon/admin/
    # echo 1 > kdamonds/nr_kdamonds && echo 1 > kdamonds/0/contexts/nr_contexts
    # echo vaddr > kdamonds/0/contexts/0/operations
    # echo 1 > kdamonds/0/contexts/0/targets/nr_targets
    # echo $(pidof <workload>) > kdamonds/0/contexts/0/targets/0/pid_target
    # echo on > kdamonds/0/state

```
### 文件层级


DAMON sysfs 接口的文件层级如下所示。在下图中，父子关系用缩进表示，每个目录带有 `/` 后缀，每个目录中的文件用逗号（","）分隔。


    /sys/kernel/mm/damon <sysfs_root>/admin
    │ kdamonds <sysfs_kdamonds>/nr_kdamonds
    │ │ 0 <sysfs_kdamond>/state,pid,refresh_ms
    │ │ │ contexts <sysfs_contexts>/nr_contexts
    │ │ │ │ 0 <sysfs_context>/avail_operations,operations,addr_unit
    │ │ │ │ │ monitoring_attrs <sysfs_monitoring_attrs>/
    │ │ │ │ │ │ intervals/sample_us,aggr_us,update_us
    │ │ │ │ │ │ │ intervals_goal/access_bp,aggrs,min_sample_us,max_sample_us
    │ │ │ │ │ │ nr_regions/min,max
    │ │ │ │ │ targets <sysfs_targets>/nr_targets
    │ │ │ │ │ │ 0 <sysfs_target>/pid_target,obsolete_target
    │ │ │ │ │ │ │ regions <sysfs_regions>/nr_regions
    │ │ │ │ │ │ │ │ 0 <sysfs_region>/start,end
    │ │ │ │ │ │ │ │ ...
    │ │ │ │ │ │ ...
    │ │ │ │ │ schemes <sysfs_schemes>/nr_schemes
    │ │ │ │ │ │ 0 <sysfs_scheme>/action,target_nid,apply_interval_us
    │ │ │ │ │ │ │ access_pattern <sysfs_access_pattern>/
    │ │ │ │ │ │ │ │ sz/min,max
    │ │ │ │ │ │ │ │ nr_accesses/min,max
    │ │ │ │ │ │ │ │ age/min,max
    │ │ │ │ │ │ │ quotas <sysfs_quotas>/ms,bytes,reset_interval_ms,effective_bytes,goal_tuner
    │ │ │ │ │ │ │ │ weights/sz_permil,nr_accesses_permil,age_permil
    │ │ │ │ │ │ │ │ goals <sysfs_schemes_quota_goals>/nr_goals
    │ │ │ │ │ │ │ │ │ 0/target_metric,target_value,current_value,nid,path
    │ │ │ │ │ │ │ watermarks <sysfs_watermarks>/metric,interval_us,high,mid,low
    │ │ │ │ │ │ │ {core_,ops_,}filters <sysfs_filters>/nr_filters
    │ │ │ │ │ │ │ │ 0/type,matching,allow,memcg_path,addr_start,addr_end,target_idx,min,max
    │ │ │ │ │ │ │ dests <damon_sysfs_dests>/nr_dests
    │ │ │ │ │ │ │ │ 0/id,weight
    │ │ │ │ │ │ │ stats <sysfs_schemes_stats>/nr_tried,sz_tried,nr_applied,sz_applied,sz_ops_filter_passed,qt_exceeds,nr_snapshots,max_nr_snapshots
    │ │ │ │ │ │ │ tried_regions <sysfs_schemes_tried_regions>/total_bytes
    │ │ │ │ │ │ │ │ 0/start,end,nr_accesses,age,sz_filter_passed
    │ │ │ │ │ │ │ │ ...
    │ │ │ │ │ │ ...
    │ │ │ │ ...
    │ │ ...


### 根目录


DAMON sysfs 接口的根是 `<sysfs>/kernel/mm/damon/`，它有一个名为 `admin` 的目录。该目录包含供特权用户空间程序控制 DAMON 的文件。拥有 root 权限的用户空间工具或守护进程可以使用该目录。


### kdamonds/


在 `admin` 目录下，有一个 `kdamonds` 目录，其中包含用于控制 kdamonds 的文件（更多细节请参考 design <damon_design_execution_model_and_data_structures>）。起初，该目录只有一个文件 `nr_kdamonds`。向该文件写入一个数字（`N`）会创建名为 `0` 到 `N-1` 的子目录。每个目录代表一个 kdamond。


### kdamonds/<N>/


在每个 kdamond 目录中，存在三个文件（`state`、`pid` 和 `refresh_ms`）以及一个目录（`contexts`）。

读取 `state` 会返回 `on`（如果 kdamond 正在运行），或 `off`（如果未运行）。

用户可以向下面的 `state` 文件写入以下命令来控制 kdamond。

- `on`：开始运行。
- `off`：停止运行。
- `commit`：再次读取 sysfs 文件中除 `state` 文件之外的用户输入。如果未指定目标区域，监控目标区域 <sysfs_regions> 的输入也会被忽略。
- `update_tuned_intervals`：用自动调谐所应用的 ``sampling interval` and `aggregation interval`` 更新该 kdamond 的 `sample_us` 和 `aggr_us` 文件内容。更多细节请参考 intervals_goal section <damon_usage_sysfs_monitoring_intervals_goal>。
- `commit_schemes_quota_goals`：读取基于 DAMON 的操作方案的 quota goals <sysfs_schemes_quota_goals>。
- `update_schemes_stats`：更新该 kdamond 每个基于 DAMON 的操作方案的统计文件内容。关于统计的细节，请参考 stats section <sysfs_schemes_stats>。
- `update_schemes_tried_regions`：更新该 kdamond 每个基于 DAMON 的操作方案的动作已尝试区域目录。关于基于 DAMON 的操作方案动作已尝试区域目录的细节，请参考 tried_regions section <sysfs_schemes_tried_regions>。
- `update_schemes_tried_bytes`：仅更新 `.../tried_regions/total_bytes` 文件。
- `clear_schemes_tried_regions`：清除该 kdamond 每个基于 DAMON 的操作方案的动作已尝试区域目录。
- `update_schemes_effective_quotas`：更新该 kdamond 每个基于 DAMON 的操作方案的 `effective_bytes` 文件内容。更多细节请参考 quotas directory <sysfs_quotas>。

如果状态为 `on`，读取 `pid` 会显示 kdamond 线程的 pid。

用户可以请求内核周期性地更新显示自动调谐参数和 DAMOS 统计的文件，而不必手动将 `update_tuned_intervals` 之类的关键字写入 `state` 文件。为此，用户应将期望的更新时间间隔（毫秒）写入 `refresh_ms` 文件。如果该间隔为零，则禁用周期性更新。读取该文件会显示当前设置的时间间隔。

`contexts` 目录包含用于控制该 kdamond 将执行的监控上下文的文件。


### kdamonds/<N>/contexts/


起初，该目录只有一个文件 `nr_contexts`。向该文件写入一个数字（`N`）会创建名为 `0` 到 `N-1` 的子目录。每个目录代表一个监控上下文（更多细节请参考 design <damon_design_execution_model_and_data_structures>）。目前每个 kdamond 只支持一个上下文，因此只能向该文件写入 `0` 或 `1`。


### contexts/<N>/


在每个上下文目录中，存在三个文件（`avail_operations`、`operations` 和 `addr_unit`）以及三个目录（`monitoring_attrs`、`targets` 和 `schemes`）。

DAMON 支持多种类型的 :ref:`monitoring operations <damon_design_configurable_operations_set>`，包括用于虚拟地址空间和物理地址空间的那些。你可以通过读取 `avail_operations` 文件获得当前运行内核上可用的监控操作集列表。根据内核配置，该文件会列出不同的可用操作集。关于所有可用操作集及其简要说明的列表，请参考 :ref:`design <damon_operations_set>`。

你可以通过向 `avail_operations` 文件写入其中列出的一个关键字，并从 `operations` 文件读取，来设置和获取 DAMON 将用于该上下文的监控操作类型。

`addr_unit` 文件用于设置和获取操作集的 :ref:`address unit <damon_design_addr_unit>` 参数。


### contexts/<N>/monitoring_attrs/


用于指定监控属性的文件（包括监控所需的质量和效率）位于 `monitoring_attrs` 目录。具体而言，该目录中有两个子目录：`intervals` 和 `nr_regions`。

在 `intervals` 目录下，存在三个文件，分别对应 DAMON 的采样间隔（`sample_us`）、聚合间隔（`aggr_us`）和更新间隔（`update_us`）。你可以通过读写这些文件以微秒为单位设置和获取这些值。

在 `nr_regions` 目录下，存在两个文件，分别对应 DAMON 监控区域的下界和上界（`min` 和 `max`），它们控制监控开销。你可以通过读写这些文件来设置和获取这些值。

关于间隔和监控区域范围的更多细节，请参考 Design 文档 ([/mm/damon/design](/mm/damon/design))。


### contexts/<N>/monitoring_attrs/intervals/intervals_goal/


在 `intervals` 目录下，还存在一个用于自动调谐 `sample_us` 和 `aggr_us` 的目录，即 `intervals_goal` 目录。该目录下有四个用于自动调谐控制的文件：`access_bp`、`aggrs`、`min_sample_us` 和 `max_sample_us`。关于调谐机制的内部原理，请参考 :ref:`design document of the feature <damon_design_monitoring_intervals_autotuning>`。读写 `intervals_goal` 目录下的四个文件会显示并更新 :ref:design doc <damon_design_monitoring_intervals_autotuning> 中描述的同名调谐参数。调谐从用户设置的 `sample_us` 和 `aggr_us` 开始。在将 `update_tuned_intervals` 写入 `state` 文件后，可以从 `sample_us` 和 `aggr_us` 文件读取两个间隔的调谐后当前值。


### contexts/<N>/targets/


起初，该目录只有一个文件 `nr_targets`。向该文件写入一个数字（`N`）会创建名为 `0` 到 `N-1` 的子目录。每个目录代表一个监控目标。


### targets/<N>/


在每个目标目录中，存在两个文件（`pid_target` 和 `obsolete_target`）以及一个目录（`regions`）。

如果你向 `contexts/<N>/operations` 写入了 `vaddr`，则每个目标都应是一个进程。你可以通过将进程的 pid 写入 `pid_target` 文件来将该进程指定给 DAMON。

用户可以通过向 `obsolete_target` 文件写入非零值并提交（向 `state` 文件写入 `commit`）来选择性地移除目标数组中间的某些目标。DAMON 会从其内部目标数组中移除匹配的目标。用户有责任重新构建目标目录，以便它们正确表示变更后的内部目标数组。



### targets/<N>/regions


对于 `fvaddr` 或 `paddr` 监控操作集，用户必须设置监控目标地址范围。对于 `vaddr` 操作集，这不是强制的，但用户可以选择将初始监控区域设置为特定地址范围。更多细节请参考 :ref:`design <damon_design_vaddr_target_regions_construction>`。

对于此类情况，用户可以通过向该目录下的文件写入适当的值，按自己的意愿显式设置初始监控目标区域。

起初，该目录只有一个文件 `nr_regions`。向该文件写入一个数字（`N`）会创建名为 `0` 到 `N-1` 的子目录。每个目录代表一个初始监控目标区域。

如果在提交新的 DAMON 参数（向 kdamond <sysfs_kdamond> 的 `state` 文件写入 `commit`）时 `nr_regions` 为零，提交逻辑会忽略目标区域。换句话说，该目标的当前监控结果会被保留。


### regions/<N>/


在每个区域目录中，你会发现两个文件（`start` 和 `end`）。你可以分别通过写和读这些文件来设置和获取初始监控目标区域的起始和结束地址。

每个区域不应与其他区域重叠。目录 `N` 的 `end` 应小于或等于目录 `N+1` 的 `start`。


### contexts/<N>/schemes/


用于基于 DAMON 的操作方案（:ref:`DAMOS <damon_design_damos>`）的目录。用户可以通过读写该目录下的文件来获取和设置这些方案。

起初，该目录只有一个文件 `nr_schemes`。向该文件写入一个数字（`N`）会创建名为 `0` 到 `N-1` 的子目录。每个目录代表一个基于 DAMON 的操作方案。


### schemes/<N>/


在每个方案目录中，存在八个目录（`access_pattern`、`quotas`、`watermarks`、`core_filters`、`ops_filters`、`filters`、`dests`、`stats` 和 `tried_regions`）以及三个文件（`action`、`target_nid` 和 `apply_interval`）。

`action` 文件用于设置和获取方案的 :ref:`action <damon_design_damos_action>`。可以写入和读取该文件的关键字及其含义与设计文档 design doc <damon_design_damos_action> 上的列表相同。

`target_nid` 文件用于设置迁移目标节点，仅当 `action` 为 `migrate_hot` 或 `migrate_cold` 时才有意义。

`apply_interval_us` 文件用于以微秒为单位设置和获取方案的 apply_interval <damon_design_damos>。


### schemes/<N>/access_pattern/


用于给定基于 DAMON 的操作方案的目标访问 :ref:`pattern <damon_design_damos_access_pattern>` 的目录。

在 `access_pattern` 目录下，存在三个目录（`sz`、`nr_accesses` 和 `age`），每个目录都有两个文件（`min` 和 `max`）。你可以通过分别写和读 `sz`、`nr_accesses` 和 `age` 目录下的 `min` 和 `max` 文件，来设置和获取给定方案的访问模式。注意 `min` 和 `max` 构成一个闭区间。


### schemes/<N>/quotas/


用于给定基于 DAMON 的操作方案的 quotas <damon_design_damos_quotas> 的目录。

在 `quotas` 目录下，存在五个文件（`ms`、`bytes`、`reset_interval_ms`、`effective_bytes` 和 `goal_tuner`）以及两个目录（`weights` 和 `goals`）。

通过将值分别写入这三个文件，你可以设置 `time quota`（毫秒）、`size quota`（字节）和 `reset interval`（毫秒）。然后，DAMON 会尝试最多使用 `time quota` 毫秒将 `action` 应用于 `access_pattern` 的内存区域，并且在 `reset_interval_ms` 内仅将动作应用于最多 `bytes` 字节的内存区域。将 `ms` 和 `bytes` 都设为零会禁用配额限制，除非至少设置了一个 goal <sysfs_schemes_quota_goals>。

你可以通过将算法名称写入 `goal_tuner` 文件，来设置要使用的基于目标的有效配额自动调谐算法。读取该文件会返回当前选定的调谐算法。关于该特性的背景设计以及可选算法的名称，请参考 automatic quota tuning goals <damon_design_damos_quotas_auto_tuning> 的设计文档。关于目标的设置，请参考 goals directory <sysfs_schemes_quota_goals>。

时间配额在内部会被转换为大小配额。在转换后的大小配额与用户指定的大小配额之间，采用较小的一个。基于用户指定的 goal <sysfs_schemes_quota_goals>，有效大小配额会进一步调整。读取 `effective_bytes` 会返回当前的有效大小配额。该文件不会实时更新，因此用户应通过向相关的 `kdamonds/<N>/state` 文件写入一个特殊关键字 `update_schemes_effective_quotas`，来请求 DAMON sysfs 接口更新该文件的统计内容。

在 `weights` 目录下，存在三个文件（`sz_permil`、`nr_accesses_permil` 和 `age_permil`）。你可以通过将值写入 `weights` 目录下的这三个文件，以千分之一为单位设置针对大小、访问频率和年龄的 :ref:`prioritization weights <damon_design_damos_quotas_prioritization>`。


### schemes/<N>/quotas/goals/


用于给定基于 DAMON 的操作方案的 :ref:`automatic quota tuning goals <damon_design_damos_quotas_auto_tuning>` 的目录。

起初，该目录只有一个文件 `nr_goals`。向该文件写入一个数字（`N`）会创建名为 `0` 到 `N-1` 的子目录。每个目录代表一个目标及其当前达成情况。在多个反馈中，使用最佳的一个。

每个目标目录包含五个文件，即 `target_metric`、`target_value`、`current_value`、`nid` 和 `path`。用户可以通过读写这些文件中的每一个，来设置和获取设计文档 design doc <damon_design_damos_quotas_auto_tuning> 中指定的配额自动调谐目标的五个参数。注意，用户还应进一步将 `commit_schemes_quota_goals` 写入 :ref:`kdamond directory <sysfs_kdamond>` 的 `state` 文件，以将反馈传递给 DAMON。


### schemes/<N>/watermarks/


用于给定基于 DAMON 的操作方案的 watermarks <damon_design_damos_watermarks> 的目录。

在 watermarks 目录下，存在五个文件（`metric`、`interval_us`、`high`、`mid` 和 `low`），用于设置度量指标、指标检查的时间间隔以及三个水位线。你可以通过分别写这些文件来设置和获取这五个值。

可以写入 `metric` 文件的关键字及其含义如下。

 - none: 忽略水位线
 - free_mem_rate: 系统的空闲内存率（每千）

`interval` 应以微秒为单位写入。


### schemes/<N>/{core\_,ops\_,}filters/


用于给定基于 DAMON 的操作方案的 filters <damon_design_damos_filters> 的目录。

`core_filters` 和 `ops_filters` 目录分别用于由 DAMON 核心层和操层集层处理的过滤器。`filters` 目录可用于安装与所处理层无关的过滤器。由 `core_filters` 和 `ops_filters` 请求的过滤器会先于 `filters` 的过滤器安装。这三个目录拥有相同的文件。

使用 `filters` 目录可能会让对给定过滤器及其目录下文件的求值顺序产生混淆。因此建议用户使用 `core_filters` 和 `ops_filters` 目录。`filters` 目录将来可能会被弃用。

起初，该目录只有一个文件 `nr_filters`。向该文件写入一个数字（`N`）会创建名为 `0` 到 `N-1` 的子目录。每个目录代表一个过滤器。过滤器按数字顺序求值。

每个过滤器目录包含九个文件，即 `type`、`matching`、`allow`、`memcg_path`、`addr_start`、`addr_end`、`min`、`max` 和 `target_idx`。你可以向 `type` 文件写入过滤器的类型。关于可用的类型名、其含义以及它们由哪一层处理，请参考 design doc <damon_design_damos_filters>。

对于 `memcg` 类型，你可以通过将内存 cgroup 从 cgroups 挂载点到 `memcg_path` 文件的路径来指定感兴趣的 memory cgroup。对于 `addr` 类型，你可以将范围（开区间）的起始和结束地址分别指定给 `addr_start` 和 `addr_end` 文件。对于 `hugepage_size` 类型，你可以将范围（闭区间）的最小和最大大小分别指定给 `min` 和 `max` 文件。对于 `target` 类型，你可以将 DAMON 上下文监控目标列表中目标的索引指定给 `target_idx` 文件。

你可以向 `matching` 文件写入 `Y` 或 `N`，以指定该过滤器是否针对与 `type` 匹配的内存。你可以向 `allow` 文件写入 `Y` 或 `N`，以指定是否允许对满足 `type` 和 `matching` 的内存应用动作。

例如，下面将一个 DAMOS 动作限制为仅应用于非匿名

```

    # cd ops_filters/0/
    # echo 2 > nr_filters
    # # disallow anonymous pages
    echo anon > 0/type
    echo Y > 0/matching
    echo N > 0/allow
    # # further filter out all cgroups except one at '/having_care_already'
    echo memcg > 1/type
    echo /having_care_already > 1/memcg_path
    echo Y > 1/matching
    echo N > 1/allow

```
关于更多细节，包括具有不同 `allow` 的多个过滤器如何工作、各个过滤器何时被支持以及统计上的差异，请参考 :ref:`DAMOS filters design documentation <damon_design_damos_filters>`。


### schemes/<N>/dests/


用于指定给定基于 DAMON 的操作方案动作目标位置的目录。如果给定方案的动作不支持多个目标，则忽略此目录。只有 `DAMOS_MIGRATE_{HOT,COLD}` 动作支持多个目标。

起初，该目录只有一个文件 `nr_dests`。向该文件写入一个数字（`N`）会创建名为 `0` 到 `N-1` 的子目录。每个目录代表一个动作目标。

每个目标目录包含两个文件，即 `id` 和 `weight`。用户可以向 `id` 文件写入和读取目标的标识符。对于 `DAMOS_MIGRATE_{HOT,COLD}` 动作，迁移目标节点的节点 id 应写入 `id` 文件。用户可以向 `weight` 文件写入和读取该目标在给定目标中的权重。权重可以是任意整数。当 DAMOS 将该动作应用于内存区域的每个实体时，它会根据目标的相对权重来选择动作的目标位置。


### schemes/<N>/stats/


DAMON 为每个方案统计计数。这些统计数据可用于方案的在线分析或调优。关于统计的更多细节，请参考 :ref:`design doc <damon_design_damos_stat>`。

可以通过读取 `stats` 目录下的文件（`nr_tried`、`sz_tried`、`nr_applied`、`sz_applied`、`sz_ops_filter_passed`、`qt_exceeds`、`nr_snapshots` 和 `max_nr_snapshots`）分别获取这些统计数据。

默认情况下，这些文件不会实时更新。用户应请求 DAMON sysfs 接口使用 `refresh_ms` 周期性更新它们，或者通过向相关的 `kdamonds/<N>/state` 文件写入特殊关键字 `update_schemes_stats` 进行一次更新。更多细节请参考 :ref:`kdamond directory <sysfs_kdamond>`。


### schemes/<N>/tried_regions/


该目录起初有一个文件 `total_bytes`。

当向相关的 `kdamonds/<N>/state` 文件写入一个特殊关键字 `update_schemes_tried_regions` 时，DAMON 会更新 `total_bytes` 文件，使读取它返回方案已尝试区域的总大小，并创建从该目录下从 `0` 开始以整数命名的目录。每个目录包含文件，暴露相应方案的 `action` 尝试应用的每个内存区域的详细信息，这些信息在相应方案的下一个 apply interval <damon_design_damos> 期间生成。这些信息包括区域的地址范围、`nr_accesses` 和 `age`。

向相关的 `kdamonds/<N>/state` 文件写入 `update_schemes_tried_bytes` 只会更新 `total_bytes` 文件，不会创建子目录。

当另一个特殊关键字 `clear_schemes_tried_regions` 被写入相关的 `kdamonds/<N>/state` 文件时，这些目录会被移除。

该目录的预期用途是调查方案的行为，以及类查询的高效数据访问监控结果检索。特别是对于后一种用例，用户可以将 `action` 设为 `stat`，并将 `access pattern` 设为其想要查询的感兴趣模式。


### tried_regions/<N>/


在每个区域目录中，你会发现五个文件（`start`、`end`、`nr_accesses`、`age` 和 `sz_filter_passed`）。读取这些文件会显示相应基于 DAMON 的操作方案 `action` 尝试应用的区域的属性。

#### 示例


以下命令应用一个方案，其含义是：“如果一个大小为 [4KiB, 8KiB] 的内存区域在 [10, 20] 的聚合间隔内，每聚合间隔的访问次数在 [0, 5] 范围内，则将该区域换出。对于换出，每秒最多使用 10ms，并且每秒换出不超过 1GiB。在此限制下，优先换出年龄更长的内存区域。此外，每 5 秒检查系统的空闲内存率，当空闲内存率低于 50% 时开始监控和换出，但如果空闲

```

    # cd <sysfs>/kernel/mm/damon/admin
    # # populate directories
    # echo 1 > kdamonds/nr_kdamonds; echo 1 > kdamonds/0/contexts/nr_contexts;
    # echo 1 > kdamonds/0/contexts/0/schemes/nr_schemes
    # cd kdamonds/0/contexts/0/schemes/0
    # # set the basic access pattern and the action
    # echo 4096 > access_pattern/sz/min
    # echo 8192 > access_pattern/sz/max
    # echo 0 > access_pattern/nr_accesses/min
    # echo 5 > access_pattern/nr_accesses/max
    # echo 10 > access_pattern/age/min
    # echo 20 > access_pattern/age/max
    # echo pageout > action
    # # set quotas
    # echo 10 > quotas/ms
    # echo $((1024*1024*1024)) > quotas/bytes
    # echo 1000 > quotas/reset_interval_ms
    # # set watermark
    # echo free_mem_rate > watermarks/metric
    # echo 5000000 > watermarks/interval_us
    # echo 600 > watermarks/high
    # echo 500 > watermarks/mid
    # echo 300 > watermarks/low

```
请注意，强烈建议使用像 `damo <https://github.com/damonitor/damo>`_ 这样的用户空间工具，而不是像上面这样手动读写文件。以上仅作为示例。


## 用于监控结果的 Tracepoints


用户可以通过 :ref:`tried_regions <sysfs_schemes_tried_regions>` 获取监控结果。该接口对于获取快照很有用，但对于完整记录所有监控结果可能效率不高。为此，提供了两个跟踪点，即 `damon:damon_aggregated` 和 `damon:damos_before_apply`。`damon:damon_aggregated` 提供完整的监控结果，而 `damon:damos_before_apply` 提供每个基于 DAMON 的操作方案（DAMOS <damon_design_damos>）将要应用的区域的监控结果。因此，`damon:damos_before_apply` 对于记录 DAMOS 的内部行为，或基于 DAMOS 目标访问模式 <damon_design_damos_access_pattern> 的类查询高效监控结果记录更有用。

在监控开启期间，你可以记录跟踪点事件，方法如下：

```

    # echo on > kdamonds/0/state
    # perf record -e damon:damon_aggregated &
    # sleep 5
    # kill 9 $(pidof perf)
    # echo off > kdamonds/0/state
    # perf script
    kdamond.0 46568 [027] 79357.842179: damon:damon_aggregated: target_id=0 nr_regions=11 122509119488-135708762112: 0 864
    [...]

```
perf script 输出的每一行代表一个监控区域。前五个字段与其他跟踪点输出一样。第六个字段（`target_id=X`）显示该区域的监控目标的 id。第七个字段（`nr_regions=X`）显示该目标的监控区域总数。第八个字段（`X-Y:`) 显示该区域以字节为单位的起始（`X`）和结束（`Y`）地址。第九个字段（`X`）显示该区域的 `nr_accesses`（关于该计数器的更多细节，请参考 design <damon_design_region_based_sampling>）。最后第十个字段（`X`）显示该区域的 `age`（关于该计数器的更多细节，请参考 design <damon_design_age_tracking>）。

如果事件是 `damon:damos_beofre_apply`，则 `perf script` 输出会

```

    kdamond.0 47293 [000] 80801.060214: damon:damos_before_apply: ctx_idx=0 scheme_idx=0 target_idx=0 nr_regions=11 121932607488-135128711168: 0 136
    [...]

```
输出的每一行代表在跟踪时刻每个基于 DAMON 的操作方案即将应用的每个监控区域。前五个字段如常。除了 `damon_aggregated` 跟踪点的输出外，它还显示方案中 DAMON 上下文在其 kdamond 上下文列表中的索引（`ctx_idx=X`），以及方案在其上下文方案列表中的索引（`scheme_idx=X`）。
