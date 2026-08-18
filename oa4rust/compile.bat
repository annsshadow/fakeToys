@echo off
chcp 65001 >nul
echo ========================================
echo   oa4rust 编译脚本
echo ========================================
echo.

:: 检查 Rust 工具链
echo [1/3] 检查 Rust 工具链...
rustc --version >nul 2>&1
if errorlevel 1 (
    echo [错误] 未检测到 Rust，请先安装 Rust 1.75+
    echo 下载地址: https://www.rust-lang.org/tools/install
    pause
    exit /b 1
)
rustc --version
cargo --version
echo.

:: 清理旧的编译产物（可选）
echo [2/3] 清理旧编译产物...
cargo clean
echo.

:: 编译整个 workspace
echo [3/3] 编译整个 workspace...
cargo build --workspace 2>&1
if errorlevel 1 (
    echo.
    echo [错误] 编译失败，请检查上方错误信息
    pause
    exit /b 1
)

echo.
echo ========================================
echo   编译成功！
echo   可执行文件位于: target\debug\oa4rust.exe
echo ========================================
pause
