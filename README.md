# trmnl-bouldering

A trmnl plugin to show how busy the local bouldering gym is. 

## Cloudflare Workers local dev setup

```
cd ./trmnl-bouldering-worker
npx wrangler dev
```

Then test in another terminal window with a GET request:
```
curl "localhost:8787?id=f40dc9c0cbd9d67d35431dcd0581baac" \
    -H "content-type:application/json"
```

or a POST request:
```
curl localhost:8787 \
    -d "{\"id\": \"f40dc9c0cbd9d67d35431dcd0581baac\"}" \
    -H "content-type:application/json"
```

## Acknowledgements

Thanks to https://github.com/eiri/climber-count for parsing inspiration.
