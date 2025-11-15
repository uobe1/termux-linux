#!/bin/bash

# 测试解压修复的脚本

echo "测试解压修复..."
echo "===================="

# 创建测试目录
TEST_DIR="/tmp/test_extraction"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"

cd "$TEST_DIR"

# 创建测试文件结构
echo "创建测试文件结构..."
mkdir -p test_rootfs/usr/bin
echo "perl5.28.1 content" > test_rootfs/usr/bin/perl5.28.1

# 创建硬链接（这在Termux中会导致权限问题）
ln test_rootfs/usr/bin/perl5.28.1 test_rootfs/usr/bin/perl 2>/dev/null || echo "创建硬链接失败（预期在Termux中）"

# 创建tar包
echo "创建测试tar包..."
tar -czf test_rootfs.tar.gz test_rootfs/

# 尝试使用旧的解压方法（应该失败）
echo ""
echo "测试旧的解压方法（应该失败）:"
echo "tar -xzf test_rootfs.tar.gz -C extracted_old/"
mkdir -p extracted_old
if tar -xzf test_rootfs.tar.gz -C extracted_old/ 2>&1 | grep -q "Cannot hard link"; then
    echo "❌ 旧的解压方法失败（出现硬链接错误）"
else
    echo "✓ 旧的解压方法成功"
fi

# 尝试使用新的解压方法（应该成功）
echo ""
echo "测试新的解压方法（使用proot --link2symlink）:"
echo "proot --link2symlink tar -C extracted_new/ -xzf test_rootfs.tar.gz"
mkdir -p extracted_new
if proot --link2symlink tar -C extracted_new/ -xzf test_rootfs.tar.gz 2>&1; then
    echo "✓ 新的解压方法成功"
    
    # 检查文件是否正确解压
    if [ -f "extracted_new/test_rootfs/usr/bin/perl5.28.1" ]; then
        echo "✓ perl5.28.1 文件存在"
    else
        echo "❌ perl5.28.1 文件不存在"
    fi
    
    if [ -f "extracted_new/test_rootfs/usr/bin/perl" ]; then
        echo "✓ perl 链接存在"
        echo "  文件类型: $(file extracted_new/test_rootfs/usr/bin/perl)"
    else
        echo "❌ perl 链接不存在"
    fi
else
    echo "❌ 新的解压方法失败"
fi

# 清理
cd /
rm -rf "$TEST_DIR"

echo ""
echo "测试完成！"
