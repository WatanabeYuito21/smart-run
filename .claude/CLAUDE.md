# smart-run — プロジェクト引き継ぎ

## 概要

frecency（頻度×最近さ）ベースのスマートコマンドヒストリーツール。
シェルの `history` をそのまま使う代わりに、よく使う・最近使ったコマンドをTUIで素早く検索・再実行できる。

**ターゲット環境:** Windows / WSL / Linux（全て同一バイナリ）

---

## 設計方針（決定済み）

| 項目         | 決定内容                                            |
| ------------ | --------------------------------------------------- |
| 言語         | Rust                                                |
| データ保存   | JSON（外部DBなし）                                  |
| 候補選択     | インタラクティブTUI（vim keybind: j/k/g/G/Enter/q） |
| 対応シェル   | bash / PowerShell / fish                            |
| スコア方式   | frecency（実行回数 × 時間減衰）                     |
| コンテキスト | カレントディレクトリごとの履歴優先表示              |

---

## CLIインターフェース

```
smart-run add <command>          # コマンドをDBに記録（シェルフックから呼ぶ）
smart-run query <keywords...>    # TUIで候補選択 → コマンドをstdoutに出力
smart-run list [--cmds-only]     # スコア順一覧（--cmds-onlyは補完用）
smart-run remove <command>       # 特定コマンドを履歴から削除
smart-run clean                  # 重複エントリを整理
smart-run init bash              # bash統合スクリプトをstdoutに出力
smart-run init powershell        # PowerShell統合スクリプトをstdoutに出力
smart-run init fish              # fish統合スクリプトをstdoutに出力
```

### シェル統合の仕組み（重要）

`query` コマンドはシェルの `$()` で呼ばれるため、**stdout にはコマンド文字列のみ**を出力する。
シェル側のラッパー関数がそれを受け取り、実行またはバッファに展開する。

```bash
# bash での使い方イメージ
eval "$(smart-run init bash)"
r             # 全履歴をTUIで表示して選択・実行
r docker      # "docker" を含む履歴をTUIで絞り込み
r deploy prod # "deploy" と "prod" 両方含む履歴を絞り込み
```

---

## 予定しているファイル構成

```
smart-run/
├── .claude/
│   ├── CLAUDE.md         ← このファイル
│   └── SKILL.md          ← Claude Code向けスキル定義
├── Cargo.toml
└── src/
    ├── main.rs           # CLIエントリポイント（clap）
    ├── db.rs             # JSON DB + frecencyスコア計算
    ├── matcher.rs        # キーワードマッチング
    ├── ui.rs             # crossterm製インタラクティブTUI
    └── shell.rs          # シェル統合スクリプト（文字列定数）
```

---

## モジュール設計

### db.rs

```rust
pub struct Entry {
    pub command: String,
    pub run_count: u32,
    pub last_run: DateTime<Utc>,
    pub last_dir: Option<String>,  // 実行時のカレントディレクトリ
}

impl Entry {
    pub fn score(&self, current_dir: Option<&str>) -> f64 {
        // frecency = run_count × time_decay × context_boost
        // time_decay = 1.0 / (1.0 + elapsed_hours × 0.01)
        // context_boost = 1.5 if last_dir == current_dir else 1.0
    }
}

pub struct Database {
    pub version: u32,  // 現在は 1
    pub entries: Vec<Entry>,
}
```

**DBファイルパス:**

- Linux/WSL: `~/.local/share/smart-run/db.json`
- Windows: `%APPDATA%\smart-run\db.json`

### matcher.rs

- 全キーワードがコマンドにマッチするAND検索
- 大文字小文字無視
- サブシーケンスマッチ（文字が順序通りに含まれるか）
- スコア順を維持して返す

### ui.rs

- **描画先は stderr**（stdout はコマンド出力専用）
- 上部に `> {query}` プロンプトを表示し、文字入力でインクリメンタル検索（Backspace で削除）
- `r docker` のように引数ありで起動するとその文字列が initial_query として入力済みの状態でTUIが開く
- vim keybind:
  - `j` / `↓` / `Ctrl+n` → 下
  - `k` / `↑` / `Ctrl+p` → 上
  - `g` → 先頭、`G` → 末尾
  - `Enter` → 確定
  - `q` / `Esc` / `Ctrl+c` → キャンセル
  - 文字入力 → クエリに追記してリアルタイムフィルタ（j/k/g/G/q を除く）
- 候補1件のみなら即決定（TUI省略）
- 最大表示件数: 10件（超えた分は「他N件」表示）

### shell.rs

`init_bash()`, `init_powershell()`, `init_fish()` の3関数。
各シェル統合スクリプトを `&'static str` で返すだけ。

シェルフックは以下を行う:

1. コマンド実行前に `smart-run add <command>` を呼び出して記録
2. `r` コマンドで `smart-run query` を呼び出し、返ってきたコマンドをバッファに展開（実行前に確認できるよう）

---

## 依存クレート（案）

```toml
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
crossterm = "0.27"
```

---

## Git ルール

- コミットメッセージに `Co-Authored-By: Claude` 行を**含めない**
- コミットメッセージは**日本語**で書く
- 先頭に `fix` / `feat` / `add` / `update` / `docs` / `test` / `refactor` などのプレフィックスをつける（詳細は `SKILL.md` 参照）

---

## 実装ステータス

| ファイル       | 状態      |
| -------------- | --------- |
| Cargo.toml     | 🔲 未着手 |
| src/main.rs    | 🔲 未着手 |
| src/db.rs      | 🔲 未着手 |
| src/matcher.rs | 🔲 未着手 |
| src/ui.rs      | 🔲 未着手 |
| src/shell.rs   | 🔲 未着手 |

**推奨実装順:** db.rs → matcher.rs → main.rs(add/list) → ui.rs → main.rs(query) → shell.rs

---

## smart-cdとの主な差分

| 項目           | smart-cd         | smart-run                              |
| -------------- | ---------------- | -------------------------------------- |
| 対象           | ディレクトリパス | シェルコマンド文字列                   |
| 追記フィールド | `path`           | `command` + `last_dir`                 |
| シェル側の動作 | `cd` を実行      | コマンドをバッファに展開して実行       |
| スコア補正     | なし             | カレントディレクトリ一致で×1.5ブースト |

---

## 今後の拡張アイデア（スコープ外）

- `--suggest` オプション：頻繁に使うコマンドからエイリアス提案
- プレースホルダー機能：`git push origin <branch>` のようなテンプレートコマンドをインタラクティブに埋める
- fzf 連携オプション
