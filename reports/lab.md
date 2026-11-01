# 汇总 lab 分支验证

此分支基于仓库 origin/lab 的完整内核版本，区别于旧版 ch2-lab 至 ch7-lab 实验框架；不将旧内存/进程框架覆盖到新线程、同步和设备驱动实现中。各章节的实验代码与报告保留于对应 edidada/chN-lab 分支。

新增独立 lab.yml，使用现有 stable 工具链构建所有应用、easy-fs 镜像及内核，QEMU 配置块设备、GPU、键盘、鼠标、网络设备。执行原版完整 usertests，包括文件、fork、管道、线程、互斥锁、信号量及预期失败程序。验证成功/预期失败数量与测试清单一致、Usertests passed 和 PID 0 正常关机；超时、内核 panic、非零 QEMU 返回码均失败。

原文档发布工作流限制到 main/chN，避免该实验分支触发额外 Pages 发布。所有构建与 QEMU 验证均由 Actions 完成，本地不编译。编译版本与依赖锁文件随日志/产物保存，不将排队或运行中当作通过。
