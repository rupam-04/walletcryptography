Take a private key from anywhere. You can also generate it using Python in terminal like:

```
>>> import secrets
>>> secrets.token_hex(32)
```

To generate an Ethereum public address run:

```
./walletcryptography ethereum <private key>
```

To generate a Bitcoin public address run:

```
./walletcryptography bitcoin <private key>
```

In both the cases a file will be created in the root directory having the details of the generated address and uncompressed public key
