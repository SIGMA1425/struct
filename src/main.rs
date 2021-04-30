//構造体
struct Sensor{
    active: bool,
    latest: u32,
}

//構造体のメソッドや関連関数の実装
impl Sensor{
    //メソッド，第一引数は必ず&self
    fn read(&self) -> u32{
        self.latest
    }

    //オブジェクトの値を変更するときは&mut
    fn init(&mut self){
        self.active = true;
        self.latest = 42;
    }

    //関連関数，selfを受け取らない．基本的にはnew()としてコンストラクタとする
    fn new() -> Sensor{
        Sensor{
            active: false,
            latest: 0,
        }
    }
}

//列挙型
enum Type{
    Int,
    Float,
    Boolean,
}

impl Type{
    //メソッド，関連関数の記述は構造体と同じ
    fn method(&self){
        unimplemented!();
    }

    fn associated_func(){
        unimplemented!();
    }
}

//match演算子を使用したパターンマッチ
fn print_type(t: Type){
    match t{
        Type::Int => println!("type is integer"),
        Type::Float => println!("type is floating"),
        Type::Boolean => println!("type is boolean"),
    }
}


fn main() {
    //オブジェクトの生成．可変にするときはmut
    let mut sensor = Sensor::new();
    let int = Type::Int;
    print_type(int);

    sensor.init();

    println!("active is {}, latest is {}", sensor.active, sensor.read());

}

