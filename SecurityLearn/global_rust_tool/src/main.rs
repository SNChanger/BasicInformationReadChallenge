#[macro_use] extern crate winconsole;
use winconsole::console::ConsoleColor;

// ipアドレスのクラスCの場合の、ホスト最大数
// この数以下の場合は、一番末端だけ加算処理をする
const ADDRESSSEARCHCOMMAND: u32 = 1;
const APPENDCOMMAND: u32 = 2;
const CLASSCHOSTPART: u32 = 256;

// apnicでのフォーマット
// http://ftp.apnic.net/stats/apnic/delegated-apnic-latest
// |registry|cc|type|start|value|
// registry.. 各ファイルの先頭で利用する文字列。
// cc........ 国際コード CountryCode.
// type ..... ipアドレスで確認するときの種類 ipv4 asn ipv6 の各種類が設定
// start..... 該当コンピュータの開始アドレス
// value..... start+ipアドレスの個数として該当する数
// date...... 集計した年月日 yyyymmdd 形式で設定される
struct IpAddressStruct {
    registry:      String,
    cc:            String,
    address_type:   String,
    start_address:  String,
    add_value:      u32,
    stat_date:      String,
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
        if menu_number == ADDRESSSEARCHCOMMAND {
            let mut line = String::new();
            cprintln!(ConsoleColor::Blue, "権限を確認する番号を入力してください");
            std::io::stdin().read_line(&mut line).unwrap();
            let input_address: String = line.replace("\r\n", "").parse().unwrap();
            // 国内であるかどうか
            let is_jp = is_jp_address(input_address);

            if is_jp {
               // 日本国内の場合
               cprintln!(ConsoleColor::Blue, "日本国内のコンピュータだったので、接続手続きを開始します");
            } else {
               // 海外の場合
               cprintln!(ConsoleColor::Blue, "システムエラー!? 詳しくは管理者にお問い合わせをしてください");
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
fn is_jp_address(_ip_address :String) -> bool {
    // 世界のコンピュータ判定
    let world_address_list:Vec<IpAddressStruct>  =vec![
        IpAddressStruct{
            registry: "apnic".to_string(),
            cc: "SG".to_string(),
            address_type: "ipv4".to_string(),
            start_address: "58.145.229.0".to_string(),
            add_value: 256,
            stat_date: "20050614".to_string(),
        },
        IpAddressStruct{
            registry: "apnic".to_string(),
            cc:             "JP".to_string(),
            address_type:             "ipv4".to_string(),
            start_address:             "58.145.234.0".to_string(),
            add_value:             256,
            stat_date:             "20050614".to_string(),
        },
        IpAddressStruct{
            registry:            "apnic".to_string(),
            cc:             "JP".to_string(),
            address_type:            "ipv4".to_string(),
            start_address: "58.145.235.0".to_string(),
            add_value: 256,
            stat_date:"20050614".to_string(),
        },
    ];


    // 世界の登録されているコンピュータの一覧を読み込む
    for world_address_data in world_address_list {
        let start_address = world_address_data.start_address;
        // 

        //あぁ、久保がブランニューデー... 酒飲んだ時の冗談として、２人と話したのか...
        //素面の時にマネージャーの話してなかったんだろうね...
        for number in 1..world_address_data.add_value {
            if number < CLASSCHOSTPART {
                // ipアドレスの一番下位部分を設定する
                let addresses: Vec<&str> = start_address.split(".").collect();

                // 各種コンピュータ毎のアドレスを作成して、確認をする
                let address_base = format!("{}.{}.{}.{}", addresses[0], addresses[1], addresses[2], number);

                // 設定済みのipアドレスが一致していれば、日本国内なのでtrueを返す
                if address_base == _ip_address {
                    return true
                }

            }
        }
    }
 
    false
}


fn clear_message() {
    print!("\x1B[2J");
}