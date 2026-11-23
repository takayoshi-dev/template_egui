# template_egui

ZedのDevContainer機能で環境構築済みのegui最小テンプレート

## 概要

[egui](https://github.com/emilk/egui) を使ったGUIアプリを、すぐに開発し始められるようにした最小限のテンプレートです。\
"Hello World!!!" だけを表示します。\
ZedのDevContainer機能を用いており、ビルドはコンテナ内で行います。\
※Debian系Linuxでの開発を想定しているため、WindowsやmacOSには対応しておりません。

## 動作環境
- Debian系Linux

## プロジェクト構成

| 項目 | 内容 |
| ---- | ---- |
| OS | Ubuntu 26.04.1 LTS |
| エディタ | Zed |
| 使用言語 | Rust |
| GUIフレームワーク | egui (eframe) v0.37 |
| 仮想化 | Docker |
| ベースイメージ | rust:1.100.0-trixie |

## セットアップ

1. リポジトリを取得する

   ```bash
   git clone https://github.com/takayoshi-dev/template_egui.git
   cd template_egui
   ```

2. コンテナのUID/GIDをホスト側に合わせる

   ホスト側で次のコマンドを実行し、UIDとGIDを確認します。

   ```bash
   id -u
   id -g
   ```

   確認した値を `devcontainer.json` の `DEV_UID` と `DEV_GID` に書き込みます。

3. Dev Containerを起動する

   Zedで「Open Remote Folder」を選択し、「Connect Dev Container」を押します。

4. Zedの制限モードを解除する

   Zedの左上に黄色い文字で「Restricted Mode」と表示されます。これを押し、確認メッセージで「Trust and Continue」を選択すると解除できます。

以上でセットアップは完了です。

## 使い方

| 操作 | コマンド | 備考 |
| ---- | -------- | ---- |
| ビルド | `cargo build` | `target/debug/` に出力 |
| ビルド(リリース) | `cargo build --release` | `target/release/` に出力 |
| 単体テスト | `cargo test` | |
| ドキュメント生成 | `cargo doc --no-deps` | |

### 動作確認(ホストPC側の操作)

```bash
./target/debug/template_egui
./target/release/template_egui
```

- バイナリ名は `Cargo.toml` の `name` で変更できます(デフォルトは `template_egui`)。
- 念のため `ls -l` で、ユーザーに実行権があることを確認してください。

## ライセンス

[MIT License](./LICENSE)

## 更新履歴

詳しくは [CHANGELOG.md](./CHANGELOG.md) をご覧ください。
