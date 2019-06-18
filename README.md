# iota-address-finder
Based on Thoralf-M's `custom-iotaaddress-finder`, which you can find [here](https://github.com/Thoralf-M/custom-iotaaddress-finder).

In order to compile this tool please switch to this somewhat older nightly toolchain as recommended by Thoral-M:
 `rustup default nightly-2019-04-30`. After cloning this repo and `cd`ing into it you compile it with `cargo b --release`. Don't forget to switch back to your default toolchain afterwards, e.g. by typing `rustup default stable`.

## About and Disclaimer
With this tool you can find IOTA addresses, that start with a certain combination of trytes, like JEANLUC999AKEJJ9MN................. either by specifying a custom seed or by letting the tool generate (<- oops, the bad word) a seed for you. Currently the tool only sees addresses as valid findings, if they **start** with the specified tryte string. Contact me on the IOTA Discord server (/alex/), if you want different options, like custom tryte combinations **anywhere** in the address, or at the beginning or at the end. Or better, implement it yourself and make a pull request. This will highly increase the success rate of finding a valid address. If your wished tryte combination is too long, then better prepare for some energy costs and waiting time. Having said all that, this tool is just for **fun**, don't use it for anything serious. 

## Closing Words 
Made with love (for Rust) and to show Thoralf what kind of a STRG-F-programmer he is xD.
