## 内核驱动 k8temp


支持的芯片：

  - AMD Athlon64/FX 鎴?Opteron CPU

    Prefix: 'k8temp'

    Addresses scanned: PCI 空间

    Datasheet: https://www.amd.com/system/files/TechDocs/32559.pdf

Author: Rudolf Marek

Contact: Rudolf Marek <r.marek@assembler.cz>

### 描述


该驱动允许读取内置于 AMD K8 系列 CPU（Athlon64/FX、Opteron）中的温度传感器的读数。官方文档称其从 K8 核心F 修订版开始工作，但实际上它似乎在除前两个修订版（SH-B0 SH-B3）之外的所K8 修订版上都有实现
请注意，你至少需lm-sensors 2.10.1 才能获得正常的用户空间支持
单个 CPU 内最多可有四个温度传感器。该驱动会自动检测传感器，并只显示已实现传感器的温度
/sys 文件的映射如下：

============= ===================================
temp1_input   Core 0 “位置0 的温temp2_input   Core 0 “位置1 的温temp3_input   Core 1 “位置0 的温temp4_input   Core 1 “位置1 的温============= ===================================

温度以摄氏度测量，测量分辨率1 C。预计未来的 CPU 会有更好的分辨率。温度每秒更新一次。有效温度范围为 -49 206 C
称为 TCaseMax 的温度是针对修订E 及之前的处器指定的。该温度定义为散热片（heat-spreader）与 CPU 外壳之间的温度，因此该驱动提供的 CPU 内部温度可能更高。没有简单的方法测量TCaseMax 温度相关联的温度
对于较新修订版的 CPU（rev F，socket AM2），有一个数学计算得到的温度称为 TControl，它必须低于 TControlMax
关系如下
	temp1_input - TjOffset*2 < TControlMax,

TjOffset 尚未由驱动导出，TControlMax 通常70 C。经验法-> CPU 温度不应过多超过 60 C