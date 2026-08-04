@echo off
chcp 65001 >nul
echo ========================================
echo   编译 shared crate
echo ========================================
echo.

cargo build -p shared 2>&1
if errorlevel 1 (
    echo [错误] shared 编译失败
    pause
    exit /b 1
)

echo [成功] shared 编译完成
pause
