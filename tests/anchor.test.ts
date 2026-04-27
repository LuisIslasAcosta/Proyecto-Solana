import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

describe("Motocicletas", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Motocicletas as Program;

  const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("agencia"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Crear agencia", async () => {
    await program.methods
      .crearAgencia("Agencia Motos")
      .accounts({
        owner: provider.wallet.publicKey,
        agencia: pda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
  });

  it("Agregar moto", async () => {
    await program.methods
      .agregarMoto("R15", "Yamaha", 50000, 5)
      .accounts({
        owner: provider.wallet.publicKey,
        agencia: pda,
      })
      .rpc();
  });

  it("Ver moto", async () => {
    const cuenta = await program.account.agencia.fetch(pda);
    console.log(cuenta);
  });
});