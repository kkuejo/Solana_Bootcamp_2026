//AnchorのRustのライブラリanchor_langを使用している
//preludeはよく使うものだけを再エクスポートしている。
//*はanchor_langのすべての機能をインポートする
use anchor_lang::prelude::*;

declare_id!("4uHSV5iG6ydTK9rXzzzJsDBHWVR96yjkQQnJaaQxwrk8");

//programマクロはプログラムのエントリーポイントを定義する。つまり、Solanaが実際に呼べる関数を定義する。
#[program]
//hello_worldモジュールはプログラムのエントリーポイントを定義する。
mod hello_world{
    //superは親モジュールを参照する。つまり、hello_worldモジュールの親モジュールを参照する。
    //親モジュールでは、use anchor_lang::prelude::*;ですべての機能をインポートしている。
    //use super::*;は親モジュールで定義されたこの機能を使用する。
    use super::*;

    pub fn hello(_ctx:Context<Hello>) -> Result<()>{
        msg!("Hello World !!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello{}
