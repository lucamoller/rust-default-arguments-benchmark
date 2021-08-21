pub struct MyFuncArgs<'a> {
  pub req1: &'a str,
  pub req2: &'a str,
  pub optional_args: MyFuncOptionalArgs<'a>,
}

pub struct MyFuncOptionalArgs<'a> {
  pub opt1: i32,
  pub opt2: &'a str,
  pub opt3: bool,
}

impl<'a> Default for MyFuncOptionalArgs<'a> {
  fn default() -> Self {
    MyFuncOptionalArgs {
      opt1: 123,
      opt2: "abc",
      opt3: false,
    }
  }
}

pub fn my_func(args: MyFuncArgs) -> bool {
  (args.req1.len() % 2 == 0)
    ^ (args.req2.len() % 2 == 0)
    ^ (args.optional_args.opt1 % 2 == 0)
    ^ (args.optional_args.opt2.len() % 2 == 0)
    ^ args.optional_args.opt3
}

#[inline(never)]
pub fn my_func_never_inlined(args: MyFuncArgs) -> bool {
  (args.req1.len() % 2 == 0)
    ^ (args.req2.len() % 2 == 0)
    ^ (args.optional_args.opt1 % 2 == 0)
    ^ (args.optional_args.opt2.len() % 2 == 0)
    ^ args.optional_args.opt3
}

pub fn exercise_my_func_calls(repetitions: i64) -> bool {
  let mut result = false;
  for i in 0..repetitions {
    result ^= my_func(MyFuncArgs {
      req1: "req1value_call1",
      req2: "req2value_call1",
      optional_args: MyFuncOptionalArgs::default(),
    });

    result ^= my_func(MyFuncArgs {
      req1: "req1value_call2",
      req2: "req2value_call2",
      optional_args: MyFuncOptionalArgs {
        opt1: 456 + (i % 2) as i32,
        opt2: "def",
        opt3: true,
        ..Default::default()
      },
    });

    result ^= my_func(MyFuncArgs {
      req1: "req1value_call3",
      req2: "req2value_call3",
      optional_args: MyFuncOptionalArgs {
        opt2: "ghi",
        ..Default::default()
      },
    });

    result ^= my_func(MyFuncArgs {
      req1: "req1value_call4",
      req2: "req2value_call4",
      optional_args: MyFuncOptionalArgs {
        opt3: true,
        opt1: 789 + (i % 3) as i32,
        ..Default::default()
      },
    });
  }
  result
}

pub fn exercise_my_func_never_inlined_calls(repetitions: i64) -> bool {
  let mut result = false;
  for i in 0..repetitions {
    result ^= my_func_never_inlined(MyFuncArgs {
      req1: "req1value_call1",
      req2: "req2value_call1",
      optional_args: MyFuncOptionalArgs::default(),
    });

    result ^= my_func_never_inlined(MyFuncArgs {
      req1: "req1value_call2",
      req2: "req2value_call2",
      optional_args: MyFuncOptionalArgs {
        opt1: 456 + (i % 2) as i32,
        opt2: "def",
        opt3: true,
        ..Default::default()
      },
    });

    result ^= my_func_never_inlined(MyFuncArgs {
      req1: "req1value_call3",
      req2: "req2value_call3",
      optional_args: MyFuncOptionalArgs {
        opt2: "ghi",
        ..Default::default()
      },
    });

    result ^= my_func_never_inlined(MyFuncArgs {
      req1: "req1value_call4",
      req2: "req2value_call4",
      optional_args: MyFuncOptionalArgs {
        opt3: true,
        opt1: 789 + (i % 3) as i32,
        ..Default::default()
      },
    });
  }
  result
}
