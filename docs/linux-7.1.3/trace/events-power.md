## 瀛愮郴缁?Trace Points: 鐢垫簮


鐢垫簮 tracing 绯荤粺 captures 浜嬩欢 related 鐢垫簮 transitions
鍐呮牳. Broadly speaking three major subheadings:

- 鐢垫簮 鐘舵€?switch 鎶ュ憡 浜嬩欢 related suspend (S-states),
cpuidle (C-states) cpufreq (P-states)
- 绯荤粺 鏃堕挓 related changes
- 鐢垫簮 domains related changes transitions

鏂囨。 鎻忚堪 tracepoints
might useful.

Cf. include/trace/浜嬩欢/鐢垫簮.h 浜嬩欢 definitions.

## 1. 鐢垫簮 鐘舵€?switch 浜嬩欢


### 1.1 Trace API


'澶勭悊鍣? 浜嬩欢 class gathers CPU-related 浜嬩欢: cpuidle
cpufreq.
```

  cpu_idle		"state=%lu cpu_id=%lu"
  cpu_frequency		"state=%lu cpu_id=%lu"
  cpu_frequency_limits	"min=%lu max=%lu cpu_id=%lu"

```
suspend 浜嬩欢 浣跨敤 indicate 绯荤粺 going out
suspend 妯″紡:
```

  machine_suspend		"state=%lu"


```
璇存槑: 鍊?'-1' '4294967295' 鐘舵€?means exit current 鐘舵€?
i.e. trace_cpu_idle(4, smp_processor_id()) means 绯荤粺
enters idle 鐘舵€?4, trace_cpu_idle(PWR_EVENT_EXIT, smp_processor_id())
means 绯荤粺 exits previous idle 鐘舵€?

浜嬩欢 '鐘舵€?4294967295' trace very important 鐢ㄦ埛
space tools 浣跨敤 妫€娴?end current 鐘舵€?
correctly draw states diagrams calculate accurate statistics .

## 2. Clocks 浜嬩欢

鏃堕挓 浜嬩欢 浣跨敤 鏃堕挓 鍚敤/绂佺敤
鏃堕挓 rate change.
```

  clock_enable		"%s state=%lu cpu_id=%lu"
  clock_disable		"%s state=%lu cpu_id=%lu"
  clock_set_rate		"%s state=%lu cpu_id=%lu"

```
first 鍙傛暟 gives 鏃堕挓 鍚嶇О (e.g. "gpio1_iclk").
second 鍙傛暟 '1' 鍚敤, '0' 绂佺敤, target
鏃堕挓 rate set_rate.

## 3. 鐢垫簮 domains 浜嬩欢

鐢垫簮 domain 浜嬩欢 浣跨敤 鐢垫簮 domains transitions
```

  power_domain_target	"%s state=%lu cpu_id=%lu"

```
first 鍙傛暟 gives 鐢垫簮 domain 鍚嶇О (e.g. "mpu_pwrdm").
second 鍙傛暟 鐢垫簮 domain target 鐘舵€?

## 4. PM QoS 浜嬩欢

PM QoS 浜嬩欢 浣跨敤 QoS 娣诲姞/鏇存柊/绉婚櫎 璇锋眰
target/鏍囧織 鏇存柊.
```

  pm_qos_update_target               "action=%s prev_value=%d curr_value=%d"
  pm_qos_update_flags                "action=%s prev_value=0x%x curr_value=0x%x"

```
first 鍙傛暟 gives QoS action 鍚嶇О (e.g. "ADD_REQ").
second 鍙傛暟 previous QoS 鍊?
third 鍙傛暟 current QoS 鍊?鏇存柊.

浜嬩欢 浣跨敤 璁惧 PM QoS 娣诲姞/鏇存柊/绉婚櫎 璇锋眰.
```

  dev_pm_qos_add_request             "device=%s type=%s new_value=%d"
  dev_pm_qos_update_request          "device=%s type=%s new_value=%d"
  dev_pm_qos_remove_request          "device=%s type=%s new_value=%d"

```
first 鍙傛暟 gives 璁惧 鍚嶇О tries 娣诲姞/鏇存柊/绉婚櫎
QoS requests.
second 鍙傛暟 gives 璇锋眰 绫诲瀷 (e.g. "DEV_PM_QOS_RESUME_LATENCY").
third 鍙傛暟 鍊?added/updated/removed.

, 浜嬩欢 浣跨敤 CPU latency QoS 娣诲姞/鏇存柊/绉婚櫎 璇锋眰.
```

  pm_qos_add_request        "value=%d"
  pm_qos_update_request     "value=%d"
  pm_qos_remove_request     "value=%d"

```
鍙傛暟 鍊?added/updated/removed.
