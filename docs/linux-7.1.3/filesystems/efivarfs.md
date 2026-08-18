## efivarfs - 涓€涓?(U)EFI 鍙橀噺鏂囦欢绯荤粺


efivarfs 鏂囦欢绯荤粺琚垱寤哄嚭鏉ワ紝浠ヨВ鍐充娇鐢?sysfs 涓殑鏉＄洰鏉ョ淮鎶?EFI 鍙橀噺鐨勪笉瓒炽€傛棫鐨?sysfs EFI 鍙橀噺浠ｇ爜鍙敮鎸佹渶澶?1024 瀛楄妭鐨勫彉閲忋€傝闄愬埗鍦?EFI 瑙勮寖鐨?0.99 鐗堟湰涓瓨鍦紝浣嗗湪浠讳綍姝ｅ紡鍙戝竷鐗堜箣鍓嶅氨琚Щ闄や簡銆傜敱浜庡彉閲忕幇鍦ㄥ彲鑳藉ぇ浜庡崟涓〉闈紝sysfs 骞朵笉鏄鐞嗘闂鐨勬渶浣虫帴鍙ｃ€?

鍙橀噺鍙互閫氳繃 efivarfs 鏂囦欢绯荤粺鍒涘缓銆佸垹闄ゅ拰淇敼銆?

```
	mount -t efivarfs none /sys/firmware/efi/efivars
```
鐢变簬瀛樺湪澶ч噺鍥轰欢缂洪櫡锛屽叾涓Щ闄ら潪鏍囧噯鐨?UEFI 鍙橀噺浼氬鑷寸郴缁熷浐浠舵棤娉曞畬鎴?POST锛堝姞鐢佃嚜妫€锛夛紝efivarfs 涓皢閭ｄ簺闈炲箍涓轰汉鐭ョ殑鏍囧噯鍖栧彉閲忓垱寤轰负涓嶅彲鍙樻枃浠躲€傝繖骞朵笉闃绘鍒犻櫎鈥斺€?chattr -i" 浠嶇劧鏈夋晥鈥斺€斾絾鍙互闃叉姝ょ被鏁呴殰琚剰澶栬Е鍙戙€?

      褰撴樉绀?/sys/firmware/efi/efivars 涓煇涓?UEFI 鍙橀噺鐨勫唴瀹规椂锛堜緥濡備娇鐢?"hexdump"锛夛紝璇锋敞鎰忚緭鍑虹殑鍓?4 涓瓧鑺備唬琛?UEFI 鍙橀噺灞炴€э紝閲囩敤灏忕鏍煎紡銆?

      瀹為檯涓婏紝姣忎釜 efivar 鐨勮緭鍑虹敱浠ヤ笅鍐呭缁勬垚锛?

          +-----------------------------------+
          |4_bytes_of_attributes + efivar_data|
          +-----------------------------------+

**鍙﹁鍙傞槄锛?*

- Documentation/admin-guide/acpi/ssdt-overlays.rst
- Documentation/ABI/removed/sysfs-firmware-efi-vars
