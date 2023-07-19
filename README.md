rust 练手工具

# Example

`mkdir example.toml` 创建一个 toml 文件

```toml
[url]
# 请求接口
localhost = ""

# GET POST DELETE PUT PATCH
method = "GET"

[body]
# 请求体设置

[params]
# x-www-form-urlencoded, get params 数据可填


[header]
# 自定义请求头设置
```

`net_util -p [PATH]` 填入相对路径即可测试

# Header 请求头设置

在当前活跃文件夹下创建 `header.toml` 文件作为所有测试接口的默认请求头配置

```toml
content-type = "application/x-www-form-urlencoded"
```

配置将会在请求时混入到请求头中
**测试文件中的 header key 权重大于 header. toml 文件中的 key**
