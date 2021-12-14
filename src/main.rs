#![allow(unused_variables)]
#![allow(dead_code)]

pub(crate) use rayon::prelude::*;
pub(crate) use std::{
    cell::{Cell, RefCell},
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    fs::File,
    mem,
    ops::AddAssign,
    thread,
    time::Duration,
};

fn main() {
    println!("write to stdout");
    eprintln!("write to stderr");

    let origin = FooPoint { x: 0, y: 0, z: 0 };

    // ignoring few values in pattern matching
    match origin {
        FooPoint { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    numbers.

    let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for x in a.chunks(2) {
        dbg!(x);
    }

    let xs: Vec<(i32, i32)> = (0..10)
        .flat_map(|i| (0..3).map(move |j| (i, j)).collect::<Vec<(i32, i32)>>())
        .collect();
    println!("{:?}", xs);

    let mut heap = BinaryHeap::new();

    // We can use peek to look at the next item in the heap. In this case,
    // there's no items in there yet so we get None.
    assert_eq!(heap.peek(), None);

    // Let's add some scores...
    heap.push(Foo {
        a: 1,
        b: String::from("a"),
    });
    heap.push(Foo {
        a: 5,
        b: String::from("b"),
    });
    heap.push(Foo {
        a: 2,
        b: String::from("c"),
    });

    println!("{:?}", heap.pop());
    println!("{:?}", heap.pop());
    println!("{:?}", heap.pop());

    let ids = &[1, 2, 3];
    let zeros: Vec<i32> = itertools::repeat_n(0, 10).collect();
    let v: HashMap<_, _> = ids.iter().zip(zeros).collect();
    println!("{:?}", v);
    println!("{}", "a//".replace("//", "/"));
    let s = String::from("book");

    let ss = pluralize(s.as_str());
    let sss = pluralize(&s); // Deref trait and Deref coercion
    println!("singular: {}, plural: {} plural: {}", s, ss, sss);

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

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number = 1, width = 6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number = 1, width = 6);

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

    // compiler will suggest you to use loop to denote infinite loops if the attribute is not added
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

    let v: Vec<Vec<u32>> = vec![vec![]];
    println!("{:?}", v);

    let sentence = ["this", "is", "cool"];
    println!("{:?}", sentence);
    println!("{}", sentence.len());

    // compilation error
    // let x = sentence[3];
    if let Some(x) = sentence.get(3) {
        println!("{}", x)
    } else {
        println!("nothing!")
    }

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
    }

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
    }

    // tuple and struct enum
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

    impl User {
        fn nick(&self) {
            println!("nick method on type User {:?}", self.name);
        }
    }

    #[derive(Debug, Default)]
    struct Hero {
        name: String,
        nick: String,
        active: bool,
    }

    impl Hero {
        pub fn builder() -> Self {
            Self::default()
        }

        pub fn name(mut self, name: String) -> Self {
            self.name = name;
            self
        }

        pub fn nick(mut self, nick: String) -> Self {
            self.nick = nick;
            self
        }

        pub fn active(mut self, active: bool) -> Self {
            self.active = active;
            self
        }
    }

    let ubermensch = Hero::builder()
        .name("Superman".into())
        .nick("Clark Kent".into());

    println!("ubermensch {:?}", ubermensch);
    dbg!(ubermensch);

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

    u1.nick();
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

    // one of the use cases for tuple struct: "new type pattern"
    // like typedef but with type checking
    #[derive(Debug)]
    struct Meter(u8);

    fn print_meters(m: Meter) {
        println!("meters: {:?}", m);
    }

    let d1: Meter = Meter(10);
    // let d2: u8 = 10;
    print_meters(d1);
    // does not compile
    // print_meters(d2);

    // You can also define structs that don’t have any fields!
    // These are called unit-like structs because they behave similarly to (), the unit type.
    // Unit-like structs can be useful in situations in which you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
    #[derive(Debug)]
    struct UnitTypeStruct();

    impl UnitTypeStruct {
        pub fn new() -> Self {
            Self {}
        }
    }

    println!("unit-type struct: {:?}", UnitTypeStruct::new());

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

    // collect forces evaluation - otherwise the iterator is not evaluated
    let word = "yzxa";
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    println!("{:?}", chars);

    // into_iter takes ownership
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

    #[allow(unused_variables)]
    for c in "      ".chars() {
        println!("{}", c);
    }

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

    slices();

    // 'as' example
    {
        // utf-8
        let s: &str = "hello there";
        for c in s.chars() {
            println!("char: {}", c);
        }

        let idx = 0;
        if let Some(c) = s.chars().nth(idx) {
            println!("idx {} char {}", idx, c);
        } else {
            println!("Out of bound idx {} ", idx);
        }

        let mut alphas = String::new();
        let mut a = 'a' as u8;
        while a <= ('z' as u8) {
            alphas.push(a as char);
            alphas.push_str(" ");
            print!("{} ", a as char);
            a += 1;
        }
        println!("\n{}\n", alphas);

        // &str <> String
        let u: &str = &alphas;
        println!("{}", u);

        let z = alphas + " DONE";
        println!("{}", z);
    }

    // pattern matching
    fn pm(v: i32) {
        match v {
            0 => println!("zero"),
            ranger @ 1..=9 => println!("single digit in range: {}", ranger),
            13 | 666 => println!("number of the beast"),
            _ => println!("say what?"),
        }
    }
    pm(0);
    pm(5);
    pm(13);
    pm(666);

    let sq = |x: i32| -> i32 { x * x };
    let square = |x| x * x;

    // closure and higher order fn
    println!("{} {}", sq(4), square(10));
    println!("addn: {}", addn(12)(1));

    let sum_of_sqs = (0..)
        .map(|x| x * x)
        .take_while(|&y| y < 100)
        .filter(|&z| z % 2 == 0)
        .fold(0, |acc, x| {
            println!("acc: {} x: {}", acc, x);
            acc + x
        });
    println!("sum of even squares: {}", sum_of_sqs);

    println!("---- traits ----");
    let john = Human { name: "john doe" };
    john.talk();
    let garfield = Cat { name: "Garfield" };
    garfield.talk();
    println!("{} {}", john.name(), garfield.name());
    let h = Human::create("newton");
    let c = Cat::create("Garfield");
    println!("{:?} {:?}", h.name(), c.name());

    let uber: Human = Animal::create("uber mensch!");
    println!("{}", uber.name());

    println!("Sum trait => sum: {}", vec![1, 2, 3, 4].sum());

    // example of enums made of other types
    enum Creature {
        Human(Human),
        Cat(Cat),
    }

    let person = Person {
        name: String::from("buddha"),
    };
    person.talk();

    let sec = 2;
    let closure = |num: i32| {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(sec));
        num
    };

    closure(1);

    let twelve = &12 as &dyn Display;
    let hi = &"Hi" as &dyn Display;

    let v = vec![twelve, hi];
    show_all(v);

    let mut xs = vec![1, 2, 3, 4];

    for x in xs.iter_mut() {
        *x = *x + 1;
    }

    for x in xs.iter() {
        println!("{}", x);
    }

    xs.iter().take(2).for_each(|x| println!("{}", x));

    println!("{:?}", xs);

    // into_iter takes ownership
    for x in xs.into_iter() {
        println!("{}", x);
    }
    // following code won't work since ownership changed above
    // println!("{:?}", xs);

    let b1 = Box::new(42); // Box is heap allocated
    let b2 = Box::new(Some(42)); // Box is heap allocated

    println!("boxed {} {:?}", b1, b2);

    let cell = Cell::new(42);
    let ref_cell = RefCell::new(42);

    match std::fs::read_to_string("/home/abhi/.gitconfig") {
        Ok(a) => println!("{:?}", a),
        Err(e) => println!("{}", e),
    };

    let is_cool = std::fs::read_to_string("/home/abhi/.gitconfig")
        .map_err(|e| println!("{:?}", e))
        .and_then(|x| Ok(x))
        .map(|x| println!("- inside - \n{}", x))
        .is_ok();

    println!("is cool =>  {}", is_cool);

    let ids = &[1, 2, 3];
    let zeros: Vec<i32> = itertools::repeat_n(0, 10).collect();
    let v: Vec<(_, _)> = ids.iter().zip(zeros).collect();
    println!("{:?}", v);

    let a = &[10, 20, 30, 40, 50];
    let b = &[1, 2, 3];
    a.iter()
        .zip(b)
        .for_each(|(s, d)| println!("{:?} {:?}", s, d));

    b.iter()
        .zip(a)
        .for_each(|(s, d)| println!("{:?} {:?}", s, d));

    let p1 = Person {
        name: "a".to_string(),
    };
    let p2 = Person {
        name: "b".to_string(),
    };

    let ps: Vec<Person> = vec![p1, p2];

    let people = People::new(ps);

    for x in people {
        println!("person: {:#?}", x); // pretty print with {:#?}
        eprintln!("person: {:#?}", x); // pretty print with {:#?}
    }

    let n = 10; // Set n to 10_000_000 to see multi-core in action
    println!(
        "counting upto to {} in parallel. Set n to 10000000 to see multi-core in action",
        n
    );
    (0..n).into_par_iter().for_each(|i| println!("{}", i));
}

// generic type
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T: Display + Clone, U: Display + Copy> {
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

#[derive(Clone, Debug, Default)]
struct Person {
    name: String,
}

impl Person {
    fn talk(&self) {
        println!("hello {}", self.name);
    }
}

trait Animal {
    fn create(name: &'static str) -> Self; // associated fn does not take a self since it is not called on instance.
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

#[derive(Debug)]
struct Human {
    name: &'static str,
}

#[derive(Debug)]
struct Cat {
    name: &'static str,
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human { name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says hello!", self.name);
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

trait Sum<T> {
    fn sum(&self) -> T;
}

impl Sum<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        self.iter().fold(0, |acc, x| x + acc)
    }
}

// pre 2018 edition
fn addn_pre_2018(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| n + x)
}

fn addn(n: i32) -> impl Fn(i32) -> i32 {
    move |x| n + x
}

fn slices() {
    let d1 = [1, 2, 3, 4, 5];
    use_slice(&d1[1..4]);

    let mut d2 = [1, 2, 3, 4, 5];
    mut_slice(&mut d2[1..3]);
}

fn use_slice(slice: &[i32]) {
    println!("0th: {} len {}", slice[0], slice.len());
}

fn mut_slice(slice: &mut [i32]) {
    slice[0] = 99;
    println!("0th: {}", slice[0]);
}

fn pluralize(s: &str) -> String {
    let mut ss = String::from(s); // or let mut ss = s.to_owned(); // creates String
    ss.push_str("s");
    ss
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
// first element is used for Ordering
struct Foo {
    a: i32,
    b: String,
}

// https://stackoverflow.com/questions/28387711/implementing-ord-for-a-type-is-awkward#28388168

struct FooPoint {
    x: i32,
    y: i32,
    z: i32,
}

fn show_all(v: Vec<&dyn Display>) {
    for item in v {
        println!("{}", item);
    }
}

pub struct RangeIter {
    curr: i32,
    stop: i32,
    step: i32,
}

impl RangeIter {
    pub fn new(start: i32, stop: i32, step: i32) -> Self {
        RangeIter {
            curr: start,
            stop,
            step,
        }
    }
}

impl Iterator for RangeIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr > self.stop {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_iter() {
        let mut m = 0;
        let it = RangeIter::new(0, 10, 1);
        for s in it {
            m += s;
        }
        assert_eq!(m, 55);
    }

    #[test]
    fn gen_range_iter() {
        let mut m = 0.0;
        let it = GenRangeIter::new(0.0, 10.0, 1.0);
        for s in it {
            m += s;
        }
        assert_eq!(m, 55.0);
    }
}

pub struct GenRangeIter<T: Rangeable> {
    curr: T,
    stop: T,
    step: T,
}

impl<T: Rangeable> GenRangeIter<T> {
    pub fn new(start: T, stop: T, step: T) -> Self {
        GenRangeIter {
            curr: start,
            stop,
            step,
        }
    }
}

pub trait Rangeable: PartialOrd + AddAssign + Copy {}

impl<T: AddAssign + PartialOrd + Copy> Rangeable for T {}

impl<T: Rangeable> Iterator for GenRangeIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr > self.stop {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

struct People {
    pub inner: Vec<Person>,
    n: usize,
}

impl People {
    pub fn new(inner: Vec<Person>) -> Self {
        People { inner: inner, n: 0 }
    }
}

// https://doc.rust-lang.org/std/iter/trait.IntoIterator.html

impl IntoIterator for People {
    type Item = Person;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

// implementing iterator the hard-way

// impl Iterator for People {
//     type Item = Person;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.inner.iter().nth(self.n) {
//             None => None,
//             Some(item) => {
//                 self.n += 1;
//                 Some(item.to_owned())
//             }
//         }
//     }
// }

fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

pub struct Things {
    foo: String,
    bar: u32,
    baz: bool,
}

pub fn do_stuff(things: Things) -> String {
    let Things { foo, bar, .. } = things;
    format!("{}{}", foo, bar)
}

pub fn do_stuff2(Things { foo, bar, baz }: Things) -> String {
    format!("{}{}{}", foo, bar, baz)
}

pub fn do_stuff3(things: Things) -> String {
    match things {
        Things { baz: true, .. } => "No formatting".to_string(),
        Things {
            foo,
            bar,
            baz: false,
        } => format!("{}{}", foo, bar),
    }
}

#[test]
fn custom_assert_failure_message() {
    assert!(true, "this is the custom failure message");
    assert_eq!(2, 2, "this is the custom failure message `{}`", 2);
}

#[test]
#[should_panic]
fn assert_panic() {
    panic!("this will panic")
}

#[test]
#[should_panic(expected = "this is the panic string")]
fn assert_panic_message() {
    panic!("this is the panic string")
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}

// https://doc.rust-lang.org/book/ch11-01-writing-tests.html#using-resultt-e-in-tests

// Writing tests so they return a Result<T, E> enables you to use the
// question mark operator in the body of tests, which can be a
// convenient way to write tests that should fail if any operation
// within them returns an Err variant.

#[test]
fn test_returns_result() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}

// Some command line options go to cargo test, and some go to the
// resulting test binary. To separate these two types of arguments,
// you list the arguments that go to cargo test followed by the
// separator -- and then the ones that go to the test binary. Running
// cargo test --help displays the options you can use with cargo test,
// and running cargo test -- --help displays the options you can use
// after the separator --.

// $ cargo test -- --test-threads=1
// $ cargo test -- --show-output
// $ cargo test -- --ignored

// use std::io;
// use std::io::Write;

// can also be written as

// use std::io::{self, Write};

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("This is a default implementation")
    }
}

// impl Foobar is a syntactic sugar for trait bounds

// pub fn notify(item1: &impl Summary, item2: &impl Summary) {
// is equivalent to
// pub fn notify<T: Summary>(item1: &T, item2: &T) {

// trait inheritance
// https://stackoverflow.com/a/47966422/68963

#[derive(Debug)]
struct Point3<T: Display, U> {
    x: T,
    y: U,
}

// https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods

struct Pair<T> {
    x: T,
    y: T,
}

// new for generic type T
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// cmp_display implemented _only_ if T implements Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
