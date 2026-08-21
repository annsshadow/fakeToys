## 用户空间工具


### 简


大多数主板都带有用于监控系统健康状态（如温度、电压、风扇转速）的传感器芯片。它们通常通过 I2C 总线连接，但也有一些直接通过 ISA 总线连接

内核驱动将传感器芯片的数据暴露在 /sys 虚拟文件系统中。随后使用用户空间工具以更友好的方式显示测量值或配置芯片

### Lm-sensors


一组核心工具，可让你获取健康信息、设置监控阈值等。你可以在其主页 http://www.lm-sensors.org/ 获取，或作为软件包从你的 Linux 发行版中获取

若从网站获取
从项目网站获lm-sensors。请注意，你只需要用户空间部分，因此使用 "make user" 编译，并使用 "make user_install" 安装

使其正常工作的一般提示：

0) 获取 lm-sensors 用户空间工具
1) I2C 与硬件监控（Hardware Monitoring）节中的所有驱动编译为内核模块
2) 运行 sensors-detect 脚本，它会告诉你需加载哪些模块
3) 加载它们并运"sensors" 命令，你应该能看到一些结果
4) 修正 sensors.conf、标签、阈值、风扇分频系
5) 若有更多问题，查FAQ 或文

### 其他工具


如果你想要一些系统健康状态的图形化指示器，可以寻找诸gkrellm、ksensors、xsensors、wmtemp、wmsensors、wmgtemp、ksysguardd、hardware-monitor 等应用程序

如果你是服务器管理员，可以尝snmpd mrtgutils
