## 硬件自旋锁框


## 简


硬件自旋锁模块为异构处理器之间、以及那些不在单一共享操作系统下运行的处理器之间，提供用于同步与互斥的硬件辅助

例如，OMAP4 拥有双核 Cortex-A9、双Cortex-M3 以及一C64x+ DSP，每一个都运行着不同的操作系统（主核 A9 通常运行 Linux，而从M3 DSP 运行某种 RTOS）

通用hwspinlock 框架允许与平台无关的驱动使用 hwspinlock 设备，以访问在远程处理器之间共享的数据结构——否则这些处理器没有其他机制来完成同步与互斥操作

例如，这对于处理器间通信是必要的：在 OMAP4 上，CPU 密集的多媒体任务由主核卸载到远程M3 C64x+ 从核处理器（通过一个名Syslink IPC 子系统）

为了实现快速的基于消息的通信，需要最小化的内核支持，以将来自远程处理器的消息投递给相应的用户进程

这种通信基于在远程处理器之间共享的简单数据结构，对其访问使用 hwspinlock 模块进行同步（远程处理器直接将新消息放入该共享数据结构中）

通用hwspinlock 接口使得编写通用的、与平台无关的驱动成为可能

## 用户 API


```
  struct hwspinlock *hwspin_lock_request_specific(unsigned int id);
```
分配一个特定的 hwspinlock id 并返回其地址，如果该 hwspinlock 已被占用则返NULL。通常板级代码会调用此函数来为预定义的目的保留特定hwspinlock id

应从进程上下文调用（可能睡眠）

```
  int of_hwspin_lock_get_id(struct device_node *np, int index);
```
检索基DT phandle 的特定锁的全局id。该函数hwspinlock 模块DT 用户提供了一种获取特hwspinlock 全局id 的方式，从而可以使用常规的 hwspin_lock_request_specific() API 来请求它

该函数成功时返回一个锁 id 号，hwspinlock 设备尚未向核心注册则返回 -EPROBE_DEFER，其他情况下返回其他错误值

应从进程上下文调用（可能睡眠）

```
  int hwspin_lock_free(struct hwspinlock *hwlock);
```
释放先前分配hwspinlock；成功时返回 0，失败时返回相应的错误码（例如，若该 hwspinlock 已经空闲，则返回 -EINVAL）

应从进程上下文调用（可能睡眠）

```
  int hwspin_lock_bust(struct hwspinlock *hwlock, unsigned int id);
```
在验hwspinlock 的拥有者之后，释放一个先前获取的 hwspinlock；成功时返回 0，失败时返回相应的错误码（例如，若该 bust 操作对特hwspinlock 未定义，则返-EOPNOTSUPP）

应从进程上下文调用（可能睡眠）

```
  int hwspin_lock_timeout(struct hwspinlock *hwlock, unsigned int timeout);
```
以超时限制（以毫秒为单位）锁定一个先前分配的 hwspinlock。如果该 hwspinlock 已被占用，函数会忙等以等待其释放，但在超时耗尽时放弃。成功从此函数返回后，抢占被禁用，因此调用者不得睡眠，并建议尽快释hwspinlock，以最小化远程核在硬件互连上的轮询

成功时返0，否则返回相应的错误码（最典型的是 -ETIMEDOUT，表示超时毫秒后hwspinlock 仍然忙）。该函数永远不会睡眠

```
  int hwspin_lock_timeout_irq(struct hwspinlock *hwlock, unsigned int timeout);
```
以超时限制（以毫秒为单位）锁定一个先前分配的 hwspinlock。如果该 hwspinlock 已被占用，函数会忙等以等待其释放，但在超时耗尽时放弃。成功从此函数返回后，抢占与本地中断被禁用，因此调用者不得睡眠，并建议尽快释hwspinlock

成功时返0，否则返回相应的错误码（最典型的是 -ETIMEDOUT，表示超时毫秒后hwspinlock 仍然忙）。该函数永远不会睡眠

```
  int hwspin_lock_timeout_irqsave(struct hwspinlock *hwlock, unsigned int to,
				  unsigned long *flags);
```
以超时限制（以毫秒为单位）锁定一个先前分配的 hwspinlock。如果该 hwspinlock 已被占用，函数会忙等以等待其释放，但在超时耗尽时放弃。成功从此函数返回后，抢占被禁用，本地中断被禁用，其先前的状态保存在给定flags 占位符中。调用者不得睡眠，并建议尽快释hwspinlock

成功时返0，否则返回相应的错误码（最典型的是 -ETIMEDOUT，表示超时毫秒后hwspinlock 仍然忙）。该函数永远不会睡眠

```
  int hwspin_lock_timeout_raw(struct hwspinlock *hwlock, unsigned int timeout);
```
以超时限制（以毫秒为单位）锁定一个先前分配的 hwspinlock。如果该 hwspinlock 已被占用，函数会忙等以等待其释放，但在超时耗尽时放弃

注意：用户必须用互斥体或自旋锁保护获取硬件锁的例程，以避免死锁，从而让用户能够在硬件锁下执行一些耗时的或可睡眠的操作

成功时返0，否则返回相应的错误码（最典型的是 -ETIMEDOUT，表示超时毫秒后hwspinlock 仍然忙）。该函数永远不会睡眠

```
  int hwspin_lock_timeout_in_atomic(struct hwspinlock *hwlock, unsigned int to);
```
以超时限制（以毫秒为单位）锁定一个先前分配的 hwspinlock。如果该 hwspinlock 已被占用，函数会忙等以等待其释放，但在超时耗尽时放弃

此函数只能从原子上下文调用，且超时值不应超过几毫秒

成功时返0，否则返回相应的错误码（最典型的是 -ETIMEDOUT，表示超时毫秒后hwspinlock 仍然忙）。该函数永远不会睡眠

```
  int hwspin_trylock(struct hwspinlock *hwlock);
```
尝试锁定一个先前分配的 hwspinlock，但如果它已被占用则立即失败

成功从此函数返回后，抢占被禁用，因此调用者不得睡眠，并建议尽快释hwspinlock，以最小化远程核在硬件互连上的轮询

成功时返0，否则返回相应的错误码（最典型的是 -EBUSY，表示该 hwspinlock 已被占用）。该函数永远不会睡眠

```
  int hwspin_trylock_irq(struct hwspinlock *hwlock);
```
尝试锁定一个先前分配的 hwspinlock，但如果它已被占用则立即失败

成功从此函数返回后，抢占与本地中断被禁用，因此调用者不得睡眠，并建议尽快释hwspinlock

成功时返0，否则返回相应的错误码（最典型的是 -EBUSY，表示该 hwspinlock 已被占用）。该函数永远不会睡眠

```
  int hwspin_trylock_irqsave(struct hwspinlock *hwlock, unsigned long *flags);
```
尝试锁定一个先前分配的 hwspinlock，但如果它已被占用则立即失败

成功从此函数返回后，抢占被禁用，本地中断被禁用，其先前的状态保存在给定flags 占位符中。调用者不得睡眠，并建议尽快释hwspinlock

成功时返0，否则返回相应的错误码（最典型的是 -EBUSY，表示该 hwspinlock 已被占用）。该函数永远不会睡眠

```
  int hwspin_trylock_raw(struct hwspinlock *hwlock);
```
尝试锁定一个先前分配的 hwspinlock，但如果它已被占用则立即失败

注意：用户必须用互斥体或自旋锁保护获取硬件锁的例程，以避免死锁，从而让用户能够在硬件锁下执行一些耗时的或可睡眠的操作

成功时返0，否则返回相应的错误码（最典型的是 -EBUSY，表示该 hwspinlock 已被占用）。该函数永远不会睡眠

```
  int hwspin_trylock_in_atomic(struct hwspinlock *hwlock);
```
尝试锁定一个先前分配的 hwspinlock，但如果它已被占用则立即失败

此函数只能从原子上下文调用

成功时返0，否则返回相应的错误码（最典型的是 -EBUSY，表示该 hwspinlock 已被占用）。该函数永远不会睡眠

```
  void hwspin_unlock(struct hwspinlock *hwlock);
```
解锁一个先前锁定的 hwspinlock。总是成功，并且可以从任何上下文调用（该函数永远不会睡眠）


  代码**绝不*去解锁一个已经解锁的 hwspinlock（对此没有任何保护）

```
  void hwspin_unlock_irq(struct hwspinlock *hwlock);
```
解锁一个先前锁定的 hwspinlock 并启用本地中断。调用*绝不*去解锁一个已经解锁的 hwspinlock。这样做被视为一个缺陷（对此没有任何保护）。成功从此函数返回后，抢占与本地中断被启用。该函数永远不会睡眠

```
  void
  hwspin_unlock_irqrestore(struct hwspinlock *hwlock, unsigned long *flags);
```
解锁一个先前锁定的 hwspinlock

调用*绝不*去解锁一个已经解锁的 hwspinlock。这样做被视为一个缺陷（对此没有任何保护）。成功从此函数返回后，抢占被重新启用，本地中断的状态被恢复到保存在给定 flags 中的状态。该函数永远不会睡眠

```
  void hwspin_unlock_raw(struct hwspinlock *hwlock);
```
解锁一个先前锁定的 hwspinlock

调用*绝不*去解锁一个已经解锁的 hwspinlock。这样做被视为一个缺陷（对此没有任何保护）。该函数永远不会睡眠

```
  void hwspin_unlock_in_atomic(struct hwspinlock *hwlock);
```
解锁一个先前锁定的 hwspinlock

调用*绝不*去解锁一个已经解锁的 hwspinlock。这样做被视为一个缺陷（对此没有任何保护）。该函数永远不会睡眠

## 典型用法


```
	#include <linux/hwspinlock.h>
	#include <linux/err.h>

	int hwspinlock_example(void)
	{
		struct hwspinlock *hwlock;
		int ret;

		/*
		* assign a specific hwspinlock id - this should be called early
		* by board init code.
		*/
		hwlock = hwspin_lock_request_specific(PREDEFINED_LOCK_ID);
		if (!hwlock)
			...

		/* try to take it, but don't spin on it */
		ret = hwspin_trylock(hwlock);
		if (!ret) {
			pr_info("lock is already taken\n");
			return -EBUSY;
		}

		/*
		* we took the lock, do our thing now, but do NOT sleep
		*/

		/* release the lock */
		hwspin_unlock(hwlock);

		/* free the lock */
		ret = hwspin_lock_free(hwlock);
		if (ret)
			...

		return ret;
	}
```
## 面向实现者的 API


```
  int hwspin_lock_register(struct hwspinlock_device *bank, struct device *dev,
		const struct hwspinlock_ops *ops, int base_id, int num_locks);
```
由底层的平台特定实现调用，以注册一个新hwspinlock 设备（通常是一组数量众多的锁）。应从进程上下文调用（该函数可能睡眠）

成功时返0，失败时返回相应的错误码

```
  int hwspin_lock_unregister(struct hwspinlock_device *bank);
```
由底层的厂商特定实现调用，以注销一hwspinlock 设备（通常是一组数量众多的锁）

应从进程上下文调用（该函数可能睡眠）

成功时返hwspinlock 的地址，错误时返回 NULL（例如，若该 hwspinlock 仍在使用中）

## 重要结构


struct hwspinlock_device 是一个通常包含一组硬件锁的设备。它由底层的 hwspinlock 实现通过 hwspin_lock_register() API 注册

```
	/**
	* struct hwspinlock_device - a device which usually spans numerous hwspinlocks
	* @dev: underlying device, will be used to invoke runtime PM api
	* @ops: platform-specific hwspinlock handlers
	* @base_id: id index of the first lock in this device
	* @num_locks: number of locks in this device
	* @lock: dynamically allocated array of 'struct hwspinlock'
	*/
	struct hwspinlock_device {
		struct device *dev;
		const struct hwspinlock_ops *ops;
		int base_id;
		int num_locks;
		struct hwspinlock lock[0];
	};
```
struct hwspinlock_device 包含一hwspinlock 结构体数组，每个
```
	/**
	* struct hwspinlock - this struct represents a single hwspinlock instance
	* @bank: the hwspinlock_device structure which owns this lock
	* @lock: initialized and used by hwspinlock core
	* @priv: private data, owned by the underlying platform-specific hwspinlock drv
	*/
	struct hwspinlock {
		struct hwspinlock_device *bank;
		spinlock_t lock;
		void *priv;
	};
```
注册一组锁时，hwspinlock 驱动只需要设置各锁的 priv 成员。其余成员由 hwspinlock 核心自身设置并初始化

## 实现回调


```
	struct hwspinlock_ops {
		int (*trylock)(struct hwspinlock *lock);
		void (*unlock)(struct hwspinlock *lock);
		void (*relax)(struct hwspinlock *lock);
	};
```
前两个回调是强制的：

->trylock() 回调应尝试一次获取锁，失败时返回 0，成功时返回 1。该回调**不得**睡眠

->unlock() 回调释放锁。它总是成功，并且同*不得**睡眠

->relax() 回调是可选的。当 hwspinlock 核心在某把锁上自旋时会被调用，底层的实现可以用它来强制在两次连续->trylock() 调用之间插入延迟。它**不得**睡眠
