## Linux 鐗规湁鐨?ELF 鐗规€?


## 瀹氫箟


"绗竴涓?绋嬪簭澶存槸鏂囦欢涓亸绉婚噺鏈€灏忕殑閭ｄ釜锛歟_phoff銆?

"鏈€鍚庝竴涓?绋嬪簭澶存槸鏂囦欢涓亸绉婚噺鏈€澶х殑閭ｄ釜锛歟_phoff + (e_phnum - 1) * sizeof(Elf_Phdr)銆?

## PT_INTERP


绗竴涓?PT_INTERP 绋嬪簭澶寸敤浜庡畾浣?ELF 瑙ｉ噴鍣ㄧ殑鏂囦欢鍚嶃€傚叾浠?PT_INTERP 澶磋蹇界暐锛堣嚜 Linux 2.4.11 璧凤級銆?

## PT_GNU_STACK


鏈€鍚庝竴涓?PT_GNU_STACK 绋嬪簭澶村畾涔夌敤鎴风┖闂存爤鐨勫彲鎵ц鎬э紙鑷?Linux 2.6.6 璧凤級銆傚叾浠?PT_GNU_STACK 澶磋蹇界暐銆?

## PT_GNU_PROPERTY


ELF 瑙ｉ噴鍣ㄧ殑鏈€鍚庝竴涓?PT_GNU_PROPERTY 绋嬪簭澶磋浣跨敤锛堣嚜 Linux 5.8 璧凤級銆傝嫢瑙ｉ噴鍣ㄦ病鏈夎澶达紝鍒欎娇鐢ㄥ彲鎵ц鏂囦欢鐨勬渶鍚庝竴涓?PT_GNU_PROPERTY 绋嬪簭澶淬€傚叾浠?PT_GNU_PROPERTY 澶磋蹇界暐銆?
