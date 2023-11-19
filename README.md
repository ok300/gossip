# Gossip (PoW WoW fork)

This is a fork of [Gossip](https://github.com/mikedilger/gossip) with an integrated PoW WoW client.


## What is a PoW WoW Client?

[PoW WoW](https://lab.oak-node.net/powwow) is a protocol which lets Clients buy PoW for Nostr events from Providers, who offer to mine them.

In this proof-of-concept, the PoW WoW client will try to autobuy PoW based on preconfigured defaults (price, max time to wait).

The payment happens via Nostr Wallet Connect.


## How to run

Clone it with

```
git clone https://github.com/ok300/gossip --branch ok300-powwow-client
cd gossip
```

Edit `pw_config.toml` and set:
- `default_difficulty` to define how much PoW you want to buy for each new event
- `default_timeframe_sec` which says how much time you're willing to wait for PoW at most, per event
- `default_price_sats` which is the amount you're willing to pay per mined PoW
- `ln.nwc.nwc_connection_string`

Start Gossip with `cargo run` and set it up (generate profile key, choose relays, etc.).

Every new event posted with Gossip (note, like, repost, etc.) will now go through this:

- the PoW WoW client will try to autobuy the first blinded PoW solution
  - that offers at least `default_difficulty` bits of PoW
  - that is posted within `default_timeframe_sec` of the client's Ask
  - that comes with an invoice of at most `default_price_sats`
- if autobuy is successful, the client will use the preimage to unblind the PoW solution
  - the client will add the PoW to your event, sign it and broadcast it
- if not successful, the client will broadcast the event without any PoW


## Docs

Read more about PoW WoW here: https://oak-node.net/book/powwow/

Read more about running a provider here: https://oak-node.net/book/powwow/usage_provider.html


## Disclaimer

* This is experimental software
* Don't trust this fork with your main Gossip account
