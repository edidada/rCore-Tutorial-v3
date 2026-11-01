# ch7-lab：文件元数据与硬链接

该旧分支 test_file0..2 要求实现 fstat、link/unlink。easy-fs 目录项复用同一磁盘 inode，inode 标识由磁盘位置稳定生成，链接数通过根目录引用数统计。unlink 用末尾目录项填洞并缩短目录大小，保留 inode 与数据块以支持打开文件继续访问；原题允许不回收最后链接的 inode/data，本实现明确采用这一策略。

fstat 输出 repr(C) 对应的 80 字节布局，填充全部置零，逐页验证写权限后写回。linkat 通过六寄存器 ABI 接收新旧路径。路径检查拒绝非法地址及过长名称。Actions 创建真实 easy-fs 镜像，经 VirtIO 磁盘启动，通过 shell 验证文件读写、元数据、四个硬链接、删除原名称后数据与 inode 相同及链接计数递减。本地不编译。
