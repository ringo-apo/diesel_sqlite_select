# diesel_sqlite_select
# SQLiteの作成

```
sqlite3 sample.db
CREATE TABLE memos ( id INTEGER NOT NULL PRIMARY KEY, comment VARCHAR);
insert into memos (comment) values ('comment1');
.exit
```

# コンパイル, 実行

```
cargo build
cargo run
```

```
Displaying 1 memos
id | comment
1 | comment1
```
