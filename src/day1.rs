use regex::Regex;

pub fn day1_1() {
    let s = std::fs::read_to_string("src/day1.txt").unwrap();
    let a = Regex::new(r#""#).unwrap();
    for cap in a.captures_iter(&s) {
        println!("{:?}", &cap[1]);
    }
    
}

pub fn day1_2() {

}
