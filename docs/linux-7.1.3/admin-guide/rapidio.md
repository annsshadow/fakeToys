## RapidIO 子系统指

:Author: Matt Porter

## 简

RapidIO 是一种面向嵌入式市场的高速交换结构互连。RapidIO 提供对内存映I/O 以及基于消息的事务在交换结构网络上的支持。RapidIO 拥有类似PCI 总线
标准的标准化发现机制，可简单检测网络中的设备
本文档供打算在新架构上支RapidIO、编写新驱动，或理解子系统内部机制的
开发者使用
## 已知缺陷与限

### 缺陷


None. ;)

### 限制


1. 不支持对 RapidIO 内存区域的访管理

2. 不支持多主机枚举

## RapidIO 驱动接口


驱动获得一组调用，以便与子系统交互，收集设备信息、请映射内存区域资源并管理邮门铃
### 函数


   :internal:

   :export:

   :export:

## 内部机制


本章包含 RapidIO 子系统的自动生成文档
### 结构

   :internal:

### 枚举与发

   :internal:

### 驱动功能


   :internal:

   :internal:

### 设备模型支持


   :internal:

### PPC32 支持


   :internal:

## 致谢


以下人员直接或间接为 RapidIO 子系统做出了贡献
1. Matt Porter\ mporter@kernel.crashing.org

2. Randy Vinson\ rvinson@mvista.com

3. Dan Malek\ dan@embeddedalley.com

以下人员为本文档做出了贡献：

1. Matt Porter\ mporter@kernel.crashing.org
