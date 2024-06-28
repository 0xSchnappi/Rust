//! 文档注释
//! 文档注释

/// 文档注释
/// 稳定注释
use std::fmt;

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
    let test = Range{start:1, end:5};
    print!("{:?}", test);

    for i in RangeInclusive::new(1, 5) {
        print!("{}", i);
    }
    // https://course.rs/basic/base-type/statement-expression.html

}
