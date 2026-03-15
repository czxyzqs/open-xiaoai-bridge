@echo off
chcp 65001 >nul

REM Open-XiaoAI Bridge 启动脚本 (Windows)
REM 用法: scripts\start.bat

REM cd to project root (parent of scripts\)
cd /d "%~dp0\.."

echo ========================================
echo   Open-XiaoAI Bridge 启动脚本
echo ========================================
echo.

set "XIAOZHI_ENABLED=%XIAOZHI_ENABLE%"

REM 1. 检查 uv
uv --version >nul 2>&1
if errorlevel 1 (
    echo [错误] 未找到 uv 命令
    echo 请先安装 uv:
    echo   powershell -c "irm https://astral.sh/uv/install.ps1 | iex"
    pause
    exit /b 1
)
echo [OK] uv 已安装

REM 2. 检查 KWS 相关模型和关键词文件
if /I "%XIAOZHI_ENABLED%"=="1" goto :run_kws_setup
if /I "%XIAOZHI_ENABLED%"=="true" goto :run_kws_setup
if /I "%XIAOZHI_ENABLED%"=="yes" goto :run_kws_setup

echo [提示] 小智未启用，跳过模型检查和关键词预生成
goto :after_kws_setup

:run_kws_setup
set "MODEL_DIR=core\models"
set "MISSING=0"

if not exist "%MODEL_DIR%\silero_vad.onnx" set "MISSING=1"
if not exist "%MODEL_DIR%\tokens.txt" set "MISSING=1"
if not exist "%MODEL_DIR%\bpe.model" set "MISSING=1"

if "%MISSING%"=="0" (
    echo [OK] 模型文件已存在
) else (
    echo [提示] 缺少模型文件，正在自动下载...

    if not exist "%MODEL_DIR%" mkdir "%MODEL_DIR%"

    set "ZIP_FILE=%MODEL_DIR%\models.zip"
    set "MODEL_URL=https://github.com/coderzc/open-xiaoai/releases/download/vad-kws-models/models.zip"

    echo [提示] 正在下载模型文件...
    powershell -Command "Invoke-WebRequest -Uri '%MODEL_URL%' -OutFile '%ZIP_FILE%'" >nul 2>&1
    if errorlevel 1 (
        echo [错误] 下载模型文件失败
        pause
        exit /b 1
    )

    echo [提示] 正在解压模型文件...
    powershell -Command "Expand-Archive -Path '%ZIP_FILE%' -DestinationPath '%MODEL_DIR%' -Force" >nul 2>&1
    if errorlevel 1 (
        echo [错误] 解压模型文件失败
        pause
        exit /b 1
    )

    REM 如果解压后有多一层 models 目录，移动文件到正确位置
    if exist "%MODEL_DIR%\models" (
        echo [提示] 整理模型文件...
        xcopy /E /I /Y "%MODEL_DIR%\models\*" "%MODEL_DIR%\" >nul 2>&1
        rmdir /S /Q "%MODEL_DIR%\models"
    )

    del "%ZIP_FILE%"
    echo [OK] 模型文件下载并解压完成
)

echo.
echo [提示] 生成关键词文件...
python core\services\audio\kws\keywords.py >nul 2>&1
if errorlevel 1 (
    echo [错误] 关键词文件生成失败
    pause
    exit /b 1
) else (
    echo [OK] 关键词文件生成完成
)

:after_kws_setup

REM 3. 启动
echo.
echo ========================================
echo   启动 Open-XiaoAI Bridge...
echo ========================================
echo.

uv run python main.py %*

pause
