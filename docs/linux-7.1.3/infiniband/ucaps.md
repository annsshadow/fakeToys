## Infiniband 用户空间能力

   User CAPability（UCAP）提供对 Infiniband（IB）设备中特定固件特性的细粒   控制。相比现有的 Linux capabilities，这种方式提供了更细化的能力，而现有的
   Linux capabilities 对于某些 FW 特性而言可能过于宽泛
   每个用户能力都表现为一个字符设备，root 可读写。root 进程可以通过允许
   访问这些字符设备（例如使chown）来授予用户特殊权限
## 用法


   UCAP 允许使用 UCAP 字符设备的文件描述符来控IB 设备的特定特性。用户启   IB 设备特定特性的方式如下
      - root 进程授予用户访问代表这些能力UCAP 文件的权限（例如使用 chown）      - 用户打开 UCAP 文件，获取文件描述符      - 在打开 IB 设备时，UCAP 文件描述符数组作为一个属性包含进来      - ib_uverbs 驱动识别这些 UCAP 文件描述符，并为IB 设备启用相应的能力
## 创建 UCAP


   要创建新UCAP，驱动必须首先在 rdma/ib_ucaps.h rdma_user_cap 枚举   定义一个类型。UCAP 字符设备的名称应添加drivers/infiniband/core/ucaps.c
   ucap_names 数组中。然后，驱动可以通过调用带有 UCAP 类型ib_create_ucap
   API 来创UCAP 字符设备
   为每UCAP 存储一个引用计数，以跟UCAP 设备的创建与移除。如果以相同类型
   （例如针对两IB 设备）发出多次创建调用，UCAP 字符设备会在首次调用   创建，后续调用递增引用计数
   UCAP 字符设备创建/dev/infiniband 下，其权限被设置为仅允许 root 读写
## 移除 UCAP


   每次移除都会递减 UCAP 的引用计数。只有当引用计数减到 0 时，UCAP 字符设备
   才会从文件系统中移除
## /dev /sys/class 文件


```

      /sys/class/infiniband_ucaps

   is created when the first UCAP character device is created.

   The UCAP character device is created under /dev/infiniband.

   For example, if mlx5_ib adds the rdma_user_cap
   RDMA_UCAP_MLX5_CTRL_LOCAL with name "mlx5_perm_ctrl_local", this will
   create the device node::

      /dev/infiniband/mlx5_perm_ctrl_local


```
