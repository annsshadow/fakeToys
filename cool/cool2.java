import org.apache.commons.codec.binary.Base64;
import javax.crypto.Cipher;
import javax.crypto.spec.IvParameterSpec;
import javax.crypto.spec.SecretKeySpec;
import java.net.URLDecoder;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.security.SecureRandom;
import java.util.Arrays;

/**
 * 2020/12/14 11:58
 */
public class EncryptUtil {
    /**
     * Base64 编码
     */
    private static final Base64 B64 = new Base64();
    /**
     * 安全的随机数源
     */
    private static final SecureRandom RANDOM = new SecureRandom();
    /**
     * AES加密算法
     */
    private static final String AES_ALGORITHM = "AES";
    private static final String AES = "AES/ECB/PKCS5Padding";

    public static String MD5(String str) {
        String re_md5 = "";
        try {
            MessageDigest md = MessageDigest.getInstance("MD5");
            md.update(str.getBytes());
            byte b[] = md.digest();
            int i;
            StringBuilder buf = new StringBuilder("");
            for (byte aB : b) {
                i = aB;
                if (i < 0) {
                    i += 256;
                }
                if (i < 16) {
                    buf.append("0");
                }
                buf.append(Integer.toHexString(i));
            }
            re_md5 = buf.toString();
        } catch (NoSuchAlgorithmException e) {
            e.printStackTrace();
        }
        return re_md5;
    }

    /**
     * AES加密
     *
     * @param str 需要加密的明文
     * @param key 密钥
     * @return 加密后的密文(str / key为null返回null)
     */
    public static String aesEncryp(String str, String key) {
        return aesEncryp(str, key, false);
    }

    /**
     * AES加密
     *
     * @param str       需要加密的明文
     * @param key       密钥
     * @param urlSafety 密文是否需要Url安全
     * @return 加密后的密文(str / key为null返回null)
     */
    public static String aesEncryp(String str, String key, boolean urlSafety) {
        if (null != str && null != key) {
            try {
                Cipher c = Cipher.getInstance(AES);
                c.init(Cipher.ENCRYPT_MODE, aesKey(key), RANDOM);
// 加密
                byte[] bytes = c.doFinal(str.getBytes("UTF-8"));
                if (urlSafety) {
                    return Base64.encodeBase64URLSafeString(bytes);
                } else {
                    return new String(B64.encode(bytes));
                }
            } catch (Exception e) {
//AES加密失败
                return new BaseOut(2, "AES加密失败, 密文：" + str + ", key：" + key, null).toString();
            }
        }
        return null;
    }

    /**
     * AES密钥
     */
    private static SecretKeySpec aesKey(String key) {
        byte[] bs = key.getBytes();
        if (bs.length != 16) {
            bs = Arrays.copyOf(bs, 16);// 处理数组长度为16
        }
        return new SecretKeySpec(bs, AES_ALGORITHM);
    }

    public static String oaMd5() {
        String key = "coolcollege20201211sc";
        String thirdSecret = "135990bd839c5fe0a1ca9cbee2475431";
        return MD5(key + thirdSecret);
    }

    public static void main(String[] args) {
//1.用户UserId免登Token生成
        StringBuffer sb1 = new StringBuffer();
        sb1.append("userId=<YOUR_USER_ID>").append("&").append("enterpriseId = <YOUR_ENTERPRISE_ID>");
        System.out.println("UserId登录加密前str:" + sb1.toString());
        String userIdToken = aesEncryp(sb1.toString(), oaMd5());
        System.out.println("UserId登录免登token:" + userIdToken);
//2.用户工号免登Token生成
        StringBuffer sb2 = new StringBuffer();
        sb2.append("userId=<YOUR_USER_ID>").append("&").append("enterpriseId=<YOUR_ENTERPRISE_ID>").append("&").append("type=JOB_NUMBER");
        System.out.println("工号登录加密前str:" + sb2.toString());
        String jobnumberToken = aesEncryp(sb2.toString(), oaMd5());
        System.out.println("工号登录免登token:" + jobnumberToken);
//3.用户手机号免登Token生成
        StringBuffer sb3 = new StringBuffer();
        sb3.append("userId=<YOUR_MOBILE>").append("&").append("enterpriseId=<YOUR_ENTERPRISE_ID>").append("&").append("type=LOGIN_MOBILE");
        System.out.println("手机号登录加密前str:" + sb3.toString());
        String mobileToken = aesEncryp(sb3.toString(), oaMd5());
        System.out.println("手机号登录免登token:" + mobileToken);
//4.用户邮箱免登Token生成
        StringBuffer sb4 = new StringBuffer();
        sb4.append("userId=user@example.com").append("&").append("enterpriseId=<YOUR_ENTERPRISE_ID>").append("&").append("type=LOGIN_EMAIL");
        System.out.println("邮箱登录加密前str:" + sb4.toString());
        String emailToken = aesEncryp(sb4.toString(), oaMd5());
        System.out.println("邮箱登录免登token:" + emailToken);
    }
}