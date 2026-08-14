
## VFIO AP 閿佹杩帮紙VFIO AP Locks Overview锛?
鏈枃妗ｆ弿杩颁簡涓?vfio_ap 璁惧椹卞姩瀹夊叏杩愯鐩稿叧鐨勯攣銆傞€氱瘒灏嗕娇鐢ㄤ互涓嬪彉閲忔潵琛ㄧず姝ゅ鎵€鎻忚堪缁撴瀯鐨勫疄渚嬶細


  struct ap_matrix_dev *matrix_dev;
  struct ap_matrix_mdev *matrix_mdev;
  struct kvm *kvm;

### Matrix Devices 閿侊紙drivers/s390/crypto/vfio_ap_private.h锛?


  struct ap_matrix_dev {
  	...
  	struct list_head mdev_list;
  	struct mutex mdevs_lock;
  	...
  }

Matrix Devices 閿侊紙matrix_dev->mdevs_lock锛夊疄鐜颁负鍖呭惈浜庡崟涓?struct ap_matrix_dev 瀵硅薄鍐呯殑鍏ㄥ眬浜掓枼閲忥紙mutex锛夈€傝閿佹帶鍒跺 matrix_dev->mdev_list 涓瘡涓?matrix_mdev 鍐呮墍鍖呭惈鐨勬墍鏈夊瓧娈电殑璁块棶銆傚湪璇诲彇銆佸啓鍏ユ垨浣跨敤鏌愪釜 matrix_mdev 瀹炰緥锛堣〃绀?vfio_ap 璁惧椹卞姩鐨勬煇涓?mediated 璁惧锛変腑鏌愬瓧娈电殑鏁版嵁鏃讹紝蹇呴』鎸佹湁璇ラ攣銆?
### KVM 閿侊紙include/linux/kvm_host.h锛?


  struct kvm {
  	...
  	struct mutex lock;
  	...
  }

KVM 閿侊紙kvm->lock锛夋帶鍒跺 KVM 瀹㈡埛鏈猴紙guest锛夌姸鎬佹暟鎹殑璁块棶銆傚綋灏嗕竴涓垨澶氫釜 AP 閫傞厤鍣ㄣ€?domain 鎴栨帶鍒跺煙锛坈ontrol domain锛夌儹鎻掓嫈锛坧lug/unplug锛夎繘鎴栫Щ鍑哄鎴锋満鏃讹紝vfio_ap 璁惧椹卞姩蹇呴』鎸佹湁璇ラ攣銆?
KVM 鎸囬拡瀛樺偍鍦?matrix_mdev 瀹炰緥涓紙matrix_mdev->kvm = kvm锛夛紝璇ュ疄渚嬪寘鍚凡闄勫姞鍒?KVM 瀹㈡埛鏈虹殑 mediated 璁惧鐨勭姸鎬併€?
### Guests 閿侊紙drivers/s390/crypto/vfio_ap_private.h锛?


  struct ap_matrix_dev {
  	...
  	struct list_head mdev_list;
  	struct mutex guests_lock;
  	...
  }

Guests 閿侊紙matrix_dev->guests_lock锛夋帶鍒跺 matrix_mdev 瀹炰緥锛坢atrix_dev->mdev_list锛夌殑璁块棶锛岃繖浜涘疄渚嬭〃绀烘寔鏈夊凡闄勫姞鍒?KVM 瀹㈡埛鏈虹殑 mediated 璁惧鐘舵€佺殑 mediated 璁惧銆傚繀椤绘寔鏈夎閿侊細

1. 鍦?vfio_ap 璁惧椹卞姩浣跨敤 KVM 鎸囬拡鏉ョ儹鎻掓嫈/绉诲嚭鐩撮€氾紙passed through锛夊埌 KVM 瀹㈡埛鏈虹殑 AP 璁惧鏃讹紝鎺у埗瀵?KVM 鎸囬拡锛坢atrix_mdev->kvm锛夌殑璁块棶銆?
2. 鍚?matrix_dev->mdev_list 娣诲姞鎴栦粠涓Щ闄?matrix_mdev 瀹炰緥銆傝繖鍦ㄩ亶鍘嗚鍒楄〃浠ユ煡鎵剧敤浜庣儹鎻掓嫈/绉诲嚭鐩撮€氬埌 KVM 瀹㈡埛鏈虹殑 AP 璁惧鐨?ap_matrix_mdev 瀹炰緥鏃讹紝瀵圭‘淇濇纭殑鍔犻攣椤哄簭鏄繀瑕佺殑銆?
   渚嬪锛屽綋浠?vfio_ap 璁惧椹卞姩绉婚櫎涓€涓槦鍒楄澶囨椂锛屽鏋滆閫傞厤鍣紙adapter锛夎鐩撮€氬埌 KVM 瀹㈡埛鏈猴紝鍒欏繀椤诲皢鍏剁Щ鍑恒€備负浜嗙‘瀹氳閫傞厤鍣ㄦ槸鍚﹁鐩撮€氾紝蹇呴』鎵惧埌璇ラ槦鍒楁墍鍒嗛厤鐨?matrix_mdev 瀵硅薄銆傞殢鍚庡彲鐢?KVM 鎸囬拡锛坢atrix_mdev->kvm锛夋潵鍒ゆ柇璇?mediated 璁惧鏄惁琚洿閫氾紙matrix_mdev->kvm != NULL锛夛紝鑻ユ槸锛屽垯绉诲嚭璇ラ€傞厤鍣ㄣ€?
濡傛灉 KVM 鎸囬拡鏈鐢ㄤ簬鐑彃鎷?绉诲嚭鐩撮€氬埌 KVM 瀹㈡埛鏈虹殑璁惧锛屽垯涓嶅繀鑾峰彇 Guests 閿侊紱浣嗗湪杩欑鎯呭喌涓嬶紝鐢变簬 KVM 鎸囬拡鏄湪 Matrix Devices 閿佺殑淇濇姢涓嬭璁剧疆鍜屾竻闄ょ殑锛屽繀椤绘寔鏈?Matrix Devices 閿侊紙matrix_dev->mdevs_lock锛夋墠鑳借闂?KVM 鎸囬拡銆備竴涓伆褰撶殑渚嬪瓙鏄鐞?PQAP(AQIC) 鎸囦护瀛愬嚱鏁版嫤鎴紙interception锛夌殑鍑芥暟銆傝澶勭悊鍑芥暟鍙渶瑕佽闂?KVM 鎸囬拡鏉ヨ缃垨娓呴櫎 IRQ 璧勬簮锛屽洜姝ゅ彧闇€鎸佹湁 matrix_dev->mdevs_lock銆?
### PQAP Hook 閿侊紙arch/s390/include/asm/kvm_host.h锛?


  typedef int (**crypto_hook)(struct kvm_vcpu **vcpu);

  struct kvm_s390_crypto {
  	...
  	struct rw_semaphore pqap_hook_rwsem;
  	crypto_hook *pqap_hook;
  	...
  };

PQAP Hook 閿佹槸涓€涓鍐欎俊鍙烽噺锛坮/w semaphore锛夛紝鎺у埗瀵瑰鐞嗗嚱鏁版寚閽?`(*kvm->arch.crypto.pqap_hook)` 鐨勮闂紝璇ユ寚閽堝湪 PQAP(AQIC) 鎸囦护瀛愬嚱鏁拌瀹夸富鏈烘嫤鎴椂浼氳璋冪敤銆傚綋璁剧疆 pqap_hook 鍊兼椂蹇呴』浠ュ啓妯″紡鎸佹湁璇ラ攣锛屽湪璋冪敤 pqap_hook 鍑芥暟鏃朵互璇绘ā寮忔寔鏈夈€?