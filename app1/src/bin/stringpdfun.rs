fn main (){
    let str = String::from("i am from INDIA");
    println!("{}",str);
    println!("{}","i am from INDIA".to_string());
    println!("{}", str.len());
    println!("{}", str.is_empty());
    println!("{}", "".is_empty());

    let mut str1 = String::from("i am from INDIA");
    str1.push('$');
    println!("{}",str1);

    str1.push_str("(+91)");
    println!("{}", str1);

    let finalstring = format!("{}/{}", "sample", "Y65JK2");
    println!("{}",finalstring);

    let s1 = String::from("sample Text");
    let s2 = String::from("Sample Message");
    let res = s1 + &s2;
    println!("{}",res);

    println!("{}", str1.contains("india"));
    println!("{}", str1.contains("from"));

    println!("{}", str1.starts_with("i am"));
    println!("{}", str1.ends_with("INDIA$(+91)"));

    //println!("{}", str1.find("i am"));
    //println!("{}", str1.find("from"));
    //println!("{}", str1.find('f'));

    println!("{}", str1.replace("m", "$"));
    println!("{}", str1.replace("from","in"));

    println!("{}", str1.replacen("m", "$", 1));

    println!("{}", str1.to_lowercase());
    println!("{}", str1.to_uppercase());

    let str2 = String::from("       india      ");
    println!("{}", str2.trim_start());
    println!("{}", str2.trim_end());
    println!("{}", str2.trim());

    for ss in str1.split(' ' ){
        println!("{}", ss);
    }

    for ss in str1.split_whitespace(){
        println!("{}", ss);
    }

    let str3 = "Ram/25/Mumbai/Manager";
    for ss in str3.split('/'){
        println!("{}", ss);
    }

    for ch in str1.chars(){
        println!("{}", ch);
    }

    println!("{:?}", str1.chars().nth(3));
    println!("{}", str1.chars().count());

    for (ind, ch) in str1.char_indices(){
        println!("index:{} char:{}", ind, ch);
    }

    let str4="*";
    println!("{}", str4.repeat(25));
    println!("{}", str1.repeat(3));

    let scores = ["29", "58", "66", "44", "13"];
    println!("{:?}", scores.join(","));

    
}