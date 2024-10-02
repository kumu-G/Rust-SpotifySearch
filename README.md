# Rust-spotify-search-yt

## Explain this code

这段代码使用了reqwest和serde库，实现了一个使用Spotify API进行音乐搜索的功能。代码的主要步骤如下：

1. 引入所需的库和模块。
2. 定义了一些结构体，用于存储从API响应中提取的数据。
3. 定义了一个打印音乐列表的函数print_tracks。
4. 在main函数中，获取命令行参数，检查参数数量是否正确。
5. 根据命令行参数构建API请求的URL。
6. 创建一个reqwest的Client对象，发送GET请求，并添加必要的请求头和参数。
7. 根据响应的状态码进行处理：
    - 如果状态码是200（OK），则解析响应的JSON数据为APIResponse结构体，并调用print_tracks函数打印音乐列表。
    - 如果状态码是401（UNAUTHORIZED），则提示需要获取新的访问令牌。
    - 其他状态码则打印出错信息。

总之，这段代码实现了一个使用Spotify API进行音乐搜索的命令行工具，可以根据用户提供的查询词和授权令牌，获取匹配的音乐列表并打印出来。

## Usage

```shell
cargo run "search query" "spotify auth_token"

# Example:
➜ cargo run "china love" "BQA5NWUAlnacrk44L1aHT-2wTevZmY"
```

## 更多参考

- spotify API文档 [Search for Item](https://developer.spotify.com/documentation/web-api/reference/search "Search for Item")。

- spotify [access token](https://developer.spotify.com/documentation/web-api/tutorials/getting-started#request-an-access-token "获取spotify token文档")。
