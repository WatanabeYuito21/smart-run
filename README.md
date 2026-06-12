# smart-run

frecency（頻度×最近さ）ベースのスマートコマンドヒストリーツール。

よく使う・最近使ったコマンドをインタラクティブなTUIで素早く検索・再実行できます。

**対応環境:** Linux / WSL / Windows（単一バイナリ）  
**対応シェル:** bash / PowerShell / fish

## インストール

```bash
cargo install --path .
```

## セットアップ

### bash

```bash
# ~/.bashrc に追加
eval "$(smart-run init bash)"
```

### fish

```fish
# ~/.config/fish/config.fish に追加
smart-run init fish | source
```

### PowerShell

```powershell
# $PROFILE に追加
Invoke-Expression (smart-run init powershell)
```

## 使い方

シェル統合後、`r` コマンドで履歴を検索できます。

```bash
r             # 全履歴をTUIで表示
r docker      # "docker" を含む履歴に絞り込み
r deploy prod # "deploy" と "prod" 両方含む履歴に絞り込み
```

TUI内の操作:

| キー | 動作 |
|---|---|
| `j` / `↓` / `Ctrl+n` | 下に移動 |
| `k` / `↑` / `Ctrl+p` | 上に移動 |
| `g` | 先頭へ |
| `G` | 末尾へ |
| 文字入力 / `Backspace` | クエリを編集してリアルタイム絞り込み |
| `Enter` | 選択したコマンドをバッファに展開 |
| `q` / `Esc` / `Ctrl+c` | キャンセル |

> 候補が1件のみの場合はTUIをスキップして即座に展開します。

## スコアリング

実行頻度と最近さを組み合わせた frecency スコアで順位付けします。

```
score = run_count × time_decay × context_boost

time_decay    = 1.0 / (1.0 + elapsed_hours × 0.01)
context_boost = 1.5  # カレントディレクトリが一致する場合
              = 1.0  # それ以外
```

同じディレクトリでよく使うコマンドが上位に表示されます。

## データ保存場所

| OS | パス |
|---|---|
| Linux / WSL | `~/.local/share/smart-run/db.json` |
| Windows | `%APPDATA%\smart-run\db.json` |

## コマンドリファレンス

```
smart-run add <command> [--dir <path>]   # コマンドを記録（シェルフックが自動で呼ぶ）
smart-run query [keywords...]            # TUIで検索・選択
smart-run list [--cmds-only]             # スコア順に一覧表示
smart-run remove <command>               # 特定コマンドを削除
smart-run clean                          # 重複エントリを整理
smart-run init bash|powershell|fish      # シェル統合スクリプトを出力
```

## 依存クレート

- [clap](https://crates.io/crates/clap) — CLIパーサー
- [serde](https://crates.io/crates/serde) / [serde_json](https://crates.io/crates/serde_json) — JSON読み書き
- [chrono](https://crates.io/crates/chrono) — 時刻処理
- [crossterm](https://crates.io/crates/crossterm) — クロスプラットフォームTUI
