#### 瑙嗛杈撳嚭鍒囨崲鍣ㄦ帶鍒?


2006骞磍uming.yu@intel.com

杈撳嚭 sysfs 绫婚┍鍔ㄧ▼搴忔彁渚涗簡涓€涓娊璞¤棰戣緭鍑哄眰锛?
鍙敤浜庢寕閽╃壒瀹氫簬骞冲彴鐨勬柟娉曚互鍚敤/绂佺敤瑙嗛杈撳嚭
閫氳繃閫氱敤 sysfs 鎺ュ彛璁块棶璁惧銆備緥濡傦紝鍦ㄦ垜鐨?IBM ThinkPad T42 涓?
绗旇鏈數鑴戯紝ACPI 瑙嗛椹卞姩绋嬪簭娉ㄥ唽鍏惰緭鍑鸿澶囧苟璇?鍐?
```

  linux:/sys/class/video_output # tree .
  .
  |-- CRT0
  |   |-- device -> ../../../devices/pci0000:00/0000:00:01.0
  |   |-- state
  |   |-- subsystem -> ../../../class/video_output
  |   `-- uevent
  |-- DVI0
  |   |-- device -> ../../../devices/pci0000:00/0000:00:01.0
  |   |-- state
  |   |-- subsystem -> ../../../class/video_output
  |   `-- uevent
  |-- LCD0
  |   |-- device -> ../../../devices/pci0000:00/0000:00:01.0
  |   |-- state
  |   |-- subsystem -> ../../../class/video_output
  |   `-- uevent
  `-- TV0
     |-- device -> ../../../devices/pci0000:00/0000:00:01.0
     |-- state
     |-- subsystem -> ../../../class/video_output
     `-- uevent


```