
//定义吃瓜群众评论的行为
trait Evaluate{
    fn wu_di(&self);
    fn yi_ban(&self);
}
//奥特曼对象
struct Ultraman {
    title: String,
    name : String

}

//定义一个超级赛亚人族群
#[derive(Debug)]
struct SuperSaiya<T1,T2>{
    title :T1,
    name:T2
}

impl<T1,T2> SuperSaiya<T1,T2>{
    //超级赛亚人生成器，通过泛型生成各种各样的赛亚人
    fn get_object( title:T1,name:T2)->SuperSaiya<T1,T2>{
        SuperSaiya{title,name}
    }
    //通过self能获取到被关联结构体SuperSaiya的属性
    fn remark(&self){
        println!("这是龙珠的世界");
    }
}

//对超级赛亚人的评论行为
impl  Evaluate for SuperSaiya<&str,&str>{
    fn wu_di(&self){
      println!("{}{}脸上写满无敌",&self.title,&self.name);
    }
    fn yi_ban(&self){
        println!("{}{}是具有潜力的小伙子!",&self.title,&self.name);
    }
}
//给奥特曼族群添加吃瓜群众评论的内容
impl Evaluate for Ultraman{
    fn wu_di(&self){
         println!("{}{}",&self.name,&self.title);
    }
    fn yi_ban(&self){
        println!("颤抖的{}",&self.name);

    }
}



fn main() {
    //生成超级赛亚人对象
    let wjt= SuperSaiya::get_object("超级赛亚人", "悟吉塔");
    //皮一下，执行无参数的方法打印标语
    wjt.remark();
    let blk= SuperSaiya::get_object("超级赛亚人", "贝洛克");
 
    //创建一个奥特曼
    let atm= Ultraman{
        name: String::from("迪加"),
        title:String::from("奥特曼")
    };
    //喊下口号
    atm.wu_di();
    //保存赛亚人到集合中（虽然没有定义集合存放的类型，但是添加第一个对象后，集合的类型就固定了，在集合中保存同种族就不行）
    let  mut super_saiya_list=Vec::new();
    super_saiya_list.push(wjt);
    super_saiya_list.push(blk);
    //创建一个普通的进行对比
    let s=SuperSaiya::get_object("生产的", "人");
    s.yi_ban();
    //遍历打印下
    for  elem in super_saiya_list.iter() {
    println!("{}{}",elem.title,elem.name);
    elem.wu_di();
   }

  

}
