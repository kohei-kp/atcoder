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

提出もCLIでやるなら、online-judge-toolsを使う。
`scripts`以下のshを実行する。
`scripts`までのパスを通しておく。

```bash
# ex) ABC430のA問題
ojd.sh 430 a # テストケースのダウンロード
ojt.sh 430 a # サンプル実行
ojs.sh 430 a # 提出
```

## 入れているCrateなど
* proconio
* itertools
* petgrapth
