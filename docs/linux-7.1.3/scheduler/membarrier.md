
## membarrier() 系统调用


## MEMBARRIER_CMD_{PRIVATE,GLOBAL}_EXPEDITED - 架构要求


### 更新 rq->curr 之前的内存屏


命令 MEMBARRIER_CMD_PRIVATE_EXPEDITED MEMBARRIER_CMD_GLOBAL_EXPEDITED 要求每个架构在从用户空间返回后、更rq->curr 之前具有一个完整内存屏障。该屏障__schedule() 中的 rq_lock(); smp_mb__after_spinlock() 序列隐含提供。该屏障membarrier 系统调用退出附近的一个完整屏障相匹配，参membarrier_{private,global}_expedited()

### 更新 rq->curr 之后的内存屏


命令 MEMBARRIER_CMD_PRIVATE_EXPEDITED MEMBARRIER_CMD_GLOBAL_EXPEDITED 要求每个架构在更rq->curr 之后、返回用户空间之前具有一个完整内存屏障。各个架构上提供该屏障的方案如下

 - alpha、arc、arm、hexagon、mips 依赖 finish_lock_switch() spin_unlock() 隐含的完整屏障

 - arm64 依赖 switch_to() 隐含的完整屏障

 - powerpc、riscv、s390、sparc、x86 依赖 switch_mm() 隐含的完整屏障（mm 不为 NULL）；否则它们依赖 mmdrop() 隐含的完整屏障。在 powerpc riscv 上，switch_mm() 依赖 membarrier_arch_switch_mm()

该屏障与 membarrier 系统调用入口附近的一个完整屏障相匹配，参membarrier_{private,global}_expedited()
