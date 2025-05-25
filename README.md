# atcoder
AtCoderやる用

## ルール
* `a.rs`のような純粋なファイル名は初回
* 解説ACはa`ref_a.rs`のようにする
* 一度やったものを再度やる場合は`retry_a.rs`のようにする

## 使い方
* `cargo make create abc{number}`でプロジェクトを作成する
* `cargo run -p abc{number} --bin a`で実行する

```bash
cargo make create abc400
cargo run -p abc400 --bin a
```

## 入れているCrateなど
* proconio
* itertools
* petgrapth

2025/05/25時点
