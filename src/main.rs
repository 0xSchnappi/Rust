//! 文档注释
//! 文档注释

/// 文档注释
/// 稳定注释
use std::fmt::{self, Display};

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

impl fmt::Display for DisplayStructure {
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

fn match_practice() -> u8 {
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

fn main() {
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
}
