
use std::collections::HashMap;

// アカウント毎のアプリ内の権限設定
#[derive(Clone)]
pub struct GrantInformation {
   pub id: u32,
   pub name: String,
   pub method_name: String
}


pub const DEFAULT_USER_GROUP_ID: u32 = 120;
pub const FULL_USER_GROUP_ID: u32 = 140;

/*
 * セキュリティ白書の学習
 * 元社員などの退職後のデータ及び、自前PCでログイン可能なリモートデスクトップがあるところから考えた 
 * 対象アカウントの権限を確認する
 * 退職後に手続きが残っている場合、権限が返ってきてしまうのでオフに運用側でするような想定 
 * 引数 検索確認をするユーザid 整数32ビット側
 * 戻り値 検索した結果の権限一覧情報　該当アカウントが存在しないものの場合は、全権限無しの空にする  
 */
pub fn get_product_list(_user_id :u32) -> Vec<GrantInformation> {


    println!("{}を検索開始します", _user_id);
    let mut user_grants = HashMap::new();
    let mut default_user_grants = Vec::<GrantInformation>::new();
    default_user_grants.push(build_grant_information(1000, "運営画面ログイン".to_string(), "admin_login".to_string()));
    default_user_grants.push(build_grant_information(1010, "定期検査".to_string(), "maintenance".to_string()));
    let mut full_user_grants = Vec::<GrantInformation>::new();
    full_user_grants.push(build_grant_information(1000, "運営画面ログイン".to_string(), "admin_login".to_string()));
    full_user_grants.push(build_grant_information(1010, "定期検査".to_string(), "maintenance".to_string()));
    full_user_grants.push(build_grant_information(1020, "機械稼働状況確認".to_string(), "check_machine_helth".to_string()));
    full_user_grants.push(build_grant_information(1030, "機械検査対応承認".to_string(), "machine_apply".to_string()));

    user_grants.insert(100, default_user_grants.to_vec());
    user_grants.insert(DEFAULT_USER_GROUP_ID, default_user_grants.to_vec());
    user_grants.insert(FULL_USER_GROUP_ID, full_user_grants.to_vec());

    // 既に退職されていて、権限がない場合は、権限なしの空情報を返す
    if user_grants.contains_key(&_user_id) == false {
        let empty_user_grants = Vec::<GrantInformation>::new();
        return empty_user_grants;
    }


    user_grants[&_user_id].to_vec()
}

/**
 * 権限情報を出力する
 * id          権限id
 * name        権限名称
 * method_name メソッド名称
 */
pub fn build_grant_information(id :u32, name :String, method_name :String) -> GrantInformation {
    GrantInformation{
        id,
        name,
        method_name,
    }
}