## Blockchain Node & Client
- node 
  
A device running the sofrware of a specific blockchain,store data,verify tx.
- Client

The software running the node (Geth,Erigon,Nethermind)
- Data

Blockchain history ,bl;ocks,txs,smart contract,etc
- State

Ethereum's state is a large data structure whitch holds not only all accounts and balances,but a machine state,whitch can execute arbitary machine code.
  
## Blockchain RPC & API
- API

Application Programming interface : software interfaces that aloow developer to interact with a blockchain network
Classifcation: REST API ,JSON API

- RPC
  
  Remote Procedure call : an RPC acccess to server node on the specified network and allow you to communaicate and interact with blockchain.
  Classification:HTTP/HTTPS,WEbSocket

RPC includes API command

### JSON RPC detail
- Request structure

"jsonrpc":include the version of jsonrpc protocol being used. 

"method":Includes the name of json-rpc being called

"params":Includes any parameters that are being passed to the json-rpc method.

"id":A unique identifier the request.Use diffient ID to make batch call.
- Response structure

"jsonrpc":Includes the version of json-rpc being used.

"id":A unique identifier the request,same from request

"result": includes the result of json-rpc request.

## RPC Application

### Adoption
Most widely used service in the web3 industry.Developers,Users,Dapps, as long as the do not run the node themselves.

## RPC Service Prividers

## The future of RPC Service

