/**
 * rust变量类型
 * 
 * rust是静态类型语言
 * 
 * * 标量类型
 * 标量类型是单个值类型的统称
 * rust中有4种标量类型：整数、浮点数、布尔、字符
 * 
 * 
 * * 复合类型
 * 元组、数组
 */
pub mod mod_type {

    pub fn type_fn() {
        /*
        * 整数：有符号和无符号，有8-16-32-64位，还有一个特殊的arch（isize、usize）他们的长度取决于程序运行的目标平台
        */
    
        /*
        * 浮点：分为f32和f64，分别占32和64位
        */
    
        let x = 1;
        let y = 1.2;
        // 不同类型不能进行运算
        // let c = x+y;
    
        // println!("{}", c);
    
        /*
        * bool
        */
        let f = true;
    
        /*
        * 字符类型，占4字节，Unicode
        */
        let c = 'Z';
    
    
        /*
        * 元组类型
        */
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        // 结构元组
        let (x, y, z) = tup;
        // 通过索引访问
        let v_1 = tup.0;
        println!("{}, {}", x, v_1);
    
        /*
        * 数组类型：数组中每一个元素的类型必须相同，数组长度固定
        */
        // 方括号中位类型 长度
        let arr: [i32; 3] = [1,2,3];
        // 方括号中位值 长度
        let arr_1 = [3; 5];

        /*
        * string
        */
        let str_1 = String::from("hello");
        println!("str_1: {}", str_1);

        let str_2 = str_1;
        // println!("str_1: {}", str_1);


    }
}
