import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DevQuest } from "../target/types/dev_quest";
import { expect } from "chai";
import { BN } from "@coral-xyz/anchor";


describe("dev-quest", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env(); 
  anchor.setProvider(provider);

  const program = anchor.workspace.devQuest as Program<DevQuest>;

  const adminPubKey = provider.wallet.publicKey;
  const userKeypair = anchor.web3.Keypair.generate();
  
  const configPDA = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("config")], program.programId)[0];
  const userAccountPDA = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("user"), provider.wallet.publicKey.toBuffer()], program.programId)[0];
  
  const taskId = new BN(0);
  const taskId1 = new BN(1);
  const taskPDA = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("task"),  Buffer.from(taskId.toArrayLike(Buffer, "le", 8))], program.programId)[0];
  const taskPDA1 = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("task"),  Buffer.from(taskId1.toArrayLike(Buffer, "le", 8))], program.programId)[0];
  const taskSubmissionPDA = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("submission"), provider.wallet.publicKey.toBuffer(), Buffer.from(taskId.toArrayLike(Buffer, "le", 8))], program.programId)[0];

  it("Initializes config account", async () => {
    const tx = await program.methods
      .initializeConfig()
      .accountsPartial({
        admin: adminPubKey,
        config: configPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Config initialized:", tx);

    const config = await program.account.config.fetch(configPDA);

    expect(config.admin.toString()).to.equal(adminPubKey.toString());
    expect(config.taskCounter.toNumber()).to.equal(0);
  });

  const name = "Costa"
  const bio = "A vida Costa!"
  const githubUsername = "JorgeCosta87"

  it("Initializes user account", async () => {
    const tx = await program.methods
      .initializeUser(name, bio, githubUsername)
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
    expect(userAccount.name).to.equal(name);
    expect(userAccount.bio).to.equal(bio);
    expect(userAccount.githubUsername).to.equal(githubUsername);
  });

  const title = "Task 1";
  const description = "Turbine Week 1";
  const points_reward = 64;
  const isActive = true;
  const dificulty = "Hard";



  it("Create Task", async () => {
    const tx = await program.methods
    .createTask(title, description, dificulty, points_reward, isActive)
    .accountsPartial({
        admin: adminPubKey,
        config: configPDA,
        task: taskPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

    const createdTask = await program.account.task.fetch(
      taskPDA
    );

    console.log("Task created: ", tx);
    console.log("First Task id: ", createdTask.taskId.toNumber())

    expect(createdTask.taskId.toNumber()).to.equal(0);
    expect(createdTask.title).to.equal(title);
    expect(createdTask.description).to.equal(description);
    expect(createdTask.dificulty).to.equal(dificulty);
    expect(createdTask.pointsReward).to.equal(points_reward);
    expect(createdTask.isActive).to.equal(isActive);
  })

  it("Create second Task, the task id is incremented", async () => {
    const tx = await program.methods
    .createTask(title, description, dificulty, points_reward, isActive)
    .accountsPartial({
        admin: adminPubKey,
        config: configPDA,
        task: taskPDA1,
        systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

    const createdTask = await program.account.task.fetch(
      taskPDA1
    );

    console.log("Task created: ", tx);
    console.log("Second Task id: ", createdTask.taskId.toNumber())

    expect(createdTask.taskId.toNumber()).to.equal(1);
    expect(createdTask.title).to.equal(title);
    expect(createdTask.description).to.equal(description);
    expect(createdTask.dificulty).to.equal(dificulty);
    expect(createdTask.pointsReward).to.equal(points_reward);
    expect(createdTask.isActive).to.equal(isActive);

  })

  const repo_url = "https://github.com/JorgeCosta87/DevQuest/tree/intialize-accounts";

  it("Submit Task", async () => {
    const tx = await program.methods
    .submitTask(taskId, repo_url)
    .accountsPartial({
        user: provider.wallet.publicKey,
        userAccount: userAccountPDA,
        task: taskPDA,
        taskSubmission: taskSubmissionPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

    const submitedTask = await program.account.taskSubmission.fetch(
      taskSubmissionPDA
    );

    console.log("Task created: ", tx);

    expect(submitedTask.taskId.toNumber()).to.equal(0);
    expect(submitedTask.repoName).to.equal(repo_url);
  })

});