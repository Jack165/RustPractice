extern crate skeptic;

#[ignore]
#[test] fn readme_0() {
    let ref s = format!("{}", r####"#[macro_use] extern crate fixedvec;
"####);
    skeptic::rt::run_test(r#"/opt/rust/RustPractice/rust_f_os/target/bootimage/bootloader/x86_64-bootloader/release/build/fixedvec-c65de2ab1401159f/out"#, s);
}


#[test] fn readme_1() {
    let ref s = format!("{}", r####"#[macro_use] extern crate fixedvec;

use fixedvec::FixedVec;

fn main() {
    let mut preallocated_space = alloc_stack!([u8; 10]);
    let mut vec = FixedVec::new(&mut preallocated_space);
    assert_eq!(vec.len(), 0);

    vec.push_all(&[1, 2, 3]).unwrap();
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.as_slice(), &[1, 2, 3]);

    vec.map_in_place(|x: &mut u8| { *x *= 2 });
    assert_eq!(vec.as_slice(), &[2, 4, 6]);
}
"####);
    skeptic::rt::run_test(r#"/opt/rust/RustPractice/rust_f_os/target/bootimage/bootloader/x86_64-bootloader/release/build/fixedvec-c65de2ab1401159f/out"#, s);
}


