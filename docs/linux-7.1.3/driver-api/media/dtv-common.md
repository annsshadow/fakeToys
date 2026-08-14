
### 鏁板瓧鐢佃閫氱敤鍑芥暟


#### DVB 璁惧


杩欎簺鍑芥暟璐熻矗澶勭悊 DVB 璁惧鑺傜偣銆?

#### 鏁板瓧鐢佃鐜舰缂撳啿鍖?

杩欎簺渚嬬▼瀹炵幇浜嗙敤浜庡鐞嗘暟瀛楃數瑙嗘暟鎹苟鍦ㄥ叾涓庣敤鎴风┖闂翠箣闂村鍒剁殑鐜舰缂撳啿鍖恒€?

  1) 鍑轰簬鎬ц兘鑰冭檻锛岃鍐欎緥绋嬩笉妫€鏌ョ紦鍐插尯澶у皬鍙?鎴栫┖闂?鍙敤瀛楄妭鏁般€?     杩欏繀椤诲湪璋冪敤杩欎簺渚嬬▼涔嬪墠瀹屾垚銆備緥濡傦細

   .. code-block:: c

        /** write @buflen: bytes **/
        free = dvb_ringbuffer_free(rbuf);
        if (free >= buflen)
                count = dvb_ringbuffer_write(rbuf, buffer, buflen);
        else
                /** do something **/

        /** read min. 1000, max. @bufsize: bytes **/
        avail = dvb_ringbuffer_avail(rbuf);
        if (avail >= 1000)
                count = dvb_ringbuffer_read(rbuf, buffer, min(avail, bufsize));
        else
                /** do something **/

  2) 濡傛灉鎭板ソ鏈変竴涓鍙栬€呭拰涓€涓啓鍏ヨ€咃紝鍒欐棤闇€瀵硅鎴栧啓鎿嶄綔鍔犻攣銆?     涓や釜鎴栨洿澶氳鍙栬€呬箣闂村繀椤讳簰鏂ュ姞閿併€?     鍒锋柊缂撳啿鍖虹畻浣滀竴娆¤鎿嶄綔銆?     閲嶇疆缂撳啿鍖虹畻浣滀竴娆¤鍜屽啓鎿嶄綔銆?     涓や釜鎴栨洿澶氬啓鍏ヨ€呬箣闂村繀椤讳簰鏂ュ姞閿併€?

#### 鏁板瓧鐢佃 VB2 澶勭悊鍣?