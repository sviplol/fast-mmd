#!/bin/bash
# Mac "应用已损坏" 修复脚本
# 用法: 双击此文件运行

echo "========================================"
echo "  修复 Fast MMD 已损坏问题"
echo "========================================"

# 获取当前脚本所在目录
DIR="$(cd "$(dirname "$0")" && pwd)"

echo ""
echo "正在修复..."
echo ""

# 方法1: xattr 清除隔离属性 - 修复当前目录下所有 .app 和 .dmg
find "$DIR" -name "*.app" -exec xattr -cr {} \; 2>/dev/null
find "$DIR" -name "*.dmg" -exec xattr -cr {} \; 2>/dev/null

# 方法2: 全局允许打开(如方法1不够)
sudo xattr -d com.apple.quarantine "$DIR"/*.app 2>/dev/null
sudo xattr -d com.apple.quarantine "$DIR"/*.dmg 2>/dev/null

# 方法3: spctl 全局放行(如需要)
sudo spctl --master-disable 2>/dev/null

echo ""
echo "✅ 修复完成！"
echo ""
echo "现在可以双击 .dmg 或 .app 文件正常安装了"
echo "如果仍有问题，请在终端执行:"
echo "  sudo xattr -rd com.apple.quarantine /Applications/Fast\\ MMD.app"
echo ""
echo "按任意键关闭..."
read -n 1
