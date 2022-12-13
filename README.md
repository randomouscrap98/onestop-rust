# What is this?

I needed a way to time code segments from various different parts of my programs but track
them with a singular list for later output. For instance, I time render code, database code,
and other code during a web request then include the output in the page itself for debugging.

### Example:

```rust
let mut stopwatch = onestop::OneDuration::new(format!("postrender#789"));
some_long_task();
stopwatch.finish();
println!("It took {:?}", stopwatch.duration);
```

## OK but...?

You could've accomplished the above with just `std::time::Instant` (which this code uses),
but now let's use our threadsafe shared list to make it more useful:

```rust
struct Service1 {
    timings: OneList<OneDuration> // The other thing in this crate
}
struct Service2 {
    timings: OneList<OneDuration>
}

impl Service1 {
    fn do_stuff(&self) -> {
        onestop!{(self.timings, "service1_dostuff") => {
           assert_eq!(4, 4);
        }};
    }
}

impl Service2 {
    fn do_stuff(&self) -> {
        onestop!{(self.timings, "service2_dostuff") => {
            println!("did service2 things!");
        }};
    }
}

// Clones of OneList are threadsafe reference-counted pointers to the original list
// stored here in 'all_timings'
let all_timings = OneList::<OneDuration>::new();
let service1 = Service1 { all_timings.clone() };
let service2 = Service2 { all_timings.clone() };

service1.do_stuff();
service2.do_stuff();

// Both go to the same list, which you can then print out somewhere later
assert_eq(2, all_timings.list_copy().len());
```

## No but for real

Yes, the library is basically useless!