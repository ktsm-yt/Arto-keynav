<!-- 読者: Markdownを読むmacOSユーザー。説明なしで使ってよい語: Markdown、macOS、GitHub、README、アプリ、フォルダ、キー、設定、ダウンロード、DMG、Apple Silicon、Intel、Windows、Linux、Terminal、APIキー、MIT。 -->
# Arto Keynav

Arto Keynavは、[Arto](https://github.com/arto-app/Arto)をもとに独自に変更したアプリです。上下キーでMarkdownファイルを切り替えられます。
本家Artoの作者はAlisueさんです。この版の変更・配布は[ktsm-yt](https://github.com/ktsm-yt)が管理しています。

## 本家との違い

- 本文を読んでいるとき、↑↓キーで開いたフォルダ内のMarkdownファイルを順に開きます。
- ←キーで親フォルダへ戻り、→キーで選んだフォルダやファイルを開きます。
- 設定画面の表示を日本語にしています。
- 左側のファイル一覧の拡大率に合わせて、左上のアイコンも拡大します。
- 本家とは設定・履歴・一時データ・APIキーの保存先を分けています。

## macOS試用版を入れる

対象はApple SiliconのmacOSです。Intel、Windows、Linux向けの配布はまだ行いません。

1. [ダウンロードページ](https://github.com/ktsm-yt/Arto-keynav/releases)から、末尾が`aarch64.dmg`のファイルを入手します。
2. DMGを開き、`Arto Keynav.app`をApplicationsフォルダへコピーします。
3. Markdownファイルやフォルダを、アプリのウィンドウへドラッグします。

この試用版はAppleの配布前チェックを受けていません。macOSが起動を止めた場合は、配布元を確認したうえで、システム設定の『プライバシーとセキュリティ』から、このアプリの起動を許可してください。
パソコン全体の保護設定を無効にする必要はありません。

ダウンロードしたDMGを確認するため、同じページに`SHA256SUMS`を載せます。両方をダウンロードフォルダに保存し、Terminalで次を実行します。

```sh
cd ~/Downloads
shasum -a 256 -c SHA256SUMS
```

DMGの名前の横に`OK`と出れば、確認できています。

## 本家と一緒に使う

本家の`Arto.app`を置き換えずにインストールできます。
設定・履歴などは`~/Library/Application Support/arto-keynav/`に保存します。本家の設定を自動ではコピーしません。
APIキーも、この版であらためて登録してください。

Finderでスペースキーを押して表示するプレビュー機能は、この版には同梱しません。本家をインストール済みなら、本家のプレビューを利用できます。
Terminalから使う場合は、次のコマンドでMarkdownファイルを開きます。

```sh
"/Applications/Arto Keynav.app/Contents/MacOS/arto" README.md
```

## 不具合を報告する

[この版のIssues](https://github.com/ktsm-yt/Arto-keynav/issues)へ、macOSのバージョン、再現手順、使ったキーを書いてください。
共有できる短いMarkdownがあると確認しやすくなります。APIキーや非公開の文章は載せないでください。
本家への報告は、本家でも同じ問題が起きると確認できた場合にお願いします。

## ソースからビルドする

Rust、Dioxus CLI、Node.js、pnpm、just、Xcode Command Line Toolsを使います。[開発環境の説明](CONTRIBUTING.md)も参照してください。
macOSの配布物は、Nixの開発環境を使わず、macOS標準のライブラリでビルドします。

```sh
just fmt check test
just build
just verify-bundle
```

DMGは`target/dx/arto/bundle/macos/macos/`にできます。DMG内のアプリ名は`Arto Keynav.app`です。
ビルド時にアプリは起動しません。

## ライセンスと出典

本家の著作権表示と[MITライセンス](LICENSE)を保持しています。配布アプリ内にもLICENSEと[依存ライブラリのライセンス表示](THIRD-PARTY-NOTICES.txt)を同梱します。
本家のリポジトリ: [arto-app/Arto](https://github.com/arto-app/Arto)

本家には、GitHub風のMarkdown表示、図や数式の表示、色の変更、検索などの機能があります。
この版はその実装を利用し、ファイルのキー操作と設定画面を変更しています。
