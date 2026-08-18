
## QTEE锛圦ualcomm 鍙俊鎵ц鐜锛?

QTEE 椹卞姩澶勭悊涓?Qualcomm TEE [^1^] 鐨勯€氫俊銆?
涓?QTEE 鐨勬渶浣庡眰绾ч€氫俊寤虹珛鍦?ARM SMC 璋冪敤绾﹀畾锛圫MCCC锛塠^2^] 涔嬩笂锛屽悗鑰呮槸 QTEE
鍐呴儴浣跨敤鐨?Secure Channel Manager锛圫CM锛塠^3^] 鐨勫熀纭€銆?
鍦ㄥ熀浜?QTEE 鐨勭郴缁熶腑锛屾湇鍔¤琛ㄧず涓哄璞★紝杩欎簺瀵硅薄甯︽湁涓€绯诲垪鍙璋冪敤浠ヤ骇鐢熺粨鏋?锛堝寘鎷叾瀹冨璞★級鐨勬搷浣溿€?
褰撲竴涓璞℃墭绠″湪 QTEE 鍐呴儴鏃讹紝鎵ц鍏舵搷浣滆绉颁负鈥滅洿鎺ヨ皟鐢ㄢ€濓紙direct invocation锛夈€?QTEE 涔熷彲浠ラ€氳繃涓€绉嶇О涓衡€滃洖璋冭姹傗€濓紙callback request锛夌殑鏂规硶璋冪敤鎵樼鍦ㄩ潪瀹夊叏
涓栫晫鐨勫璞°€?
SCM 鎻愪緵涓や釜鍑芥暟鏉ユ敮鎸佺洿鎺ヨ皟鐢ㄥ拰鍥炶皟璇锋眰锛?
- QCOM_SCM_SMCINVOKE_INVOKE锛氱敤浜庣洿鎺ヨ皟鐢ㄣ€傚畠鍙互杩斿洖涓€涓粨鏋滄垨鍙戣捣涓€涓?  鍥炶皟璇锋眰銆?- QCOM_SCM_SMCINVOKE_CB_RSP锛氱敤浜庢彁浜ゅ鍏堝墠鐩存帴璋冪敤瑙﹀彂鐨勫洖璋冭姹傜殑鍝嶅簲銆?
QTEE 浼犺緭娑堟伅 [^4^] 寤虹珛鍦?SCM 椹卞姩鍑芥暟涔嬩笂銆?
涓€鏉℃秷鎭敱涓?QTEE 鍏变韩鐨勪袱涓紦鍐插尯缁勬垚锛氬叆绔欑紦鍐插尯鍜屽嚭绔欑紦鍐插尯銆傚叆绔欑紦鍐插尯
鐢ㄤ簬鐩存帴璋冪敤锛屽嚭绔欑紦鍐插尯鐢ㄤ簬鍙戣捣鍥炶皟璇锋眰銆備笅鍥惧睍绀轰簡
```
                                      +---------------------+
                                      |                     v
    +-----------------+-------+-------+------+--------------------------+
    | qcomtee_msg_    |object | buffer       |                          |
    |  object_invoke  |  id   | offset, size |                          | (inbound buffer)
    +-----------------+-------+--------------+--------------------------+
    <---- header -----><---- arguments ------><- in/out buffer payload ->

                                      +-----------+
                                      |           v
    +-----------------+-------+-------+------+----------------------+
    | qcomtee_msg_    |object | buffer       |                      |
    |  callback       |  id   | offset, size |                      | (outbound buffer)
    +-----------------+-------+--------------+----------------------+

```
姣忎釜缂撳啿鍖轰互涓€涓ご閮ㄥ拰涓€缁勫弬鏁版暟缁勫紑濮嬨€?
QTEE 浼犺緭娑堟伅鏀寔鍥涚绫诲瀷鐨勫弬鏁帮細

- Input Object锛圛O锛岃緭鍏ュ璞★級鏄綋鍓嶈皟鐢ㄦ垨鍥炶皟璇锋眰鐨勫璞″弬鏁般€?- Output Object锛圤O锛岃緭鍑哄璞★級鏄綋鍓嶈皟鐢ㄦ垨鍥炶皟璇锋眰鐨勫璞″弬鏁般€?- Input Buffer锛圛B锛岃緭鍏ョ紦鍐插尯锛夋槸鎸囧悜鍏ョ珯鎴栧嚭绔欏尯鍩熺殑 (offset, size) 瀵癸紝
  鐢ㄤ簬瀛樺偍褰撳墠璋冪敤鎴栧洖璋冭姹傜殑鍙傛暟銆?- Output Buffer锛圤B锛岃緭鍑虹紦鍐插尯锛夋槸鎸囧悜鍏ョ珯鎴栧嚭绔欏尯鍩熺殑 (offset, size) 瀵癸紝
  鐢ㄤ簬瀛樺偍鏉ヨ嚜褰撳墠璋冪敤鎴栧洖璋冭姹傜殑鍙傛暟銆?
鍚勭粍浠跺湪 QTEE 涓浉浜掑叧绯荤殑绀烘剰鍥?```
         User space               Kernel                     Secure world
         ~~~~~~~~~~               ~~~~~~                     ~~~~~~~~~~~~
   +--------+   +----------+                                +--------------+
   | Client |   |callback  |                                | Trusted      |
   +--------+   |server    |                                | Application  |
      /\        +----------+                                +--------------+
      ||  +----------+ /\                                          /\
      ||  |callback  | ||                                          ||
      ||  |server    | ||                                          \/
      ||  +----------+ ||                                   +--------------+
      ||       /\      ||                                   | TEE Internal |
      ||       ||      ||                                   | API          |
      \/       \/      \/   +--------+--------+             +--------------+
   +---------------------+  | TEE    | QTEE   |             | QTEE         |
   |   libqcomtee [5]    |  | subsys | driver |             | Trusted OS   |
   +-------+-------------+--+----+-------+----+-------------+--------------+
   |      Generic TEE API        |       |   QTEE MSG                      |
   |      IOCTL (TEE_IOC_*)      |       |   SMCCC (QCOM_SCM_SMCINVOKE_*)  |
   +-----------------------------+       +---------------------------------+

```
## 鍙傝€?

[^1^] https://docs.qualcomm.com/bundle/publicresource/topics/80-70015-11/qualcomm-trusted-execution-environment.html

[^2^] http://infocenter.arm.com/help/topic/com.arm.doc.den0028a/index.html

[^3^] drivers/firmware/qcom/qcom_scm.c

[^4^] drivers/tee/qcomtee/qcomtee_msg.h

[^5^] https://github.com/quic/quic-teec
