
## Lockdep-RCU Splat


Lockdep-RCU 浜?2010 骞村垵鍔犲叆 Linux 鍐呮牳锛坔ttp://lwn.net/Articles/371986/锛夈€傝鏈哄埗妫€鏌ヤ竴浜涘父瑙佺殑
RCU API 璇敤鎯呭喌锛屾渶鍏稿瀷鐨勬槸鍦ㄦ湭鍔犻€傚綋淇濇姢鐨勬儏鍐典笅浣跨敤 rcu_dereference()
绯诲垪涔嬩竴鏉ヨ闂彈 RCU 淇濇姢鐨勬寚閽堛€傚綋妫€娴嬪埌姝ょ被璇敤鏃讹紝浼氬彂鍑轰竴鏉?lockdep-RCU splat銆?

lockdep-RCU splat 閫氬父鐨勬垚鍥犳槸鏈変汉璁块棶鍙?RCU 淇濇姢鐨勬暟鎹粨鏋勬椂锛屾棦娌℃湁
锛?锛夊浜庢纭被鍨嬬殑 RCU 璇讳晶涓寸晫鍖猴紝涔熸病鏈夛紙2锛夋寔鏈夋纭殑鏇存柊渚ч攣銆?
鍥犳璇ラ棶棰樺彲鑳藉緢涓ラ噸锛氬畠鍙兘瀵艰嚧闅忔満鐨勫唴瀛樿鍐欐垨鏇寸碂鐨勬儏鍐点€傚綋鐒朵篃鍙兘瀛樺湪
璇姤锛屾瘯绔熻繖灏辨槸鐜板疄涓栫晫銆?

璁╂垜浠湅涓€涓潵鑷?3.0-rc5 鐨?RCU lockdep splat 绀轰緥锛屽叾涓竴涓?
```

    =============================
    WARNING: suspicious RCU usage
    -----------------------------
    block/cfq-iosched.c:2776 suspicious rcu_dereference_protected() usage!

```
```

    rcu_scheduler_active = 1, debug_locks = 0
    3 locks held by scsi_scan_6/1552:
    #0:  (&shost->scan_mutex){+.+.}, at: [<ffffffff8145efca>]
    scsi_scan_host_selected+0x5a/0x150
    #1:  (&eq->sysfs_lock){+.+.}, at: [<ffffffff812a5032>]
    elevator_exit+0x22/0x60
    #2:  (&(&q->__queue_lock)->rlock){-.-.}, at: [<ffffffff812b6233>]
    cfq_exit_queue+0x43/0x190

    stack backtrace:
    Pid: 1552, comm: scsi_scan_6 Not tainted 3.0.0-rc5 #17
    Call Trace:
    [<ffffffff810abb9b>] lockdep_rcu_dereference+0xbb/0xc0
    [<ffffffff812b6139>] __cfq_exit_single_io_context+0xe9/0x120
    [<ffffffff812b626c>] cfq_exit_queue+0x7c/0x190
    [<ffffffff812a5046>] elevator_exit+0x36/0x60
    [<ffffffff812a802a>] blk_cleanup_queue+0x4a/0x60
    [<ffffffff8145cc09>] scsi_free_queue+0x9/0x10
    [<ffffffff81460944>] __scsi_remove_device+0x84/0xd0
    [<ffffffff8145dca3>] scsi_probe_and_add_lun+0x353/0xb10
    [<ffffffff817da069>] ? error_exit+0x29/0xb0
    [<ffffffff817d98ed>] ? _raw_spin_unlock_irqrestore+0x3d/0x80
    [<ffffffff8145e722>] __scsi_scan_target+0x112/0x680
    [<ffffffff812c690d>] ? trace_hardirqs_off_thunk+0x3a/0x3c
    [<ffffffff817da069>] ? error_exit+0x29/0xb0
    [<ffffffff812bcc60>] ? kobject_del+0x40/0x40
    [<ffffffff8145ed16>] scsi_scan_channel+0x86/0xb0
    [<ffffffff8145f0b0>] scsi_scan_host_selected+0x140/0x150
    [<ffffffff8145f149>] do_scsi_scan_host+0x89/0x90
    [<ffffffff8145f170>] do_scan_async+0x20/0x160
    [<ffffffff8145f150>] ? do_scsi_scan_host+0x90/0x90
    [<ffffffff810975b6>] kthread+0xa6/0xb0
    [<ffffffff817db154>] kernel_thread_helper+0x4/0x10
    [<ffffffff81066430>] ? finish_task_switch+0x80/0x110
    [<ffffffff817d9c04>] ? retint_restore_args+0xe/0xe
    [<ffffffff81097510>] ? __kthread_init_worker+0x70/0x70
    [<ffffffff817db150>] ? gs_change+0xb/0xb

```
```

	if (rcu_dereference(ioc->ioc_data) == cic) {

```
璇ュ舰寮忚〃鏄庡畠蹇呴』澶勪簬涓€涓櫘閫氱殑鍘熺敓 RCU 璇讳晶涓寸晫鍖轰腑锛屼絾涓婇潰鐨勨€滃叾浠栦俊鎭€濆垪琛?
鏄剧ず鎯呭喌骞堕潪濡傛銆傜浉鍙嶏紝鎴戜滑鎸佹湁涓夋妸閿侊紝鍏朵腑涓€鎶婂彲鑳戒笌 RCU 鐩稿叧銆?
涔熻閭ｆ妸閿佺‘瀹炰繚鎶や簡杩欎釜寮曠敤銆傚鏋滄槸杩欐牱锛屼慨澶嶆柟娉曟槸鍛婄煡 RCU锛屼緥濡傚皢
__cfq_exit_single_io_context() 鏀逛负浠?cfq_exit_queue() 鎺ユ敹 struct request_queue 鐨勨€渜鈥濅綔涓哄弬鏁帮紝
```

	if (rcu_dereference_protected(ioc->ioc_data,
				      lockdep_is_held(&q->queue_lock)) == cic) {

```
缁忚繃姝や慨鏀癸紝鑻ヨ繖娈典唬鐮佹槸浠?RCU 璇讳晶涓寸晫鍖哄唴璋冪敤锛屾垨鏄寔鏈変簡 ->queue_lock锛?
灏变笉浼氬啀鍙戝嚭 lockdep-RCU splat銆傚挨鍏舵槸锛岀敱浜?->queue_lock 琚寔鏈夛紙瑙佷笂闈㈠垪琛ㄤ腑鐨?#2锛夛紝
杩欏師鏈氨浼氭姂鍒朵笂杩?lockdep-RCU splat銆?

鍙︿竴鏂归潰锛屼篃璁告垜浠‘瀹為渶瑕佷竴涓?RCU 璇讳晶涓寸晫鍖恒€傚湪杩欑鎯呭喌涓嬶紝涓寸晫鍖哄繀椤昏鐩?
rcu_dereference() 杩斿洖鍊肩殑鏁翠釜浣跨敤杩囩▼锛屾垨鑷冲皯鎸佺画鍒版煇涓紩鐢ㄨ鏁拌閫掑涔嬬被鐨勬搷浣滃畬鎴愩€?
澶勭悊姝ら棶棰樼殑涓€绉嶆柟寮忔槸
```

	rcu_read_lock();
	if (rcu_dereference(ioc->ioc_data) == cic) {
		spin_lock(&ioc->lock);
		rcu_assign_pointer(ioc->ioc_data, NULL);
		spin_unlock(&ioc->lock);
	}
	rcu_read_unlock();

```
缁忚繃姝や慨鏀癸紝rcu_dereference() 濮嬬粓浣嶄簬 RCU 璇讳晶涓寸晫鍖轰箣鍐咃紝杩欏悓鏍蜂細鎶戝埗涓婅堪
lockdep-RCU splat銆?

浣嗗湪杩欎竴鐗瑰畾鎯呭喌涓嬶紝鎴戜滑骞舵病鏈夌湡姝ｈВ寮曠敤 rcu_dereference() 杩斿洖鐨勬寚閽堛€?
鐩稿弽锛岃鎸囬拡鍙槸涓?cic 鎸囬拡杩涜姣旇緝锛岃繖鎰忓懗鐫€ rcu_dereference() 鍙互鏇挎崲涓?
```

	if (rcu_access_pointer(ioc->ioc_data) == cic) {

```
鐢变簬 rcu_access_pointer() 鍙互鍦ㄦ棤淇濇姢鐨勬儏鍐典笅鍚堟硶璋冪敤锛屾淇敼鍚屾牱浼氭姂鍒朵笂杩?
lockdep-RCU splat銆?

