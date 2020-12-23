use std::io::{self,prelude::*,BufReader,Write};
use std::net::TcpStream;
use std::str;
fn main() -> std::io::Result<()> {
    let mut stream=TcpStream::connect("127.0.0.1:8000")?;
    for _ in 0..10{
        let mut input =String::new();
        io::stdin().read_line(&mut input).expect("读取控制台输入失败");
        stream.write(input.as_bytes()).expect("写入发送流失败");
       
        let mut reader=BufReader::new(&stream);
        let mut buffer:Vec<u8> =Vec::new();
        reader.read_until(b'\n', &mut buffer).expect("缓存写入字符串失败");
    
    }
    Ok(())

}
