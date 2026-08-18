
## UltraSoc - SoC 涓婄殑纭欢杈呭姪杩借釜

   :Author:   Qi Liu <liuqi115@huawei.com>
   :Date:     2023 骞?1 鏈?
### 绠€浠?

UltraSoc SMB 鏄竴涓?per SCCL锛圫uper CPU Cluster锛夌殑纭欢銆傚畠鎻愪緵浜嗕竴绉嶅湪鍏变韩绯荤粺鍐呭瓨鐨?鏌愪釜鍖哄煙涓紦鍐蹭笌瀛樺偍 CPU 杩借釜娑堟伅鐨勬柟寮忋€傝璁惧鍏呭綋 coresight sink 璁惧锛岀浉搴旂殑杩借釜
鐢熸垚鍣紙ETM锛変綔涓烘簮璁惧杩炴帴鍏朵笂銆?
### Sysfs 鏂囦欢涓庣洰褰?

SMB 璁惧涓庡叾浠栬澶囦竴璧峰嚭鐜板湪宸叉湁鐨?coresight 鎬荤嚎涓?```

	$# ls /sys/bus/coresight/devices/
	ultra_smb0   ultra_smb1   ultra_smb2   ultra_smb3

```
```

	$# ls /sys/bus/coresight/devices/ultra_smb0
	enable_sink   mgmt
	$# ls /sys/bus/coresight/devices/ultra_smb0/mgmt
	buf_size  buf_status  read_pos  write_pos

```
鍏抽敭鏂囦欢椤瑰涓嬶細

   - `read_pos`锛氭樉绀鸿鎸囬拡瀵勫瓨鍣ㄧ殑鍊笺€?   - `write_pos`锛氭樉绀哄啓鎸囬拡瀵勫瓨鍣ㄧ殑鍊笺€?   - `buf_status`锛氭樉绀虹姸鎬佸瘎瀛樺櫒鐨勫€笺€侭IT(0) 涓洪浂鍊硷紝琛ㄧず缂撳啿鍖轰负绌恒€?   - `buf_size`锛氭樉绀烘瘡涓澶囩殑缂撳啿鍖哄ぇ灏忋€?
### 鍥轰欢缁戝畾


璇ヨ澶囦粎鏀寔 ACPI銆傚叾缁戝畾鎻忚堪璁惧鏍囪瘑绗︺€佽祫婧愪俊鎭笌鍥剧粨鏋勩€?
璇ヨ澶囪鏍囪瘑涓?ACPI HID "HISI03A1"銆傝澶囪祫婧愪娇鐢?_CRS 鏂规硶鍒嗛厤銆傛瘡涓澶囧繀椤绘彁渚涗袱涓熀鍦板潃锛?绗竴涓槸璁惧鐨勯厤缃熀鍦板潃锛岀浜屼釜鏄叡浜郴缁熷唴瀛樼殑 32 浣嶅熀鍦板潃銆?
```

    Device(USMB) {                                               \
      Name(_HID, "HISI03A1")                                     \
      Name(_CRS, ResourceTemplate() {                            \
          QWordMemory (ResourceConsumer, , MinFixed, MaxFixed, NonCacheable, \
		       ReadWrite, 0x0, 0x95100000, 0x951FFFFF, 0x0, 0x100000) \
          QWordMemory (ResourceConsumer, , MinFixed, MaxFixed, Cacheable, \
		       ReadWrite, 0x0, 0x50000000, 0x53FFFFFF, 0x0, 0x4000000) \
      })                                                         \
      Name(_DSD, Package() {                                     \
        ToUUID("ab02a46b-74c7-45a2-bd68-f7d344ef2153"),          \
	/* 浣跨敤 CoreSight Graph ACPI 缁戝畾鏉ユ弿杩拌繛鎺ユ嫇鎵?*/
        Package() {                                              \
          0,                                                     \
          1,                                                     \
          Package() {                                            \
            1,                                                   \
            ToUUID("3ecbc8b6-1d0e-4fb3-8107-e627f805c6cd"),      \
            8,                                                   \
            Package() {0x8, 0, \_SB.S00.SL11.CL28.F008, 0},       \
            Package() {0x9, 0, \_SB.S00.SL11.CL29.F009, 0},       \
            Package() {0xa, 0, \_SB.S00.SL11.CL2A.F010, 0},       \
            Package() {0xb, 0, \_SB.S00.SL11.CL2B.F011, 0},       \
            Package() {0xc, 0, \_SB.S00.SL11.CL2C.F012, 0},       \
            Package() {0xd, 0, \_SB.S00.SL11.CL2D.F013, 0},       \
            Package() {0xe, 0, \_SB.S00.SL11.CL2E.F014, 0},       \
            Package() {0xf, 0, \_SB.S00.SL11.CL2F.F015, 0},       \
          }                                                      \
        }                                                        \
      })                                                         \
    }

```
