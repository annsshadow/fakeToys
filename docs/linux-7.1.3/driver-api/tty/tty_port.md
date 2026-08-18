
## TTY Port

鏈〉闈粙缁?TTY 瀛愮郴缁熶腑 struct tty_port 杈呭姪鏈哄埗鍙婂叾瀵瑰 API锛岄潰鍚?TTY 椹卞姩寮€鍙戣€咃紝娑电洊绔彛鐨勫垵濮嬪寲銆佹墦寮€/鍏抽棴/鎸傛柇澶勭悊銆佸紩鐢ㄨ鏁颁笌璋冨埗瑙ｈ皟鍣ㄤ俊鍙锋帶鍒剁瓑杈呭姪鍑芥暟銆?


寤鸿 TTY 椹卞姩灏藉彲鑳戒娇鐢?struct tty_port 杈呭姪鍑芥暟銆傚鏋滈┍鍔ㄥ疄鐜颁簡
:c`tty_port.ops.activate()` 涓?:c`tty_port.ops.shutdown()`锛屽畠浠彲浠ュ湪鐩稿簲鐨?
:c`tty_struct.ops` 閽╁瓙涓娇鐢?tty_port_open()銆乼ty_port_close() 涓?
tty_port_hangup()銆?

寮曠敤涓庣粏鑺傚寘鍚湪搴曢儴鐨?`TTY Port Reference`_ 涓?`TTY Port Operations Reference`_
灏忚妭涓€?

## TTY Port 鍑芥暟


### 鍒濆鍖栦笌閿€姣?


   :identifiers: tty_port_init tty_port_destroy
        tty_port_get tty_port_put

### Open/Close/Hangup 杈呭姪鍑芥暟


   :identifiers: tty_port_install tty_port_open tty_port_block_til_ready
        tty_port_close tty_port_close_start tty_port_close_end tty_port_hangup
        tty_port_shutdown

### TTY 寮曠敤璁℃暟


   :identifiers: tty_port_tty_get tty_port_tty_set

### TTY 杈呭姪鍑芥暟


   :identifiers: tty_port_tty_hangup tty_port_tty_vhangup
   :identifiers: tty_port_tty_wakeup

### 璋冨埗瑙ｈ皟鍣ㄤ俊鍙?


   :identifiers: tty_port_carrier_raised tty_port_raise_dtr_rts
        tty_port_lower_dtr_rts

----

## TTY Port 寮曠敤


   :identifiers: tty_port

----

## TTY Port 鎿嶄綔寮曠敤


   :identifiers: tty_port_operations
