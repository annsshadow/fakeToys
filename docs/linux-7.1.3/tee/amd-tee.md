
## AMD-TEE锛圓MD 鐨勫彲淇℃墽琛岀幆澧冿級

AMD-TEE 椹卞姩璐熻矗涓?AMD 鐨?TEE 鐜杩涜閫氫俊銆傝 TEE 鐜鐢?AMD Secure
Processor 鎻愪緵銆?
AMD Secure Processor锛堝墠韬О涓?Platform Security Processor锛岀畝绉?PSP锛夋槸涓€棰?涓撶敤澶勭悊鍣紝鍏峰 ARM TrustZone 鎶€鏈紝浠ュ強涓烘敮鎸佺涓夋柟鍙俊搴旂敤锛圱rusted
Application锛夎€岃璁＄殑鍩轰簬杞欢鐨勫彲淇℃墽琛岀幆澧冿紙TEE锛夈€傜洰鍓嶈鍔熻兘浠呭 APU 鍚敤銆?
```

                                             |
    x86                                      |
                                             |
 User space            (Kernel space)        |    AMD Secure Processor (PSP)
 ~~~~~~~~~~            ~~~~~~~~~~~~~~        |    ~~~~~~~~~~~~~~~~~~~~~~~~~~
                                             |
 +--------+                                  |       +-------------+
 | Client |                                  |       | Trusted     |
 +--------+                                  |       | Application |
     /\                                      |       +-------------+
     ||                                      |             /\
     ||                                      |             ||
     ||                                      |             \/
     ||                                      |         +----------+
     ||                                      |         |   TEE    |
     ||                                      |         | Internal |
     \/                                      |         |   API    |
 +---------+           +-----------+---------+         +----------+
 | TEE     |           | TEE       | AMD-TEE |         | AMD-TEE  |
 | Client  |           | subsystem | driver  |         | Trusted  |
 | API     |           |           |         |         |   OS     |
 +---------+-----------+----+------+---------+---------+----------+
 |   Generic TEE API        |      | ASP     |      Mailbox       |
 |   IOCTL (TEE_IOC_*)      |      | driver  | Register Protocol  |
 +--------------------------+      +---------+--------------------+

```
鍦ㄦ渶搴曞眰锛坸86 涓婏級锛孉MD Secure Processor锛圓SP锛夐┍鍔ㄤ娇鐢?CPU 鍒?PSP 鐨?mailbox
瀵勫瓨鍣ㄥ悜 PSP 鎻愪氦鍛戒护銆傚懡浠ょ紦鍐插尯鐨勬牸寮忓 ASP 椹卞姩鏄笉閫忔槑鐨勩€傚畠鐨勮亴璐ｆ槸鍚戝畨鍏?澶勭悊鍣ㄦ彁浜ゅ懡浠わ紝骞跺皢缁撴灉杩斿洖缁?AMD-TEE 椹卞姩銆侫MD-TEE 椹卞姩涓?AMD Secure
Processor 椹卞姩涔嬮棿鐨勬帴鍙ｅ彲鍦?[^1^] 涓壘鍒般€?
AMD-TEE 椹卞姩灏嗗懡浠ょ紦鍐插尯璐熻浇鎵撳寘锛屼互渚垮湪 TEE 涓鐞嗐€備笉鍚?TEE 鍛戒护鐨勫懡浠ょ紦鍐?鍖烘牸寮忓彲鍦?[^2^] 涓壘鍒般€?
AMD-TEE Trusted OS 鏀寔鐨?TEE 鍛戒护鍖呮嫭锛?
- TEE_CMD_ID_LOAD_TA          - 灏嗕竴涓彲淇″簲鐢紙TA锛変簩杩涘埗鏂囦欢鍔犺浇鍒?TEE 鐜涓€?- TEE_CMD_ID_UNLOAD_TA        - 浠?TEE 鐜涓嵏杞?TA 浜岃繘鍒舵枃浠躲€?- TEE_CMD_ID_OPEN_SESSION     - 涓庡凡鍔犺浇鐨?TA 鎵撳紑涓€涓細璇濄€?- TEE_CMD_ID_CLOSE_SESSION    - 鍏抽棴涓庡凡鍔犺浇 TA 鐨勪細璇濄€?- TEE_CMD_ID_INVOKE_CMD       - 璋冪敤宸插姞杞?TA 鐨勪竴涓懡浠ゃ€?- TEE_CMD_ID_MAP_SHARED_MEM   - 鏄犲皠鍏变韩鍐呭瓨銆?- TEE_CMD_ID_UNMAP_SHARED_MEM - 鍙栨秷鏄犲皠鍏变韩鍐呭瓨銆?
AMD-TEE Trusted OS 鏄繍琛屽湪 AMD Secure Processor 涓婄殑鍥轰欢銆?
AMD-TEE 椹卞姩鍚?TEE 瀛愮郴缁熸敞鍐岃嚜韬紝骞跺疄鐜颁互涓嬮┍鍔ㄥ嚱鏁板洖璋冿細

- get_version - 杩斿洖椹卞姩瀹炵幇 id 涓庤兘鍔涳紙capability锛夈€?- open - 璁剧疆椹卞姩涓婁笅鏂囨暟鎹粨鏋勩€?- release - 閲婃斁椹卞姩璧勬簮銆?- open_session - 鍔犺浇 TA 浜岃繘鍒舵枃浠跺苟涓庡凡鍔犺浇鐨?TA 鎵撳紑浼氳瘽銆?- close_session - 鍏抽棴涓庡凡鍔犺浇 TA 鐨勪細璇濆苟鍗歌浇瀹冦€?- invoke_func - 璋冪敤宸插姞杞?TA 鐨勪竴涓懡浠ゃ€?
AMD-TEE 涓嶆敮鎸?cancel_req 椹卞姩鍥炶皟銆?
鐢ㄦ埛绌洪棿锛堝鎴风锛夊彲浠ヤ娇鐢?GlobalPlatform TEE Client API [^3^] 涓?AMD 鐨?TEE
閫氫俊銆侫MD 鐨?TEE 涓哄姞杞姐€佹墦寮€浼氳瘽銆佽皟鐢ㄥ懡浠や互鍙婂叧闂笌 TA 鐨勪細璇濇彁渚涗簡涓€涓畨鍏?鐜銆?
## 鍙傝€冭祫鏂?
[^1^] include/linux/psp-tee.h

[^2^] drivers/tee/amdtee/amdtee_if.h

[^3^] http://www.globalplatform.org/specificationsdevice.asp look for
    "TEE Client API Specification v1.0" and click download.
