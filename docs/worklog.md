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

