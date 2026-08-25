
## rtla-osnoise-top

### 显示操作系统噪声（operating system noise）的摘要


:Manual section: 1

## 概要（SYNOPSIS
**rtla osnoise top** [**OPTIONS**]

## 描述（DESCRIPTION

**rtla osnoise top** **osnoise** tracer 收集周期性摘要，包括干扰源（interference source）发生的计数，并以用户友好的格式显示结果
该工具还允许**osnoise** tracer 进行许多配置以及收集 tracer 的输出
## 选项（OPTIONS


## 示例（EXAMPLE
在下面的示例中，**rtla osnoise top** 工具被设置为以实时优先级 **FIFO:1**，在 CPU **0-3** 上运行，每个周期运行 **900ms**（默**1s**）。减少运行时间的原因是为了避免使 rtla 工具饿死。该工具还被设置为运**一分钟**，并显示
```

  [root@f34 ~]# rtla osnoise top -P F:1 -c 0-3 -r 900000 -d 1M -q
                                          Operating System Noise
  duration:   0 00:01:00 | time is in us
  CPU Period       Runtime        Noise  % CPU Aval   Max Noise   Max Single          HW          NMI          IRQ      Softirq       Thread
    0 #59         53100000       304896    99.42580        6978           56         549            0        53111         1590           13
    1 #59         53100000       338339    99.36282        8092           24         399            0        53130         1448           31
    2 #59         53100000       290842    99.45227        6582           39         855            0        53110         1406           12
    3 #59         53100000       204935    99.61405        6251           33         290            0        53156         1460           12

```
## 参见（SEE ALSO

**rtla-osnoise**\(1), **rtla-osnoise-hist**\(1)

`Osnoise tracer <https://docs.kernel.org/trace/osnoise-tracer.html>`__

## 作者（AUTHOR

Written by Daniel Bristot de Oliveira <bristot@kernel.org>
