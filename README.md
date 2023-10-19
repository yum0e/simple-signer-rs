## Simple signer

Easily sign a message with an ethereum private key. The signature can then be used to verify the message was signed by the owner of the private key.

It is not recommended to use this for anything other than testing purposes. The best security is to never expose your private key.

## Usage

```bash
cargo run "<message>" "<private-key>"
```

## Example

```bash
cargo run "Hello World" "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"

# Output
# Address: 0xf39f…2266
# {"signature": "65e72b1cf8e189569963750e10ccb88fe89389daeeb8b735277d59cd6885ee823eb5a6982b540f185703492dab77b863a88ce01f27e21ade8b2879c10fc9e6531c", "message": "Hello World"}
```
