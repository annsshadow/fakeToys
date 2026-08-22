## FPGA Region


### 概述


本文档旨在简要概FPGA region API 的用法。关region 更具概念性的介绍可以Device Tree
绑定文档 [#f1]_ 中找到
就本 API 文档而言，我们不妨说一region FPGA Manager 与一座（或多座）桥接关联FPGA 一个可重编程区域或整个 FPGA。该 API 提供了注region 以及region 进行编程的方法
目前fpga-region.c 之上、内核中唯一的层[#f1]_ 中描述的 Device Tree 支持
（of-fpga-region.c）。DT 支持层使region FPGA 进行编程，然后使DT 处理枚举。通用region 代码旨在被其他在编程后有其它枚举方式的方案所使用
一fpga-region 可以配置为了解以下内容：

 - 使用哪个 FPGA manager 进行编程

 - 在编程前禁用、编程后启用的桥
编程 FPGA 镜像所需的额外信息通过 struct fpga_image_info 传入，包括：

 - 指向镜像的指针，可以是分聚集缓冲区、连续缓冲区，或固件文件
 - 指示具体特性的标志，例如镜像是否用于部分重配置

### 如何添加一个新FPGA region


使用示例可见 [#f2]_ probe 函数
### 添加新的 FPGA region API


- struct fpga_region - FPGA region 结构- struct fpga_region_info - __fpga_region_register_full() 的参数结构体
- __fpga_region_register_full() - 使用 fpga_region_info 结构体创建并注册一FPGA region  以提供最大化的选项灵活- __fpga_region_register() - 使用标准参数创建并注册一FPGA region
- fpga_region_unregister() - 注销一FPGA region

辅助`fpga_region_register()` `fpga_region_register_full()` 会自动将注册FPGA region
的模块设为拥有者
FPGA region probe 函数需要获取对它将用于编程FPGA Manager 的引用。这通常会在 region probe 函数期间发生
- fpga_mgr_get() - 获取FPGA manager 的引用，增加引用计数
- of_fpga_mgr_get() - 获取FPGA manager 的引用，增加引用计数，给定一个设备节- fpga_mgr_put() - 释放一FPGA manager

FPGA region 需要指定在编程 FPGA 时要控制哪些桥接。region 驱动可以probe 期间构建一个桥接列c`fpga_region->bridge_list`），也可以有一个函数用于在编程前立即创建要编程的桥接列c`fpga_region->get_bridges`）。FPGA bridge 框架提供以下 API 来处理构建或拆除该列表
- fpga_bridge_get_to_list() - 获取FPGA bridge 的引用，将其加入列表
- of_fpga_bridge_get_to_list() - 获取FPGA bridge 的引用，将其加入列表，给定一个设备节- fpga_bridges_put() - 给定一个桥接列表，释放它们

   :functions: fpga_region

   :functions: fpga_region_info

   :functions: __fpga_region_register_full

   :functions: __fpga_region_register

   :functions: fpga_region_unregister

   :functions: fpga_mgr_get

   :functions: of_fpga_mgr_get

   :functions: fpga_mgr_put

   :functions: fpga_bridge_get_to_list

   :functions: of_fpga_bridge_get_to_list

   :functions: fpga_bridges_put
