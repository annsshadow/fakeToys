## Adjunct Processor (AP) facility

## 杈呭姪澶勭悊鍣紙AP锛夎鏂?


## Introduction

## 绠€浠?

The Adjunct Processor (AP) facility is an IBM Z cryptographic facility comprised
of three AP instructions and from 1 up to 256 PCIe cryptographic adapter cards.
The AP devices provide cryptographic functions to all CPUs assigned to a
linux system running in an IBM Z system LPAR.

杈呭姪澶勭悊鍣紙AP锛夎鏂芥槸 IBM Z 鐨勫瘑鐮佸璁炬柦锛岀敱涓夋潯 AP 鎸囦护浠ュ強 1 鍒?256 鍧?
PCIe 瀵嗙爜閫傞厤鍣ㄥ崱缁勬垚銆侫P 璁惧涓哄垎閰嶇粰杩愯鍦?IBM Z 绯荤粺 LPAR 涓殑 Linux 绯荤粺鐨?
鎵€鏈?CPU 鎻愪緵瀵嗙爜瀛﹀姛鑳姐€?

The AP adapter cards are exposed via the AP bus. The motivation for vfio-ap
is to make AP cards available to KVM guests using the VFIO mediated device
framework. This implementation relies considerably on the s390 virtualization
facilities which do most of the hard work of providing direct access to AP
devices.

AP 閫傞厤鍣ㄥ崱閫氳繃 AP 鎬荤嚎鏆撮湶銆倂fio-ap 鐨勫姩鏈烘槸浣跨敤 VFIO 涓粙锛坢ediated锛夎澶?
妗嗘灦浣?AP 鍗″ KVM 瀹㈡埛鏈哄彲鐢ㄣ€傝瀹炵幇鍦ㄥ緢澶х▼搴︿笂渚濊禆 s390 鐨勮櫄鎷熷寲璁炬柦锛屽悗鑰?
瀹屾垚浜嗘彁渚?AP 璁惧鐩存帴璁块棶鐨勫ぇ閮ㄥ垎鑹伴毦宸ヤ綔銆?

## AP Architectural Overview

## AP 鏋舵瀯姒傝堪

To facilitate the comprehension of the design, let's start with some
definitions:

涓轰簡渚夸簬鐞嗚В璇ヨ璁★紝璁╂垜浠粠涓€浜涘畾涔夊紑濮嬶細

- AP adapter

- AP 閫傞厤鍣?

  An AP adapter is an IBM Z adapter card that can perform cryptographic
  functions. There can be from 0 to 256 adapters assigned to an LPAR. Adapters
  assigned to the LPAR in which a linux host is running will be available to
  the linux host. Each adapter is identified by a number from 0 to 255; however,
  the maximum adapter number is determined by machine model and/or adapter type.
  When installed, an AP adapter is accessed by AP instructions executed by any
  CPU.

  涓€涓?AP 閫傞厤鍣ㄦ槸涓€鍧楄兘澶熸墽琛屽瘑鐮佸鍔熻兘鐨?IBM Z 閫傞厤鍣ㄥ崱銆傚彲浠ュ垎閰嶇粰涓€涓?LPAR
  鐨勯€傞厤鍣ㄦ暟閲忎粠 0 鍒?256 涓嶇瓑銆傚垎閰嶇粰杩愯 Linux 涓绘満鐨?LPAR 鐨勯€傞厤鍣ㄥ皢瀵?
  Linux 涓绘満鍙敤銆傛瘡涓€傞厤鍣ㄧ敱涓€涓?0 鍒?255 涔嬮棿鐨勬暟瀛楁爣璇嗭紱涓嶈繃锛屾渶澶ч€傞厤鍣ㄥ彿
  鐢辨満鍨嬶紙machine model锛夊拰/鎴栭€傞厤鍣ㄧ被鍨嬪喅瀹氥€傚畨瑁呭悗锛孉P 閫傞厤鍣ㄧ敱浠讳綍 CPU 鎵ц鐨?
  AP 鎸囦护璁块棶銆?

  The AP adapter cards are assigned to a given LPAR via the system's Activation
  Profile which can be edited via the HMC. When the linux host system is IPL'd
  in the LPAR, the AP bus detects the AP adapter cards assigned to the LPAR and
  creates a sysfs device for each assigned adapter. For example, if AP adapters
  4 and 10 (0x0a) are assigned to the LPAR, the AP bus will create the following

  AP 閫傞厤鍣ㄥ崱閫氳繃绯荤粺鐨勬縺娲绘瑕侊紙Activation Profile锛夊垎閰嶇粰缁欏畾鐨?LPAR锛岃姒傝
  鍙€氳繃 HMC 缂栬緫銆傚綋 Linux 涓绘満绯荤粺鍦ㄨ LPAR 涓?IPL 鍚庯紝AP 鎬荤嚎浼氭娴嬪垎閰嶇粰璇?
  LPAR 鐨?AP 閫傞厤鍣ㄥ崱锛屽苟涓烘瘡涓鍒嗛厤鐨勯€傞厤鍣ㄥ垱寤轰竴涓?sysfs 璁惧銆備緥濡傦紝濡傛灉
  AP 閫傞厤鍣?4 鍜?10锛?x0a锛夎鍒嗛厤缁欒 LPAR锛孉P 鎬荤嚎灏嗗垱寤轰互涓?

```

    /sys/devices/ap/card04
    /sys/devices/ap/card0a

  Symbolic links to these devices will also be created in the AP bus devices
  sub-directory::

    /sys/bus/ap/devices/[card04]
    /sys/bus/ap/devices/[card04]

```

- AP domain

- AP 鍩?

  An adapter is partitioned into domains. An adapter can hold up to 256 domains
  depending upon the adapter type and hardware configuration. A domain is
  identified by a number from 0 to 255; however, the maximum domain number is
  determined by machine model and/or adapter type.. A domain can be thought of
  as a set of hardware registers and memory used for processing AP commands. A
  domain can be configured with a secure private key used for clear key
  encryption. A domain is classified in one of two ways depending upon how it
  may be accessed:

  涓€涓€傞厤鍣ㄨ鍒掑垎涓哄涓煙銆傛牴鎹€傞厤鍣ㄧ被鍨嬪拰纭欢閰嶇疆锛屼竴涓€傞厤鍣ㄦ渶澶氬彲瀹圭撼 256 涓?
  鍩熴€備竴涓煙鐢变竴涓?0 鍒?255 涔嬮棿鐨勬暟瀛楁爣璇嗭紱涓嶈繃锛屾渶澶у煙鍚嶇敱鏈哄瀷锛坢achine model锛?
  鍜?鎴栭€傞厤鍣ㄧ被鍨嬪喅瀹氥€備竴涓煙鍙互琚湅浣滄槸涓€缁勭敤浜庡鐞?AP 鍛戒护鐨勭‖浠跺瘎瀛樺櫒鍜屽唴瀛樸€?
  涓€涓煙鍙互閰嶇疆涓€涓敤浜庢槑鏂囧瘑閽ワ紙clear key锛夊姞瀵嗙殑瀹夊叏绉侀挜銆傛牴鎹煙鐨勮闂柟寮忥紝
  鍩熻鍒嗕负涓ょ被锛?

    - Usage domains are domains that are targeted by an AP instruction to
      process an AP command.

    - 浣跨敤鍩燂紙Usage domain锛夋槸鎸囪 AP 鎸囦护浣滀负鐩爣浠ュ鐞?AP 鍛戒护鐨勫煙銆?

    - Control domains are domains that are changed by an AP command sent to a
      usage domain; for example, to set the secure private key for the control
      domain.

    - 鎺у埗鍩燂紙Control domain锛夋槸鎸囩敱鍙戝線浣跨敤鍩熺殑 AP 鍛戒护鏇存敼鐨勫煙锛涗緥濡傦紝涓烘帶鍒跺煙
      璁剧疆瀹夊叏绉侀挜銆?

  The AP usage and control domains are assigned to a given LPAR via the system's
  Activation Profile which can be edited via the HMC. When a linux host system
  is IPL'd in the LPAR, the AP bus module detects the AP usage and control
  domains assigned to the LPAR. The domain number of each usage domain and
  adapter number of each AP adapter are combined to create AP queue devices
  (see AP Queue section below). The domain number of each control domain will be
  represented in a bitmask and stored in a sysfs file
  /sys/bus/ap/ap_control_domain_mask. The bits in the mask, from most to least
  significant bit, correspond to domains 0-255.

  AP 浣跨敤鍩熷拰鎺у埗鍩熼€氳繃绯荤粺鐨勬縺娲绘瑕佸垎閰嶇粰缁欏畾鐨?LPAR锛岃姒傝鍙€氳繃 HMC 缂栬緫銆?
  褰?Linux 涓绘満绯荤粺鍦ㄨ LPAR 涓?IPL 鍚庯紝AP 鎬荤嚎妯″潡浼氭娴嬪垎閰嶇粰璇?LPAR 鐨?AP 浣跨敤鍩?
  鍜屾帶鍒跺煙銆傛瘡涓娇鐢ㄥ煙鐨勫煙鍚嶅拰姣忎釜 AP 閫傞厤鍣ㄧ殑閫傞厤鍣ㄥ彿琚粍鍚堣捣鏉ュ垱寤?AP 闃熷垪璁惧
  锛堣涓嬫枃"AP 闃熷垪"涓€鑺傦級銆傛瘡涓帶鍒跺煙鐨勫煙鍚嶅皢鐢ㄤ竴涓綅鎺╃爜琛ㄧず锛屽苟瀛樺偍鍦?sysfs 鏂囦欢
  /sys/bus/ap/ap_control_domain_mask 涓€傛帺鐮佷腑鐨勪綅锛屼粠鏈€楂樻湁鏁堜綅鍒版渶浣庢湁鏁堜綅锛?
  鍒嗗埆瀵瑰簲鍩?0-255銆?

- AP Queue

- AP 闃熷垪

  An AP queue is the means by which an AP command is sent to a usage domain
  inside a specific adapter. An AP queue is identified by a tuple
  comprised of an AP adapter ID (APID) and an AP queue index (APQI). The
  APQI corresponds to a given usage domain number within the adapter. This tuple
  forms an AP Queue Number (APQN) uniquely identifying an AP queue. AP
  instructions include a field containing the APQN to identify the AP queue to
  which the AP command is to be sent for processing.

  AP 闃熷垪鏄皢 AP 鍛戒护鍙戦€佸埌鐗瑰畾閫傞厤鍣ㄥ唴閮ㄤ娇鐢ㄥ煙鐨勬墜娈点€備竴涓?AP 闃熷垪鐢变竴涓厓缁勬爣璇嗭紝
  璇ュ厓缁勭敱 AP 閫傞厤鍣?ID锛圓PID锛夊拰 AP 闃熷垪绱㈠紩锛圓PQI锛夌粍鎴愩€侫PQI 瀵瑰簲浜庨€傞厤鍣ㄥ唴
  缁欏畾鐨勪娇鐢ㄥ煙鍙枫€傝繖涓厓缁勬瀯鎴愪竴涓?AP 闃熷垪鍙凤紙APQN锛夛紝鍞竴鍦版爣璇嗕竴涓?AP 闃熷垪銆侫P
  鎸囦护鍖呭惈涓€涓瓨鏀?APQN 鐨勫瓧娈碉紝鐢ㄤ簬鏍囪瘑瑕佸皢 AP 鍛戒护鍙戝線鍝釜 AP 闃熷垪杩涜澶勭悊銆?

  The AP bus will create a sysfs device for each APQN that can be derived from
  the cross product of the AP adapter and usage domain numbers detected when the
  AP bus module is loaded. For example, if adapters 4 and 10 (0x0a) and usage
  domains 6 and 71 (0x47) are assigned to the LPAR, the AP bus will create the

  AP 鎬荤嚎浼氫负鍙粠 AP 鎬荤嚎妯″潡鍔犺浇鏃舵娴嬪埌鐨?AP 閫傞厤鍣ㄥ彿鍜岀敤娉曞煙鍚嶅弶绉帹瀵煎嚭鐨勬瘡涓?
  APQN 鍒涘缓涓€涓?sysfs 璁惧銆備緥濡傦紝濡傛灉閫傞厤鍣?4 鍜?10锛?x0a锛変互鍙婁娇鐢ㄥ煙 6 鍜?71
  锛?x47锛夎鍒嗛厤缁欒 LPAR锛孉P 鎬荤嚎灏嗗垱寤轰互涓?

```

    /sys/devices/ap/card04/04.0006
    /sys/devices/ap/card04/04.0047
    /sys/devices/ap/card0a/0a.0006
    /sys/devices/ap/card0a/0a.0047

  The following symbolic links to these devices will be created in the AP bus
  devices subdirectory::

    /sys/bus/ap/devices/[04.0006]
    /sys/bus/ap/devices/[04.0047]
    /sys/bus/ap/devices/[0a.0006]
    /sys/bus/ap/devices/[0a.0047]

```

- AP Instructions:

- AP 鎸囦护锛?

  There are three AP instructions:

  鏈変笁鏉?AP 鎸囦护锛?

  - NQAP: to enqueue an AP command-request message to a queue
  - DQAP: to dequeue an AP command-reply message from a queue
  - PQAP: to administer the queues

  - NQAP锛氬皢涓€涓?AP 鍛戒护璇锋眰娑堟伅鍏ラ槦鍒版煇涓槦鍒?
  - DQAP锛氫粠涓€涓槦鍒椾腑灏嗕竴涓?AP 鍛戒护搴旂瓟娑堟伅鍑洪槦
  - PQAP锛氱鐞嗚繖浜涢槦鍒?

  AP instructions identify the domain that is targeted to process the AP
  command; this must be one of the usage domains. An AP command may modify a
  domain that is not one of the usage domains, but the modified domain
  must be one of the control domains.

  AP 鎸囦护鏍囪瘑浣滀负鐩爣澶勭悊 AP 鍛戒护鐨勫煙锛涜繖蹇呴』鏄娇鐢ㄥ煙涔嬩竴銆備竴鏉?AP 鍛戒护鍙互淇敼涓€涓?
  骞堕潪浣跨敤鍩熺殑鍩燂紝浣嗚淇敼鐨勫煙蹇呴』鏄帶鍒跺煙涔嬩竴銆?

## AP and SIE

## AP 涓?SIE

Let's now take a look at how AP instructions executed on a guest are interpreted
by the hardware.

鐜板湪璁╂垜浠湅鐪嬪湪瀹㈡埛鏈轰笂鎵ц鐨?AP 鎸囦护鏄浣曡纭欢瑙ｉ噴鐨勩€?

A satellite control block called the Crypto Control Block (CRYCB) is attached to
our main hardware virtualization control block. The CRYCB contains an AP Control
Block (APCB) that has three fields to identify the adapters, usage domains and
control domains assigned to the KVM guest:

涓€涓О涓哄瘑鐮佹帶鍒跺潡锛圕rypto Control Block锛孋RYCB锛夌殑杈呭姪鎺у埗鍧楄闄勫姞鍒版垜浠殑涓荤‖浠?
铏氭嫙鍖栨帶鍒跺潡涓娿€侰RYCB 鍖呭惈涓€涓?AP 鎺у埗鍧楋紙APCB锛夛紝瀹冩湁涓変釜瀛楁鏉ユ爣璇嗗垎閰嶇粰 KVM
瀹㈡埛鏈虹殑閫傞厤鍣ㄣ€佷娇鐢ㄥ煙鍜屾帶鍒跺煙锛?

- The AP Mask (APM) field is a bit mask that identifies the AP adapters assigned
  to the KVM guest. Each bit in the mask, from left to right, corresponds to
  an APID from 0-255. If a bit is set, the corresponding adapter is valid for
  use by the KVM guest.

- AP 鎺╃爜锛圓PM锛夊瓧娈垫槸涓€涓綅鎺╃爜锛屾爣璇嗗垎閰嶇粰 KVM 瀹㈡埛鏈虹殑 AP 閫傞厤鍣ㄣ€傛帺鐮佷腑鐨勪綅锛?
  浠庡乏鍒板彸锛屽垎鍒搴斾竴涓粠 0-255 鐨?APID銆傚鏋滄煇浣嶈缃綅锛屽垯鐩稿簲閫傞厤鍣ㄥ彲渚?KVM
  瀹㈡埛鏈轰娇鐢ㄣ€?

- The AP Queue Mask (AQM) field is a bit mask identifying the AP usage domains
  assigned to the KVM guest. Each bit in the mask, from left to right,
  corresponds to an AP queue index (APQI) from 0-255. If a bit is set, the
  corresponding queue is valid for use by the KVM guest.

- AP 闃熷垪鎺╃爜锛圓QM锛夊瓧娈垫槸涓€涓綅鎺╃爜锛屾爣璇嗗垎閰嶇粰 KVM 瀹㈡埛鏈虹殑 AP 浣跨敤鍩熴€傛帺鐮佷腑鐨?
  浣嶏紝浠庡乏鍒板彸锛屽垎鍒搴斾竴涓粠 0-255 鐨?AP 闃熷垪绱㈠紩锛圓PQI锛夈€傚鏋滄煇浣嶈缃綅锛屽垯
  鐩稿簲闃熷垪鍙緵 KVM 瀹㈡埛鏈轰娇鐢ㄣ€?

- The AP Domain Mask field is a bit mask that identifies the AP control domains
  assigned to the KVM guest. The ADM bit mask controls which domains can be
  changed by an AP command-request message sent to a usage domain from the
  guest. Each bit in the mask, from left to right, corresponds to a domain from
  0-255. If a bit is set, the corresponding domain can be modified by an AP
  command-request message sent to a usage domain.

- AP 鍩熸帺鐮佸瓧娈垫槸涓€涓綅鎺╃爜锛屾爣璇嗗垎閰嶇粰 KVM 瀹㈡埛鏈虹殑 AP 鎺у埗鍩熴€侫DM 浣嶆帺鐮佹帶鍒跺摢浜?
  鍩熷彲浠ヨ浠庡鎴锋満鍙戝線浣跨敤鍩熺殑 AP 鍛戒护璇锋眰娑堟伅鏇存敼銆傛帺鐮佷腑鐨勪綅锛屼粠宸﹀埌鍙筹紝鍒嗗埆瀵瑰簲
  涓€涓粠 0-255 鐨勫煙銆傚鏋滄煇浣嶈缃綅锛屽垯鐩稿簲鍩熷彲浠ヨ浠庡鎴锋満鍙戝線浣跨敤鍩熺殑 AP 鍛戒护璇锋眰
  娑堟伅淇敼銆?

If you recall from the description of an AP Queue, AP instructions include
an APQN to identify the AP queue to which an AP command-request message is to be
sent (NQAP and PQAP instructions), or from which a command-reply message is to
be received (DQAP instruction). The validity of an APQN is defined by the matrix
calculated from the APM and AQM; it is the Cartesian product of all assigned
adapter numbers (APM) with all assigned queue indexes (AQM). For example, if
adapters 1 and 2 and usage domains 5 and 6 are assigned to a guest, the APQNs
(1,5), (1,6), (2,5) and (2,6) will be valid for the guest.

濡傛灉浣犺繕璁板緱 AP 闃熷垪鐨勬弿杩帮紝AP 鎸囦护鍖呭惈涓€涓?APQN 浠ユ爣璇嗚灏?AP 鍛戒护璇锋眰娑堟伅鍙戝線
锛圢QAP 鍜?PQAP 鎸囦护锛夋垨浠庡摢涓槦鍒楁帴鏀跺懡浠ゅ簲绛旀秷鎭紙DQAP 鎸囦护锛夌殑 AP 闃熷垪銆備竴涓?
APQN 鐨勬湁鏁堟€х敱浠?APM 鍜?AQM 璁＄畻鍑虹殑鐭╅樀瀹氫箟锛涘畠鏄墍鏈夎鍒嗛厤鐨勯€傞厤鍣ㄥ彿锛圓PM锛?
涓庢墍鏈夎鍒嗛厤鐨勯槦鍒楃储寮曪紙AQM锛夌殑绗涘崱灏旂Н銆備緥濡傦紝濡傛灉閫傞厤鍣?1 鍜?2 浠ュ強浣跨敤鍩?5 鍜?6
琚垎閰嶅埌涓€涓鎴锋満锛岄偅涔?APQN (1,5)銆?1,6)銆?2,5) 鍜?(2,6) 瀵硅瀹㈡埛鏈烘湁鏁堛€?

The APQNs can provide secure key functionality - i.e., a private key is stored
on the adapter card for each of its domains - so each APQN must be assigned to
```

   Example 1: Valid configuration:
   ------------------------------
   Guest1: adapters 1,2  domains 5,6
   Guest2: adapter  1,2  domain 7

   This is valid because both guests have a unique set of APQNs:
      Guest1 has APQNs (1,5), (1,6), (2,5), (2,6);
      Guest2 has APQNs (1,7), (2,7)

   Example 2: Valid configuration:
   ------------------------------
   Guest1: adapters 1,2 domains 5,6
   Guest2: adapters 3,4 domains 5,6

   This is also valid because both guests have a unique set of APQNs:
      Guest1 has APQNs (1,5), (1,6), (2,5), (2,6);
      Guest2 has APQNs (3,5), (3,6), (4,5), (4,6)

   Example 3: Invalid configuration:
   --------------------------------
   Guest1: adapters 1,2  domains 5,6
   Guest2: adapter  1    domains 6,7

   This is an invalid configuration because both guests have access to
   APQN (1,6).

```

## The Design

## 璁捐

The design introduces three new objects:

璇ヨ璁″紩鍏ヤ簡涓変釜鏂板璞★細

1. AP matrix device
2. VFIO AP device driver (vfio_ap.ko)
3. VFIO AP mediated pass-through device

1. AP 鐭╅樀璁惧
2. VFIO AP 璁惧椹卞姩锛坴fio_ap.ko锛?
3. VFIO AP 涓粙鐩撮€氾紙pass-through锛夎澶?

### The VFIO AP device driver

### VFIO AP 璁惧椹卞姩

The VFIO AP (vfio_ap) device driver serves the following purposes:

VFIO AP锛坴fio_ap锛夎澶囬┍鍔ㄧ敤浜庝互涓嬬洰鐨勶細

1. Provides the interfaces to secure APQNs for exclusive use of KVM guests.

1. 鎻愪緵鎺ュ彛浠ラ鐣?APQN 渚?KVM 瀹㈡埛鏈虹嫭鍗犱娇鐢ㄣ€?

2. Sets up the VFIO mediated device interfaces to manage a vfio_ap mediated
   device and creates the sysfs interfaces for assigning adapters, usage
   domains, and control domains comprising the matrix for a KVM guest.

2. 寤虹珛 VFIO 涓粙璁惧鎺ュ彛浠ョ鐞嗕竴涓?vfio_ap 涓粙璁惧锛屽苟鍒涘缓鐢ㄤ簬鍒嗛厤鏋勬垚 KVM
   瀹㈡埛鏈虹煩闃电殑閫傞厤鍣ㄣ€佷娇鐢ㄥ煙鍜屾帶鍒跺煙鐨?sysfs 鎺ュ彛銆?

3. Configures the APM, AQM and ADM in the APCB contained in the CRYCB referenced
   by a KVM guest's SIE state description to grant the guest access to a matrix
   of AP devices

3. 閰嶇疆浣嶄簬 KVM 瀹㈡埛鏈?SIE 鐘舵€佹弿杩版墍寮曠敤鐨?CRYCB 涓殑 APCB 鍐呯殑 APM銆丄QM 鍜?ADM锛?
   浠ユ巿浜堝鎴锋満瀵逛竴缁?AP 璁惧鐨勮闂潈闄?

### Reserve APQNs for exclusive use of KVM guests

### 涓?KVM 瀹㈡埛鏈虹嫭鍗犱娇鐢ㄨ€岄鐣?APQN

The following block diagram illustrates the mechanism by which APQNs are
```

				+------------------+
		 7 remove       |                  |
	   +--------------------> cex4queue driver |
	   |                    |                  |
	   |                    +------------------+
	   |
	   |
	   |                    +------------------+          +----------------+
	   |  5 register driver |                  | 3 create |                |
	   |   +---------------->   Device core    +---------->  matrix device |
	   |   |                |                  |          |                |
	   |   |                +--------^---------+          +----------------+
	   |   |                         |
	   |   |                         +-------------------+
	   |   | +-----------------------------------+       |
	   |   | |      4 register AP driver         |       | 2 register device
	   |   | |                                   |       |
  +--------+---+-v---+                      +--------+-------+-+
  |                  |                      |                  |
  |      ap_bus      +--------------------- >  vfio_ap driver  |
  |                  |       8 probe        |                  |
  +--------^---------+                      +--^--^------------+
  6 edit   |                                   |  |
    apmask |     +-----------------------------+  | 11 mdev create
    aqmask |     |           1 modprobe           |
  +--------+-----+---+           +----------------+-+         +----------------+
  |                  |           |                  |10 create|     mediated   |
  |      admin       |           | VFIO device core |--------->     matrix     |
  |                  +           |                  |         |     device     |
  +------+-+---------+           +--------^---------+         +--------^-------+
	 | |                              |                            |
	 | | 9 create vfio_ap-passthrough |                            |
	 | +------------------------------+                            |
	 +-------------------------------------------------------------+
		     12  assign adapter/domain/control domain

```

The process for reserving an AP queue for use by a KVM guest is:

涓?KVM 瀹㈡埛鏈洪鐣欎竴涓?AP 闃熷垪鐨勮繃绋嬫槸锛?

1. The administrator loads the vfio_ap device driver
2. The vfio-ap driver during its initialization will register a single 'matrix'
   device with the device core. This will serve as the parent device for
   all vfio_ap mediated devices used to configure an AP matrix for a guest.
3. The /sys/devices/vfio_ap/matrix device is created by the device core
4. The vfio_ap device driver will register with the AP bus for AP queue devices
   of type 10 and higher (CEX4 and newer). The driver will provide the vfio_ap
   driver's probe and remove callback interfaces. Devices older than CEX4 queues
   are not supported to simplify the implementation by not needlessly
   complicating the design by supporting older devices that will go out of
   service in the relatively near future, and for which there are few older
   systems around on which to test.
5. The AP bus registers the vfio_ap device driver with the device core
6. The administrator edits the AP adapter and queue masks to reserve AP queues
   for use by the vfio_ap device driver.
7. The AP bus removes the AP queues reserved for the vfio_ap driver from the
   default zcrypt cex4queue driver.
8. The AP bus probes the vfio_ap device driver to bind the queues reserved for
   it.
9. The administrator creates a passthrough type vfio_ap mediated device to be
   used by a guest
10. The administrator assigns the adapters, usage domains and control domains
    to be exclusively used by a guest.

1. 绠＄悊鍛樺姞杞?vfio_ap 璁惧椹卞姩
2. vfio-ap 椹卞姩鍦ㄥ叾鍒濆鍖栨湡闂村皢鍚戣澶囨牳蹇冿紙device core锛夋敞鍐屼竴涓崟涓€鐨勨€滅煩闃碘€?
   锛坢atrix锛夎澶囥€傚畠灏嗕綔涓虹敤浜庝负瀹㈡埛鏈洪厤缃?AP 鐭╅樀鐨勬墍鏈?vfio_ap 涓粙璁惧鐨勭埗璁惧銆?
3. /sys/devices/vfio_ap/matrix 璁惧鐢辫澶囨牳蹇冨垱寤?
4. vfio_ap 璁惧椹卞姩灏嗗悜 AP 鎬荤嚎娉ㄥ唽浠ュ鐞嗙被鍨嬩负 10 鍙婃洿楂橈紙CEX4 鍙婃洿鏂帮級鐨?AP 闃熷垪
   璁惧銆傝椹卞姩灏嗘彁渚?vfio_ap 椹卞姩鐨?probe 鍜?remove 鍥炶皟鎺ュ彛銆備笉鏀寔鏃╀簬 CEX4 闃熷垪鐨?
   璁惧锛岃繖鏄负浜嗛€氳繃涓嶄负鍦ㄧ浉瀵逛笉涔呯殑灏嗘潵浼氬仠姝㈡湇鍔°€佷笖鍙敤浜庢祴璇曠殑鏃х郴缁熷緢灏戠殑鏃ц澶?
   鎻愪緵鏀寔锛屼粠鑰岄伩鍏嶄笉蹇呰鍦颁娇璁捐澶嶆潅鍖栵紝浠ョ畝鍖栧疄鐜般€?
5. AP 鎬荤嚎灏?vfio_ap 璁惧椹卞姩娉ㄥ唽鍒拌澶囨牳蹇?
6. 绠＄悊鍛樼紪杈?AP 閫傞厤鍣ㄥ拰闃熷垪鎺╃爜浠ラ鐣欎緵 vfio_ap 璁惧椹卞姩浣跨敤鐨?AP 闃熷垪銆?
7. AP 鎬荤嚎灏嗕粠榛樿鐨?zcrypt cex4queue 椹卞姩涓Щ闄や负 vfio_ap 椹卞姩棰勭暀鐨?AP 闃熷垪銆?
8. AP 鎬荤嚎鎺㈡祴 vfio_ap 璁惧椹卞姩浠ョ粦瀹氫负鍏堕鐣欑殑闃熷垪銆?
9. 绠＄悊鍛樺垱寤轰竴涓洿閫氱被鍨嬬殑 vfio_ap 涓粙璁惧渚涘鎴锋満浣跨敤
10. 绠＄悊鍛樺垎閰嶄緵瀹㈡埛鏈虹嫭鍗犱娇鐢ㄧ殑閫傞厤鍣ㄣ€佷娇鐢ㄥ煙鍜屾帶鍒跺煙銆?

### Set up the VFIO mediated device interfaces

### 寤虹珛 VFIO 涓粙璁惧鎺ュ彛

The VFIO AP device driver utilizes the common interfaces of the VFIO mediated
device core driver to:

VFIO AP 璁惧椹卞姩鍒╃敤 VFIO 涓粙璁惧鏍稿績椹卞姩鐨勯€氱敤鎺ュ彛鏉ワ細

- Register an AP mediated bus driver to add a vfio_ap mediated device to and
  remove it from a VFIO group.
- Create and destroy a vfio_ap mediated device
- Add a vfio_ap mediated device to and remove it from the AP mediated bus driver
- Add a vfio_ap mediated device to and remove it from an IOMMU group

- 娉ㄥ唽涓€涓?AP 涓粙鎬荤嚎椹卞姩锛屼互灏?vfio_ap 涓粙璁惧鍔犲叆鎴栫Щ鍑?VFIO 缁勩€?
- 鍒涘缓骞堕攢姣佷竴涓?vfio_ap 涓粙璁惧
- 灏?vfio_ap 涓粙璁惧鍔犲叆鎴栫Щ鍑?AP 涓粙鎬荤嚎椹卞姩
- 灏?vfio_ap 涓粙璁惧鍔犲叆鎴栫Щ鍑?IOMMU 缁?

The following high-level block diagram shows the main components and interfaces
```

   +-------------+
   |             |
   | +---------+ | mdev_register_driver() +--------------+
   | |  Mdev   | +<-----------------------+              |
   | |  bus    | |                        | vfio_mdev.ko |
   | | driver  | +----------------------->+              |<-> VFIO user
   | +---------+ |    probe()/remove()    +--------------+    APIs
   |             |
   |  MDEV CORE  |
   |   MODULE    |
   |   mdev.ko   |
   | +---------+ | mdev_register_parent() +--------------+
   | |Physical | +<-----------------------+              |
   | | device  | |                        |  vfio_ap.ko  |<-> matrix
   | |interface| +----------------------->+              |    device
   | +---------+ |       callback         +--------------+
   +-------------+

```

During initialization of the vfio_ap module, the matrix device is registered
with an 'mdev_parent_ops' structure that provides the sysfs attribute
structures, mdev functions and callback interfaces for managing the mediated
matrix device.

鍦?vfio_ap 妯″潡鐨勫垵濮嬪寲鏈熼棿锛岀煩闃佃澶囦細鐢ㄤ竴涓?'mdev_parent_ops' 缁撴瀯娉ㄥ唽锛岃缁撴瀯
鎻愪緵鐢ㄤ簬绠＄悊涓粙鐭╅樀璁惧鐨?sysfs 灞炴€х粨鏋勩€乵dev 鍑芥暟鍜屽洖璋冩帴鍙ｃ€?

- sysfs attribute structures:

- sysfs 灞炴€х粨鏋勶細

  supported_type_groups
    The VFIO mediated device framework supports creation of user-defined
    mediated device types. These mediated device types are specified
    via the 'supported_type_groups' structure when a device is registered
    with the mediated device framework. The registration process creates the
    sysfs structures for each mediated device type specified in the
    'mdev_supported_types' sub-directory of the device being registered. Along
    with the device type, the sysfs attributes of the mediated device type are
    provided.

  supported_type_groups
    VFIO 涓粙璁惧妗嗘灦鏀寔鍒涘缓鐢ㄦ埛瀹氫箟鐨勪腑浠嬭澶囩被鍨嬨€傝繖浜涗腑浠嬭澶囩被鍨嬪湪璁惧鍚戜腑浠?
    璁惧妗嗘灦娉ㄥ唽鏃堕€氳繃 'supported_type_groups' 缁撴瀯鎸囧畾銆傛敞鍐岃繃绋嬩細涓鸿娉ㄥ唽璁惧鐨?
    'mdev_supported_types' 瀛愮洰褰曚腑鎸囧畾鐨勬瘡绉嶄腑浠嬭澶囩被鍨嬪垱寤?sysfs 缁撴瀯銆傝繛鍚岃澶?
    绫诲瀷锛岃繕浼氭彁渚涜涓粙璁惧绫诲瀷鐨?sysfs 灞炴€с€?

    The VFIO AP device driver will register one mediated device type for
    passthrough devices:

    VFIO AP 璁惧椹卞姩灏嗕负鐩撮€氳澶囨敞鍐屼竴绉嶄腑浠嬭澶囩被鍨嬶細

      /sys/devices/vfio_ap/matrix/mdev_supported_types/vfio_ap-passthrough

    Only the read-only attributes required by the VFIO mdev framework will
```

	... name
	... device_api
	... available_instances
	... device_api

    Where:

    鍏朵腑锛?

	* name:
	    specifies the name of the mediated device type
	* device_api:
	    the mediated device type's API
	* available_instances:
	    the number of vfio_ap mediated passthrough devices
	    that can be created
	* device_api:
	    specifies the VFIO API
  mdev_attr_groups
    This attribute group identifies the user-defined sysfs attributes of the
    mediated device. When a device is registered with the VFIO mediated device
    framework, the sysfs attribute files identified in the 'mdev_attr_groups'
    structure will be created in the vfio_ap mediated device's directory. The
    sysfs attributes for a vfio_ap mediated device are:

	* name:
	    鎸囧畾涓粙璁惧绫诲瀷鐨勫悕绉?
	* device_api:
	    涓粙璁惧绫诲瀷鐨?API
	* available_instances:
	    鍙垱寤虹殑 vfio_ap 涓粙鐩撮€氳澶囨暟閲?
	* device_api:
	    鎸囧畾 VFIO API
  mdev_attr_groups
    璇ュ睘鎬х粍鏍囪瘑涓粙璁惧鐨勭敤鎴峰畾涔?sysfs 灞炴€с€傚綋璁惧鍚?VFIO 涓粙璁惧妗嗘灦娉ㄥ唽鏃讹紝
    鍦?'mdev_attr_groups' 缁撴瀯涓爣璇嗙殑 sysfs 灞炴€ф枃浠跺皢鍒涘缓鍦?vfio_ap 涓粙璁惧鐨?
    鐩綍涓€倂fio_ap 涓粙璁惧鐨?sysfs 灞炴€т负锛?

    assign_adapter / unassign_adapter:
      Write-only attributes for assigning/unassigning an AP adapter to/from the
      vfio_ap mediated device. To assign/unassign an adapter, the APID of the
      adapter is echoed into the respective attribute file.
    assign_domain / unassign_domain:
      Write-only attributes for assigning/unassigning an AP usage domain to/from
      the vfio_ap mediated device. To assign/unassign a domain, the domain
      number of the usage domain is echoed into the respective attribute
      file.
    matrix:
      A read-only file for displaying the APQNs derived from the Cartesian
      product of the adapter and domain numbers assigned to the vfio_ap mediated
      device.
    guest_matrix:
      A read-only file for displaying the APQNs derived from the Cartesian
      product of the adapter and domain numbers assigned to the APM and AQM
      fields respectively of the KVM guest's CRYCB. This may differ from the
      the APQNs assigned to the vfio_ap mediated device if any APQN does not
      reference a queue device bound to the vfio_ap device driver (i.e., the
      queue is not in the host's AP configuration).
    assign_control_domain / unassign_control_domain:
      Write-only attributes for assigning/unassigning an AP control domain
      to/from the vfio_ap mediated device. To assign/unassign a control domain,
      the ID of the domain to be assigned/unassigned is echoed into the
      respective attribute file.
    control_domains:
      A read-only file for displaying the control domain numbers assigned to the
      vfio_ap mediated device.
    ap_config:
      A read/write file that, when written to, allows all three of the
      vfio_ap mediated device's ap matrix masks to be replaced in one shot.
      Three masks are given, one for adapters, one for domains, and one for
      control domains. If the given state cannot be set then no changes are
      made to the vfio-ap mediated device.

    assign_adapter / unassign_adapter锛?
      鐢ㄤ簬灏?AP 閫傞厤鍣ㄥ垎閰?瑙ｉ櫎鍒嗛厤鍒?vfio_ap 涓粙璁惧鐨勫彧鍐欏睘鎬с€傝鍒嗛厤/瑙ｉ櫎鍒嗛厤
      涓€涓€傞厤鍣紝璇峰皢璇ラ€傞厤鍣ㄧ殑 APID 鍥炴樉锛坋cho锛夊埌鐩稿簲鐨勫睘鎬ф枃浠躲€?
    assign_domain / unassign_domain锛?
      鐢ㄤ簬灏?AP 浣跨敤鍩熷垎閰?瑙ｉ櫎鍒嗛厤鍒?vfio_ap 涓粙璁惧鐨勫彧鍐欏睘鎬с€傝鍒嗛厤/瑙ｉ櫎鍒嗛厤
      涓€涓煙锛岃灏嗚浣跨敤鍩熺殑鍩熷悕鍥炴樉鍒扮浉搴旂殑灞炴€ф枃浠躲€?
    matrix锛?
      涓€涓彧璇绘枃浠讹紝鐢ㄤ簬鏄剧ず浠庡垎閰嶇粰 vfio_ap 涓粙璁惧鐨勯€傞厤鍣ㄥ拰鍩熷悕绗涘崱灏旂Н鎺ㄥ鍑虹殑
      APQN銆?
    guest_matrix锛?
      涓€涓彧璇绘枃浠讹紝鐢ㄤ簬鏄剧ず浠庡垎鍒垎閰嶇粰 KVM 瀹㈡埛鏈?CRYCB 鐨?APM 鍜?AQM 瀛楁鐨勯€傞厤鍣?
      鍜屽煙鍚嶇瑳鍗″皵绉帹瀵煎嚭鐨?APQN銆傚鏋滀换浣?APQN 鏈紩鐢ㄧ粦瀹氬埌 vfio_ap 璁惧椹卞姩锛堝嵆璇?
      闃熷垪涓嶅湪涓绘満鐨?AP 閰嶇疆涓級鐨勯槦鍒楄澶囷紝鍒欏畠鍙兘涓庡垎閰嶇粰 vfio_ap 涓粙璁惧鐨?
      APQN 涓嶅悓銆?
    assign_control_domain / unassign_control_domain锛?
      鐢ㄤ簬灏?AP 鎺у埗鍩熷垎閰?瑙ｉ櫎鍒嗛厤鍒?vfio_ap 涓粙璁惧鐨勫彧鍐欏睘鎬с€傝鍒嗛厤/瑙ｉ櫎鍒嗛厤
      涓€涓帶鍒跺煙锛岃灏嗚鍒嗛厤/瑙ｉ櫎鍒嗛厤鐨勫煙 ID 鍥炴樉鍒扮浉搴旂殑灞炴€ф枃浠躲€?
    control_domains锛?
      涓€涓彧璇绘枃浠讹紝鐢ㄤ簬鏄剧ず鍒嗛厤缁?vfio_ap 涓粙璁惧鐨勬帶鍒跺煙鍚嶃€?
    ap_config锛?
      涓€涓/鍐欐枃浠讹紝鍐欏叆鏃跺厑璁镐竴娆℃€ф浛鎹?vfio_ap 涓粙璁惧鐨勫叏閮ㄤ笁涓?AP 鐭╅樀鎺╃爜銆?
      鎻愪緵涓変釜鎺╃爜锛屽垎鍒敤浜庨€傞厤鍣ㄣ€佸煙鍜屾帶鍒跺煙銆傚鏋滅粰瀹氱姸鎬佹棤娉曡缃紝鍒欎笉浼氬
      vfio-ap 涓粙璁惧鍋氫换浣曟洿鏀广€?

      The format of the data written to ap_config is as follows:
      {amask},{dmask},{cmask}\n

      \n is a newline character.

      amask, dmask, and cmask are masks identifying which adapters, domains,
      and control domains should be assigned to the mediated device.

      The format of a mask is as follows:
      0xNN..NN

      Where NN..NN is 64 hexadecimal characters representing a 256-bit value.
      The leftmost (highest order) bit represents adapter/domain 0.

      For an example set of masks that represent your mdev's current
      configuration, simply cat ap_config.

      Setting an adapter or domain number greater than the maximum allowed for
      the system will result in an error.

      This attribute is intended to be used by automation. End users would be
      better served using the respective assign/unassign attributes for
      adapters, domains, and control domains.

      鍐欏叆 ap_config 鐨勬暟鎹牸寮忓涓嬶細
      {amask},{dmask},{cmask}\n

      \n 鏄竴涓崲琛岀銆?

      amask銆乨mask 鍜?cmask 鏄帺鐮侊紝鏍囪瘑搴斿皢鍝簺閫傞厤鍣ㄣ€佸煙鍜屾帶鍒跺煙鍒嗛厤缁欒涓粙璁惧銆?

      鎺╃爜鐨勬牸寮忓涓嬶細
      0xNN..NN

      鍏朵腑 NN..NN 鏄?64 涓崄鍏繘鍒跺瓧绗︼紝琛ㄧず涓€涓?256 浣嶅€笺€傛渶宸﹁竟锛堟渶楂樹綅锛夌殑浣?
      琛ㄧず閫傞厤鍣?鍩?0銆?

      瑕佽幏鍙栬〃绀轰綘鐨?mdev 褰撳墠閰嶇疆鐨勪竴缁勬帺鐮佺ず渚嬶紝鍙渶 cat ap_config銆?

      璁剧疆涓€涓ぇ浜庣郴缁熸墍鍏佽鏈€澶у€肩殑閫傞厤鍣ㄦ垨鍩熷悕灏嗗鑷撮敊璇€?

      璇ュ睘鎬ф棬鍦ㄤ緵鑷姩鍖栦娇鐢ㄣ€傛渶缁堢敤鎴锋渶濂戒娇鐢ㄥ悇鑷殑 assign/unassign 灞炴€ф潵鎿嶄綔
      閫傞厤鍣ㄣ€佸煙鍜屾帶鍒跺煙銆?

```

- functions:

- 鍑芥暟锛?

  create:
    allocates the ap_matrix_mdev structure used by the vfio_ap driver to:

    - Store the reference to the KVM structure for the guest using the mdev
    - Store the AP matrix configuration for the adapters, domains, and control
      domains assigned via the corresponding sysfs attributes files
    - Store the AP matrix configuration for the adapters, domains and control
      domains available to a guest. A guest may not be provided access to APQNs
      referencing queue devices that do not exist, or are not bound to the
      vfio_ap device driver.

  create锛?
    鍒嗛厤 vfio_ap 椹卞姩鐢ㄤ簬浠ヤ笅鐢ㄩ€旂殑 ap_matrix_mdev 缁撴瀯锛?

    - 瀛樺偍浣跨敤 mdev 鐨勫鎴锋満鐨?KVM 缁撴瀯寮曠敤
    - 瀛樺偍閫氳繃鐩稿簲 sysfs 灞炴€ф枃浠跺垎閰嶇殑閫傞厤鍣ㄣ€佸煙鍜屾帶鍒跺煙鐨?AP 鐭╅樀閰嶇疆
    - 瀛樺偍瀹㈡埛鏈哄彲鐢ㄧ殑閫傞厤鍣ㄣ€佸煙鍜屾帶鍒跺煙鐨?AP 鐭╅樀閰嶇疆銆備笉寰楀悜瀹㈡埛鏈烘彁渚涘寮曠敤涓嶅瓨鍦?
      鎴栨湭缁戝畾鍒?vfio_ap 璁惧椹卞姩鐨勯槦鍒楄澶囩殑 APQN 鐨勮闂€?

  remove:
    deallocates the vfio_ap mediated device's ap_matrix_mdev structure.
    This will be allowed only if a running guest is not using the mdev.

  remove锛?
    閲婃斁 vfio_ap 涓粙璁惧鐨?ap_matrix_mdev 缁撴瀯銆?
    浠呭綋娌℃湁姝ｅ湪杩愯鐨勫鎴锋満浣跨敤璇?mdev 鏃舵墠鍏佽銆?

- callback interfaces

- 鍥炶皟鎺ュ彛

  open_device:
    the open_device callback is invoked by userspace to connect the
    VFIO iommu group for the matrix mdev device to the MDEV bus.  The
    callback retrieves the KVM structure used to configure the KVM guest
    and configures the guest's access to the AP matrix defined via the
    vfio_ap mediated device's sysfs attribute files.

  open_device锛?
    open_device 鍥炶皟鐢辩敤鎴风┖闂磋皟鐢紝浠ュ皢鐭╅樀 mdev 璁惧鐨?VFIO iommu 缁勮繛鎺ュ埌 MDEV
    鎬荤嚎銆傝鍥炶皟妫€绱㈢敤浜庨厤缃?KVM 瀹㈡埛鏈虹殑 KVM 缁撴瀯锛屽苟閰嶇疆瀹㈡埛鏈哄閫氳繃 vfio_ap 涓粙
    璁惧 sysfs 灞炴€ф枃浠跺畾涔夌殑 AP 鐭╅樀鐨勮闂€?

  close_device:
    this callback deconfigures the guest's AP matrix.

  close_device锛?
    璇ュ洖璋冨彇娑堥厤缃鎴锋満鐨?AP 鐭╅樀銆?

  ioctl:
    this callback handles the VFIO_DEVICE_GET_INFO and VFIO_DEVICE_RESET ioctls
    defined by the vfio framework.

  ioctl锛?
    璇ュ洖璋冨鐞?vfio 妗嗘灦瀹氫箟鐨?VFIO_DEVICE_GET_INFO 鍜?VFIO_DEVICE_RESET ioctls銆?

### Configure the guest's AP resources

### 閰嶇疆瀹㈡埛鏈虹殑 AP 璧勬簮

Configuring the AP resources for a KVM guest will be performed at the
time of `open_device` and `close_device`. The guest's AP resources are
configured via its APCB by:

涓?KVM 瀹㈡埛鏈洪厤缃?AP 璧勬簮灏嗗湪 `open_device` 鍜?`close_device` 鏃惰繘琛屻€傚鎴锋満鐨?
AP 璧勬簮閫氳繃鍏?APCB 閰嶇疆濡備笅锛?

- Setting the bits in the APM corresponding to the APIDs assigned to the
  vfio_ap mediated device via its 'assign_adapter' interface.
- Setting the bits in the AQM corresponding to the domains assigned to the
  vfio_ap mediated device via its 'assign_domain' interface.
- Setting the bits in the ADM corresponding to the domain dIDs assigned to the
  vfio_ap mediated device via its 'assign_control_domains' interface.

- 璁剧疆 APM 涓搴斾簬閫氳繃 'assign_adapter' 鎺ュ彛鍒嗛厤缁?vfio_ap 涓粙璁惧鐨?APID 鐨勪綅銆?
- 璁剧疆 AQM 涓搴斾簬閫氳繃 'assign_domain' 鎺ュ彛鍒嗛厤缁?vfio_ap 涓粙璁惧鐨勫煙鐨勪綅銆?
- 璁剧疆 ADM 涓搴斾簬閫氳繃 'assign_control_domains' 鎺ュ彛鍒嗛厤缁?vfio_ap 涓粙璁惧鐨?
  鍩?dID 鐨勪綅銆?

The linux device model precludes passing a device through to a KVM guest that
is not bound to the device driver facilitating its pass-through. Consequently,
an APQN that does not reference a queue device bound to the vfio_ap device
driver will not be assigned to a KVM guest's matrix. The AP architecture,
however, does not provide a means to filter individual APQNs from the guest's
matrix, so the adapters, domains and control domains assigned to vfio_ap
mediated device via its sysfs 'assign_adapter', 'assign_domain' and
'assign_control_domain' interfaces will be filtered before providing the AP
configuration to a guest:

Linux 璁惧妯″瀷涓嶅厑璁稿皢涓€涓湭缁戝畾鍒颁績鎴愬叾鐩撮€氱殑椹卞姩鐨勮澶囩洿閫氱粰 KVM 瀹㈡埛鏈恒€傚洜姝わ紝
涓嶅紩鐢ㄧ粦瀹氬埌 vfio_ap 璁惧椹卞姩鐨勯槦鍒楄澶囩殑 APQN 涓嶄細琚垎閰嶇粰 KVM 瀹㈡埛鏈虹殑鐭╅樀銆傜劧鑰岋紝
AP 鏋舵瀯娌℃湁鎻愪緵浠庡鎴锋満鐭╅樀涓繃婊ゅ崟涓?APQN 鐨勬墜娈碉紝鍥犳鍦ㄥ悜瀹㈡埛鏈烘彁渚?AP 閰嶇疆涔嬪墠锛?
浼氶€氳繃鍏?sysfs 'assign_adapter'銆?assign_domain' 鍜?'assign_control_domain' 鎺ュ彛鍒嗛厤缁?
vfio_ap 涓粙璁惧鐨勯€傞厤鍣ㄣ€佸煙鍜屾帶鍒跺煙灏嗚杩囨护锛?

- The APIDs of the adapters, the APQIs of the domains and the domain numbers of
  the control domains assigned to the matrix mdev that are not also assigned to
  the host's AP configuration will be filtered.

- 鍒嗛厤缁欑煩闃?mdev 鐨勯€傞厤鍣?APID銆佸煙 APQI 鍜屾帶鍒跺煙鍚嶄腑锛岄偅浜涙湭鍚屾椂鍒嗛厤缁欎富鏈?AP 閰嶇疆鐨?
  閮ㄥ垎灏嗚杩囨护銆?

- Each APQN derived from the Cartesian product of the APIDs and APQIs assigned
  to the vfio_ap mdev is examined and if any one of them does not reference a
  queue device bound to the vfio_ap device driver, the adapter will not be
  plugged into the guest (i.e., the bit corresponding to its APID will not be
  set in the APM of the guest's APCB).

- 妫€鏌ヤ粠鍒嗛厤缁?vfio_ap mdev 鐨?APID 鍜?APQI 鐨勭瑳鍗″皵绉帹瀵煎嚭鐨勬瘡涓?APQN锛屽鏋滃叾涓?
  浠绘剰涓€涓笉寮曠敤缁戝畾鍒?vfio_ap 璁惧椹卞姩鐨勯槦鍒楄澶囷紝鍒欒閫傞厤鍣ㄥ皢涓嶄細琚彃鍏ュ鎴锋満
  锛堝嵆锛屽叾 APID 瀵瑰簲鐨勪綅涓嶄細琚缃湪瀹㈡埛鏈?APCB 鐨?APM 涓級銆?

### The CPU model features for AP

### 鐢ㄤ簬 AP 鐨?CPU 妯″瀷鐗规€?

The AP stack relies on the presence of the AP instructions as well as three
facilities: The AP Facilities Test (APFT) facility; the AP Query
Configuration Information (QCI) facility; and the AP Queue Interruption Control
facility. These features/facilities are made available to a KVM guest via the
following CPU model features:

AP 鍗忚鏍堜緷璧?AP 鎸囦护鐨勫瓨鍦ㄤ互鍙婁笁涓鏂斤細AP Facilities Test锛圓PFT锛夎鏂斤紱AP Query
Configuration Information锛圦CI锛夎鏂斤紱浠ュ強 AP Queue Interruption Control 璁炬柦銆傝繖浜?
鐗规€?璁炬柦閫氳繃浠ヤ笅 CPU 妯″瀷鐗规€ф彁渚涚粰 KVM 瀹㈡埛鏈猴細

1. ap: Indicates whether the AP instructions are installed on the guest. This
   feature will be enabled by KVM only if the AP instructions are installed
   on the host.

1. ap锛氭寚绀哄鎴锋満涓婃槸鍚﹀畨瑁呬簡 AP 鎸囦护銆備粎褰撲富鏈轰笂瀹夎浜?AP 鎸囦护鏃讹紝KVM 鎵嶄細鍚敤璇?
   鐗规€с€?

2. apft: Indicates the APFT facility is available on the guest. This facility
   can be made available to the guest only if it is available on the host (i.e.,
   facility bit 15 is set).

2. apft锛氭寚绀?APFT 璁炬柦鍦ㄥ鎴锋満涓婂彲鐢ㄣ€備粎褰撲富鏈轰笂鍙敤璇ヨ鏂芥椂锛堝嵆璁炬柦浣?15 琚疆浣嶏級锛?
   鎵嶈兘灏嗗叾鎻愪緵缁欏鎴锋満銆?

3. apqci: Indicates the AP QCI facility is available on the guest. This facility
   can be made available to the guest only if it is available on the host (i.e.,
   facility bit 12 is set).

3. apqci锛氭寚绀?AP QCI 璁炬柦鍦ㄥ鎴锋満涓婂彲鐢ㄣ€備粎褰撲富鏈轰笂鍙敤璇ヨ鏂芥椂锛堝嵆璁炬柦浣?12 琚疆浣嶏級锛?
   鎵嶈兘灏嗗叾鎻愪緵缁欏鎴锋満銆?

4. apqi: Indicates AP Queue Interruption Control faclity is available on the
   guest. This facility can be made available to the guest only if it is
   available on the host (i.e., facility bit 65 is set).

4. apqi锛氭寚绀?AP Queue Interruption Control 璁炬柦鍦ㄥ鎴锋満涓婂彲鐢ㄣ€備粎褰撲富鏈轰笂鍙敤璇ヨ鏂芥椂
   锛堝嵆璁炬柦浣?65 琚疆浣嶏級锛屾墠鑳藉皢鍏舵彁渚涚粰瀹㈡埛鏈恒€?

Note: If the user chooses to specify a CPU model different than the 'host'
model to QEMU, the CPU model features and facilities need to be turned on
```

     /usr/bin/qemu-system-s390x ... -cpu z13,ap=on,apqci=on,apft=on,apqi=on

```

A guest can be precluded from using AP features/facilities by turning them off
```

     /usr/bin/qemu-system-s390x ... -cpu host,ap=off,apqci=off,apft=off,apqi=off

```

Note: If the APFT facility is turned off (apft=off) for the guest, the guest
will not see any AP devices. The zcrypt device drivers on the guest that
register for type 10 and newer AP devices - i.e., the cex4card and cex4queue
device drivers - need the APFT facility to ascertain the facilities installed on
a given AP device. If the APFT facility is not installed on the guest, then no
adapter or domain devices will get created by the AP bus running on the
guest because only type 10 and newer devices can be configured for guest use.

娉ㄦ剰锛氬鏋滀负瀹㈡埛鏈哄叧闂簡 APFT 璁炬柦锛坅pft=off锛夛紝瀹㈡埛鏈哄皢鐪嬩笉鍒颁换浣?AP 璁惧銆傚鎴锋満
涓婃敞鍐岀敤浜庣被鍨?10 鍙婃洿鏂?AP 璁惧锛堝嵆 cex4card 鍜?cex4queue 璁惧椹卞姩锛夌殑 zcrypt 璁惧
椹卞姩闇€瑕?APFT 璁炬柦鏉ョ‘瀹氱粰瀹?AP 璁惧涓婂畨瑁呯殑璁炬柦銆傚鏋滃鎴锋満涓婃湭瀹夎 APFT 璁炬柦锛岄偅涔?
鍦ㄥ鎴锋満涓婅繍琛岀殑 AP 鎬荤嚎灏嗕笉浼氬垱寤轰换浣曢€傞厤鍣ㄦ垨鍩熻澶囷紝鍥犱负鍙兘涓虹被鍨?10 鍙婃洿鏂扮殑璁惧
閰嶇疆渚涘鎴锋満浣跨敤銆?

## Example

## 绀轰緥

Let's now provide an example to illustrate how KVM guests may be given
access to AP facilities. For this example, we will show how to configure
three guests such that executing the lszcrypt command on the guests would
look like this:

鐜板湪璁╂垜浠彁渚涗竴涓ず渚嬶紝璇存槑濡備綍鎺堜簣 KVM 瀹㈡埛鏈哄 AP 璁炬柦鐨勮闂潈闄愩€傚湪鏈ず渚嬩腑锛屾垜浠?
灏嗗睍绀哄浣曢厤缃笁涓鎴锋満锛屼娇寰楀湪瀹㈡埛鏈轰笂鎵ц lszcrypt 鍛戒护鏃舵樉绀哄涓嬪唴瀹癸細

### Guest1

### 瀹㈡埛鏈?

=========== ===== ============
CARD.DOMAIN TYPE  MODE
=========== ===== ============
05          CEX5C CCA-Coproc
05.0004     CEX5C CCA-Coproc
05.00ab     CEX5C CCA-Coproc
06          CEX5A Accelerator
06.0004     CEX5A Accelerator
06.00ab     CEX5A Accelerator
=========== ===== ============

### Guest2

### 瀹㈡埛鏈?

=========== ===== ============
CARD.DOMAIN TYPE  MODE
=========== ===== ============
05          CEX5C CCA-Coproc
05.0047     CEX5C CCA-Coproc
05.00ff     CEX5C CCA-Coproc
=========== ===== ============

### Guest3

### 瀹㈡埛鏈?

=========== ===== ============
CARD.DOMAIN TYPE  MODE
=========== ===== ============
06          CEX5A Accelerator
06.0047     CEX5A Accelerator
06.00ff     CEX5A Accelerator
=========== ===== ============

These are the steps:

姝ラ濡備笅锛?

1. Install the vfio_ap module on the linux host. The dependency chain for the
   vfio_ap module is:
   - iommu
   - s390
   - zcrypt
   - vfio
   - vfio_mdev
   - vfio_mdev_device
   - KVM

   To build the vfio_ap module, the kernel build must be configured with the
   following Kconfig elements selected:
   - IOMMU_SUPPORT
   - S390
   - AP
   - VFIO
   - KVM

1. 鍦?Linux 涓绘満涓婂畨瑁?vfio_ap 妯″潡銆倂fio_ap 妯″潡鐨勪緷璧栭摼涓猴細
   - iommu
   - s390
   - zcrypt
   - vfio
   - vfio_mdev
   - vfio_mdev_device
   - KVM

   瑕佹瀯寤?vfio_ap 妯″潡锛屽唴鏍告瀯寤哄繀椤婚厤缃负閫変腑浠ヤ笅 Kconfig 閫夐」锛?
   - IOMMU_SUPPORT
   - S390
   - AP
   - VFIO
   - KVM

```

     -> Device Drivers
	-> IOMMU Hardware Support
	   select S390 AP IOMMU Support
	-> VFIO Non-Privileged userspace driver framework
	   -> Mediated device driver frramework
	      -> VFIO driver for Mediated devices
     -> I/O subsystem
	-> VFIO support for AP devices

```

2. Secure the AP queues to be used by the three guests so that the host can not
   access them. To secure them, there are two sysfs files that specify
   bitmasks marking a subset of the APQN range as usable only by the default AP
   queue device drivers. All remaining APQNs are available for use by
   any other device driver. The vfio_ap device driver is currently the only
   non-default device driver. The location of the sysfs files containing the
```

     /sys/bus/ap/apmask
     /sys/bus/ap/aqmask

   The 'apmask' is a 256-bit mask that identifies a set of AP adapter IDs
   (APID). Each bit in the mask, from left to right, corresponds to an APID from
   0-255. If a bit is set, the APID belongs to the subset of APQNs marked as
   available only to the default AP queue device drivers.

   The 'aqmask' is a 256-bit mask that identifies a set of AP queue indexes
   (APQI). Each bit in the mask, from left to right, corresponds to an APQI from
   0-255. If a bit is set, the APQI belongs to the subset of APQNs marked as
   available only to the default AP queue device drivers.

   The Cartesian product of the APIDs corresponding to the bits set in the
   apmask and the APQIs corresponding to the bits set in the aqmask comprise
   the subset of APQNs that can be used only by the host default device drivers.
   All other APQNs are available to the non-default device drivers such as the
   vfio_ap driver.

   Take, for example, the following masks::

      apmask:
      0x7d00000000000000000000000000000000000000000000000000000000000000

      aqmask:
      0x8000000000000000000000000000000000000000000000000000000000000000

   The masks indicate:

   * Adapters 1, 2, 3, 4, 5, and 7 are available for use by the host default
     device drivers.

   * Domain 0 is available for use by the host default device drivers

   * The subset of APQNs available for use only by the default host device
     drivers are:

     (1,0), (2,0), (3,0), (4.0), (5,0) and (7,0)

   * All other APQNs are available for use by the non-default device drivers.

   The APQN of each AP queue device assigned to the linux host is checked by the
   AP bus against the set of APQNs derived from the Cartesian product of APIDs
   and APQIs marked as available to the default AP queue device drivers. If a
   match is detected,  only the default AP queue device drivers will be probed;
   otherwise, the vfio_ap device driver will be probed.

   By default, the two masks are set to reserve all APQNs for use by the default
   AP queue device drivers. There are two ways the default masks can be changed:

   1. The sysfs mask files can be edited by echoing a string into the
      respective sysfs mask file in one of two formats:

      * An absolute hex string starting with 0x - like "0x12345678" - sets
	the mask. If the given string is shorter than the mask, it is padded
	with 0s on the right; for example, specifying a mask value of 0x41 is
	the same as specifying::

	   0x4100000000000000000000000000000000000000000000000000000000000000

	Keep in mind that the mask reads from left to right, so the mask
	above identifies device numbers 1 and 7 (01000001).

	If the string is longer than the mask, the operation is terminated with
	an error (EINVAL).

      * Individual bits in the mask can be switched on and off by specifying
	each bit number to be switched in a comma separated list. Each bit
	number string must be prepended with a ('+') or minus ('-') to indicate
	the corresponding bit is to be switched on ('+') or off ('-'). Some
	valid values are:

	   - "+0"    switches bit 0 on
	   - "-13"   switches bit 13 off
	   - "+0x41" switches bit 65 on
	   - "-0xff" switches bit 255 off

	The following example:

	      +0,-6,+0x47,-0xf0

	Switches bits 0 and 71 (0x47) on

	Switches bits 6 and 240 (0xf0) off

	Note that the bits not specified in the list remain as they were before
	the operation.

   2. The masks can also be changed at boot time via parameters on the kernel
      command line like this:

	 ap.apmask=0xffff ap.aqmask=0x40

	 This would create the following masks::

	    apmask:
	    0xffff000000000000000000000000000000000000000000000000000000000000

	    aqmask:
	    0x4000000000000000000000000000000000000000000000000000000000000000

	 Resulting in these two pools::

	    default drivers pool:    adapter 0-15, domain 1
	    alternate drivers pool:  adapter 16-255, domains 0, 2-255

   **Note:**
   Changing a mask such that one or more APQNs will be taken from a vfio_ap
   mediated device (see below) will fail with an error (EBUSY). A message
   is logged to the kernel ring buffer which can be viewed with the 'dmesg'
   command. The output identifies each APQN flagged as 'in use' and identifies
   the vfio_ap mediated device to which it is assigned; for example:

   Userspace may not re-assign queue 05.0054 already assigned to 62177883-f1bb-47f0-914d-32a22e3a8804
   Userspace may not re-assign queue 04.0054 already assigned to cef03c3c-903d-4ecc-9a83-40694cb8aee4

```

### Securing the APQNs for our example

### 涓烘垜浠殑绀轰緥棰勭暀 APQN

   To secure the AP queues 05.0004, 05.0047, 05.00ab, 05.00ff, 06.0004, 06.0047,
   06.00ab, and 06.00ff for use by the vfio_ap device driver, the corresponding
   APQNs can be removed from the default masks using either of the following
```

      echo -5,-6 > /sys/bus/ap/apmask

      echo -4,-0x47,-0xab,-0xff > /sys/bus/ap/aqmask

   Or the masks can be set as follows::

      echo 0xf9ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff \
      > apmask

      echo 0xf7fffffffffffffffeffffffffffffffffffffffffeffffffffffffffffffffe \
      > aqmask

   This will result in AP queues 05.0004, 05.0047, 05.00ab, 05.00ff, 06.0004,
   06.0047, 06.00ab, and 06.00ff getting bound to the vfio_ap device driver. The
   sysfs directory for the vfio_ap device driver will now contain symbolic links
   to the AP queue devices bound to it::

     /sys/bus/ap
     ... [drivers]
     ...... [vfio_ap]
     ......... [05.0004]
     ......... [05.0047]
     ......... [05.00ab]
     ......... [05.00ff]
     ......... [06.0004]
     ......... [06.0047]
     ......... [06.00ab]
     ......... [06.00ff]

   Keep in mind that only type 10 and newer adapters (i.e., CEX4 and later)
   can be bound to the vfio_ap device driver. The reason for this is to
   simplify the implementation by not needlessly complicating the design by
   supporting older devices that will go out of service in the relatively near
   future and for which there are few older systems on which to test.

   The administrator, therefore, must take care to secure only AP queues that
   can be bound to the vfio_ap device driver. The device type for a given AP
   queue device can be read from the parent card's sysfs directory. For example,
   to see the hardware type of the queue 05.0004:

     cat /sys/bus/ap/devices/card05/hwtype

   The hwtype must be 10 or higher (CEX4 or newer) in order to be bound to the
   vfio_ap device driver.

```

3. Create the mediated devices needed to configure the AP matrixes for the
   three guests and to provide an interface to the vfio_ap driver for
```

     /sys/devices/vfio_ap/matrix/
     --- [mdev_supported_types]
     ------ [vfio_ap-passthrough] (passthrough vfio_ap mediated device type)
     --------- create
     --------- [devices]

   To create the mediated devices for the three guests::

	uuidgen > create
	uuidgen > create
	uuidgen > create

	or

	echo $uuid1 > create
	echo $uuid2 > create
	echo $uuid3 > create

   This will create three mediated devices in the [devices] subdirectory named
   after the UUID written to the create attribute file. We call them $uuid1,
   $uuid2 and $uuid3 and this is the sysfs directory structure after creation::

     /sys/devices/vfio_ap/matrix/
     --- [mdev_supported_types]
     ------ [vfio_ap-passthrough]
     --------- [devices]
     ------------ [$uuid1]
     --------------- assign_adapter
     --------------- assign_control_domain
     --------------- assign_domain
     --------------- matrix
     --------------- unassign_adapter
     --------------- unassign_control_domain
     --------------- unassign_domain

     ------------ [$uuid2]
     --------------- assign_adapter
     --------------- assign_control_domain
     --------------- assign_domain
     --------------- matrix
     --------------- unassign_adapter
     ----------------unassign_control_domain
     ----------------unassign_domain

     ------------ [$uuid3]
     --------------- assign_adapter
     --------------- assign_control_domain
     --------------- assign_domain
     --------------- matrix
     --------------- unassign_adapter
     ----------------unassign_control_domain
     ----------------unassign_domain

   Note *****: The vfio_ap mdevs do not persist across reboots unless the
               mdevctl tool is used to create and persist them.

```

4. The administrator now needs to configure the matrixes for the mediated
   devices $uuid1 (for Guest1), $uuid2 (for Guest2) and $uuid3 (for Guest3).

4. 绠＄悊鍛樼幇鍦ㄩ渶瑕佷负涓粙璁惧 $uuid1锛堢敤浜庡鎴锋満1锛夈€?uuid2锛堢敤浜庡鎴锋満2锛夊拰 $uuid3
   锛堢敤浜庡鎴锋満3锛夐厤缃煩闃点€?

```

      echo 5 > assign_adapter
      echo 6 > assign_adapter
      echo 4 > assign_domain
      echo 0xab > assign_domain

   Control domains can similarly be assigned using the assign_control_domain
   sysfs file.

   If a mistake is made configuring an adapter, domain or control domain,
   you can use the unassign_xxx files to unassign the adapter, domain or
   control domain.

   To display the matrix configuration for Guest1::

	 cat matrix

   To display the matrix that is or will be assigned to Guest1::

	 cat guest_matrix

   This is how the matrix is configured for Guest2::

      echo 5 > assign_adapter
      echo 0x47 > assign_domain
      echo 0xff > assign_domain

   This is how the matrix is configured for Guest3::

      echo 6 > assign_adapter
      echo 0x47 > assign_domain
      echo 0xff > assign_domain

   In order to successfully assign an adapter:

   * The adapter number specified must represent a value from 0 up to the
     maximum adapter number configured for the system. If an adapter number
     higher than the maximum is specified, the operation will terminate with
     an error (ENODEV).

     Note: The maximum adapter number can be obtained via the sysfs
	   /sys/bus/ap/ap_max_adapter_id attribute file.

   * Each APQN derived from the Cartesian product of the APID of the adapter
     being assigned and the APQIs of the domains previously assigned:

     - Must only be available to the vfio_ap device driver as specified in the
       sysfs /sys/bus/ap/apmask and /sys/bus/ap/aqmask attribute files. If even
       one APQN is reserved for use by the host device driver, the operation
       will terminate with an error (EADDRNOTAVAIL).

     - Must NOT be assigned to another vfio_ap mediated device. If even one APQN
       is assigned to another vfio_ap mediated device, the operation will
       terminate with an error (EBUSY).

     - Must NOT be assigned while the sysfs /sys/bus/ap/apmask and
       /sys/bus/ap/aqmask attribute files are being edited or the operation may
       terminate with an error (EBUSY).

   In order to successfully assign a domain:

   * The domain number specified must represent a value from 0 up to the
     maximum domain number configured for the system. If a domain number
     higher than the maximum is specified, the operation will terminate with
     an error (ENODEV).

     Note: The maximum domain number can be obtained via the sysfs
	   /sys/bus/ap/ap_max_domain_id attribute file.

    * Each APQN derived from the Cartesian product of the APQI of the domain
      being assigned and the APIDs of the adapters previously assigned:

     - Must only be available to the vfio_ap device driver as specified in the
       sysfs /sys/bus/ap/apmask and /sys/bus/ap/aqmask attribute files. If even
       one APQN is reserved for use by the host device driver, the operation
       will terminate with an error (EADDRNOTAVAIL).

     - Must NOT be assigned to another vfio_ap mediated device. If even one APQN
       is assigned to another vfio_ap mediated device, the operation will
       terminate with an error (EBUSY).

     - Must NOT be assigned while the sysfs /sys/bus/ap/apmask and
       /sys/bus/ap/aqmask attribute files are being edited or the operation may
       terminate with an error (EBUSY).

   In order to successfully assign a control domain:

   * The domain number specified must represent a value from 0 up to the maximum
     domain number configured for the system. If a control domain number higher
     than the maximum is specified, the operation will terminate with an
     error (ENODEV).

```

```

     /usr/bin/qemu-system-s390x ... -cpu host,ap=on,apqci=on,apft=on,apqi=on \
	-device vfio-ap,sysfsdev=/sys/devices/vfio_ap/matrix/$uuid1 ...

```

```

     /usr/bin/qemu-system-s390x ... -cpu host,ap=on,apqci=on,apft=on,apqi=on \
	-device vfio-ap,sysfsdev=/sys/devices/vfio_ap/matrix/$uuid2 ...

```

```

     /usr/bin/qemu-system-s390x ... -cpu host,ap=on,apqci=on,apft=on,apqi=on \
	-device vfio-ap,sysfsdev=/sys/devices/vfio_ap/matrix/$uuid3 ...

```

When the guest is shut down, the vfio_ap mediated devices may be removed.

褰撳鎴锋満鍏虫満鏃讹紝vfio_ap 涓粙璁惧鍙绉婚櫎銆?

```

   /sys/devices/vfio_ap/matrix/
      --- [mdev_supported_types]
      ------ [vfio_ap-passthrough]
      --------- [devices]
      ------------ [$uuid1]
      --------------- remove

```

```

   echo 1 > remove

```

This will remove all of the matrix mdev device's sysfs structures including
the mdev device itself. To recreate and reconfigure the matrix mdev device,
all of the steps starting with step 3 will have to be performed again. Note
that the remove will fail if a guest using the vfio_ap mdev is still running.

杩欏皢绉婚櫎鐭╅樀 mdev 璁惧鐨勬墍鏈?sysfs 缁撴瀯锛屽寘鎷?mdev 璁惧鏈韩銆傝閲嶆柊鍒涘缓骞堕噸鏂伴厤缃?
鐭╅樀 mdev 璁惧锛屽繀椤婚噸鏂版墽琛屼粠姝ラ 3 寮€濮嬬殑鎵€鏈夋楠ゃ€傛敞鎰忥紝濡傛灉姝ｅ湪浣跨敤 vfio_ap mdev
鐨勫鎴锋満浠嶅湪杩愯锛宺emove 灏嗗け璐ャ€?

It is not necessary to remove a vfio_ap mdev, but one may want to
remove it if no guest will use it during the remaining lifetime of the linux
host. If the vfio_ap mdev is removed, one may want to also reconfigure
the pool of adapters and queues reserved for use by the default drivers.

骞堕潪蹇呴』绉婚櫎涓€涓?vfio_ap mdev锛屼絾濡傛灉鍦ㄥ叾浣欑殑 Linux 涓绘満鐢熷懡鍛ㄦ湡鍐呮病鏈夊鎴锋満浼氫娇鐢?
瀹冿紝鍙兘浼氭兂瑕佺Щ闄ゅ畠銆傚鏋滅Щ闄や簡 vfio_ap mdev锛屽彲鑳借繕鎯宠閲嶆柊閰嶇疆涓洪粯璁ら┍鍔ㄩ鐣欑殑
閫傞厤鍣ㄥ拰闃熷垪姹犮€?

## Hot plug/unplug support:

## 鐑彃鎷旀敮鎸侊細

An adapter, domain or control domain may be hot plugged into a running KVM
guest by assigning it to the vfio_ap mediated device being used by the guest if
the following conditions are met:

鍦ㄦ弧瓒充互涓嬫潯浠舵椂锛屽彲浠ラ€氳繃灏嗛€傞厤鍣ㄣ€佸煙鎴栨帶鍒跺煙鍒嗛厤缁欏鎴锋満姝ｅ湪浣跨敤鐨?vfio_ap 涓粙
璁惧锛屽皢鍏剁儹鎻掑叆鍒颁竴涓鍦ㄨ繍琛岀殑 KVM 瀹㈡埛鏈猴細

- The adapter, domain or control domain must also be assigned to the host's
  AP configuration.

- 璇ラ€傞厤鍣ㄣ€佸煙鎴栨帶鍒跺煙杩樺繀椤诲凡鍒嗛厤缁欎富鏈虹殑 AP 閰嶇疆銆?

- Each APQN derived from the Cartesian product comprised of the APID of the
  adapter being assigned and the APQIs of the domains assigned must reference a
  queue device bound to the vfio_ap device driver.

- 鐢辫鍒嗛厤閫傞厤鍣ㄧ殑 APID 涓庡凡鍒嗛厤鍩熺殑 APQI 缁勬垚鐨勭瑳鍗″皵绉帹瀵煎嚭鐨勬瘡涓?APQN 蹇呴』寮曠敤
  涓€涓粦瀹氬埌 vfio_ap 璁惧椹卞姩鐨勯槦鍒楄澶囥€?

- To hot plug a domain, each APQN derived from the Cartesian product
  comprised of the APQI of the domain being assigned and the APIDs of the
  adapters assigned must reference a queue device bound to the vfio_ap device
  driver.

- 瑕佺儹鎻掓嫈涓€涓煙锛岀敱琚垎閰嶅煙鐨?APQI 涓庡凡鍒嗛厤閫傞厤鍣ㄧ殑 APID 缁勬垚鐨勭瑳鍗″皵绉帹瀵煎嚭鐨勬瘡涓?
  APQN 蹇呴』寮曠敤涓€涓粦瀹氬埌 vfio_ap 璁惧椹卞姩鐨勯槦鍒楄澶囥€?

An adapter, domain or control domain may be hot unplugged from a running KVM
guest by unassigning it from the vfio_ap mediated device being used by the
guest.

鍙互閫氳繃灏嗛€傞厤鍣ㄣ€佸煙鎴栨帶鍒跺煙浠庡鎴锋満姝ｅ湪浣跨敤鐨?vfio_ap 涓粙璁惧瑙ｉ櫎鍒嗛厤锛屽皢鍏朵粠姝ｅ湪
杩愯鐨?KVM 瀹㈡埛鏈虹儹鎷斿嚭銆?

## Over-provisioning of AP queues for a KVM guest:

## 涓?KVM 瀹㈡埛鏈鸿繃搴﹂厤缃?AP 闃熷垪锛?

Over-provisioning is defined herein as the assignment of adapters or domains to
a vfio_ap mediated device that do not reference AP devices in the host's AP
configuration. The idea here is that when the adapter or domain becomes
available, it will be automatically hot-plugged into the KVM guest using
the vfio_ap mediated device to which it is assigned as long as each new APQN
resulting from plugging it in references a queue device bound to the vfio_ap
device driver.

姝ゅ灏嗚繃搴﹂厤缃紙Over-provisioning锛夊畾涔変负灏嗕笉寮曠敤涓绘満 AP 閰嶇疆涓?AP 璁惧鐨勯€傞厤鍣ㄦ垨鍩?
鍒嗛厤缁?vfio_ap 涓粙璁惧銆傝繖閲岀殑鎯虫硶鏄紝褰撻€傞厤鍣ㄦ垨鍩熷彉寰楀彲鐢ㄦ椂锛屽彧瑕佹彃鍏ュ畠鎵€浜х敓鐨勬瘡涓?
鏂?APQN 閮藉紩鐢ㄤ竴涓粦瀹氬埌 vfio_ap 璁惧椹卞姩鐨勯槦鍒楄澶囷紝瀹冨氨浼氳鑷姩鐑彃鍏ュ埌鍒嗛厤缁欏畠鐨?
vfio_ap 涓粙璁惧鎵€鍦ㄧ殑 KVM 瀹㈡埛鏈恒€?

## Driver Features

## 椹卞姩鐗规€?

The vfio_ap driver exposes a sysfs file containing supported features.
This exists so third party tools (like Libvirt and mdevctl) can query the
availability of specific features.

vfio_ap 椹卞姩鏆撮湶涓€涓寘鍚墍鏀寔鐗规€х殑 sysfs 鏂囦欢銆傚畠鐨勫瓨鍦ㄦ槸涓轰簡璁╃涓夋柟宸ュ叿锛堝
Libvirt 鍜?mdevctl锛夎兘澶熸煡璇㈢壒瀹氱壒鎬х殑鍙敤鎬с€?

The features list can be found here: /sys/bus/matrix/devices/matrix/features

鐗规€у垪琛ㄥ彲鍦ㄦ澶勬壘鍒帮細/sys/bus/matrix/devices/matrix/features

Entries are space delimited. Each entry consists of a combination of
alphanumeric and underscore characters.

鍚勯」浠ョ┖鏍煎垎闅斻€傛瘡涓€椤圭敱瀛楁瘝鏁板瓧鍜屼笅鍒掔嚎瀛楃鐨勭粍鍚堟瀯鎴愩€?

Example:
cat /sys/bus/matrix/devices/matrix/features
guest_matrix dyn ap_config

绀轰緥锛?
cat /sys/bus/matrix/devices/matrix/features
guest_matrix dyn ap_config

the following features are advertised:

灏嗛€氬憡浠ヤ笅鐗规€э細

---------------+---------------------------------------------------------------+
| Flag         | Description                                                   |
+==============+===============================================================+
| guest_matrix | guest_matrix attribute exists. It reports the matrix of       |
|              | adapters and domains that are or will be passed through to a  |
|              | guest when the mdev is attached to it.                        |
+--------------+---------------------------------------------------------------+
| dyn          | Indicates hot plug/unplug of AP adapters, domains and control |
|              | domains for a guest to which the mdev is attached.            |
+------------+-----------------------------------------------------------------+
| ap_config    | ap_config interface for one-shot modifications to mdev config |
+--------------+---------------------------------------------------------------+

---------------+---------------------------------------------------------------+
| 鏍囧織         | 鎻忚堪                                                          |
+==============+===============================================================+
| guest_matrix | guest_matrix 灞炴€у瓨鍦ㄣ€傚畠鎶ュ憡褰?mdev 闄勫姞鍒板鎴锋満鏃讹紝姝ｅ湪鎴?|
|              | 灏嗚鐩撮€氱粰璇ュ鎴锋満鐨勯€傞厤鍣ㄥ拰鍩熺殑鐭╅樀銆?                      |
+--------------+---------------------------------------------------------------+
| dyn          | 鎸囩ず鐑彃鎷旈檮鍔犱簡 mdev 鐨勫鎴锋満鐨?AP 閫傞厤鍣ㄣ€佸煙鍜屾帶鍒跺煙銆?  |
+------------+-----------------------------------------------------------------+
| ap_config    | 鐢ㄤ簬涓€娆℃€т慨鏀?mdev 閰嶇疆鐨?ap_config 鎺ュ彛銆?                 |
+--------------+---------------------------------------------------------------+

## Limitations

## 闄愬埗

Live guest migration is not supported for guests using AP devices without
intervention by a system administrator. Before a KVM guest can be migrated,
the vfio_ap mediated device must be removed. Unfortunately, it can not be
removed manually (i.e., echo 1 > /sys/devices/vfio_ap/matrix/$UUID/remove) while
the mdev is in use by a KVM guest. If the guest is being emulated by QEMU,
its mdev can be hot unplugged from the guest in one of two ways:

瀵逛簬浣跨敤 AP 璁惧鐨勫鎴锋満锛屼笉鏀寔鍦ㄧ郴缁熺鐞嗗憳涓嶅共棰勭殑鎯呭喌涓嬭繘琛屽疄鏃跺鎴锋満杩佺Щ锛坙ive
guest migration锛夈€傚湪 KVM 瀹㈡埛鏈鸿兘澶熻杩佺Щ涔嬪墠锛屽繀椤荤Щ闄?vfio_ap 涓粙璁惧銆傞仐鎲剧殑鏄紝
褰?mdev 姝ｈ KVM 瀹㈡埛鏈轰娇鐢ㄦ椂锛屾棤娉曟墜鍔ㄧЩ闄ゅ畠锛堝嵆 echo 1 >
/sys/devices/vfio_ap/matrix/$UUID/remove锛夈€傚鏋滃鎴锋満鐢?QEMU 妯℃嫙锛屽垯鍏?mdev 鍙互閫氳繃浠ヤ笅
涓ょ鏂瑰紡涔嬩竴浠庡鎴锋満鐑嫈鍑猴細

1. If the KVM guest was started with libvirt, you can hot unplug the mdev via
   the following commands:

1. 濡傛灉 KVM 瀹㈡埛鏈烘槸鐢?libvirt 鍚姩鐨勶紝鍙互閫氳繃浠ヤ笅鍛戒护鐑嫈鍑?mdev锛?

      virsh detach-device <guestname> <path-to-device-xml>

      For example, to hot unplug mdev 62177883-f1bb-47f0-914d-32a22e3a8804 from
      the guest named 'my-guest':

      渚嬪锛岃浠庡悕涓?'my-guest' 鐨勫鎴锋満鐑嫈鍑?mdev 62177883-f1bb-47f0-914d-32a22e3a8804锛?

         virsh detach-device my-guest ~/config/my-guest-hostdev.xml

            The contents of my-guest-hostdev.xml:


            <hostdev mode='subsystem' type='mdev' managed='no' model='vfio-ap'>
              <source>
                <address uuid='62177883-f1bb-47f0-914d-32a22e3a8804'/>
              </source>
            </hostdev>


      virsh qemu-monitor-command <guest-name> --hmp "device-del <device-id>"

      For example, to hot unplug the vfio_ap mediated device identified on the
      qemu command line with 'id=hostdev0' from the guest named 'my-guest':


         virsh qemu-monitor-command my-guest --hmp "device_del hostdev0"

2. A vfio_ap mediated device can be hot unplugged by attaching the qemu monitor
   to the guest and using the following qemu monitor command:

2. 鍙互閫氳繃灏?qemu monitor 杩炴帴鍒板鎴锋満骞朵娇鐢ㄤ互涓?qemu monitor 鍛戒护鏉ョ儹鎷斿嚭 vfio_ap
   涓粙璁惧锛?

      (QEMU) device-del id=<device-id>

      For example, to hot unplug the vfio_ap mediated device that was specified
      on the qemu command line with 'id=hostdev0' when the guest was started:


         (QEMU) device-del id=hostdev0

After live migration of the KVM guest completes, an AP configuration can be
restored to the KVM guest by hot plugging a vfio_ap mediated device on the target
system into the guest in one of two ways:

鍦?KVM 瀹㈡埛鏈哄疄鏃惰縼绉诲畬鎴愬悗锛屽彲浠ラ€氳繃鍦ㄧ洰鏍囩郴缁熶笂灏?vfio_ap 涓粙璁惧鐑彃鍏ュ埌瀹㈡埛鏈?
鏉ユ仮澶嶅叾 AP 閰嶇疆锛屾湁涓ょ鏂瑰紡锛?

1. If the KVM guest was started with libvirt, you can hot plug a matrix mediated
   device into the guest via the following virsh commands:

1. 濡傛灉 KVM 瀹㈡埛鏈烘槸鐢?libvirt 鍚姩鐨勶紝鍙互閫氳繃浠ヤ笅 virsh 鍛戒护灏嗙煩闃典腑浠嬭澶囩儹鎻掑叆
   瀹㈡埛鏈猴細

   virsh attach-device <guestname> <path-to-device-xml>

      For example, to hot plug mdev 62177883-f1bb-47f0-914d-32a22e3a8804 into
      the guest named 'my-guest':

      渚嬪锛岃灏?mdev 62177883-f1bb-47f0-914d-32a22e3a8804 鐑彃鍏ュ悕涓?'my-guest' 鐨?
      瀹㈡埛鏈猴細

         virsh attach-device my-guest ~/config/my-guest-hostdev.xml

            The contents of my-guest-hostdev.xml:


            <hostdev mode='subsystem' type='mdev' managed='no' model='vfio-ap'>
              <source>
                <address uuid='62177883-f1bb-47f0-914d-32a22e3a8804'/>
              </source>
            </hostdev>


   virsh qemu-monitor-command <guest-name> --hmp \
   "device_add vfio-ap,sysfsdev=<path-to-mdev>,id=<device-id>"

      For example, to hot plug the vfio_ap mediated device
      62177883-f1bb-47f0-914d-32a22e3a8804 into the guest named 'my-guest' with
      device-id hostdev0:

      virsh qemu-monitor-command my-guest --hmp \
      "device_add vfio-ap,\
      sysfsdev=/sys/devices/vfio_ap/matrix/62177883-f1bb-47f0-914d-32a22e3a8804,\
      id=hostdev0"

2. A vfio_ap mediated device can be hot plugged by attaching the qemu monitor
   to the guest and using the following qemu monitor command:

2. 鍙互閫氳繃灏?qemu monitor 杩炴帴鍒板鎴锋満骞朵娇鐢ㄤ互涓?qemu monitor 鍛戒护鏉ョ儹鎻掑叆 vfio_ap
   涓粙璁惧锛?

      (qemu) device_add "vfio-ap,sysfsdev=<path-to-mdev>,id=<device-id>"

      For example, to plug the vfio_ap mediated device
      62177883-f1bb-47f0-914d-32a22e3a8804 into the guest with the device-id
      hostdev0:


         (QEMU) device-add "vfio-ap,\
         sysfsdev=/sys/devices/vfio_ap/matrix/62177883-f1bb-47f0-914d-32a22e3a8804,\
         id=hostdev0"