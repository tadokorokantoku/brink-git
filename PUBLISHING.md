# npm 公開手順

`brink-git` はメインパッケージ、ネイティブバイナリはプラットフォーム別パッケージ（`brink-git-darwin-arm64` など）として **optionalDependencies** で配布します。

## 初回セットアップ

1. [npm](https://www.npmjs.com/) アカウントを作成
2. ローカルで `npm login`
3. GitHub リポジトリの **Secrets** に `NPM_TOKEN` を追加（Automation または Publish トークン）
4. `npm/package.json` の `repository.url` を実際のリポジトリ URL に更新

## 自動リリース（推奨）

タグを push すると `.github/workflows/release.yml` が実行されます。

```bash
# Cargo.toml の version を更新してから
git tag v0.1.0
git push origin v0.1.0
```

Workflow の内容:

1. 5 プラットフォーム向けに Rust バイナリをビルド
2. プラットフォームパッケージを npm に publish
3. `brink-git` を npm に publish
4. GitHub Release に zip を添付

## 手動リリース（ローカル）

全プラットフォームのビルドは OS ごとに制約があります。通常は CI に任せ、ローカルでは dry-run のみ推奨です。

```bash
cd npm
npm run sync-version          # Cargo.toml の version を npm に反映
npm run build:platform -- darwin-arm64   # 今の OS 向けのみビルド可能
npm run publish:dry-run       # ビルド済み platform のみ dry-run（要 npm login）
```

本番公開（**全 platform のバイナリが platforms/ に揃っていること**）:

```bash
cd npm
npm run publish:all
```

## インストール（ユーザー向け）

```bash
npm i -g brink-git
brink doc
```

## パッケージ一覧

| パッケージ | 内容 |
|-----------|------|
| `brink-git` | Node ラッパー + `brink` コマンド |
| `brink-git-darwin-arm64` | macOS Apple Silicon バイナリ |
| `brink-git-darwin-x64` | macOS Intel バイナリ |
| `brink-git-linux-x64` | Linux x64 バイナリ |
| `brink-git-linux-arm64` | Linux arm64 バイナリ |
| `brink-git-win32-x64` | Windows x64 バイナリ |

## トラブルシュート

- **postinstall で警告**: 現在の OS 用 optional パッケージが入っていない。`npm i -g brink-git` を再実行するか、`cargo install --path .` で `BRINK_BIN` を設定
- **publish 失敗 (403)**: `NPM_TOKEN` の権限、またはパッケージ名の占有を確認
- **linux-arm64 ビルド失敗**: CI では `gcc-aarch64-linux-gnu` を入れています。ローカルでも同様のリンカが必要
