# notisbak

A back for notis

## To work

In order to work notisbak

```
mkdir notisbak
cd notisbak
mkdir pgadmin posgres_data
sudo chown -R 5050 pgadmin
sudo chown -R 999 posgtres_data
wget https://raw.githubusercontent.com/atareao/notisbakpg/main/docker-compose.yml
wget https://raw.githubusercontent.com/atareao/notisbakpg/main/sample.env
cp sample.env .env
docker-compose up -d
```

## Some notes to work with it

Dependecies:

* `sqlx-cli`. To do all work

Create the file `.env` with the following content,

```
DATABASE_URL=sqlite:notis.db
```

To create the database run,

```
sqlx database create
```

and to remove the database run,

```
sqlx database drop
```

In order to create migrations,

```
sqlx migrate add <name>
```

And if you want to create a migration with revert,

```
sqlx migrate add -r <name>
```

To run a migration,


```
sqlx migrate run
```

To revert a migration

```
sqlx migrate revert
```

To enable guilding in "offline mode" with "query!()",

```
cargo sqlx prepare
```

To check,

```
cargo sqlx prepare --check
```

