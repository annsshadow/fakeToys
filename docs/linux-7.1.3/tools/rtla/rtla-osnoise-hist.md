
## rtla-osnoise-hist

### 显示 osnoise 跟踪器采样结果的直方

:Manual section: 1

## 概要（SYNOPSIS

**rtla osnoise hist** [**OPTIONS**]

## 描述（DESCRIPTION

**rtla osnoise hist** 工具将所有的 **osnoise:sample_threshold** 事件发生情况收集
到一张直方图中，并以对用户友好的方式显示结果。该工具还允许对 **osnoise** 跟踪进行许多配置并收集跟踪器输出
## 选项（OPTIONS

## 示例（EXAMPLE

在下面的示例中，**osnoise** 跟踪器线程被设置为以实时优先**FIFO:1** CPUs **0-11** 上运行，每个周期运行 **900ms**（默**1s**）。缩短运行时间的原因
是为了避免饿**rtla** 工具。该工具还被设置为运**one minute**。输出如下：

```

  [root@f34 ~/]# rtla osnoise hist -P F:1 -c 0-11 -r 900000 -d 1M -b 10 -E 25
  # RTLA osnoise histogram
  # Time unit is microseconds (us)
  # Duration:   0 00:01:00
  Index   CPU-000   CPU-001   CPU-002   CPU-003   CPU-004   CPU-005   CPU-006   CPU-007   CPU-008   CPU-009   CPU-010   CPU-011
  0         42982     46287     51779     53740     52024     44817     49898     36500     50408     50128     49523     52377
  10        12224      8356      2912       878      2667     10155      4573     18894      4214      4836      5708      2413
  20            8         5        12         2        13        24        20        41        29        53        39        39
  30            1         1         0         0        10         3         6        19        15        31        30        38
  40            0         0         0         0         0         4         2         7         2         3         8        11
  50            0         0         0         0         0         0         0         0         0         1         1         2
  over:         0         0         0         0         0         0         0         0         0         0         0         0
  count:    55215     54649     54703     54620     54714     55003     54499     55461     54668     55052     55309     54880
  min:          0         0         0         0         0         0         0         0         0         0         0         0
  avg:          0         0         0         0         0         0         0         0         0         0         0         0
  max:         30        30        20        20        30        40        40        40        40        50        50        50

```
## 另请参阅（SEE ALSO

**rtla-osnoise**\(1), **rtla-osnoise-top**\(1)

`Osnoise tracer <https://docs.kernel.org/trace/osnoise-tracer.html>`__

## 作者（AUTHOR

Written by Daniel Bristot de Oliveira <bristot@kernel.org>
