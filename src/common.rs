pub fn my_func(req1: &str, req2: &str, opt1: i32, opt2: &str, opt3: bool)
    -> bool
{
  (req1.len() % 2 == 0)
    ^ (req2.len() % 2 == 0)
    ^ (opt1 % 2 == 0)
    ^ (opt2.len() % 2 == 0)
    ^ opt3
}

