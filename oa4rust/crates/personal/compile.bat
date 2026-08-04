@echo off
chcp 65001 >nul
echo ========================================
echo   编译 personal crate
echo ========================================
echo.

cargo build -p personal 2>&1
if errorlevel 1 (
    echo [错误] personal 编译失败
    pause
    exit /b 1
)

echo [成功] personal 编译完成
pause
