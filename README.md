# hcskr
rust 언어 교육청 자가진단 라이브러리

## Install
```toml
[dependencies]
hcskr = "0.1.0"
```

## Example
```rust
extern crate hcskr;

fn main() {
    let result = hcskr::self_check("이름", "생년월일", "지역", "학교", "학교 종류", "비밀번호", "유저명").unwrap();
    println!("{:?}", result);
}
```