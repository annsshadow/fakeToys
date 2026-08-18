## 璁惧椹卞姩璁捐妯″紡


鏈枃妗ｆ弿杩颁簡璁惧椹卞姩涓竴浜涘父瑙佺殑璁捐妯″紡銆傚瓙绯荤粺缁存姢鑰呭緢鍙兘浼氳姹傞┍鍔ㄥ紑鍙戣€呴伒寰繖浜涜璁℃ā寮忋€?

1. 鐘舵€佸鍣紙State Container锛?
2. container_of()

#### 1. 鐘舵€佸鍣?


铏界劧鍐呮牳涓湁灏戞暟璁惧椹卞姩鍋囧畾瀹冧滑鍦ㄦ煇涓郴缁熶笂鍙細琚?probe() 涓€娆★紙鍗曚緥锛夛紝浣嗕範鎯笂搴斿亣瀹氶┍鍔ㄦ墍缁戝畾鐨勮澶囦細鍑虹幇澶氫釜瀹炰緥銆傝繖鎰忓懗鐫€ probe() 鍑芥暟涓庢墍鏈夊洖璋冮兘蹇呴』鏄彲閲嶅叆鐨勩€?

瀹炵幇杩欎竴鐐规渶甯歌鐨勬柟娉曟槸浣跨敤鐘舵€佸鍣ㄨ璁?
```

  struct foo {
      spinlock_t lock; /* 绀轰緥鎴愬憳 */
      (...)
  };

  static int foo_probe(...)
  {
      struct foo *foo;

      foo = devm_kzalloc(dev, sizeof(*foo), GFP_KERNEL);
      if (!foo)
          return -ENOMEM;
      spin_lock_init(&foo->lock);
      (...)
  }

```

姣忔璋冪敤 probe() 鏃讹紝杩欎細鍦ㄥ唴瀛樹腑鍒涘缓涓€涓?struct foo 鐨勫疄渚嬨€傝繖灏辨槸璇ヨ澶囬┍鍔ㄥ疄渚嬬殑鐘舵€佸鍣ㄣ€傚綋鐒讹紝涔嬪悗鏈夊繀瑕佸缁堝皢杩欎釜鐘舵€佸疄渚嬩紶閫掔粰鎵€鏈夐渶瑕佽闂鐘舵€佸強鍏舵垚鍛樼殑鍑芥暟銆?

渚嬪锛屽鏋滈┍鍔ㄦ鍦ㄦ敞鍐屼竴涓腑鏂鐞嗗嚱鏁帮紝浣犱細
```

  static irqreturn_t foo_handler(int irq, void *arg)
  {
      struct foo *foo = arg;
      (...)
  }

  static int foo_probe(...)
  {
      struct foo *foo;

      (...)
      ret = request_irq(irq, foo_handler, 0, "foo", foo);
  }

```

杩欐牱锛屽湪涓柇澶勭悊鍑芥暟涓綘鎬昏兘鍙栧洖鎸囧悜姝ｇ‘ foo 瀹炰緥鐨勬寚閽堛€?

#### 2. container_of()


```

  struct foo {
      spinlock_t lock;
      struct workqueue_struct *wq;
      struct work_struct offload;
      (...)
  };

  static void foo_work(struct work_struct *work)
  {
      struct foo *foo = container_of(work, struct foo, offload);

      (...)
  }

  static irqreturn_t foo_handler(int irq, void *arg)
  {
      struct foo *foo = arg;

      queue_work(foo->wq, &foo->offload);
      (...)
  }

  static int foo_probe(...)
  {
      struct foo *foo;

      foo->wq = create_singlethread_workqueue("foo-wq");
      INIT_WORK(&foo->offload, foo_work);
      (...)
  }

```

瀵逛簬 hrtimer 鎴栫被浼肩殑銆佸湪鍥炶皟涓繑鍥炲崟涓弬鏁帮紙鎸囧悜缁撴瀯浣撴垚鍛樼殑鎸囬拡锛夌殑鎯呭喌锛岃璁℃ā寮忔槸鐩稿悓鐨勩€?

container_of() 鏄?<linux/container_of.h> 涓畾涔夌殑瀹忋€?

container_of() 鎵€鍋氱殑浜嬫儏鏄紝鍒╃敤鏍囧噯 C 鐨?offsetof() 瀹忛€氳繃绠€鍗曠殑鍑忔硶锛屼粠涓€涓寚鍚戞垚鍛樼殑鎸囬拡鑾峰緱鎸囧悜鍖呭惈瀹冪殑缁撴瀯浣撶殑鎸囬拡锛屼粠鑰屽疄鐜扮被浼间簬闈㈠悜瀵硅薄鐨勮涓恒€傛敞鎰忥紝琚寘鍚殑鎴愬憳涓嶈兘鏄紙鎸囧悜鍙︿竴缁撴瀯鐨勶級鎸囬拡锛岃€屽繀椤绘槸瀹為檯鐨勬垚鍛樻墠鑳藉伐浣溿€?

鎴戜滑鍙互鐪嬪埌锛岄€氳繃杩欑鏂瑰紡鎴戜滑閬垮厤浜嗘寔鏈夋寚鍚?struct foo * 瀹炰緥鐨勫叏灞€鎸囬拡锛屽悓鏃跺皢浼犻€掔粰 work 鍑芥暟鐨勫弬鏁版暟閲忎繚鎸佸湪鍗曚釜鎸囬拡銆?
