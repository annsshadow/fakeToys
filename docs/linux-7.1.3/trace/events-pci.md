
## 瀛愮郴缁熻拷韪偣锛歅CI


## 姒傝堪

PCI 杩借釜绯荤粺鎻愪緵杩借釜鐐癸紝鐢ㄤ簬鐩戞帶鍙兘褰卞搷绯荤粺鎬ц兘涓庡彲闈犳€х殑鍏抽敭纭欢浜嬩欢銆傝繖浜涗簨浠堕€氬父鍑虹幇鍦ㄤ互涓嬩綅缃細

	/sys/kernel/tracing/events/pci

鐩稿叧浜嬩欢瀹氫箟瑙?include/trace/events/pci.h銆?
## 鍙敤鐨勮拷韪偣


### pci_hp_event


鐩戞帶 PCI 鐑彃鎷斾簨浠讹紝鍖呮嫭鍗＄殑鎻掑叆/绉婚櫎浠ュ強閾捐矾鐘舵€佸彉鍖栥€?```

    pci_hp_event  "%s slot:%s, event:%s\n"

```
**浜嬩欢绫诲瀷**锛?
- `LINK_UP` - PCIe 閾捐矾宸插缓绔?- `LINK_DOWN` - PCIe 閾捐矾涓㈠け
- `CARD_PRESENT` - 鎻掓Ы涓娴嬪埌鍗?- `CARD_NOT_PRESENT` - 鍗″凡浠庢彃妲界Щ闄?
```

    # 鍚敤杩借釜鐐?    echo 1 > /sys/kernel/debug/tracing/events/pci/pci_hp_event/enable

    # 鐩戞帶浜嬩欢锛堜互涓嬭緭鍑哄湪璁惧鐑彃鎷旀椂浜х敓锛?    cat /sys/kernel/debug/tracing/trace_pipe
       irq/51-pciehp-88      [001] .....  1311.177459: pci_hp_event: 0000:00:02.0 slot:10, event:CARD_PRESENT

       irq/51-pciehp-88      [001] .....  1311.177566: pci_hp_event: 0000:00:02.0 slot:10, event:LINK_UP

```
### pcie_link_event


鐩戞帶 PCIe 閾捐矾閫熺巼鍙樺寲锛屽苟鎻愪緵璇︾粏鐨勯摼璺姸鎬佷俊鎭€?```

    pcie_link_event  "%s type:%d, reason:%d, cur_bus_speed:%d, max_bus_speed:%d, width:%u, flit_mode:%u, status:%s\n"

```
**鍙傛暟**锛?
- `type` - PCIe 璁惧绫诲瀷锛?=Root Port锛岀瓑锛?- `reason` - 閾捐矾鍙樺寲鐨勫師鍥狅細

  - `0` - 閾捐矾閲嶈缁?  - `1` - 鎬荤嚎鏋氫妇
  - `2` - 甯﹀閫氱煡浣胯兘
  - `3` - 甯﹀閫氱煡 IRQ
  - `4` - 鐑彃鎷斾簨浠?

```

    # 鍚敤杩借釜鐐?    echo 1 > /sys/kernel/debug/tracing/events/pci/pcie_link_event/enable

    # 鐩戞帶浜嬩欢锛堜互涓嬭緭鍑哄湪璁惧鐑彃鎷旀椂浜х敓锛?    cat /sys/kernel/debug/tracing/trace_pipe
       irq/51-pciehp-88      [001] .....   381.545386: pcie_link_event: 0000:00:02.0 type:4, reason:4, cur_bus_speed:20, max_bus_speed:23, width:1, flit_mode:0, status:DLLLA

```
