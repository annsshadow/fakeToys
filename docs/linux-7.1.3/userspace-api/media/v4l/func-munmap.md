######## V4L2 munmap()


## 鍚嶇О


v4l2-munmap - 瑙ｉ櫎璁惧鍐呭瓨鏄犲皠

## 鎽樿



    #include <unistd.h>
    #include <sys/mman.h>


## 鍙傛暟


`start`
    鐢?`mmap()` 鍑芥暟杩斿洖鐨勫凡鏄犲皠缂撳啿鍖虹殑鍦板潃銆?

`length`
    宸叉槧灏勭紦鍐插尯鐨勯暱搴︺€傝鍊煎繀椤讳笌浼犵粰 `mmap()` 鐨勫€肩浉鍚岋紝涓斿浜庡崟骞抽潰锛坰ingle-planar锛堿PI 鐢遍┍鍔ㄥ湪缁撴瀯浣?`v4l2_buffer` 鐨?`length` 瀛楁杩斿洖锛屽浜庡骞抽潰锛坢ulti-planar锛堿PI 鐢辩粨鏋勪綋 `v4l2_plane` 鐨?`length` 瀛楁杩斿洖銆?

## 璇存槑


瑙ｉ櫎鍏堝墠閫氳繃 `mmap()` 鍑芥暟鏄犲皠鐨勭紦鍐插尯锛屽苟鍦ㄥ彲鑳芥椂閲婃斁瀹冦€?

## 杩斿洖鍊?


鎴愬姛鏃?`munmap()` 杩斿洖 0锛屽け璐ユ椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺锛?

EINVAL
    `start` 鎴?`length` 涓嶆纭紝鎴栬€呭皻鏈槧灏勪换浣曠紦鍐插尯銆?
