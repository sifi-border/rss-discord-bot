# WORKLOG

## 2025/10/04 Sat

### 14:00- 初期setup

`cargo new rss-discord-bot`

README作成

memo: 作業ログを直接docとしてリポジトリに含めることにする。

Cargo.tomlに依存関係追加

git ignore 編集

Discordで Webhook URL取得し、.envに保存

`git commit -m "Initial commit: Project setup with README and dependencies"`

リモートリポジトリを作成

### 15:00- Discord へのPOST

main.rsを編集

```rust
    let client = Client::new();
    let payload = serde_json::json!({
        "content": "🦀 Hello from Rust!",
    });

    let response = client
        .post(&discord_webhook_url)
        .json(&payload)
        .send()
        .await?;

    println!("Response: {:?}", response);
```

204が返ってきて、discordのチャンネルに投稿されていた。

わざわざbotを作らなくても、投稿だけならこんなにシンプルなのんな〜

上記の処理をmainから関数に切り出した。

### 15:30- rss取得

RSSの取得を試みる

feedのparser crateがあるのでありがたく拝借するあるのでありがたく用いる。

```
Feed Title: This Week in Rust
Number of Entries: 4
Title: This Week in Rust 619
Link: https://this-week-in-rust.org/blog/2025/10/01/this-week-in-rust-619/
Summary: <p>Hello and welcome to another issue of <em>This Week in Rust</em>!
<a href="https://www.rust-lang.org/">Rust</a> is a programming langu...
---
Title: This Week in Rust 618
Link: https://this-week-in-rust.org/blog/2025/09/24/this-week-in-rust-618/
Summary: <p>Hello and welcome to another issue of <em>This Week in Rust</em>!
<a href="https://www.rust-lang.org/">Rust</a> is a programming langu...
---
Title: This Week in Rust 617
Link: https://this-week-in-rust.org/blog/2025/09/17/this-week-in-rust-617/
Summary: <p>Hello and welcome to another issue of <em>This Week in Rust</em>!
<a href="https://www.rust-lang.org/">Rust</a> is a programming langu...
---
```

Summaryと言いつつhtmlなのでびっくりだ。

一旦Summaryは投稿内容に含めないことにする。