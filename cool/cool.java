package net.coolcollege.login.helper;

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
    /**
     * 字符集
     */
    private static final String CHARSET = "UTF-8";
    private static final String AES_CBC = "AES/CBC/PKCS5Padding";

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
     * AES解密
     *
     * @param str 需要解密的密文(base64编码字符串)
     * @param key 密钥
     * @return 解密后的明文
     */
    public static String aesDecrypt(String str, String key) {
        if (null != str && null != key) {
            try {
                Cipher c = Cipher.getInstance(AES);
                c.init(Cipher.DECRYPT_MODE, aesKey(key), RANDOM);
// 解密
                return new String(c.doFinal(B64.decode(str)), "UTF-8");
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
        return null;
    }

    /**
     * aes 解密
     * 兼容.net 解密算法
     *
     * @param data 密文
     * @return
     */
    public static String decryptData(String data, String key) {
        String IV = key;
        if (key.length() > 16) {
// IV为商户MD5密钥后16位
            IV = key.substring(key.length() - 16);
// RES的KEY 为商户MD5密钥的前16位
            key = key.substring(0, 16);
        }
        try {
            byte[] encrypted1 = Base64.decodeBase64(data.getBytes("UTF-8"
            ));
            Cipher cipher = Cipher.getInstance(AES_CBC);
            SecretKeySpec keyspec = new SecretKeySpec(key.getBytes(), AES_ALGORITHM);
            IvParameterSpec ivspec = new IvParameterSpec(IV.getBytes());
            cipher.init(Cipher.DECRYPT_MODE, keyspec, ivspec);
            return new String(cipher.doFinal(encrypted1), "UTF-8");
        } catch (Exception e) {
            e.printStackTrace();
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

    /**
     * AES加密
     *
     * @param data 密文
     * @return
     */
    public static String decryptData(String data, byte[] key) throws Exception {
        try {
            byte[] encrypted1 = Base64.decodeBase64(data.getBytes(CHARSET
            ));
            Cipher cipher = Cipher.getInstance(AES);
            SecretKeySpec keySpec = new SecretKeySpec(key, AES_ALGORITHM);
            cipher.init(Cipher.DECRYPT_MODE, keySpec);
            byte[] original = cipher.doFinal(encrypted1);
            String originalString = new String(original, CHARSET);
            return originalString;
        } catch (Exception e) {
            throw e;
        }
    }

    /**
     * 根据传入密文完成解码
     *
     * @param ssoKey
     * @return
     * @throws Exception
     */
    public static String decodeTicket(String ssoKey) throws Exception {
        String key = "whM1376SiX5=78";
        MessageDigest md = MessageDigest.getInstance("MD5");
        md.update(key.getBytes());
        byte[] digest = md.digest();
        try {
            return decryptData(URLDecoder.decode(ssoKey, CHARSET), digest
            );
        } catch (Exception e) {
            return decryptData(ssoKey, digest);
        }
    }

    public static void main(String[] args) {
//1.用户UserId免登Token生成
        StringBuffer sb1 = new StringBuffer();
        sb1.append("userId=<YOUR_USER_ID>").append("&").append("enterpriseId=<YOUR_ENTERPRISE_ID>");
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

class BaseOut {
    private int code = 0;
    private String msg;
    private Object data;

    public BaseOut() {
    }

    public BaseOut(int code) {
        this.code = code;
    }

    public BaseOut(int code, String msg) {
        this.code = code;
        this.msg = msg;
    }

    public BaseOut(int code, String msg, Object data) {
        this.code = code;
        this.msg = msg;
        this.data = data;
    }

    public int getCode() {
        return code;
    }

    public void setCode(int code) {
        this.code = code;
    }

    public String getMsg() {
        return msg;
    }

    public void setMsg(String msg) {
        this.msg = msg;
    }

    public Object getData() {
        return data;
    }

    public void setData(Object data) {
        this.data = data;
    }
}