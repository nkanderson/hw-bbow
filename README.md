# HW3: Big Bag of Words (BBOW)

**Author:** Niklas Anderson / forked from Bart Massey

**Description:** A library that can be used to generate a BBOW by processing documents stored as strings in memory. To utilize storage efficiently and reduce expensive string copying, uses the [COW type](https://doc.rust-lang.org/std/borrow/enum.Cow.html) to store a string slice from the original text when possible.

## Example
```rust
# use bbow::Bbow;

let bbow = Bbow::new().extend_from_text("Hello world.");
// Number of unique words
assert_eq!(2, bbow.len());
// Number of matching words
assert_eq!(1, bbow.match_count("hello"));

let bbow = bbow.extend_from_text("adding twice: banana banana");
assert_eq!(2, bbow.match_count("banana"));

// Total sum of all words
assert_eq!(6, bbow.count());

assert_eq!(5, bbow.len());

assert_eq!(false, bbow.is_empty());
```

## Run the library tests

To run the library tests, including assertions in the rustdoc examples, use the following command from within the project root directory:
```sh
$ cargo test
```

## Additional commands

The following are additional helpful commands to run during development.

Check for errors:
```sh
$ cargo check
```

Run code linter:
```sh
$ cargo clippy
```

Format code:
```sh
$ cargo fmt
```
**Note:** This can be run with the `--check` option to check for formatting changes without modifying existing code.

As noted above, command to run tests:
```sh
$ cargo test
```

Build project documentation:
```sh
$ cargo doc
```
Output is placed in `target/doc`. Use the `--open` flag to open the docs in the default browser.

## Issues

Future work may include breaking the `extend_from_text` method into a number of smaller helper utilities. It is a relatively straightforward chain of method or utility function calls at this point, but if the definition of a valid word changes, the readability may be reduced depending on the required changes.

Another possible way to improve this method and the `BBOW`'s memory usage would be to replace any owned `String`s with the string slice, if the same word is encountered later. For example, if the word `Banana` is encountered before the word `banana`, the current implementation will maintain the original owned `COW` `String` instead of replacing it with the `&str` present in the source text.
