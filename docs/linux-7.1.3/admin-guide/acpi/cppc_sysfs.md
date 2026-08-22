
## 协作式处理器性能控制（CPPC


## CPPC


ACPI 规范中定义的 CPPC 描述了一种机制，供操作系统在连续且抽象的性能刻度上管理逻辑处理器的性能。CPPC 暴露一组寄存器来描述抽象性能刻度、请求性能级别以及测量CPU 的交付性能
有关 CPPC 的更多细节，请参ACPI 规范
http://uefi.org/specifications

```

  /sys/devices/system/cpu/cpuX/acpi_cppc/

```
```

  $ ls -lR  /sys/devices/system/cpu/cpu0/acpi_cppc/
  /sys/devices/system/cpu/cpu0/acpi_cppc/:
  total 0
  -r--r--r-- 1 root root 65536 Mar  5 19:38 feedback_ctrs
  -r--r--r-- 1 root root 65536 Mar  5 19:38 highest_perf
  -r--r--r-- 1 root root 65536 Mar  5 19:38 lowest_freq
  -r--r--r-- 1 root root 65536 Mar  5 19:38 lowest_nonlinear_perf
  -r--r--r-- 1 root root 65536 Mar  5 19:38 lowest_perf
  -r--r--r-- 1 root root 65536 Mar  5 19:38 nominal_freq
  -r--r--r-- 1 root root 65536 Mar  5 19:38 nominal_perf
  -r--r--r-- 1 root root 65536 Mar  5 19:38 reference_perf
  -r--r--r-- 1 root root 65536 Mar  5 19:38 wraparound_time

```
- highest_perf：本处理器的最高性能（抽象刻度）- nominal_perf：本处理器的最高持续性能（抽象刻度）- lowest_nonlinear_perf：本处理器在非线性节能下的最低性能（抽象刻度）- lowest_perf：本处理器的最低性能（抽象刻度）
- lowest_freq：对lowest_perf CPU 频率（单MHz）- nominal_freq：对nominal_perf CPU 频率（单MHz）  上述频率仅应用于以频率而非抽象刻度来报告处理器性能，不应将其用于任何功能性决策
- feedback_ctrs：包含参考性能计数器与交付性能计数器  参考计数器随处理器参考性能成比例递增  交付计数器随处理器交付性能成比例递增- wraparound_time：反馈计数器回绕所需的最短时间（单位秒）- reference_perf：参考性能计数器累加时的性能级别（抽象刻度）

## 计算平均交付性能


下面描述通过在时T1 T2 两次获取反馈计数器快照来计算平均交付性能的步骤
  T1: feedback_ctrs 读取fbc_t1
      等待或运行某些工作负
  T2: feedback_ctrs 读取fbc_t2

```

  delivered_counter_delta = fbc_t2[del] - fbc_t1[del]
  reference_counter_delta = fbc_t2[ref] - fbc_t1[ref]

  delivered_perf = (reference_perf x delivered_counter_delta) / reference_counter_delta

```
