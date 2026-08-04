@echo off
chcp 65001 >nul
echo ========================================
echo   编译 auth crate
echo ========================================
echo.

cargo build -p auth 2>&1
if errorlevel 1 (
    echo [错误] auth 编译失败
    pause
    exit /b 1
)

echo [成功] auth 编译完成
pause
