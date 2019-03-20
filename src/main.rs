#[allow(dead_code)]

use rand::Rng;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::fmt;
use std::u64;

struct Header{  //構造体を定義
    data: String,
    hash: u64,
    nonce:u8,
}

impl fmt::Display for Header {  //Headerを表示できるようにfmt::Displayトレイトを実装
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n data: {} \n nonce: {} \n hash: {:X}", self.data, self.nonce, self.hash)
    }
}

fn main() {
    let mut nnonce : u8;    //符号なし整数で表されたナンス。繰り返しごとに数が変化するためmutをつける
    let mut nonce;  //Stringで表されたナンス。繰り返しごとに数が変化するためmutをつける
    let mut hash : u64;
    let diff : u64 = 1000000000000000000;   //ディフィカルティ。マイニングの難易度。数が小さいほど難しい。18446744073709551615が最大

    let args: Vec<String> = std::env::args().collect(); //std::env::args().collect()でコマンドライン引数を配列に収めている
    let data = &args[1];    //こちらはきちんと参照を使っているのでセーフ
    //let data = args[1];   //これは所有権の移動ができないためにエラーが起こる。そのため参照の＆をつける必要がある。
    let mut i: u64 = 1;

    loop{//無限ループ
        
        let mut rng = rand::thread_rng();   //乱数のインスタンス生成
        let mut sha256 = Sha256::new(); //sha256のインスタンスを作成

        nnonce = rng.gen(); //乱数生成
        nonce = nnonce.to_string(); //乱数を文字に直す
        nonce = nonce + &data;  //文字列の結合
        
        sha256.input_str(&nonce);   //結合した文字列をハッシュ関数のぶち込む
        let string = &String::from(sha256.result_str())[0..16]; //256bitはデフォルトだと扱えないので先頭64bitを切り出す
        hash = u64::from_str_radix(string, 16).unwrap();    //切り出した64bitは16進数なので10進数の符号なし整数に直す
        if hash < diff{     //ディフィカルティより小さければOKなのでループを抜ける
            break;
        }
        i = i + 1;
    }

    let header = Header {   //Headerを生成
        data: data.clone(), //dataを移動することはできないので複製する
        nonce: nnonce, //hashとnonceは数字列＝copy可能なので明示的に複製する必要はない
        hash: hash,
    };

    println!("{} \n Computed {} times.", header, i);    //実装したDisplayトレイトを用いて表示
    println!("\n args is {:?}", args);
    
    
}

 
