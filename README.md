# iter_vals

This crate allows for on the fly iterator creation.

## Examples

You can use it to return an iterator over literals:

```
use iter_vals::iter_vals;

#[derive(Debug, PartialEq)]
enum Media {
    Book,
    Newspaper,
    TV,
    PC,
}

use Media::*;

impl Media {
    fn digital() -> impl Iterator<Item = Media> {
        iter_vals!(TV, PC)
    }

    fn non_digital() -> impl Iterator<Item = Media> {
        iter_vals!(Book, Newspaper)
    }
}

let mut digital = Media::digital();
assert_eq!(digital.next(), Some(TV));
assert_eq!(digital.next(), Some(PC));
assert_eq!(digital.next(), None);

let mut non_digital = Media::non_digital();
assert_eq!(non_digital.next(), Some(Book));
assert_eq!(non_digital.next(), Some(Newspaper));
assert_eq!(non_digital.next(), None);
```

You can conditionally include values.
This can be useful to return a variable number of values without having to allocate:

```
use iter_vals::iter_vals;

fn next_numbers(start: i32, include_first_number: bool) -> impl Iterator<Item = i32> {
    iter_vals!(
        [..= include_first_number; start + 1],
        (start + 2),
        (start + 3)
    )
}

let mut next_nums = next_numbers(5, true);
assert_eq!(next_nums.next(), Some(6));
assert_eq!(next_nums.next(), Some(7));
assert_eq!(next_nums.next(), Some(8));
assert_eq!(next_nums.next(), None);

let mut next_nums = next_numbers(5, false);
assert_eq!(next_nums.next(), Some(7));
assert_eq!(next_nums.next(), Some(8));
assert_eq!(next_nums.next(), None);
```

You can expand other iterators inside the iterator you return.
This can be especially useful, when dealing with `Option`s:

```
use iter_vals::iter_vals;

fn make_iter(num1: i32, num2: Option<i32>, num3: i32) -> impl Iterator<Item = i32> {
    iter_vals!(
        num1,
        [.. num2],
        num3
    )
}

let mut nums = make_iter(1, Some(2), 3);
assert_eq!(nums.next(), Some(1));
assert_eq!(nums.next(), Some(2));
assert_eq!(nums.next(), Some(3));
assert_eq!(nums.next(), None);

let mut nums = make_iter(1, None, 3);
assert_eq!(nums.next(), Some(1));
assert_eq!(nums.next(), Some(3));
assert_eq!(nums.next(), None);

let mut nums = iter_vals!(1, [.. vec![2, 2]], 3);
assert_eq!(nums.next(), Some(1));
assert_eq!(nums.next(), Some(2));
assert_eq!(nums.next(), Some(2));
assert_eq!(nums.next(), Some(3));
assert_eq!(nums.next(), None);
```

## Note

If you want to return computed values, you currently have to put them in parenthesis for that
to work, because of a limitation in the macro system.

This will **not** work:

```no_compile
use iter_vals::iter_vals;

let nums: Vec<_> = iter_vals!(1 + 1, 2 + 2, 3 + 3).collect();
assert_eq!(nums, vec![2, 4, 6]);
```

But this will:

```
use iter_vals::iter_vals;

let nums: Vec<_> = iter_vals!((1 + 1), (2 + 2), (3 + 3)).collect();
assert_eq!(nums, vec![2, 4, 6]);
```
