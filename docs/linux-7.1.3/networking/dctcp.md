## DCTCP锛堟暟鎹腑蹇?TCP锛?

DCTCP 鏄鐢ㄤ簬鏁版嵁涓績缃戠粶鐨?TCP 鎷ュ鎺у埗绠楁硶鐨勫寮猴紝瀹冨埄鐢ㄦ暟鎹腑蹇冪綉缁滀腑鐨?鏄惧紡鎷ュ閫氱煡锛圗CN锛夊悜缁堢涓绘満鎻愪緵澶氫綅鍙嶉銆?
```

  sysctl -w net.ipv4.tcp_congestion_control=dctcp
  sysctl -w net.ipv4.tcp_ecn_fallback=0 (optional)

```
杩愯 DCTCP 鐨勬暟鎹腑蹇冪綉缁滀腑鐨勬墍鏈変氦鎹㈡満蹇呴』鏀寔 ECN 鏍囪锛屽苟琚厤缃负鍦ㄨ揪鍒?瀹氫箟鐨勪氦鎹㈡満缂撳啿鍖洪槇鍊兼椂杩涜鏍囪銆備氦鎹㈡満涓?DCTCP 鐨勯粯璁?ECN 鏍囪闃堝€煎惎鍙戝紡
鍊间负 1Gbps 鏃?20 涓暟鎹寘锛?0KB锛夛紝10Gbps 鏃?65 涓暟鎹寘锛堢害 100KB锛夛紝浣嗗彲鑳?闇€瑕佽繘涓€姝ヤ粩缁嗚皟鏁淬€?
鏈夊叧鏇村缁嗚妭锛岃鍙傞槄浠ヤ笅鏂囨。锛?
璁烘枃锛?
璇ョ畻娉曞湪浠ヤ笅涓ょ瘒 SIGCOMM/SIGMETRICS 璁烘枃涓湁杩涗竴姝ヨ缁嗘弿杩帮細

 i) Mohammad Alizadeh, Albert Greenberg, David A. Maltz, Jitendra Padhye,
    Parveen Patel, Balaji Prabhakar, Sudipta Sengupta, and Murari Sridharan:

      "Data Center TCP (DCTCP)", Data Center Networks session"

      Proc. ACM SIGCOMM, New Delhi, 2010.

    http://simula.stanford.edu/~alizade/Site/DCTCP_files/dctcp-final.pdf
    http://www.sigcomm.org/ccr/papers/2010/October/1851275.1851192

ii) Mohammad Alizadeh, Adel Javanmard, and Balaji Prabhakar:

      "Analysis of DCTCP: Stability, Convergence, and Fairness"
      Proc. ACM SIGMETRICS, San Jose, 2011.

    http://simula.stanford.edu/~alizade/Site/DCTCP_files/dctcp_analysis-full.pdf

IETF 淇℃伅鎬ц崏妗堬細

  http://tools.ietf.org/html/draft-bensley-tcpm-dctcp-00

DCTCP 绔欑偣锛?
  http://simula.stanford.edu/~alizade/Site/DCTCP.html
