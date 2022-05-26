## jsty

JSON to TypeScript type converter

```shell
$ ./target/release/jsty <<EOF
{
  "name": "reon",
  "age": 25,
  "jobs": [
    "YouTuber",
    "TikToker"
  ],
  "address": {
    "prefecture": "東京都",
    "city": "港区"
  }
}
EOF
{
    address: {
        city: string,
        prefecture: string,
    },
    age: number,
    jobs: string[],
    name: string,
} 
```