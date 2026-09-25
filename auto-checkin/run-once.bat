@echo off
REM Called by Windows Task Scheduler: repair the browser, run all sites once, then exit.
REM Keep this file ASCII-only: cmd.exe mis-parses UTF-8 non-ASCII bytes under GBK.
setlocal EnableExtensions EnableDelayedExpansion

REM Establish a failure channel before probing Node/npm or changing directories.
set "WRAPPER_LOG=%~dp0logs\task-wrapper.log"
if not exist "%~dp0logs" mkdir "%~dp0logs" >nul 2>&1
if not exist "%~dp0logs" set "WRAPPER_LOG=%TEMP%\auto-checkin-task-wrapper.log"
echo [%DATE% %TIME%] wrapper:start >> "%WRAPPER_LOG%"
cd /d "%~dp0"
if errorlevel 1 (
  echo [%DATE% %TIME%] wrapper:cd_failed >> "%WRAPPER_LOG%"
  exit /b 1
)

REM Prefer the Node version used by the task, but fall back to stable npm shims.
REM Validate each candidate instead of trusting a stale or broken first path.
set "NPM_CMD="
call :try_npm "C:\Users\Administrator\AppData\Local\nvm\v20.18.1\npm.cmd"
call :try_npm "%APPDATA%\npm\npm.cmd"
call :try_npm "%ProgramFiles%\nodejs\npm.cmd"
for /f "delims=" %%I in ('where npm 2^>nul') do call :try_npm "%%I"
if not defined NPM_CMD (
  echo [%DATE% %TIME%] wrapper:no_working_npm >> "%WRAPPER_LOG%"
  echo ERROR: no working npm was found; cannot run auto-checkin.
  exit /b 9009
)

echo [%DATE% %TIME%] wrapper:npm=%NPM_CMD% >> "%WRAPPER_LOG%"

REM Playwright stores browsers outside the repository. Repair a missing/cache-mismatched
REM browser before every scheduled run; the command is a no-op when it is already present.
call "%NPM_CMD%" run setup:browser >> "%WRAPPER_LOG%" 2>&1
set "RC=!ERRORLEVEL!"
if not "!RC!"=="0" goto setup_failed

REM Verify the actual headless executable, not only Playwright's installation marker.
call "%NPM_CMD%" run verify:browser >> "%WRAPPER_LOG%" 2>&1
set "RC=!ERRORLEVEL!"
if "!RC!"=="20" goto browser_repair
if not "!RC!"=="0" goto browser_failed
goto browser_ready

:browser_repair
echo [%DATE% %TIME%] wrapper:browser_verify_failed; forcing reinstall >> "%WRAPPER_LOG%"
call "%NPM_CMD%" run repair:browser >> "%WRAPPER_LOG%" 2>&1
set "RC=!ERRORLEVEL!"
if not "!RC!"=="0" goto browser_failed
call "%NPM_CMD%" run verify:browser >> "%WRAPPER_LOG%" 2>&1
set "RC=!ERRORLEVEL!"
if not "!RC!"=="0" goto browser_failed

:browser_ready
call "%NPM_CMD%" run run-once >> "%WRAPPER_LOG%" 2>&1
set "RC=!ERRORLEVEL!"
echo [%DATE% %TIME%] wrapper:run exit=!RC! >> "%WRAPPER_LOG%"
exit /b !RC!

:setup_failed
echo [%DATE% %TIME%] wrapper:browser_setup_failed exit=!RC! >> "%WRAPPER_LOG%"
echo ERROR: Playwright browser setup failed; see logs\task-wrapper.log
exit /b !RC!

:browser_failed
echo [%DATE% %TIME%] wrapper:browser_verify_failed exit=!RC! >> "%WRAPPER_LOG%"
echo ERROR: Playwright browser verification failed; see logs\task-wrapper.log
exit /b !RC!

:try_npm
if defined NPM_CMD goto :eof
if not exist "%~1" goto :eof
set "NODE_DIR="
for %%I in ("%~1") do set "NODE_DIR=%%~dpI"
set "NODE_EXE=%NODE_DIR%node.exe"
if not exist "%NODE_EXE%" set "NODE_EXE=node"
set "NODE_VERSION="
for /f "delims=" %%V in ('"%NODE_EXE%" --version 2^>nul') do if not defined NODE_VERSION set "NODE_VERSION=%%V"
if not defined NODE_VERSION goto :eof
set "NODE_MAJOR=%NODE_VERSION:v=%"
for /f "tokens=1 delims=." %%M in ("%NODE_MAJOR%") do set "NODE_MAJOR=%%M"
if not defined NODE_MAJOR goto :eof
if %NODE_MAJOR% LSS 20 goto :eof
call "%~1" --version >nul 2>&1
if errorlevel 1 goto :eof
set "NPM_CMD=%~1"
goto :eof
