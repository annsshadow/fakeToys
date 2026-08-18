## Fujitsu Uncore 鎬ц兘鐩戞帶鍗曞厓锛圥MU锛?


鏈┍鍔ㄦ敮鎸?Fujitsu 鑺墖涓殑 Uncore MAC PMU 涓?Uncore PCI PMU銆?
杩欎簺鑺墖涓婄殑姣忎釜 MAC PMU 閮戒綔涓轰竴涓?uncore perf PMU 鏆撮湶鍑烘潵锛岃澶囧悕涓?
mac_iod<iod>_mac<mac>_ch<ch>銆?
姣忎釜 PCI PMU 浣滀负 uncore perf PMU 鏆撮湶锛岃澶囧悕涓?pci_iod<iod>_pci<pci>銆?

椹卞姩鍦?sysfs 涓彁渚涘叾鍙敤浜嬩欢涓庨厤缃€夐」鐨勬弿杩帮紝鍙傝
/sys/bus/event_sources/devices/mac_iod<iod>_mac<mac>_ch<ch>/
涓?/sys/bus/event_sources/devices/pci_iod<iod>_pci<pci>/銆?
鏈┍鍔ㄥ鍑猴細

- formats锛氫緵 perf 鐢ㄦ埛绌洪棿鍙婂叾浠栧伐鍏烽厤缃簨浠朵娇鐢?
- events锛氫緵 perf 鐢ㄦ埛绌洪棿鍙婂叾浠栧伐鍏峰垱寤轰簨浠朵娇鐢?
```

    perf stat -a -e mac_iod0_mac0_ch0/event=0x21/ ls
    perf stat -a -e pci_iod0_pci0/event=0x24/ ls

```
- cpumask锛氫緵 perf 鐢ㄦ埛绌洪棿鍙婂叾浠栧伐鍏蜂簡瑙ｅ簲鍦ㄥ摢浜?CPU 涓婃墦寮€浜嬩欢

鏈┍鍔ㄤ负 MAC 鏀寔浠ヤ笅浜嬩欢锛?

- cycles
  姝や簨浠剁粺璁?MAC 鍦?MAC 棰戠巼涓嬬殑鍛ㄦ湡鏁般€?
- read-count
  姝や簨浠剁粺璁″彂寰€ MAC 鐨勮璇锋眰鏁伴噺銆?
- read-count-request
  姝や簨浠剁粺璁″寘鍚噸璇曠殑銆佸彂寰€ MAC 鐨勮璇锋眰鏁伴噺銆?
- read-count-return
  姝や簨浠剁粺璁″鍙戝線 MAC 鐨勮璇锋眰鐨勫搷搴旀暟閲忋€?
- read-count-request-pftgt
  姝や簨浠剁粺璁″甫鏈?PFTGT 鏍囧織銆佸寘鍚噸璇曠殑璇昏姹傛暟閲忋€?
- read-count-request-normal
  姝や簨浠剁粺璁′笉甯?PFTGT 鏍囧織銆佸寘鍚噸璇曠殑璇昏姹傛暟閲忋€?
- read-count-return-pftgt-hit
  姝や簨浠剁粺璁″懡涓?PFTGT 缂撳啿鍖虹殑璇昏姹傚搷搴旀暟閲忋€?
- read-count-return-pftgt-miss
  姝や簨浠剁粺璁℃湭鍛戒腑 PFTGT 缂撳啿鍖虹殑璇昏姹傚搷搴旀暟閲忋€?
- read-wait
  姝や簨浠剁粺璁℃瘡涓懆鏈熺敱 DDR 鍐呭瓨鎺у埗鍣ㄥ彂鍑虹殑鏈畬鎴愯璇锋眰鏁伴噺銆?
- write-count
  姝や簨浠剁粺璁″彂寰€ MAC 鐨勫啓璇锋眰鏁伴噺锛堝寘鎷浂鍐欍€佸叏鍐欍€侀儴鍒嗗啓銆佸啓鍙栨秷锛夈€?
- write-count-write
  姝や簨浠剁粺璁″彂寰€ MAC 鐨勫叏鍐欒姹傛暟閲忥紙涓嶅寘鎷浂鍐欙級銆?
- write-count-pwrite
  姝や簨浠剁粺璁″彂寰€ MAC 鐨勯儴鍒嗗啓璇锋眰鏁伴噺銆?
- memory-read-count
  姝や簨浠剁粺璁?MAC 鍙戝線鍐呭瓨鐨勮璇锋眰鏁伴噺銆?
- memory-write-count
  姝や簨浠剁粺璁?MAC 鍙戝線鍐呭瓨鐨勫叏鍐欒姹傛暟閲忋€?
- memory-pwrite-count
  姝や簨浠剁粺璁?MAC 鍙戝線鍐呭瓨鐨勯儴鍒嗗啓璇锋眰鏁伴噺銆?
- ea-mac
  姝や簨浠剁粺璁?MAC 鐨勮兘鑰椼€?
- ea-memory
  姝や簨浠剁粺璁″唴瀛樼殑鑳借€椼€?
- ea-memory-mac-write
  姝や簨浠剁粺璁?MAC 鍙戝線鍐呭瓨鐨勫啓璇锋眰鏁伴噺銆?
- ea-ha
  姝や簨浠剁粺璁?HA 鐨勮兘鑰椼€?

  'ea' 鏄?'Energy Analyzer'锛堣兘鑰楀垎鏋愬櫒锛夌殑缂╁啓銆?

```

  perf stat -e mac_iod0_mac0_ch0/ea-mac/ ls

```
姝ゅ锛屾湰椹卞姩涓?PCI 鏀寔浠ヤ笅浜嬩欢锛?

- pci-port0-cycles
  姝や簨浠剁粺璁?port0 涓?PCI 鍦?PCI 棰戠巼涓嬬殑鍛ㄦ湡鏁般€?
- pci-port0-read-count
  姝や簨浠剁粺璁?port0 涓敤浜庢暟鎹紶杈撶殑璇讳簨鍔℃暟閲忋€?
- pci-port0-read-count-bus
  姝や簨浠剁粺璁?port0 涓敤浜庢€荤嚎鍗犵敤鐨勮浜嬪姟鏁伴噺銆?
- pci-port0-write-count
  姝や簨浠剁粺璁?port0 涓敤浜庢暟鎹紶杈撶殑鍐欎簨鍔℃暟閲忋€?
- pci-port0-write-count-bus
  姝や簨浠剁粺璁?port0 涓敤浜庢€荤嚎鍗犵敤鐨勫啓浜嬪姟鏁伴噺銆?
- pci-port1-cycles
  姝や簨浠剁粺璁?port1 涓?PCI 鍦?PCI 棰戠巼涓嬬殑鍛ㄦ湡鏁般€?
- pci-port1-read-count
  姝や簨浠剁粺璁?port1 涓敤浜庢暟鎹紶杈撶殑璇讳簨鍔℃暟閲忋€?
- pci-port1-read-count-bus
  姝や簨浠剁粺璁?port1 涓敤浜庢€荤嚎鍗犵敤鐨勮浜嬪姟鏁伴噺銆?
- pci-port1-write-count
  姝や簨浠剁粺璁?port1 涓敤浜庢暟鎹紶杈撶殑鍐欎簨鍔℃暟閲忋€?
- pci-port1-write-count-bus
  姝や簨浠剁粺璁?port1 涓敤浜庢€荤嚎鍗犵敤鐨勫啓浜嬪姟鏁伴噺銆?
- ea-pci
  姝や簨浠剁粺璁?PCI 鐨勮兘鑰椼€?

  'ea' 鏄?'Energy Analyzer'锛堣兘鑰楀垎鏋愬櫒锛夌殑缂╁啓銆?

```

  perf stat -e pci_iod0_pci0/ea-pci/ ls

```
鐢变簬杩欎簺鏄?uncore PMU锛岄┍鍔ㄤ笉鏀寔閲囨牱锛屽洜姝?"perf record" 鏃犳硶浣跨敤銆備篃涓嶆敮鎸佹寜浠诲姟鐨?perf 浼氳瘽銆?
