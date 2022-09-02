# Off-Chain Worker 教程

- Substrate 密码学

- 介绍及代码前期衔接

  - Send Signed Transaction
  - Send Unsigned Transaction
  - Unsigned transaction with signed payload

- 发送HTTP请求及JSON解析

- Off-chain worker 存储数据库

- FRAME 权益证明相关模块介绍

  - im-online
  - staking
  - session

- 作业

  编程作业，需要完成以下要求并提交代码链接：

  以lecture-demo 作为基础，把它拷到assignment目录里来修改，最后提交这个代码库。

  利用offchain worker取出DOT当前对USD的价格，并把它写到一个Vec的存储里，你们自己选一种方法提交回链上，并在代码注释为什么用这种方法提交回链上最好。只保留当前最近的10个价格，其他价格可以丢弃（就是Vec的长度到10后，这时再插入一个值，要先丢弃最早的那个值）。

  这个http请求可得到当前DOT价格： https://api.coincap.io/v2/assets/polkadot

作业时间：1.25

# 链下工作机

