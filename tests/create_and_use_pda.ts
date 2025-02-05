import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CreateAndUsePda } from "../target/types/create_and_use_pda";

describe("PDA Counter Test", () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.CreateAndUsePda as Program<CreateAndUsePda>;

  it("Creates a PDA counter", async () => {
    const [counterPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
  
    try {
      const existingAccount = await program.account.baseAccount.fetch(counterPDA);
      console.log("PDA already exists with count:", existingAccount.count);
    } catch (err) {
      console.log("PDA not found, creating...");
  
      await program.methods.createCounter()
        .accounts({
          baseAccount: counterPDA,
          user: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
    }
  });  

  it("Increments the PDA counter", async () => {
    const [counterPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    await program.methods.incrementCounter()
      .accounts({
        baseAccount: counterPDA,
        user: provider.wallet.publicKey,
      })
      .rpc();
  });
});