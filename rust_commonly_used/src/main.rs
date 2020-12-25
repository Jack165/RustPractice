fn main() {
    let mi="qweasdasdweqweqweqwde".chars();
    
    for c in mi {
        let s= (c as u8-32) as char;
        println!("{}",s);
    }
    
    let   a=jiajiemi("hello word");
    println!("加密后字符串：{}",a);
    let b=jiajiemi(&a);
    println!("解密后字符串{}",b);


}

fn jiajiemi (s:&str) -> String {
    let text=s.to_uppercase();
    text.chars().map(|c| match c{
        'A'..='M' =>((c as u8)+13) as char,
        'N'..='Z'=>((c as u8)-13) as char,
        _=>c,
    })
    .collect()
}
