# Rust CLI: According to the user's date of birth, identify the user's constellation information.
## Description
This project is used to identify the user's constellation information based on the user's date of birth.

## Usage
- User can type in `cargo run -- subcommand -- commandLineArg1 commandLineArg2` in command line. The first command line argument is the user's month of birth. The second command line argument is the user's date of birth.

- Here is an example:
    ```
    cargo run -- constellation -- 9 10
    cargo run -- constellation -- 8 13
    ```
- Output examples: 
  
  <img width="826" alt="Screen Shot 2023-02-15 at 04 28 35" src="https://user-images.githubusercontent.com/93239143/218994763-d272d452-94df-49fa-ba4e-b5318494a728.png">
  
    - This means that those born on September 10 are a Virgo and those born on August 13 are a Leo.

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
