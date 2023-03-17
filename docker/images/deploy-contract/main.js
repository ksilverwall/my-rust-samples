const fs = require("fs");
const yargs = require("yargs/yargs");
const { hideBin } = require("yargs/helpers");
const Web3 = require("web3");


const registerContract = async (
  web3,
  fromAddress,
  passwordFile,
  binFile,
  abiFile,
  outFile
) => {
  const abi = JSON.parse(fs.readFileSync(abiFile, "utf8"));
  const bin = fs.readFileSync(binFile, "utf8").trim();
  const password = fs.readFileSync(passwordFile, "utf8").trim();

  const unlock = await web3.eth.personal.unlockAccount(
    fromAddress,
    password,
    30
  );
  if (!unlock) {
    throw new Error("Failed to unlock account.");
  }

  const contract = new web3.eth.Contract(abi);
  const tx = await contract.deploy({ data: "0x" + bin });

  const deployedContract = await tx.send({
    from: fromAddress,
    gasPrice: "1",
    gas: 1000000000,
  });

  fs.writeFileSync(outFile, deployedContract.options.address + "\n");
  console.log(`account output to: ${outFile}`);
};

const main = async (web3) => {
  const argv = yargs(hideBin(process.argv))
    .option("account", {
      description: "password file path",
      type: "string",
      demandOption: true,
    })
    .option("password", {
      description: "password file path",
      type: "string",
      demandOption: true,
    })
    .option("abi", {
      description: "abi file path",
      type: "string",
      demandOption: true,
    })
    .option("bin", {
      description: "bin file path",
      type: "string",
      demandOption: true,
    })
    .option("out", {
      description: "output file path",
      type: "string",
      demandOption: true,
    })
    .parse();

  await registerContract(
    web3,
    "0x" + argv.account,
    argv.password,
    argv.bin,
    argv.abi,
    argv.out
  );
};

main(new Web3(process.env.ETH_URL || "http://localhost:8545"));
