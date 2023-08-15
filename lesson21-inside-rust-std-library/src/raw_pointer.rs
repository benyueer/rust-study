#[test]
fn raw_pointer() {
    // 裸指针
    let x = 3;
    let ptr = &x as *const i32;

    unsafe {
        println!("{:?}", *ptr);
    }

}

#[test]
fn raw_pointer_slice() {
    let data = [1, 2, 3];
    let slice = &data[..];
    let ptr = slice as *const [i32];

    unsafe {
        let len = (*ptr).len();
        let data_ptr = (*ptr).as_ptr();

        println!("{:?}, {:?}", ptr, data_ptr);

        println!("{:?}, {:?}", len, *data_ptr.add(1));
    }

}