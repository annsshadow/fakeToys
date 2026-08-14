## HPE iLO NMI 看门狗驱动


## 用于基于 iLO 的 ProLiant 服务器


最后审阅：2018/08/20

 HPE iLO NMI 看门狗驱动是一个内核模块，提供基本的看门狗功能，以及针对 iLO“向系统产生 NMI”
虚拟按钮的处理程序。

 本文档中所有对 iLO 的引用都意味着它同样适用于 iLO2 及所有后续代际。

 看门狗功能的启用方式与其他任何常见的看门狗驱动相同。也就是说，需要启动一个应用程序来
 周期性地“踢”看门狗定时器。在 tools/testing/selftests/watchdog/ 中有一个名为 watchdog-test.c
 的基础应用程序。只需编译该 C 文件并运行它。如果系统进入不良状态并挂起，HPE ProLiant iLO
 定时器寄存器就不会被及时更新，并会发生一次硬件系统复位（也称为自动服务器恢复（ASR））事件。

 hpwdt 驱动还有以下模块参数：

 ============  ================================================================
 soft_margin   允许用户设置看门狗定时器值。默认值为 30 秒。
 timeout       soft_margin 的别名。
 pretimeout    允许用户设置看门狗预超时值。这是超时之前、向系统投递 NMI
               的秒数。将该值设为零会禁用预超时 NMI。默认值为 9 秒。
 nowayout      基础看门狗参数，不允许定时器被重启或逃避即将发生的 ASR。
               默认值在编译内核时设定。如果被设为“Y”，那么一旦看门狗被启动，
               就无法禁用它。
 kdumptimeout  收到 NMI 后、调用 panic 之前应用的最小超时秒数。（-1）禁用看门狗。
               当值 > 0 时，定时器会被重新编程为 value 或当前超时值中的较大者。
 ============  ================================================================

 注意：
       关于看门狗驱动的一般性更多信息，包括对 /dev/watchdog 的 ioctl 接口，可以在
       Documentation/watchdog/watchdog-api.rst 与 Documentation/driver-api/ipmi.rst 中找到。

 由于 iLO 硬件的限制，如果启用了 NMI 预超时，它只能被设为 9 秒。尝试将 pretimeout 设为其他
 非零值会被取整，可能取整为零。用户在尝试设置 pretimeout 或 timeout 之后应核实 pretimeout 的值。

 收到来自 iLO 的 NMI 后，hpwdt 驱动会触发一次 panic。这是为了能够收集崩溃转储。用户有义务
 正确地为 kdump 配置系统。

 发生 panic 时 Linux 内核的默认行为是打印内核 tombstone 并永远循环。这通常不是看门狗用户所期望的。

 希望了解更多者请参见：
 - Documentation/admin-guide/kdump/kdump.rst
 - Documentation/admin-guide/kernel-parameters.txt（panic=）
 - 你所使用的 Linux 发行版专有文档。

 如果 hpwdt 没有收到与到期定时器关联的 NMI，iLO 会在超时时继续复位系统（如果定时器尚未被更新）。

--

 HPE iLO NMI 看门狗驱动与文档最初由 Tom Mingarelli 开发。
