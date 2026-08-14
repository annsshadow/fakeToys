
## rv-mon
### 列出可用监视器


:Manual section: 1

## 概要


**rv mon** [**-h**] **monitor_name** [**-h**] [**MONITOR OPTIONS**]

## 描述


**rv mon** 命令运行名为 **monitor_name** 的监视器。每个监视器都有自己的一组
选项。**rv list** 命令显示所有可用的监视器。

## 选项


**-h**, **--help**

        打印帮助菜单。

## 可用监视器


**rv** 工具提供了一组监视器的接口。使用 **rv list** 命令列出所有可用的
监视器。

每个监视器都有自己的一组选项。有关每个特定监视器的详细信息，请参阅 man
**rv-mon**-**monitor_name**。此外，运行 **rv mon** **monitor_name** **-h**
会显示带有可用选项的帮助菜单。

## 另请参阅


**rv**\(1), **rv-mon**\(1)

Linux 内核 **RV** 文档：
<https://www.kernel.org/doc/html/latest/trace/rv/index.html>

## 作者


由 Daniel Bristot de Oliveira <bristot@kernel.org> 编写
