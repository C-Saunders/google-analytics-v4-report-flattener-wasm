# google-analytics-v4-report-flattener-wasm

Converts Google Analytics API v4 reports to flat/delimited data.

This is a wrapper around [this Rust package](https://crates.io/crates/ga-v4-flattener) to make it available as a WebAssembly package. Credit to [wasm-pack](https://github.com/rustwasm/wasm-pack) for doing the hard work for the wasm compilation.

## API

### `toDelimited(data: string, delimiter: string): Array<string>`
### `parsedtoFlatJson(data: Array<Report>, delimiter: string): Array<string>`
```ts
const { toDelimited, parsedToDelimited } = require('google-analytics-v4-report-flattener-wasm')
const data = require('./test.json')

// if you did not parse the response from Google, use this
// (you wouln't need to stringify first)
toDelimited(JSON.stringify(data), ',')

// if you parsed the JSON response in JavaScript already
// for example, if the library does it or you need to inspect it
parsedToDelimited(data, ',')

// the result is an array in either case, starting with v0.2
[ '"ga:deviceCategory","ga:sessions","ga:bounces"\n"desktop",25,17\n"mobile",2,2\n',
  '"ga:country","ga:sessions","ga:bounces"\n"Azerbaijan",1,0\n"France",18,11\n"Japan",4,4\n"Switzerland",1,1\n"United States",3,3\n' ]
```

### `toFlatJson(data: string): Arrray<{ [field: string]: number | string }>`
### `parsedToFlatJson(data: Array<Report>): Arrray<{ [field: string]: number | string }>`
```ts
const { toFlatJson, parsedToFlatJson } = require('google-analytics-v4-report-flattener-wasm')
const data = require('./test.json')

// if you did not parse the response from Google, use this
// (you wouln't need to stringify first)
toFlatJson(JSON.stringify(data))

// if you parsed the JSON response in JavaScript already
// for example, if the library does it or you need to inspect it
parsedToFlatJson(data)

// the result is an array in either case, starting with v0.2
[
  [{
      'ga:bounces': 17,
      'ga:deviceCategory': 'desktop',
      'ga:sessions': 25
    },
    {
      'ga:bounces': 2,
      'ga:deviceCategory': 'mobile',
      'ga:sessions': 2
    }
  ],
  [{
      'ga:bounces': 0,
      'ga:country': 'Azerbaijan',
      'ga:sessions': 1
    },
    {
      'ga:bounces': 11,
      'ga:country': 'France',
      'ga:sessions': 18
    },
    {
      'ga:bounces': 4,
      'ga:country': 'Japan',
      'ga:sessions': 4
    },
    {
      'ga:bounces': 1,
      'ga:country': 'Switzerland',
      'ga:sessions': 1
    },
    {
      'ga:bounces': 3,
      'ga:country': 'United States',
      'ga:sessions': 3
    }
  ]
]
```

## Building from source
You'll need [wasm-pack](https://github.com/rustwasm/wasm-pack) and its dependencies, then `wasm-pack build --target nodejs`

## Contributing
Issues and pull requests welcome. Please be nice.

## License
[MIT](https://opensource.org/licenses/MIT)
