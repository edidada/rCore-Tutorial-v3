# ch4-lab：用户地址空间

依据 https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter4/8exercise.html 及分支原有 test2_mmap/test2_unmap 测例，实现 mmap/munmap。映射前检查页对齐、权限掩码、地址溢出与全部页重叠；解除映射前检查全部页，失败不产生部分解除。帧由 MapArea 持有并回收，允许按页解除子区间。修正末级无效 PTE 被 translate 当作有效的问题。

继承 sys_write 安全检查，改为逐页检查 U/R/W，全部合法后才输出；get_time 通过跨页物理缓冲区写回，不直接解引用用户虚拟地址。QEMU virt 时钟设为 10MHz。

Actions 运行原有基础、sleep、mmap/munmap 测例；只读写入、非法 W-only 页和解除映射后的访问应终止当前任务，其他应用继续。本地不编译。当前实现为立即分配页，lazy/COW/swap 属于课后扩展，未将其声称为本分支实验交付。
