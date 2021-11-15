# Profit distributor B
btn.group's profit distributor to work along side BUTT lode.

## How it works
* User deposits incentivized token.
* This smart contract receives profits and distributes to depositors.

## The three pillars of blockchain
The three pillars refers to blockchain itself but we are attempting to follow the ethos as much as possible.

### 1. Decentralization
This contract has no admin functions once initialized

### 2. Transparency
All involved smart contracts are publicly viewable and audited by third parties. All aspects of this smart contract is public.

### 3. Immutability
This is secured by the Secret network.

## Regarding privacy
We have thought long and hard about this and have decided to make all aspects public. 

We thought about a centralized option where we only hold the viewing keys and show a delayed balance, but this would mean that the user base would have to take our word for it.

The point of blockchain is to be decentralized and trustless. One scam I can think of off the top of my head would be to inflate our numbers so as to attract more investors.

We think privacy is important, but it should be privacy for individuals and transparency for organizations.

###  Why we implemented then removed the pool shares token
The only reason we implemented a pool shares token was so that it could be used for users that deposited Buttcoin into this contract to still be able to vote. With what's going on in the world and from what I've seen in other crypto protocols, governance is a total sham. It's being used to look like there is a democratic process to disguise clear and present nepotism. Blockchain was created to counter this sort of thing. "Code is law" is the ethos. If there was ever to be a democratizing process, it must be based on one vote per person. I understand that democracy amongst share holders is different, but I don't like how this false democracy is being portrayed to users. We are going to stick to immutable code as was intended when Blockchain was conceptualized.

## Testing locally
```
# Run chain locally
docker run -it --rm -p 26657:26657 -p 26656:26656 -p 1337:1337 -v $(pwd):/root/code --name secretdev enigmampc/secret-network-sw-dev

# Access container via separate terminal window 
docker exec -it secretdev /bin/bash

# cd into code folder
cd code

# Store contracts required for test
secretcli tx compute store buttcoin.wasm.gz --from a --gas 3000000 -y --keyring-backend test
secretcli tx compute store snip-20-reference-impl.wasm.gz --from a --gas 3000000 -y --keyring-backend test
secretcli tx compute store profit-distributor-b.wasm.gz --from a --gas 3000000 -y --keyring-backend test

# Get the contract's id
secretcli query compute list-code

# Init Buttcoin 
CODE_ID=1
INIT='{"name": "Buttcoin", "symbol": "BUTT", "decimals": 6, "initial_balances": [{"address": "secret1qwkd2mdr0w79fyz6zyljs7u3cnff6dtekp3y39", "amount": "1000000000000000000"},{"address": "secret1wz95rde3wrf9e4hvdtwgey4d9zeys35sevchg5", "amount": "1000000000000000000"}], "prng_seed": "testing"}'
secretcli tx compute instantiate $CODE_ID "$INIT" --from a --label "Buttcoin" -y --keyring-backend test --gas 3000000 --gas-prices=3.0uscrt

# Set viewing key for Buttcoin
secretcli tx compute execute secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg '{"set_viewing_key": { "key": "testing" }}' --from a -y --keyring-backend test
secretcli tx compute execute secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg '{"set_viewing_key": { "key": "testing" }}' --from b -y --keyring-backend test

# Init stake token
CODE_ID=2
INIT='{"name": "Secret Finance", "symbol": "SEFI", "decimals": 6, "prng_seed": "testing", "config": {"enable_burn": true, "enable_mint": true, "public_total_supply": true}}'
secretcli tx compute instantiate $CODE_ID "$INIT" --from a --label "sefi" -y --keyring-backend test --gas 3000000 --gas-prices=3.0uscrt

# Mint SEFI to users
secretcli tx compute execute secret10pyejy66429refv3g35g2t7am0was7ya6hvrzf '{ "mint": { "recipient": "secret1qwkd2mdr0w79fyz6zyljs7u3cnff6dtekp3y39", "amount": "1000000000000000000" } }' --from a -y --keyring-backend test --gas 3000000 --gas-prices=3.0uscrt
secretcli tx compute execute secret10pyejy66429refv3g35g2t7am0was7ya6hvrzf '{ "mint": { "recipient": "secret1wz95rde3wrf9e4hvdtwgey4d9zeys35sevchg5", "amount": "1000000000000000000" } }' --from a -y --keyring-backend test --gas 3000000 --gas-prices=3.0uscrt

# Set viewing key for SEFI
secretcli tx compute execute secret10pyejy66429refv3g35g2t7am0was7ya6hvrzf '{"set_viewing_key": { "key": "testing" }}' --from a -y --keyring-backend test
secretcli tx compute execute secret10pyejy66429refv3g35g2t7am0was7ya6hvrzf '{"set_viewing_key": { "key": "testing" }}' --from b -y --keyring-backend test

# Init profit distributor B
CODE_ID=3
INIT='{"profit_token": {"address": "secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg", "contract_hash": "4CD7F64B9ADE65200E595216265932A0C7689C4804BE7B4A5F8CEBED250BF7EA"}, "incentivized_token": {"address": "secret10pyejy66429refv3g35g2t7am0was7ya6hvrzf", "contract_hash": "35F5DB2BC5CD56815D10C7A567D6827BECCB8EAF45BC3FA016930C4A8209EA69"}, "viewing_key": "DoTheRightThing."}'
secretcli tx compute instantiate $CODE_ID "$INIT" --from a --label "profit-distributor-b" -y --keyring-backend test --gas 3000000 --gas-prices=3.0uscrt

# Query config for Profit distributor B
secretcli query compute query secret1sh36qn08g4cqg685cfzmyxqv2952q6r8vqktuh '{"config": {}}'

# Send token to receivable address
secretcli tx compute execute secret10pyejy66429refv3g35g2t7am0was7ya6hvrzf '{"send": {"amount": "9894645645646", "recipient": "secret1sh36qn08g4cqg685cfzmyxqv2952q6r8vqktuh", "msg": "eyJkZXBvc2l0Ijoge319"}}' --from a -y --keyring-backend test --gas 3000000 --gas-prices=3.0uscrt
secretcli tx compute execute secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg '{"send": {"amount": "46545464556", "recipient": "secret1sh36qn08g4cqg685cfzmyxqv2952q6r8vqktuh", "msg": "eyJkZXBvc2l0Ijoge319"}}' --from a -y --keyring-backend test --gas 3000000 --gas-prices=3.0uscrt
secretcli tx compute execute secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg '{"send": {"amount": "97989446513", "recipient": "secret1sh36qn08g4cqg685cfzmyxqv2952q6r8vqktuh", "msg": "eyJkZXBvc2l0Ijoge319"}}' --from b -y --keyring-backend test --gas 3000000 --gas-prices=3.0uscrt

# Query user
secretcli query compute query secret1sh36qn08g4cqg685cfzmyxqv2952q6r8vqktuh '{"user": {"user_address": "secret1xzlgeyuuyqje79ma6vllregprkmgwgavk8y798"}}'
secretcli query compute query secret1sh36qn08g4cqg685cfzmyxqv2952q6r8vqktuh '{"user": {"user_address": "secret1qwkd2mdr0w79fyz6zyljs7u3cnff6dtekp3y39"}}'
secretcli query compute query secret1sh36qn08g4cqg685cfzmyxqv2952q6r8vqktuh '{"user": {"user_address": "secret1wz95rde3wrf9e4hvdtwgey4d9zeys35sevchg5"}}'

# Query claimable profit
secretcli query compute query secret1sh36qn08g4cqg685cfzmyxqv2952q6r8vqktuh '{"claimable_profit": {"user_address": "secret1xzlgeyuuyqje79ma6vllregprkmgwgavk8y798"}}'
secretcli query compute query secret1sh36qn08g4cqg685cfzmyxqv2952q6r8vqktuh '{"claimable_profit": {"user_address": "secret1qwkd2mdr0w79fyz6zyljs7u3cnff6dtekp3y39"}}'
secretcli query compute query secret1sh36qn08g4cqg685cfzmyxqv2952q6r8vqktuh '{"claimable_profit": {"user_address": "secret1wz95rde3wrf9e4hvdtwgey4d9zeys35sevchg5"}}'

# Withdraw token
secretcli tx compute execute secret1sh36qn08g4cqg685cfzmyxqv2952q6r8vqktuh '{"withdraw": {"amount": "1000000"}}' --from a -y --keyring-backend test --gas 3000000 --gas-prices=3.0uscrt
secretcli tx compute execute secret1sh36qn08g4cqg685cfzmyxqv2952q6r8vqktuh '{"withdraw": {"amount": "1000000"}}' --from b -y --keyring-backend test --gas 3000000 --gas-prices=3.0uscrt
```

## References
1. Yield optimizer: https://btn.group/secret_network/yield_optimizer
2. Secret contracts guide: https://github.com/enigmampc/secret-contracts-guide

