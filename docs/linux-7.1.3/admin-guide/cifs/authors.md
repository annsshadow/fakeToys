## 作

### 原始作

Steve French (smfrench@gmail.com, sfrench@samba.org)

作者希望表达他的感激与谢意：感谢 Andrew Tridgell（Samba 团队）关SMB/CIFS VFS 改进的早期建议。感IBM 给予我时间与测试资源来推进此项目，感IBM Jim McDonough（以Samba 团队）的帮助，感IBM Linux JFS 团队对许多深Linux 文件系统特性的讲解。Samba 团队Jeremy Allison 在完成原CIFS Unix 扩展的服务端以及将较新的 CIFS POSIX 扩展的部分内容审查并实现Samba 3 文件服务器中做出了宝贵工作。感IBM Rochester Dave Boutcher（OS/400 smb/cifs 文件系统客户端的作者）多年前的证明：在Unix 操作系统上可以实现非常好smb/cifs 客户端。Volker Lendecke、Andrew Tridgell、Urban Widmark、John Newbigin 及其他人感谢他们Linux smbfs 模块上的工作。感谢存储网络工业协会（SNIA）CIFS 技术工作组其他成员在规范这一高度复杂协议上的工作，最后感Samba 团队的技术建议与鼓励
### 补丁贡献

- Zwane Mwaikambo
- Andi Kleen
- Amrut Joshi
- Shobhit Dayal
- Sergey Vlasov
- Richard Hughes
- Yury Umanets
- Mark Hamzy（部分早cifs IPv6 工作- Domen Puncer
- Jesper Juhl（尤其贡献了大量空白/格式清理- Vince Negri Dave Stahl（发现了重要的缓bug- Adrian Bunk（kcalloc 清理- Miklos Szeredi
- Kazeon 团队，各种修复，尤其2.4 版本- Asser Ferno（Change Notify 支持- Shaggy（Dave Kleikamp），无数小的文件系统建议和一些良好的清理
- Gunter Kukkukk（针对老旧服务器支持的测试与建议）
- Igor Mammedov（DFS 支持- Jeff Layton（许多、许多修复，以及 cifs Kerberos 代码的出色工作）
- Scott Lovenberg
- Pavel Shilovsky（在添加 SMB2 支持以及各种 SMB3 特性上的出色工作）
- Aurelien Aptel（DFS SMB3 工作以及一些关bug 修复- Ronnie Sahlberg（SMB3 xattr 工作、bug 修复以及大量在复合（compounding）上的出色工作）
- Shirish Pargaonkar（多年来许多 ACL 补丁- Sachin Prabhu（许bug 修复，包括重连、复制卸载和安全性）
- Paulo Alcantara（在 DFS 以及SMB3 启动上的出色工作- Long Li（在 RDMA、SMB Direct 上的一些出色工作）


### 测试用例与缺陷报告贡献

感谢社区中提交详细缺陷报告并调试所发现问题的人：Jochen Dolze、David Blaine、Rene Scharfe、Martin Josefsson、Alexander Wild、Anthony Liguori、Lars Muller、Urban Widmark、Massimiliano Ferrero、Howard Owen、Olaf Kirch、Kieron Briggs、Nick Millington 等。也特别提及 Stanford Checker（SWAT），它指出了错误路径中的许多bug。Al Viro Dave Miller 也给出了宝贵的建议
并感IBM LTC Power 测试团队，以SuSE、Citrix RedHat 的测试人员在优秀的压力测试运行中发现了多bug