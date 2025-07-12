# trmnl-bouldering

A trmnl plugin to show how busy the local bouldering gym is. 

# Installation

## Dashboard UI

It's available as a Community Recipe for installation [on your TRMNL here][link_plugin].

## Manually

[Create a new Private Plugin](https://usetrmnl.com/plugin_settings/new?keyname=private_plugin):
- Strategy: `polling`
- Polling URL(s): `https://trmnl-bouldering-worker.hello-a31.workers.dev?id={{ id }}&gym_id={{ gym_id }}`
- Polling Verb: `GET`
- Polling Headers: `&content-type=application/json`
- Form Fields:
```
- keyname: id
  name: RockGymPro ID
  field_type: string
  placeholder: {{ Your local gym's RGP ID }}

- keyname: gym_id
  name: Gym (facility) ID
  field_type: string
  optional: true
  default: ""
  placeholder: {{ Your local gym's RGP Gym ID - you can probably leave this blank }}
```

## local trmnl layout setup

```
docker run \
    -p 4567:4567 \
    -v ./:/plugin \
    -e "id=f40dc9c0cbd9d67d35431dcd0581baac" \
    trmnl/trmnlp
```

## local Cloudflare Workers dev setup

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

[link_plugin]: https://usetrmnl.com/recipes/91950
