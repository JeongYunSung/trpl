pub fn raw_pointer() {
    let address = 0x012345;
    let r = address as *const i32; // Primitive Type이기에 포인터 변수가 아니어도 주소를 찾아올 수 있음 정확히는 값을 주소로 치환해버림 ( 최적화 )

    let address1 = 0x012345;
    let r1 = &address1 as *const i32; // 실제 address1이 저장된 주소를 가져 옴

    let address2 = String::from("Test");
    let r2 = &address2 as *const String;

    println!("add : {:?}, {:?}", address, r);
    println!("add1 : {:?}, {:?}, {:?}", address1, r1, unsafe { *r1 }); // Copy Trait이기에 값을 복사해서 넘겨줌
    unsafe {
        println!("add2 : {:?}, {:?}, {:?}", address2, r2, *r2); // Copy Trait이 아니기에 unsafe 내에서 작업해야 함
    }
}

static mut TEST: i32 = 5;

pub fn unsafe_static() {
    unsafe {
        println!("{}", TEST);
        TEST += 1;
        println!("{}", TEST);
    }
}