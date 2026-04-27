console.log("Wallet:", pg.wallet.publicKey.toString());

const balance = await pg.connection.getBalance(pg.wallet.publicKey);
console.log("Balance:", balance / web3.LAMPORTS_PER_SOL, "SOL");