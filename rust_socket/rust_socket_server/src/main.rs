use std::thread;
use std::net::{TcpListener,TcpStream};
use std::io::{Error,Read,Write};
use std::time;

 
/**
 *处理客户端的类
 */
fn handle_client(mut stream:TcpStream) -> Result<(), Error>{
    //一次最多读取512 byte
      let mut buf=[0;512];
      for _ in 0..100 {
        //每次接受数据前初始化一次，去掉上次接受的数据残留  
        buf=[0;512];
        let bytes_read=stream.read(&mut buf)?;
        if bytes_read == 0{
        return  Ok(());
        }
       //将byte转为utf-8格式的字符串
       let s= String::from_utf8_lossy(&buf);
        println!("收到客户端消息:{}",s);
        //将内容回输入回去
        stream.write(&buf[..bytes_read])?;
        //线程停顿，避免占用cpu过高
        thread::sleep(time::Duration::from_secs(1 as u64));
      }
         Ok(())
    }


    /**
     *程序入口
     */  
    fn main()  -> std::io::Result<()>{

        //绑定接口
    let listener=TcpListener::bind("127.0.0.1:8000")?;
    //定义一个Vec保存线程
    let mut thread_vec:Vec<thread::JoinHandle<()>>=Vec::new();
    let mut index=1;
    //此处阻塞方法，等有客户端链接后进入
    for stream in listener.incoming(){
        println!("有客户端进入");
        //出现异常后返回信息
        let stream=stream.expect("出现异常");
         //这个线程中使用了主线程的index变量，并且使用move关键字，所以在这线程之后不能在使用index
        let handle=thread::spawn(move || {
            println!("线程:{}",index);
           handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}",error));
           //此处可以发现index永远是1，说明线程内的数据，不会逃逸。
           index=index+1;
        });
        thread_vec.push(handle);
    }
    for handle in thread_vec{
        handle.join().unwrap();
    }
    Ok(())
}
