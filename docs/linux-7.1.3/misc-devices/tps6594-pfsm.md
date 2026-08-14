
## Texas Instruments TPS6594 PFSM 椹卞姩


Author: Julien Panis (jpanis@baylibre.com)

## 姒傝堪


涓ユ牸鏉ヨ锛孭FSM锛堥閰嶇疆鏈夐檺鐘舵€佹満锛孭re-configurable Finite State Machine锛夊苟闈炵‖浠躲€傚畠鏄竴娈典唬鐮併€?
TPS6594 PMIC锛堢數婧愮鐞?IC锛孭ower Management IC锛夐泦鎴愪簡涓€涓鐞嗚繍琛屾ā寮忕殑鐘舵€佹満銆傛牴鎹綋鍓嶇殑杩愯妯″紡锛屾煇浜涚數鍘嬪煙淇濇寔涓婄數锛岃€屽叾浠栧煙鍙互鍏抽棴銆?
PFSM 椹卞姩鍙敤浜庤Е鍙戝凡閰嶇疆鐘舵€佷箣闂寸殑杞崲銆傚畠杩樻彁渚涘璁惧瀵勫瓨鍣ㄧ殑璇?鍐欒闂€?
### 鏀寔鐨勮澶?

- tps6594-q1
- tps6593-q1
- lp8764-q1

## 椹卞姩浣嶇疆


drivers/misc/tps6594-pfsm.c

## 椹卞姩绫诲瀷瀹氫箟


include/uapi/linux/tps6594_pfsm.h

## 椹卞姩 IOCTL


`PMIC_GOTO_STANDBY`
鎵€鏈夎澶囪祫婧愬潎鏂數銆傚鐞嗗櫒鍏抽棴锛屾病鏈変换浣曠數鍘嬪煙涓婄數銆?
`PMIC_GOTO_LP_STANDBY`
PMIC 涓笉闇€瑕佸父寮€鐨勬暟瀛椾笌妯℃嫙鍔熻兘琚叧闂紙浣庡姛鑰楋級銆?
`PMIC_UPDATE_PGM`
瑙﹀彂鍥轰欢鏇存柊銆?
`PMIC_SET_ACTIVE_STATE`
杩愯妯″紡涔嬩竴銆?PMIC 瀹屽叏姝ｅ父宸ヤ綔锛屽苟鍚戞墍鏈?PDN 璐熻浇渚涚數銆?MCU 涓庝富澶勭悊鍣ㄤ袱涓儴鍒嗙殑鐢靛帇鍩熷潎涓婄數銆?
`PMIC_SET_MCU_ONLY_STATE`
杩愯妯″紡涔嬩竴銆?浠呮湁鍒嗛厤缁?MCU Safety Island 鐨勭數婧愯祫婧愬紑鍚€?
`PMIC_SET_RETENTION_STATE`
杩愯妯″紡涔嬩竴銆?鏍规嵁鎵€璁剧疆鐨勮Е鍙戝櫒锛岄儴鍒?DDR/GPIO 鐢靛帇鍩熷彲淇濇寔涓婄數锛岃€屾墍鏈夊叾浠栧煙鍏抽棴锛屼互鏈€灏忓寲绯荤粺鎬诲姛鑰椼€?
## 椹卞姩浣跨敤


```

    # ls /dev/pfsm*

```
```

    # hexdump -C /dev/pfsm-0-0x48

```
```

    # cat /proc/interrupts

```
### 鐢ㄦ埛绌洪棿浠ｇ爜绀轰緥


samples/pfsm/pfsm-wakeup.c
