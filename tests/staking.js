const anchor = require('@project-serum/anchor');
const assert = require("assert");
const {
  TOKEN_PROGRAM_ID,
  getTokenAccount,
  createMint,
  createTokenAccount,
  mintToAccount,
  updateMintAuthority,
} = require("./utils");

describe('staking', () => {

  const provider = anchor.Provider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.Staking;

  // sorry for this horribleness
  let sancmint;
  let stakedsancmint;
  let usersanc;
  let userstakedsanc;

  it('Runs the constructor!', async () => {
    // Add your test here.
    await program.state.rpc.new({
      accounts: {
        authority: provider.wallet.publicKey,
      },
    });
    
    const state = await program.state.fetch();
    assert.ok(state.totalStaked.eq(new anchor.BN(0)));
  });

  it('Updates the authority!', async () => {
    const newAuthority = anchor.web3.Keypair.generate();

    await program.state.rpc.updateAuthority(
      newAuthority.publicKey,
      {
      accounts: {
        authority: provider.wallet.publicKey,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      },
    });
  });

  it('Stakes $SANC!', async () => {
    // Create $SANC mint
    const sancMint = await createMint(provider, provider.wallet.publicKey);

    // Program signer PDA
    const [_programSigner, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [sancMint.toBuffer()],
      program.programId
    );

    const programSigner = _programSigner;

    // Create $sSANC mint
    const stakedSancMint = await createMint(provider, programSigner);

    // User $SANC account
    const userSanc = await createTokenAccount(provider, sancMint, provider.wallet.publicKey);
    
    // 1000 $SANC
    const amount = new anchor.BN(1e12);
    await mintToAccount(provider, sancMint, userSanc, amount, provider.wallet.publicKey);

    await updateMintAuthority(provider, sancMint, provider.wallet.publicKey, programSigner);
    //console.log("Mint authority updated");

    const userStakedSanc = await createTokenAccount(provider, stakedSancMint, provider.wallet.publicKey);

    await program.state.rpc.stake(
      amount,
      bump,
      {
      accounts: {
        programSigner: programSigner,
        authority: provider.wallet.publicKey,
        sancMint: sancMint,
        userSanc: userSanc,
        stakedSancMint: stakedSancMint,
        userStakedSanc: userStakedSanc,
        tokenProgram: TOKEN_PROGRAM_ID,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      },
    });

    const userSancData = await getTokenAccount(provider, userSanc);

    const userStakedSancData = await getTokenAccount(provider, userStakedSanc);

    const state = await program.state.fetch();

    assert.ok(userSancData.amount.eq(new anchor.BN(0)));
    assert.ok(userStakedSancData.amount.eq(amount));
    assert.ok(state.totalStaked.eq(amount));

    sancmint = sancMint;
    stakedsancmint = stakedSancMint;
    usersanc = userSanc;
    userstakedsanc = userStakedSanc;
  });

  it("Unstakes $SANC!", async () => {
    // Program signer PDA
    const [_programSigner, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [sancmint.toBuffer()],
      program.programId
    );

    const programSigner = _programSigner;
    const amount = new anchor.BN(1e12);

    await program.state.rpc.unstake(
    amount,
    bump,
    {
      accounts: {
        programSigner: programSigner,
        authority: provider.wallet.publicKey,
        sancMint: sancmint,
        userSanc: usersanc,
        stakedSancMint: stakedsancmint,
        userStakedSanc: userstakedsanc,
        tokenProgram: TOKEN_PROGRAM_ID,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      },
    });

    const userSancData = await getTokenAccount(provider, usersanc);

    const userStakedSancData = await getTokenAccount(provider, userstakedsanc);

    //const state = await program.state.fetch();
    //console.log(state.totalStaked);
    //console.log(amount);

    assert.ok(userSancData.amount.eq(amount));
    assert.ok(userStakedSancData.amount.eq(new anchor.BN(0)));
    //assert.ok(state.totalStaked.eq(new anchor.BN(0)));
  });
});
