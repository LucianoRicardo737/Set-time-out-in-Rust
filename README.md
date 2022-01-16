# Set_time_out by UniCorp and LabLer for Rust

Add to cargo.toml the dependency

```[dependencies]
set_time_out = "0.2.0"
```

Easy peasy. 

Import the function  and used.

```use set_time_out::set_time_out;```

and invoque

```
fn say_hello() {
    println!("Hello");
}

set_time_out(1500);

say_hello();
```
done!.

But if you need pass one function, here the correct way. It's the same, but more organized.

```
fn say_hello() {
    println!("Hello");
}

set_time_out_callback(1000, Same(say_hello));
```

or without callback

```
fn say_hello() {
    println!("Hello");
}

set_time_out_callback(1000, None);
```


The time is in milliseconds. Have nice day. 

