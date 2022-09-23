const anchor = require("@project-serum/anchor");

const main = async () => {
  console.log("🚀 Starting test...");

  // Here we're telling anchor to set my provider. It gets this data from `solana config get`
  // Its grabbing my local environment & tells Anchor to run the code locally:
  anchor.setProvider(anchor.AnchorProvider.env());
  
  // This automatically compiles the code in lib.rs & deploys it locally on a local validator:
  const program = anchor.workspace.Sceneportal;
  const tx = await program.rpc.startStuffOff();
  console.log("📝 Your transaction signature: ", tx);
}

const runMain = async () => {
  try { 
    await main(); 
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
}

runMain();