PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"
WALLET_ALICE="${PWD}/mx-hackathon/wallets/alice.pem"
WALLET_BOB="${PWD}/mx-hackathon/wallets/bob.pem"
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqy50k2gdfhlvg9keky2jeh45td3kt87t37wpq4qnas0"
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
 mxpy contract deploy --proxy=${PROXY} \
    --chain="D" \
    --project=inner-circles \
    --pem="inner-circles/wallets/alice.pem" \
    --gas-limit=60000000 \
    --recall-nonce \
    --send \
    --metadata-payable
}

upgrade() {
 mxpy contract upgrade ${CONTRACT_ADDRESS} \
    --pem="inner-circles/wallets/alice.pem" \
    --chain=${CHAIN_ID} \
    --proxy=${PROXY} \
    --recall-nonce \
    --project=inner-circles \
    --gas-limit=60000000 \
    --send \
    --metadata-payable
}


######## CONTRACT CALLS

TKN_NAME="Band2"
TKN_TICKER="BND2"
NR=1000

issueFungibleToken() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --value=50000000000000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="inner-circles/wallets/alice.pem" \
    --gas-limit=140000000 \
    --function="issueFungibleToken" \
    --arguments "str:"$TKN_NAME "str:"$TKN_TICKER $NR 
} 

issueSemiFungibleToken() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --value=50000000000000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="inner-circles/wallets/alice.pem" \
    --gas-limit=70000000 \
    --function="issueSemiFungibleToken" \
    --arguments "str:"$TKN_NAME "str:"$TKN_TICKER  
} 

setLocalRoles() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="inner-circles/wallets/alice.pem" \
    --gas-limit=70000000 \
    --function="setLocalRoles"
} 

SFT="DUMMY"
URI="https://ipfs.io/ipfs/QmVATYyiL7r9RRbZAzqVassCt58M5JNji2CrVjaTQzk5Bm?filename=Band2"

createSft() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="inner-circles/wallets/alice.pem" \
    --gas-limit=70000000 \
    --function="createSft" \
    --arguments "str:"$SFT_NAME "str:"$URI
} 
 

######## QUERRIES

getUserToken() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getUserToken" \
    --arguments ${ALICE_ADDRESS_HEXX}
    }  

getUserSft() {
    echo ${CONTRACT_ADDRESS}
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getUserSft" \
    --arguments ${ALICE_ADDRESS_HEXX}
    }  