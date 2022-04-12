fn main() {
    
    chp4();
}

fn chp4(){

    // chp4.1 所有权
    let s1 = "s1";

    {
        let s2 = "s2";

        println!("s2 {}",s2);
    }// 已经释放 s2，失去所有权。

    // 错误，s2 已释放
    // println!("s2 {}",s2);

    println!("s1 {}",s1);

    let mut s3 = String::from("s3");

    s3.push_str(",push_str");

    println!("s3 {}",s3);

    {
        let s4 = String::from("s4");

        println!("s4 {}",s4);
    }// s4 已释放，drop

    // 错误
    // println!("s4 => {}",s4);

    let s5 = "s5";
    let s6 = s5;

    println!("s5,s6 {},{}",s5,s6);

    // 对象二次释放错误
    // let s7 = String::from("s7");
    // let s8 = s7;

    let s7 = String::from("s7");
    let s8 = s7.clone();

    println!("s7,s8 {},{}",s7,s8);

    let s9 = String::from("s9");

    takes_ownership(s9);

    // s9 拥有权在 task_ownership() 就结束了
    // println!("s9 {}",s9);
}

fn takes_ownership(str: String){

    println!("take_ownership => {}",str);
}
