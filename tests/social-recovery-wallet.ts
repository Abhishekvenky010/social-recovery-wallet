import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SocialRecoveryWallet } from "../target/types/social_recovery_wallet";
import { Keypair } from "@solana/web3.js";
import assert from "assert";

describe("social-recovery-wallet", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.socialRecoveryWallet as Program<SocialRecoveryWallet>;

  // Generate test keypairs
  const owner = anchor.web3.Keypair.generate();
  const guardian1 = anchor.web3.Keypair.generate();
  const guardian2 = anchor.web3.Keypair.generate();
  const guardian3 = anchor.web3.Keypair.generate();
  const newOwner = anchor.web3.Keypair.generate();

  // Derive wallet PDA
  const [walletPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("wallet"), owner.publicKey.toBuffer()],
    program.programId
  );

  before(async () => {
    // Airdrop SOL to owner for testing
    const airdropSignature = await program.provider.connection.requestAirdrop(
      owner.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 2
    );
    await program.provider.connection.confirmTransaction(airdropSignature);
  });

  it("Initializes a new wallet with threshold", async () => {
    const threshold = 2;

    const tx = await program.methods
      .initialize(threshold, bump)
      .accounts({
        payer: owner.publicKey,
        wallet: walletPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([owner])
      .rpc();

    console.log("Initialize transaction signature:", tx);

    // Verify wallet account was created and initialized correctly
    const walletAccount = await program.account.wallet.fetch(walletPda);
    assert.equal(walletAccount.owner.toString(), owner.publicKey.toString());
    assert.equal(walletAccount.guardians.length, 0);
    assert.equal(walletAccount.threshold, threshold);
    assert.equal(walletAccount.recoveryInProgress, false);
    assert.equal(walletAccount.newOwner, null);
    assert.equal(walletAccount.recoveryInitiatedAt, null);
    assert.equal(walletAccount.approvals.length, 0);
    assert.equal(walletAccount.bump, bump);
  });

  it("Adds guardians to the wallet", async () => {
    // Add first guardian
    const addGuardian1Tx = await program.methods
      .addGuardian(guardian1.publicKey)
      .accounts({
        owner: owner.publicKey,
        wallet: walletPda,
      })
      .signers([owner])
      .rpc();

    console.log("Add guardian 1 transaction signature:", addGuardian1Tx);

    // Add second guardian
    const addGuardian2Tx = await program.methods
      .addGuardian(guardian2.publicKey)
      .accounts({
        owner: owner.publicKey,
        wallet: walletPda,
      })
      .signers([owner])
      .rpc();

    console.log("Add guardian 2 transaction signature:", addGuardian2Tx);

    // Verify guardians were added
    const walletAccount = await program.account.wallet.fetch(walletPda);
    assert.equal(walletAccount.guardians.length, 2);
    assert.equal(walletAccount.guardians[0].toString(), guardian1.publicKey.toString());
    assert.equal(walletAccount.guardians[1].toString(), guardian2.publicKey.toString());
  });

  it("Updates wallet threshold", async () => {
    const newThreshold = 1;

    const tx = await program.methods
      .updateThreshold(newThreshold)
      .accounts({
        owner: owner.publicKey,
        wallet: walletPda,
      })
      .signers([owner])
      .rpc();

    console.log("Update threshold transaction signature:", tx);

    // Verify threshold was updated
    const walletAccount = await program.account.wallet.fetch(walletPda);
    assert.equal(walletAccount.threshold, newThreshold);
  });

  it("Initiates a recovery process", async () => {
    const tx = await program.methods
      .initiateRecovery()
      .accounts({
        guardian: guardian1.publicKey,
        wallet: walletPda,
        newOwner: newOwner.publicKey,
      })
      .signers([guardian1])
      .rpc();

    console.log("Initiate recovery transaction signature:", tx);

    // Verify recovery was initiated
    const walletAccount = await program.account.wallet.fetch(walletPda);
    assert.equal(walletAccount.recoveryInProgress, true);
    assert.equal(walletAccount.newOwner.toString(), newOwner.publicKey.toString());
    assert.notStrictEqual(walletAccount.recoveryInitiatedAt, null);
    assert.equal(walletAccount.approvals.length, 1);
    assert.equal(walletAccount.approvals[0].toString(), guardian1.publicKey.toString());
  });

  it("Approves a recovery process", async () => {
    const tx = await program.methods
      .approveRecovery()
      .accounts({
        guardian: guardian2.publicKey,
        wallet: walletPda,
      })
      .signers([guardian2])
      .rpc();

    console.log("Approve recovery transaction signature:", tx);

    // Verify approval was added
    const walletAccount = await program.account.wallet.fetch(walletPda);
    assert.equal(walletAccount.approvals.length, 2);
    assert.equal(walletAccount.approvals[1].toString(), guardian2.publicKey.toString());
  });

  it("Cancels an in-progress recovery", async () => {
    const tx = await program.methods
      .cancelRecovery()
      .accounts({
        owner: owner.publicKey,
        wallet: walletPda,
      })
      .signers([owner])
      .rpc();

    console.log("Cancel recovery transaction signature:", tx);

    // Verify recovery was canceled
    const walletAccount = await program.account.wallet.fetch(walletPda);
    assert.equal(walletAccount.recoveryInProgress, false);
    assert.equal(walletAccount.newOwner, null);
    assert.equal(walletAccount.recoveryInitiatedAt, null);
    assert.equal(walletAccount.approvals.length, 0);
  });

  it("Removes a guardian from the wallet", async () => {
    const tx = await program.methods
      .removeGuardian(guardian1.publicKey)
      .accounts({
        owner: owner.publicKey,
        wallet: walletPda,
      })
      .signers([owner])
      .rpc();

    console.log("Remove guardian transaction signature:", tx);

    // Verify guardian was removed
    const walletAccount = await program.account.wallet.fetch(walletPda);
    assert.equal(walletAccount.guardians.length, 1);
    assert.equal(walletAccount.guardians[0].toString(), guardian2.publicKey.toString());
  });
});
