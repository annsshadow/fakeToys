## Highpoint RocketRAID 3xxx/4xxx 閫傞厤鍣ㄩ┍鍔紙hptiop锛?

### 鎺у埗鍣ㄥ瘎瀛樺櫒鏄犲皠


瀵逛簬鍩轰簬 RR44xx Intel IOP 鐨勯€傞厤鍣紝鎺у埗鍣?IOP 閫氳繃 PCI BAR0 涓?BAR2 璁块棶

     ============== ==================================
     BAR0 offset    Register
     ============== ==================================
            0x11C5C Link Interface IRQ Set
            0x11C60 Link Interface IRQ Clear
     ============== ==================================

     ============== ==================================
     BAR2 offset    Register
     ============== ==================================
            0x10    Inbound Message Register 0
            0x14    Inbound Message Register 1
            0x18    Outbound Message Register 0
            0x1C    Outbound Message Register 1
            0x20    Inbound Doorbell Register
            0x24    Inbound Interrupt Status Register
            0x28    Inbound Interrupt Mask Register
            0x30    Outbound Interrupt Status Register
            0x34    Outbound Interrupt Mask Register
            0x40    Inbound Queue Port
            0x44    Outbound Queue Port
     ============== ==================================

瀵逛簬鍩轰簬 Intel IOP 鐨勯€傞厤鍣紝鎺у埗鍣?IOP 閫氳繃 PCI BAR0 璁块棶锛?
     ============== ==================================
     BAR0 offset    Register
     ============== ==================================
            0x10    Inbound Message Register 0
            0x14    Inbound Message Register 1
            0x18    Outbound Message Register 0
            0x1C    Outbound Message Register 1
            0x20    Inbound Doorbell Register
            0x24    Inbound Interrupt Status Register
            0x28    Inbound Interrupt Mask Register
            0x30    Outbound Interrupt Status Register
            0x34    Outbound Interrupt Mask Register
            0x40    Inbound Queue Port
            0x44    Outbound Queue Port
     ============== ==================================

瀵逛簬鍩轰簬 Marvell锛堥潪 Frey锛塈OP 鐨勯€傞厤鍣紝IOP 閫氳繃 PCI BAR0 涓?BAR1 璁块棶锛?
     ============== ==================================
     BAR0 offset    Register
     ============== ==================================
         0x20400    Inbound Doorbell Register
         0x20404    Inbound Interrupt Mask Register
         0x20408    Outbound Doorbell Register
         0x2040C    Outbound Interrupt Mask Register
     ============== ==================================

     ============== ==================================
     BAR1 offset    Register
     ============== ==================================
             0x0    Inbound Queue Head Pointer
             0x4    Inbound Queue Tail Pointer
             0x8    Outbound Queue Head Pointer
             0xC    Outbound Queue Tail Pointer
            0x10    Inbound Message Register
            0x14    Outbound Message Register
     0x40-0x1040    Inbound Queue
     0x1040-0x2040  Outbound Queue
     ============== ==================================

瀵逛簬鍩轰簬 Marvell Frey IOP 鐨勯€傞厤鍣紝IOP 閫氳繃 PCI BAR0 涓?BAR1 璁块棶锛?
     ============== ==================================
     BAR0 offset    Register
     ============== ==================================
             0x0    IOP configuration information.
     ============== ==================================

     ============== ===================================================
     BAR1 offset    Register
     ============== ===================================================
          0x4000    Inbound List Base Address Low
          0x4004    Inbound List Base Address High
          0x4018    Inbound List Write Pointer
          0x402C    Inbound List Configuration and Control
          0x4050    Outbound List Base Address Low
          0x4054    Outbound List Base Address High
          0x4058    Outbound List Copy Pointer Shadow Base Address Low
          0x405C    Outbound List Copy Pointer Shadow Base Address High
          0x4088    Outbound List Interrupt Cause
          0x408C    Outbound List Interrupt Enable
         0x1020C    PCIe Function 0 Interrupt Enable
         0x10400    PCIe Function 0 to CPU Message A
         0x10420    CPU to PCIe Function 0 Message A
         0x10480    CPU to PCIe Function 0 Doorbell
         0x10484    CPU to PCIe Function 0 Doorbell Enable
     ============== ===================================================


### 闈?Marvell Frey 鐨?I/O 璇锋眰宸ヤ綔娴?

鎵€鏈夋帓闃熺殑璇锋眰閮介€氳繃鍏ョ珯/鍑虹珯闃熷垪绔彛澶勭悊銆?璇锋眰鍖呭彲浠ュ湪 IOP 鎴栦富鏈哄唴瀛樹腑鍒嗛厤銆?
瑕佸悜鎺у埗鍣ㄥ彂閫佽姹傦細

    - 閫氳繃璇诲彇鍏ョ珯闃熷垪绔彛鑾峰彇涓€涓┖闂茶姹傚寘锛屾垨
      鍦ㄤ富鏈?DMA 涓€鑷存€у唴瀛樹腑鍒嗛厤涓€涓┖闂茶姹傘€?
      浠庡叆绔欓槦鍒楃鍙ｈ繑鍥炵殑鍊兼槸涓€涓浉瀵逛簬 IOP BAR0 鐨勫亸绉婚噺銆?
      鍦ㄤ富鏈哄唴瀛樹腑鍒嗛厤鐨勮姹傚繀椤绘寜 32 瀛楄妭杈圭晫瀵归綈銆?
    - 濉厖璇ュ寘銆?
    - 閫氳繃灏嗗寘鍐欏叆鍏ョ珯闃熷垪灏嗗叾鎶曢€掔粰 IOP銆傚浜庡湪 IOP 鍐呭瓨涓垎閰嶇殑璇锋眰锛?      灏嗗亸绉婚噺鍐欏叆鍏ョ珯闃熷垪绔彛銆傚浜庡湪涓绘満鍐呭瓨涓垎閰嶇殑璇锋眰锛屽皢 (0x80000000|(bus_addr>>5))
      鍐欏叆鍏ョ珯闃熷垪绔彛銆?
    - IOP 澶勭悊璇ヨ姹傘€傚綋璇锋眰瀹屾垚鏃讹紝瀹冨皢琚斁鍏ュ嚭绔欓槦鍒椼€傚皢浜х敓涓€涓嚭绔欎腑鏂€?
      瀵逛簬鍦?IOP 鍐呭瓨涓垎閰嶇殑璇锋眰锛岃姹傚亸绉婚噺琚姇閫掑埌鍑虹珯闃熷垪銆?
      瀵逛簬鍦ㄤ富鏈哄唴瀛樹腑鍒嗛厤鐨勮姹傦紝(0x80000000|(bus_addr>>5))
      琚姇閫掑埌鍑虹珯闃熷垪銆傚鏋滆姹備腑璁剧疆浜?IOP_REQUEST_FLAG_OUTPUT_CONTEXT 鏍囧織锛?      鍒欐敼涓烘姇閫掍綆 32 浣嶄笂涓嬫枃鍊笺€?
    - 涓绘満璇诲彇鍑虹珯闃熷垪骞跺畬鎴愯姹傘€?
      瀵逛簬鍦?IOP 鍐呭瓨涓垎閰嶇殑璇锋眰锛屼富鏈洪┍鍔ㄩ€氳繃灏嗗叾鍐欏叆鍑虹珯闃熷垪鏉ラ噴鏀捐璇锋眰銆?
闈炴帓闃熻姹傦紙reset/flush 绛夛級鍙互閫氳繃鍏ョ珯娑堟伅瀵勫瓨鍣?0 鍙戦€併€傚甫鏈夌浉鍚屽€肩殑鍑虹珯娑堟伅琛ㄧず
鍏ョ珯娑堟伅鐨勫畬鎴愩€?

### Marvell Frey 鐨?I/O 璇锋眰宸ヤ綔娴?

鎵€鏈夋帓闃熺殑璇锋眰閮介€氳繃鍏ョ珯/鍑虹珯鍒楄〃澶勭悊銆?
瑕佸悜鎺у埗鍣ㄥ彂閫佽姹傦細

    - 鍦ㄤ富鏈?DMA 涓€鑷存€у唴瀛樹腑鍒嗛厤涓€涓┖闂茶姹傘€?
      鍦ㄤ富鏈哄唴瀛樹腑鍒嗛厤鐨勮姹傚繀椤绘寜 32 瀛楄妭杈圭晫瀵归綈銆?
    - 鐢ㄨ姹傚湪鏍囧織涓殑绱㈠紩濉厖璇锋眰銆?
      鐢ㄤ竴涓┖闂插叆绔欏垪琛ㄥ崟鍏冨～鍏呰姹傜殑鐗╃悊鍦板潃涓庡ぇ灏忋€?
      鐢ㄥ墠涓€涓崟鍏冪殑绱㈠紩璁剧疆鍏ョ珯鍒楄〃鍐欐寚閽堬紝褰撶储寮曡揪鍒版敮鎸佺殑璇锋眰璁℃暟鏃跺洖缁曞埌 0銆?
    - 灏嗗叆绔欏垪琛ㄥ啓鎸囬拡鎶曢€掔粰 IOP銆?
    - IOP 澶勭悊璇ヨ姹傘€傚綋璇锋眰瀹屾垚鏃讹紝甯︽湁鎴栬繍绠椾簡 IOPMU_QUEUE_MASK_HOST_BITS 鏍囧織鐨勮姹傚皢琚斁鍏ヤ竴涓?      绌洪棽鍑虹珯鍒楄〃鍗曞厓锛屽苟涓斿嚭绔欏垪琛ㄥ崟鍏冪殑绱㈠紩灏嗚鏀惧叆澶嶅埗鎸囬拡褰卞瓙锛坈opy pointer shadow锛夊瘎瀛樺櫒銆傚皢浜х敓涓€涓嚭绔欎腑鏂€?
    - 涓绘満璇诲彇鍑虹珯鍒楄〃澶嶅埗鎸囬拡褰卞瓙瀵勫瓨鍣紝骞朵笌涔嬪墠淇濆瓨鐨勮鎸囬拡 N 姣旇緝銆傚鏋滃畠浠笉鍚岋紝涓绘満灏?      璇诲彇绗?(N+1) 涓嚭绔欏垪琛ㄥ崟鍏冦€?
      涓绘満浠庣 (N+1) 涓嚭绔欏垪琛ㄥ崟鍏冭幏鍙栬姹傜殑绱㈠紩骞跺畬鎴愯璇锋眰銆?
闈炴帓闃熻姹傦紙reset communication/reset/flush 绛夛級鍙互閫氳繃 PCIe Function 0 to CPU Message A 瀵勫瓨鍣ㄥ彂閫併€傚甫鏈夌浉鍚屽€肩殑
CPU to PCIe Function 0 Message 瀵勫瓨鍣ㄨ〃绀鸿娑堟伅鐨勫畬鎴愩€?

### 鐢ㄦ埛绾ф帴鍙?

璇ラ┍鍔ㄥ鍑轰互涓?sysfs 灞炴€э細

     ==================   ===    ========================
     NAME                 R/W    Description
     ==================   ===    ========================
     driver-version        R     driver version string
     firmware-version      R     firmware version string
     ==================   ===    ========================


-----------------------------------------------------------------------------

Copyright |copy| 2006-2012 HighPoint Technologies, Inc. All Rights Reserved.

  鏈枃浠朵互鈥滃笇鏈涘畠鏈夌敤鈥濈殑鏂瑰紡鍒嗗彂锛?  浣?WITHOUT ANY WARRANTY锛堜笉鎻愪緵浠讳綍鎷呬繚锛夛紱鐢氳嚦涓嶆殫绀哄
  MERCHANTABILITY锛堥€傞攢鎬э級鎴?FITNESS FOR A PARTICULAR PURPOSE锛堢壒瀹氱敤閫旈€傜敤鎬э級鐨勬媴淇濄€傝瑙?  GNU General Public License銆?
  linux@highpoint-tech.com

  http://www.highpoint-tech.com
