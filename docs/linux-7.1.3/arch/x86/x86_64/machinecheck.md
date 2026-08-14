## x86-64 机器检查（machine check）代码的可配置 sysfs 参数


机器检查（Machine check）报告由 CPU 检测到的内部硬件错误状态。未纠正（Uncorrected）的错误通常会导致一次机器检查（通常伴随 panic），已纠正的错误则会生成一条机器检查日志条目。

机器检查按 bank（通常关联某个硬件子系统）以及 bank 内的子事件（subevent）组织。bank 与子事件的确切含义是 CPU 相关的。

mcelog 知道如何对它们进行解码。

当你在系统日志中看到 "Machine check errors logged" 消息时，应当运行 mcelog 来从 /dev/mcelog 收集并解码机器检查条目。通常，mcelog 应通过 cron 任务定期运行。

每个 CPU 在 /sys/devices/system/machinecheck/machinecheckN 下都有一个目录（N = CPU 编号）。

该目录包含一些可配置项。更多细节请参阅 Documentation/ABI/testing/sysfs-mce。

待定（TBD）：记录 AMD 阈值中断（threshold interrupt）配置相关条目。

有关 x86 机器检查架构的更多细节，请参阅 Intel 和 AMD 开发者网站上的架构手册。

有关该架构的更多细节，请参阅 http://one.firstfloor.org/~andi/mce.pdf
