@echo off
echo 🚀 启动 Cloudflare 管理平台...

REM 检查 Docker 是否安装
docker --version >nul 2>&1
if errorlevel 1 (
    echo ❌ 错误: Docker 未安装
    echo 请访问 https://docs.docker.com/get-docker/ 安装 Docker
    exit /b 1
)

REM 检查 Docker Compose 是否安装
docker-compose --version >nul 2>&1
if errorlevel 1 (
    echo ❌ 错误: Docker Compose 未安装
    echo 请访问 https://docs.docker.com/compose/install/ 安装 Docker Compose
    exit /b 1
)

REM 停止现有容器
echo 📦 停止现有容器...
docker-compose down

REM 构建并启动服务
echo 🔨 构建并启动服务...
docker-compose up -d --build

REM 等待服务启动
echo ⏳ 等待服务启动...
timeout /t 5 /nobreak >nul

REM 检查服务状态
echo ✅ 检查服务状态...
docker-compose ps

echo.
echo ✨ 部署完成!
echo 🌐 访问地址: http://localhost:3000
echo.
echo 📝 常用命令:
echo   查看日志: docker-compose logs -f
echo   停止服务: docker-compose down
echo   重启服务: docker-compose restart
echo.
pause
