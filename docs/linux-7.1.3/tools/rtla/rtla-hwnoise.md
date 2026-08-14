
## rtla-hwnoise

### 检测并量化与硬件相关的噪声


:Manual section: 1

## 概要（SYNOPSIS）


**rtla hwnoise** [**OPTIONS**]

## 描述（DESCRIPTION）


**rtla hwnoise** 收集 **osnoise** tracer 在**禁用中断**情况下运行的周期性摘要。通过禁用中断，
进而禁用线程的调度，只允许不可屏蔽中断（NMI）和与硬件相关的噪声通过。

该工具还允许配置 **osnoise** tracer 并收集 tracer 的输出。

## 选项（OPTIONS）


## 示例（EXAMPLE）


在下面的示例中，**rtla hwnoise** 工具被设置为在一个具有 8 核/16 线程且启用了超线程的系统上
运行于 CPU **1-7**。

该工具被设置为检测任何高于**一微秒**的噪声，运行**十分钟**，并在报告末尾显示一个摘要
```

  # rtla hwnoise -c 1-7 -T 1 -d 10m -q
                                          Hardware-related Noise
  duration:   0 00:10:00 | time is in us
  CPU Period       Runtime        Noise  % CPU Aval   Max Noise   Max Single          HW          NMI
    1 #599       599000000          138    99.99997           3            3           4           74
    2 #599       599000000           85    99.99998           3            3           4           75
    3 #599       599000000           86    99.99998           4            3           6           75
    4 #599       599000000           81    99.99998           4            4           2           75
    5 #599       599000000           85    99.99998           2            2           2           75
    6 #599       599000000           76    99.99998           2            2           0           75
    7 #599       599000000           77    99.99998           3            3           0           75


```
第一列显示 **CPU**，第二列显示工具在本次会话中运行的 **Periods**（周期）数量。**Runtime** 是
工具在 CPU 上实际运行的时间。**Noise** 列是工具观察到的所有噪声之和，**% CPU Aval** 是
**Runtime** 与 **Noise** 之间的关系。

**Max Noise** 列是工具在单个周期内检测到的最大硬件噪声，**Max Single** 是观察到的最大单次噪声。

**HW** 和 **NMI** 列显示工具观察到的 **hardware**（硬件）和 **NMI** 噪声出现的总次数。

例如，**CPU 3** 运行了 **599** 个 **1 秒 Runtime** 的周期。CPU 在整个执行过程中收到了 **86 us**
的噪声，为应用程序留下了 **99.99997 %** 的 CPU 时间。在最差的单个周期中，CPU 给应用程序造成了
**4 us** 的噪声，但这肯定是由不止一次单次噪声造成的，因为 **Max Single** 噪声是 **3 us**。该 CPU
有 **HW 噪声**，频率为 **6 次/十分钟**。该 CPU 也有 **NMI**，频率更高：大约**每秒 7 次**。

在理想情况下，该工具应报告 **0** 个与硬件相关的噪声。例如，通过禁用超线程来消除硬件噪声，并
禁用 TSC 看门狗来消除 NMI（可以使用 **rtla hwnoise** 的追踪选项来识别这一点），就能够达到
```

  # rtla hwnoise -c 1-7 -T 1 -d 10m -q
                                          Hardware-related Noise
  duration:   0 00:10:00 | time is in us
  CPU Period       Runtime        Noise  % CPU Aval   Max Noise   Max Single          HW          NMI
    1 #599       599000000            0   100.00000           0            0           0            0
    2 #599       599000000            0   100.00000           0            0           0            0
    3 #599       599000000            0   100.00000           0            0           0            0
    4 #599       599000000            0   100.00000           0            0           0            0
    5 #599       599000000            0   100.00000           0            0           0            0
    6 #599       599000000            0   100.00000           0            0           0            0
    7 #599       599000000            0   100.00000           0            0           0            0

```
## 另请参阅（SEE ALSO）


**rtla-osnoise**\(1)

`Osnoise tracer <https://docs.kernel.org/trace/osnoise-tracer.html>`__

## 作者（AUTHOR）


Written by Daniel Bristot de Oliveira <bristot@kernel.org>
