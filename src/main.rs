fn main() {
    for f in cranelift_assembler_x64::generated_files() {
        dbg!(&f);
        if f.extension().and_then(|s| s.to_str()) == Some("isle") {
            assert!(f.exists());
        }
    }
}
