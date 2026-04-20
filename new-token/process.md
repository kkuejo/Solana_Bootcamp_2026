1. Token mint accountの作成
2. npm init -y
3. npm init -y
4. npm i @solana/kit @solana-program/system @solana-program/token-2022 
5. touch new-token.tsの作成
6. 動作確認
　 terminalは
 　TSXのインストール
　　npm i -D tsx
　 package.jsonを書き換え、  "type": "commonjs",を  "type": "module"へに変更
7. ローカルチェーン起動
　　surfpool start
8. npx tsx new-token.ts
　　