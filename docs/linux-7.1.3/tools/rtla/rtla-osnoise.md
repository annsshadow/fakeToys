## rtla-osnoise

### 测量操作系统噪声


:Manual section: 1

## 概要

**rtla osnoise** [**MODE**] ...

## 描述


**osnoise** tracer 以两种方式输出信息。它定期打印操作系统噪声的摘要，包括
干扰源发生的计数器。它还通过 **osnoise:** 跟踪点提供每个噪声的信息。**rtla osnoise top**
模式显示来自 **osnoise** tracer 的周期性摘要信息。**rtla osnoise hist** 模式使用
**osnoise:** 跟踪点显示噪声信息。更多细节请参阅相应的手册页。

## 模式

**top**

        打印 osnoise tracer 的摘要。

**hist**

        打印 osnoise 样本的直方图。

如果未给定模式，则调用 top 模式并传递参数。

## 选项


**-h**, **--help**

        显示帮助文本。

其他选项请参阅相应模式的手册页。

## 另见

**rtla-osnoise-top**\(1), **rtla-osnoise-hist**\(1)

`Osnoise tracer <https://docs.kernel.org/trace/osnoise-tracer.html>`__

## 作者

由 Daniel Bristot de Oliveira <bristot@kernel.org> 编写
