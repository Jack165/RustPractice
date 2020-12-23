

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

fn main() {
   
}
