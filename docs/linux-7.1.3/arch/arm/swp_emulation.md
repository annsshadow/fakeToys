### 已弃用的 SWP 指令的软件模拟 (CONFIG_SWP_EMULATE)


ARMv6 架构不建议使用 SWP/SWPB 指令，并建议
转向加载锁定/存储条件指令 LDREX 和 STREX。

ARMv7 多处理扩展引入了禁用这些功能的能力
指令，执行时触发未定义的指令异常。
使用 LDREX/STREX 或 LDREXB/STREXB 模拟捕获指令
顺序。如果发生内存访问错误（中止），则会出现分段错误
向触发进程发出信号。

/proc/cpu/swp_emulation 保存一些统计信息/信息，包括 PID
```

  Emulated SWP:		12
  Emulated SWPB:		0
  Aborted SWP{B}:		1
  Last process:		314


```
笔记：
当访问未缓存的共享区域时，LDREX/STREX 依赖于外部
事务监控块称为全局监视器来保持更新
原子性。如果您的系统没有实现全局监视器，则此选项可以
导致对未缓存内存执行 SWP 操作的程序死锁，如
STREX 操作总是会失败。
