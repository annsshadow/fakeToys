
## 鍐呮牳涓?HYP 涔嬮棿鐨勫唴閮?ABI


鏈枃浠惰褰曚簡 Linux 鍐呮牳涓?hypervisor 灞傚湪灏?Linux 浣滀负 hypervisor 杩愯锛堜緥濡?KVM锛夋椂鐨勪氦浜掋€傚畠涓嶆兜鐩栧唴鏍镐綔涓哄鎴锋満锛堝湪 Xen銆並VM 鎴栦换浣曞叾浠?hypervisor 涔嬩笅锛夎繍琛屾椂涓?hypervisor 鐨勪氦浜掞紝涔熶笉娑电洊鍐呮牳浣滀负瀹夸富鏃朵换浣?hypervisor 鐗规湁鐨勪氦浜掋€?
娉ㄦ剰锛欿VM/arm 宸蹭粠鍐呮牳涓Щ闄ゃ€備絾姝ゅ鎻忚堪鐨?API 浠嶇劧鏈夋晥锛屽洜涓哄畠鍏佽鍐呮牳鍦ㄤ互 HYP 妯″紡鍚姩鏃惰繘琛?kexec銆傚鏈夊繀瑕侊紝闈?KVM 鐨?hypervisor 涔熷彲浠ヤ娇鐢ㄥ畠銆?
鍦?arm 鍜?arm64锛堟棤 VHE锛変笂锛屽唴鏍稿苟涓嶈繍琛屽湪 hypervisor 妯″紡涓嬶紝浣嗕粛闇€瑕佷笌涔嬩氦浜掞紝浠ヤ究瀹夎鎴栨媶闄ゅ唴缃殑 hypervisor銆?
涓轰簡瀹炵幇杩欎竴鐐癸紝鍐呮牳蹇呴』鍦?HYP锛坅rm锛夋垨 EL2锛坅rm64锛変笅鍚姩锛屼粠鑰岃兘澶熷湪鍒囧叆 SVC/EL1 涔嬪墠瀹夎涓€缁勬々鍑芥暟锛坰tub锛夈€傝繖浜涙々鍑芥暟鍙€氳繃 `hvc #0` 鎸囦护璁块棶锛屽苟涓斾粎浣滅敤浜庡崟涓?CPU銆?
闄ら潪鍙︽湁璇存槑锛屼换浣曞唴缃?hypervisor 閮藉繀椤诲疄鐜颁互涓嬪嚱鏁帮紙鍙傝 arch/arm{,64}/include/asm/virt.h锛夛細

```

    r0/x0 = HVC_SET_VECTORS
    r1/x1 = vectors

  Set HVBAR/VBAR_EL2 to 'vectors' to enable a hypervisor. 'vectors'
  must be a physical address, and respect the alignment requirements
  of the architecture. Only implemented by the initial stubs, not by
  Linux hypervisors.

```
```

    r0/x0 = HVC_RESET_VECTORS

  Turn HYP/EL2 MMU off, and reset HVBAR/VBAR_EL2 to the initials
  stubs' exception vector value. This effectively disables an existing
  hypervisor.

```
```

    r0/x0 = HVC_SOFT_RESTART
    r1/x1 = restart address
    x2 = x0's value when entering the next payload (arm64)
    x3 = x1's value when entering the next payload (arm64)
    x4 = x2's value when entering the next payload (arm64)

  Mask all exceptions, disable the MMU, clear I+D bits, move the arguments
  into place (arm64 only), and jump to the restart address while at HYP/EL2.
  This hypercall is not expected to return to its caller.

```
```

    x0 = HVC_FINALISE_EL2 (arm64 only)

  Finish configuring EL2 depending on the command-line options,
  including an attempt to upgrade the kernel's exception level from
  EL1 to EL2 by enabling the VHE mode. This is conditioned by the CPU
  supporting VHE, the EL2 MMU being off, and VHE not being disabled by
  any other means (command line option, for example).

```
r0/x0 鐨勪换浣曞叾浠栧彇鍊间細瑙﹀彂 hypervisor 鐗规湁鐨勫鐞嗭紝姝ゅ涓嶄簣璁板綍銆?
妗╁嚱鏁?hypercall 鐨勮繑鍥炲€肩敱 r0/x0 淇濆瓨锛屾垚鍔熸椂涓?0锛屽嚭閿欐椂涓?HVC_STUB_ERR銆傛々鍑芥暟 hypercall 鍏佽鐮村潖浠讳綍璋冪敤鑰呬繚瀛樼殑瀵勫瓨鍣紙arm64 涓婁负 x0-x18锛宎rm 涓婁负 r0-r3 鍜?ip锛夈€傚洜姝ゅ缓璁娇鐢ㄥ嚱鏁拌皟鐢ㄦ潵鎵ц璇?hypercall銆?