# CLI Utility for yabf



## Running

- Install Rust from https://www.rust-lang.org/
```
git clone --recurse-submodules https://github.com/serd223/yabf-cli
cd yabf-cli
cargo run
```


## Example Usage
```
>>begin
1 ++++.
2 end
>>set
>>dbg
>>n
Current Instruction: +; Program Counter: 1
>>d
 _________________________________________
|[ 1 ]|  0  |  0  |  0  |  0  |  0  |  0  |
 -----------------------------------------
>>no
The Program has ended.
Program Output:
♦
```
