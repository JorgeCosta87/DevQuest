import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DevQuest } from "../target/types/dev_quest";
import { expect } from "chai";

describe("dev-quest", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env(); 
  anchor.setProvider(provider);

  const program = anchor.workspace.devQuest as Program<DevQuest>;

  const adminKeypair = provider.wallet.publicKey;
  const userKeypair = anchor.web3.Keypair.generate();
  
  const configPDA = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("config")], program.programId)[0];
  const userAccountPDA = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("user"), provider.wallet.publicKey.toBuffer()], program.programId)[0];

  it("Initializes config account", async () => {
    const tx = await program.methods
      .initializeConfig()
      .accountsPartial({
        admin: adminKeypair,
        config: configPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Config initialized:", tx);

    const config = await program.account.config.fetch(configPDA);

    expect(config.admin.toString()).to.equal(adminKeypair.toString());
    expect(config.taskCounter.toNumber()).to.equal(0);
  });

  it("Initializes user account", async () => {
    const tx = await program.methods
      .initializeUser()
      .accountsPartial({
        user: provider.wallet.publicKey,
        userAccount: userAccountPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Config initialized:", tx);
    
    const userAccount = await program.account.userAccount.fetch(
      userAccountPDA
    );

    expect(userAccount.totalPoints.toNumber()).to.equal(0);
    expect(userAccount.name).to.equal("");
    expect(userAccount.bio).to.equal("");
    expect(userAccount.githubUsername).to.equal("");
  });

});