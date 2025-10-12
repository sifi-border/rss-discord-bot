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

<s>Summaryと言いつつhtmlなのでびっくりだ。

一旦Summaryは投稿内容に含めないことにする。</s>

 RSS自体Rich Site Summaryの略なので、Summaryが本体だったっぽい。エアプでした。

ここでRSSに[blueskyへのリンク](https://bsky.app/profile/thisweekinrust.bsky.social)が内容に含まれていることに気づく。

ユーザー目線だとSNSで事足りるよね現代はね。

一旦moduleに切り分ける

### 16:15- Summaryを短縮

htmlタグを除去して140字程度に切り詰める。

```
Feed Title: This Week in Rust
Number of Entries: 4
Title: This Week in Rust 619
Link: https://this-week-in-rust.org/blog/2025/10/01/this-week-in-rust-619/
Summary: Hello and welcome to another issue of This Week in Rust!
Rust is a programming language empowering everyone to build reliable and efficient ...
---
```

情報量が増えておらず、涙。

## 2025/10/11 Sat

### 22:00-

EmbedPostを定義。

discord.rsに置いてるが、将来的にリファクタして移したさはある。

適当にEmbedPost用の構造体を書きPOSTした。→400 Bad Requestが返ってきた。

これはカラーコードを写経したら桁が多くなっていたのが原因。

この点を修正するときちんとEmbedでpostされていた。

### 23:00-

カラーコード用のEnumを定義。良い感じに渡せるようになった。

```rust
DiscordEmbedPost::new(title, url, summary, SourceCategory::Rust, Utc::now());
```

ところでtimestampいる？Discordだとポスト自体の投稿時間が表示されるのでいらない気もする。

### 23:30-

重複判定は一旦ファイルに書き出す形にする。

以下のようなjsonでfeed毎に管理。

```json
{
  "feeds": {
    "https://this-week-in-rust.org/rss.xml": {
      "latest_title": "This Week in Rust 620",
      "latest_article_url": "https://...",
      "last_updated": "2025-10-11T14:00:00Z"
    }
  }
}
```

また、post成功時のみ書き出すようにする。

将来DBを導入するかもなのでRepository Patternで実装する。

mainでfeed.entryの情報をこねくり回している（どこかに移したい）が、&strで扱うようrefactorした。

正常系の動作確認done(jsonの出力とDiscordへのpostを確認)。

再度runしたところ、期待通り重複を確認し何もpostしなかった。

## 2025/10/12 Sun

### 19:15-

github actionsで諸々できるようにyamlを書く

とりあえず[ドキュメント](https://docs.github.com/ja/actions)に目を通す。

> リポジトリで、pull request のオープンや issue の作成などの イベント が発生したときにトリガーされるように GitHub Actions ワークフロー を構成できます。
ワークフローには、1 つ以上の ジョブ が含まれており、ジョブは順次にまたは並列で実行できます。
各ジョブは、専用の仮想マシン ランナー 内、またはコンテナー内で実行され、定義した スクリプト を実行するか、
または アクション (ワークフローを簡略化できる再利用可能な拡張機能) を実行する 1 つ以上のステップで構成されます。

ということらしい。

### 20:30-

原型はclaudeに作ってもらい、古いActionを用いている部分を修正。
