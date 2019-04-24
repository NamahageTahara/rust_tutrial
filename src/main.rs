
use rand::Rng;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::fmt;
use std::u64;

struct Header{  //構造体を定義
    data: String,
    hash: u64,
    nonce:u64,
}

impl fmt::Display for Header {  //Headerを表示できるようにfmt::Displayトレイトを実装
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n data: {} \n nonce: {} \n hash: {:X}", self.data, self.nonce, self.hash)
    }
}

fn main() {

    let args: Vec<String> = std::env::args().collect();
    //let head = &arg[0]; //std::env::args().collect()でコマンドライン引数を配列に収めている
    //let data = args[1];   //これは所有権の移動ができないためにエラーが起こる。そのため参照の＆をつける必要がある。

    for i in 1..4 {
        let data = &args[i];
        let (hash, count, nnonce) = cal_hash(&data);

        let header = Header {   //Headerを生成
            data: data.to_string(), //dataを移動することはできないので複製する
            nonce: nnonce, //hashとnonceは数字列＝copy可能なので明示的に複製する必要はない
            hash: hash,
        };

        println!("{} \n Computed {} times.", header, count);    //実装したDisplayトレイトを用いて表示
    }
}

fn cal_hash(_data: &String) -> (u64,u64,u64)  {
    let mut _count : u64 = 0;
    let mut _nonce;
    let mut _nnonce : u64;
    let mut _hash;

    let diff : u64 = 1000000000000000000;   //ディフィカルティ。マイニングの難易度。数が小さいほど難しい。18446744073709551615が最大

    loop{//無限ループ
        
        let mut rng = rand::thread_rng();   //乱数のインスタンス生成
        let mut sha256 = Sha256::new(); //sha256のインスタンスを作成
        _nnonce = rng.gen(); //乱数生成
        _nonce = _nnonce.to_string();
         //乱数を文字に直す

        sha256.input_str(&format!("{}{}", _nonce, _data.to_string()));   //結合した文字列をハッシュ関数のぶち込む
        let string = &String::from(sha256.result_str())[0..16]; //256bitはデフォルトだと扱えないので先頭64bitを切り出す
        _hash = u64::from_str_radix(string, 16).unwrap();    //切り出した64bitは16進数なので10進数の符号なし整数に直す

        if _hash < diff{     //ディフィカルティより小さければOKなのでループを抜ける
            break;
        }

        _count = _count + 1;
    }

    return (_hash, _count, _nnonce);   
}

 
