#!/bin/bash

# The relays used by this provider to listen and react to Client Asks
# export POWPUB_RELAYS=["wss://relay.damus.io","wss://nos.lol"]

# Maximum difficulty of a Client Ask this provider is willing to mine, measured in bits. Any Client Asks with higher difficulty are ignored.
# Hex requests have 4 bits per character, Npub requests have 5 bits per character, and PoW requests explicitly say the number of bits they need.
# export POWPUB_PROVIDER_MAX_MINING_DIFFICULTY=25

# When creating an invoice for a Client Ask, the invoice amount will be based on this unit price x expected number of hashes
# until a matching pubkey is found, then everything rounded to sats.
# 1 hex char prefix  = 16 (2^4) expected hashes on average, 2 hex char prefix  =  256 (2^8) expected hashes on average, etc
# 1 npub char prefix = 32 (2^5) expected hashes on average, 2 npub char prefix = 1024 (2^10) expected hashes on average, etc
# invoice_amount = POWPUB_PROVIDER_MSATS_PER_EXPECTED_HASH * (expected hashes on average)
# The value has to expressed as a decimal (1.0, 15.0, 1.21, 3.4, etc).
# export POWPUB_PROVIDER_MSATS_PER_EXPECTED_HASH=0.21

# To preserve provider reputation across restarts, generate and set a Nostr private key (nsec) for your provider here
# If not set, on each restart your provider will start with zero reputation, which may cause some Clients to not take your offers
# in favor of Providers with higher reputation
# export POWPUB_PROVIDER_NOSTR_PRV_KEY=


export POWPUB_PROVIDER_LN_ADDRESS="your@lighting-address.here"

cargo run --release