@echo off
chcp 65001 >nul
echo ========================================
echo   Cool 项目编译 (Java)
echo ========================================

javac --version >nul 2>&1
if errorlevel 1 (
    echo [错误] 未检测到 JDK，请先安装 JDK 11+
    pause
    exit /b 1
)

echo [1/1] 编译 Java 源文件 ...
javac -encoding UTF-8 -d . cool.java
if errorlevel 1 (
    echo [错误] 编译失败
    pause
    exit /b 1
)

echo.
echo [编译结果]
dir /b *.class 2>nul
echo.
echo ========================================
echo   编译完成
echo ========================================
pause
