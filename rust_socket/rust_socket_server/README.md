本段代码中使用了 move关键字:
使用 move关键字 强制所有权转移,强制将线程中使用的线程外的变量的作用域，指定到线程内，线程之后的代码将不能使用被线程使用的变量，否则编译不通过
       假设如下代码
       如下代码段
       代码中有传index 到线程中

        let handle=thread::spawn(move || {
            println!("线程:{}",index);
           handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}",error));
           index=index+1;
        });


        在此代码段前声明了index，所以index作用域应该是整个main方法，但是因为在新创建的线程中使用了index，并且使用了move 关键字，导致index的所有权被线程捕获，即创建线程之后的代码，将不能在使用index这个变量。
        
        此例的move关键字的作用 强制将线程内使用的所有线程外声明的变量的生存周期转移到线程内。线程外将不能再使用被转移所有权的变量