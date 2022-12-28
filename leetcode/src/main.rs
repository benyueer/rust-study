mod stack;
mod tree;
mod queue;
mod binary_search;
mod two_pointer;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test{
    // [1694] 重新格式化电话号码
    pub fn reformat_number(number: String) -> String {
        let mut res = String::from("");
        let mut num_str = number.replace("-", "").replace(" ", "");
        
        let len = num_str.len();

        let mut cur_idx = 0;

        while cur_idx + 4 < len {
            res.push_str(&num_str[cur_idx..cur_idx+3]);
            res.push('-');
            cur_idx += 3;
        }

        
        if len - cur_idx == 4{
            res.push_str(&num_str[cur_idx..cur_idx+2]);
            cur_idx += 2;
            res.push('-');
            res.push_str(&num_str[cur_idx..cur_idx+2]);
            
        } else {
            res.push_str(&num_str[cur_idx..])
        }
        
        println!("{res}");
        res
    }

    #[test]
    fn test_reformat_number() {
        reformat_number(String::from("1234"));
    }
}
