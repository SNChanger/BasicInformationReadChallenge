mod auth;
#[macro_use] extern crate winconsole;
use winconsole::console;
use winconsole::console::ConsoleColor;

const PERSONSEARCHCOMMAND: u32 = 1;
const ALLCHECKCOMMAND: u32 = 2;
const APPENDCOMMAND: u32 = 3;

fn main() {
    // アカウント毎の権限情報を検索する

    start_message();

    loop {
        
        clear_message();

        menu_message();
        let mut menu = String::new();
        std::io::stdin().read_line(&mut menu).unwrap();
        let menu_number: u32 = menu.replace("\r\n", "").parse().unwrap();
        if menu_number == PERSONSEARCHCOMMAND {
            let mut line = String::new();
            cprintln!(ConsoleColor::Blue, "権限を確認する番号を入力してください");
            std::io::stdin().read_line(&mut line).unwrap();
            let personal_number: u32 = line.replace("\r\n", "").parse().unwrap();
            // 作業チームの権限
            output_grant_list(personal_number, "社員検索".to_string());
        } else if menu_number == ALLCHECKCOMMAND {
           // 管理者側の権限
           output_grant_list(auth::FULL_USER_GROUP_ID, "中部管理部隊".to_string());
    
           output_grant_list(240, "旧東北部署".to_string());

        } else if menu_number == APPENDCOMMAND {
            // アプリ終了
            break
        }

        // 端末画面なので、作業者が観終わるまで経過を待つ
        let mut line = String::new();
        cprintln!(ConsoleColor::Blue, "他の操作をするためにEnterを押してください");
        std::io::stdin().read_line(&mut line).unwrap();
    
    }

    output_end_message();


}

// アカウントに応じた権限情報を出力する
fn output_grant_list(_user_id :u32, _display_name :String) {
    // 開始メッセージ
    println!("アカウント利用権限確認を開始します");
    println!("{}", _display_name);
    let list = auth::get_product_list(_user_id);

    // 退職や閉鎖アカウントの権限確認が残っている場合は
    // アラートメッセージを出して終了
    if list.len() <= 0 {
        console::flush_output().unwrap();
        cprintln!(ConsoleColor::Yellow, "終了済みアカウントの確認作業が実行されました");
        cprintln!(ConsoleColor::Yellow, "設定画面で確認をしてください");
        return
    }

    // アカウントが持っている必要な権限一覧を出力する
    for grant_data in list {
        println!("{} {} {}", grant_data.id, grant_data.name, grant_data.method_name);
    }

}


fn start_message () {
    cprintln!(ConsoleColor::Blue, "これから、アカウント情報検査を開始");
}


fn output_end_message () {
    cprintln!(ConsoleColor::Blue, "検査終了");
    cprintln!(ConsoleColor::Blue, "お疲れ様でした");
}

fn menu_message() {
    cprintln!(ConsoleColor::Blue, "番号を選択してください");
    cprintln!(ConsoleColor::Yellow, "1.社員番号検査");
    cprintln!(ConsoleColor::Yellow, "2.登録済み事務所状況確認");
    cprintln!(ConsoleColor::Yellow, "3.終了");

}

fn clear_message() {
    print!("\x1B[2J");
}