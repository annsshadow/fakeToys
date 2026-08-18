
## 浜插拰鎬у彈绠′腑鏂?


IRQ 鏍稿績鎻愪緵鏍规嵁鎸囧畾 CPU 浜插拰鎬ф潵绠＄悊涓柇鐨勬敮鎸併€傚湪姝ｅ父鎿嶄綔涓紝涓€涓腑鏂笌鏌愪釜鐗瑰畾
CPU 鍏宠仈銆傚鏋滆 CPU 绂荤嚎锛屼腑鏂細琚縼绉诲埌鍙︿竴涓湪绾?CPU銆?

鍏锋湁澶ч噺涓柇鍚戦噺鐨勮澶囦細鍗犵敤鍙敤鐨勫悜閲忕┖闂淬€備緥濡傦紝鍦ㄨ嚦灏戞湁 128 涓?CPU 鐨勭郴缁熶笂锛屼竴涓?
鍏锋湁 128 涓?I/O 闃熷垪鐨?NVMe 璁惧閫氬父姣忎釜闃熷垪璇锋眰涓€涓腑鏂€傚洜姝や袱涓繖鏍风殑璁惧璇锋眰 256
涓腑鏂€傚湪 x86 涓婏紝涓柇鍚戦噺绌洪棿 notoriously 寰堜綆锛屾瘡涓?CPU 浠呮彁渚?256 涓悜閲忥紝涓斿唴鏍镐繚鐣?
浜嗗叾涓竴閮ㄥ垎锛岃繘涓€姝ュ噺灏戜簡鍙敤浜庤澶囦腑鏂殑鏁伴噺銆傚湪瀹炶返涓繖涓嶆槸闂锛屽洜涓轰腑鏂鍒嗗竷鍒?
璁稿 CPU 涓婏紝鍥犳姣忎釜 CPU 鍙帴鏀跺皯閲忓悜閲忋€?

鐒惰€岋紝鍦ㄧ郴缁熸寕璧锋湡闂达紝鎵€鏈夋绾?CPU 閮界绾匡紝鎵€鏈変腑鏂兘琚縼绉诲埌鍞竴鍦ㄧ嚎鐨?CPU銆傝繖鍙兘
鑰楀敖璇?CPU 涓婂彲鐢ㄧ殑涓柇鍚戦噺锛屽苟瀵艰嚧鎸傝捣鎿嶄綔澶辫触銆?

浜插拰鎬у彈绠′腑鏂В鍐充簡杩欎竴闄愬埗銆傛瘡涓腑鏂璧嬩竴涓?CPU 浜插拰鎬ф帺鐮侊紝鎸囧畾璇ヤ腑鏂彲浠ヨ瀹氬悜鍒扮殑
CPU 闆嗗悎銆傚綋鎺╃爜涓殑涓€涓?CPU 绂荤嚎鏃讹紝涓柇琚Щ鍔ㄥ埌鎺╃爜涓殑涓嬩竴涓?CPU銆傚鏋滄帺鐮佷腑鐨勬渶鍚庝竴涓?
CPU 绂荤嚎锛岃涓柇琚叧闂€備娇鐢ㄤ翰鍜屾€у彈绠′腑鏂殑椹卞姩蹇呴』纭繚鍦ㄤ腑鏂绂佺敤涔嬪墠鐩稿叧鐨勯槦鍒楀凡闈欐锛?
浠ュ厤浜х敓杩涗竴姝ョ殑涓柇銆傚綋浜插拰鎬ф帺鐮佷腑鐨勪竴涓?CPU 閲嶆柊涓婄嚎鏃讹紝璇ヤ腑鏂閲嶆柊鍚敤銆?

### 瀹炵幇


璁惧蹇呴』鎻愪緵姣忓疄渚嬩腑鏂紝渚嬪 NVMe 杩欑被瀛樺偍璁惧鐨勬瘡 I/O 闃熷垪涓柇銆傞┍鍔ㄤ娇鐢?struct
irq_affinity 鍒嗛厤鍏锋湁鎵€闇€浜插拰鎬ц缃殑涓柇鍚戦噺銆傚浜?MSI-X 璁惧锛岃繖鏄€氳繃甯︽湁
PCI_IRQ_AFFINITY 鏍囧織鐨?pci_alloc_irq_vectors_affinity() 瀹屾垚鐨勩€?

鍩轰簬鎻愪緵鐨勪翰鍜屾€т俊鎭紝IRQ 鏍稿績灏濊瘯鎶婁腑鏂潎鍖€鍦版暎甯冨埌鏁翠釜绯荤粺銆備翰鍜屾€ф帺鐮佸湪杩欎竴鍒嗛厤姝ラ涓?
璁＄畻锛屼絾鏈€缁堢殑 IRQ 鍒嗛厤鏄湪璋冪敤 request_irq() 鏃舵墽琛岀殑銆?

### 闅旂鐨?CPU


鍙楃涓柇鐨勪翰鍜屾€у畬鍏ㄥ湪鍐呮牳涓鐞嗭紝鏃犳硶閫氳繃 /proc 鎺ュ彛浠庣敤鎴风┖闂翠慨鏀广€俰solcpus 鍚姩閫夐」鐨?
managed_irq 瀛愬弬鏁版寚瀹氫竴涓?CPU 鎺╃爜锛屽彈绠′腑鏂簲褰撳敖閲忛伩鍏嶃€傝繖绉嶉殧绂绘槸灏藉姏鑰屼负鐨勶紝浠呭綋鑷姩
鍒嗛厤鐨勪腑鏂帺鐮佷篃鍖呭惈琚伩寮€鎺╃爜涔嬪鐨勫湪绾?CPU 鏃舵墠閫傜敤銆傚鏋滆姹傜殑鎺╃爜鍙寘鍚殧绂荤殑 CPU锛?
鍒欒璁剧疆涓嶈捣浣滅敤銆?

鍒楀湪閬垮紑鎺╃爜涓殑 CPU 浠嶇劧鏄腑鏂翰鍜屾€ф帺鐮佺殑涓€閮ㄥ垎銆傝繖鎰忓懗鐫€濡傛灉鎵€鏈夐潪闅旂鐨?CPU 绂荤嚎鑰?
闅旂鐨?CPU 浠嶇劧鍦ㄧ嚎锛岃涓柇浼氳鍒嗛厤缁欏叾涓竴涓殧绂荤殑 CPU銆?

浠ヤ笅绀轰緥鍋囪涓€涓叿鏈?8 涓?CPU 鐨勭郴缁熴€?

- 涓€涓?QEMU 瀹炰緥浠?"-device virtio-scsi-pci" 鍚姩銆傝 MSI-X 璁惧鏆撮湶 11 涓腑鏂細3 涓?
  "绠＄悊"涓柇鍜?8 涓?"闃熷垪"涓柇銆傞┍鍔ㄨ姹傝繖 8 涓槦鍒椾腑鏂紝姣忎釜閮藉垰濂戒翰鍜屼簬涓€涓?CPU銆?
  濡傛灉璇?CPU 绂荤嚎锛岃涓柇琚叧闂€?

```

    /proc/irq/48/effective_affinity_list:7
    /proc/irq/48/smp_affinity_list:7

  This indicates that the interrupt is served only by CPU7. Shutting down CPU7
  does not migrate the interrupt to another CPU::

    /proc/irq/48/effective_affinity_list:0
    /proc/irq/48/smp_affinity_list:7

  This can be verified via the debugfs interface
  (/sys/kernel/debug/irq/irqs/48). The dstate field will include
  IRQD_IRQ_DISABLED, IRQD_IRQ_MASKED and IRQD_MANAGED_SHUTDOWN.

```
- 涓€涓?QEMU 瀹炰緥浠?"-device virtio-scsi-pci,num_queues=2" 鍚姩锛屽苟涓斿唴鏍稿懡浠よ鍖呭惈锛?
  "irqaffinity=0,1 isolcpus=domain,2-7 isolcpus=managed_irq,1-3,5-7"銆傝 MSI-X 璁惧鏆撮湶
  5 涓腑鏂細3 涓鐞嗕腑鏂拰 2 涓槦鍒椾腑鏂€傜鐞嗕腑鏂伒寰?irqaffinity= 璁剧疆銆?
```

    /proc/irq/47/effective_affinity_list:0
    /proc/irq/47/smp_affinity_list:0-3
    /proc/irq/48/effective_affinity_list:4
    /proc/irq/48/smp_affinity_list:4-7

  The two queue interrupts are evenly distributed. Interrupt 48 is placed on CPU4
  because the managed_irq mask avoids CPUs 5鈥? when possible.

  Replacing the managed_irq argument with "isolcpus=managed_irq,1-3,4-5,7"
  results in::

    /proc/irq/48/effective_affinity_list:6
    /proc/irq/48/smp_affinity_list:4-7

  Interrupt 48 is now served on CPU6 because the system avoids CPUs 4, 5 and
  7. If CPU6 is taken offline, the interrupt migrates to one of the "isolated"
  CPUs::

    /proc/irq/48/effective_affinity_list:7
    /proc/irq/48/smp_affinity_list:4-7

  The interrupt is shut down once all CPUs listed in its smp_affinity mask are
  offline.

```