## Get started 
(e.g. on MacOS without Rust installed yet)

`brew install rust`

## Compile

`cargo build --target-dir target`

## Run

`./target/debug/emoji-info "😎"`

or 

`./target/debug/emoji-info "U+1FAE9"`


## Results

If everything is compiled and installed properly this:

```
%> ./target/debug/emoji-info "😎"
```

Should return something like:

```
Emoji: 😎
Name: smiling face with sunglasses
Shortcodes: ["sunglasses"]
Unicode Version: UnicodeVersion { major: 1, minor: 0 }
Group: SmileysAndEmotion
```
