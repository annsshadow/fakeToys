
## Flexcan CAN 鎺у埗鍣ㄩ┍鍔?


Authors: Marc Kleine-Budde <mkl@pengutronix.de>,
Dario Binacchi <dario.binacchi@amarulasolutions.com>

## RTR 甯ф帴鏀剁殑寮€鍚?鍏抽棴


瀵逛簬澶у鏁?flexcan IP 鏍革紝璇ラ┍鍔ㄦ敮鎸佷袱绉?RX 妯″紡锛?

- FIFO
- mailbox

杈冩棫鐨?flexcan 鏍革紙闆嗘垚浜?i.MX25銆乮.MX28銆乮.MX35
鍜?i.MX53 SoC锛変粎鍦ㄦ帶鍒跺櫒閰嶇疆涓?RX-FIFO 妯″紡鏃?
鎵嶈兘鎺ユ敹 RTR 甯с€?

RX FIFO 妯″紡浣跨敤娣卞害涓?6 涓?CAN 甯х殑纭欢 FIFO锛?
鑰?mailbox 妯″紡浣跨敤娣卞害鏈€楂樿揪 62 涓?
CAN 甯х殑杞欢 FIFO銆傚€熷姪鏇村ぇ鐨勭紦鍐插尯锛宮ailbox 妯″紡
鍦ㄩ珮绯荤粺璐熻浇涓嬭〃鐜版洿濂姐€?

鐢变簬鎺ユ敹 RTR 甯ф槸 CAN 鏍囧噯鐨勪竴閮ㄥ垎锛屾墍鏈?flexcan
鏍镐笂鐢垫椂澶勪簬鍙帴鏀?RTR 甯х殑妯″紡銆?

閫氳繃 "rx-rtr" 绉佹湁鏍囧織锛屽彲浠ユ斁寮冩帴鏀?RTR 甯х殑鑳藉姏锛?
浠ｄ环鏄け鍘绘帴鏀?RTR
娑堟伅鐨勮兘鍔涖€傝繖绉嶆潈琛″湪鏌愪簺鐢ㄤ緥涓槸鏈夊埄鐨勩€?

"rx-rtr" on
  鎺ユ敹 RTR 甯с€傦紙榛樿锛?

  CAN 鎺у埗鍣ㄨ兘澶熷苟涓斿皢浼氭帴鏀?RTR 甯с€?

  鍦ㄦ煇浜?IP 鏍镐笂锛屾帶鍒跺櫒鏃犳硶鍦ㄦ€ц兘鏇村ソ鐨?"RX mailbox" 妯″紡涓?
  鎺ユ敹 RTR 甯э紝鑰屼細浣跨敤 "RX FIFO" 妯″紡
  浠ｆ浛銆?

"rx-rtr" off

  鏀惧純鎺ユ敹 RTR 甯х殑鑳藉姏銆傦紙骞堕潪鎵€鏈?IP 鏍搁兘鏀寔锛?

  璇ユā寮忎細婵€娲?"RX mailbox 妯″紡" 浠ヨ幏寰楁洿濂芥€ц兘锛屽湪鏌愪簺
  IP 鏍镐笂鍒欐棤娉曞啀鎺ユ敹 RTR 甯с€?

```

    ip link set dev can0 down
    ethtool --set-priv-flags can0 rx-rtr {off|on}
    ip link set dev can0 up

```
