//Mochaのテスト構文。
//describeは、テストのグループを定義する。
//itは、テストの具体的な内容を定義する。
//awaitは、非同期処理を待つ。
//pg.program.methodsは、プログラムのメソッドを呼び出す。
//Anchor が生成したクライアント（pg はテスト環境で用意されているプログラム用オブジェクト）から、オンチェーンの hello 命令を呼び出す。
//.hello()は、helloメソッドを呼び出す。
//.accounts({})は、アカウントを渡す。
//.rpc()は、RPCを呼び出す。
describe("Test", () => {
  it("hello", async () =>{
    await pg.program.methods
    .hello()
    .accounts({})
    .rpc()
  })
})