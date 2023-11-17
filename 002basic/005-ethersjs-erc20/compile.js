const fs = require("fs");
const solc = require("solc");
// Get path and Load Contract
const source = fs.readFileSync("SimpleToken.sol", "utf8");
function findImports(path) {
  if (fs.existsSync(path)) {
    return {
      contents: fs.readFileSync("./node_modules/" + path),
    };
  } else if (fs.existsSync("./node_modules/" + path)) {
    return {
      contents: fs.readFileSync("./node_modules/" + path, "utf8"),
    };
  } else {
    return { error: "File not found" };
  }
}
// Compile Contract
const input = {
  language: "Solidity",
  sources: {
    "SimpleToken.sol": {
      content: source,
    },
  },
  settings: {
    outputSelection: {
      "*": {
        "*": ["*"],
      },
    },
  },
};

const tempFile = JSON.parse(
  solc.compile(JSON.stringify(input), { import: findImports })
);
const contractFile = tempFile.contracts["SimpleToken.sol"]["SimpleToken"];
//console.log("contractFile:", contractFile);
// Export Contract Data
module.exports = contractFile;
