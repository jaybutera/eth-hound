Gives you a list of all your addresses that interacted with the target address.

```bash
cargo run -- \
--target 0x<addr of where you know you sent the funds> \
--addrs addrs.txt
--api_key <etherscan api key>
```

Put all your possible addresses in a file `addrs.txt`.

You can put the api key in `api_key.txt` and simply run


```bash
cargo run -- --target 0x123...
```
