## NT 鍚屾鍘熻椹卞姩


鏈〉璁板綍浜?ntsync 椹卞姩鐨勭敤鎴风┖闂?API銆?
ntsync 鏄竴涓敤浜庢敮鎸佺敤鎴风┖闂?NT 妯℃嫙鍣ㄦā鎷?NT 鍚屾鍘熻鐨勮緟鍔╅┍鍔ㄣ€傚畠涔嬫墍浠?瀛樺湪锛屾槸鍥犱负浣跨敤鐜版湁宸ュ叿鍦ㄧ敤鎴风┖闂磋繘琛屽疄鐜帮紝鏃犳硶鍦ㄦ彁渚涘噯纭涔夌殑鍚屾椂鍖归厤
Windows 鐨勬€ц兘銆傚畠瀹屽叏鐢辫蒋浠跺疄鐜帮紝涓嶉┍鍔ㄤ换浣曠‖浠惰澶囥€?
璇ユ帴鍙ｄ粎浣滀负鍏煎鎬у伐鍏蜂娇鐢紝涓嶅簲琚敤浜庨€氱敤鍚屾銆傚簲褰撴敼鐢ㄩ€氱敤銆佺伒娲荤殑鎺ュ彛锛?渚嬪 futex(2) 涓?poll(2)銆?
## 鍚屾鍘熻


ntsync 椹卞姩鏆撮湶涓夌绫诲瀷鐨勫悓姝ュ師璇細淇″彿閲忥紙semaphore锛夈€佷簰鏂ヤ綋锛坢utex锛夊拰
浜嬩欢锛坋vent锛夈€?
淇″彿閲忔寔鏈変竴涓槗澶辩殑 32 浣嶈鏁板櫒锛屼互鍙婁竴涓〃绀烘渶澶у€肩殑闈欐€?32 浣嶆暣鏁般€傚綋璁℃暟鍣?闈為浂鏃讹紝璁や负瀹冨浜庢湁淇″彿锛坰ignaled锛夌姸鎬侊紙鍗冲彲浠ユ棤绔炰簤鍦拌幏鍙栵紝鎴栧皢鍞ら啋涓€涓?绛夊緟鐨勭嚎绋嬶級銆傚綋鏌愪釜绛夊緟琚弧瓒虫椂锛岃鏁板櫒鍑忎竴銆傚垵濮嬭鏁颁笌鏈€澶ц鏁伴兘鍦ㄤ俊鍙烽噺
琚垱寤烘椂纭畾銆?
浜掓枼浣撴寔鏈変竴涓槗澶辩殑 32 浣嶉€掑綊璁℃暟锛屼互鍙婁竴涓〃绀哄叾鎷ユ湁鑰呯殑鏄撳け 32 浣嶆爣璇嗙銆?褰撳畠鐨勬嫢鏈夎€呬负闆讹紙琛ㄧず鏈鎷ユ湁锛夋椂锛岃涓轰簰鏂ヤ綋澶勪簬鏈変俊鍙风姸鎬併€傚綋鏌愪釜绛夊緟琚?婊¤冻鏃讹紝閫掑綊璁℃暟鍔犱竴锛屽苟涓旀嫢鏈夎€呰璁剧疆涓虹粰瀹氱殑鏍囪瘑绗︺€?
浜掓枼浣撹繕鎸佹湁涓€涓唴閮ㄦ爣蹇楋紝琛ㄧず鍏跺墠涓€涓嫢鏈夎€呮槸鍚﹀凡缁忊€滄浜♀€濓紱杩欐牱鐨勪簰鏂ヤ綋
琚О涓鸿閬楀純锛坅bandoned锛夈€傛嫢鏈夎€呯殑姝讳骸骞朵笉浼氬熀浜庣嚎绋嬫浜¤鑷姩璺熻釜锛岃€屾槸蹇呴』
閫氳繃 `NTSYNC_IOC_MUTEX_KILL` 鏉ュ憡鐭ャ€傝閬楀純鐨勪簰鏂ヤ綋鏈川涓婅瑙嗕负鏈鎷ユ湁銆?
闄や簡闆舵墍浠ｈ〃鐨勨€滄湭鎷ユ湁鈥濊涔変箣澶栵紝ntsync 椹卞姩瀵规嫢鏈夎€呮爣璇嗙鐨勫疄闄呭€煎畬鍏ㄤ笉鍔?瑙ｉ噴銆傚叾棰勬湡鐢ㄩ€旀槸瀛樺偍涓€涓嚎绋嬫爣璇嗙锛涗笉杩囷紝ntsync 椹卞姩瀹為檯涓婂苟涓嶄細鏍￠獙璋冪敤
绾跨▼鎻愪緵鐨勬爣璇嗙鏄惁涓€鑷存垨鍞竴銆?
浜嬩欢绫讳技浜庢渶澶ц鏁颁负涓€鐨勪俊鍙烽噺銆傚畠鎸佹湁涓€涓〃绀哄叾鏄惁鏈変俊鍙风殑鏄撳け甯冨皵鐘舵€併€?浜嬩欢鏈変袱绉嶇被鍨嬶細鑷姩澶嶄綅锛坅uto-reset锛変笌鎵嬪姩澶嶄綅锛坢anual-reset锛夈€傝嚜鍔ㄥ浣?浜嬩欢鍦ㄨ绛夊緟婊¤冻鏃朵細琚В闄や俊鍙风姸鎬侊紙designaled锛夛紱鎵嬪姩澶嶄綅浜嬩欢鍒欎笉浼氳瑙ｉ櫎銆?浜嬩欢绫诲瀷鍦ㄤ簨浠惰鍒涘缓鏃舵寚瀹氥€?
闄ら潪鍙︽湁璇存槑锛岄拡瀵瑰悓涓€瀵硅薄鐨勬墍鏈夋搷浣滈兘鏄師瀛愮殑锛屽苟涓旂浉瀵逛簬璇ュ璞′笂鐨勫叾浠栨搷浣?瀹屽叏鏈夊簭銆?
瀵硅薄鐢辨枃浠惰〃绀恒€傚綋鎸囧悜鏌愪釜瀵硅薄鐨勬墍鏈夋枃浠舵弿杩扮閮借鍏抽棴鏃讹紝璇ュ璞″嵆琚垹闄ゃ€?
## 瀛楃璁惧


ntsync 椹卞姩鍒涘缓涓€涓崟涓€鐨勫瓧绗﹁澶?/dev/ntsync銆傚湪璇ヨ澶囦笂鎵撳紑鐨勬瘡涓枃浠舵弿杩扮
閮戒唬琛ㄤ竴涓敮涓€鐨勫疄渚嬶紝鐢ㄤ簬鏀拺涓€涓嫭绔嬬殑 NT 铏氭嫙鏈恒€傜敱涓€涓?ntsync 瀹炰緥鎵€鍒涘缓
鐨勫璞★紝鍙兘涓庣敱鍚屼竴瀹炰緥鍒涘缓鐨勫叾浠栧璞′竴璧蜂娇鐢ㄣ€?
## ioctl 鍙傝€?

瀵硅璁惧鐨勬墍鏈夋搷浣滈兘閫氳繃 ioctl 瀹屾垚銆傚叡鏈夊洓绉?```
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
鏍规嵁 ioctl 鐨勪笉鍚岋紝缁撴瀯浣撲腑鐨勬垚鍛樺彲鐢ㄤ綔杈撳叆銆佽緭鍑猴紝鎴栧畬鍏ㄤ笉鐢ㄣ€?
璁惧鏂囦欢涓婄殑 ioctl 濡備笅锛?

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

鍚勪釜瀵硅薄涓婄殑 ioctl 濡備笅锛?

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
