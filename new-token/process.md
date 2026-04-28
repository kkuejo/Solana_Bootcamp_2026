＜Token Accountの作成＞
DiFiなどでトークンを管理するアカウント。
1. token 2022パッケージからのインポート
   getInitializeAccount2Instruction,
   getTokenSize,
2. Account Creationでnew^token.tsに以下のプログラムを作成。
 const tokenAccount = await generateKeyPairSigner();

  const tokenAccountSpace = BigInt(getTokenSize());
  const tokenAccountRent = await rpc
    .getMinimumBalanceForRentExemption(tokenAccountSpace)
    .send();

  const createTokenAccountInstruction = getCreateAccountInstruction({
    payer: feePayer,
    newAccount: tokenAccount,
    lamports: tokenAccountRent,
    space: tokenAccountSpace,
    programAddress: TOKEN_2022_PROGRAM_ADDRESS,
  });

  const initializeTokenAccountInstruction = getInitializeAccount2Instruction({
    account: tokenAccount.address,
    mint: mint.address,
    owner: feePayer.address,
  });

  const tokenAccountInstructions = [
    createTokenAccountInstruction,
    initializeTokenAccountInstruction,
  ];

  const { value: createTokenAccountLatestBlockhash } = await rpc
    .getLatestBlockhash()
    .send();

  const tokenAccountTxMessage = pipe(
    createTransactionMessage({ version: 0 }),
    (tx) => setTransactionMessageFeePayerSigner(feePayer, tx),
    (tx) =>
      setTransactionMessageLifetimeUsingBlockhash(
        createTokenAccountLatestBlockhash,
        tx,
      ),
    (tx) => appendTransactionMessageInstructions(tokenAccountInstructions, tx),
  );

  const signedTokenAccountTxMessage = await signTransactionMessageWithSigners(
    tokenAccountTxMessage,
  );

  const signedTokenAccountTxMessageWithLifetime =
    signedTokenAccountTxMessage as typeof signedTokenAccountTxMessage & {
      lifetimeConstraint: {
        lastValidBlockHeight: bigint;
      };
    };

  await sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions })(
    signedTokenAccountTxMessageWithLifetime,
    { commitment: "confirmed" },
  );

  const tokenAccountTxSignature = getSignatureFromTransaction(
    signedTokenAccountTxMessageWithLifetime,
  );

  console.log("Mint Address:", mint.address);
  console.log("Transaction Signature:", transactionSignature);
  console.log("\nToken Account Address:", tokenAccount.address);
  console.log("Token Account Transaction Signature:", tokenAccountTxSignature);
}

3. npx tsx new-token.tsを打って実行。
    Proart% npx tsx new-token.ts
Mint Address: Hzgr2W8QsmSYC6eMnoyw71XMP8UfLiLHvJcDa4191b6T
Transaction Signature: 5c47HuwiwoFsCbwbsS8Fx8BuhuPZGQRL5MhfmetxjTtNFiGfYwbsQ6jqB6XVuJWFbjhAmWyFbiH4SJa4ae85S5Yf

Token Account Address: 3QbdmgxZUAoa83kH4j9xepQD9J6Pp1Aubk5pLKtN7Mqc
Token Account Transaction Signature: 5ymFaqxYGMC3WyuXFXUf18tEX3uvib1SVq37phXxQtXdWkGJ34FQ1FLF8bCkhzhK8fB7oMYnSMfaiykd1QzyUdbp
のようにToken Accountが作成されたのが分かる。

＜Associated Token Accountの作成＞
各ユーザのウォレットが特定のアドレスのトークンを保有するアカウント

4. token 2022パッケージからのインポート
     getCreateAssociatedTokenInstructionAsync,
     findAssociatedTokenPda,

5. Account Creationでnew^token.tsに以下のプログラムを作成。
const [associatedTokenAccountAddress] = await findAssociatedTokenPda({
  mint: mint.address,
  owner: feePayer.address,
  tokenProgram: TOKEN_2022_PROGRAM_ADDRESS,
});

console.log(
  "\nAssociated Token Account Address:",
  associatedTokenAccountAddress,
);

const createAtaInstruction = await getCreateAssociatedTokenInstructionAsync({
  payer: feePayer,
  mint: mint.address,
  owner: feePayer.address,
});

const { value: createAtaLatestBlockhash } = await rpc
  .getLatestBlockhash()
  .send();

const ataTxMessage = pipe(
  createTransactionMessage({ version: 0 }),
  (tx) => setTransactionMessageFeePayerSigner(feePayer, tx),
  (tx) =>
    setTransactionMessageLifetimeUsingBlockhash(createAtaLatestBlockhash, tx),
  (tx) => appendTransactionMessageInstructions([createAtaInstruction], tx),
);

const signedAtaTxMessage =
  await signTransactionMessageWithSigners(ataTxMessage);

const signedAtaTxMessageWithLifetime =
  signedAtaTxMessage as typeof signedAtaTxMessage & {
    lifetimeConstraint: {
      lastValidBlockHeight: bigint;
    };
  };

await sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions })(
  signedAtaTxMessageWithLifetime,
  { commitment: "confirmed" },
);

const ataTxSignature = getSignatureFromTransaction(
  signedAtaTxMessageWithLifetime,
);

console.log(
  "\nAssociated Token Account Creation Transaction Signature:",
  ataTxSignature,
);

6. npx tsx new-token.tsを実行
Mint Address: 5aRG1q5g1yNQqCKuTzCsw3phL4YfpGUfrvXdzQzLKeqx
Transaction Signature: 5WxnDeJCVSMrz8pbMK9cMXuR2woMo4TJUZbUYpaZoLuTXTxW4DQGwnMqstBmVktLkMu7exuBUui9SfQmXjkB2PkA

Token Account Address: HNDtCe3FmoxvMireSQSKZRpsNyUzrbyC976vhp2K14LL
Token Account Transaction Signature: 59rsJHfwksaLSVXzccrrgKPVkbHyLUxiqr5kpA1komB4RGuUJobpRuWMjjHabDsHHr3qHTDmSpXxZ7VYu88CR13C

Associated Token Account Address: Gkx4LgMuEVV8de5Qk77iqTF3ecqP5j71eQNL1mcJnbCj

Associated Token Account Creation Transaction Signature: 3x9JYRbDsjeB6q89DcLgBVcm925v4JWY2S5DP6wHiRVHZm3A3CqufiubwXLJjTRRPULyAw9W3ZhwAQhJjRNSmtnN

のようにAssociated Token Accountが作成されたのが分かる。

1:46:00まで。