# Fahrenheit / Celcius converer

This is a simple project based on the assignment of [Chapter 3 of the Rust Book](https://rust-book.cs.brown.edu/ch03-05-control-flow.html#summary).
This program converts degree fahrenheit to degree celcius and vice-versa.

# How to use it

> [!IMPORTANT]
> For NixOS users, please run `nix-develop .`, for other users, make sure you have setup your Rust 
> development environment correctly, otherwise please check [the official documentation](https://rust-lang.org/tools/install/).


> [!NOTE]
> You can separately build by invoking `cargo build`.

```bash
cargo run
```

# What should I expect

You should expect to see on your console the following (please follow the prompt):

```bash
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/fahrenheit_celcius_converter`
*****************************************
Fahrenheit <--> Celcius converter program
*****************************************
Select the conversion type (d/D: degree -> fahrenheit, f/F: fahrenheit -> degree):
d
Enter the value you want to convert (a floating value is acceptable):
10
Conversion result: 10 °C = 50 °C
```

Thank you!
