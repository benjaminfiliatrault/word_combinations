use std::collections::HashSet;

pub fn share_char(a: &str, b: &str) -> bool {
    // get which one is shorter
    let (shorter, longer) = if a.len() > b.len() { (b, a) } else { (a, b) };
    // fill the set with the characters from the shorter string
    let set: HashSet<char> = shorter.chars().collect();
    longer.chars().any(|c| set.contains(&c))
}

#[test]
fn test() {
    let str1 = "abler";
    let str2 = "ablow";
    let str3 = "zymin";

    assert!(share_char(str1, str2));
    assert!(!share_char(str1, str3));
}


pub fn clear_line(){
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}