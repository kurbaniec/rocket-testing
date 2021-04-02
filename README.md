# rocket-testing



## Notes

Install Diesel CLI on Windows

Follow [this](https://steemit.com/programming/@mrblueberry/installing-rust-and-diesel-for-rocket-on-windows-10) tutorial. Based on [this](https://www.reddit.com/r/rust/comments/g44xae/install_latest_rust_diesel_in_windows_10_and_fix/) thread. Issue tracker is [here](https://github.com/diesel-rs/diesel/issues/1286).

Install the C-Connector from [here](https://downloads.mysql.com/archives/c-c/) to a directory and it to the path like this:

```
setx MYSQLCLIENT_LIB_DIR "<your path>"
```

Now you can install Diesel CLI with only the MySQL feature

```
cargo install diesel_cli --no-default-features --features mysql
```

After installation I needed to put the `libmysql.dll` from the original C-Connector also to the folder of the Diesel binary in `C:\Users\<user>\.cargo\bin` in order to work.



Setting Database

Not working under Windows? [See](https://users.rust-lang.org/t/diesel-error-message-actix/48320/3)

---

Create `.env` File

```
echo "MYSQL_DATABASE_URL=mysql://root@127.0.0.1:3306/diesel_test" > .env
```

See [here](https://github.com/diesel-rs/diesel/blob/master/.env.sample) for more examples.

---

Setup Password

[See](https://stackoverflow.com/a/59218981/12347616)

```
ALTER USER 'root'@'localhost' IDENTIFIED WITH mysql_native_password BY '1234';
```

Add `url` manually in CLI

```
diesel setup --database-url "mysql://root:1234@127.0.0.1:3306/diesel_test"
```



