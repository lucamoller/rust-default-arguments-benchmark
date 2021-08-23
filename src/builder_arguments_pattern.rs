use crate::common;

pub struct MyFuncArgs<'a> {
  req1: &'a str,
  req2: &'a str,
  opt1: i32,
  opt2: &'a str,
  opt3: bool,
}

const OPT1_DEFAULT_VALUE: i32 = 123;
const OPT2_DEFAULT_VALUE: &str = "abc";
const OPT3_DEFAULT_VALUE: bool = false;

struct MyFuncArgsBuilder<'a> {
  req1: &'a str,
  req2: &'a str,
  opt1: Option<i32>,
  opt2: Option<&'a str>,
  opt3: Option<bool>,
}

impl<'a> MyFuncArgsBuilder<'a> {
  pub fn new(req1: &'a str, req2: &'a str) -> MyFuncArgsBuilder<'a> {
    MyFuncArgsBuilder {
      req1,
      req2,
      opt1: None,
      opt2: None,
      opt3: None,
    }
  }

  pub fn opt1(self, opt1: i32) -> Self {
    let mut new = self;
    new.opt1 = Some(opt1);
    new
  }

  pub fn opt2(self, opt2: &'a str) -> Self {
    let mut new = self;
    new.opt2 = Some(opt2);
    new
  }

  pub fn opt3(self, opt3: bool) -> Self {
    let mut new = self;
    new.opt3 = Some(opt3);
    new
  }

  pub fn build(self) -> MyFuncArgs<'a> {
    MyFuncArgs {
      req1: self.req1,
      req2: self.req2,
      opt1: match self.opt1 {
        Some(opt1) => opt1,
        None => OPT1_DEFAULT_VALUE,
      },
      opt2: match self.opt2 {
        Some(opt2) => opt2,
        None => OPT2_DEFAULT_VALUE,
      },
      opt3: match self.opt3 {
        Some(opt3) => opt3,
        None => OPT3_DEFAULT_VALUE,
      },
    }
  }
}

pub fn my_func(args: MyFuncArgs) -> bool {
  common::my_func(args.req1, args.req2, args.opt1, args.opt2, args.opt3)
}

#[inline(never)]
pub fn my_func_never_inlined(args: MyFuncArgs) -> bool {
  my_func(args)
}

pub fn exercise_my_func_calls(repetitions: i64) -> bool {
  let mut result = false;
  for i in 0..repetitions {
    result ^= my_func(MyFuncArgsBuilder::new("req1value_call1", "req2value_call1").build());

    result ^= my_func(
      MyFuncArgsBuilder::new("req1value_call2", "req2value_call2")
        .opt1(456 + (i % 2) as i32)
        .opt2("def")
        .opt3(true)
        .build(),
    );

    result ^= my_func(
      MyFuncArgsBuilder::new("req1value_call3", "req2value_call3")
        .opt2("ghi")
        .build(),
    );

    result ^= my_func(
      MyFuncArgsBuilder::new("req1value_call2", "req2value_call2")
        .opt3(true)
        .opt1(789 + (i % 3) as i32)
        .build(),
    );
  }
  result
}

pub fn exercise_my_func_never_inlined_calls(repetitions: i64) -> bool {
  let mut result = false;
  for i in 0..repetitions {
    result ^=
      my_func_never_inlined(MyFuncArgsBuilder::new("req1value_call1", "req2value_call1").build());

    result ^= my_func_never_inlined(
      MyFuncArgsBuilder::new("req1value_call2", "req2value_call2")
        .opt1(456 + (i % 2) as i32)
        .opt2("def")
        .opt3(true)
        .build(),
    );

    result ^= my_func_never_inlined(
      MyFuncArgsBuilder::new("req1value_call3", "req2value_call3")
        .opt2("ghi")
        .build(),
    );

    result ^= my_func_never_inlined(
      MyFuncArgsBuilder::new("req1value_call2", "req2value_call2")
        .opt3(true)
        .opt1(789 + (i % 3) as i32)
        .build(),
    );
  }
  result
}
