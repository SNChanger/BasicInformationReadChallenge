#[macro_use] extern crate winconsole;
use winconsole::console;
use winconsole::console::ConsoleColor;

const PERSONSEARCHCOMMAND: u32 = 1;
const ALLCHECKCOMMAND: u32 = 2;
const APPENDCOMMAND: u32 = 3;

// apnicでのフォーマット
// http://ftp.apnic.net/stats/apnic/delegated-apnic-latest
// |registry|cc|type|start|value|
// registry.. 各ファイルの先頭で利用する文字列。
// cc........ 国際コード CountryCode.
// type ..... ipアドレスで確認するときの種類 ipv4 asn ipv6 の各種類が設定
// start..... 該当コンピュータの開始アドレス
// value..... start+ipアドレスの個数として該当する数
// date...... 集計した年月日 yyyymmdd 形式で設定される
type IpAddressStruct struct {
    Registry      string
    Cc            string
    AddressType   string
    StartAddress  string
    AddValue      u32
    StatDate      string
}

// セキュリティ研修読み込み
// 利用文献 https://www.ipa.go.jp/files/000100473.pdf
// 海外の IPアドレスからのアクセスを遮断する
fn main() {
    // アカウント毎の権限情報を検索する


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
            let input_address: String = line.replace("\r\n", "").parse();
            // 国内であるかどうか
            let isJp = isJpAddress(input_address);

            if isJp {
               // 日本国内の場合
            } else {
               // 海外の場合
            }


        } else if menu_number == APPENDCOMMAND {
            // アプリ終了
            break
        }

        // 端末画面なので、作業者が観終わるまで経過を待つ
        let mut line = String::new();
        cprintln!(ConsoleColor::Blue, "他の操作をするためにEnterを押してください");
        std::io::stdin().read_line(&mut line).unwrap();
    
    }


}

// ipアドレス調査端末用
fn menu_message() {
    cprintln!(ConsoleColor::Blue, "番号を選択してください");
    cprintln!(ConsoleColor::Yellow, "1.ipアドレス判定(日本かどうか)");
    cprintln!(ConsoleColor::Yellow, "2.終了");
}


// ipアドレスの国内判定
// 日本国内の通信の場合は true
// 海外の通信の場合は     false
fn isJpAddress(_ip_address :String) bool {
    // 世界のコンピュータ判定
    // apnic|JP|ipv4|58.145.228.0|256|20050614|allocated
    // apnic|SG|ipv4|58.145.229.0|256|20050614|allocated
    // apnic|JP|ipv4|58.145.234.0|256|20050614|allocated
    // apnic|SG|ipv4|58.145.235.0|256|20050614|allocated
    worldAddressList := []IpAddressStruct{
        {


        },
        {

        },
        {
            
        },        
    }
 
 
 
    return false
}