
## eCryptfs锛歀inux 鐨勫爢鍙犲紡鍔犲瘑鏂囦欢绯荤粺


eCryptfs 鏄嚜鐢辫蒋浠躲€傝鎯呰鍙傞槄 COPYING 鏂囦欢銆?鏂囨。璇峰弬闃?doc/ 瀛愮洰褰曚腑鐨勬枃浠躲€傛瀯寤轰笌瀹夎璇存槑璇峰弬闃?INSTALL 鏂囦欢銆?
:Maintainer: Phillip Hellewell
:Lead developer: Michael A. Halcrow <mhalcrow@us.ibm.com>
:Developers: Michael C. Thompson
             Kent Yoder
:Web Site: http://ecryptfs.sf.net

鏈蒋浠跺綋鍓嶄粛鍦ㄥ紑鍙戜腑銆傝鍔″繀瀵瑰啓鍏?eCryptfs 鐨勪换浣曟暟鎹?淇濈暀涓€浠藉浠藉壇鏈€?
eCryptfs 闇€瑕佸彲浠?SourceForge 绔欑偣涓嬭浇鐨勭敤鎴风┖闂村伐鍏凤細

http://sourceforge.net/projects/ecryptfs/

鐢ㄦ埛绌洪棿闇€姹傚寘鎷細

- David Howells 鐨勭敤鎴风┖闂村瘑閽ョ幆澶存枃浠朵笌搴擄紙鐗堟湰 1.0 鎴栨洿楂橈級锛?  鍙粠浠ヤ笅鍦板潃鑾峰彇锛?  http://people.redhat.com/~dhowells/keyutils/
- Libgcrypt



   鍦?eCryptfs 鐨勬祴璇曠増/瀹為獙鎬у彂甯冧腑锛屽崌绾?eCryptfs 鏃讹紝浣犲簲璇ュ厛灏嗘枃浠?   澶嶅埗鍒版湭鍔犲瘑鐨勪綅缃紝鐒跺悗鍐嶅皢鏂囦欢澶嶅埗鍥炴柊鐨?eCryptfs 鎸傝浇鐐癸紝
   浠ヨ縼绉昏繖浜涙枃浠躲€?

## 鎸傝浇绾у彛浠?

鍒涘缓涓€涓柊鐩綍锛宔Cryptfs 灏嗘妸鍔犲瘑鏂囦欢鍐欏叆鍏朵腑锛堜緥濡?/root/crypt锛夈€?鐒跺悗锛屽垱寤烘寕杞界偣鐩綍

```

    mount -t ecryptfs /root/crypt /mnt/crypt

```
绯荤粺浼氭彁绀轰綘杈撳叆鍙ｄ护涓庣洂鍊硷紙鐩愬€煎彲浠ヤ负绌猴級銆?
```

    echo "Hello, World" > /mnt/crypt/hello.txt

```
鎿嶄綔灏嗗畬鎴愩€傛敞鎰?/root/crypt 涓嚭鐜颁簡涓€涓ぇ灏忚嚦灏戜负 12288 瀛楄妭
锛堝彇鍐充簬瀹夸富鏈洪〉澶у皬锛夌殑鏂版枃浠躲€傝繖灏辨槸浣犲垰鍒氬啓鍏ュ唴瀹圭殑鍔犲瘑搴曞眰鏂囦欢銆?瑕佸畬鏁村湴娴嬭瘯璇诲彇锛屼綘闇€瑕佹竻绌虹敤鎴蜂細璇濆瘑閽ョ幆锛?
keyctl clear @u

鐒跺悗鎸夌収涓婇潰缁欏嚭鐨勮鏄庡嵏杞?/mnt/crypt 骞堕噸鏂版寕杞姐€?
```

    cat /mnt/crypt/hello.txt


```
## 娉ㄦ剰浜嬮」


eCryptfs 0.1 鐗堟湰鍙簲鎸傝浇鍒帮紙1锛夌┖鐩綍锛屾垨锛?锛変粎鍖呭惈鐢?eCryptfs
鍒涘缓鐨勬枃浠剁殑鐩綍涓€傚鏋滀綘鎸傝浇涓€涓寘鍚潪 eCryptfs 鍒涘缓鐨?鏃㈡湁鏂囦欢鐨勭洰褰曪紝鍏惰涓烘槸鏈畾涔夌殑銆傞櫎闈炵函绮瑰嚭浜庤皟璇曟垨寮€鍙戠洰鐨勶紝
鍚﹀垯涓嶈浠ユ洿楂樼殑璇︾粏绾у埆杩愯 eCryptfs锛屽洜涓哄湪閭ｇ鎯呭喌涓?鏈哄瘑鍊间細琚啓鍏ョ郴缁熸棩蹇椼€?

Mike Halcrow
mhalcrow@us.ibm.com
