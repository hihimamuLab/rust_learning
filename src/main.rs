use std::io;

fn main() {
    let mut input = String::new();

    // Ctrl+D (Unix) または Ctrl+Z (Windows) を入力するまで繰り返し入力を受け付ける
    while let Ok(bytes_read) = io::stdin().read_line(&mut input) {
        if bytes_read == 0 {
            break;
        }

        // 入力が空行だったらループを抜ける
        if input.trim().is_empty() {
            break;
        }

        // 入力を使って何か処理を行う（ここでは入力を表示するだけ）
        println!("入力: {}", input);

        // 次の入力を受け付けるために input をクリアする
        input.clear();
    }
}
