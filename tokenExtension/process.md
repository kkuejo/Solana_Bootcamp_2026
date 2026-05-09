1. Token Extensionの為のフォルダを作成
2. 上のフォルダに移り、pnpm initでプロジェクトを初期化
   (pnpmが何なのをを説明してください。)
3. pnpm add @solana-program/system @solana-program/token-2022 @solana/kit
   (net上の@solana/web3.jsは古いパッケージ、現在は@solana/kitを用いる)
4. pnpm add -D tsx
5.Package.jsonに以下のように、"type": "module"を追加。awaitを使う時に入れたほうが良い。
  "name": "tokenExtension",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "type": "module",
6. touch src/metadata.tsでファイルを作成
7. validatorを立ち上げておく(surfpool start)
8. metadata.tsを作成
9. pnpm tsx src/metadata.ts

3:42:30