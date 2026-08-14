## rtla

### 实时 Linux 分析工具


:Manual section: 1

## 概要


**rtla** **COMMAND** [**OPTIONS**]

## 描述


**rtla** 是一个元工具（meta-tool），包含一组旨在分析 Linux 实时特性的命令。与将 Linux 当作黑盒测试不同，**rtla** 利用内核跟踪能力，提供关于异常结果特性及其根本原因的精确信息。

## 命令


**hwnoise**

        检测并量化与硬件相关的噪声。

**osnoise**

        提供有关操作系统噪声（osnoise）的信息。

**timerlat**

        测量 IRQ 与线程定时器延迟。

## 选项


**-h**, **--help**

        显示帮助文本。

其他选项请参阅对应命令的手册页。

## 另请参阅


**rtla-hwnoise**\(1), **rtla-osnoise**\(1), **rtla-timerlat**\(1)

## 作者


Daniel Bristot de Oliveira <bristot@kernel.org>
