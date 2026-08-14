## NT 同步原语驱动


本页记录了 ntsync 驱动的用户空间 API。

ntsync 是一个用于支持用户空间 NT 模拟器模拟 NT 同步原语的辅助驱动。它之所以
存在，是因为使用现有工具在用户空间进行实现，无法在提供准确语义的同时匹配
Windows 的性能。它完全由软件实现，不驱动任何硬件设备。

该接口仅作为兼容性工具使用，不应被用于通用同步。应当改用通用、灵活的接口，
例如 futex(2) 与 poll(2)。

## 同步原语


ntsync 驱动暴露三种类型的同步原语：信号量（semaphore）、互斥体（mutex）和
事件（event）。

信号量持有一个易失的 32 位计数器，以及一个表示最大值的静态 32 位整数。当计数器
非零时，认为它处于有信号（signaled）状态（即可以无竞争地获取，或将唤醒一个
等待的线程）。当某个等待被满足时，计数器减一。初始计数与最大计数都在信号量
被创建时确定。

互斥体持有一个易失的 32 位递归计数，以及一个表示其拥有者的易失 32 位标识符。
当它的拥有者为零（表示未被拥有）时，认为互斥体处于有信号状态。当某个等待被
满足时，递归计数加一，并且拥有者被设置为给定的标识符。

互斥体还持有一个内部标志，表示其前一个拥有者是否已经“死亡”；这样的互斥体
被称为被遗弃（abandoned）。拥有者的死亡并不会基于线程死亡被自动跟踪，而是必须
通过 `NTSYNC_IOC_MUTEX_KILL` 来告知。被遗弃的互斥体本质上被视为未被拥有。

除了零所代表的“未拥有”语义之外，ntsync 驱动对拥有者标识符的实际值完全不加
解释。其预期用途是存储一个线程标识符；不过，ntsync 驱动实际上并不会校验调用
线程提供的标识符是否一致或唯一。

事件类似于最大计数为一的信号量。它持有一个表示其是否有信号的易失布尔状态。
事件有两种类型：自动复位（auto-reset）与手动复位（manual-reset）。自动复位
事件在被等待满足时会被解除信号状态（designaled）；手动复位事件则不会被解除。
事件类型在事件被创建时指定。

除非另有说明，针对同一对象的所有操作都是原子的，并且相对于该对象上的其他操作
完全有序。

对象由文件表示。当指向某个对象的所有文件描述符都被关闭时，该对象即被删除。

## 字符设备


ntsync 驱动创建一个单一的字符设备 /dev/ntsync。在该设备上打开的每个文件描述符
都代表一个唯一的实例，用于支撑一个独立的 NT 虚拟机。由一个 ntsync 实例所创建
的对象，只能与由同一实例创建的其他对象一起使用。

## ioctl 参考


对该设备的所有操作都通过 ioctl 完成。共有四种
```
   struct ntsync_sem_args {
   	__u32 count;
   	__u32 max;
   };

   struct ntsync_mutex_args {
   	__u32 owner;
   	__u32 count;
   };

   struct ntsync_event_args {
   	__u32 signaled;
   	__u32 manual;
   };

   struct ntsync_wait_args {
   	__u64 timeout;
   	__u64 objs;
   	__u32 count;
   	__u32 owner;
   	__u32 index;
   	__u32 alert;
   	__u32 flags;
   	__u32 pad;
   };

```
根据 ioctl 的不同，结构体中的成员可用作输入、输出，或完全不用。

设备文件上的 ioctl 如下：


  Create a semaphore object. Takes a pointer to struct
  `ntsync_sem_args`, which is used as follows:

```
     * - ``count``
       - Initial count of the semaphore.
     * - ``max``
       - Maximum count of the semaphore.

  Fails with ``EINVAL`` if ``count`` is greater than ``max``.
  On success, returns a file descriptor the created semaphore.

```

  Create a mutex object. Takes a pointer to struct
  `ntsync_mutex_args`, which is used as follows:

```
     * - ``count``
       - Initial recursion count of the mutex.
     * - ``owner``
       - Initial owner of the mutex.

  If ``owner`` is nonzero and ``count`` is zero, or if ``owner`` is
  zero and ``count`` is nonzero, the function fails with ``EINVAL``.
  On success, returns a file descriptor the created mutex.

```

  Create an event object. Takes a pointer to struct
  `ntsync_event_args`, which is used as follows:

```
     * - ``signaled``
       - If nonzero, the event is initially signaled, otherwise
         nonsignaled.
     * - ``manual``
       - If nonzero, the event is a manual-reset event, otherwise
         auto-reset.

  On success, returns a file descriptor the created event.

```

各个对象上的 ioctl 如下：


  Post to a semaphore object. Takes a pointer to a 32-bit integer,
  which on input holds the count to be added to the semaphore, and on
  output contains its previous count.

  If adding to the semaphore's current count would raise the latter
  past the semaphore's maximum count, the ioctl fails with
  `EOVERFLOW` and the semaphore is not affected. If raising the
  semaphore's count causes it to become signaled, eligible threads
  waiting on this semaphore will be woken and the semaphore's count
  decremented appropriately.


  Release a mutex object. Takes a pointer to struct
  `ntsync_mutex_args`, which is used as follows:

```
     * - ``owner``
       - Specifies the owner trying to release this mutex.
     * - ``count``
       - On output, contains the previous recursion count.

  If ``owner`` is zero, the ioctl fails with ``EINVAL``. If ``owner``
  is not the current owner of the mutex, the ioctl fails with
  ``EPERM``.

  The mutex's count will be decremented by one. If decrementing the
  mutex's count causes it to become zero, the mutex is marked as
  unowned and signaled, and eligible threads waiting on it will be
  woken as appropriate.

```

  Signal an event object. Takes a pointer to a 32-bit integer, which on
  output contains the previous state of the event.

  Eligible threads will be woken, and auto-reset events will be
  designaled appropriately.


  Designal an event object. Takes a pointer to a 32-bit integer, which
  on output contains the previous state of the event.


  Wake threads waiting on an event object while leaving it in an
  unsignaled state. Takes a pointer to a 32-bit integer, which on
  output contains the previous state of the event.

  A pulse operation can be thought of as a set followed by a reset,
  performed as a single atomic operation. If two threads are waiting on
  an auto-reset event which is pulsed, only one will be woken. If two
  threads are waiting a manual-reset event which is pulsed, both will
  be woken. However, in both cases, the event will be unsignaled
  afterwards, and a simultaneous read operation will always report the
  event as unsignaled.


  Read the current state of a semaphore object. Takes a pointer to
  struct `ntsync_sem_args`, which is used as follows:

```
     * - ``count``
       - On output, contains the current count of the semaphore.
     * - ``max``
       - On output, contains the maximum count of the semaphore.

```

  Read the current state of a mutex object. Takes a pointer to struct
  `ntsync_mutex_args`, which is used as follows:

```
     * - ``owner``
       - On output, contains the current owner of the mutex, or zero
         if the mutex is not currently owned.
     * - ``count``
       - On output, contains the current recursion count of the mutex.

  If the mutex is marked as abandoned, the function fails with
  ``EOWNERDEAD``. In this case, ``count`` and ``owner`` are set to
  zero.

```

  Read the current state of an event object. Takes a pointer to struct
  `ntsync_event_args`, which is used as follows:

```
     * - ``signaled``
       - On output, contains the current state of the event.
     * - ``manual``
       - On output, contains 1 if the event is a manual-reset event,
         and 0 otherwise.

```

  Mark a mutex as unowned and abandoned if it is owned by the given
  owner. Takes an input-only pointer to a 32-bit integer denoting the
  owner. If the owner is zero, the ioctl fails with `EINVAL`. If the
  owner does not own the mutex, the function fails with `EPERM`.

  Eligible threads waiting on the mutex will be woken as appropriate
  (and such waits will fail with `EOWNERDEAD`, as described below).


  Poll on any of a list of objects, atomically acquiring at most one.
  Takes a pointer to struct `ntsync_wait_args`, which is
  used as follows:

```
     * - ``timeout``
       - Absolute timeout in nanoseconds. If ``NTSYNC_WAIT_REALTIME``
         is set, the timeout is measured against the REALTIME clock;
         otherwise it is measured against the MONOTONIC clock. If the
         timeout is equal to or earlier than the current time, the
         function returns immediately without sleeping. If ``timeout``
         is U64_MAX, the function will sleep until an object is
         signaled, and will not fail with ``ETIMEDOUT``.
     * - ``objs``
       - Pointer to an array of ``count`` file descriptors
         (specified as an integer so that the structure has the same
         size regardless of architecture). If any object is
         invalid, the function fails with ``EINVAL``.
     * - ``count``
       - Number of objects specified in the ``objs`` array.
         If greater than ``NTSYNC_MAX_WAIT_COUNT``, the function fails
         with ``EINVAL``.
     * - ``owner``
       - Mutex owner identifier. If any object in ``objs`` is a mutex,
         the ioctl will attempt to acquire that mutex on behalf of
         ``owner``. If ``owner`` is zero, the ioctl fails with
         ``EINVAL``.
     * - ``index``
       - On success, contains the index (into ``objs``) of the object
         which was signaled. If ``alert`` was signaled instead,
         this contains ``count``.
     * - ``alert``
       - Optional event object file descriptor. If nonzero, this
         specifies an "alert" event object which, if signaled, will
         terminate the wait. If nonzero, the identifier must point to a
         valid event.
     * - ``flags``
       - Zero or more flags. Currently the only flag is
         ``NTSYNC_WAIT_REALTIME``, which causes the timeout to be
         measured against the REALTIME clock instead of MONOTONIC.
     * - ``pad``
       - Unused, must be set to zero.

```

  This function attempts to acquire one of the given objects. If unable
  to do so, it sleeps until an object becomes signaled, subsequently
  acquiring it, or the timeout expires. In the latter case the ioctl
  fails with ``ETIMEDOUT``. The function only acquires one object, even
  if multiple objects are signaled.

  A semaphore is considered to be signaled if its count is nonzero, and
  is acquired by decrementing its count by one. A mutex is considered
  to be signaled if it is unowned or if its owner matches the ``owner``
  argument, and is acquired by incrementing its recursion count by one
  and setting its owner to the ``owner`` argument. An auto-reset event
  is acquired by designaling it; a manual-reset event is not affected
  by acquisition.

  Acquisition is atomic and totally ordered with respect to other
  operations on the same object. If two wait operations (with different
  ``owner`` identifiers) are queued on the same mutex, only one is
  signaled. If two wait operations are queued on the same semaphore,
  and a value of one is posted to it, only one is signaled.

  If an abandoned mutex is acquired, the ioctl fails with
  ``EOWNERDEAD``. Although this is a failure return, the function may
  otherwise be considered successful. The mutex is marked as owned by
  the given owner (with a recursion count of 1) and as no longer
  abandoned, and ``index`` is still set to the index of the mutex.

  The ``alert`` argument is an "extra" event which can terminate the
  wait, independently of all other objects.

  It is valid to pass the same object more than once, including by
  passing the same event in the ``objs`` array and in ``alert``. If a
  wakeup occurs due to that object being signaled, ``index`` is set to
  the lowest index corresponding to that object.

  The function may fail with ``EINTR`` if a signal is received.


  Poll on a list of objects, atomically acquiring all of them. Takes a pointer to struct `ntsync_wait_args`, which is
  used identically to `NTSYNC_IOC_WAIT_ANY`, except that `index` is always filled with zero on success if not woken via alert.

  This function attempts to simultaneously acquire all of the given
  objects. If unable to do so, it sleeps until all objects become
  simultaneously signaled, subsequently acquiring them, or the timeout
  expires. In the latter case the ioctl fails with `ETIMEDOUT` and no
  objects are modified.

  Objects may become signaled and subsequently designaled (through
  acquisition by other threads) while this thread is sleeping. Only
  once all objects are simultaneously signaled does the ioctl acquire
  them and return. The entire acquisition is atomic and totally ordered
  with respect to other operations on any of the given objects.

  If an abandoned mutex is acquired, the ioctl fails with
  `EOWNERDEAD`. Similarly to `NTSYNC_IOC_WAIT_ANY`, all objects are
  nevertheless marked as acquired. Note that if multiple mutex objects
  are specified, there is no way to know which were marked as
  abandoned.

  As with "any" waits, the `alert` argument is an "extra" event which
  can terminate the wait. Critically, however, an "all" wait will
  succeed if all members in `objs` are signaled, **or** if `alert` is
  signaled. In the latter case `index` will be set to `count`. As
  with "any" waits, if both conditions are filled, the former takes
  priority, and objects in `objs` will be acquired.

  Unlike `NTSYNC_IOC_WAIT_ANY`, it is not valid to pass the same
  object more than once, nor is it valid to pass the same object in
  the `objs` and in `alert`. If this is attempted, the function fails
  with `EINVAL`.
