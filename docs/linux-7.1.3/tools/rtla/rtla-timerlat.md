## rtla-timerlat
### 测量操作系统定时器延迟


:Manual section: 1

## 概要

**rtla timerlat** [**MODE**] ...

## 描述


**rtla timerlat top** 模式显示来自 **timerlat** 跟踪器的周期性输出摘要。
**rtla timerlat hist** 模式显示每个跟踪器事件发生的直方图。更多详细信息，
请参阅相应的 man 手册页。

## 模式

**top**

        打印来自 **timerlat** 跟踪器的摘要。

**hist**

        打印 timerlat 样本的直方图。

如果未给定 **MODE**，则调用 top 模式并传递参数。

## 选项

**-h**, **--help**

        显示帮助文本。

有关其他选项，请参阅相应模式的 man 手册页。

## 另请参阅
**rtla-timerlat-top**\(1), **rtla-timerlat-hist**\(1)

`Timerlat 跟踪器 <https://docs.kernel.org/trace/timerlat-tracer.html>`__

## 作者
由 Daniel Bristot de Oliveira <bristot@kernel.org> 编写
