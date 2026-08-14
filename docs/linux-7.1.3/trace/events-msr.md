## MSR 跟踪事件


x86 内核支持跟踪大多数 MSR（Model Specific Register，模型特定寄存器）访问。
有关 Intel 系统上 MSR 的定义，请参见 SDM：https://www.intel.com/sdm（第 3 卷）

可用的跟踪点：

/sys/kernel/tracing/events/msr/

跟踪 MSR 读：

read_msr

  - msr: MSR 编号
  - val: 写入的值
  - failed: 若访问失败则为 1，否则为 0


跟踪 MSR 写：

write_msr

  - msr: MSR 编号
  - val: 写入的值
  - failed: 若访问失败则为 1，否则为 0


跟踪内核中的 RDPMC：

rdpmc

```

  cat /sys/kernel/tracing/trace | decode_msr.py /usr/src/linux/include/asm/msr-index.h

```
以添加符号化的 MSR 名称。
