Take a private key from anywhere. You can use an online service that uses BIP-39 or also generate it using Python in terminal like (the latter should only be used for testing):

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

In both the cases a file will be created in the root directory having the details of the generated address and uncompressed public key. You also don't need to have an internet connection while doing this.
