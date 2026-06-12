pub fn init_bash() -> &'static str {
    r#"
# smart-run bash integration
__smart_run_precmd() {
    local cmd
    cmd="$(HISTTIMEFORMAT= history 1 | sed 's/^[[:space:]]*[0-9]\+[[:space:]]*//')"
    [[ -z "$cmd" || "$cmd" == "r" || "$cmd" == r\ * || "$cmd" == "smart-run"* ]] && return
    smart-run add "$cmd" --dir "$PWD" 2>/dev/null
}
PROMPT_COMMAND="${PROMPT_COMMAND:+${PROMPT_COMMAND%;}; }__smart_run_precmd"

r() {
    local cmd confirmed
    cmd="$(smart-run query "$@")" || return
    # コマンドを編集可能な状態で表示し、Enter で実行 / Ctrl+C でキャンセル
    read -e -i "$cmd" -p "$ " confirmed
    [[ -n "$confirmed" ]] || return
    history -s "$confirmed"
    eval "$confirmed"
}
"#
}

pub fn init_powershell() -> &'static str {
    r#"
# smart-run PowerShell integration
$__original_prompt = $function:prompt
function prompt {
    $cmd = (Get-History -Count 1).CommandLine
    if ($cmd -and $cmd -notmatch '^(r |r$|smart-run)') {
        smart-run add $cmd --dir (Get-Location).Path 2>$null
    }
    & $__original_prompt
}

function r {
    $cmd = smart-run query @args
    if ($cmd) {
        # バッファにコマンドを挿入（Enter で実行 / Ctrl+C でキャンセル）
        [Microsoft.PowerShell.PSConsoleReadLine]::Insert($cmd)
    }
}
"#
}

pub fn init_fish() -> &'static str {
    r#"
# smart-run fish integration
function __smart_run_postexec --on-event fish_postexec
    set -l cmd $argv[1]
    if test -n "$cmd"
        and not string match -qr '^(r |r$|smart-run)' -- "$cmd"
        smart-run add "$cmd" --dir (pwd) 2>/dev/null
    end
end

function r
    set cmd (smart-run query $argv)
    if test -n "$cmd"
        # バッファにコマンドを挿入（Enter で実行 / Ctrl+C でキャンセル）
        commandline $cmd
    end
end
"#
}
