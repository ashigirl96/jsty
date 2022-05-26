## jsty

JSON to TypeScript type converter

### Default

```shell
$ ./target/release/jsty Personal <<EOF
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
interface Personal {address: {city: string;prefecture: string;};age: number;jobs: string[];name: string;}
```

### Pretty 

```shell
$ ./target/debug/examples/pretty  Personal <<EOF
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
interface Personal {
    address: {
        city: string;
        prefecture: string;
    };
    age: number;
    jobs: string[];
    name: string;
}
```