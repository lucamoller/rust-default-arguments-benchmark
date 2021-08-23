use crate::common::my_func;

#[inline(never)]
pub fn my_func_never_inlined(req1: &str, req2: &str, opt1: i32, opt2: &str, opt3: bool)
    -> bool
{
    my_func(req1, req2, opt1, opt2, opt3)
}

pub fn exercise_my_func_calls(repetitions: i64) -> bool {
  let mut result = false;
  for i in 0..repetitions {
    result ^= my_func("req1value_call1", "req2value_call1", 123, "abc", false);

    result ^= my_func(
      "req1value_call2",
      "req2value_call2",
      456 + (i % 2) as i32,
      "def",
      true,
    );

    result ^= my_func("req1value_call3", "req2value_call3", 123, "ghi", false);

    result ^= my_func(
      "req1value_call4",
      "req2value_call4",
      789 + (i % 3) as i32,
      "abc",
      true,
    );
  }
  result
}

pub fn exercise_my_func_never_inlined_calls(repetitions: i64) -> bool {
  let mut result = false;
  for i in 0..repetitions {
    result ^= my_func_never_inlined("req1value_call1", "req2value_call1", 123, "abc", false);

    result ^= my_func_never_inlined(
      "req1value_call2",
      "req2value_call2",
      456 + (i % 2) as i32,
      "def",
      true,
    );

    result ^= my_func_never_inlined("req1value_call3", "req2value_call3", 123, "ghi", false);

    result ^= my_func_never_inlined(
      "req1value_call4",
      "req2value_call4",
      789 + (i % 3) as i32,
      "abc",
      true,
    );
  }
  result
}
