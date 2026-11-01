# 汇总 lab 分支验证

此分支基于仓库 origin/lab 的完整内核版本，区别于旧版 ch2-lab 至 ch7-lab 实验框架；不将旧内存/进程框架覆盖到新线程、同步和设备驱动实现中。各章节的实验代码与报告保留于对应 edidada/chN-lab 分支。

新增独立 lab.yml，使用现有 stable 工具链构建所有应用、easy-fs 镜像及内核，QEMU 配置块设备、GPU、键盘、鼠标、网络设备。执行原版完整 usertests，包括文件、fork、管道、线程、互斥锁、信号量及预期失败程序。验证成功/预期失败数量与测试清单一致、Usertests passed 和 PID 0 正常关机；超时、内核 panic、非零 QEMU 返回码均失败。

原文档发布工作流限制到 main/chN，避免该实验分支触发额外 Pages 发布。所有构建与 QEMU 验证均由 Actions 完成，本地不编译。编译版本与依赖锁文件随日志/产物保存，不将排队或运行中当作通过。

## 章节实验索引

各分支独立保存代码、workflow 和 reports/labN.md；旧章节与汇总分支使用不同代的内核接口。

| 分支 | 实现 | 已验证提交 | Actions |
|---|---|---|---|
| edidada/ch2-lab | sys_write 地址/栈边界与异常隔离 | cec7a88c | https://github.com/edidada/rCore-Tutorial-v3/actions/runs/41290571202 |
| edidada/ch3-lab | task_info、调用计数和 Stride | ac610e3d | https://github.com/edidada/rCore-Tutorial-v3/actions/runs/41291049929 |
| edidada/ch4-lab | mmap/munmap、跨页用户缓冲区 | 1b95433d | https://github.com/edidada/rCore-Tutorial-v3/actions/runs/41292526918 |
| edidada/ch5-lab | spawn、父子关系与退出码回收 | 337cf7c5 | https://github.com/edidada/rCore-Tutorial-v3/actions/runs/41292755466 |
| edidada/ch6-lab | 有界邮箱 IPC、父子通信 | 919496a6 | https://github.com/edidada/rCore-Tutorial-v3/actions/runs/41292969844 |
| edidada/ch7-lab | fstat 与持久目录硬链接 | 82bc685f | https://github.com/edidada/rCore-Tutorial-v3/actions/runs/41313427358 |

源码契约以对应 origin/chN-lab 的原始测例为基础；现行教程已调整章节分配，不能把旧 ch6 邮箱实验与新版 ch6 文件系统实验名称直接等同。官方教程入口：https://rcore-os.cn/rCore-Tutorial-Book-v3/ 。
