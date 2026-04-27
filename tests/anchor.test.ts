import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

describe("Refaccionaria", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Refaccionaria as Program;

  const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("refaccionaria"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Crear refaccionaria", async () => {
    await program.methods
      .crearRefaccionaria("Refaccionaria Lopez")
      .accounts({
        owner: provider.wallet.publicKey,
        refaccionaria: pda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Refaccionaria creada");
  });

  it("Agregar refaccion", async () => {
    await program.methods
      .agregarRefaccion("Filtro de aceite", 200, 10)
      .accounts({
        owner: provider.wallet.publicKey,
        refaccionaria: pda,
      })
      .rpc();

    console.log("Refaccion agregada");
  });

  it("Actualizar stock", async () => {
    await program.methods
      .actualizarStock("Filtro de aceite", 20)
      .accounts({
        owner: provider.wallet.publicKey,
        refaccionaria: pda,
      })
      .rpc();

    console.log("Stock actualizado");
  });

  it("Ver refacciones", async () => {
    const cuenta = await program.account.refaccionaria.fetch(pda);
    console.log("Datos:", cuenta);
  });
});