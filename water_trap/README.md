## Water Trap
### Usage
* Install [rust](https://www.rust-lang.org/en-US/install.html)
* Install [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
* Build the program by running `cargo build --release`
* To run the program using randomized bin heights, execute `./target/release/water_trap`
* To run the program against a file with bin heights data, execute `./target/release/water_trap -i <INPUT_FILE_PATH>`

### Sample execution

```
$ ./target/release/water_trap
No input file passed, using a random heights vector of size = 1000000
Bins = 1000000, Result = 499785509, Time taken = PT0.001608529S

$ ./target/release/water_trap -i bins.txt
Bins = 10000, Result = 52848147, Time taken = PT0.000021007S
```
