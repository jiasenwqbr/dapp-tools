const axios = require("axios");

const SOLANA_RPC_URL =
  "https://mainnet.helius-rpc.com/?api-key=a9562e4b-2ec7-4040-b869-9f5a11f823d8";
const solanaAddress = "Aez4fVqQp2cfZoGeYtGJt2tNJSWd1GuuhCqrD5gRMSY6"; // 替换为你要查询的地址

async function getSignatures() {
  const response = await axios.post(SOLANA_RPC_URL, {
    jsonrpc: "2.0",
    id: 1,
    method: "getConfirmedSignaturesForAddress2",
    params: [solanaAddress, { limit: 5 }],
  });

  console.log(response.data);
}

getSignatures();
