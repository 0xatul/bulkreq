# bulkreq

A fast tool to make async requests to a list of urls passed from stdin and returns 
```
status_code url mime_type protocol server --> redirect[if any]
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

Thanks to [junn](https://github.com/junnlikestea/)
