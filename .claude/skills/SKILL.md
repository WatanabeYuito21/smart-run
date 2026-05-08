---
name: smart-run
description: |
  smart-run（Rustで作るfrecencyベースのコマンドヒストリーCLIツール）の
  開発を支援するスキル。
  「smart-runを実装して」「ビルドエラーを直して」「シェル統合を追加して」
  「テストを書いて」など、このプロジェクトに関する作業が来たら必ずこのスキルを使うこと。
---

# smart-run 開発スキル

## 作業開始時に必ずやること

1. `.claude/CLAUDE.md` を読んで設計方針・実装ステータスを把握する
2. `cargo build` でビルドが通るか確認する
3. ユーザーに「どこから始めるか」を確認する（指示がなければ実装ステータスの上から順）

---

## Git ルール

- コミットメッセージに `Co-Authored-By: Claude` 行を**含めない**
- コミットメッセージは**日本語**で書く
- 先頭に以下のプレフィックスをつける

| プレフィックス | 用途                                               |
| -------------- | -------------------------------------------------- |
| fix            | 既存機能のバグ修正                                 |
| hotfix         | 緊急の変更                                         |
| add            | 新規ファイル・機能の追加                           |
| feat           | 新機能・新ファイルの追加                           |
| update         | 既存機能の問題なし修正                             |
| change         | 仕様変更による既存機能の修正                       |
| clean/refactor | コードの整理・改善                                 |
| improve        | コードの改善                                       |
| disable        | 機能の一時無効化                                   |
| remove/delete  | ファイル・機能の削除                               |
| rename         | ファイル名変更                                     |
| move           | ファイル移動                                       |
| upgrade        | バージョンアップグレード                           |
| revert         | 以前のコミットへの差し戻し                         |
| docs           | ドキュメント修正                                   |
| style          | コーディングスタイル修正                           |
| perf           | パフォーマンス改善                                 |
| test           | テストコードの追加・修正                           |
| chore          | ビルドツール・自動生成物、上記に当てはまらない修正 |

---

## 重要ルール

### stderr / stdout の使い分け

`query` コマンドはシェルの `$()` で呼ばれる。混同すると統合が壊れる。

- **stdout** → 選択されたコマンド文字列（1行のみ）
- **stderr** → TUI描画・エラー・進捗

### クロスプラットフォーム注意点

| 項目     | Windows                       | Linux/WSL                          |
| -------- | ----------------------------- | ---------------------------------- |
| DBパス   | `%APPDATA%\smart-run\db.json` | `~/.local/share/smart-run/db.json` |
| 環境変数 | `APPDATA`                     | `HOME`                             |

パス操作は `std::path::PathBuf` を使えばほぼ吸収できる。

### スコア計算

```
score = run_count × time_decay × context_boost
time_decay = 1.0 / (1.0 + elapsed_hours × 0.01)
context_boost = 1.5  // last_dir == current_dir の場合
              = 1.0  // それ以外
```

---

## よく使うコマンド

```bash
cargo build                           # ビルド
cargo check                           # 型チェックのみ（速い）
cargo test                            # テスト実行

# 手動動作確認
./target/debug/smart-run add "git status"
./target/debug/smart-run list
./target/debug/smart-run query git    # TUI起動（ターミナルで実行すること）
./target/debug/smart-run init bash
```

---

## ファイル別の役割

| ファイル         | 役割                              |
| ---------------- | --------------------------------- |
| `src/main.rs`    | CLIコマンドのルーティング（clap） |
| `src/db.rs`      | JSON読み書き・frecencyスコア計算  |
| `src/matcher.rs` | キーワードマッチング              |
| `src/ui.rs`      | TUI描画・キー入力（crossterm）    |
| `src/shell.rs`   | シェル統合スクリプト文字列        |

---

## smart-cdとの実装上の差分ポイント

- `Entry` の主キーは `path` ではなく `command`（文字列）
- `last_dir` フィールドを追加し、スコアにコンテキストブーストを掛ける
- シェルフックは `cd` ではなくコマンドをバッファに展開する（bash なら `READLINE_LINE`）
- `add` サブコマンドは `$PWD` も受け取るか、環境変数 `PWD` から取得する
