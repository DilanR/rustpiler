export const EXAMPLES = {
  hello: {
    category: "Features",
    name: "Hello World",
    code: `fn main() -> () {
    println!("Hello World!")
}`,
  },

  variables: {
    category: "Features",
    name: "Variables",
    code: `fn main() -> i32 {
    let mut x = 1;
    x = x + 1;
    x
}`,
  },

  function_call: {
    category: "Features",
    name: "Function Call",
    code: `fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() -> i32 {
    add(1, 2)
}`,
  },

  while_loop: {
    category: "Features",
    name: "While Loop",
    code: `fn main() -> i32 {
    let mut x = 0;

    while x < 10 {
        x = x + 1;
    }

    x
}`,
  },

  type_mismatch: {
    category: "Diagnostics",
    name: "Type Mismatch",
    code: `fn main() -> i32 {
    ()
}`,
  },

  immutable_assignment: {
    category: "Diagnostics",
    name: "Immutable Assignment",
    code: `fn main() -> i32 {
    let x = 1;
    x = 2;
    x
}`,
  },

  unknown_variable: {
    category: "Diagnostics",
    name: "Unknown Variable",
    code: `fn main() -> i32 {
    y
}`,
  },

  unknown_function: {
    category: "Diagnostics",
    name: "Unknown Function",
    code: `fn main() -> i32 {
    foo()
}`,
  },

  sum_loop: {
    category: "Programs",
    name: "Sum Loop",
    code: `fn main() -> i32 {
    let mut sum = 0;
    let mut i = 0;

    while i < 10 {
        sum = sum + i;
        i = i + 1;
    }

    sum
}`,
  },

  max_of_two: {
    category: "Programs",
    name: "Max Of Two",
    code: `fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn main() -> i32 {
    max(10, 20)
}`,
  },

  gcd: {
    category: "Programs",
    name: "GCD",
    code: `fn gcd(a: i32, b: i32) -> i32 {
    let mut x = a;
    let mut y = b;

    while !(x == y) {
        if x > y {
            x = x - y;
        } else {
            y = y - x;
        }
    }

    x
}

fn main() -> () {
    let result = gcd(48, 18);
    println!("GCD: {}",result);
}
`,




  },
} as const;
