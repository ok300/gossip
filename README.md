# Gossip (PoW Pub fork)

This is a fork of [Gossip](https://github.com/mikedilger/gossip) with an integrated PoW Pub provider.

It has a single commit difference on top of the gossip `master` branch, which adds a separate thread to run the PoW Pub provider.

This means you can run Gossip to browse nostr, while the Provider quietly sits in the background and occasionally mines, earning you sats.

## What is a PoW Pub Provider?

[PoW Pub](https://lab.oak-node.net/powpub) is a protocol which lets Clients buy the mining of vanity Nostr pubkeys from Providers, who offer to mine them. Due to the split-key approach to mining, the resulting secret key is only known to the Client.

The Provider is essentially a program which runs the following loop:
- listen for PoW Pub Client Asks (requests for vanity pubkeys)
- if the requested difficulty is at or below a configured threshold, create a LN invoice and offer to mine it
- if the Client chooses this Provider's offer (e.g. pays the invoice and posts the preimage as proof), the Provider starts mining
- when done mining, the Provider posts the result (which the Client uses to derive the vanity secret key)

## How to run

Clone it with

```
git clone https://github.com/ok300/gossip --branch ok300-powpub-provider
cd gossip
```

Fill in your LN Address and start it with

```bash
POWPUB_PROVIDER_LN_ADDRESS="your@lighting-address.here" cargo run
```

For more advanced configuration, see the script `start-with-powpub-provider.sh`.

## How to test it

The quickest way to test that your provider is running and can pick up mining requests:

* Start the provider
* Go to the [PoW Pub WASM Demo](https://lab.oak-node.net/powpub/uv/wasm-client/) website (your browser will need a WebLN extension, like Alby)
* Create an Ask that would be very quick to mine (e.g. choose a 1 character hex prefix)
* Your request should appear in the table, along with the offers from interested Providers
* Expand your Ask by clicking the `+` icon at the left of the row
* Find your provider's pubkey and click "Pay to Take Offer", then pay with WebLN
* The offer should shortly turn blue (offer taken) then green (mining results posted), and your LN Address should receive the payment
* Collapse and expand the row in table (UI bug, working on a fix) to see the "Mined Keys" icon. Clicking it will show the resulting vanity keys.


## Disclaimer

* This is experimental software
* Don't trust this fork with your main Gossip account
* Mining will use all but one of your CPU cores

