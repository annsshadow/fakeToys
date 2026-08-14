## 子系统 Trace Points: 电源


电源 tracing 系统 captures 事件 related 电源 transitions
内核. Broadly speaking three major subheadings:

- 电源 状态 switch 报告 事件 related suspend (S-states),
cpuidle (C-states) cpufreq (P-states)
- 系统 时钟 related changes
- 电源 domains related changes transitions

文档 描述 tracepoints
might useful.

Cf. include/trace/事件/电源.h 事件 definitions.

## 1. 电源 状态 switch 事件


### 1.1 Trace API


'处理器' 事件 class gathers CPU-related 事件: cpuidle
cpufreq.
```

  cpu_idle		"state=%lu cpu_id=%lu"
  cpu_frequency		"state=%lu cpu_id=%lu"
  cpu_frequency_limits	"min=%lu max=%lu cpu_id=%lu"

```
suspend 事件 使用 indicate 系统 going out
suspend 模式:
```

  machine_suspend		"state=%lu"


```
说明: 值 '-1' '4294967295' 状态 means exit current 状态,
i.e. trace_cpu_idle(4, smp_processor_id()) means 系统
enters idle 状态 4, trace_cpu_idle(PWR_EVENT_EXIT, smp_processor_id())
means 系统 exits previous idle 状态.

事件 '状态=4294967295' trace very important 用户
space tools 使用 检测 end current 状态,
correctly draw states diagrams calculate accurate statistics .

## 2. Clocks 事件

时钟 事件 使用 时钟 启用/禁用
时钟 rate change.
```

  clock_enable		"%s state=%lu cpu_id=%lu"
  clock_disable		"%s state=%lu cpu_id=%lu"
  clock_set_rate		"%s state=%lu cpu_id=%lu"

```
first 参数 gives 时钟 名称 (e.g. "gpio1_iclk").
second 参数 '1' 启用, '0' 禁用, target
时钟 rate set_rate.

## 3. 电源 domains 事件

电源 domain 事件 使用 电源 domains transitions
```

  power_domain_target	"%s state=%lu cpu_id=%lu"

```
first 参数 gives 电源 domain 名称 (e.g. "mpu_pwrdm").
second 参数 电源 domain target 状态.

## 4. PM QoS 事件

PM QoS 事件 使用 QoS 添加/更新/移除 请求
target/标志 更新.
```

  pm_qos_update_target               "action=%s prev_value=%d curr_value=%d"
  pm_qos_update_flags                "action=%s prev_value=0x%x curr_value=0x%x"

```
first 参数 gives QoS action 名称 (e.g. "ADD_REQ").
second 参数 previous QoS 值.
third 参数 current QoS 值 更新.

事件 使用 设备 PM QoS 添加/更新/移除 请求.
```

  dev_pm_qos_add_request             "device=%s type=%s new_value=%d"
  dev_pm_qos_update_request          "device=%s type=%s new_value=%d"
  dev_pm_qos_remove_request          "device=%s type=%s new_value=%d"

```
first 参数 gives 设备 名称 tries 添加/更新/移除
QoS requests.
second 参数 gives 请求 类型 (e.g. "DEV_PM_QOS_RESUME_LATENCY").
third 参数 值 added/updated/removed.

, 事件 使用 CPU latency QoS 添加/更新/移除 请求.
```

  pm_qos_add_request        "value=%d"
  pm_qos_update_request     "value=%d"
  pm_qos_remove_request     "value=%d"

```
参数 值 added/updated/removed.
