PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"
WALLET_ALICE="${PWD}/mx-hackathon/wallets/alice.pem"
WALLET_BOB="${PWD}/mx-hackathon/wallets/bob.pem"
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqcx9d8v3wwnt96dhu8j7vyk0y4r2au2l77wpq9nv3gp"
ALICE_ADDRESS="erd1aqd2v3hsrpgpcscls6a6al35uc3vqjjmskj6vnvl0k93e73x7wpqtpctqw"
ALICE_ADDRESS_HEX="$(erdpy wallet bech32 --decode ${ALICE_ADDRESS})"
ALICE_ADDRESS_HEXX="0x$(erdpy wallet bech32 --decode ${ALICE_ADDRESS})"
BOB_ADDRESS="erd1wh2rz67zlq5nea7j4lvs39n0yavjlaxal88f744k2ps036ary8dq3ptyd4"
BOB_ADDRESS_HEX="$(erdpy wallet bech32 --decode ${BOB_ADDRESS})"
BOB_ADDRESS_HEXX="0x$(erdpy wallet bech32 --decode ${BOB_ADDRESS})"
MARTA_ADDRESS="erd1uycnjd0epww6xrmn0xjdkfhjengpaf4l5866rlrd8qpcsamrqr8qs6ucxx"
MARTA_ADDRESS_HEX="$(erdpy wallet bech32 --decode ${MARTA_ADDRESS})"
MARTA_ADDRESS_HEXX="0x$(erdpy wallet bech32 --decode ${MARTA_ADDRESS})"

deploy() {
 erdpy contract deploy
    --proxy=${PROXY} \
    --chain="D" \
    --project=inner-circles \
    --pem="inner-circles/wallets/alice.pem" \
    --gas-limit=60000000 \
    --recall-nonce \
    --send \
    --metadata-payable
}

upgrade() {
 mxdpy contract upgrade ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \ 
    --chain=${CHAIN_ID} \
    --project=inner-circles \
    --pem="inner-circles/wallets/alice.pem" \
    --gas-limit=60000000 \
    --recall-nonce \
    --send \
    --metadata-payable
}



######## CONTRACT CALLS

TKN_NAME="Band1"
TKN_TICKER="BND1"
NR=1000

issueFungibleToken() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --value=50000000000000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="inner-circles/wallets/alice.pem" \
    --gas-limit=70000000 \
    --function="issueFungibleToken" \
    --arguments "str:"$TKN_NAME "str:"$TKN_TICKER $NR 
} 



######## QUERRIES

getUserToken() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getUserToken" \
    --arguments ${ALICE_ADDRESS_HEXX}
    }  