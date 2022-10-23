mod auth;



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_grant_information() {
        let grant_id: u32 = 100;
        let test_grant_name = "テスト".to_string();
        let method_name = "health_check".to_string();
        let grant_info = auth::build_grant_information(grant_id, test_grant_name, method_name);

        assert!(grant_info.id == grant_id);
        assert!(grant_info.name.as_str() == "テスト");
        assert!(grant_info.method_name.as_str() == "health_check");
    }

    #[test]
    fn can_get_product_list() {
        // 標準として登録されているデータ確認をする
        assert!(auth::get_product_list(auth::DEFAULT_USER_GROUP_ID).len() > 0);
        assert!(auth::get_product_list(auth::FULL_USER_GROUP_ID).len() > 0);
    }
}