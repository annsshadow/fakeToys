## Linux 2.6.x 鍦?MPC52xx 绯诲垪涓?

鏈€鏂颁俊鎭璁块棶 https://www.246tNt.com/mpc52xx/

瑕佺紪璇?浣跨敤 锛?
```
     # <缂栬緫 Makefile锛岃缃?ARCH=ppc 涓?CROSS_COMPILE=...锛堝鏋滈渶瑕佷篃鍙缃?EXTRAVERSION锛夈€?     # make lite5200_defconfig
     # make uImage

     鐒跺悗锛屽湪 U-Boot 涓細
     => tftpboot 200000 uImage
     => tftpboot 400000 pRamdisk
     => bootm 200000 400000

  - DBug::

     # <缂栬緫 Makefile锛岃缃?ARCH=ppc 涓?CROSS_COMPILE=...锛堝鏋滈渶瑕佷篃鍙缃?EXTRAVERSION锛夈€?     # make lite5200_defconfig
     # cp your_initrd.gz arch/ppc/boot/images/ramdisk.image.gz
     # make zImage.initrd
     # make

     鐒跺悗鍦?DBug 涓細
     DBug> dn -i zImage.initrd.lite5200

```

涓€浜涜鏄庯細

 - 璇ョЩ妞嶅悕涓?mpc52xxx锛岄厤缃€夐」涓?PPC_MPC52xx銆侻GT5100 涓嶅彈鏀寔锛屾垜涓嶇‘瀹氭槸鍚︽湁浜烘湁鍏磋叮瀵瑰叾杩涜寮€鍙戙€傛垜娌℃湁閲囩敤 5xxx锛屾槸鍥犱负鏄剧劧瀛樺湪澶ч噺涓?MPC5200 姣棤鍏崇郴鐨?5xxx銆傚嚭浜庡悓鏍风殑鍘熷洜锛屾垜鍔犲叆浜?'MPC'銆? - 褰撶劧锛屾垜鍊熼壌浜?2.4 鐗堟湰鐨勭Щ妞嶃€傚鏋滀綘璁や负鎴戝湪鏌愪簺浠ｇ爜鐨勭増鏉冨０鏄庝腑閬楁紡浜嗕綘/浣犵殑鍏徃锛屾垜浼氬敖蹇洿姝ｃ€?