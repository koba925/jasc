// TODO: Option<Rc<Refcell>>を<Option<Box>>とOption::takeで書き直せるのでは？ → ダメぽい
// TODO: ifやwhileをExprにするか、Exprが出てくるところをStmtにするかする
// TODO: 比較演算子を作る
// TODO: continueを作る
// TODO: 配列を作る
// TODO: 組み込み関数を作る
// TODO: 文字列を作る
// TODO: 辞書を作る
// TODO: モジュールを作る

use jasc::ast::Value;
use std::io;

fn main() {
    let src = io::read_to_string(io::stdin()).expect("Error: failed to read the code.");
    match jasc::run(src) {
        Ok(Value::Null) => {}
        Ok(value) => println!("{}", value),
        Err(errors) => {
            for e in errors {
                e.report();
            }
        }
    }
}
