PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"
WALLET_ALICE="${PWD}/inner-circles/wallets/alice.pem"
WALLET_BOB="${PWD}/inner-circles/wallets/bob.pem"
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqy50k2gdfhlvg9keky2jeh45td3kt87t37wpq4qnas0"
ALICE_ADDRESS="erd1aqd2v3hsrpgpcscls6a6al35uc3vqjjmskj6vnvl0k93e73x7wpqtpctqw"
ALICE_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${ALICE_ADDRESS})"
ALICE_ADDRESS_HEXX="0x$(mxpy wallet bech32 --decode ${ALICE_ADDRESS})"
BOB_ADDRESS="erd1wh2rz67zlq5nea7j4lvs39n0yavjlaxal88f744k2ps036ary8dq3ptyd4"
BOB_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${BOB_ADDRESS})"
BOB_ADDRESS_HEXX="0x$(mxpy wallet bech32 --decode ${BOB_ADDRESS})"
MARTA_ADDRESS="erd1uycnjd0epww6xrmn0xjdkfhjengpaf4l5866rlrd8qpcsamrqr8qs6ucxx"
MARTA_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${MARTA_ADDRESS})"
MARTA_ADDRESS_HEXX="0x$(mxpy wallet bech32 --decode ${MARTA_ADDRESS})"

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

TKN_NAME="BandNFT"
TKN_TICKER="BNDNFT"
NR=1000

SFT="DUMMYSFT"
NFT="DUMMYNFT"
URI="https://ipfs.io/ipfs/QmVATYyiL7r9RRbZAzqVassCt58M5JNji2CrVjaTQzk5Bm"
ATTR="AttributeName"

issueFungibleToken() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
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
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --value=50000000000000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="inner-circles/wallets/alice.pem" \
    --gas-limit=170000000 \
    --function="issueSemiFungibleToken" \
    --arguments "str:"$TKN_NAME "str:"$TKN_TICKER  
} 
 
createSft() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="inner-circles/wallets/alice.pem" \
    --gas-limit=5500000 \
    --function="createSft" \
    --arguments "str:"$SFT_NAME "str:"$URI "str:"$ATTR 
} 

TOKEN_AMOUNT=1
CLAIM_TOKEN=BND2-90614b
CLAIM_TOKEN_HEX="$(echo -n ${CLAIM_TOKEN} | xxd -p -u | tr -d '\n')"

BAND_SFT=str:BND6-5dc391

claimToken() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --pem=${WALLET_ALICE} \
    --recall-nonce \
    --gas-limit=10000000 \
    --function="claimToken" \
    --arguments $TOKEN_AMOUNT "str:"$CLAIM_TOKEN  
}

AMOUNT=1
AMOUNT_HEX=$(python3 ${PWD}/inner-circles/interaction/to_hex.py ${AMOUNT})

SFT_NONCE=1
SFT_NONCE_HEX=$(python3 ${PWD}/inner-circles/interaction/to_hex.py ${SFT_NONCE})
 
BUY_FUNCTION="buySft"
BUY_FUNCTION_HEX="$(echo -n ${BUY_FUNCTION} | xxd -p -u | tr -d '\n')"

buySft() {
    erdpy --verbose tx new \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --pem=${WALLET_ALICE} \
    --recall-nonce \
    --gas-limit=10000000 \
    --receiver=${CONTRACT_ADDRESS} \
    --data="ESDTTransfer@${CLAIM_TOKEN_HEX}@${AMOUNT_HEX}@${BUY_FUNCTION_HEX}@${ALICE_ADDRESS_HEX}@${SFT_NONCE_HEX}"  
}



issueNonFungibleToken() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --value=50000000000000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem=${WALLET_ALICE} \
    --gas-limit=170000000 \
    --function="issueNonFungibleToken" \
    --arguments "str:"$TKN_NAME "str:"$TKN_TICKER  
} 
 
createNft() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem=${WALLET_ALICE} \
    --gas-limit=5500000 \
    --function="createNft" \
    --arguments "str:"$NFT_NAME "str:"$URI "str:"$ATTR 
}
 
######## QUERRIES

getUserToken() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getUserToken" \
    --arguments ${ALICE_ADDRESS_HEXX}
    }  

getUserSft() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getUserSft" \
    --arguments ${ALICE_ADDRESS_HEXX}
    }  

getUserNft() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getUserNft" \
    --arguments ${ALICE_ADDRESS_HEXX}
    }  