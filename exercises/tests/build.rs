// build.rs

fn main() {
    // 给 tests7 用：设置环境变量 TEST_FOO 为当前时间戳
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 给 tests8 用：开启一个编译条件，相当于有个 feature = "pass"
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
