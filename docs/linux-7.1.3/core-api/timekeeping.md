## ktime 访问

设备驱动可以使用 ktime_get() 以及 linux/timekeeping.h 中声明的许多相关函数来读当前时间。作为经验法则，如果两个访问器对某个特定用例同样适用，应优先使用名字更短那个
### 基于基本 ktime_t 的接

推荐的最简单形式返回一个不透明ktime_t，并带有为不同时钟参考返回时间的变体

	CLOCK_MONOTONIC

	适用于可靠的 timestamps 以及准确测量短时间间隔。从系统启动时开始计时，但在
	挂起（suspend）期间停止

	CLOCK_BOOTTIME

	类似 ktime_get()，但在挂起时不会停止。这可用于例如需要与其它机器跨挂起操	保持同步的密钥过期时间

	CLOCK_REALTIME

	返回相对于始1970 年的 UNIX 纪元（epoch）的时间，使用协调世界时（UTC），	用户空间gettimeofday() 相同。这用于所有需要跨重启保持timestamps，例	inode 时间，但应避免用于内部用途，因为它可能因闰秒更新、NTP 调整或来自用户空间的
	settimeofday() 操作而向后跳变

	 CLOCK_TAI

	类似 ktime_get_real()，但使用国际原子时（TAI）参考而非 UTC，以避免在闰秒更新时
	跳变。这在内核中很少有用

	CLOCK_MONOTONIC_RAW

	类似 ktime_get()，但以与硬件 clocksource 相同的速率运行，不做（NTP）时钟漂	调整。在内核中也很少需要
### 纳秒、timespec64 和秒输出


对于上述所有接口，都有根据调用者需求以不同格式返回时间的变体：

		u64 ktime_get_boottime_ns( void )
		u64 ktime_get_real_ns( void )
		u64 ktime_get_clocktai_ns( void )
		u64 ktime_get_raw_ns( void )

	与上述普通的 ktime_get 函数相同，但返回相应时间参考下u64 纳秒数，对某	调用者可能更方便
		void ktime_get_boottime_ts64( struct timespec64 * )
		void ktime_get_real_ts64( struct timespec64 * )
		void ktime_get_clocktai_ts64( struct timespec64 * )
		void ktime_get_raw_ts64( struct timespec64 * )

	与上述相同，但以 ‘struct timespec64形式返回时间，拆分为秒和纳秒。这可以	打印时间，或将时间传入期‘timespec‘timeval结构的外部接口时避免一次额	的除法
		time64_t ktime_get_boottime_seconds( void )
		time64_t ktime_get_real_seconds( void )
		time64_t ktime_get_clocktai_seconds( void )
		time64_t ktime_get_raw_seconds( void )

	以标time64_t 形式返回一个粗粒度（coarse-grained）的时间版本。这避免了访问时	硬件，并使用相应参考将秒向下取整到上一个定时器节拍（timer tick）的完整秒数
### 粗粒度与 fast_ns 访问


还有一些用于更专门场景的变体：

		ktime_t ktime_get_coarse_boottime( void )
		ktime_t ktime_get_coarse_real( void )
		ktime_t ktime_get_coarse_clocktai( void )

		u64 ktime_get_coarse_boottime_ns( void )
		u64 ktime_get_coarse_real_ns( void )
		u64 ktime_get_coarse_clocktai_ns( void )

		void ktime_get_coarse_boottime_ts64( struct timespec64 * )
		void ktime_get_coarse_real_ts64( struct timespec64 * )
		void ktime_get_coarse_clocktai_ts64( struct timespec64 * )

	这些比非粗粒度版本更快，但精度较低，对应于用户空间中CLOCK_MONOTONIC_COARSE
	CLOCK_REALTIME_COARSE，以及用户空间中不可用的等效 boottime/tai/raw 时基
	这里返回的时间对应于上一个定时器节拍，在过去可能长达 10ms（对CONFIG_HZ=100），
	与读‘jiffies变量相同。这些仅在对时效性要求高（fast path）且仍期望优于秒	精度、但又无法轻松使‘jiffies的情况下有用，例如用inode 时间戳。跳过硬	时钟访问在现代大多数带有可靠周期计数器的机器上可节省100 CPU 周期，但在带	外部 clocksource 的较旧硬件上最多可达数微秒
		u64 ktime_get_raw_fast_ns( void )
		u64 ktime_get_boot_fast_ns( void )
		u64 ktime_get_tai_fast_ns( void )
		u64 ktime_get_real_fast_ns( void )

	这些变体可以安全地从任何上下文中调用，包括在 timekeeper 更新期间的不可屏蔽中	（NMI）中，以及在我们进入挂起clocksource 断电时。这在一些跟踪或调试代码以及
	机器检查（machine check）报告中很有用，但大多数驱动绝不应调用它们，因为该时	在某些条件下允许跳变
### 已废弃的时间接口


较旧的内核使用了一些其它接口，现在正在逐步淘汰，但可能出现在被移植到这里的三方驱动
中。特别是，所有返‘struct timeval‘struct timespec的接口都已被替换，因32 位体系结构上 tv_sec 成员会在 2038 年溢出。以下是推荐的替换：


	使用 ktime_get() ktime_get_ts64() 代替
		void getnstimeofday( struct timespec * )
		void getnstimeofday64( struct timespec64 * )
		void ktime_get_real_ts( struct timespec * )

	ktime_get_real_ts64() 是直接替换，但考虑使用单调时间（ktime_get_ts64()）和/或基	ktime_t 的接口（ktime_get()/ktime_get_real()）
		struct timespec64 current_kernel_time64( void )
		struct timespec get_monotonic_coarse( void )
		struct timespec64 get_monotonic_coarse64( void )

	这些ktime_get_coarse_real_ts64() ktime_get_coarse_ts64() 替换。然而，许多
	需要粗粒度时间的代码可以改用简单的 ‘jiffies’，而如今一些驱动可能实际上想要更高
	分辨率的访问器
		struct timespec64 getrawmonotonic64( void )
		struct timespec timekeeping_clocktai( void )
		struct timespec64 timekeeping_clocktai64( void )
		struct timespec get_monotonic_boottime( void )
		struct timespec64 get_monotonic_boottime64( void )

	这些ktime_get_raw()/ktime_get_raw_ts64()、ktime_get_clocktai()/
	ktime_get_clocktai_ts64() 以及 ktime_get_boottime()/ktime_get_boottime_ts64() 替换	然而，如果用户并不在意时钟源的具体选择，为了一致性考虑改用 ktime_get()/
	ktime_get_ts64()銆。
