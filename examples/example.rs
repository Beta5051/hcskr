extern crate hcskr;

fn main() {
    let result = hcskr::self_check("이름", "생년월일", "지역", "학교", "학교 종류", "비밀번호", "유저명").unwrap();
    println!("{:?}", result);
}