// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.


use std::num::ParseIntError;

/// GPT: 在调用者中使用 ? 操作符来简化错误处理， 则调用者(main)
/// 返回一个实现了 std::process::Termination trait 的类型。
/// 代码中， main 函数的返回类型须是 Result<(), ParseIntError>，
/// 这是因为在 main 函数中使用了 ? 操作符，而 ? 操作符要求返回一个 Result 类型。
/// 这样，如果任何操作返回 Err，它将会自动从 main 函数返回该错误，而不是继续执行。
/// 我: 有点像 throw. 因为我们只处理了total_cost的正确结果，
/// 而没有处理来自total_cost的错误，所以main这里需要将其抛出
fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    /// 在这一句之后处理 Err 的话就不需要修改 main 的 return type
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
