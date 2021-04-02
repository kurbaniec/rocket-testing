# rocket-testing



## Notes

Install Diesel CLI on Windows

https://www.reddit.com/r/rust/comments/g44xae/install_latest_rust_diesel_in_windows_10_and_fix/

https://steemit.com/programming/@mrblueberry/installing-rust-and-diesel-for-rocket-on-windows-10

Install only MySQL feature

```
cargo install diesel_cli --no-default-features --features mysql
```

Create `.env` File

```
echo "MYSQL_DATABASE_URL=mysql://root@127.0.0.1:3306/diesel_test" > .env
```

See [here](https://github.com/diesel-rs/diesel/blob/master/.env.sample) for more examples.

