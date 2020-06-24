# bulkreq

A fast tool to make async requests to a list of urls passed from stdin and returns 
```
status_code url mime_type protocol server md5sum_of_response --> redirect[if any]
```

### How to install?
- Run the following commands to install rust and use nightly toolchain as default

```
curl https://sh.rustup.rs -sSf | sh
rustup install nightly
rustup toolchain nightly
```
- clone this repo and compile
```
git clone https://github.com/0xatul/bulkreq && \
cd bulkreq && \
cargo build --release  
```
- move the binary to your $PATH: 
```
sudo mv target/release/bulkreq /usr/local/bin
```

### usage 
-flags
```
❯ ./bulkreq --help
bulkreq v0.1.0
Make lots of requests quickly

USAGE:
    bulkreq -f <list_of_urls> or bulkreq < /path/to/urls.txt

FLAGS:
    -f, --file       read urls from a file
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    print some extra information
```
- On its own
```
bulkreq < /path/to/urls.txt
```
- with others tools 
```
assetfinder --subs-only target.com | httprobe --prefer-https | bulkreq 
```
```
echo 'target.com' | gau | bulkreq | egrep '200|405'
```

## TO-DO: 
- add arguement parsing 
- add support to read a file containing urls and do the thing 

Thanks to [junn](https://github.com/junnlikestea/) for helping me around things :D
