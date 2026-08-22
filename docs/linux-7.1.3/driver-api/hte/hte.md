
## Linux 硬件时间戳引擎（HTE, Hardware Timestamping Engine

:Author: Dipen Patel

### 简

某些设备内置有硬件时间戳引擎，可以实时监视一组系统信号、线路、总线等的状变化；一旦检测到变化，它们可以自动存储发生变化时刻的时间戳。与使用软件方式
（即 ktime 及其同类）相比，此类功能有助于获得更准确的时间戳
本文档描述了供硬件时间戳引擎的提供方（provider）与消费方（consumer）驱动使API，这些驱动希望使用硬件时间戳引擎（HTE）框架。消费方与提供方都必须包`#include <linux/hte.h>`
### 提供给提供方HTE 框架 API


   :functions: devm_hte_register_chip hte_push_ts_ns

### 提供给消费方HTE 框架 API


   :functions: hte_init_line_attr hte_ts_get hte_ts_put devm_hte_request_ts_ns hte_request_ts_ns hte_enable_ts hte_disable_ts of_hte_req_count hte_get_clk_src_info

### HTE 框架公共结构

### 关于 HTE 时间戳数据的更多说明


`struct hte_ts_data` 用于在消费方与提供方之间传递时间戳详细信息。它u64
表达纳秒级的时间戳数据。下面是 GPIO 线路典型时间戳数据生命周期的一个示例：

```

 - 监视 GPIO 线路变化 - 检GPIO 线路上的状态变化 - 将时间戳转换为纳秒 - 如果提供方具备该硬件能力，则GPIO 原始电平存入 raw_level 变量 - 将该 hte_ts_data 对象推送给 HTE 子系统 - HTE 子系统递增 seq 计数器，并调用消费方提供的回调   根据回调的返回值，HTE 核心在线程上下文中调用次级回调
```

### HTE 子系debugfs 属

HTE 子系统在 `/sys/kernel/debug/hte/` 创建 debugfs 属性。它还在
`/sys/kernel/debug/hte/<provider>/<label or line id>/` 创建与线信号相关debugfs 属性。注意这些属性都是只读的
`ts_requested`
		从给定提供方请求的实体总数，其中实体由提供方定义，可能代表
		线路、GPIO、芯片信号、总线等…                该属性位`/sys/kernel/debug/hte/<provider>/`
`total_ts`
		提供方支持的实体总数                该属性位`/sys/kernel/debug/hte/<provider>/`
`dropped_timestamps`
		给定线路上被丢弃的时间戳                该属性位`/sys/kernel/debug/hte/<provider>/<label or line id>/`