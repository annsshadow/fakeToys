
## ACPICA 跟踪机制（Trace Facility

:Copyright: |copy| 2015, Intel Corporation
:Author: Lv Zheng <lv.zheng@intel.com>


## 摘要

本文档描述方法跟踪（method tracing）机制的功能与接口
## 功能与使用示

ACPICA 提供了方法跟踪能力。目前基于该能力实现了两个功能
### 日志缩减器（Log reducer

当启CONFIG_ACPI_DEBUG 时，ACPICA 子系统会输出调试信息。通过 ACPI_DEBUG_PRINT() 宏输出的调试消息可以在两个层级上被缩减——按组件层级（称debug layer，通过 /sys/module/acpi/parameters/debug_layer 配置）和按类型层级（称为 debug level，通过 /sys/module/acpi/parameters/debug_level 配置）
但是，当把特layer/level 应用于控制方法求值时，调试输出的数量仍可能大到无法放入内核日志缓冲区。因此产生了这样的思路：仅在控制方法求值开始时启用特定 debug layer/level（通常更详细）的日志，并在控制方法求值停止时禁用详细日志
以下命令示例说明了“日志缩减器”功能的使用
a. 当控制方法求值时，过滤掉匹配 debug layer/level 的日```

      # cd /sys/module/acpi/parameters
      # echo "0xXXXXXXXX" > trace_debug_layer
      # echo "0xYYYYYYYY" > trace_debug_level
      # echo "enable" > trace_state

```
b. 当指定的控制方法求值时，过滤掉匹配 debug layer/level 的日```

      # cd /sys/module/acpi/parameters
      # echo "0xXXXXXXXX" > trace_debug_layer
      # echo "0xYYYYYYYY" > trace_debug_level
      # echo "\PPPP.AAAA.TTTT.HHHH" > trace_method_name
      # echo "method" > /sys/module/acpi/parameters/trace_state

```
c. 当指定的控制方法求值时，过滤掉匹配 debug layer/level 的日志（仅一次）
```

      # cd /sys/module/acpi/parameters
      # echo "0xXXXXXXXX" > trace_debug_layer
      # echo "0xYYYYYYYY" > trace_debug_level
      # echo "\PPPP.AAAA.TTTT.HHHH" > trace_method_name
      # echo "method-once" > /sys/module/acpi/parameters/trace_state

```
其中   0xXXXXXXXX/0xYYYYYYYY
     有关可能debug layer/level 掩码取值，请参Documentation/firmware-guide/acpi/debug.rst   \PPPP.AAAA.TTTT.HHHH
     ACPI 命名空间中某个控制方法的完整路径     它不必是控制方法求值的入口
### AML 跟踪器（AML tracer

方法跟踪机制会在 AML 解释器开停止执行某个控制方法或某AML 操作码（opcode）的“跟踪点”处添加特殊的日志条目。注意这些日志条目的格式```

   [    0.186427]   exdebug-0398 ex_trace_point        : Method Begin [0xf58394d8:\_SB.PCI0.LPCB.ECOK] execution.
   [    0.186630]   exdebug-0398 ex_trace_point        : Opcode Begin [0xf5905c88:If] execution.
   [    0.186820]   exdebug-0398 ex_trace_point        : Opcode Begin [0xf5905cc0:LEqual] execution.
   [    0.187010]   exdebug-0398 ex_trace_point        : Opcode Begin [0xf5905a20:-NamePath-] execution.
   [    0.187214]   exdebug-0398 ex_trace_point        : Opcode End [0xf5905a20:-NamePath-] execution.
   [    0.187407]   exdebug-0398 ex_trace_point        : Opcode Begin [0xf5905f60:One] execution.
   [    0.187594]   exdebug-0398 ex_trace_point        : Opcode End [0xf5905f60:One] execution.
   [    0.187789]   exdebug-0398 ex_trace_point        : Opcode End [0xf5905cc0:LEqual] execution.
   [    0.187980]   exdebug-0398 ex_trace_point        : Opcode Begin [0xf5905cc0:Return] execution.
   [    0.188146]   exdebug-0398 ex_trace_point        : Opcode Begin [0xf5905f60:One] execution.
   [    0.188334]   exdebug-0398 ex_trace_point        : Opcode End [0xf5905f60:One] execution.
   [    0.188524]   exdebug-0398 ex_trace_point        : Opcode End [0xf5905cc0:Return] execution.
   [    0.188712]   exdebug-0398 ex_trace_point        : Opcode End [0xf5905c88:If] execution.
   [    0.188903]   exdebug-0398 ex_trace_point        : Method End [0xf58394d8:\_SB.PCI0.LPCB.ECOK] execution.

```
开发者可以利用这些特殊日志条目来追踪 AML 解释过程，从而有助于问题调试和性能调优。注意，由于“AML tracer”日志是通过 ACPI_DEBUG_PRINT() 宏实现的，启用“AML tracer”日志同样需要开CONFIG_ACPI_DEBUG
以下命令示例说明了“AML tracer”功能的使用
a. 当控制方法开停止时，过滤出方法开停止的“AML tracer”日```

      # cd /sys/module/acpi/parameters
      # echo "0x80" > trace_debug_layer
      # echo "0x10" > trace_debug_level
      # echo "enable" > trace_state

```
b. 当指定的控制方法开停止时，过滤出“AML tracer”日```

      # cd /sys/module/acpi/parameters
      # echo "0x80" > trace_debug_layer
      # echo "0x10" > trace_debug_level
      # echo "\PPPP.AAAA.TTTT.HHHH" > trace_method_name
      # echo "method" > trace_state

```
c. 当指定的控制方法开停止时，过滤出“AML tracer”日志（仅一次）
```

      # cd /sys/module/acpi/parameters
      # echo "0x80" > trace_debug_layer
      # echo "0x10" > trace_debug_level
      # echo "\PPPP.AAAA.TTTT.HHHH" > trace_method_name
      # echo "method-once" > trace_state

```
d. 当指定的方法/操作码开停止时，过滤出“AML tracer”日```

      # cd /sys/module/acpi/parameters
      # echo "0x80" > trace_debug_layer
      # echo "0x10" > trace_debug_level
      # echo "\PPPP.AAAA.TTTT.HHHH" > trace_method_name
      # echo "opcode" > trace_state

```
e. 当指定的方法/操作码开停止时，过滤出“AML tracer”日```

      # cd /sys/module/acpi/parameters
      # echo "0x80" > trace_debug_layer
      # echo "0x10" > trace_debug_level
      # echo "\PPPP.AAAA.TTTT.HHHH" > trace_method_name
      # echo "opcode-opcode" > trace_state

```
注意，上述所有方法跟踪机制相关的模块参数也可通过内核启动参数指定
```

   acpi.trace_debug_layer=0x80 acpi.trace_debug_level=0x10 \
   acpi.trace_method_name=\_SB.LID0._LID acpi.trace_state=opcode-once


```
## 接口描述


所有方法跟踪功能都可通过 ACPI 模块参数配置，这些参数位/sys/module/acpi/parameters/
trace_method_name
  用户想要跟踪AML 方法的完整路径
  注意完整路径的名称段中不应包含结尾的 “_”，但可以包“\以构成绝对路径
trace_debug_layer
  启用跟踪功能时使用的临时 debug_layer
  默认使用 ACPI_EXECUTER (0x80)，即用于匹配所有“AML tracer”日志的 debug_layer
trace_debug_level
  启用跟踪功能时使用的临时 debug_level
  默认使用 ACPI_LV_TRACE_POINT (0x10)，即用于匹配所有“AML tracer”日志的 debug_level
trace_state
  跟踪功能的状态
  用户可通过执行
```

   # echo string > /sys/module/acpi/parameters/trace_state

```
来启禁用该调试跟踪功能。其“string应为以下之一
"disable"
  禁用方法跟踪功能
"enable"
  启用方法跟踪功能
  在任意方法执行期间，匹配 “trace_debug_layer/trace_debug_levelACPICA 调试消息都会被记录
"method"
  启用方法跟踪功能
  “trace_method_name的方法执行期间，匹配 “trace_debug_layer/trace_debug_levelACPICA 调试消息会被记录
"method-once"
  启用方法跟踪功能
  “trace_method_name的方法执行期间，匹配 “trace_debug_layer/trace_debug_levelACPICA 调试消息仅会被记录一次
"opcode"
  启用方法跟踪功能
  “trace_method_name的方操作码执行期间，匹配 “trace_debug_layer/trace_debug_levelACPICA 调试消息会被记录
"opcode-once"
  启用方法跟踪功能
  “trace_method_name的方操作码执行期间，匹配 “trace_debug_layer/trace_debug_levelACPICA 调试消息仅会被记录一次
注意，“enable与其他功能启用选项的区别在于：

1. 指定 “enable时，由于 “trace_debug_layer/trace_debug_level会应用于所有控制方法求值，因此在将 “trace_state配置“enable后，“trace_method_name会被重置NULL2. 指定 “method/opcode时，如果在将这些选项配置“trace_state“trace_method_nameNULL，则 “trace_debug_layer/trace_debug_level会应用于所有控制方法求值