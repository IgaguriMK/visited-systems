visited-systems
==============

Elite: DangerousのVisited Starsの修正用のImportStars.txtをジャーナルファイルから作成するツール

## 用法

1. `C:\Users\(ユーザー名)\Saved Games\Frontier Developments\Elite Dangerous` にジャーナルファイルが存在することを確認する。（不足していればバックアップ等から戻して追加する）
2. このツールを実行する。
3. `outputs`以下にコマンダーごとのフォルダに分けて`ImportStars.txt`が生成される。
4. `outputs/id.txt`にコマンダーとIDの対応が書いてあるので、IDを確認する（頭の`F`は不要）。
5. 適応したい`ImportStars.txt`を`C:\(ユーザー名)\Appdata\Local\Frontier Developments\Elite Dangerous\(コマンダーID)\ImportStars.txt`に配置する。
6. インポートしたいコマンダーでゲームを起動すると自動的に読み込まれる。

### アップロード漏れチェックモード

コマンド: `visited-systems check-dump <CMDR NAME> ImportStars.txt`

`ImportStars.txt` に含まれない星系へのジャンプ履歴のあるジャーナルファイルを探し、`missing_journals`以下にコピーする。
EDSMにジャーナルがアップロードされていないことを間接的に検出して手動でアップロードすることを想定している。

## ダウンロード

→ [Releases](https://github.com/IgaguriMK/visited-systems/releases)

## License

This software is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
