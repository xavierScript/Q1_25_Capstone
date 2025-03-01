import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CapstoneProject } from "../target/types/capstone_project";
import { assert } from "chai";

describe("capstone-project", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CapstoneProject as Program<CapstoneProject>;

  let maker = anchor.web3.Keypair.generate();
  let taker = anchor.web3.Keypair.generate();
  let mintA = anchor.web3.Keypair.generate();
  let escrowAccount = anchor.web3.Keypair.generate();
  let vault = anchor.web3.Keypair.generate();

  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("Initializes an escrow account", async () => {
    const seed = new anchor.BN(123);
    const receiveAmount = new anchor.BN(100);

    await program.methods
      .initEscrow(seed, receiveAmount)
      .accounts({
        maker: maker.publicKey,
        mintA: mintA.publicKey,
        makerAtaA: vault.publicKey,
        escrow: escrowAccount.publicKey,
        vault: vault.publicKey,
      })
      .signers([maker, escrowAccount])
      .rpc();

    const escrowData = await program.account.escrow.fetch(
      escrowAccount.publicKey
    );
    assert.equal(escrowData.maker.toString(), maker.publicKey.toString());
    assert.equal(escrowData.receive.toNumber(), 100);
  });

  it("Deposits tokens into escrow", async () => {
    await program.methods
      .deposit(new anchor.BN(50))
      .accounts({
        maker: maker.publicKey,
        mintA: mintA.publicKey,
        makerAtaA: vault.publicKey,
        escrow: escrowAccount.publicKey,
        vault: vault.publicKey,
      })
      .signers([maker])
      .rpc();

    const vaultData = await program.account.tokenAccount.fetch(vault.publicKey);
    assert.equal(vaultData.amount.toNumber(), 50);
  });

  it("Allows taker to withdraw and close vault", async () => {
    await program.methods
      .withdrawAndCloseVault()
      .accounts({
        taker: taker.publicKey,
        maker: maker.publicKey,
        mintA: mintA.publicKey,
        takerAtaA: vault.publicKey,
        escrow: escrowAccount.publicKey,
        vault: vault.publicKey,
      })
      .signers([taker])
      .rpc();

    try {
      await program.account.tokenAccount.fetch(vault.publicKey);
      assert.fail("Vault account should be closed");
    } catch (e) {
      assert.ok(e.message.includes("Account does not exist"));
    }
  });

  it("Allows maker to refund and close escrow", async () => {
    await program.methods
      .refundAndCloseVault()
      .accounts({
        maker: maker.publicKey,
        mintA: mintA.publicKey,
        makerAtaA: vault.publicKey,
        escrow: escrowAccount.publicKey,
        vault: vault.publicKey,
      })
      .signers([maker])
      .rpc();

    try {
      await program.account.escrow.fetch(escrowAccount.publicKey);
      assert.fail("Escrow account should be closed");
    } catch (e) {
      assert.ok(e.message.includes("Account does not exist"));
    }
  });
});
