# Web3js ERC20

This basic task is to show how to interact with ERC20 contract, so the developer can understand the basic interface of ERC20 contract.
本样例演示了 ERC20 合约的基本调用, 让开发者了解 ERC20 合约的基本接口

## Getting started 开始
### SimpleToken contract function description   SimpleToken合约功能描述

- IERC20
  totalSupply: Get the total amount of ERC20 token in the contract 获取该合约内总的 ERC20 Token 总量  
  balanceOf: Get the amount of ERC20 token of specific account in the contract 获取特定账户的 ERC20 Token 总量  
  transfer: Transfer ERC20 token to specific account 向目标账户转移特定数量的 ERC20 Token
  allowance: Get the amount of ERC20 tokens of the source account that the target account can use 获取目标账户能够使用的源账户的 ERC20 Token 数量  
  approve: Authorize the target account to transfer the specified amount of ERC20 Tokens 向目标账户授权, 可以转移指定额度的 ERC20 Token 数量  
  transferFrom: (Third party call) Transfer a specific amount of ERC20 token from the source account to target account ( 第三方调用 ) 从源账户向目标账户转移制定数量的 ERC20 Token

- IERC20Metadata
  name: Get the name of the Token 返回 Token 的名称  
  symbol: Get the symbol of the Token 返回 Token 的符号  
  decimals: Get the decimals of the Token 返回 Token 所支持的精度