# Sanity RS

The open source client for consuming https://sanity.io with Rust, based on `reqwest`.

The goal behind this project is to provide a relatively low level wrapper for consuming Sanity-powered APIs. The first goal is to make sure all bases are covered for running bare [GROQ query strings](https://www.sanity.io/docs/overview-groq), then I'd like to add in support for some sort of ORM to run queries against (If you know any easy way to implement this please let me know).

Stretch goal would be adding a higher level GraphQL consumer to make Sanity operations a breeze, but there are other GraphQL projects for Rust so that's not a high priority :)

### This project is in alpha, so very limited features are available.

## Getting started

Add the `sanity` crate to you dependencies:

```toml
[dependencies]
sanity = "0.1.0"
```

Or directly via github:

```toml
[dependencies]
sanity = { git = "https://github.com/DukeFerdinand/sanity.rs" }
```

Then include as you would any other external crate:

```rust
// main.rs or wherever

...
extern crate sanity;
...

fn main {
  ...
}
```

## Usage

As of right now (`v0.1.0`), only `get` requests are supported.

### `GET` request

```rust
extern crate sanity;
use sanity::helpers::get_json;

fn main() {
  // Ideally you would pull these values from an env of some sort
  // PLEASE do not use bare strings in your project
  let mut sn = sanity::create(
    "proj_id",                // Sanity project ID to use
    "data_set",               // Data set to query. i.e. "development"
    "Long_string_for_token",  // Bearer token
    false,                    // Use prod cdn or not
  );
  let res = sn.get(&String::from("*[_type == 'recipe']"));
  if res.is_ok() {
    println!("{:?}", get_json(res.unwrap()));
  }
}
```

## Contributing

I'm admittedly pretty new to Rust, so if you see anything you'd like to change or anything you'd like to see added, _please_ open a feature request in the github issues :)

I'm open to accepting any and all PRs as long as they fit the project and contain good code!
