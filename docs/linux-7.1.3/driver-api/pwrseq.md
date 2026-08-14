## 电源 Sequencing API

:Author:Bartosz Golaszewski

## 简介

该框架旨在对 Linux 内核中多个逻辑设备所共享的复杂上电时序（power-up sequence）进行抽象。

其设计意图是允许使用者（consumer）获取由电源时序提供者（provider）所暴露的电源时序处理过程；提供者会代为请求对底层资源的实际控制，并能够在幕后缓解多个使用者之间的潜在冲突。

### 术语表（Glossary）

电源时序（power sequencing）API 使用了该子系统特有的若干术语：

Unit

单元（Unit）是电源时序中的一个离散片段。例如，一个单元可能负责启用并配置一组 regulator，或启用某个特定的 GPIO。单元之间通过定义依赖关系，构成启用其他单元的基础。

Target

目标（Target）由若干单元组成（即“最终”单元及其依赖项），使用者在请求某个电源时序器（sequencer）时通过名称来指定目标。在依赖系统中，多个目标可以共享电源时序的相同部分，而忽略与之无关的部分。

描述符

描述符（Descriptor）由每个使用者传递给 pwrseq 核心，作为进入提供者层的入口点。它确保不同使用者之间的一致性，并保持引用计数（reference counting）的统一。

## Consumer 接口

使用者接口（Consumer API）的设计目标是尽可能简单。对获取电源时序器描述符感兴趣的驱动调用 pwrseq_get()，并指定想要达到的目标（target）名称，随后通过调用 pwrseq_power_up() 来执行该时序。描述符通过调用 pwrseq_put() 释放；使用者在请求关闭目标电源时调用 pwrseq_power_off()。需要说明的是，pwrseq_power_off() 保证不会影响仍被多个使用者使用的底层资源——这些资源会保持激活状态。

## Provider 接口

提供者接口（Provider API）诚然不像使用者 API 那样直白，但它换来了更大的灵活性。

提供者会将上电时序在逻辑上拆分为若干离散片段（单元）并定义它们之间的依赖关系；随后向使用者暴露具名的目标，即使用者希望达到的时序终点。

最后，提供者填写配置结构体，并通过调用 pwrseq_device_register() 将自己注册到 pwrseq 子系统。

### 动态使用者匹配（Dynamic consumer matching）

pwrseq 与 Linux 内核中其他提供者机制的主要区别在于：它能够在使用者与提供者之间进行动态匹配。每个电源时序提供者驱动都会实现一个 `match()` 回调，并在注册到 pwrseq 核心时将其传入。

当客户请求某个时序器处理时，核心会调用每个已注册提供者的回调，以便灵活判断所提议的客户设备是否确实属于某个使用者。举例来说：提供者可以绑定一个代表电源管理单元（PMU）芯片组的设备树（device-tree）节点；而当使用者驱动控制某个模块时，提供者驱动会解析设备树中相关的 regulator 供电属性，从而找到对应的 PMU 使用者。

## API 参考

:internal:

:export:
