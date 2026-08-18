
## POWER 涓婄殑宓屽 KVM


## 绠€浠?

鏈枃妗ｈВ閲婁簡涓€涓鎴锋満鎿嶄綔绯荤粺濡備綍鍦ㄧ鐞嗙▼搴忓疄鐜颁簡鐩稿叧瓒呯骇璋冪敤锛坔ypercall锛夌殑鎯呭喌涓嬶紝鍏呭綋绠＄悊绋嬪簭骞堕€氳繃浣跨敤瓒呯骇璋冪敤鏉ヨ繍琛屽祵濂楀鎴锋満銆傛湳璇?L0銆丩1 涓?L2 鐢ㄤ簬鎸囦唬涓嶅悓鐨勮蒋浠跺疄浣撱€侺0 鏄浜庣鐞嗙▼搴忔ā寮忕殑瀹炰綋锛岄€氬父琚О涓衡€滃涓绘満锛坔ost锛夆€濇垨鈥滅鐞嗙▼搴忥紙hypervisor锛夆€濄€侺1 鏄竴涓洿鎺ュ湪 L0 涔嬩笅杩愯銆佺敱 L0 鍙戣捣骞舵帶鍒剁殑瀹㈡埛鏈鸿櫄鎷熸満銆侺2 鏄竴涓敱鍏呭綋绠＄悊绋嬪簭鐨?L1 鍙戣捣骞舵帶鍒剁殑瀹㈡埛鏈鸿櫄鎷熸満銆?
## 鐜版湁 API


Linux/KVM 鑷?2018 骞磋捣灏辨敮鎸佷綔涓?L0 鎴?L1 杩涜宓屽锛圢esting锛?
```

   commit 8e3f5fc1045dc49fd175b978c5457f5f51e7a2ce
   Author: Paul Mackerras <paulus@ozlabs.org>
   Date:   Mon Oct 8 16:31:03 2018 +1100
   KVM: PPC: Book3S HV: Framework and hcall stubs for nested virtualization

```
```
   commit 360cae313702cdd0b90f82c261a8302fecef030a
   Author: Paul Mackerras <paulus@ozlabs.org>
   Date:   Mon Oct 8 16:31:04 2018 +1100
   KVM: PPC: Book3S HV: Nested guest entry via hypercall

```
姝?API 涓昏鍊熷姪鍗曚竴鐨勮秴绾ц皟鐢?h_enter_nested() 宸ヤ綔銆傝璋冪敤鐢?L1 鍙戝嚭锛岀敤浠ュ憡鐭?L0 浠ョ粰瀹氱姸鎬佸惎鍔ㄤ竴涓?L2 vCPU銆傞殢鍚?L0 鍚姩杩欎釜 L2 骞惰繍琛岋紝鐩村埌杈惧埌涓€涓?L2 閫€鍑烘潯浠躲€備竴鏃?L2 閫€鍑猴紝L2 鐨勭姸鎬佸氨鐢?L0 浜よ繕缁?L1銆傛瘡褰?L2 杩愯鏃讹紝瀹屾暣鐨?L2 vCPU 鐘舵€佹€绘槸鍦?L1 涔嬮棿鏉ュ洖浼犻€掋€侺0 涓嶅湪 L2 vCPU 涓婁繚鐣欎换浣曠姸鎬侊紙闄や簡鍦?L0 涓?L1 -> L2 杩涘叆涓?L2 -> L1 閫€鍑虹殑鐭殏搴忓垪鏈熼棿锛夈€?
L0 淇濈暀鐨勫敮涓€鐘舵€佹槸鍒嗗尯琛紙partition table锛夈€侺1 浣跨敤 h_set_partition_table() 瓒呯骇璋冪敤娉ㄥ唽瀹冪殑鍒嗗尯琛ㄣ€侺0 鎸佹湁鐨勫叧浜?L2 鐨勬墍鏈夊叾瀹冪姸鎬侀兘鏄缂撳瓨鐨勭姸鎬侊紙渚嬪褰卞瓙椤佃〃锛夈€?
L1 鍙互鍦ㄤ笉浜嬪厛閫氱煡 L0 鐨勬儏鍐典笅杩愯浠讳綍 L2 鎴?vCPU銆傚畠鍙渶浣跨敤 h_enter_nested() 鍚姩 vCPU 鍗冲彲銆侺2 涓?vCPU 鐨勫垱寤哄湪姣忔璋冪敤 h_enter_nested() 鏃堕殣寮忓畬鎴愩€?
鍦ㄦ湰鏂囨。涓紝鎴戜滑绉拌繖涓幇鏈?API 涓?v1 API銆?
## 鏂扮殑 PAPR API


鏂扮殑 PAPR API 涓?v1 API 鐨勪笉鍚屼箣澶勫湪浜庯細鍒涘缓 L2 鍙婂叾鍏宠仈鐨?vCPU 鏄樉寮忕殑銆傚湪鏈枃妗ｄ腑锛屾垜浠О涔嬩负 v2 API銆?
h_enter_nested() 琚?H_GUEST_VCPU_RUN() 鍙栦唬銆傚湪杩欎箣鍓嶏紝L1 蹇呴』浣跨敤 h_guest_create() 鏄惧紡鍦板垱寤?L2锛屽苟浣跨敤 h_guest_create_vCPU() 鍒涘缓浠讳綍鍏宠仈鐨?vCPU銆傝幏鍙栦笌璁剧疆 vCPU 鐘舵€佷篃鍙互浣跨敤 h_guest_{g|s}et 瓒呯骇璋冪敤瀹屾垚銆?
L1 鍒涘缓涓€涓?L2銆佽繍琛屽畠骞跺垹闄ゅ畠鐨勫熀鏈墽琛屾祦绋嬫槸锛?
- L1 涓?L0 閫氳繃 H_GUEST_{G,S}ET_CAPABILITIES() 鍗忓晢鑳藉姏锛堥€氬父鍦?L1 鍚姩鏃讹級銆?
- L1 璇锋眰 L0 鐢?H_GUEST_CREATE() 鍒涘缓涓€涓?L2锛屽苟鏀跺埌涓€涓护鐗岋紙token锛?
- L1 璇锋眰 L0 鐢?H_GUEST_CREATE_VCPU() 鍒涘缓涓€涓?L2 vCPU

- L1 涓?L0 浣跨敤 H_GUEST_{G,S}ET() 瓒呯骇璋冪敤娌熼€?vCPU 鐘舵€?
- L1 璇锋眰 L0 閫氳繃杩愯 H_GUEST_VCPU_RUN() 瓒呯骇璋冪敤鏉ヨ繍琛岃 vCPU

- L1 鐢?H_GUEST_DELETE() 鍒犻櫎 L2

鍏充簬鍚勪釜瓒呯骇璋冪敤鐨勬洿澶氱粏鑺傚涓嬶細

## HCALL 璇︽儏


鎻愪緵鏈枃妗ｆ槸涓轰簡璁╀汉瀵?API 鏈変竴涓暣浣撶殑鐞嗚В銆傚畠骞朵笉鏃ㄥ湪鎻愪緵瀹炵幇涓€涓?L1 鎴?L0 鎵€闇€鐨勫叏閮ㄧ粏鑺傘€傛洿澶氱粏鑺傚彲鍙傝€冩渶鏂扮増鏈殑 PAPR銆?
鎵€鏈夎繖浜?HCALL 閮界敱 L1 鍚?L0 鍙戝嚭銆?
### H_GUEST_GET_CAPABILITIES()


姝よ皟鐢ㄧ敤浜庤幏鍙?L0 宓屽绠＄悊绋嬪簭鐨勮兘鍔涖€傝繖鍖呮嫭璇稿 CPU 鐗堟湰涔嬬被鐨勮兘鍔涳紙渚嬪
```

  H_GUEST_GET_CAPABILITIES(uint64 flags)

  Parameters:
    Input:
      flags: Reserved
    Output:
      R3: Return code
      R4: Hypervisor Supported Capabilities bitmap 1

```
### H_GUEST_SET_CAPABILITIES()


姝よ皟鐢ㄧ敤浜庡皢 L1 绠＄悊绋嬪簭鐨勮兘鍔涘憡鐭?L0銆傝繖閲屼紶鍏ョ殑鏍囧織闆嗗悎涓?H_GUEST_GET_CAPABILITIES() 鐩稿悓銆?
閫氬父锛屽厛璋冪敤 GET锛岀劧鍚庡啀鐢ㄤ粠 GET 杩斿洖鐨勬爣蹇楀瓙闆嗚皟鐢?SET銆傝繖涓€杩囩▼鍏佽 L0 涓?```

  H_GUEST_SET_CAPABILITIES(uint64 flags,
                           uint64 capabilitiesBitmap1)
  Parameters:
    Input:
      flags: Reserved
      capabilitiesBitmap1: Only capabilities advertised through
                           H_GUEST_GET_CAPABILITIES
    Output:
      R3: Return code
      R4: If R3 = H_P2: The number of invalid bitmaps
      R5: If R3 = H_P2: The index of first invalid bitmap

```
### H_GUEST_CREATE()


姝よ皟鐢ㄧ敤浜庡垱寤轰竴涓?L2銆備細杩斿洖鎵€鍒涘缓 L2 鐨勫敮涓€ ID锛堢被浼间簬涓€涓?LPID锛夛紝鍙湪鍚庣画 HCALL 涓娇鐢ㄥ畠鏉?```

  H_GUEST_CREATE(uint64 flags,
                 uint64 continueToken);
  Parameters:
    Input:
      flags: Reserved
      continueToken: Initial call set to -1. Subsequent calls,
                     after H_Busy or H_LongBusyOrder has been
                     returned, value that was returned in R4.
    Output:
      R3: Return code. Notable:
        H_Not_Enough_Resources: Unable to create Guest VCPU due to not
        enough Hypervisor memory. See H_GUEST_CREATE_GET_STATE(flags =
        takeOwnershipOfVcpuState)
      R4: If R3 = H_Busy or_H_LongBusyOrder -> continueToken

```
### H_GUEST_CREATE_VCPU()


姝よ皟鐢ㄧ敤浜庡垱寤轰竴涓笌 L2 鍏宠仈鐨?vCPU銆傚簲褰撲紶鍏?L2 鐨?id锛堜粠 H_GUEST_CREATE() 杩斿洖锛夈€傚悓鏃朵紶鍏ョ殑杩樻湁涓€涓紙瀵规 L2 鑰岃█锛夊敮涓€鐨?vCPUid銆傝繖涓?vCPUid 鐢?```

  H_GUEST_CREATE_VCPU(uint64 flags,
                      uint64 guestId,
                      uint64 vcpuId);
  Parameters:
    Input:
      flags: Reserved
      guestId: ID obtained from H_GUEST_CREATE
      vcpuId: ID of the vCPU to be created. This must be within the
              range of 0 to 2047
    Output:
      R3: Return code. Notable:
        H_Not_Enough_Resources: Unable to create Guest VCPU due to not
        enough Hypervisor memory. See H_GUEST_CREATE_GET_STATE(flags =
        takeOwnershipOfVcpuState)

```
### H_GUEST_GET_STATE()


姝よ皟鐢ㄧ敤浜庤幏鍙栦笌 L2 鍏宠仈鐨勭姸鎬侊紙瀹㈡埛鏈虹骇鎴?vCPU 鐗瑰畾锛夈€傝淇℃伅閫氳繃瀹㈡埛鏈虹姸鎬佺紦鍐插尯锛圙SB锛変紶閫掞紝瀹冩槸涓€绉嶆爣鍑嗘牸寮忥紝濡傛湰鏂囨。鍚庨潰鎵€瑙ｉ噴锛屽繀瑕佺粏鑺傚涓嬶細

杩欏彲浠ヨ幏鍙?L2 绾ф垨 vCPU 鐗瑰畾鐨勪俊鎭€侺2 绾х殑渚嬪瓙鏈夋椂鍩哄亸绉绘垨杩涚▼浣滅敤鍩熼〉琛ㄤ俊鎭€倂CPU 鐗瑰畾鐨勪緥瀛愭湁 GPR 鎴?VSR銆俧lags 鍙傛暟涓殑涓€涓綅鎸囨槑姝よ皟鐢ㄦ槸 L2 绾ц繕鏄?vCPU 鐗瑰畾鐨勶紝骞朵笖 GSB 涓殑 ID 蹇呴』涓庝箣鍖归厤銆?
L1 鎻愪緵涓€涓寚鍚?GSB 鐨勬寚閽堜綔涓烘璋冪敤鐨勫弬鏁般€傚悓鏃舵彁渚涚殑杩樻湁涓庤璁剧疆鐨勭姸鎬佸叧鑱旂殑 L2 涓?vCPU ID銆?
L1 鍙湪 GSB 涓啓鍏?ID 涓庡ぇ灏忋€侺0 鍐欏叆
```

  H_GUEST_GET_STATE(uint64 flags,
                           uint64 guestId,
                           uint64 vcpuId,
                           uint64 dataBuffer,
                           uint64 dataBufferSizeInBytes);
  Parameters:
    Input:
      flags:
         Bit 0: getGuestWideState: Request state of the Guest instead
           of an individual VCPU.
         Bit 1: getHostWideState: Request stats of the Host. This causes
           the guestId and vcpuId parameters to be ignored and attempting
           to get the VCPU/Guest state will cause an error.
         Bits 2-63: Reserved
      guestId: ID obtained from H_GUEST_CREATE
      vcpuId: ID of the vCPU pass to H_GUEST_CREATE_VCPU
      dataBuffer: A L1 real address of the GSB.
        If takeOwnershipOfVcpuState, size must be at least the size
        returned by ID=0x0001
      dataBufferSizeInBytes: Size of dataBuffer
    Output:
      R3: Return code
      R4: If R3 = H_Invalid_Element_Id: The array index of the bad
            element ID.
          If R3 = H_Invalid_Element_Size: The array index of the bad
             element size.
          If R3 = H_Invalid_Element_Value: The array index of the bad
             element value.

```
### H_GUEST_SET_STATE()


姝よ皟鐢ㄧ敤浜庤缃?L2 绾ф垨 vCPU 鐗瑰畾鐨?L2 鐘舵€併€傝淇℃伅閫氳繃瀹㈡埛鏈虹姸鎬佺紦鍐插尯锛圙SB锛変紶閫掞紝蹇呰缁嗚妭濡備笅锛?
杩欏彲浠ヨ缃?L2 绾ф垨 vCPU 鐗瑰畾鐨勪俊鎭€侺2 绾х殑渚嬪瓙鏈夋椂鍩哄亸绉绘垨杩涚▼浣滅敤鍩熼〉琛ㄤ俊鎭€倂CPU 鐗瑰畾鐨勪緥瀛愭湁 GPR 鎴?VSR銆俧lags 鍙傛暟涓殑涓€涓綅鎸囨槑姝よ皟鐢ㄦ槸 L2 绾ц繕鏄?vCPU 鐗瑰畾鐨勶紝骞朵笖 GSB 涓殑 ID 蹇呴』涓庝箣鍖归厤銆?
L1 鎻愪緵涓€涓寚鍚?GSB 鐨勬寚閽堜綔涓烘璋冪敤鐨勫弬鏁般€傚悓鏃舵彁渚涚殑杩樻湁涓庤璁剧疆鐨勭姸鎬佸叧鑱旂殑 L2 涓?vCPU ID銆?
L1 鍦?GSB 涓啓鍏ユ墍鏈夊€硷紝鑰?L0 鍙鍙?GSB 涓殑
```

  H_GUEST_SET_STATE(uint64 flags,
                    uint64 guestId,
                    uint64 vcpuId,
                    uint64 dataBuffer,
                    uint64 dataBufferSizeInBytes);
  Parameters:
    Input:
      flags:
         Bit 0: getGuestWideState: Request state of the Guest instead
           of an individual VCPU.
         Bit 1: returnOwnershipOfVcpuState Return Guest VCPU state. See
           GET_STATE takeOwnershipOfVcpuState
         Bits 2-63: Reserved
      guestId: ID obtained from H_GUEST_CREATE
      vcpuId: ID of the vCPU pass to H_GUEST_CREATE_VCPU
      dataBuffer: A L1 real address of the GSB.
        If takeOwnershipOfVcpuState, size must be at least the size
        returned by ID=0x0001
      dataBufferSizeInBytes: Size of dataBuffer
    Output:
      R3: Return code
      R4: If R3 = H_Invalid_Element_Id: The array index of the bad
            element ID.
          If R3 = H_Invalid_Element_Size: The array index of the bad
             element size.
          If R3 = H_Invalid_Element_Value: The array index of the bad
             element value.

```
### H_GUEST_RUN_VCPU()


姝よ皟鐢ㄧ敤浜庤繍琛屼竴涓?L2 vCPU銆侺2 涓?vCPU ID 浣滀负鍙傛暟浼犲叆銆傝 vCPU 浠ヤ箣鍓嶄娇鐢?H_GUEST_SET_STATE() 璁剧疆鐨勭姸鎬佽繍琛屻€傚綋 L2 閫€鍑烘椂锛孡1 灏嗕粠杩欎釜瓒呯骇璋冪敤澶勬仮澶嶆墽琛屻€?
杩欎釜瓒呯骇璋冪敤杩樻湁鍏宠仈鐨勮緭鍏ヤ笌杈撳嚭 GSB銆備笌 H_GUEST_{S,G}ET_STATE() 涓嶅悓锛岃繖浜?GSB 鎸囬拡涓嶆槸浣滀负瓒呯骇璋冪敤鐨勫弬鏁颁紶鍏ョ殑锛堣繖鏍峰仛鏄嚭浜庢€ц兘鑰冭檻锛夈€傝繖浜?GSB 鐨勪綅缃繀椤讳娇鐢?H_GUEST_SET_STATE() 璋冪敤銆佷互 ID 0x0c00 涓?0x0c01锛堣涓嬭〃锛夐鍏堟敞鍐屻€?
杈撳叆 GSB 鍙兘鍙寘鍚璁剧疆鐨?vCPU 鐗瑰畾鍏冪礌銆傝繖涓?GSB 涔熷彲浠ュ寘鍚浂涓厓绱狅紙鍗?GSB 鍓?4 瀛楄妭涓?0锛夛紝濡傛灉鏃犻渶璁剧疆浠讳綍涓滆タ鐨勮瘽銆?
浠庤秴绾ц皟鐢ㄩ€€鍑烘椂锛岃緭鍑虹紦鍐插尯琚～鍏ョ敱 L0 鍐冲畾鐨勫厓绱犮€傞€€鍑虹殑鍘熷洜鍖呭惈鍦?GPR4 涓紙鍗?NIP 琚斁鍏?GPR4锛夈€傝繑鍥炵殑鍏冪礌鍙栧喅浜庨€€鍑虹被鍨嬨€備緥濡傦紝濡傛灉閫€鍑哄師鍥犳槸 L2 鎵ц浜嗕竴涓秴绾ц皟鐢紙GPR4 = 0xc00锛夛紝閭ｄ箞 GPR3-12 浼氳鎻愪緵鍦ㄨ緭鍑?GSB 涓紝鍥犱负杩欐槸鏈嶅姟璇ヨ秴绾ц皟鐢ㄥ彲鑳介渶瑕佺殑鐘舵€併€傚鏋滈渶瑕侀澶栫殑鐘舵€侊紝L1 鍙互璋冪敤 H_GUEST_GET_STATE()銆?
瑕佸湪 L2 涓悎鎴愪腑鏂紝褰撹皟鐢?H_GUEST_RUN_VCPU() 鏃讹紝L1 鍙互璁剧疆涓€涓爣蹇楋紙浣滀负瓒呯骇璋冪敤鍙傛暟锛夛紝L0 灏变細鍦?L2 涓悎鎴愯涓柇銆傛垨鑰咃紝L1 涔熷彲浠ヤ娇鐢?H_GUEST_SET_STATE() 鑷鍚堟垚涓柇锛屾垨
```

  H_GUEST_RUN_VCPU(uint64 flags,
                   uint64 guestId,
                   uint64 vcpuId,
                   uint64 dataBuffer,
                   uint64 dataBufferSizeInBytes);
  Parameters:
    Input:
      flags:
         Bit 0: generateExternalInterrupt: Generate an external interrupt
         Bit 1: generatePrivilegedDoorbell: Generate a Privileged Doorbell
         Bit 2: sendToSystemReset鈥? Generate a System Reset Interrupt
         Bits 3-63: Reserved
      guestId: ID obtained from H_GUEST_CREATE
      vcpuId: ID of the vCPU pass to H_GUEST_CREATE_VCPU
    Output:
      R3: Return code
      R4: If R3 = H_Success: The reason L1 VCPU exited (ie. NIA)
            0x000: The VCPU stopped running for an unspecified reason. An
              example of this is the Hypervisor stopping a VCPU running
              due to an outstanding interrupt for the Host Partition.
            0x980: HDEC
            0xC00: HCALL
            0xE00: HDSI
            0xE20: HISI
            0xE40: HEA
            0xF80: HV Fac Unavail
          If R3 = H_Invalid_Element_Id, H_Invalid_Element_Size, or
            H_Invalid_Element_Value: R4 is offset of the invalid element
            in the input buffer.

```
### H_GUEST_DELETE()


姝よ皟鐢ㄧ敤浜庡垹闄や竴涓?L2銆傛墍鏈夊叧鑱旂殑 vCPU 涔熶細琚垹闄ゃ€備笉鎻愪緵鍗曠嫭鐨?vCPU 鍒犻櫎璋冪敤銆?
鍙互鎻愪緵涓€涓爣蹇楁潵鍒犻櫎鎵€鏈夊鎴锋満銆傝繖鐢ㄤ簬閲嶇疆
```

  H_GUEST_DELETE(uint64 flags,
                 uint64 guestId)
  Parameters:
    Input:
      flags:
         Bit 0: deleteAllGuests: deletes all guests
         Bits 1-63: Reserved
      guestId: ID obtained from H_GUEST_CREATE
    Output:
      R3: Return code

```
## 瀹㈡埛鏈虹姸鎬佺紦鍐插尯


瀹㈡埛鏈虹姸鎬佺紦鍐插尯锛圙SB锛夋槸 L1 涓?L0 涔嬮棿閫氳繃 H_GUEST_{G,S}ET() 涓?H_GUEST_VCPU_RUN() 璋冪敤娌熼€?L2 鐘舵€佺殑涓昏鏂规硶銆?
鐘舵€佸彲浠ヤ笌鏁翠釜 L2 鍏宠仈锛堜緥濡傛椂鍩哄亸绉伙級锛屼篃鍙互涓庣壒瀹氱殑 L2 vCPU 鍏宠仈锛堜緥濡?GPR 鐘舵€侊級銆傚彧鏈?L2 VCPU 鐘舵€佸彲鑳界敱 H_GUEST_VCPU_RUN() 璁剧疆銆?
GSB 涓殑鎵€鏈夋暟鎹兘鏄ぇ绔紙big endian锛夌殑锛堜笌 PAPR 涓殑鏍囧噯涓€鑷达級銆?
瀹㈡埛鏈虹姸鎬佺紦鍐插尯鏈変竴涓ご閮紝缁欏嚭鍏冪礌鐨勬暟閲忥紝闅忓悗鏄?GSB 鍏冪礌鏈韩銆?
GSB 澶撮儴锛?
+----------+----------+-------------------------------------------+
|  Offset  |  Size    |  Purpose                                  |
|  Bytes   |  Bytes   |                                           |
+==========+==========+===========================================+
|    0     |    4     |  Number of elements                       |
+----------+----------+-------------------------------------------+
|    4     |          |  Guest state buffer elements              |
+----------+----------+-------------------------------------------+

GSB 鍏冪礌锛?
+----------+----------+-------------------------------------------+
|  Offset  |  Size    |  Purpose                                  |
|  Bytes   |  Bytes   |                                           |
+==========+==========+===========================================+
|    0     |    2     |  ID                                       |
+----------+----------+-------------------------------------------+
|    2     |    2     |  Size of Value                            |
+----------+----------+-------------------------------------------+
|    4     | As above |  Value                                    |
+----------+----------+-------------------------------------------+

GSB 鍏冪礌涓殑 ID 鎸囧畾浜嗚璁剧疆浠€涔堛€傝繖鍖呮嫭鏋舵瀯鐘舵€侊紙濡?GPR銆乂SR銆丼PR锛夛紝浠ュ強涓€浜涘叧浜庡垎鍖虹殑鍏冩暟鎹紝濡傛椂鍩哄亸绉讳笌鍒嗗尯浣滅敤鍩熼〉琛ㄤ俊鎭€?
+--------+-------+----+--------+----------------------------------+
|   ID   | Size  | RW |(H)ost  | Details                          |
|        | Bytes |    |(G)uest |                                  |
|        |       |    |(T)hread|                                  |
|        |       |    |Scope   |                                  |
+========+=======+====+========+==================================+
| 0x0000 |       | RW |   TG   | NOP element                      |
+--------+-------+----+--------+----------------------------------+
| 0x0001 | 0x08  | R  |   G    | Size of L0 vCPU state. See:      |
|        |       |    |        | H_GUEST_GET_STATE:               |
|        |       |    |        | flags = takeOwnershipOfVcpuState |
+--------+-------+----+--------+----------------------------------+
| 0x0002 | 0x08  | R  |   G    | Size Run vCPU out buffer         |
+--------+-------+----+--------+----------------------------------+
| 0x0003 | 0x04  | RW |   G    | Logical PVR                      |
+--------+-------+----+--------+----------------------------------+
| 0x0004 | 0x08  | RW |   G    | TB Offset (L1 relative)          |
+--------+-------+----+--------+----------------------------------+
| 0x0005 | 0x18  | RW |   G    |Partition scoped page tbl info:   |
|        |       |    |        |                                  |
|        |       |    |        |- 0x00 Addr part scope table      |
|        |       |    |        |- 0x08 Num addr bits              |
|        |       |    |        |- 0x10 Size root dir              |
+--------+-------+----+--------+----------------------------------+
| 0x0006 | 0x10  | RW |   G    |Process Table Information:        |
|        |       |    |        |                                  |
|        |       |    |        |- 0x0 Addr proc scope table       |
|        |       |    |        |- 0x8 Table size.                 |
+--------+-------+----+--------+----------------------------------+
| 0x0007-|       |    |        | Reserved                         |
| 0x07FF |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x0800 | 0x08  | R  |   H    | Current usage in bytes of the    |
|        |       |    |        | L0's Guest Management Space      |
|        |       |    |        | for an L1-Lpar.                  |
+--------+-------+----+--------+----------------------------------+
| 0x0801 | 0x08  | R  |   H    | Max bytes available in the       |
|        |       |    |        | L0's Guest Management Space for  |
|        |       |    |        | an L1-Lpar                       |
+--------+-------+----+--------+----------------------------------+
| 0x0802 | 0x08  | R  |   H    | Current usage in bytes of the    |
|        |       |    |        | L0's Guest Page Table Management |
|        |       |    |        | Space for an L1-Lpar             |
+--------+-------+----+--------+----------------------------------+
| 0x0803 | 0x08  | R  |   H    | Max bytes available in the L0's  |
|        |       |    |        | Guest Page Table Management      |
|        |       |    |        | Space for an L1-Lpar             |
+--------+-------+----+--------+----------------------------------+
| 0x0804 | 0x08  | R  |   H    | Cumulative Reclaimed bytes from  |
|        |       |    |        | L0 Guest's Page Table Management |
|        |       |    |        | Space due to overcommit          |
+--------+-------+----+--------+----------------------------------+
| 0x0805-|       |    |        | Reserved                         |
| 0x0BFF |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x0C00 | 0x10  | RW |   T    |Run vCPU Input Buffer:            |
|        |       |    |        |                                  |
|        |       |    |        |- 0x0 Addr of buffer              |
|        |       |    |        |- 0x8 Buffer Size.                |
+--------+-------+----+--------+----------------------------------+
| 0x0C01 | 0x10  | RW |   T    |Run vCPU Output Buffer:           |
|        |       |    |        |                                  |
|        |       |    |        |- 0x0 Addr of buffer              |
|        |       |    |        |- 0x8 Buffer Size.                |
+--------+-------+----+--------+----------------------------------+
| 0x0C02 | 0x08  | RW |   T    | vCPU VPA Address                 |
+--------+-------+----+--------+----------------------------------+
| 0x0C03-|       |    |        | Reserved                         |
| 0x0FFF |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x1000-| 0x08  | RW |   T    | GPR 0-31                         |
| 0x101F |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x1020 |  0x08 | T  |   T    | HDEC expiry TB                   |
+--------+-------+----+--------+----------------------------------+
| 0x1021 | 0x08  | RW |   T    | NIA                              |
+--------+-------+----+--------+----------------------------------+
| 0x1022 | 0x08  | RW |   T    | MSR                              |
+--------+-------+----+--------+----------------------------------+
| 0x1023 | 0x08  | RW |   T    | LR                               |
+--------+-------+----+--------+----------------------------------+
| 0x1024 | 0x08  | RW |   T    | XER                              |
+--------+-------+----+--------+----------------------------------+
| 0x1025 | 0x08  | RW |   T    | CTR                              |
+--------+-------+----+--------+----------------------------------+
| 0x1026 | 0x08  | RW |   T    | CFAR                             |
+--------+-------+----+--------+----------------------------------+
| 0x1027 | 0x08  | RW |   T    | SRR0                             |
+--------+-------+----+--------+----------------------------------+
| 0x1028 | 0x08  | RW |   T    | SRR1                             |
+--------+-------+----+--------+----------------------------------+
| 0x1029 | 0x08  | RW |   T    | DAR                              |
+--------+-------+----+--------+----------------------------------+
| 0x102A | 0x08  | RW |   T    | DEC expiry TB                    |
+--------+-------+----+--------+----------------------------------+
| 0x102B | 0x08  | RW |   T    | VTB                              |
+--------+-------+----+--------+----------------------------------+
| 0x102C | 0x08  | RW |   T    | LPCR                             |
+--------+-------+----+--------+----------------------------------+
| 0x102D | 0x08  | RW |   T    | HFSCR                            |
+--------+-------+----+--------+----------------------------------+
| 0x102E | 0x08  | RW |   T    | FSCR                             |
+--------+-------+----+--------+----------------------------------+
| 0x102F | 0x08  | RW |   T    | FPSCR                            |
+--------+-------+----+--------+----------------------------------+
| 0x1030 | 0x08  | RW |   T    | DAWR0                            |
+--------+-------+----+--------+----------------------------------+
| 0x1031 | 0x08  | RW |   T    | DAWR1                            |
+--------+-------+----+--------+----------------------------------+
| 0x1032 | 0x08  | RW |   T    | CIABR                            |
+--------+-------+----+--------+----------------------------------+
| 0x1033 | 0x08  | RW |   T    | PURR                             |
+--------+-------+----+--------+----------------------------------+
| 0x1034 | 0x08  | RW |   T    | SPURR                            |
+--------+-------+----+--------+----------------------------------+
| 0x1035 | 0x08  | RW |   T    | IC                               |
+--------+-------+----+--------+----------------------------------+
| 0x1036-| 0x08  | RW |   T    | SPRG 0-3                         |
| 0x1039 |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x103A | 0x08  | W  |   T    | PPR                              |
+--------+-------+----+--------+----------------------------------+
| 0x103B | 0x08  | RW |   T    | MMCR 0-3                         |
| 0x103E |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x103F | 0x08  | RW |   T    | MMCRA                            |
+--------+-------+----+--------+----------------------------------+
| 0x1040 | 0x08  | RW |   T    | SIER                             |
+--------+-------+----+--------+----------------------------------+
| 0x1041 | 0x08  | RW |   T    | SIER 2                           |
+--------+-------+----+--------+----------------------------------+
| 0x1042 | 0x08  | RW |   T    | SIER 3                           |
+--------+-------+----+--------+----------------------------------+
| 0x1043 | 0x08  | RW |   T    | BESCR                            |
+--------+-------+----+--------+----------------------------------+
| 0x1044 | 0x08  | RW |   T    | EBBHR                            |
+--------+-------+----+--------+----------------------------------+
| 0x1045 | 0x08  | RW |   T    | EBBRR                            |
+--------+-------+----+--------+----------------------------------+
| 0x1046 | 0x08  | RW |   T    | AMR                              |
+--------+-------+----+--------+----------------------------------+
| 0x1047 | 0x08  | RW |   T    | IAMR                             |
+--------+-------+----+--------+----------------------------------+
| 0x1048 | 0x08  | RW |   T    | AMOR                             |
+--------+-------+----+--------+----------------------------------+
| 0x1049 | 0x08  | RW |   T    | UAMOR                            |
+--------+-------+----+--------+----------------------------------+
| 0x104A | 0x08  | RW |   T    | SDAR                             |
+--------+-------+----+--------+----------------------------------+
| 0x104B | 0x08  | RW |   T    | SIAR                             |
+--------+-------+----+--------+----------------------------------+
| 0x104C | 0x08  | RW |   T    | DSCR                             |
+--------+-------+----+--------+----------------------------------+
| 0x104D | 0x08  | RW |   T    | TAR                              |
+--------+-------+----+--------+----------------------------------+
| 0x104E | 0x08  | RW |   T    | DEXCR                            |
+--------+-------+----+--------+----------------------------------+
| 0x104F | 0x08  | RW |   T    | HDEXCR                           |
+--------+-------+----+--------+----------------------------------+
| 0x1050 | 0x08  | RW |   T    | HASHKEYR                         |
+--------+-------+----+--------+----------------------------------+
| 0x1051 | 0x08  | RW |   T    | HASHPKEYR                        |
+--------+-------+----+--------+----------------------------------+
| 0x1052 | 0x08  | RW |   T    | CTRL                             |
+--------+-------+----+--------+----------------------------------+
| 0x1053 | 0x08  | RW |   T    | DPDES                            |
+--------+-------+----+--------+----------------------------------+
| 0x1054-|       |    |        | Reserved                         |
| 0x1FFF |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x2000 | 0x04  | RW |   T    | CR                               |
+--------+-------+----+--------+----------------------------------+
| 0x2001 | 0x04  | RW |   T    | PIDR                             |
+--------+-------+----+--------+----------------------------------+
| 0x2002 | 0x04  | RW |   T    | DSISR                            |
+--------+-------+----+--------+----------------------------------+
| 0x2003 | 0x04  | RW |   T    | VSCR                             |
+--------+-------+----+--------+----------------------------------+
| 0x2004 | 0x04  | RW |   T    | VRSAVE                           |
+--------+-------+----+--------+----------------------------------+
| 0x2005 | 0x04  | RW |   T    | DAWRX0                           |
+--------+-------+----+--------+----------------------------------+
| 0x2006 | 0x04  | RW |   T    | DAWRX1                           |
+--------+-------+----+--------+----------------------------------+
| 0x2007-| 0x04  | RW |   T    | PMC 1-6                          |
| 0x200c |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x200D | 0x04  | RW |   T    | WORT                             |
+--------+-------+----+--------+----------------------------------+
| 0x200E | 0x04  | RW |   T    | PSPB                             |
+--------+-------+----+--------+----------------------------------+
| 0x200F-|       |    |        | Reserved                         |
| 0x2FFF |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x3000-| 0x10  | RW |   T    | VSR 0-63                         |
| 0x303F |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0x3040-|       |    |        | Reserved                         |
| 0xEFFF |       |    |        |                                  |
+--------+-------+----+--------+----------------------------------+
| 0xF000 | 0x08  | R  |   T    | HDAR                             |
+--------+-------+----+--------+----------------------------------+
| 0xF001 | 0x04  | R  |   T    | HDSISR                           |
+--------+-------+----+--------+----------------------------------+
| 0xF002 | 0x04  | R  |   T    | HEIR                             |
+--------+-------+----+--------+----------------------------------+
| 0xF003 | 0x08  | R  |   T    | ASDR                             |
+--------+-------+----+--------+----------------------------------+


## 鏉傞」淇℃伅


### 涓嶅湪 ptregs/hvregs 涓殑鐘舵€?

鍦?v1 API 涓紝鏌愪簺鐘舵€佷笉鍦?ptregs/hvstate 涓€傝繖鍖呮嫭鍚戦噺瀵勫瓨鍣ㄤ笌鏌愪簺 SPR銆備负浜嗚 L1 涓?L2 璁剧疆姝ょ姸鎬侊紝L1 鍦?h_enter_nested() 璋冪敤涔嬪墠杞藉叆杩欎簺纭欢瀵勫瓨鍣紝鑰?L0 纭繚瀹冧滑鏈€缁堟垚涓?L2 鐘舵€侊紙閫氳繃涓嶅幓瑙︾瀹冧滑锛夈€?
v2 API 绉婚櫎浜嗚繖涓€鐐癸紝骞堕€氳繃 GSB 鏄惧紡鍦拌缃鐘舵€併€?
### L1 瀹炵幇缁嗚妭锛氱紦瀛樼姸鎬?

鍦?v1 API 涓紝鎵€鏈夌姸鎬侀兘鍦ㄦ瘡娆?h_enter_nested() 瓒呯骇璋冪敤鏃朵粠 L1 鍙戝線 L0锛屽弽涔嬩害鐒躲€傚鏋?L0 褰撳墠娌℃湁杩愯浠讳綍 L2锛孡0 灏辨病鏈夊叧浜庡畠浠殑鐘舵€佷俊鎭€傚敮涓€鐨勪緥澶栨槸閫氳繃 h_set_partition_table() 娉ㄥ唽鐨勫垎鍖鸿〃鐨勪綅缃€?
v2 API 鏀瑰彉浜嗚繖涓€鐐癸紝浣垮緱 L0 鍗充娇鍦ㄥ畠鐨?vCPU 涓嶅啀杩愯鏃朵篃淇濈暀 L2 鐘舵€併€傝繖鎰忓懗鐫€ L1 鍙渶瑕佸湪闇€瑕佷慨鏀?L2 鐘舵€併€佹垨鑰呭畠鐨勫€煎凡杩囨湡鏃讹紝鎵嶄笌 L0 娌熼€?L2 鐘舵€併€傝繖鎻愪緵浜嗕竴涓€ц兘浼樺寲鐨勬満浼氥€?
褰撲竴涓?vCPU 浠?H_GUEST_RUN_VCPU() 璋冪敤閫€鍑烘椂锛孡1 鍦ㄥ唴閮ㄥ皢鎵€鏈?L2 鐘舵€佹爣璁颁负鏃犳晥銆傝繖鎰忓懗鐫€濡傛灉 L1 鎯宠鐭ラ亾 L2 鐘舵€侊紙姣斿閫氳繃 kvm_get_one_reg() 璋冪敤锛夛紝瀹冮渶瑕佽皟鐢?H_GUEST_GET_STATE() 鏉ヨ幏鍙栬鐘舵€併€備竴鏃﹁鍙栵紝瀹冨湪 L1 涓鏍囪涓烘湁鏁堬紝鐩村埌 L2 鍐嶆杩愯銆?
姝ゅ锛屽綋 L1 淇敼 L2 vCPU 鐘舵€佹椂锛屽畠涓嶉渶瑕佸湪 L2 vCPU 鍐嶆杩愯涔嬪墠鎶婂畠鍐欏叆 L0銆傚洜姝ゅ綋 L1 鏇存柊鐘舵€侊紙姣斿閫氳繃 kvm_set_one_reg() 璋冪敤锛夋椂锛屽畠鍐欏叆涓€涓唴閮ㄧ殑 L1 鍓湰锛屽苟涓斿彧鍦?L2 閫氳繃 H_GUEST_VCPU_RUN() 杈撳叆缂撳啿鍖哄啀娆¤繍琛屾椂锛屾墠鎶婅繖涓壇鏈埛鏂板埌 L0銆?
L1 杩欑鎯版€ф洿鏂扮姸鎬佺殑鍋氭硶閬垮厤浜嗕笉蹇呰鐨?H_GUEST_{G|S}ET_STATE() 璋冪敤銆?