
## Devlink Reload（devlink 重新加载）


`devlink-reload` 提供了一种机制，用于重新初始化驱动实体，并应用
`devlink-params` 与 `devlink-resources` 的新值。它还提供了一种激活固件的
机制。

## 重新加载动作（Reload Actions）


用户可以选择一个重新加载动作。默认选择的是 `driver_reinit` 动作。

   :widths: 5 90

   - - Name
     - Description
   - - `driver-reinit`
     - devlink 驱动实体的重新初始化，包括在驱动加载过程中所使用的 devlink
       实体上应用新值，这些实体包括：

       - 配置模式为 `driverinit` 的 `devlink-params`
       - `devlink-resources`

       其他 devlink 实体在重新初始化过程中可以保持不变：

       - `devlink-health-reporter`
       - `devlink-region`

       其余的 devlink 实体则必须被移除并重新添加。
   - - `fw_activate`
     - 激活固件。如果存在待激活的固件镜像，则激活新固件。如果没有指定任何
       限制，该动作可能会涉及固件复位。如果没有待激活的新镜像，则该动作会
       重新加载当前的固件镜像。

请注意，即使用户请求了某个特定动作，驱动的实现也可能需要同时执行另一个
动作。例如，某些驱动不支持在不激活固件的情况下进行驱动重新初始化。因此，
devlink reload 命令会返回实际执行的动作列表。

## 重新加载限制（Reload Limits）


默认情况下，重新加载动作不受限制，驱动实现可以根据需要执行复位或停机以
完成相应动作。

不过，某些驱动支持动作限制，将动作的实现限定在特定约束之内。

   :widths: 5 90

   - - Name
     - Description
   - - `no_reset`
     - 不允许复位，不允许停机，不允许链路抖动，且不会丢失任何配置。

## 切换命名空间（Change Namespace）


netns 选项允许用户在 devlink reload 操作过程中将 devlink 实例移动到其他
命名空间。默认情况下，所有 devlink 实例都在 init_net 中创建并保留在那里。

### 使用示例（example usage）


    $ devlink dev reload help
    $ devlink dev reload DEV [ netns { PID | NAME | ID } ] [ action { driver_reinit | fw_activate } ] [ limit no_reset ]

    # 运行重新加载命令以重新初始化 devlink 驱动实体：
    $ devlink dev reload pci/0000:82:00.0 action driver_reinit
    reload_actions_performed:
      driver_reinit

    # 运行重新加载命令以激活固件：
    # 注意：mlx5 驱动在激活固件的同时会重新加载驱动
    $ devlink dev reload pci/0000:82:00.0 action fw_activate
    reload_actions_performed:
      driver_reinit fw_activate
