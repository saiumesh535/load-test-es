# loadtest-es
Loading test data in elastic search

##Loading test data into Elastic search from json files

Commands to run

**Data will be inserted from json files on data folder in sort.**

To insert all the json file from **data** folder run following command

```cmd
cargo run
```

<!-- To insert single json file from **data** folder run following Command

```cmd
FILE_NAME=a.json go run main.go
``` -->

##config

to connect Elastic search and it's index, this will use **config.json** file as follows

```js
{
    "URL": "https://some-es-url.com",
    "Index": "someesindex" // index doesn't support some characters - so don't use 😂
}
```