use std::process;
use std::collections::HashMap;

fn check_calendar (japaese: &str) -> &str {
    if japaese == "M" {
        return "明治";
    }
    if japaese == "T" {
        return "大正";
    }
    if japaese == "S" {
        return "昭和";
    }
    if japaese == "H" {
        return "平成";
    }
    if japaese == "R" {
        return "令和";
    }
    print!("未対応の和暦が入力されました");
    process::exit(0);
}

fn main () {
    println!("和暦を入力 例)令和ならばRを入力");
    let mut calendar_input = String::new();
    std::io::stdin().read_line(&mut calendar_input).ok();
    
    // trimしないと文字列一致しないため
    let japanese_calendar = check_calendar(&calendar_input.trim());
    println!("{}", japanese_calendar);



    println!("何年？");
    let mut year_input = String::new();
    std::io::stdin().read_line(&mut year_input).ok();

    let year: i32 = match year_input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("数字を入力してください");
            return;
        }
    };

    let mut convert_table: HashMap<&str, i32> = HashMap::new();
    convert_table.insert("明治", 1868);
    convert_table.insert("大正", 1912);
    convert_table.insert("昭和", 1926);
    convert_table.insert("平成", 1989);
    convert_table.insert("令和", 2019);

    let western_calendar = match convert_table.get(japanese_calendar) {
        Some(&value) => {
            value
        },
        None => {
            println!("oops!");
            process::exit(0);
        },
    };

    println!("西暦{}年です", western_calendar + year -1);
}

