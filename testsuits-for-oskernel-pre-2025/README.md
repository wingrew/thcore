# 2025年系统能力培养赛操作系统赛题目

本项目是2025年系统能力培养赛操作系统赛题目，包括basic、busybox、iozone、libc-test、lua五类。

## 构建方法

0. 你的电脑上需要安装[docker](https://docs.docker.com/engine/install/)。
1. 运行`make docker`进入docker环境。
2. 运行`make`构建评测样例和SD卡镜像文件。

## syscalls测试用例Qemu运行环境
[syscalls测试用例 for Linux on Qemu RV64运行环境](riscv-linux-rootfs) ： 在Linux上运行测试用例主要是用于对比自己实现的OS


