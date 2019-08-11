#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::mem;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

// implementation on generic type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// implementation on concrete type
// impl Point<i32> {
//     fn x(&self) -> &i32 {
//         &self.x
//     }
// }

// end of https://doc.rust-lang.org/book/second-edition/ch10-01-syntax.html
// You might be wondering whether there is a runtime cost when you re
// using generic type parameters. The good news is that Rust implements
// generics in such a way that your code doesn t run any slower using
// generic types than it would with concrete types.

// Rust accomplishes this by performing monomorphization of the code that
// is using generics at compile time. Monomorphization is the process of
// turning generic code into specific code by filling in the concrete
// types that are used when compiled.

fn sample() {
    let vec_i32 = vec![10, 1, 20, 3, 4, 5];
    let vec_char = vec!['a', 'b', 'z', 'y', 'w'];

    let rs_i32 = largest_i32(&vec_i32);
    let rs_char = largest_char(&vec_char);

    let (li, lc) = (largest(&vec_i32), largest(&vec_char));
    println!("{} {} {} {}", rs_i32, rs_char, li, lc);

    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 1.0, y: 1.0 };
    let p3 = Point2 { x: 1, y: 1.0 };

    println!("{:?} {:?} {:?}", p1, p2, p3);
    println!("{}", p1.x());
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &x in list.iter() {
        if x > largest {
            largest = x
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &x in list.iter() {
        if x > largest {
            largest = x
        }
    }

    largest
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut l = list[0];

    for &x in list.iter() {
        if x > l {
            l = x
        }
    }

    l
}

fn panik() -> () {
    let f = File::open("hello");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("No such file: {:?}", error),
    };
}

const ANSWER_TO_LIFE: u32 = 42;
// immutable
static MONTHS: u32 = 12; // fixed address

// mutable static - needs to be enclosed with unsafe block when using it
static mut GLOBAL_STATIC_MUT: u8 = 2; // fixed address

fn main() {
    println!("hello world!");
    let foo = format!("hello rustacean!");
    println!("{}", foo);

    let x = 5 + 10;
    println!("x is {}", x);

    #[allow(unused_variables)]
    let y: i32 = 42;

    let w = 1;
    let wbytes = mem::size_of_val(&w);
    println!("\n{} bytes", wbytes);

    let z: isize = 1; // size_t
    let zbytes = mem::size_of_val(&z);
    println!("\n{} bytes. {} bit os\n", zbytes, zbytes * 8);

    let t: usize = 1;
    let tbytes = mem::size_of_val(&t);
    println!("\n{} bytes. {} bit os\n", tbytes, tbytes * 8);

    // ignore unused_variable by starting var with _
    let _truth: bool = false;

    // supress warning with rustc -A unused_variables flag
    // RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo build
    let mut khar: char = 'c';

    println!("khar = {}", khar);
    khar = 'd';
    println!("khar = {}", khar);

    let (a, b) = (1, "a");
    println!("a = {} b = {}", a, b);

    println!("{0}, {1} {0}", "Bond", "James");
    println!(
        "{second_name}, {first_name} {second_name}",
        first_name = "James",
        second_name = "Bond"
    );

    #[allow(dead_code)]
    struct Structure(i32);

    // println!("this won't print {:?}", Structure(2));

    #[allow(dead_code)]
    struct UnPrintable(i32);

    #[derive(Debug)]
    struct Printable(i32);

    println!("this will print printable {:?}", Printable(2));

    for x in 1..4 {
        println!("{}", x);
    }

    for (i, x) in (1..4).enumerate() {
        println!("{} -> {}", i, x);
    }

    let mut i = 0;

    while i < 3 {
        println!("while {}", i);
        i += 1
    }

    println!("{}", "abc" > "abcd");

    let mut n = 1;
    println!("n = {}", n);
    n = 2;
    println!("n = {}", n);
    let n = 3.14; // redeclaration into different type allowed
    println!("n = {}", n);

    if x > 0 && x < 10 {
        println!("oh");
    } else if x == 15 {
        println!("ah");
    } else {
        println!("whatever");
    }

    let mut z = 0;

    // compiler will suggest you use loop to denote infinite loops if the attribute is not added
    #[allow(while_true)]
    while true {
        if z == 5 {
            println!("done counting to 4");
            break;
        } else {
            z += 1
        }
    }

    // write this instead of while true is what the compiler is suggesting
    loop {
        if z == 5 {
            println!("done counting to 4");
            break;
        } else {
            z += 1
        }
    }

    let sentence = ["this", "is", "cool"];
    println!("{:?}", sentence);
    println!("{}", sentence.len());

    #[allow(unused_mut, unused_variables)]
    let mut coll_of_str = ["a", "b", "c"]; // type [&str; 3]
                                           /* mismatched types
                                           (expected an array with a fixed size of 3 elements, found one with 2 elements) [E0308]
                                               */
    // coll_of_str = ["a", "b"]; // type [&str); 2] -> overall type does not match
    // coll_of_str = [1, 2, 3]; // expected &str but got integer
    coll_of_str[2] = "Y";
    println!("{:?}", coll_of_str);

    let fixed_coll = ["x"; 10]; // array of fixed size 10 initialized with "x"
    println!("{:?}", fixed_coll);

    // multi-dimensional array
    let arr = [[0; 5]; 5];
    println!("{:?}", arr);

    let one_dim_arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", one_dim_arr);

    let inited_arr = [0; 5];
    println!("{:?}", inited_arr);

    // can't compare `[{integer}; 5]` with `[{integer}; 2]`
    // if inited_arr == [1, 2] {
    //     println!("matched");
    // }

    let uninited_arr: [u16; 5];
    // println!("{:?}", uninited_arr); // uninitialized variable

    let two_dim_arr: [[i32; 3]; 3] = [[1, 2, 3], [1, 2, 3], [1, 2, 3]];
    println!("{:?}", two_dim_arr);

    // vectors are dynamic arrays

    let mut v: Vec<&str> = vec!["this", "is"];
    v.push("cool!");
    println!("{:?}", v);
    if let Some(x) = v.pop() {
        println!("popped {}", x);
    }
    println!("{:?}", v);
    v.insert(2, "rad");
    v.insert(0, "awesome");
    println!("{:?}", v);
    v.remove(0);
    println!("{:?}", v);

    let hexadecimal = 0x10;
    let decimal = 10;
    let octal = 0o10;
    let binary = 0b10;
    println!(
        "\nNumbers: {} {} {} {}\n",
        hexadecimal, decimal, octal, binary
    );

    let a: () = ();
    let b = {
        12;
        87;
        283
    }; // expression with value of 283 -> notice lack of semi-colon
    let c = {
        12;
        87;
        283;
    }; // expression with value of () -> notice semi-colon
    let d = {};
    let e = if false {};
    let f = while false {};
    print!("---- {:?} {:?} {:?} {:?} {:?} {:?} ----", a, b, c, d, e, f);

    #[allow(dead_code)]
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    let day = Day::Friday;

    match day {
        Day::Monday => println!("monday"),
        Day::Tuesday => println!("tuesday"),
        Day::Wednesday => println!("wednesday"),
        Day::Thursday => println!("thursday"),
        Day::Friday => println!("friday"),
        Day::Saturday => println!("saturday"),
        Day::Sunday => println!("sunday"),
    }

    #[allow(dead_code)]
    enum Editor {
        Emacs,
        Vim,
        Ed,
        Nano,
    };

    let editor = Editor::Emacs;

    // compiler error -> non-exhaustive pattern match
    // match editor {
    //     Editor::Ed => "3",
    //     Editor::Emacs => "1",
    //     Editor::Vim => "2",
    // };

    #[allow(dead_code)]
    enum Result {
        Success(f64),
        Failure(u16, char),
        Uncertainty,
    };

    enum Colour {
        Red,
        Green,
        Blue,
        GgbColour(u8, u8, u8),
        CmykColour {
            cyan: u8,
            magenta: u8,
            yellow: u8,
            black: u8,
        },
    }

    let colour = Colour::CmykColour {
        cyan: 0,
        magenta: 0,
        yellow: 0,
        black: 255,
    };

    match colour {
        Colour::Red => println!("Colour: red"),
        Colour::Blue => println!("Colour: blue"),
        Colour::Green => println!("Colour: green"),
        Colour::GgbColour(r, g, b) => println!("Colour: r {} g {} b {}", r, g, b),
        Colour::CmykColour {
            cyan: c,
            magenta: _,
            yellow: _,
            black: b,
        } => println!("Colour: c {} b {} ", c, b),
    }

    // let outcome = Result::Success(23.67);
    let outcome = Result::Failure(1200, 'X');

    match outcome {
        Result::Success(value) => println!("Result: {}", value),
        Result::Failure(error_code, module) => {
            println!("Error n. {} in module {}", error_code, module)
        }
        Result::Uncertainty => {}
    };

    match outcome {
        Result::Success(value) if value != 10. => println!("Result: {}", value), // Guards
        Result::Success(value) => println!("Result: {}", value),
        Result::Failure(error_code, module) => {
            println!("Error n. {} in module {}", error_code, module)
        }
        Result::Uncertainty => {}
    };

    let tup = ("tuple", 1);
    println!("{:?}", tup);

    #[allow(dead_code)]
    struct PureStructPoint {
        x: u8,
        y: u8,
    };

    #[allow(dead_code)]
    struct TupleStructPoint(u8, u8);

    #[derive(Debug)]
    struct User {
        name: String,
        email: String,
        age: u8,
        active: bool,
    }

    union IntOrFloat {
        i: i32,
        f: f32,
    }

    let mut iorf = IntOrFloat { i: 123 };
    iorf.i = 234;
    // type is indeterministic since it can be either int or float.
    // needs an unsafe block
    unsafe {
        println!("irof {}", iorf.i);
    }

    fn _creat_user(name: String, age: u8, email: String) -> User {
        User {
            name: name,
            age: age,
            email: email,
            active: true,
        }
    }

    // Field init short-hand syntax
    fn _shorthand_creat_user(name: String, age: u8, email: String) -> User {
        User {
            name,
            age,
            email,
            active: true,
        }
    }

    // Struct update syntax
    let u1 = User {
        name: String::from("a"),
        email: String::from("a@a.com"),
        age: 1,
        active: true,
    };

    let u2 = User {
        name: String::from("b"),
        email: String::from("b@b.com"),
        ..u1
    };

    println!("{:?} {:?}", u1, u2);

    // Tuple structs are useful when you want to give the whole tuple
    // a name and make the tuple be a different type than other
    // tuples, but naming each field as in a regular struct would be
    // verbose or redundant.

    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    // Note that the black and origin values are different types,
    // since they're instances of different tuple structs. Each struct
    // we define is its own type, even though the fields within the
    // struct have the same types. For example, a function that takes
    // a parameter of type Color cannot take a Point as an argument,
    // even though both types are made up of three i32
    // values. Otherwise, tuple struct instances behave like tuples:
    // you can destructure them into their individual pieces and you
    // can use a . followed by the index to access an individual
    // value, and so on.

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Point(x, y, z) = origin;
    println!(
        "ORIGIN {} {} {} {} {} {}",
        x, y, z, origin.0, origin.1, origin.2
    );

    println!("{:?} {:?}", black, origin);

    #[allow(dead_code)]
    const MAX_LIMIT: u16 = 10; // type is mandatory. Has no fixed address

    println!(
        "answer to life: {} number of months {}",
        ANSWER_TO_LIFE, MONTHS
    );

    unsafe {
        GLOBAL_STATIC_MUT = 0;
        println!("global static mutable {}", GLOBAL_STATIC_MUT);
    }

    // return type is ()
    // equivalent to fn cool() -> () { ... }
    fn cool() {
        println!("cool!")
    }

    cool();

    let res = if true { 1 } else { 2 };
    // let res = if true { 1 } else { 2.0 };

    /*
     compiler error:
     expected type `{integer}` found type `{float}` [E0308]
     if and else have incompatible types (expected integral
     variable, found floating-point variable) [E0308]

    */

    println!("{:?}", res);

    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    println!("sum {}", sum(1, 2));
    println!("sum {}", sum(2, 2));

    fn inc(mut n: i32) -> i32 {
        n += 1;
        return n;
    }

    println!("{}", inc(1));

    fn add1(a: &mut [i32]) {
        for i in 0..(a.len() - 1) {
            a[i] += 1;
        }
    }
    let mut arr = [1, 2, 3, 4, 5];
    add1(&mut arr);
    println!("mut arr: {:?}", arr);

    fn add1_with_arr_size(a: &mut [i32; 5]) {
        for i in 0..5 {
            a[i] += 1;
        }
    }

    add1_with_arr_size(&mut arr);
    println!("mut arr with size: {:?}", arr);

    let num = 10;
    let ref_num = &num;
    println!("Ref num {} {} {}", num, *ref_num, ref_num);

    // Generic fn
    fn fun<T>(ch: char, num1: T, num2: T) -> T {
        if ch == 'a' {
            num1
        } else {
            num2
        }
    }

    let a: i16 = fun::<i16>('a', 37, 41);
    let b: f64 = fun::<f64>('b', 37.2, 41.1);
    println!("generic {} {}", a, b);

    let a: i16 = fun('a', 37, 41);
    let b: f64 = fun('b', 37.2, 41.1);
    println!("generic {} {}", a, b);

    let a = fun('a', 37, 41);
    let b = fun('b', 37.2, 41.1);
    println!("generic {} {}", a, b);

    fn simple<S>(simp: S) -> S {
        simp
    }

    let s1 = simple(1);
    let s2 = simple(1.0);
    let s3 = simple('a');
    let s4 = simple("asdf");
    println!("{} {} {} {}", s1, s2, s3, s4);

    struct Table<Pk> {
        id: i64,
        key: Pk,
    }

    let t = Table { key: "a", id: 1 };
    println!("Table {} -> {}", t.id, t.key);

    let t = Table { key: 1, id: 1 };
    println!("Table {} -> {}", t.id, t.key);

    let t = Table { key: "asdf", id: 1 };
    println!("Table {} -> {}", t.id, t.key);

    let word = "yzxa";
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    println!("{:?}", chars);

    let s1: String = chars.into_iter().collect();

    let word2 = "asdf";
    let mut chars2: Vec<char> = word2.chars().collect();
    chars2.sort();
    println!("{:?}", chars2);

    use std::iter::FromIterator;

    let s2: String = String::from_iter(chars2);

    println!("{:?} {:?}", s1, s2);

    fn plus1(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus1(five);
    let none = plus1(None);
    println!("plus1: {:?} {:?} {:?}", six, five, none);

    // if let demonstration - common usage pattern

    let something = Some(1);
    match something {
        Some(x) => println!("{}", x),
        _ => (),
    }

    if let Some(x) = Some(1) {
        println!("if let: {}", x);
    }

    let something_else = Some(1);
    match something_else {
        Some(x) => println!("{}", x),
        _ => println!("Gone!"),
    }

    if let Some(x) = Some(1) {
        println!("if let: {}", x);
    } else {
        println!("if let: Gone!");
    }

    let v1: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];
    println!("vectors: {:?} {:?}", v1, v2);

    let mut v3 = vec![1, 2, 3]; // has to be mutable to pop
    while let Some(x) = v3.pop() {
        println!("popped {}", x);
    }

    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(2);

    println!("{:?}", v3);

    let v4 = vec![1, 2, 3, 4, 5];
    let third = &v4[2];
    let t = v4.get(2); // returns option type
    println!("{:?} {:?} {:?}", third, t, v4.get(10));

    if let Some(x) = v4.get(10) {
        println!("within bound");
    } else {
        println!("out of bound");
    }

    let v = vec![1, 2, 3, 4, 5];

    let _third: &i32 = &v[2];
    let _third: Option<&i32> = v.get(2);

    let s = "dummy";

    println!("{}, {}, {}", s, "asdf".to_string(), String::from(s));

    let s1 = "hello ".to_string();
    let s2 = "world!".to_string();
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // does not work since unicode chars are multibyte
    // let s1 = String::from("hello");
    // let h = s1[0];
    let _len = String::from("Hola").len();

    // #![allow(unused_variables)]
    // for c in "      ".chars() {
    //     println!("{}", c);
    // }

    let mut m = HashMap::new();
    m.insert(String::from("a"), 1);
    m.insert(String::from("b"), 2);
    println!("hashmap: {:?}", m);

    let ks = vec![String::from("a"), String::from("b")];
    let vs = vec![1, 2];

    let m: HashMap<_, _> = ks.iter().zip(vs.iter()).collect();
    println!("hashmap: {:?}", m);

    let s = String::from("a");
    let k = m.get(&s);
    println!("{:?}", k);

    for (k, v) in &m {
        println!("{} {}", k, v);
    }

    let mut m = HashMap::new();

    m.insert(String::from("a"), 1);
    m.entry(String::from("b")).or_insert(2);

    let mut m = HashMap::new();
    let s = "goodbye cruel world";

    for word in s.split_whitespace() {
        let c = m.entry(word).or_insert(0);
        *c += 1;
    }
    println!("{:?}", m);

    sample();
}
