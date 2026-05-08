# 保守用タスク

# Cargo.toml の互換範囲内で依存クレートを更新する
update-safe:
    {{CARGO}} update

# 依存クレートを最新版へ更新し、Cargo.lock も更新する
update:
    {{CARGO}} upgrade -i
    {{CARGO}} update
