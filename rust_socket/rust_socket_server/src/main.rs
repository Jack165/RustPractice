use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::net::{TcpListener,TcpStream};
use std::io::{Error,Read,Write};
use std::time;

#[derive(Debug)]
struct stud<T1,T2>{
    id :T1,
    name:T2
}
impl<T1,T2> stud<T1,T2>{
    fn init( id:T1,name:T2)->stud<T1,T2>{
         stud{id,name}
    }
}


impl<T1,T2> stud<T1,T2>{
    fn handle_client(mut stream: TcpStream,id:T1,name:T2) -> Result<(), Error>{
      let mut buf=[0;512];
      for _ in 0..100 {
        buf=[0;512];
        let bytes_read=stream.read(&mut buf)?;
        if bytes_read == 0{
        return  Ok(());
        }
       
       let s= String::from_utf8_lossy(&buf);
        println!("收到客户端消息:{}",s);
        stream.write(&buf[..bytes_read])?;
        thread::sleep(time::Duration::from_secs(1 as u64));
      }
         Ok(())
    }
} 

fn main()  -> std::io::Result<()>{
    let mut studServer= stud::init("超级赛亚人","悟吉塔");
    let listener=TcpListener::bind("127.0.0.1:8000")?;
    let mut thread_vec:Vec<thread::JoinHandle<()>>=Vec::new();

    for stream in listener.incoming(){
        println!("进入");
        let stream=stream.expect("faild");
        let handle=thread::spawn(move || {
            println!("线程内");
            stud::handle_client(stream,"这是服务端id","这是服务端姓名").unwrap_or_else(|error| eprintln!("{:?}",error));
        });
        thread_vec.push(handle);
    }
    for handle in thread_vec{
        handle.join().unwrap();
    }
    Ok(())
 /*
let handle= thread::spawn(move || {
    for i in 0..6 {
        let st1= stud::init(i, String::from("悟吉塔"));
        println!("Hello, world! {:#?} ",st1);
        println!("值是{}",test_func());
        let  a= i;
        match a {
            1=>{
                println!("这是{}",i);
            },
            2=>{
                println!("这是2");
            },
            3=>{
                println!("这是3");
            },
             4=>{
                println!("这是4");
            }, 
            5=>{
                println!("这是5");
            },
            _=>{}
    
        }
    }
});

let (tx,rx) = mpsc::channel();

let _thread1= std::thread::spawn(move || {
    thread::sleep(Duration::from_millis(1));
    let val=String::from("这是线程1");
    println!("进入线程1");
    tx.send(val).unwrap();
    println!("传值结束");

});

let _thread2=std::thread::spawn(move || {
    let val=rx.recv().unwrap();
    println!("进入线程2");
    println!("线程1的值:{}",val);

    println!("读值结束");
});
thread::sleep(Duration::from_millis(5));
println!("结束");
  */

}
fn test_func()->u32{
    let a=1;
    let b=a;
    return b;
}
