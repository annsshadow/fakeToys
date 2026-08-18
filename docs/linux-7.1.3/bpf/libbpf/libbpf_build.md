## 鏋勫缓 libbpf


libelf 涓?zlib 鏄?libbpf 鐨勫唴閮ㄤ緷璧栵紝鍥犳闇€瑕佷笌涔嬮摼鎺ワ紝骞朵笖蹇呴』瀹夎鍦ㄧ郴缁熶笂浠ヤ究搴旂敤绋嬪簭姝ｅ父宸ヤ綔銆傞粯璁や娇鐢?pkg-config 鏌ユ壘 libelf锛屾墍璋冪敤鐨勭▼搴忓彲閫氳繃 PKG_CONFIG 瑕嗙洊銆?
濡傛灉鍦ㄦ瀯寤烘椂涓嶅笇鏈涗娇鐢?pkg-config锛屽彲鍦ㄨ皟鐢?make 鏃惰缃?NO_PKG_CONFIG=1 灏嗗叾绂佺敤銆?
瑕佸悓鏃舵瀯寤洪潤鎬佸簱 libbpf.a 涓庡叡浜簱 libbpf.so锛?

    $ cd src
    $ make

瑕佷粎鍦?build/ 鐩綍涓瀯寤洪潤鎬佸簱 libbpf.a锛屽苟杩炲悓 libbpf 澶存枃浠朵竴璧峰畨瑁呭埌鏆傚瓨鐩綍 root/锛?

    $ cd src
    $ mkdir build root
    $ BUILD_STATIC_ONLY=y OBJDIR=build DESTDIR=root make install

瑕侀拡瀵瑰畨瑁呭湪 /build/root/ 涓嬬殑鑷畾涔?libelf 渚濊禆鏋勫缓闈欐€佸簱 libbpf.a 涓庡叡浜簱 libbpf.so锛屽苟杩炲悓 libbpf 澶存枃浠朵竴璧峰畨瑁呭埌鏋勫缓鐩綍 /build/root/锛?

    $ cd src
    $ PKG_CONFIG_PATH=/build/root/lib64/pkgconfig DESTDIR=/build/root make
