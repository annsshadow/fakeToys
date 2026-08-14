
## 控制台（Console）

本页面介绍 Linux 内核控制台（console）子系统的核心数据结构与对外 API，面向驱动与内核开发者，涵盖 struct console、struct consw 的定义，以及控制台注册、输出与内部实现相关的函数接口。


本文件记录内核控制台（console）子系统的数据结构与 API。

## 结构体 Console


   :identifiers: console cons_flags

### 内部实现


   :identifiers: nbcon_state nbcon_prio nbcon_context nbcon_write_context

## 结构体 Consw


   :identifiers: consw

## 控制台函数


   :identifiers: console_srcu_read_flags console_srcu_write_flags
        console_is_registered for_each_console_srcu for_each_console

   :export:
   :export:

### 内部实现


   :internal:
   :internal:
