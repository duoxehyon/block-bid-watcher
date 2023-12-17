# Block Builder Bids Poller 
This program polls for the latest block values for each relay submitted by the builder. Example on how to use in `src/main.rs`.

# Location, IP Values for relays

## relay.ultrasound.money

Ultrasound.money does not use any Content Delivery Networks (CDNs). A quick DNS query give the IP address 147.135.143.8, which is located in France. Detailed info is as follows:
```{
    "query": "147.135.143.8",
    "continent": "Europe",
    "continentCode": "EU",
    "country": "France",
    "countryCode": "FR",
    "region": "HDF",
    "regionName": "Hauts-de-France",
    "city": "Roubaix",
    "district": "",
    "zip": "59100",
    "lat": 50.6916,
    "lon": 3.20151,
    "timezone": "Europe/Paris",
    "offset": 3600,
    "currency": "EUR",
    "isp": "OVH SAS",
    "org": "OVH",
    "as": "AS16276 OVH SAS",
    "asname": "OVH",
    "mobile": false,
    "proxy": false,
    "hosting": true
}
```

## agnostic-relay.net
The Agnostic relay, which also does not use CDNs, has its the IP 13.38.156.156. This IP corresponds to an AWS instance based in France:
```
{
    "query": "13.38.156.156",
    "continent": "Europe",
    "continentCode": "EU",
    "country": "France",
    "countryCode": "FR",
    "region": "IDF",
    "regionName": "Île-de-France",
    "city": "Paris",
    "district": "",
    "zip": "75000",
    "lat": 48.8566,
    "lon": 2.35222,
    "timezone": "Europe/Paris",
    "offset": 3600,
    "currency": "EUR",
    "isp": "Amazon Technologies Inc.",
    "org": "AWS EC2 (eu-west-3)",
    "as": "AS16509 Amazon.com, Inc.",
    "asname": "AMAZON-02",
    "mobile": false,
    "proxy": false,
    "hosting": true
}
```

## mainnet.aestus.live
The IP for mainnet.aestus.live is 57.128.162.97. This IP does not use CDNs, and the server is located in the UK. Here are the details:
```
{
    "query": "57.128.162.97",
    "continent": "Europe",
    "continentCode": "EU",
    "country": "United Kingdom",
    "countryCode": "GB",
    "region": "ENG",
    "regionName": "England",
    "city": "Rainham",
    "district": "",
    "zip": "RM13",
    "lat": 51.5177,
    "lon": 0.194831,
    "timezone": "Europe/London",
    "offset": 0,
    "currency": "GBP",
    "isp": "OVH SAS",
    "org": "OVH Ltd",
    "as": "AS16276 OVH SAS",
    "asname": "OVH",
    "mobile": true,
    "proxy": false,
    "hosting": true
}
```

## mainnet-relay.securerpc.com
The IP addresses for mainnet-relay.securerpc.com are 15.204.142.24, 15.204.196.74, 15.204.196.75, 15.204.208.26, 15.204.196.73. All these IPs are located in North America.