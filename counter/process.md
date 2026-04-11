1. npx create-solana-dapp --list-templatesとターミナルに打つ。
2. npx create-solana-dapp -t web3js-next-tailwind-counterで、web3js-next-tailwind-counterのテンプレートをセットアップ
3. プログラムのBuildを行う。
    cd anchorに移動
    anchor buildと打つ。
    anchor buildはRustコードをコンパイルして、バイトコードを生成するのみ
4. チェーンを起動(Solana Test Validator を利用する。)
    別のターミナルを起動
    cd anchorに移動
    solana-test-validatorと打って実行
5. Deployを行う。
    buidを行ったターミナルに戻る。
    anchor deployと打つ
    Anchor Deployは、プログラムをブロックチェーンにアップロードして、アカウントを作成。更に、トランザクションを送信して、プログラムIDを取得という4ステップを行う。
6.エラーが出るのでプログラムIDを設定する。
    lib.rsとAnchor.tomlにプログラムIDを設定する。
    ビルドとデプロイを行う。
7. テストを行う
    anchor test  --skip-local-validatorと打つ。
8. フロントエンドを立ち上げる
    新しいターミナルで、npm run devと打つ。
9. Phantom Walletの設定で、デベロッパー設定から、Solana Localnetでの接続にする。
10. 新しいターミナルで、solana airdrop 2 <アドレス>と打つと、アドレスに2solを送信する。
11. フロントエンドのCounter Programタブで、Counterが動くことを確認。
12. 次からカウンターで2増やせる機能を追加する。
    12-1. スマートコントラクトの修正: anchor/programs/counter/src/lib.rsに、「pub fn increment_by_two」を追加する。
    12-2. src/components/counterのcounter-data-access.tsxとcounter-ui.tsxを修正する。
        counter-data-access.tsxには、「const incrementByTwoMutation」を追加
    12-3. counter-ui.tsxには、タブを追加

