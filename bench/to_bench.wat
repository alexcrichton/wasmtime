(module
(memory 1)

  (func (export "0")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 0))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "1")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 1))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "2")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 2))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "3")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 3))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "4")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 4))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "5")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 5))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "6")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 6))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "7")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 7))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "8")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 8))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "9")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 9))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "10")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 10))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "11")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 11))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "12")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 12))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "13")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 13))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "14")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 14))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "15")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 15))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "16")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 16))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "17")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 17))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "18")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 18))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "19")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 19))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "20")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 20))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "21")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 21))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "22")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 22))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "23")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 23))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "24")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 24))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "25")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 25))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "26")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 26))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "27")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 27))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "28")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 28))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "29")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 29))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "30")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 30))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "31")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 31))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "32")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 32))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "33")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 33))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "34")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 34))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "35")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 35))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "36")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 36))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "37")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 37))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "38")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 38))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "39")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 39))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "40")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 40))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "41")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 41))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "42")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 42))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "43")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 43))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "44")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 44))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "45")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 45))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "46")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 46))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "47")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 47))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "48")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 48))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "49")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 49))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "50")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 50))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "51")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 51))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "52")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 52))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "53")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 53))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "54")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 54))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "55")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 55))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "56")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 56))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "57")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 57))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "58")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 58))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "59")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 59))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "60")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 60))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "61")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 61))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "62")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 62))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "63")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 63))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  

  (func (export "64")
      (param $cnt i64)
      (param $src i32)
      (param $dst i32)

    loop $loop
      (memory.copy
        (local.get $dst)
        (local.get $src)
        (i32.const 64))
      (local.tee $cnt (i64.sub (local.get $cnt) (i64.const 1)))
      i64.eqz
      if
      else
        br $loop
      end
    end
  )

  
)
