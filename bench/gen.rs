const N: usize = 64;

fn main() {
    println!("(module");
    println!("(memory 1)");
    for i in 0..=N {
        println!(
            r#"
  (func (export "{i}")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const {i}))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  "#
        );
    }
    println!(")");
}
