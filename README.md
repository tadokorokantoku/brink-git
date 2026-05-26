# brink-git

Git の **現在の branch** に、issue / esa / Figma など任意のキーで URL や文字列を紐づける CLI です。  
データは **`{git-common-dir}/brink/data.json`** に保存され、リポジトリにはコミットされません。

## インストール

### Cargo（推奨・開発時）

```bash
cargo install --path .
# brink が PATH に入る
```

### npm

```bash
# リポジトリから（先に Rust バイナリをビルド）
cargo build --release
npm i -g ./npm

# またはバイナリを npm パッケージに同梱してから
cd npm && npm run build:binaries && npm i -g .
```

npm パッケージ名は **`brink-git`**、コマンド名は **`brink`** です。

## 使い方

```bash
brink set esa https://esa.io/posts/123
brink set issue https://github.com/org/repo/issues/42

brink get esa
brink has esa    # 存在すれば exit 0（出力なし）
brink list
brink list --json
brink doc              # Markdown docs (AI-friendly)
brink doc set          # docs for one command
```

### 例: `list` 出力

```text
branch: feature/add-login
esa     https://esa.io/posts/123
issue   https://github.com/org/repo/issues/42
```

### 例: `list --json`

```json
{
  "branch": "feature/add-login",
  "entries": {
    "esa": "https://esa.io/posts/123",
    "issue": "https://github.com/org/repo/issues/42"
  }
}
```

## AI / エージェント向けドキュメント

`brink doc` は Hono CLI の `hono docs` と同様、**Markdown を stdout に出す**コマンドです（Git 不要）。

```bash
brink doc              # 索引 + クイックリファレンス
brink doc overview     # 保存場所・制約
brink doc set          # 各サブコマンドの詳細
```

## 仕様メモ

| 状況 | 挙動 |
|------|------|
| Git 管理外 | エラー |
| detached HEAD | エラー |
| 未設定の `get` | exit 1 + stderr にヒント |
| 同じ key の `set` | 上書き |

## 開発

```bash
cargo test
cargo build --release
./target/release/brink list
```

## npm 公開

詳細は [PUBLISHING.md](./PUBLISHING.md) を参照してください。

- タグ `v*` を push → GitHub Actions がビルド・npm publish
- パッケージ名: **`brink-git`**（コマンドは `brink`）

## ライセンス

MIT
