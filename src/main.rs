//! 文档注释
//! 文档注释

use std::fmt;
/// 文档注释
/// 稳定注释
use std::fmt::Display;
use std::sync::mpsc::sync_channel;
use std::thread::sleep;
use std::thread::spawn;

// 有理数和复数社区库，没有标准库
use num::complex::Complex;

fn comment() {
    // 普通行注释

    /*
     * 普通块注释
     */
}
fn out() {
    /*
     * 格式化输出
     * format!：将格式化文本写到字符串。
     * print!：与 format! 类似，但将文本输出到控制台（io::stdout）。
     * println!: 与 print! 类似，但输出结果追加一个换行符。
     * eprint!：与 print! 类似，但将文本输出到标准错误（io::stderr）。
     * eprintln!：与 eprint! 类似，但输出结果追加一个换行符。
     */
    // 通常情况下，`{}` 会被任意变量内容所替换。
    // 变量内容会转化成字符串。
    println!("{} days", 31);

    // 不加后缀的话，31 就自动成为 i32 类型。
    // 你可以添加后缀来改变 31 的类型（例如使用 31i64 声明 31 为 i64 类型）。

    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用命名参数。
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // 可以在 `:` 后面指定特殊的格式。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 "     1"，5 个空格后面连着 1。
    println!("{number:>width$}", number = 1, width = 6);

    // 你可以在数字左边补 0。下面语句输出 "000001"。
    println!("{number:>0width$}", number = 1, width = 6);

    // println! 会检查使用到的参数数量是否正确。
    // println!("My name is {0}, {1} {0}", "Bond");
    // 改正 ^ 补上漏掉的参数："James"

    // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    #[allow(dead_code)]
    struct Structure(i32);

    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    // println!("This struct `{}` won't print...", Structure(3));
    // 改正 ^ 注释掉此行。
}

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn debugout() {
    // 使用 `{:?}` 打印和使用 `{}` 类似。
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` 也可以打印！
    println!("Now {:?} will print!", Structure(3));

    // 使用 `derive` 的一个问题是不能控制输出的形式。
    // 假如我只想展示一个 `7` 怎么办？
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化打印
    println!("{:#?}", peter);
}

struct DisplayStructure(i32);

impl Display for DisplayStructure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x:{}, y:{}", self.x, self.y)
    }
}

fn displayout() {
    let display_structure = DisplayStructure(1);
    println!("Compare structures");
    println!("Display:{}", display_structure);
    // println!("Debug:{:?}", display_structure);

    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // 报错。`Debug` 和 `Display` 都被实现了，但 `{:b}` 需要 `fmt::Binary`
    // 得到实现。这语句不能运行。
    // println!("What does Point2D look like in binary: {:b}?", point);
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

fn DisplayList() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn DisplayCity() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city)
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // 在添加了针对 fmt::Display 的实现后，请改用 {} 检验效果。
        println!("{:?}", *color)
    }
}

fn hellworld() {
    comment();
    out();
    debugout();
    displayout();
    DisplayList();
    DisplayCity();
    println!("Hello World");
}

fn var_shadowing() {
    /*
    *变量遮蔽的用处在于，如果你在某个作用域内无需再使用之前的变量（在被遮蔽后，无法再访问到之前的同名变量），
    就可以重复的使用变量名字，而不用绞尽脑汁去想更多的名字。
    *   */
    let x = 5;
    // 在var_shadowing函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The vaule of x in the scope is:{}", x);
    }
    println!("The value of x is: {}", x);
}

fn int_overflow() {
    /*
     * 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
     * 如果使用 checked_* 方法时发生溢出，则返回 None 值
     * 使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
     * 使用 saturating_* 方法，可以限定计算后的结果不超过目标类型的最大值或低于最小值
     */
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    let c: Option<u8> = a.checked_add(20);
    println!("b = {} c = {:?}", b, c);
}

fn float_trap() {
    // 浮点数比较大小
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);

    // Rust浮点数类型使用NaN(not a number)类型处理
    // 所有跟NaN交互的操作，都会返回一个NaN
    // 未定义的数学行为(负数取平方根)
    // let x = (-42.0_f32).sqrt();
    // assert_eq!(x, x);

    // 防御性编程
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定的数学行为")
    }
}

fn range_chapter() {
    // 序列仅限于数字和字符
    for i in 'A'..='s' {
        print!("{}", i);
    }
}

fn complex_num() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let res = a + b;
    println!("{} + {}i", res.re, res.im);
}

/*
 * 结构内存布局重排
 * 重排后的结果是b, c, a，按照最大成员内存对齐
 */
struct A {
    a: u8,
    b: u32,
    c: u16,
}

// 按照C的结构体内存布局，不允许内存重排
#[repr(C)]
struct B {
    a: u8,
    b: u32,
    c: u16,
}

//  枚举类型：以枚举类型成员最大的对齐值为准，不需要为每个枚举体值都对齐
enum D {
    One,
    Two,
}
enum E {
    N,
    H(u32),
    M(Box<u32>),
}

union F {
    u: u32,
    v: u64,
}

// trait 重载
struct G;
impl G {
    fn hello(&self) {
        println!("in G");
    }
}

trait Hello {
    fn hello(&self);
}

// 函数重载
impl Hello for G {
    fn hello(&self) {
        println!("from hello trait")
    }
}

// 泛型
fn foo<T>(x: T) -> T {
    return x;
}

fn data_type() {
    println!("A结构体占用内存大小:{:?}", std::mem::size_of::<A>());
    println!("B结构体占用内存大小:{:?}", std::mem::size_of::<B>());

    println!("D: {:?}", std::mem::size_of::<D>());
    println!("Box<u32>: {:?}", std::mem::size_of::<Box<u32>>());
    println!("E: {:?}", std::mem::size_of::<E>());
    println!("F: {:?}", std::mem::size_of::<F>());

    // copy和move隐式使用
    let a = 42;
    let b = a; // Rust对于简单类型使用copy，因为所占内存大小固定，在栈上复制,a可以再用
    println!("{:?}", a);
    let c = "hello".to_string();
    let d = c; // Rust使用的是move，hello的内存绑定到变量d上，之前c上的绑定就不能使用了，c无法再用
               // println!("c: {:?}", c);
               /* 重要：copy另一个使用就是不可变引用(&T)，但是注意可变引用(&mut T)是不可以Copy的 */
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}", x, y);

    let e = G;
    e.hello();

    // trait 类似于接口
    <G as Hello>::hello(&e);

    // Rust泛型速度快，不占用内存--- 静态分发
    assert_eq!(foo(1), 1);
    /*
     * Rust静态分发就是在编译阶段将foo(1)转为静态函数
     * fn foo_1(x:i32) -> i32 {
     *     return x;
     * }
     */
    assert_eq!(foo("hello"), "hello");
    /*
     * 同理foo("hello")将被转换如下
     * fn foo_2(x: &'static str) -> &'static str{
     *     return x;
     * }
     */
}

// 所有权
fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给
    // 调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域

    a_string // 返回 a_string 并移出给调用的函数
}

fn ownership() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x

    let s1 = gives_ownership(); // gives_ownership 将返回值
                                // 移给 s1

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到
                                       // takes_and_gives_back 中,
                                       // 它也将返回值移给 s3
    println!("s3 test {:?}", s3)
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作
  // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn first_word(s: &String) -> &str {
    &s[..1]
}

fn say_hello_str(s: &str) {
    println!("{}", s);
}

fn say_hello_string(s: String) {
    println!("{}", s);
}
fn string_unicode() {
    // 还能使用 \ 来连接多行字符串
    let long_string = "String literals
        can span multiple lines.
        The linebreak and indentation here \
         can be escaped too!";
    println!("{}", long_string);
    // rust中的字符是unicode占4个字节，字符串使用utf-8是可变字符，占(1-4)个字符
    // str是硬编码进可执行文件的，不能修改，string类型是可增长、可改变且具有所有权的UTF-8编码字符串
    let s = "中国人";
    // 在对字符串使用切片语法时需要格外小心，切片的索引必须落在字符之间的边界位置，
    // 也就是 UTF-8 字符的边界，例如中文在 UTF-8 中占用三个字节，下面的代码就会崩溃：
    // let a = &s[0..1];
    let a = &s[0..3];
    println!("{}", a);

    let mut b = String::from("hello world");

    let word = first_word(&b);

    /*
     * 回忆一下借用的规则：当我们已经有了可变借用时，就无法再拥有不可变的借用。因为 clear 需要清空改
     * 变 String，因此它需要一个可变借用（利用 VSCode 可以看到该方法的声明是 pub fn clear(&mut self) ，
     * 参数是对自身的可变借用 ）；而之后的 println! 又使用了不可变借用，也就是在 s.clear() 处可变借用与
     * 不可变借用试图同时生效，因此编译无法通过。
     */
    // b.clear(); // error!

    println!("the first word is: {}", word);

    // str和string相互转换
    let a_str = "hello str";
    let a_string = String::from("hello string");
    // string ==> &str
    say_hello_str(&a_string);
    say_hello_str(&a_string[..]);
    say_hello_str(a_string.as_str());
    // &str ==> string
    say_hello_string(a_str.to_string());

    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 连接字符串
    let d = format!("{} {}!", s1, s2);
    println!("{}", d);
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello,world!");
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);
    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
    // 遍历utf-8字符串的方法
    for c in "中国人".chars() {
        println!("{}", c);
    }
    // 获取utf-8子串的考虑使用utf8_slice
    // 切片
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2]; // 为什么此处需要引用

    let s2: &str = "hello, world";
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 可以作为类似于构造函数
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Person_1 {
    name: String,
    age: Box<u8>,
}

struct Point {
    x: i32,
    y: i32,
}

fn struct_practice() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // 添加mut，结构体字段才可以修改
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // 简化上述表达
    let user3 = User {
        email: String::from("example@qq.com"),
        ..user2
    };
    println!("{}", user1.active);
    // 下面这行会报错
    // 原因是user2获取了user1的string类型的所有权
    // println!("{:?}", user1);
    println!("{:?}", user3);
    println!("{:#?}", user3);
    dbg!(&user3);

    /*
     * 元组结构体特性和使用场景
     * 特性：元组结构体不需要字段名
     * 应用： 表示颜色或者3d(x,y,z)行业默认顺序的，并且不用字段名解释的
     */
    struct Color(i32, i32, i32);
    let _black = Color(0, 0, 0);

    /*
     * 单元结构体
     * 特性： 不需要字段，只关注行为
     */
    struct AlwaysEqual;

    let _subject = AlwaysEqual;

    // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
    // impl SomeTrait  for AlwaysEqual {}

    let person = Person_1 {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // 但是，这里 `age` 变量却是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
    let Person_1 { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    //println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);

    // 解构结构体
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

#[derive(Debug)]
struct PokerCard {
    suit: PokerSuit,
    value: u8,
}

#[derive(Debug)]
enum PokerCard_1 {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

fn enum_practice() {
    let c1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };
    let c2 = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 12,
    };
    println!("c1{:?}", c1);
    println!("c2{:?}", c2);

    let c1 = PokerCard_1::Spades(5);
    let c2 = PokerCard_1::Diamonds(13);
    println!("c1{:?}", c1);
    println!("c2{:?}", c2);
}

fn arrary_practice() {
    // 输入多个数组元素非基础类型
    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));

    println!("{:#?}", array);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 索引访问
    println!("index=1: {}", a[1]);

    // 数组切片
    let slice: &[i32] = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn control() {
    // if语句块是表达式，也是没有C中的?关键字的原因
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    for i in 1..=5 {
        println!("{}", i);
    }

    // for循环遍历集合往往使用引用，除非你不想使用该集合，因为不使用引用会导致集合的所有权转移到for语句块
    /*
     *      使用方法                等价使用方法                                           所有权
     * for item in collection       for item in IntoIterator::into_iter(collection)         转移所有权
     * for item in &collection      for item in collection.iter()                           不可变借用
     * for item in &mut collection  for item in collection.iter_mut()                       可变借用
     */
    // 获取元素索引
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }

    // 对比两种循环方式的优劣
    /*
     * 性能：第一种使用方式中 collection[index] 的索引访问，会因为边界检查(Bounds Checking)导
     * 致运行时的性能损耗 —— Rust 会检查并确认 index 是否落在集合内，但是第二种直接迭代的方式
     * 就不会触发这种检查，因为编译器会在编译时就完成分析并证明这种访问是合法的
     *
     * 安全：第一种方式里对 collection 的索引访问是非连续的，存在一定可能性在两次访问之间，
     * collection 发生了变化，导致脏数据产生。而第二种直接迭代的方式是连续访问，因此不存在这种
     * 风险( 由于所有权限制，在访问过程中，数据并不会发生变化)。
     */
    // 第一种
    let collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() {
        let item = collection[i];
        // ...
    }

    // 第二种
    for item in collection {}

    // continue 跳出当前循环
    for i in 1..4 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }

    // break 跳出整个循环
    for i in 1..4 {
        if i == 2 {
            break;
        }
        println!("{}", i);
    }

    // 条件循环
    let mut n = 0;

    while n <= 5 {
        println!("{}!", n);

        n = n + 1;
    }

    println!("我出来了！");

    // 无限循环
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break 带返回值返回
        }
    };

    println!("The result is {}", result);

    // 当有多层循环时，你可以使用 continue 或 break 来控制外层的循环。要实现这一点，
    // 外部的循环必须拥有一个标签 'label, 然后在 break 或 continue 时指定该标签
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // 这只会跳出 inner1 循环
                break 'inner1; // 这里使用 `break` 也是一样的
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }

            continue 'outer;
        }
    }

    assert!(count == 30)
}

enum Direction {
    East,
    West,
    North,
    South,
}

enum IpAddr {
    Ipv4,
    Ipv6,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 25美分硬币
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn show_message(msg: Message) {
    match msg {
        Message::Move { x: a, y: b } => {
            // 这里匹配 Message::Move重点是非元组的如何匹配
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        }
        Message::ChangeColor(r, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        _ => println!("no data in these variants"),
    }
}

fn match_practice() -> u8 {
    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg)
    }

    let dire = Direction::South;
    let n = match dire {
        Direction::East => 3,
        Direction::North | Direction::South => {
            println!("South or North");
            1 // 可以有返回值
        }
        _ => 2,
    };

    let ip1 = IpAddr::Ipv6;
    // 模式匹配赋值
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);

    // 模式绑定
    let coin = Coin::Quarter(UsState::Alabama);
    let coin_size = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    };

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }

    // 匹配范围
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // 匹配守卫
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // matches!宏
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));

    // if let 匹配
    let v = Some(3u8);
    if let Some(3) = v {
        println!("three");
    }

    let age = Some(30);
    println!("在匹配前，age是{:?}", age);
    match age {
        Some(age) => println!("匹配出来的age是{}", age), // 存在变量遮蔽，遮蔽会在离开语句块后消失
        _ => (),
    }
    println!("在匹配后，age是{:?}", age);

    let dire = Direction::South;
    match dire {
        Direction::East => 3,
        Direction::North | Direction::South => {
            println!("South or North");
            1 // 可以作为函数返回值
        }
        _ => 2,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn deconstruct_option() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn scenes() {
    // while 和 let 条件循环
    // Vec是动态数组
    let mut stack = Vec::new();

    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // stack.pop从数组尾部弹出元素
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

// 队列
/*
 * 初始化
 * 入队
 * 出队
 * 求取队列长度
 */

#[derive(Debug)]
struct MyQueue<T> {
    qdata: Vec<T>,
}

impl<T> MyQueue<T> {
    fn new() -> Self {
        MyQueue { qdata: Vec::new() }
    }

    fn enqueue(&mut self, item: T) {
        self.qdata.push(item);
    }

    fn dequeue(&mut self) -> Option<T> {
        let l = self.qdata.len();

        if l > 0 {
            let v = self.qdata.remove(0);
            Some(v)
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        self.qdata.len()
    }
}

fn queue_practice() {
    let mut q = MyQueue::new();
    q.enqueue(1);
    q.enqueue(2);
    println!("{:?}", q);
    println!("size = {}", q.size());

    q.dequeue();
    println!("{:?}", q);
    println!("size = {}", q.size());

    q.dequeue();
    println!("{:?}", q);
    println!("size = {}", q.size());

    q.dequeue();
    println!("{:?}", q);
    println!("size = {}", q.size());
}

// 循环队列
/*
 * MyCircularQueue(k): 构造器，设置队列长度为 k 。
 * Front: 从队首获取元素。如果队列为空，返回 -1 。
 * Rear: 获取队尾元素。如果队列为空，返回 -1 。
 * enQueue(value): 向循环队列插入一个元素。如果成功插入则返回真。
 * deQueue(): 从循环队列中删除一个元素。如果成功删除则返回真。
 * isEmpty(): 检查循环队列是否为空。
 * isFull(): 检查循环队列是否已满。
 */

struct MyCircularQueue {
    v: Vec<i32>,
    head: i32, // -1表示没有任何数据
    tail: i32, // -1表示没有任何数据，其他情况指向下一个可用下标
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        MyCircularQueue {
            v: vec![0; k as usize],
            head: -1,
            tail: -1,
        }
    }

    /* 插入节点 */
    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        if self.is_empty() {
            self.tail = 1;
            self.head = 0;
            self.v[0] = value;
        } else {
            self.v[self.tail as usize] = value;
            self.tail += 1;
            if self.tail >= self.v.len() as i32 {
                self.tail = 0; // 队列已满
            }
        }
        true
    }

    /* 删除节点 */
    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.head += 1;
        if self.head >= self.v.len() as i32 {
            self.head = 0;
        }
        if self.tail == self.head {
            self.tail = -1;
            self.head = -1;
        }
        true
    }

    // 获取队列最后一个节点
    fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let mut l = self.tail - 1;
        if l < 0 {
            l = (self.v.len() - 1) as i32;
        }
        self.v[l as usize]
    }

    /* 检查队列是否为空 */
    fn is_empty(&self) -> bool {
        return self.head == -1 && self.tail == -1; // 设定head tail为-1
    }
    /* 检查队列是否已满 */
    fn is_full(&self) -> bool {
        return self.head == self.tail && self.tail != -1; // ？？？
    }
}

fn circular_queue_practice() {
    let mut obj = MyCircularQueue::new(3);
    assert_eq!(true, obj.en_queue(1));
    assert_eq!(true, obj.en_queue(2));
    assert_eq!(true, obj.en_queue(3));
    assert_eq!(false, obj.en_queue(4));
    assert_eq!(3, obj.rear());
    assert_eq!(true, obj.is_full());
    assert_eq!(true, obj.de_queue());
    assert_eq!(true, obj.en_queue(4));
    assert_eq!(4, obj.rear());
}

fn foo_1(_: i32, y: i32) {
    // 忽略参数
    println!("This code only uses the y parameter: {}", y);
}

// 结构体方法前面有，在此忽略
// 为枚举实现方法
#[allow(unused)]
enum Messagea {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Messagea {
    fn call(&self) {
        // 在这里定义方法体
    }
}

// 结构体泛型
struct Point_1<T> {
    x: T,
    y: T,
}

// 泛型函数
impl<T> Point_1<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point_2<T, U> {
    x: T,
    y: U,
}

// 泛型结构体和泛型函数结合
impl<T, U> Point_2<T, U> {
    fn mixup<V, W>(self, other: Point_2<V, W>) -> Point_2<T, W> {
        Point_2 {
            x: self.x,
            y: other.y,
        }
    }
}

// const泛型、值泛型
// T: std::fmt::Debug是为了T可以使用{:?}格式化输出
fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    // 可以处理任何数组
    println!("{:?}", arr);
}

// N 就是 const 泛型，定义的语法是 const N: usize，表示 const 泛型 N ，它基于的值类型是 usize
fn display_array_1<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

fn generics() {
    let integer = Point_1 { x: 5, y: 10 };
    let float = Point_1 { x: 1.0, y: 4.0 };
    let p = Point_1 { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p1 = Point_2 { x: 5, y: 10.4 };
    let p2 = Point_2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let arr: [i32; 3] = [1, 2, 3];
    display_array(&arr);

    let arr: [i32; 2] = [1, 2];
    display_array(&arr);

    let arr: [i32; 3] = [1, 2, 3];
    display_array_1(arr);

    let arr: [i32; 2] = [1, 2];
    display_array_1(arr);
}

#[derive(Debug)]
enum IpAddr_1 {
    V4(String),
    V6(String),
}

fn show_addr(ip: IpAddr_1) {
    println!("{:?}", ip);
}

trait IpAddr_2 {
    fn display(&self);
}

struct V4(String);
impl IpAddr_2 for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}
struct V6(String);
impl IpAddr_2 for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}

#[derive(Debug)]
struct Person_2 {
    name: String,
    age: u32,
}

impl Person_2 {
    fn new(name: String, age: u32) -> Person_2 {
        Person_2 { name, age }
    }
}

// 排序需要我们实现Ord特性
// 实现 Ord 需要我们实现 Ord、Eq、PartialEq、PartialOrd 这些属性

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Person_3 {
    name: String,
    age: u32,
}

impl Person_3 {
    fn new(name: String, age: u32) -> Person_3 {
        Person_3 { name, age }
    }
}
fn vector() {
    // get相比于索引获取值，会进行数组越界访问检查
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("第三个元素是 {}", third);

    match v.get(2) {
        Some(third) => println!("第三个元素是 {third}"),
        None => println!("去你的第三个元素，根本没有！"),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6);   // error first进行了数组的不可变借用，这里又进行了可变借用，
    /*
     * 原因在于：数组的大小是可变的，当旧数组的大小不够用时，Rust 会重新分配一块更大的内存空间，然后把旧数组拷贝过来。
     * 这种情况下，之前的引用显然会指向一块无效的内存，这非常 rusty —— 对用户进行严格的教育。
     */

    println!("The first element is: {first}");

    // 数组存储不同的值枚举类型实现
    let v = vec![
        IpAddr_1::V4("127.0.0.1".to_string()),
        IpAddr_1::V6("::1".to_string()),
    ];

    for ip in v {
        show_addr(ip)
    }

    let v: Vec<Box<dyn IpAddr_2>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }

    let mut v = Vec::with_capacity(10);
    v.extend([1, 2, 3]); // 附加数据到 v
    println!("Vector 长度是: {}, 容量是: {}", v.len(), v.capacity());

    v.reserve(100); // 调整 v 的容量，至少要有 100 的容量
    println!(
        "Vector（reserve） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );

    v.shrink_to_fit(); // 释放剩余的容量，一般情况下，不会主动去释放容量
    println!(
        "Vector（shrink_to_fit） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );

    // 结构体数组排序
    let mut people = vec![
        Person_2::new("Zoe".to_string(), 25),
        Person_2::new("Al".to_string(), 60),
        Person_2::new("John".to_string(), 1),
    ];
    // 定义一个按照年龄倒序排序的对比函数
    people.sort_unstable_by(|a, b| b.age.cmp(&a.age));

    println!("{:?}", people);

    let mut people = vec![
        Person_3::new("Zoe".to_string(), 25),
        Person_3::new("Al".to_string(), 60),
        Person_3::new("Al".to_string(), 30),
        Person_3::new("John".to_string(), 1),
        Person_3::new("John".to_string(), 25),
    ];

    people.sort_unstable();

    println!("{:?}", people);
}

fn hash_map() {
    use std::collections::HashMap;

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let teams_map: HashMap<_, _> = teams_list.into_iter().collect();
    //使用into_iter将vec转为迭代器，再使用collect函数收集成集合类型，支持多种集合类型,_代表类型由collect推测

    println!("{:?}", teams_map);

    // hashmap查找
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);

    // 循环遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow".to_string()).or_insert(5);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    // 根据空格来切分字符串(英文单词都是通过空格切分)
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // 若存在，则对已有的值更新
        *count += 1;
    }

    println!("{:?}", map);

    // rust hash函数的安全性很高，函数SipHash在中等大小的key上性能不错,
    // 但对于小型key（整数）或者大型key（字符串）来说，性能不够，
    // 如果需要机制性能，可以考虑ahash库
    // use std::hash::BuildHasherDefault;
    // use std::collections::HashMap;
    // // 引入第三方的哈希函数
    // use twox_hash::XxHash64;

    // // 指定HashMap使用第三方的哈希函数XxHash64
    // let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
    // hash.insert(42, "the answer");
    // assert_eq!(hash.get(&42), Some(&"the answer"));
}
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("author: {}", self.username)
    }
}

// 特征约束
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// 函数两个参数是不同的类型，只要这两个类型都实现了Summary特性即可
pub fn notify_3(item1: &impl Summary, item2: &impl Summary) {}
// 强制两个参数是同一种类型
pub fn notify_4<T: Summary>(item1: &T, item2: &T) {}
// 多重特征约束
pub fn notify_1(item: &(impl Summary + Display)) {}
pub fn notify_2<T: Summary + Display>(item: &T) {}
// where 约束
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {}

//函数返回 impl trait,但是该函数只能返回一种类型
fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from("m1 maxx太厉害"),
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            // 不是所以的类型都有比较大小的实现，所以PartialOrd
            largest = item; // 所以对于实现Copy，才能深拷贝，不然所有权转移，list使用会出现问题
        }
    }

    largest
}

use std::ops::Add;

// 为Point结构体派生Debug特征，用于格式化输出
#[derive(Debug)]
struct Point_3<T: Add<T, Output = T>> {
    //限制类型T必须实现了Add特征，否则无法进行+操作。
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point_3<T> {
    type Output = Point_3<T>;

    fn add(self, p: Point_3<T>) -> Point_3<T> {
        Point_3 {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

fn t() {
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "好像微博没有tweet好用".to_string(),
    };
    println!("{}", weibo.summarize());
    notify(&weibo);
    let summary = returns_summarizable();

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p1 = Point_3 {
        x: 1.1f32,
        y: 1.1f32,
    };
    let p2 = Point_3 {
        x: 2.1f32,
        y: 2.1f32,
    };
    println!("{:?}", add(p1, p2));
}

pub trait Draw {
    fn draw(&self);
}
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 绘制按钮的代码
        println!("{:?}", self.label);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制SelectBox的代码
        println!("{:?}", self.options);
    }
}

// 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
fn draw1(x: Box<dyn Draw>) {
    // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
    x.draw();
}

fn draw2(x: &dyn Draw) {
    x.draw();
}

// 这种写法有点是components可以存储实现Draw的所有类型
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
// 如果只存储Draw的其中一个类型，可以使用泛型搭配特征约束，代码更清晰，性能更好
pub struct Screen_1<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen_1<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
// struct Counter {
//     x: i32,
//     y: i32,
// }
// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

// impl Iterator for Counter {
//     type Item = i32;

//     fn next(&mut self) -> Option<Self::Item> {
//         //
//     }
// }

// 泛型
// trait Container<A,B> {
//     fn contains(&self,a: A,b: B) -> bool;
// }

// fn difference<A,B,C>(container: &C) -> i32
//   where
//     C : Container<A,B> {...}
// trait
// trait Container{
//     type A;
//     type B;
//     fn contains(&self, a: &Self::A, b: &Self::B) -> bool;
// }

// fn difference<C: Container>(container: &C) {}

// 调用同名方法
// 优先调用类型上的方法
// 调用特征上的方案
trait Pilot {
    fn fly(&self);
    fn baby_name() -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }

    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// 特征定义中的特征约束
trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// 在外部类型上实现外部特征（newtype）---- 打破孤儿规则
// 为Vec实现Display
struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}

// 特征对象限制(特征安全)，主要原因不知的类型了
// 方法的返回值不能是Self
// 方法没有任何泛型参数
fn t_obj() {
    let button = Button {
        width: 1,
        height: 2,
        label: String::from("I'am Button."),
    };
    let selectbox = SelectBox {
        width: 1,
        height: 2,
        options: vec![String::from("I'am SelectBox."); 3],
    };

    // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
    draw2(&button);
    draw2(&selectbox);
    // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
    draw1(Box::new(button));
    // 基于 y 的值创建一个 Box<u8> 类型的智能指针
    draw1(Box::new(selectbox));

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // 优先调用类型上的方法
    let person = Human;
    person.fly();
    // 调用特征上的方法
    Pilot::fly(&person);
    // 当特征上没有self参数如何调用-----完全限定语法
    // <Human as Pilot>相当于告诉编译器调用为Human实现Pilot的baby_name函数
    println!("A baby pilot is called a {}", <Human as Pilot>::baby_name());

    // 打破孤儿规则 newtype
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    print!("w = {}", w);
}

/*
 * 这个示例会运行错误，原因就是生命周期
 * 因为编译器想通过返回值确保函数调用后的引用生命周期分析
 */
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 结构体生命周期，场景：当结构体使用引用类型时，就需要使用使用生命周期，如果不使用引用（比如String）会发生所有权转移
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 方法生命周期
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// 生命周期约束
// 'a是生命周期比'b长的约束
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part_1<'b>(&'a self, announcement: &'b str) -> &'b str
    where
        'a: 'b,
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 静态生命周期
// 'static 拥有该生命周期的引用可以和整个程序活得一样久

fn lifetime() {
    // 生命周期标注并不会改变任何引用的实际作用域--鲁迅
    // 生命周期的意义就是告诉编译器多个引用之间的关系
    // 生命周期语法用来将函数的多个引用参数和返回值的作用域关联到一起，一旦关联到一起后，Rust 就拥有充分的信息来确保我们的操作是内存安全的。
    let x = String::from("Hello world.");
    let y = "hello";
    println!("longest: {}", longest(x.as_str(), y));

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // println!("The longest string is {}", result); //因为longest的返回值的生命周期是string2的生命周期，因为是按照最小生命周期去计算的
    /*
     * 在上例中，string1 的作用域直到 main 函数的结束，而 string2 的作用域到内部花括号的结束 }，那
     * 么根据之前的理论，'a 是两者中作用域较小的那个，也就是 'a 的生命周期等于 string2 的生命周期，
     * 同理，由于函数返回的生命周期也是 'a，可以得出函数返回的生命周期也等于 string2 的生命周期。
     *
     * 现在来验证下上面的结论：result 的生命周期等于参数中生命周期最小的，因此要等于 string2 的生命
     * 周期，也就是说，result 要活得和 string2 一样久，观察下代码的实现，可以发现这个结论是正确的！
     */

    // 结构体生命周期
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    /*
     * 生命周期消除的三条规则
     * 1.每一个引用参数都会获得独自的生命周期
     * 2.若只有一个输入生命周期（函数参数中只有一个引用类型），那么该生命周期会被赋给所有的输出生命周期
     * 3.若存在多个输入生命周期，且其中一个是&self或&mut self，则&self的生命周期被赋给所有的输出生命周期
     */
}
fn error() -> Result<String, std::io::Error> {
    // 可恢复错误处理
    use std::fs::File;
    use std::io::ErrorKind;
    let f = File::open("hello.txt")?;
    Ok("string".to_string())
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match  File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         }
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     }
    // };
}

fn base_parctice() {
    hellworld();
    var_shadowing();
    int_overflow();
    float_trap();
    range_chapter();
    complex_num();

    use std::ops::{Range, RangeInclusive};
    // for i in Range{start: 1, end:5} {
    //     print!("{}",i);
    // }
    let test = Range { start: 1, end: 5 };
    print!("{:?}", test);

    for i in RangeInclusive::new(1, 5) {
        print!("{}", i);
    }

    data_type();
    ownership();
    string_unicode();

    struct_practice();
    enum_practice();
    arrary_practice();
    control();
    match_practice();
    deconstruct_option();
    scenes();
    queue_practice();
    circular_queue_practice();
    let v = vec![1; 5 as usize];
    println!("{:?}", v);
    // https://stevenbai.top/rust-leetcode/2019-06-07/
    // https://www.yiibai.com/data_structure/circular-queue.html
    // https://learnku.com/articles/43145
    foo_1(3, 4);
    let m = Messagea::Write(String::from("hello"));
    m.call();
    generics();
    vector();
    hash_map();
    t();
    t_obj();
    lifetime();
    error();
}

#[derive(Debug)]
struct Foo3;

impl Foo3 {
    fn mutate_and_share(&mut self) -> &Self {
        // 参数和返回值的生命周期一样
        &*self
    }
    fn share(&self) {}
}

// 无界生命周期
/**
 * x参数传入的时候没有生命周期，凭空产生，实际上比'static要强大
 */
fn f<'a, T>(x: *const T) -> &'a T {
    unsafe { &*x }
}

fn advanced_lifetime() {
    // 例1
    let mut foo = Foo3;
    let loan = foo.mutate_and_share();
    // foo.share();
    // &mut self 借用的生命周期和 loan 的生命周期相同，将持续到 println 结束。
    // 而在此期间 foo.share() 又进行了一次不可变 &foo 借用，违背了可变借用与不可变借用不能同时存在的规则，最终导致了编译错误。
    println!("{:?}", loan);

    // 例2
    // use std::collections::HashMap;
    // use std::hash::Hash;
    // fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
    // where
    //     K: Clone + Eq + Hash,
    //     V: Default,
    // {
    //     match map.get_mut(&key) {
    //         Some(value) => value,
    //         None => {
    //             map.insert(key.clone(), V::default());
    //             map.get_mut(&key).unwrap()
    //         }
    //     }
    // }
    /*
     * 分析代码可知在 match map.get_mut(&key) 方法调用完成后，对 map 的可变借用就可以结束了。
     * 但从报错看来，编译器不太聪明，它认为该借用会持续到整个 match 语句块的结束，这便造成了后续借用的失败。
     */
}

// `block_on`会阻塞当前线程直到指定的`Future`执行完成，这种阻塞当前线程以等待任务完成的方式较为简单、粗暴，
// 好在其它运行时的执行器(executor)会提供更加复杂的行为，例如将多个`future`调度到同一个线程上执行
use futures::executor::block_on;

async fn hello_cat() {
    println!("hello, kity!");
}

// .await 不会阻塞hello_world的线程，而是异步的等待hello_word的完成，
// 在一个async fn 函数中去调用另一个async fn并等待完成后再执行后续的代码
async fn hello_world() {
    hello_cat().await;
    println!("hello, world");
}

// 载歌载舞练习
struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "曲婉婷".to_string(),
        name: String::from("《我的歌声里》"),
    }
}

async fn sing_song(song: Song) {
    println!(
        "给大家献上一首{}的{} ~ {}",
        song.author, song.name, "你存在我深深的脑海～ ～"
    );
}

async fn dance() {
    println!("唱到情深处，身体不由自主的动了起来～ ～")
}

async fn learn_and_sing() {
    // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，完全可以去执行跳舞的任务
    let song = learn_song().await;

    // 唱歌必须要在学歌之后
    sing_song(song).await;
}

async fn asyc_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!`可以并发的处理和等待多个`Future`，若`learn_and_sing Future`被阻塞，那`dance Future`可以拿过线程的所有权继续执行。若`dance`也变成阻塞状态，那`learn_and_sing`又可以再次拿回线程所有权，继续执行。
    // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
    futures::join!(f1, f2);
}

fn advanced_futures() {
    let future = hello_world(); // 返回一个Future, 因此不会打印任何输出
    block_on(future); // 执行`Future`并等待其运行完成，此时"hello, world!"会被打印输出

    block_on(asyc_main());
}

// 结构体闭包
// impl<T> Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     fn new(query: T) -> Cacher<T> {
//         Cacher {
//             query,
//             value: None,
//         }
//     }

//     // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.query)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }

// fn fn_once<F>(func: F)
// where
//     F: FnOnce(usize) -> bool,
// {
//     println!("{}", func(3));
//     println!("{}", func(4));    // 所有权转移
// }

fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
}

fn closure() {
    /*
     * 闭包是一种匿名函数，它可以赋值给变量也可以作为参数传递给其它函数，
     * 不同于函数的是，它允许捕获调用者作用域中的值
     */
    let x = 1;
    let sum = |y| x + y;
    assert_eq!(3, sum(2));
    let add_sum_one = |a: i32, b: i32| a + b + 1;
    assert_eq!(8, add_sum_one(2, 5));

    // 当编译器推导出一种类型后，它就会一直使用该类型
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);

    let equal_to_x = |z| z == x;

    let y = 1;

    assert!(equal_to_x(y));

    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len());

    use std::thread;
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        // move强制闭包取得捕获变量的所有权，
        // 这种用法通常用于闭包的生命周期大于捕获变量的生命周期时，例如将闭包返回或移入其他线程。
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();

    // 可变借用闭包，需要闭包变量是mut
    let mut s = String::new();

    let mut update_string = |str| s.push_str(str);
    update_string("hello");

    println!("{:?}", s);

    //
    let mut s = String::new();

    // 只要闭包捕获的类型都实现了Copy特征的话，这个闭包就会默认实现Copy特征
    let update_string = |str| s.push_str(str);

    exec(update_string);
    // 如果拿到的是s的所有权或可变引用，都是不能Copy的
    // exec(update_string);
}

// 实现Iterator特征
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn Iterator_parctice() {
    // 迭代器就是实现了IntoIterator的next方法，next方法返回值是Option类型
    // pub trait Iterator {
    //     type Item;

    //     fn next(&mut self) -> Option<Self::Item>;

    //     // 省略其余有默认实现的方法
    // }
    let arr = [1, 2, 3];
    let mut arr_iter = arr.into_iter();

    assert_eq!(arr_iter.next(), Some(1));
    assert_eq!(arr_iter.next(), Some(2));
    assert_eq!(arr_iter.next(), Some(3));
    assert_eq!(arr_iter.next(), None); // 迭代器是消耗性的，每次消耗它一个元素，最终迭代器中将没有任何元素，只能返回 None

    // into_iter 会夺走所有权
    // iter 是借用
    // iter_mut 是可变借用

    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
    let v = vec![1u64, 2, 3, 4, 5, 6];
    let val = v
        .iter()
        .enumerate()
        // 每两个元素剔除一个
        // [1, 3, 5]
        .filter(|&(idx, _)| idx % 2 == 0)
        .map(|(_, val)| val)
        // 累加 1+3+5 = 9
        .fold(0u64, |sum, acm| sum + acm);

    println!("{}", val);
}

fn foo_3() -> i32 {
    0
}

// 生命周期
// 将 'b 生命周期延长至 'static 生命周期
unsafe fn extend_lifetime<'b>(r: R1<'b>) -> R1<'static> {
    std::mem::transmute::<R1<'b>, R1<'static>>(r)
}

// 将 'static 生命周期缩短至 'c 生命周期
unsafe fn shorten_invariant_lifetime<'b, 'c>(r: &'b mut R1<'static>) -> &'b mut R1<'c> {
    std::mem::transmute::<&'b mut R1<'static>, &'b mut R1<'c>>(r)
}

struct R1<'a>(&'a i32);

// newtype 就是元组结构体
// 孤儿原则的限制，Vev(类型)和Display(trait)没有一个在作用域就不能实现Display
// 如果使用newtype就可以绕过孤儿原则，因为newtype在作用域，属于新类型
// 此时Wrapper_1不能使用Vec的所有方法，可以通过Wrapper_1.0.method()的方式使用
struct Wrapper_1(Vec<String>);

impl fmt::Display for Wrapper_1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// 使用第三方库完成整数转枚举类型
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
enum MyEnum {
    A = 1,
    B,
    C,
}

//  Tryfrom
use std::convert::TryFrom;

enum MyEnum1 {
    A = 1,
    B,
    C,
}

impl TryFrom<i32> for MyEnum1 {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == MyEnum1::A as i32 => Ok(MyEnum1::A),
            x if x == MyEnum1::B as i32 => Ok(MyEnum1::B),
            x if x == MyEnum1::C as i32 => Ok(MyEnum1::C),
            _ => Err(()),
        }
    }
}

// 使用TryFrom+宏
#[macro_export]
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<i32> for $name {
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

back_to_enum! {
    enum MyEnum2 {
        A = 1,
        B,
        C,
    }
}

fn type_parctice() {
    let a: i32 = 10;
    let b: u16 = 1000;

    // 通常表示范围小的转表示范围大的，反之则会出现意外
    if a < (b as i32) {
        println!("The is less than one hundred");
    }

    // 内存地址转指针
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize; // 将p1内存地址转为一个整数
    let second_address = first_address + 4; // 4==std::mem::sizeof::<i32>(), i32类型占用4个字节
    let p2 = second_address as *mut i32;
    unsafe {
        *p2 += 1;
    }
    assert_eq!(values[1], 3);

    // 类型转换不具有传递性
    //  e as U1 as U2 是合法的，也不能说明 e as U2 是合法的（e 不能直接转换成 U2）

    let b: i16 = 1500;

    let b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };

    // 裸指针转函数在指针

    let pointer = foo_3 as *const ();
    let function = unsafe {
        // 将裸指针转换为函数指针
        std::mem::transmute::<*const (), fn() -> i32>(pointer)
    };
    assert_eq!(function(), 0);

    let w = Wrapper_1(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    // 类型别名
    type Meters = u32;

    // 整数转枚举类型支持不友好，实现的方法
    // 第三方库实现

    let x = 2;

    match FromPrimitive::from_i32(x) {
        Some(MyEnum::A) => println!("Got A"),
        Some(MyEnum::B) => println!("Got B"),
        Some(MyEnum::C) => println!("Got C"),
        None => println!("Couldn't convert {}", x),
    }

    // TryFrom实现
    let x = MyEnum1::C as i32;

    match x.try_into() {
        Ok(MyEnum1::A) => println!("a"),
        Ok(MyEnum1::B) => println!("b"),
        Ok(MyEnum1::C) => println!("c"),
        Err(_) => eprintln!("unknow number"),
    }

    // TryFrom+宏
    let x = MyEnum2::C as i32;

    match x.try_into() {
        Ok(MyEnum2::A) => println!("a"),

        Ok(MyEnum2::B) => println!("b"),
        Ok(MyEnum2::C) => println!("c"),
        Err(_) => eprintln!("unknow number"),
    }
}

trait Draw1 {
    fn draw(&self);
}

struct Button1 {
    id: u32,
}
impl Draw1 for Button1 {
    fn draw(&self) {
        println!("这是屏幕上第{}号按钮", self.id)
    }
}

struct Select {
    id: u32,
}

impl Draw1 for Select {
    fn draw(&self) {
        println!("这个选择框贼难用{}", self.id)
    }
}

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn auto_ptr() {
    // 使用Box<T>将数据存储在堆上
    let a = Box::new(3); // Box实现了Drop特征，当离开作用域就会自动被释放
    println!("a = {}", a); // a = 3     自动调用Deref解引用

    // 下面一行代码将报错
    // let b = a + 1; // cannot add `{integer}` to `Box<{integer}>`
    let b = *a + 1; // *显式解引用

    // 避免栈上的数据的拷贝
    // 在栈上创建一个长度为1000的数组
    let arr = [0; 1000];
    // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
    let arr1 = arr;

    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
    let arr = Box::new([0; 1000]);
    // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
    // println!("{:?}", arr.len());

    // 将动态大小类型变为Sized固定大小类型
    // enum List {
    //     Cons(i32, List), // List无限嵌套，编译器不知道这个枚举类型到底多大
    //     Nil,
    // }
    // 解决方法
    // enum List {
    //     Cons(i32, Box<List>),
    //     Nil,
    // }

    // 特征对象
    let elems: Vec<Box<dyn Draw1>> = vec![Box::new(Button1 { id: 1 }), Box::new(Select { id: 2 })];

    for e in elems {
        e.draw()
    }

    let a = Rc::new(String::from("hello, world"));
    let b = Rc::clone(&a); // 复制了智能指针并增加了引用计数，并没有克隆底层数据

    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));

    let a = Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&c));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // 多线程不能使用Rc，需要使用Arc(atomic Rc)，实现了原子化，实现了Send特征
    let s = Arc::new(String::from("多线程漫游者"));
    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || println!("{}", s));
    }

    // Cell 和 RefCell 在功能上没有区别，区别在于 Cell<T> 适用于 T 实现 Copy 的情况
    let c = Cell::new("asdf");
    let one = c.get();
    c.set("qwer");
    let two = c.get();
    println!("{},{}", one, two);

    /*
     * Rust 规则                                        智能指针带来的额外规则
     * 1. 一个数据只有有一个所有者                          Rc/Arc让一个数据可以拥有多个所有者
     * 2. 要么多个不可变借用，要么一个可变借用                RefCell实现编译期可变、不可变引用共存(RefCell 只是将借用规则从编译期推迟到程序运行期，并不能帮你绕过这个规则)
     * 3. 违背规则导致编译错误                              违背规则导致运行时panic
     */

    // Rc实现可以有多个所有者， RefCell实现可变性
    let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

    let s1 = s.clone();
    let s2 = s.clone();
    // let mut s2 = s.borrow_mut();
    s2.borrow_mut().push_str(", oh yeah!");

    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}

use std::sync::Barrier;
use std::time::Duration;

enum Fruit {
    Apple(u8),
    Orange(String),
}

use std::ops::Sub;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::JoinHandle;
use std::time::Instant;

const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;

static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

// 内部可变性
struct Counter1 {
    count: u64,
}

// 裸指针实现Send
#[derive(Debug)]
struct MyBox(*mut u8);
unsafe impl Send for MyBox {}

// 裸指针实现Sync
#[derive(Debug)]
struct MyBox1(*const u8);
unsafe impl Send for MyBox1 {}
unsafe impl Sync for MyBox1 {}

fn multiple_thread() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // move 获取v所有权
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    // 下面代码会报错borrow of moved value: `v`
    // println!("{:?}",v);

    // 线程屏障 Barrier
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait(); // 等待所有线程执行到这里时继续执行
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 线程局部存储
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1)); //  使用 static 声明为生命周期为 'static 的静态变量。

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    // 每个线程开始时都会拿到线程局部变量的FOO的初始值
    let t = thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });

    // 等待线程完成
    t.join().unwrap();

    // 尽管子线程中修改为了3，我们在这里依然拥有main线程中的局部值：2
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });

    // 第三方库
    use thread_local::ThreadLocal;
    let tls = Arc::new(ThreadLocal::new());
    let mut v = vec![];
    // 创建多个线程
    for _ in 0..5 {
        let tls2 = tls.clone();
        let handle = thread::spawn(move || {
            // 将计数器加1
            // 请注意，由于线程 ID 在线程退出时会被回收，因此一个线程有可能回收另一个线程的对象
            // 这只能在线程退出后发生，因此不会导致任何竞争条件
            let cell = tls2.get_or(|| Cell::new(0));
            cell.set(cell.get() + 1);
        });
        v.push(handle);
    }

    for handle in v {
        handle.join().unwrap();
    }

    // 一旦所有子线程结束，收集它们的线程局部变量中的计数器值，然后进行求和
    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| {
        // 打印每个线程局部变量中的计数器值，发现不一定有5个线程，
        // 因为一些线程已退出，并且其他线程会回收退出线程的对象
        println!("x: {}, y: {}", x, y.get());
        x + y.get()
    });

    // 和为5
    assert_eq!(total, 5);

    // 条件控制线程的挂起和执行
    use std::sync::{Condvar, Mutex};
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("started changed");

    // 只被调用一次的函数
    use std::sync::Once;
    static mut VAL: usize = 0;
    static INIT: Once = Once::new();
    let handle1 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 1;
        });
    });

    let handle2 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 2;
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL });

    // 消息传递
    // 单发送者，单接收者
    use std::sync::mpsc;

    // 创建一个消息通道，返回一个元组：（发送者，接收者）
    let (tx, rx) = mpsc::channel();

    // 创建线程，并发送消息
    thread::spawn(move || {
        // 发送一个数字1，send方法返回Result<T,E>，通过unwrap进行快速错误处理
        tx.send(1).unwrap();

        // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，
        // 那么Option<i32>类型将产生不匹配错误
        // tx.send(Some(1)).unwrap();
    });

    // 在主线程中接收子线程发送的消息并输出
    println!("receive {}", rx.recv().unwrap()); // 阻塞线程直到读取到值或者通道被关闭

    // 不阻塞方法

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    println!("receive {:?}", rx.try_recv());

    // 传输具有所有权的数据
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let s = String::from("我，飞走咯！");
        tx.send(s).unwrap();
        // println!("val is {}", s);    // s所有权已经发生转移
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // for循环接收
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // 使用多发送者
    // 由于子线程拿走发送者的所有权，所以需要克隆
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        tx.send(String::from("hi from raw tx")).unwrap();
    });

    thread::spawn(move || {
        tx1.send(String::from("hi from cloned tx")).unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // 同步通道

    let (tx, rx) = mpsc::sync_channel(0);

    let hande = thread::spawn(move || {
        println!("发生之前");
        tx.send(1).unwrap();
        println!("发生之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    hande.join().unwrap();
    // 使用异步消息虽然能非常高效且不会造成发送线程的阻塞，但是存在消息未及时消费，最终内存过大的问题。
    // 在实际项目中，可以考虑使用一个带缓冲值的同步通道来避免这种风险。

    // 消息通道传输多种类型的数据
    use std::sync::mpsc::{Receiver, Sender};
    let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();

    tx.send(Fruit::Orange("sweet".to_string())).unwrap();
    tx.send(Fruit::Apple(2)).unwrap();

    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) => println!("received {} apples", count),
            Fruit::Orange(flavor) => println!("receiived {} oranges", flavor),
        }
    }

    // 新手坑
    println!("新手坑");
    let (send, recv) = mpsc::channel();
    let num_threads = 3;
    for i in 0..num_threads {
        let thread_send = send.clone();
        thread::spawn(move || {
            thread_send.send(i).unwrap();
            println!("thread {:?} finished", i);
        });
    }

    // 在这里drop send...
    drop(send); // 如果不drop send，这个send通道一直在打开状态，下面的循环则会不断循环获取

    for x in recv {
        println!("Got: {}", x);
    }
    println!("finished iterating");

    // 性能更好的多发送多接收库
    // crossbeam-channel、flume

    // 单线程中使用Mutex
    let m = Mutex::new(5);

    {
        // 获取锁，然后deref为`m`的引用
        // lock返回的是Result
        let mut num = m.lock().unwrap();
        *num = 6;
        // 锁自动被drop
    }
    println!("m = {:?}", m);

    // 多线程中使用Mutex
    // 通过`Arc`实现`Mutex`的所有权
    //  Arc它的内部计数器是多线程安全的
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // 创建子线程，并将`Mutex`的所有权拷贝传入到子线程中
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // 等待所有子线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 输出最终的计数结果
    println!("Result: {}", *counter.lock().unwrap());

    // 总结：Rc<T>/RefCell<T>用于单线程内部可变性， Arc<T>/Mutex<T>用于多线程内部可变性。
    // 使用try_lock可以缓解死锁问题，因为它会尝试去获取一次资源，如果无法获取会返回一个错误，因此不会发生阻塞

    // 读写锁
    use std::sync::RwLock;

    let lock = RwLock::new(5);

    // 同一时间允许多个读
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        let r3 = lock.try_read();

        println!("锁的获取结果{:?}", r3);
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }

    // 同一时间只允许一个写
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
        // 以下代码会阻塞发生死锁，因为读和写不允许同时存在
        // 写锁w直到该语句块结束才被释放，因此下面的读锁依然处于`w`的作用域中
        // let r1 = lock.read();
        // println!("{:?}",r1);
        let r1 = lock.try_read();
        println!("锁的获取结果{:?}", r1)
    }

    // 总结：使用RwLock要确保满足以下两个条件：并发读，且需要对读到的资源进行"长时间"的操作

    // 锁的三方库：parking_lot、spin(性能高于前者，但是最近没更新了)

    // 用条件变量(Condvar)控制线程同步

    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = Arc::clone(&flag);
    let ccond = Arc::clone(&cond);

    let hdl = spawn(move || {
        let mut lock = cflag.lock().unwrap();
        let mut counter = 0;

        while counter < 3 {
            while !*lock {
                // wait 方法会接收一个MutexGuard<'a, T>，且它会自动地暂时释放这个锁，使其他线程可以拿到这个锁并更新数据
                // 同时当前线程在此处会被阻塞，直到被其他地方notify后，他会将原来的MutexGuard<'a, T>还给我们，即重新获取到了锁，同时唤醒了此线程
                lock = ccond.wait(lock).unwrap();
            }
            *lock = false;

            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;
    loop {
        sleep(Duration::from_millis(1000));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("outside counter: {}", counter);
        cond.notify_one();
    }
    hdl.join().unwrap();
    println!("{:?}", flag);

    // Atomic原子操作与内存顺序
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);

    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));

    println!("{:?}", Instant::now().sub(s));

    // 多线程中使用Atomic
    use std::hint;
    use std::sync::atomic::AtomicUsize;

    let spinlock = Arc::new(AtomicUsize::new(1));

    let spinlock_clone = Arc::clone(&spinlock);
    let thread = thread::spawn(move || {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    // 等待其他线程释放锁
    while spinlock.load(Ordering::SeqCst) != 0 {
        hint::spin_loop();
    }

    if let Err(panic) = thread.join() {
        println!("Thread had an error:{:?}", panic);
    }

    // 为裸指针实现Send
    let p = MyBox(5 as *mut u8);
    let t = thread::spawn(move || {
        println!("{:?}", p);
    });

    t.join().unwrap();

    // 为裸指针实现Sync
    let b = &MyBox1(5 as *const u8);
    let v = Arc::new(Mutex::new(b));
    let t = thread::spawn(move || {
        let _v1 = v.lock().unwrap();
    });

    t.join().unwrap();
}

fn advanced_parctice() {
    // advanced
    // advanced_lifetime();
    // advanced_futures();
    // closure();
    // Iterator_parctice();
    // type_parctice();
    // auto_ptr();
    multiple_thread();
}

// 信号量 Semaphore
use tokio::sync::Semaphore;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // base_parctice();
    advanced_parctice();

    // 信号量 Semaphore
    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_handles = Vec::new();

    for _ in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            //
            // 这里执行任务
            //
            drop(permit);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }

    Ok(())
}
