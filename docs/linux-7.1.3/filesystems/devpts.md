
## Devpts 鏂囦欢绯荤粺


鐜板湪姣忔 devpts 鏂囦欢绯荤粺鐨勬寕杞介兘鏄嫭绔嬬殑锛屼娇寰楀湪涓€涓寕杞戒腑鍒嗛厤鐨?pty 鍙婂叾
绱㈠紩鐙珛浜庢墍鏈夊叾浠栨寕杞戒腑鐨?pty 鍙婂叾绱㈠紩銆?
鐜板湪鎵€鏈?devpts 鏂囦欢绯荤粺鐨勬寕杞介兘浼氬垱寤轰竴涓潈闄愪负 `0000` 鐨?`/dev/pts/ptmx`
鑺傜偣銆?
涓轰繚鎸佸悜鍚庡吋瀹癸紝褰撴墦寮€浠?`mknod name c 5 2` 鍒涘缓鐨?ptmx 璁惧鑺傜偣锛堝嵆浠讳綍
姝ょ被鑺傜偣锛夋椂锛屼細鍦ㄤ笌 ptmx 璁惧鑺傜偣鐩稿悓鐨勭洰褰曚笅鏌ユ壘鍚嶄负 `pts` 鐨?devpts
瀹炰緥銆?
浣滀负涓€绉嶉€夋嫨锛岄櫎浜嗗湪 `/dev/ptmx` 鏀剧疆 `/dev/ptmx` 璁惧鑺傜偣澶栵紝涔熷彲浠ュ湪
`/dev/ptmx` 鏀剧疆鎸囧悜 `/dev/pts/ptmx` 鐨勭鍙烽摼鎺ワ紝鎴栧皢 `/dev/ptx/ptmx` 缁戝畾
鎸傝浇鍒?`/dev/ptmx`銆傚鏋滀綘閫夋嫨浠ヨ繖绉嶆柟寮忎娇鐢?devpts 鏂囦欢绯荤粺锛屽垯搴斾互
`ptmxmode=0666` 鎸傝浇 devpts锛屾垨璋冪敤 `chmod 0666 /dev/pts/ptmx`銆?
```

    kernel.pty.max = 4096	- 鍏ㄥ眬闄愬埗
    kernel.pty.reserve = 1024	- 涓轰粠鍒濆鎸傝浇鍛藉悕绌洪棿鎸傝浇鐨勬枃浠剁郴缁熶繚鐣?    kernel.pty.nr		- 褰撳墠 pty 璁℃暟

```
姣忓疄渚嬮檺鍒跺彲閫氳繃娣诲姞鎸傝浇閫夐」 `max=<count>` 璁剧疆銆?
姝ょ壒鎬у湪鍐呮牳 3.4 涓笌 `sysctl kernel.pty.reserve` 涓€骞跺姞鍏ャ€?
鍦ㄦ棭浜?3.4 鐨勫唴鏍镐腑锛宻ysctl `kernel.pty.max` 浣滀负姣忓疄渚嬮檺鍒跺伐浣溿€?