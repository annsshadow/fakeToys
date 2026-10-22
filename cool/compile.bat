@echo off
chcp 65001 >nul
echo ========================================
echo   Cool 项目编译 (Java)
echo ========================================

rem `javac -version`（而非 --version）在 JDK 8 上也可用，与 README 的 JDK 8+ 声明一致。
javac -version >nul 2>&1
if errorlevel 1 (
    echo [错误] 未检测到 JDK，请先安装 JDK 8+
    pause
    exit /b 1
)

rem ── 定位 Apache Commons Codec ────────────────────────────────────────────
rem 源码 import 了 org.apache.commons.codec.binary.Base64，缺这个 jar 编译必失败。
rem 查找顺序：lib\commons-codec*.jar → 环境变量 COMMONS_CODEC_JAR → 报错退出。
set "CP="
if exist "lib\commons-codec*.jar" (
    for %%f in ("lib\commons-codec*.jar") do set "CP=%%~ff"
)
if not defined CP if defined COMMONS_CODEC_JAR set "CP=%COMMONS_CODEC_JAR%"
if not defined CP (
    echo [错误] 未找到 commons-codec jar。
    echo        请放到 lib\commons-codec-*.jar，或设置环境变量 COMMONS_CODEC_JAR 指向该 jar。
    echo        下载：https://commons.apache.org/proper/commons-codec/download_codec.cgi
    pause
    exit /b 1
)

echo [1/1] 编译 Java 源文件 ...
rem -Xlint:all 打开全部 lint，-Werror 把任意告警升级为错误：
rem 本目录的编译门禁是「0 告警」，一旦出现告警这里直接失败。
javac -Xlint:all -Werror -encoding UTF-8 -cp "%CP%" -d . EncryptUtil.java
if errorlevel 1 (
    echo [错误] 编译失败
    pause
    exit /b 1
)

echo.
echo [编译结果]
dir /b /s net\coolcollege\login\helper\*.class 2>nul
echo.
echo ========================================
echo   编译完成（0 告警）
echo ========================================
pause
