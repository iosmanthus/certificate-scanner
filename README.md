# Certificate Scanner

### Introduction

本项目旨在利用 [OpenData](https://opendata.rapid7.com/sonar.ssl/) 爬取`IPV4`的`RSA`证书:

1. 提取出其公钥模数`N`, 存放在`MySQL`中
2. 使用`batch-gcd`找他们的公因子，试图分解`N`, 参考https://facthacks.cr.yp.to/batchgcd.html
3. 通过过滤/查询, 可以知道哪个网站与哪个网站具有相同的公因子, 从而破解其`RSA`证书



* 如果我们分解了一个政府网站的模数，可能我们(我)就发达了.

* 其次，如果我们一个模数都分解不了，我们就发一篇博客盛赞我国网站证书做得好，给他们发表扬信 23333

### Details

```
.
├── analysis // 分析爬取的证书
└── data     // 从OpenData爬取的RSA证书
```

1. 首先, 我们需要一个高性能的`batch-gcd`, 感谢`Rust`, 感谢[`rayon`](https://github.com/rayon-rs/rayon), 让我们有了并发迭代器还有极为方便的`join`, [我写了一个性能还不错的(自夸)](https://github.com/iosmanthus/batch-gcd) .
2. 然后就是:
   * 登录数据库
   * 将一定量的模数放入`Vec`
   * 放入`batch_gcd`扫描
   * 输出结果...

### 致谢

感谢上学期[斌头老师](https://github.com/lbwang)的耐心教导 :)