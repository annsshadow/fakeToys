## PARPORT 鎺ュ彛鏂囨。


:Time-stamp: <2000-02-24 13:30:20 twaugh>

杩欓噷鎻忚堪浠ヤ笅鍑芥暟锛?
```
  parport_register_driver
  parport_unregister_driver
  parport_enumerate
  parport_register_device
  parport_unregister_device
  parport_claim
  parport_claim_or_block
  parport_release
  parport_yield
  parport_yield_blocking
  parport_wait_peripheral
  parport_poll_peripheral
  parport_wait_event
  parport_negotiate
  parport_read
  parport_write
  parport_open
  parport_close
  parport_device_id
  parport_device_coords
  parport_find_class
  parport_find_device
  parport_set_timeout
```
绔彛鍑芥暟锛堝彲琚簳灞傞┍鍔ㄨ鐩栵級锛?
```
    port->ops->read_data
    port->ops->write_data
    port->ops->read_status
    port->ops->read_control
    port->ops->write_control
    port->ops->frob_control
    port->ops->enable_irq
    port->ops->disable_irq
    port->ops->data_forward
    port->ops->data_reverse

  EPP::

    port->ops->epp_write_data
    port->ops->epp_read_data
    port->ops->epp_write_addr
    port->ops->epp_read_addr

  ECP::

    port->ops->ecp_write_data
    port->ops->ecp_read_data
    port->ops->ecp_write_addr

  Other::

    port->ops->nibble_read_data
    port->ops->byte_read_data
    port->ops->compat_write_data
```
parport 瀛愮郴缁熷寘鍚?`parport`锛堟牳蹇冪殑绔彛鍏变韩浠ｇ爜锛夛紝浠ュ強鍚勭鍚勬牱鐨勫簳灞傞┍鍔紝瀹冧滑
瀹為檯鎵ц绔彛璁块棶銆傛瘡涓簳灞傞┍鍔ㄥ鐞嗕竴绉嶇壒瀹氶鏍肩殑绔彛锛圥C銆丄miga 绛夛級銆?
parport 闈㈠悜璁惧椹卞姩浣滆€呯殑鎺ュ彛鍙互鍒嗕负鍏ㄥ眬鍑芥暟鍜岀鍙ｅ嚱鏁般€?
鍏ㄥ眬鍑芥暟涓昏鐢ㄤ簬璁惧椹卞姩涓?parport 瀛愮郴缁熶箣闂寸殑閫氫俊锛氳幏鍙栧彲鐢ㄧ鍙ｅ垪琛ㄣ€佷负鐙崰浣跨敤
澹版槑涓€涓鍙ｇ瓑绛夈€傚畠浠繕鍖呮嫭鐢ㄤ簬鎵ц鏍囧噯鎿嶄綔鐨?`generic` 鍑芥暟锛岃繖浜涙搷浣滃彲鍦ㄤ换浣?鏀寔 IEEE 1284 鐨勬灦鏋勪笂宸ヤ綔銆?
绔彛鍑芥暟鐢卞簳灞傞┍鍔ㄦ彁渚涳紝灏界鏍稿績 parport 妯″潡涓烘煇浜涗緥绋嬫彁渚涗簡閫氱敤鐨?`defaults`銆?绔彛鍑芥暟鍙互鍒嗘垚涓夌粍锛歋PP銆丒PP 鍜?ECP銆?
SPP锛堟爣鍑嗗苟琛岀鍙ｏ級鍑芥暟淇敼鎵€璋撶殑 `SPP` 瀵勫瓨鍣細data銆乻tatus 鍜?control銆傜‖浠舵湭蹇?鐪熺殑鎷ユ湁瀹屽叏閭ｆ牱鐨勫瘎瀛樺櫒锛屼絾 PC 鏈夛紝骞朵笖杩欎釜鎺ュ彛鏄豢鐓у父瑙佺殑 PC 瀹炵幇寤烘ā鐨勩€傚叾浠?搴曞眰椹卞姩鍙兘鑳藉妯℃嫙鍏朵腑澶ч儴鍒嗗姛鑳姐€?
EPP锛堝寮哄苟琛岀鍙ｏ級鍑芥暟鐢ㄤ簬浠?IEEE 1284 EPP 妯″紡杩涜璇诲啓锛岃€?ECP锛堟墿灞曡兘鍔涚鍙ｏ級鍑芥暟
鐢ㄤ簬 IEEE 1284 ECP 妯″紡銆傦紙閭?BECP 鍛紵鏈変汉鍏冲績鍚楋紵锛?
鐢ㄤ簬 EPP 鍜?鎴?ECP 浼犺緭鐨勭‖浠惰緟鍔╁彲鑳藉彲鐢紝涔熷彲鑳戒笉鍙敤锛涘鏋滃彲鐢紝鍙兘浣跨敤锛屼篃
鍙兘涓嶄娇鐢ㄣ€傚鏋滄病鏈変娇鐢ㄧ‖浠讹紝浼犺緭灏嗙敱杞欢椹卞姩銆備负浜嗗簲瀵归偅浜涘彧鏄媺寮烘敮鎸?IEEE 1284
鐨勫璁撅紝鎻愪緵浜嗕竴涓簳灞傞┍鍔ㄧ壒瀹氱殑鍑芥暟锛岀敤浜庤皟鏁?鈥渇udge factors锛堝井璋冨洜瀛愶級鈥濄€?
## 鍏ㄥ眬鍑芥暟


### parport_register_driver - 鍚?parport 娉ㄥ唽涓€涓澶囬┍鍔?

##### 姒傝


```
	#include <linux/parport.h>

	struct parport_driver {
		const char *name;
		void (*attach) (struct parport *);
		void (*detach) (struct parport *);
		struct parport_driver *next;
	};
	int parport_register_driver (struct parport_driver *driver);
```
##### 鎻忚堪


涓轰簡鑳藉湪骞惰绔彛琚娴嬪埌鏃舵敹鍒伴€氱煡锛屽簲璇ヨ皟鐢?parport_register_driver銆備綘鐨勯┍鍔ㄥ皢
绔嬪嵆鏀跺埌鎵€鏈夊凡缁忚妫€娴嬪埌鐨勭鍙ｇ殑閫氱煡锛屽苟涓斿湪搴曞眰椹卞姩琚姞杞芥椂锛屾敹鍒版瘡涓柊绔彛鐨?閫氱煡銆?
涓€涓?`struct parport_driver` 鍖呭惈浣犵殑椹卞姩鐨勬枃鏈悕绉般€佷竴涓寚鍚戠敤浜庡鐞嗘柊绔彛鐨勫嚱鏁扮殑
鎸囬拡锛屼互鍙婁竴涓寚鍚戠敤浜庡鐞嗗洜搴曞眰椹卞姩鍗歌浇鑰屾秷澶辩殑绔彛鐨勫嚱鏁扮殑鎸囬拡銆傜鍙ｅ彧鏈夊湪鏈
浣跨敤鏃讹紙鍗充笂闈㈡病鏈夋敞鍐屼换浣曡澶囷級鎵嶄細琚垎绂汇€?
浼犵粰 `struct parport *` 鍙傛暟鐨勫彲瑙侀儴鍒嗗涓嬶細
```
	struct parport
	{
		struct parport *next; /* next parport in list */
		const char *name;     /* port's name */
		unsigned int modes;   /* bitfield of hardware modes */
		struct parport_device_info probe_info;
				/* IEEE1284 info */
		int number;           /* parport index */
		struct parport_operations *ops;
		...
	};
```
缁撴瀯涓繕鏈夊叾浠栨垚鍛橈紝浣嗕笉搴旇鍘昏Е纰板畠浠€?
`modes` 鎴愬憳鎬荤粨浜嗗簳灞傜‖浠剁殑鑳藉姏銆傚畠鐢卞彲浠ヤ綅鎴栫粍鍚堝湪涓€璧风殑鑻ュ共鏍囧織缁勬垚锛?
  ============================= ===============================================
  PARPORT_MODE_PCSPP		IBM PC 瀵勫瓨鍣ㄥ彲鐢紝鍗充綔鐢ㄤ簬 data銆乧ontrol 鍜?				status 瀵勫瓨鍣ㄧ殑鍑芥暟鍙兘鏄湪鐩存帴鍐欏叆纭欢銆?  PARPORT_MODE_TRISTATE		鏁版嵁椹卞姩鍣ㄥ彲浠ヨ鍏抽棴銆傝繖鍏佽鏁版嵁绾胯鐢ㄤ簬
				鍙嶅悜锛堝璁惧埌涓绘満锛変紶杈撱€?  PARPORT_MODE_COMPAT		纭欢鍙互杈呭姪鍏煎妯″紡锛堟墦鍗版満锛変紶杈擄紝鍗?				compat_write_block銆?  PARPORT_MODE_EPP		纭欢鍙互杈呭姪 EPP 浼犺緭銆?  PARPORT_MODE_ECP		纭欢鍙互杈呭姪 ECP 浼犺緭銆?  PARPORT_MODE_DMA		纭欢鍙互浣跨敤 DMA锛屽洜姝や綘鍙兘鎯宠鎶?ISA 鍙?DMA
				鐨勫唴瀛橈紙鍗充娇鐢?kmalloc 鐨?GFP_DMA 鏍囧織鍒嗛厤鐨勫唴瀛橈級
				浼犵粰搴曞眰椹卞姩锛屼互鍒╃敤瀹冦€?  ============================= ===============================================

`modes` 涓彲鑳借繕鏈夊叾浠栨爣蹇椼€?
`modes` 鐨勫唴瀹逛粎渚涘弬鑰冦€備緥濡傦紝濡傛灉纭欢鑳藉浣跨敤 DMA锛屽苟涓?PARPORT_MODE_DMA 鍦?`modes`
涓紝杩欏苟涓嶅繀鐒舵剰鍛崇潃鍦ㄥ彲鑳芥椂浼氭€绘槸浣跨敤 DMA銆傜被浼煎湴锛岃兘澶熻緟鍔?ECP 浼犺緭鐨勭‖浠朵篃涓嶅繀鐒?浼氳浣跨敤銆?
##### 杩斿洖鍊?

鎴愬姛鏃朵负闆讹紝鍚﹀垯涓轰竴涓敊璇爜銆?
##### 閿欒


鏃犮€傦紙瀹冨彲鑳藉け璐ュ悧锛熶负浠€涔堣杩斿洖 int锛燂級

##### 绀轰緥


```
	static void lp_attach (struct parport *port)
	{
		...
		private = kmalloc (...);
		dev[count++] = parport_register_device (...);
		...
	}

	static void lp_detach (struct parport *port)
	{
		...
	}

	static struct parport_driver lp_driver = {
		"lp",
		lp_attach,
		lp_detach,
		NULL /* always put NULL here */
	};

	int lp_init (void)
	{
		...
		if (parport_register_driver (&lp_driver)) {
			/* Failed; nothing we can do. */
			return -EIO;
		}
		...
	}
```
##### 鍙﹁鍙傞槄


parport_unregister_driver, parport_register_device, parport_enumerate



### parport_unregister_driver - 鍛婅瘔 parport 蹇樻帀杩欎釜椹卞姩


##### 姒傝


```
	#include <linux/parport.h>

	struct parport_driver {
		const char *name;
		void (*attach) (struct parport *);
		void (*detach) (struct parport *);
		struct parport_driver *next;
	};
	void parport_unregister_driver (struct parport_driver *driver);
```
##### 鎻忚堪


杩欏憡璇?parport 涓嶈鍐嶅悜璁惧椹卞姩閫氱煡鏂扮鍙ｆ垨绔彛娑堝け銆傚睘浜庤椹卞姩鐨勫凡娉ㄥ唽璁惧涓嶄細琚?娉ㄩ攢锛氬繀椤诲姣忎釜璁惧浣跨敤 parport_unregister_device銆?
##### 绀轰緥


```
	void cleanup_module (void)
	{
		...
		/* Stop notifications. */
		parport_unregister_driver (&lp_driver);

		/* Unregister devices. */
		for (i = 0; i < NUM_DEVS; i++)
			parport_unregister_device (dev[i]);
		...
	}
```
##### 鍙﹁鍙傞槄


parport_register_driver, parport_enumerate



### parport_enumerate - 鑾峰彇骞惰绔彛鍒楄〃锛堝凡搴熷純锛?

##### 姒傝


```
	#include <linux/parport.h>

	struct parport *parport_enumerate (void);
```
##### 鎻忚堪


鑾峰彇鏈満鍣ㄦ湁鏁堝苟琛岀鍙ｅ垪琛ㄤ腑鐨勭涓€涓€傚彲浠ヤ娇鐢ㄨ繑鍥炵殑 `struct parport **` 涓殑
``struct parport **next` 鍏冪礌鎵惧埌鍚庣画鐨勫苟琛岀鍙ｃ€傚鏋?`next`` 涓?NULL锛屽垯鍒楄〃涓?娌℃湁鏇村骞惰绔彛浜嗐€傚垪琛ㄤ腑鐨勭鍙ｆ暟閲忎笉浼氳秴杩?PARPORT_MAX銆?
##### 杩斿洖鍊?

涓€涓弿杩版湰鏈哄櫒鏈夋晥骞惰绔彛鐨?`struct parport *`锛屽鏋滄病鏈夊垯涓?NULL銆?
##### 閿欒


杩欎釜鍑芥暟鍙互杩斿洖 NULL锛岃〃绀烘病鏈夊彲鐢ㄧ殑骞惰绔彛銆?
##### 绀轰緥


```
	int detect_device (void)
	{
		struct parport *port;

		for (port = parport_enumerate ();
		port != NULL;
		port = port->next) {
			/* Try to detect a device on the port... */
			...
		}
		}

		...
	}
```
##### 娉ㄦ剰


parport_enumerate 宸茶搴熷純锛涘簲璇ヤ娇鐢?parport_register_driver 浠ｆ浛銆?
##### 鍙﹁鍙傞槄


parport_register_driver, parport_unregister_driver



### parport_register_device - 娉ㄥ唽浠ヤ娇鐢ㄤ竴涓鍙?

##### 姒傝


```
	#include <linux/parport.h>

	typedef int (*preempt_func) (void *handle);
	typedef void (*wakeup_func) (void *handle);
	typedef int (*irq_func) (int irq, void *handle, struct pt_regs *);

	struct pardevice *parport_register_device(struct parport *port,
						  const char *name,
						  preempt_func preempt,
						  wakeup_func wakeup,
						  irq_func irq,
						  int flags,
						  void *handle);
```
##### 鎻忚堪


浣跨敤杩欎釜鍑芥暟鍦ㄥ苟琛岀鍙ｏ紙`port`锛変笂娉ㄥ唽浣犵殑璁惧椹卞姩銆備竴鏃︿綘杩欐牱鍋氫簡锛屼綘灏嗚兘澶熶娇鐢?parport_claim 鍜?parport_release 鏉ヤ娇鐢ㄨ绔彛銆?
锛坄name`锛夊弬鏁版槸鍑虹幇鍦?/proc 鏂囦欢绯荤粺涓殑璁惧鍚嶇О銆傝瀛楃涓插繀椤诲湪璁惧鐨勬暣涓敓鍛藉懆鏈?锛堢洿鍒拌皟鐢?parport_unregister_device锛夊唴淇濇寔鏈夋晥銆?
杩欎釜鍑芥暟浼氬悜浣犵殑椹卞姩娉ㄥ唽涓変釜鍥炶皟锛歚preempt`銆乣wakeup` 鍜?`irq`銆傚畠浠瘡涓€涓兘鍙互鏄?NULL锛屼互琛ㄧず浣犱笉鎯宠璇ュ洖璋冦€?
褰?`preempt` 鍑芥暟琚皟鐢ㄦ椂锛屾槸鍥犱负鍙︿竴涓┍鍔ㄥ笇鏈涗娇鐢ㄥ苟琛岀鍙ｃ€俙preempt` 鍑芥暟濡傛灉杩斿洖
闈為浂鍊硷紝琛ㄧず骞惰绔彛灏氫笉鑳介噴鏀锯€斺€斿鏋滆繑鍥為浂锛岃绔彛灏变涪澶辩粰浜嗗彟涓€涓┍鍔紝骞朵笖鍦ㄤ娇鐢?涔嬪墠蹇呴』閲嶆柊澹版槑璇ョ鍙ｃ€?
`wakeup` 鍑芥暟鍦ㄥ彟涓€涓┍鍔ㄩ噴鏀句簡绔彛銆佷笖杩樻病鏈夊叾浠栭┍鍔ㄥ０鏄庡畠鏃惰璋冪敤銆備綘鍙互浠?`wakeup`
鍑芥暟鍐呴儴澹版槑骞惰绔彛锛堣繖绉嶆儏鍐典笅澹版槑淇濊瘉浼氭垚鍔燂級锛屾垨鑰呭鏋滀綘鐜板湪涓嶉渶瑕佷篃鍙互涓嶅０鏄庛€?
濡傛灉鍦ㄤ綘鐨勯┍鍔ㄥ凡澹版槑鐨勫苟琛岀鍙ｄ笂鍙戠敓浜嗕腑鏂紝`irq` 鍑芥暟灏嗚璋冪敤銆傦紙鍦ㄦ鍐欎竴浜涘叧浜?鍏变韩涓柇鐨勫唴瀹广€傦級

`handle` 鏄竴涓寚鍚戦┍鍔ㄧ壒瀹氭暟鎹殑鎸囬拡锛屽苟琚紶缁欏洖璋冨嚱鏁般€?
`flags` 鍙互鏄笅鍒楁爣蹇楃殑浣嶇粍鍚堬細

  ===================== =================================================
        Flag            Meaning
  ===================== =================================================
  PARPORT_DEV_EXCL	璁惧鏍规湰涓嶈兘鍏变韩骞惰绔彛銆備粎鍦ㄧ粷瀵瑰繀瑕佹椂浣跨敤銆?  ===================== =================================================

杩欎簺 typedef 瀹為檯涓婂苟鏈畾涔夆€斺€斿畠浠彧鏄负浜嗚鍑芥暟鍘熷瀷鏇村叿鍙鎬ц€屽睍绀哄嚭鏉ャ€?
```
	struct pardevice {
		struct parport *port;	/* Associated port */
		void *private;		/* Device driver's 'handle' */
		...
	};
```
##### 杩斿洖鍊?

涓€涓?`struct pardevice *`锛氭寚鍚戝凡娉ㄥ唽骞惰绔彛璁惧鐨勫彞鏌勶紝鍙敤浜?parport_claim銆?parport_release 绛夈€?
##### 閿欒


杩斿洖鍊间负 NULL 琛ㄧず鍦ㄨ绔彛涓婃敞鍐岃澶囨椂鍙戠敓浜嗛棶棰樸€?
##### 绀轰緥


```
	static int preempt (void *handle)
	{
		if (busy_right_now)
			return 1;

		must_reclaim_port = 1;
		return 0;
	}

	static void wakeup (void *handle)
	{
		struct toaster *private = handle;
		struct pardevice *dev = private->dev;
		if (!dev) return; /* avoid races */

		if (want_port)
			parport_claim (dev);
	}

	static int toaster_detect (struct toaster *private, struct parport *port)
	{
		private->dev = parport_register_device (port, "toaster", preempt,
							wakeup, NULL, 0,
							private);
		if (!private->dev)
			/* Couldn't register with parport. */
			return -EIO;

		must_reclaim_port = 0;
		busy_right_now = 1;
		parport_claim_or_block (private->dev);
		...
		/* Don't need the port while the toaster warms up. */
		busy_right_now = 0;
		...
		busy_right_now = 1;
		if (must_reclaim_port) {
			parport_claim_or_block (private->dev);
			must_reclaim_port = 0;
		}
		...
	}
```
##### 鍙﹁鍙傞槄


parport_unregister_device, parport_claim



### parport_unregister_device - 缁撴潫浣跨敤涓€涓鍙?

SYNPOPSIS

```
	#include <linux/parport.h>

	void parport_unregister_device (struct pardevice *dev);
```
##### 鎻忚堪


杩欎釜鍑芥暟涓?parport_register_device 鐩稿弽銆備娇鐢?parport_unregister_device 涔嬪悗锛宍dev`
涓嶅啀鏄竴涓湁鏁堢殑璁惧鍙ユ焺銆?
浣犱笉搴旀敞閿€涓€涓綋鍓嶅凡琚０鏄庣殑璁惧锛屽敖绠″鏋滀綘杩欐牱鍋氫簡瀹冧細琚嚜鍔ㄩ噴鏀俱€?
##### 绀轰緥


```
	...
	kfree (dev->private); /* before we lose the pointer */
	parport_unregister_device (dev);
	...
```
##### 鍙﹁鍙傞槄


parport_unregister_driver

### parport_claim, parport_claim_or_block - 涓轰竴涓澶囧０鏄庡苟琛岀鍙?

##### 姒傝


```
	#include <linux/parport.h>

	int parport_claim (struct pardevice *dev);
	int parport_claim_or_block (struct pardevice *dev);
```
##### 鎻忚堪


杩欎簺鍑芥暟灏濊瘯鑾峰彇 `dev` 鎵€娉ㄥ唽鐨勫苟琛岀鍙ｇ殑鎺у埗鏉冦€俙parport_claim` 涓嶉樆濉烇紝浣?`parport_claim_or_block` 鍙兘浼氶樆濉炪€傦紙鍦ㄦ鍐欎竴浜涘叧浜庡彲涓柇鎴栦笉鍙腑鏂樆濉炵殑鍐呭銆傦級

浣犱笉搴斿皾璇曞０鏄庝竴涓綘宸茬粡澹版槑杩囩殑绔彛銆?
##### 杩斿洖鍊?

杩斿洖鍊间负闆惰〃绀虹鍙ｈ鎴愬姛澹版槑锛岃皟鐢ㄨ€呯幇鍦ㄦ嫢鏈変簡璇ュ苟琛岀鍙ｃ€?
濡傛灉 `parport_claim_or_block` 鍦ㄦ垚鍔熻繑鍥炰箣鍓嶉樆濉炰簡锛岃繑鍥炲€间负姝ｅ€笺€?
##### 閿欒


========== ==========================================================
  -EAGAIN  绔彛褰撳墠涓嶅彲鐢紝浣嗗啀娆″皾璇曞０鏄庡畠鍙兘浼氭垚鍔熴€?========== ==========================================================

##### 鍙﹁鍙傞槄


parport_release

### parport_release - 閲婃斁骞惰绔彛


##### 姒傝


```
	#include <linux/parport.h>

	void parport_release (struct pardevice *dev);
```
##### 鎻忚堪


涓€鏃︿竴涓苟琛岀鍙ｈ澶囪澹版槑锛屽氨鍙互浣跨敤 `parport_release` 閲婃斁瀹冦€傚畠涓嶄細澶辫触锛屼絾浣?涓嶅簲閲婃斁涓€涓綘骞朵笉鎷ユ湁鐨勮澶囥€?
##### 绀轰緥


```
	static size_t write (struct pardevice *dev, const void *buf,
			size_t len)
	{
		...
		written = dev->port->ops->write_ecp_data (dev->port, buf,
							len);
		parport_release (dev);
		...
	}
```
##### 鍙﹁鍙傞槄


change_mode, parport_claim, parport_claim_or_block, parport_yield



### parport_yield, parport_yield_blocking - 涓存椂閲婃斁涓€涓苟琛岀鍙?

##### 姒傝


```
	#include <linux/parport.h>

	int parport_yield (struct pardevice *dev)
	int parport_yield_blocking (struct pardevice *dev);
```
##### 鎻忚堪


褰撲竴涓┍鍔ㄦ嫢鏈夊苟琛岀鍙ｇ殑鎺у埗鏉冩椂锛屽畠鍙互鍏佽鍙︿竴涓┍鍔ㄤ复鏃?`鍊熺敤` 瀹冦€俙parport_yield`
涓嶉樆濉烇紱`parport_yield_blocking` 鍙兘浼氶樆濉炪€?
##### 杩斿洖鍊?

杩斿洖鍊间负闆惰〃绀鸿皟鐢ㄨ€呬粛鐒舵嫢鏈夎绔彛锛屼笖璋冪敤娌℃湁闃诲銆?
鏉ヨ嚜 `parport_yield_blocking` 鐨勬杩斿洖鍊艰〃绀鸿皟鐢ㄨ€呬粛鐒舵嫢鏈夎绔彛锛屼笖璋冪敤鍙戠敓浜嗛樆濉炪€?
杩斿洖鍊间负 -EAGAIN 琛ㄧず璋冪敤鑰呬笉鍐嶆嫢鏈夎绔彛锛屽苟涓斿湪浣跨敤鍓嶅繀椤婚噸鏂板０鏄庡畠銆?
##### 閿欒


========= ==========================================================
  -EAGAIN  骞惰绔彛鐨勬墍鏈夋潈琚鍑轰簡銆?========= ==========================================================

##### 鍙﹁鍙傞槄


parport_release



### parport_wait_peripheral - 绛夊緟鐘舵€佺嚎锛屾渶澶?35ms


##### 姒傝


```
	#include <linux/parport.h>

	int parport_wait_peripheral (struct parport *port,
				     unsigned char mask,
				     unsigned char val);
```
##### 鎻忚堪


绛夊緟 mask 涓殑鐘舵€佺嚎鍖归厤 val 涓殑鍊笺€?
##### 杩斿洖鍊?

======== ==========================================================
 -EINTR  鏈変俊鍙锋寕璧?      0  mask 涓殑鐘舵€佺嚎鐨勫€间笌 val 涓殑涓€鑷?      1  绛夊緟瓒呮椂锛堝凡杩?35ms锛?======== ==========================================================

##### 鍙﹁鍙傞槄


parport_poll_peripheral



### parport_poll_peripheral - 绛夊緟鐘舵€佺嚎锛屼互寰璁?

##### 姒傝


```
	#include <linux/parport.h>

	int parport_poll_peripheral (struct parport *port,
				     unsigned char mask,
				     unsigned char val,
				     int usec);
```
##### 鎻忚堪


绛夊緟 mask 涓殑鐘舵€佺嚎鍖归厤 val 涓殑鍊笺€?
##### 杩斿洖鍊?

======== ==========================================================
 -EINTR  鏈変俊鍙锋寕璧?      0  mask 涓殑鐘舵€佺嚎鐨勫€间笌 val 涓殑涓€鑷?      1  绛夊緟瓒呮椂锛堝凡杩?usec 寰锛?======== ==========================================================

##### 鍙﹁鍙傞槄


parport_wait_peripheral



### parport_wait_event - 绛夊緟绔彛涓婄殑浜嬩欢


##### 姒傝


```
	#include <linux/parport.h>

	int parport_wait_event (struct parport *port, signed long timeout)
```
##### 鎻忚堪


绛夊緟绔彛涓婄殑浜嬩欢锛堜緥濡備腑鏂級銆傝秴鏃朵互 jiffies 璁°€?
##### 杩斿洖鍊?

======= ==========================================================
      0  鎴愬姛
     <0  閿欒锛堝敖蹇€€鍑猴級
     >0  瓒呮椂
======= ==========================================================

### parport_negotiate - 鎵ц IEEE 1284 鍗忓晢


##### 姒傝


```
	#include <linux/parport.h>

	int parport_negotiate (struct parport *, int mode);
```
##### 鎻忚堪


鎵ц IEEE 1284 鍗忓晢銆?
##### 杩斿洖鍊?

======= ==========================================================
     0  鎻℃墜鎴愬姛锛汭EEE 1284 澶栬鍜屾ā寮忓彲鐢?    -1  鎻℃墜澶辫触锛涘璁句笉鍏煎锛堟垨涓嶅瓨鍦級
     1  鎻℃墜鎴愬姛锛涘瓨鍦?IEEE 1284 澶栬浣嗘ā寮忎笉鍙敤
======= ==========================================================

##### 鍙﹁鍙傞槄


parport_read, parport_write



### parport_read - 浠庤澶囪鍙栨暟鎹?

##### 姒傝


```
	#include <linux/parport.h>

	ssize_t parport_read (struct parport *, void *buf, size_t len);
```
##### 鎻忚堪


浠ュ綋鍓?IEEE 1284 浼犺緭妯″紡浠庤澶囪鍙栨暟鎹€傝繖浠呭鏀寔鍙嶅悜鏁版嵁浼犺緭鐨勬ā寮忔湁鏁堛€?
##### 杩斿洖鍊?

濡傛灉涓鸿礋锛屽垯涓洪敊璇爜锛涘惁鍒欎负浼犺緭鐨勫瓧鑺傛暟銆?
##### 鍙﹁鍙傞槄


parport_write, parport_negotiate



### parport_write - 鍚戣澶囧啓鍏ユ暟鎹?

##### 姒傝


```
	#include <linux/parport.h>

	ssize_t parport_write (struct parport *, const void *buf, size_t len);
```
##### 鎻忚堪


浠ュ綋鍓?IEEE 1284 浼犺緭妯″紡鍚戣澶囧啓鍏ユ暟鎹€傝繖浠呭鏀寔姝ｅ悜鏁版嵁浼犺緭鐨勬ā寮忔湁鏁堛€?
##### 杩斿洖鍊?

濡傛灉涓鸿礋锛屽垯涓洪敊璇爜锛涘惁鍒欎负浼犺緭鐨勫瓧鑺傛暟銆?
##### 鍙﹁鍙傞槄


parport_read, parport_negotiate



### parport_open - 涓虹壒瀹氳澶囧彿娉ㄥ唽璁惧


##### 姒傝


```
	#include <linux/parport.h>

	struct pardevice *parport_open (int devnum, const char *name,
				        int (*pf) (void *),
					void (*kf) (void *),
					void (*irqf) (int, void *,
						      struct pt_regs *),
					int flags, void *handle);
```
##### 鎻忚堪


杩欑被浼间簬 parport_register_device锛屼絾鎺ュ彈涓€涓澶囧彿鑰屼笉鏄竴涓寚鍚?struct parport 鐨?鎸囬拡銆?
##### 杩斿洖鍊?

鍙傝 parport_register_device銆傚鏋滄病鏈変笌 devnum 鍏宠仈鐨勮澶囷紝杩斿洖 NULL銆?
##### 鍙﹁鍙傞槄


parport_register_device



### parport_close - 涓虹壒瀹氳澶囧彿娉ㄩ攢璁惧


##### 姒傝


```
	#include <linux/parport.h>

	void parport_close (struct pardevice *dev);
```
##### 鎻忚堪


杩欐槸 parport_open 瀵瑰簲鐨?parport_unregister_device銆?
##### 鍙﹁鍙傞槄


parport_unregister_device, parport_open



### parport_device_id - 鑾峰彇 IEEE 1284 璁惧 ID


##### 姒傝


```
	#include <linux/parport.h>

	ssize_t parport_device_id (int devnum, char *buffer, size_t len);
```
##### 鎻忚堪


鑾峰彇涓庣粰瀹氳澶囧叧鑱旂殑 IEEE 1284 璁惧 ID銆?
##### 杩斿洖鍊?

濡傛灉涓鸿礋锛屽垯涓洪敊璇爜锛涘惁鍒欎负鍖呭惈璁惧 ID 鐨?buffer 鐨勫瓧鑺傛暟銆傝澶?ID 鐨勬牸寮忓涓嬶細
```
	[length][ID]
```
鍓嶄袱涓瓧鑺傝〃绀烘暣涓澶?ID 鐨勫寘鍚€ч暱搴︼紝涓斾互澶х瀛楄妭搴忔帓鍒椼€侷D 鏄竴绯诲垪杩欐牱鐨?閰嶅锛?```
	key:value;
```
##### 娉ㄦ剰


璁稿璁惧鏈夋牸寮忎笉姝ｇ‘鐨?IEEE 1284 璁惧 ID銆?
##### 鍙﹁鍙傞槄


parport_find_class, parport_find_device



### parport_device_coords - 灏嗚澶囧彿杞崲涓鸿澶囧潗鏍?

##### 姒傝


```
	#include <linux/parport.h>

	int parport_device_coords (int devnum, int *parport, int *mux,
				   int *daisy);
```
##### 鎻忚堪


鍦ㄨ澶囧彿锛堜粠闆跺紑濮嬶級涓庤澶囧潗鏍囷紙绔彛銆佸璺鐢ㄥ櫒銆佽強鑺遍摼鍦板潃锛変箣闂磋浆鎹€?
##### 杩斿洖鍊?

鎴愬姛鏃朵负闆讹紝姝ゆ椂鍧愭爣涓?(`**parport`, `**mux`, `*daisy`)銆?
##### 鍙﹁鍙傞槄


parport_open, parport_device_id



### parport_find_class - 鎸夌被鍒煡鎵捐澶?

##### 姒傝


```
	#include <linux/parport.h>

	typedef enum {
		PARPORT_CLASS_LEGACY = 0,       /* Non-IEEE1284 device */
		PARPORT_CLASS_PRINTER,
		PARPORT_CLASS_MODEM,
		PARPORT_CLASS_NET,
		PARPORT_CLASS_HDC,              /* Hard disk controller */
		PARPORT_CLASS_PCMCIA,
		PARPORT_CLASS_MEDIA,            /* Multimedia device */
		PARPORT_CLASS_FDC,              /* Floppy disk controller */
		PARPORT_CLASS_PORTS,
		PARPORT_CLASS_SCANNER,
		PARPORT_CLASS_DIGCAM,
		PARPORT_CLASS_OTHER,            /* Anything else */
		PARPORT_CLASS_UNSPEC,           /* No CLS field in ID */
		PARPORT_CLASS_SCSIADAPTER
	} parport_device_class;

	int parport_find_class (parport_device_class cls, int from);
```
##### 鎻忚堪


鎸夌被鍒煡鎵捐澶囥€傛悳绱粠璁惧鍙?from+1 寮€濮嬨€?
##### 杩斿洖鍊?

璇ョ被鍒腑涓嬩竴涓澶囩殑璁惧鍙凤紝濡傛灉涓嶅瓨鍦ㄨ繖鏍风殑璁惧鍒欎负 -1銆?
##### 娉ㄦ剰


```
	int devnum = -1;
	while ((devnum = parport_find_class (PARPORT_CLASS_DIGCAM, devnum)) != -1) {
		struct pardevice *dev = parport_open (devnum, ...);
		...
	}
```
##### 鍙﹁鍙傞槄


parport_find_device, parport_open, parport_device_id



### parport_find_device - 鎸夌被鍒煡鎵捐澶?

##### 姒傝


```
	#include <linux/parport.h>

	int parport_find_device (const char *mfg, const char *mdl, int from);
```
##### 鎻忚堪


鎸夊巶鍟嗗拰鍨嬪彿鏌ユ壘璁惧銆傛悳绱粠璁惧鍙?from+1 寮€濮嬨€?
##### 杩斿洖鍊?

涓嬩竴涓尮閰嶈鏍肩殑璁惧鐨勮澶囧彿锛屽鏋滀笉瀛樺湪杩欐牱鐨勮澶囧垯涓?-1銆?
##### 娉ㄦ剰


```
	int devnum = -1;
	while ((devnum = parport_find_device ("IOMEGA", "ZIP+", devnum)) != -1) {
		struct pardevice *dev = parport_open (devnum, ...);
		...
	}
```
##### 鍙﹁鍙傞槄


parport_find_class, parport_open, parport_device_id



### parport_set_timeout - 璁剧疆涓嶆椿鍔ㄨ秴鏃?

##### 姒傝


```
	#include <linux/parport.h>

	long parport_set_timeout (struct pardevice *dev, long inactivity);
```
##### 鎻忚堪


涓哄凡娉ㄥ唽鐨勮澶囪缃笉娲诲姩瓒呮椂锛屼互 jiffies 璁°€傝繑鍥炲厛鍓嶇殑瓒呮椂鍊笺€?
##### 杩斿洖鍊?

鍏堝墠鐨勮秴鏃跺€硷紝浠?jiffies 璁°€?
##### 娉ㄦ剰


鐢变簬澶栬鐨勫欢杩燂紝鏌愪釜绔彛鐨?port->ops 鍑芥暟鍙兘浼氳€楁椂銆傚湪澶栬瓒呰繃 `inactivity` 涓?jiffies 娌℃湁鍝嶅簲涔嬪悗锛屽皢鍙戠敓瓒呮椂锛屽苟涓旈樆濉炵殑鍑芥暟灏嗚繑鍥炪€?
0 涓?jiffies 鐨勮秴鏃舵槸涓€涓壒渚嬶細鍑芥暟蹇呴』灏藉彲鑳藉鍦板畬鎴愬伐浣滐紝鑰屼笉闃诲鎴栧皢纭欢鐣欏湪
鏈煡鐘舵€併€備緥濡傦紝濡傛灉绔彛鎿嶄綔鏄湪涓柇澶勭悊绋嬪簭鍐呴儴鎵ц鐨勶紝鍒欏簲璇ヤ娇鐢?0 涓?jiffies
鐨勮秴鏃躲€?
涓€鏃︿负宸叉敞鍐岃澶囪缃紝瓒呮椂灏嗕繚鎸佸湪鎵€璁剧疆鐨勫€硷紝鐩村埌鍐嶆琚缃€?
##### 鍙﹁鍙傞槄


port->ops->xxx_read/write_yyy



## 绔彛鍑芥暟


port->ops 缁撴瀯锛坰truct parport_operations锛変腑鐨勫嚱鏁扮敱璐熻矗璇ョ鍙ｇ殑搴曞眰椹卞姩鎻愪緵銆?
### port->ops->read_data - 璇诲彇鏁版嵁瀵勫瓨鍣?

##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		unsigned char (*read_data) (struct parport *port);
		...
	};
```
##### 鎻忚堪


濡傛灉 port->modes 鍖呭惈 PARPORT_MODE_TRISTATE 鏍囧織锛屼笖 control 瀵勫瓨鍣ㄤ腑鐨?PARPORT_CONTROL_DIRECTION 浣嶈璁剧疆锛屽垯杩斿洖鏁版嵁寮曡剼涓婄殑鍊笺€傚鏋?port->modes 鍖呭惈
PARPORT_MODE_TRISTATE 鏍囧織锛岃€?PARPORT_CONTROL_DIRECTION 浣嶆湭琚缃紝鍒欒繑鍥?鍊?鍙兘*鏄啓鍏ユ暟鎹瘎瀛樺櫒鐨勬渶鍚庝竴涓€笺€傚惁鍒欒繑鍥炲€兼槸鏈畾涔夌殑銆?
##### 鍙﹁鍙傞槄


write_data, read_status, write_control



### port->ops->write_data - 鍐欏叆鏁版嵁瀵勫瓨鍣?

##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		void (*write_data) (struct parport *port, unsigned char d);
		...
	};
```
##### 鎻忚堪


鍐欏叆鏁版嵁瀵勫瓨鍣ㄣ€傚彲鑳戒細鏈夊壇浣滅敤锛堜緥濡備竴涓?STROBE 鑴夊啿锛夈€?
##### 鍙﹁鍙傞槄


read_data, read_status, write_control



### port->ops->read_status - 璇诲彇鐘舵€佸瘎瀛樺櫒


##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		unsigned char (*read_status) (struct parport *port);
		...
	};
```
##### 鎻忚堪


浠庣姸鎬佸瘎瀛樺櫒璇诲彇銆傝繖鏄竴涓綅鎺╃爜锛?
- PARPORT_STATUS_ERROR (鎵撳嵃鏈烘晠闅? "nFault")
- PARPORT_STATUS_SELECT (鍦ㄧ嚎, "Select")
- PARPORT_STATUS_PAPEROUT (鏃犵焊, "PError")
- PARPORT_STATUS_ACK (鎻℃墜, "nAck")
- PARPORT_STATUS_BUSY (蹇? "Busy")

鍙兘杩樻湁鍏朵粬浣嶈璁剧疆銆?
##### 鍙﹁鍙傞槄


read_data, write_data, write_control



### port->ops->read_control - 璇诲彇鎺у埗瀵勫瓨鍣?

##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		unsigned char (*read_control) (struct parport *port);
		...
	};
```
##### 鎻忚堪


杩斿洖鍐欏叆鎺у埗瀵勫瓨鍣ㄧ殑鏈€鍚庝竴涓€硷紙鏉ヨ嚜 write_control 鎴?frob_control锛夈€備笉鎵ц绔彛璁块棶銆?
##### 鍙﹁鍙傞槄


read_data, write_data, read_status, write_control



### port->ops->write_control - 鍐欏叆鎺у埗瀵勫瓨鍣?

##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		void (*write_control) (struct parport *port, unsigned char s);
		...
	};
```
##### 鎻忚堪


```
				  _______
	- PARPORT_CONTROL_STROBE (nStrobe)
				  _______
	- PARPORT_CONTROL_AUTOFD (nAutoFd)
				_____
	- PARPORT_CONTROL_INIT (nInit)
				  _________
	- PARPORT_CONTROL_SELECT (nSelectIn)
```
##### 鍙﹁鍙傞槄


read_data, write_data, read_status, frob_control



### port->ops->frob_control - 鍐欏叆鎺у埗瀵勫瓨鍣ㄤ綅


##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		unsigned char (*frob_control) (struct parport *port,
					unsigned char mask,
					unsigned char val);
		...
	};
```
##### 鎻忚堪


杩欑瓑浠蜂簬浠庢帶鍒跺瘎瀛樺櫒璇诲彇銆佹帺鎺?mask 涓殑浣嶃€佷笌 val 涓殑浣嶅仛寮傛垨锛岀劧鍚庡皢缁撴灉鍐欏叆
鎺у埗瀵勫瓨鍣ㄣ€?
鐢变簬鏌愪簺绔彛涓嶅厑璁镐粠鎺у埗绔彛璇诲彇锛屼細缁存姢鍏跺唴瀹圭殑杞欢鍓湰锛屽洜姝?frob_control 瀹為檯涓?鍙繘琛屼竴娆＄鍙ｈ闂€?
##### 鍙﹁鍙傞槄


read_data, write_data, read_status, write_control



### port->ops->enable_irq - 鍚敤涓柇鐢熸垚


##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		void (*enable_irq) (struct parport *port);
		...
	};
```
##### 鎻忚堪


骞惰绔彛纭欢琚寚绀哄湪閫傚綋鏃跺埢鐢熸垚涓柇锛屽敖绠￠偅浜涙椂鍒绘槸鏋舵瀯鐗瑰畾鐨勩€傚浜?PC 鏋舵瀯锛屼腑鏂?閫氬父鍦?nAck 鐨勪笂鍗囨部鐢熸垚銆?
##### 鍙﹁鍙傞槄


disable_irq



### port->ops->disable_irq - 绂佺敤涓柇鐢熸垚


##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		void (*disable_irq) (struct parport *port);
		...
	};
```
##### 鎻忚堪


骞惰绔彛纭欢琚寚绀轰笉瑕佺敓鎴愪腑鏂€備腑鏂湰韬苟鏈灞忚斀銆?
##### 鍙﹁鍙傞槄


enable_irq



### port->ops->data_forward - 鍚敤鏁版嵁椹卞姩鍣?

##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		void (*data_forward) (struct parport *port);
		...
	};
```
##### 鎻忚堪


鍚敤鏁版嵁绾块┍鍔ㄥ櫒锛岀敤浜?8 浣嶄富鏈哄埌澶栬鐨勯€氫俊銆?
##### 鍙﹁鍙傞槄


data_reverse



### port->ops->data_reverse - 灏嗙紦鍐插櫒缃负涓夋€?

##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		void (*data_reverse) (struct parport *port);
		...
	};
```
##### 鎻忚堪


濡傛灉 port->modes 璁剧疆浜?PARPORT_MODE_TRISTATE 浣嶏紝灏嗘暟鎹€荤嚎缃簬楂橀樆鎶楃姸鎬併€?
##### 鍙﹁鍙傞槄


data_forward



### port->ops->epp_write_data - 鍐欏叆 EPP 鏁版嵁


##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*epp_write_data) (struct parport *port, const void *buf,
					size_t len, int flags);
		...
	};
```
##### 鎻忚堪


浠?EPP 妯″紡鍐欏叆鏁版嵁锛屽苟杩斿洖鍐欏叆鐨勫瓧鑺傛暟銆?
`flags` 鍙傛暟鍙互鏄竴涓垨澶氫釜涓嬪垪鏍囧織鐨勪綅鎴栫粍鍚堬細

======================= =================================================
PARPORT_EPP_FAST	浣跨敤蹇€熶紶杈撱€傛煇浜涜姱鐗囨彁渚?16 浣嶅拰 32 浣嶅瘎瀛樺櫒銆?			浣嗘槸锛屽鏋滀竴娆′紶杈撹秴鏃讹紝杩斿洖鍊煎彲鑳戒笉鍙潬銆?======================= =================================================

##### 鍙﹁鍙傞槄


epp_read_data, epp_write_addr, epp_read_addr



### port->ops->epp_read_data - 璇诲彇 EPP 鏁版嵁


##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*epp_read_data) (struct parport *port, void *buf,
					size_t len, int flags);
		...
	};
```
##### 鎻忚堪


浠?EPP 妯″紡璇诲彇鏁版嵁锛屽苟杩斿洖璇诲彇鐨勫瓧鑺傛暟銆?
`flags` 鍙傛暟鍙互鏄竴涓垨澶氫釜涓嬪垪鏍囧織鐨勪綅鎴栫粍鍚堬細

======================= =================================================
PARPORT_EPP_FAST	浣跨敤蹇€熶紶杈撱€傛煇浜涜姱鐗囨彁渚?16 浣嶅拰 32 浣嶅瘎瀛樺櫒銆?			浣嗘槸锛屽鏋滀竴娆′紶杈撹秴鏃讹紝杩斿洖鍊煎彲鑳戒笉鍙潬銆?======================= =================================================

##### 鍙﹁鍙傞槄


epp_write_data, epp_write_addr, epp_read_addr



### port->ops->epp_write_addr - 鍐欏叆 EPP 鍦板潃


##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*epp_write_addr) (struct parport *port,
					const void *buf, size_t len, int flags);
		...
	};
```
##### 鎻忚堪


鍐欏叆 EPP 鍦板潃锛堟瘡涓?8 浣嶏級锛屽苟杩斿洖鍐欏叆鐨勬暟閲忋€?
`flags` 鍙傛暟鍙互鏄竴涓垨澶氫釜涓嬪垪鏍囧織鐨勪綅鎴栫粍鍚堬細

======================= =================================================
PARPORT_EPP_FAST	浣跨敤蹇€熶紶杈撱€傛煇浜涜姱鐗囨彁渚?16 浣嶅拰 32 浣嶅瘎瀛樺櫒銆?			浣嗘槸锛屽鏋滀竴娆′紶杈撹秴鏃讹紝杩斿洖鍊煎彲鑳戒笉鍙潬銆?======================= =================================================

锛圥ARPORT_EPP_FAST 瀵硅繖涓嚱鏁版湁鎰忎箟鍚楋紵锛?
##### 鍙﹁鍙傞槄


epp_write_data, epp_read_data, epp_read_addr



### port->ops->epp_read_addr - 璇诲彇 EPP 鍦板潃


##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*epp_read_addr) (struct parport *port, void *buf,
					size_t len, int flags);
		...
	};
```
##### 鎻忚堪


璇诲彇 EPP 鍦板潃锛堟瘡涓?8 浣嶏級锛屽苟杩斿洖璇诲彇鐨勬暟閲忋€?
`flags` 鍙傛暟鍙互鏄竴涓垨澶氫釜涓嬪垪鏍囧織鐨勪綅鎴栫粍鍚堬細

======================= =================================================
PARPORT_EPP_FAST	浣跨敤蹇€熶紶杈撱€傛煇浜涜姱鐗囨彁渚?16 浣嶅拰 32 浣嶅瘎瀛樺櫒銆?			浣嗘槸锛屽鏋滀竴娆′紶杈撹秴鏃讹紝杩斿洖鍊煎彲鑳戒笉鍙潬銆?======================= =================================================

锛圥ARPORT_EPP_FAST 瀵硅繖涓嚱鏁版湁鎰忎箟鍚楋紵锛?
##### 鍙﹁鍙傞槄


epp_write_data, epp_read_data, epp_write_addr



### port->ops->ecp_write_data - 鍐欏叆涓€鍧?ECP 鏁版嵁


##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*ecp_write_data) (struct parport *port,
					const void *buf, size_t len, int flags);
		...
	};
```
##### 鎻忚堪


鍐欏叆涓€鍧?ECP 鏁版嵁銆俙flags` 鍙傛暟琚拷鐣ャ€?
##### 杩斿洖鍊?

鍐欏叆鐨勫瓧鑺傛暟銆?
##### 鍙﹁鍙傞槄


ecp_read_data, ecp_write_addr



### port->ops->ecp_read_data - 璇诲彇涓€鍧?ECP 鏁版嵁


##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*ecp_read_data) (struct parport *port,
					void *buf, size_t len, int flags);
		...
	};
```
##### 鎻忚堪


璇诲彇涓€鍧?ECP 鏁版嵁銆俙flags` 鍙傛暟琚拷鐣ャ€?
##### 杩斿洖鍊?

璇诲彇鐨勫瓧鑺傛暟銆傛敞鎰忥細FIFO 涓彲鑳借繕鏈夋洿澶氭湭璇绘暟鎹€傛湁娌℃湁鍔炴硶璁?FIFO 鏆傚仠浠ラ槻姝㈣繖绉?鎯呭喌锛?
##### 鍙﹁鍙傞槄


ecp_write_block, ecp_write_addr



### port->ops->ecp_write_addr - 鍐欏叆涓€鍧?ECP 鍦板潃


##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*ecp_write_addr) (struct parport *port,
					const void *buf, size_t len, int flags);
		...
	};
```
##### 鎻忚堪


鍐欏叆涓€鍧?ECP 鍦板潃銆俙flags` 鍙傛暟琚拷鐣ャ€?
##### 杩斿洖鍊?

鍐欏叆鐨勫瓧鑺傛暟銆?
##### 娉ㄦ剰


杩欏彲鑳戒娇鐢ㄤ竴涓?FIFO锛屽鏋滄槸杩欐牱锛屽湪 FIFO 娓呯┖涔嬪墠涓嶅簲杩斿洖銆?
##### 鍙﹁鍙傞槄


ecp_read_data, ecp_write_data



### port->ops->nibble_read_data - 浠?nibble 妯″紡璇诲彇涓€鍧楁暟鎹?

##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*nibble_read_data) (struct parport *port,
					void *buf, size_t len, int flags);
		...
	};
```
##### 鎻忚堪


浠?nibble 妯″紡璇诲彇涓€鍧楁暟鎹€俙flags` 鍙傛暟琚拷鐣ャ€?
##### 杩斿洖鍊?

璇诲彇鐨勫畬鏁村瓧鑺傛暟銆?
##### 鍙﹁鍙傞槄


byte_read_data, compat_write_data



### port->ops->byte_read_data - 浠ュ瓧鑺傛ā寮忚鍙栦竴鍧楁暟鎹?

##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*byte_read_data) (struct parport *port,
					void *buf, size_t len, int flags);
		...
	};
```
##### 鎻忚堪


浠ュ瓧鑺傛ā寮忚鍙栦竴鍧楁暟鎹€俙flags` 鍙傛暟琚拷鐣ャ€?
##### 杩斿洖鍊?

璇诲彇鐨勫瓧鑺傛暟銆?
##### 鍙﹁鍙傞槄


nibble_read_data, compat_write_data



### port->ops->compat_write_data - 浠ュ吋瀹规ā寮忓啓鍏ヤ竴鍧楁暟鎹?

##### 姒傝


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*compat_write_data) (struct parport *port,
					const void *buf, size_t len, int flags);
		...
	};
```
##### 鎻忚堪


浠ュ吋瀹规ā寮忓啓鍏ヤ竴鍧楁暟鎹€俙flags` 鍙傛暟琚拷鐣ャ€?
##### 杩斿洖鍊?

鍐欏叆鐨勫瓧鑺傛暟銆?
##### 鍙﹁鍙傞槄


nibble_read_data, byte_read_data
