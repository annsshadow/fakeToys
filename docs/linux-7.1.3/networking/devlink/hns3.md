## hns3 devlink 支持


本文档描述了 `hns3` 设备驱动实现devlink 特性

`hns3` 驱动支持通过 `DEVLINK_CMD_RELOAD` 进行重新加载

## Info versions


`hns3` 驱动报告以下版本

   :widths: 10 10 80

   - - Name
     - Type
     - Description
   - - `fw`
     - running
     - 用于表示固件版本
   - - `fw.scc`
     - running
     - 用于表示软拥塞控制（SSC）固件版本
       SCC 是一个固件组件，提供多种 RDMA 拥塞控制算法，包DCQCN
