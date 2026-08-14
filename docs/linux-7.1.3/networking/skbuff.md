## struct sk_buff


`sk_buff` 鏄〃绀烘暟鎹寘鐨勪富瑕佺綉缁滅粨鏋勩€?

### 鍩烘湰 sk_buff 鍑犱綍甯冨眬


   :doc: 鍩烘湰 sk_buff 鍑犱綍甯冨眬

### 鍏变韩 skb 涓?skb 鍏嬮殕


:c`sk_buff.users` 鏄竴涓畝鍗曠殑寮曠敤璁℃暟锛屽厑璁稿涓疄浣撲繚鎸?struct sk_buff 瀛樻椿銆傚叿鏈?`sk_buff.users != 1` 鐨?skb 琚О涓哄叡浜?skb锛堣 skb_shared()锛夈€?

skb_clone() 鍏佽蹇€熷鍒?skb銆傛病鏈変换浣曟暟鎹紦鍐插尯琚鍒讹紝浣嗚皟鐢ㄨ€呬細鑾峰緱涓€涓柊鐨勫厓鏁版嵁缁撴瀯浣擄紙struct sk_buff锛夈€?
&skb_shared_info.refcount 琛ㄧず鎸囧悜鍚屼竴鏁版嵁鍖呮暟鎹紙鍗冲厠闅嗭級鐨?skb 鏁伴噺銆?

### dataref 涓庢棤澶撮儴 skb


   :doc: dataref 涓庢棤澶撮儴 skb

### 鏍￠獙鍜屼俊鎭?


   :doc: skb 鏍￠獙鍜?
