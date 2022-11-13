
# プログラム説明

# プログラム内の構造

## 関数の説明

| 関数名  | 説明 |  引数  |  戻り値  |
| main | メイン関数 | ---- |  ---- | 
| menu_message | ipアドレス検索画面用関数 | ---- |  ---- | 
| isJpAddress | 日本国内のipアドレスであるかどうか | _ip_address ...検索するipアドレス |  論理型 日本国内の場合はtrue | 

## ipアドレス構造情報の説明

| 物理名称  |  説明  |  設定例  | 型 |
| Registry | ファイルの先頭に付与する文字列 | ---- | 文字列 |
| Cc | 国際コード CountryCode. | JP | 文字列 |
| AddressType | ipアドレスで確認するときの種類 | ipv4 ipv6 | 文字列 |
| StartAddress | 該当行の開始ipアドレス | 1.0.1.0 , 192.100.30.1 | 文字列 |
| AddValue | ipアドレスの個数として利用する数 | 256 | 整数 |
| StatDate | 集計した年月日 yyyymmdd形式 | 2022/08/05 | 文字列 |
