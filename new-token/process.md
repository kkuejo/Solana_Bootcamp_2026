<トークンの発行>
1. 次のようにToken-2022から、getMintToInstruction、fetchTokenの関数をインポートする。
import {
  getInitializeMintInstruction,
  getInitializeAccount2Instruction,
  getCreateAssociatedTokenInstructionAsync,
  getMintToInstruction,
  getMintSize,
  getTokenSize,
  TOKEN_2022_PROGRAM_ADDRESS,
  findAssociatedTokenPda,
  fetchToken,
} from "@solana-program/token-2022";

2. 次のようなコードをnew-token.tsに追加
const mintToInstruction = getMintToInstruction({
  mint: mint.address,
  token: associatedTokenAccountAddress,
  mintAuthority: feePayer.address,
  amount: 1_000_000_000n,
});

const { value: mintToLatestBlockhash } = await rpc.getLatestBlockhash().send();

const mintToTxMessage = pipe(
  createTransactionMessage({ version: 0 }),
  (tx) => setTransactionMessageFeePayerSigner(feePayer, tx),
  (tx) =>
    setTransactionMessageLifetimeUsingBlockhash(mintToLatestBlockhash, tx),
  (tx) => appendTransactionMessageInstructions([mintToInstruction], tx),
);

const signedMintToTxMessage =
  await signTransactionMessageWithSigners(mintToTxMessage);

const signedMintToTxMessageWithLifetime =
  signedMintToTxMessage as typeof signedMintToTxMessage & {
    lifetimeConstraint: {
      lastValidBlockHeight: bigint;
    };
  };

await sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions })(
  signedMintToTxMessageWithLifetime,
  { commitment: "confirmed" },
);

const mintToTxSignature = getSignatureFromTransaction(
  signedMintToTxMessageWithLifetime,
);

console.log("\nMint To Transaction Signature:", mintToTxSignature);

const ataData = await fetchToken(rpc, associatedTokenAccountAddress, {
  commitment: "confirmed",
});

const ataBalance = ataData.data.amount;

console.log(
  "Associated Token Account Balance:",
  Number(ataBalance) / 1_000_000_000,
);

3. npx tsx new-token.tsを実行すると、以下のように、1トークンがミントされたことが分かる。
Mint To Transaction Signature: 21hqZ45srE5kAcaSBUCevn3nvpB9Pc68S5JDyUXPHNeFeJmVMRJNNDUZ1DjqfzWogWtZHePb8NUGJYftekgVjGuu
Associated Token Account Balance: 1


<トークンの送金>

4.次のようにToken-2022から、getTransferInstructionの関数をインポートする。
import {
  getInitializeMintInstruction,
  getInitializeAccount2Instruction,
  getCreateAssociatedTokenInstructionAsync,
  getMintToInstruction,
  getTransferInstruction,
  getMintSize,
  getTokenSize,
  TOKEN_2022_PROGRAM_ADDRESS,
  findAssociatedTokenPda,
  fetchToken,
} from "@solana-program/token-2022";


5. new-token.tsに以下のコードを追加。
const recipient = await generateKeyPairSigner();

const [recipientAssociatedTokenAddress] = await findAssociatedTokenPda({
  mint: mint.address,
  owner: recipient.address,
  tokenProgram: TOKEN_2022_PROGRAM_ADDRESS,
});

console.log(
  "\nRecipient Associated Token Account Address:",
  recipientAssociatedTokenAddress,
);

const createRecipientAtaInstruction =
  await getCreateAssociatedTokenInstructionAsync({
    payer: feePayer,
    mint: mint.address,
    owner: recipient.address,
  });

const { value: createRecipientAtaLatestBlockhash } = await rpc
  .getLatestBlockhash()
  .send();

const recipientAtaTxMessage = pipe(
  createTransactionMessage({ version: 0 }),
  (tx) => setTransactionMessageFeePayerSigner(feePayer, tx),
  (tx) =>
    setTransactionMessageLifetimeUsingBlockhash(
      createRecipientAtaLatestBlockhash,
      tx,
    ),
  (tx) =>
    appendTransactionMessageInstructions([createRecipientAtaInstruction], tx),
);

const signedRecipientAtaTxMessage = await signTransactionMessageWithSigners(
  recipientAtaTxMessage,
);

const signedRecipientAtaTxMessageWithLifetime =
  signedRecipientAtaTxMessage as typeof signedRecipientAtaTxMessage & {
    lifetimeConstraint: {
      lastValidBlockHeight: bigint;
    };
  };
await sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions })(
  signedRecipientAtaTxMessageWithLifetime,
  { commitment: "confirmed" },
);

const recipientAtaTxSignature = getSignatureFromTransaction(
  signedRecipientAtaTxMessageWithLifetime,
);

console.log(
  "\nRecipient Associated Token Account Creation Transaction Signature:",
  recipientAtaTxSignature,
);

const transferInstruction = getTransferInstruction({
  source: associatedTokenAccountAddress,
  destination: recipientAssociatedTokenAddress,
  authority: feePayer.address,
  amount: 500_000_000n,
});

const { value: transferLatestBlockhash } = await rpc
  .getLatestBlockhash()
  .send();

const transferTxMessage = pipe(
  createTransactionMessage({ version: 0 }),
  (tx) => setTransactionMessageFeePayerSigner(feePayer, tx),
  (tx) =>
    setTransactionMessageLifetimeUsingBlockhash(transferLatestBlockhash, tx),
  (tx) => appendTransactionMessageInstructions([transferInstruction], tx),
);

const signedTransferTxMessage =
  await signTransactionMessageWithSigners(transferTxMessage);

const signedTransferTxMessageWithLifetime =
  signedTransferTxMessage as typeof signedTransferTxMessage & {
    lifetimeConstraint: {
      lastValidBlockHeight: bigint;
    };
  };

await sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions })(
  signedTransferTxMessageWithLifetime,
  { commitment: "confirmed" },
);

const transferTxSignature = getSignatureFromTransaction(
  signedTransferTxMessageWithLifetime,
);

console.log("\nTransfer Transaction Signature:", transferTxSignature);

const senderAtaData = await fetchToken(rpc, associatedTokenAccountAddress, {
  commitment: "confirmed",
});

const senderAtaBalance = Number(senderAtaData.data.amount);

console.log(
  "Sender Associated Token Account Balance:",
  senderAtaBalance / 1_000_000_000,
);

const recipientAtaData = await fetchToken(
  rpc,
  recipientAssociatedTokenAddress,
  { commitment: "confirmed" },
);

const recipientAtaBalance = Number(recipientAtaData.data.amount);

console.log(
  "Recipient Associated Token Account Balance:",
  recipientAtaBalance / 1_000_000_000,
);

6. npx tsx new-token.tsを実行すると、以下のように、0.5トークンがtransferされたことが分かる。
Recipient Associated Token Account Address: AGtRbGGovaDiTCBDux9rWgu8rVRab3c4ibEtvbFkyY1M

Recipient Associated Token Account Creation Transaction Signature: 5FyR3argyKmhripAr6EZc4afEovifnMSSnsJHyMT2WrESXrtVbz4ubMynk4b5WZZJ7GaRd9nuEiv6hyH7nAcxKg3

Transfer Transaction Signature: GBdMAnFNVw1km5p2UjqtcchJidnaS94vskCNjUtvM2BWU35e5UnUA8JgneEL3h5dfNqsTBr6jJAtnCAU35o4T3S
Sender Associated Token Account Balance: 0.5
Recipient Associated Token Account Balance: 0.5


<トークンのフリーズ>
フリーズすると、そのトークンアカウントからtransferやバーンを禁止する。
フリーズを出来るアカウントを最初に指定しておくと、そのアカウントからのみフリーズできる。

7. 次のようにToken-2022から、getFreezeAccountInstructionの関数をインポートする。
import {
  getInitializeMintInstruction,
  getInitializeAccount2Instruction,
  getCreateAssociatedTokenInstructionAsync,
  getMintToInstruction,
  getTransferInstruction,
  getFreezeAccountInstruction,
  getMintSize,
  getTokenSize,
  TOKEN_2022_PROGRAM_ADDRESS,
  findAssociatedTokenPda,
  fetchToken,
} from "@solana-program/token-2022";

8.  Mintを初期化するところに、freezeAuthorityを次のように定義する。
  const initializeMintInstruction = getInitializeMintInstruction({
    mint: mint.address,
    decimals: 9,
    mintAuthority: feePayer.address,
    freezeAuthority: feePayer.address,
  });

9. new-token.tsに以下のコードを追加。
const freezeInstruction = getFreezeAccountInstruction({
  account: associatedTokenAccountAddress,
  mint: mint.address,
  owner: feePayer.address,
});

const { value: freezeLatestBlockhash } = await rpc.getLatestBlockhash().send();

const freezeTxMessage = pipe(
  createTransactionMessage({ version: 0 }),
  (tx) => setTransactionMessageFeePayerSigner(feePayer, tx),
  (tx) =>
    setTransactionMessageLifetimeUsingBlockhash(freezeLatestBlockhash, tx),
  (tx) => appendTransactionMessageInstructions([freezeInstruction], tx),
);

const signedFreezeTxMessage =
  await signTransactionMessageWithSigners(freezeTxMessage);

const signedFreezeTxMessageWithLifetime =
  signedFreezeTxMessage as typeof signedFreezeTxMessage & {
    lifetimeConstraint: {
      lastValidBlockHeight: bigint;
    };
  };

await sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions })(
  signedFreezeTxMessageWithLifetime,
  { commitment: "confirmed" },
);

const freezeTxSignature = getSignatureFromTransaction(
  signedFreezeTxMessageWithLifetime,
);

console.log("\nFreeze Account Transaction Signature:", freezeTxSignature);

const freezeTransferInstruction = getTransferInstruction({
  source: associatedTokenAccountAddress,
  destination: recipientAssociatedTokenAddress,
  authority: feePayer.address,
  amount: 500_000_000n,
});

const { value: freezeTransferLatestBlockhash } = await rpc
  .getLatestBlockhash()
  .send();

const freezeTransferTxMessage = pipe(
  createTransactionMessage({ version: 0 }),
  (tx) => setTransactionMessageFeePayerSigner(feePayer, tx),
  (tx) =>
    setTransactionMessageLifetimeUsingBlockhash(
      freezeTransferLatestBlockhash,
      tx,
    ),
  (tx) => appendTransactionMessageInstructions([freezeTransferInstruction], tx),
);

const signedFreezeTransferTxMessage = await signTransactionMessageWithSigners(
  freezeTransferTxMessage,
);

const signedFreezeTransferTxMessageWithLifetime =
  signedFreezeTransferTxMessage as typeof signedFreezeTransferTxMessage & {
    lifetimeConstraint: {
      lastValidBlockHeight: bigint;
    };
  };

await sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions })(
  signedFreezeTransferTxMessageWithLifetime,
  { commitment: "confirmed" },
);

10. npx tsx new-token.tsを実行することで、以下を得る。

10-1. アカウントのフリーズが成功していることの確認。
Freeze Account Transaction Signature: 3jSTXVHzZisbVetMM2FbWbLRh1nJoV7RrdyycxwjWkrwHszUok925rEiMaC3EyyFpFK7eMAoWWF76MJBuUqrZ7nm

10-2. アカウントがフリーズされていることの確認。
      'Program log: Instruction: Transfer',
      'Program log: Error: Account is frozen',


<フリーズの解除(=Thaw)>
11. 以下の送金の命令を削除する。
const freezeTransferInstruction = getTransferInstruction({
    source: associatedTokenAccountAddress,
    destination: recipientAssociatedTokenAddress,
    authority: feePayer.address,
    amount: 500_000_000n,
  });
  
  const { value: freezeTransferLatestBlockhash } = await rpc
    .getLatestBlockhash()
    .send();
  
  const freezeTransferTxMessage = pipe(
    createTransactionMessage({ version: 0 }),
    (tx) => setTransactionMessageFeePayerSigner(feePayer, tx),
    (tx) =>
      setTransactionMessageLifetimeUsingBlockhash(
        freezeTransferLatestBlockhash,
        tx,
      ),
    (tx) => appendTransactionMessageInstructions([freezeTransferInstruction], tx),
  );
  
  const signedFreezeTransferTxMessage = await signTransactionMessageWithSigners(
    freezeTransferTxMessage,
  );
  
  const signedFreezeTransferTxMessageWithLifetime =
    signedFreezeTransferTxMessage as typeof signedFreezeTransferTxMessage & {
      lifetimeConstraint: {
        lastValidBlockHeight: bigint;
      };
    };
  
  await sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions })(
    signedFreezeTransferTxMessageWithLifetime,
    { commitment: "confirmed" },
  );

12.次のようにToken-2022から、getThawAccountInstructionの関数をインポートする。
import {
  getInitializeMintInstruction,
  getInitializeAccount2Instruction,
  getCreateAssociatedTokenInstructionAsync,
  getMintToInstruction,
  getTransferInstruction,
  getFreezeAccountInstruction,
  getThawAccountInstruction,
  getMintSize,
  getTokenSize,
  TOKEN_2022_PROGRAM_ADDRESS,
  findAssociatedTokenPda,
  fetchToken,
} from "@solana-program/token-2022";

13. フリーズの権限を与えられた人のみが、フリーズの解除もできるので、以下の変更は不要。
  const initializeMintInstruction = getInitializeMintInstruction({
    mint: mint.address,
    decimals: 9,
    mintAuthority: feePayer.address,
    freezeAuthority: feePayer.address,
  });

14. new-token.tsに以下のコードを追加。
const thawInstruction = getThawAccountInstruction({
  account: associatedTokenAccountAddress,
  mint: mint.address,
  owner: feePayer.address,
});

const { value: thawLatestBlockhash } = await rpc.getLatestBlockhash().send();

const thawTxMessage = pipe(
  createTransactionMessage({ version: 0 }),
  (tx) => setTransactionMessageFeePayerSigner(feePayer, tx),
  (tx) => setTransactionMessageLifetimeUsingBlockhash(thawLatestBlockhash, tx),
  (tx) => appendTransactionMessageInstructions([thawInstruction], tx),
);

const signedThawTxMessage =
  await signTransactionMessageWithSigners(thawTxMessage);

const signedThawTxMessageWithLifetime =
  signedThawTxMessage as typeof signedThawTxMessage & {
    lifetimeConstraint: {
      lastValidBlockHeight: bigint;
    };
  };

await sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions })(
  signedThawTxMessageWithLifetime,
  { commitment: "confirmed" },
);

const thawTxSignature = getSignatureFromTransaction(
  signedThawTxMessageWithLifetime,
);

console.log("\nThaw Account Transaction Signature:", thawTxSignature);

15. npx tsx new-token.tsを実行することで、アカウントの凍結の解除が行われていることを確認出来る。
Thaw Account Transaction Signature: 4Kfk2688hU9Afx6coKtEiQF27cwRR3haLqhemQD4RjptMMKQhDznbk8HP3vhHVSEprg5F7VioAGqYpsnz8KLbMFC

<トークンのバーン>
16. 次のようにToken-2022から、getBurnCheckedInstructionの関数をインポートする。
import {
  getInitializeMintInstruction,
  getInitializeAccount2Instruction,
  getCreateAssociatedTokenInstructionAsync,
  getMintToInstruction,
  getTransferInstruction,
  getFreezeAccountInstruction,
  getThawAccountInstruction,
  getBurnCheckedInstruction,
  getMintSize,
  getTokenSize,
  TOKEN_2022_PROGRAM_ADDRESS,
  findAssociatedTokenPda,
  fetchToken,
} from "@solana-program/token-2022";

17. new-token.tsに以下のコードを追加。
const ataBeforeBurn = await fetchToken(rpc, associatedTokenAccountAddress, {
  commitment: "confirmed",
});

console.log(
  "\nAssociated Token Account Balance Before Burn:",
  Number(ataBeforeBurn.data.amount) / 1_000_000_000,
);

const burnInstruction = getBurnCheckedInstruction({
  account: associatedTokenAccountAddress,
  mint: mint.address,
  authority: feePayer.address,
  amount: 500_000_000n,
  decimals: 9,
});

const { value: burnLatestBlockhash } = await rpc.getLatestBlockhash().send();

const burnTxMessage = pipe(
  createTransactionMessage({ version: 0 }),
  (tx) => setTransactionMessageFeePayerSigner(feePayer, tx),
  (tx) => setTransactionMessageLifetimeUsingBlockhash(burnLatestBlockhash, tx),
  (tx) => appendTransactionMessageInstructions([burnInstruction], tx),
);

const signedBurnTxMessage =
  await signTransactionMessageWithSigners(burnTxMessage);

const signedBurnTxMessageWithLifetime =
  signedBurnTxMessage as typeof signedBurnTxMessage & {
    lifetimeConstraint: {
      lastValidBlockHeight: bigint;
    };
  };

await sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions })(
  signedBurnTxMessageWithLifetime,
  { commitment: "confirmed" },
);

const burnTxSignature = getSignatureFromTransaction(
  signedBurnTxMessageWithLifetime,
);

console.log("\nBurn Transaction Signature:", burnTxSignature);

const ataAfterBurn = await fetchToken(rpc, associatedTokenAccountAddress, {
  commitment: "confirmed",
});

console.log(
  "Associated Token Account Balance After Burn:",
  Number(ataAfterBurn.data.amount) / 1_000_000_000,
);

18. npx tsx new-token.tsを実行することで、バーンしたことを確認出来る。

Associated Token Account Balance Before Burn: 0.5

Burn Transaction Signature: 4wDh1ziFeocRq8VypT9nWfne6thm79wfUWmgZVDPEsimrhSJsTVagQLpQJgZT8TUJd3dgRSX3fytDZvx8UGoLfhe
Associated Token Account Balance After Burn: 0


<アカウントのクローズ>
クローズする前にはアカウント残高をゼロにする。アカウントをクローズするとrent feeが払い戻される。
クローズの権限者のみがクローズできる。

19. 次のようにToken-2022から、getCloseAccountInstructionの関数をインポートする。
import {
  getInitializeMintInstruction,
  getInitializeAccount2Instruction,
  getCreateAssociatedTokenInstructionAsync,
  getMintToInstruction,
  getTransferInstruction,
  getFreezeAccountInstruction,
  getThawAccountInstruction,
  getBurnCheckedInstruction,
  getCloseAccountInstruction,
  getMintSize,
  getTokenSize,
  TOKEN_2022_PROGRAM_ADDRESS,
  findAssociatedTokenPda,
  fetchToken,
} from "@solana-program/token-2022";


20. new-token.tsに以下のコードを追加。
const destination = await generateKeyPairSigner();

const { value: destinationBalanceBeforeClose } = await rpc
  .getBalance(destination.address, {
    commitment: "confirmed",
  })
  .send();

console.log(
  "\nDestination Account Balance Before Close:",
  Number(destinationBalanceBeforeClose) / 1_000_000_000,
);

const { value: closeLatestBlockhash } = await rpc.getLatestBlockhash().send();

const closeAccountInstruction = getCloseAccountInstruction({
  account: associatedTokenAccountAddress,
  destination: destination.address,
  owner: feePayer.address,
});

const closeAccountTxMessage = pipe(
  createTransactionMessage({ version: 0 }),
  (tx) => setTransactionMessageFeePayerSigner(feePayer, tx),
  (tx) => setTransactionMessageLifetimeUsingBlockhash(closeLatestBlockhash, tx),
  (tx) => appendTransactionMessageInstructions([closeAccountInstruction], tx),
);

const signedCloseAccountTxMessage = await signTransactionMessageWithSigners(
  closeAccountTxMessage,
);

const signedCloseAccountTxMessageWithLifetime =
  signedCloseAccountTxMessage as typeof signedCloseAccountTxMessage & {
    lifetimeConstraint: {
      lastValidBlockHeight: bigint;
    };
  };

await sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions })(
  signedCloseAccountTxMessageWithLifetime,
  { commitment: "confirmed" },
);

const closeAccountTxSignature = getSignatureFromTransaction(
  signedCloseAccountTxMessageWithLifetime,
);

console.log("\nClose Account Transaction Signature:", closeAccountTxSignature);

const { value: destinationBalanceAfterClose } = await rpc
  .getBalance(destination.address, {
    commitment: "confirmed",
  })
  .send();

console.log(
  "Destination Account Balance After Close:",
  Number(destinationBalanceAfterClose) / 1_000_000_000,
);

21. npx tsx new-token.tsを実行することで、アカウントをクローズし、rent feeを受け取ったことを確認出来る。
Destination Account Balance Before Close: 0

Close Account Transaction Signature: 5yfRZLbHQFatjUrfanpUevppgz6Cx19nZTfSzwALbqq7c6PA8bxxk2Enk64X96wf2FxizcVUUmX2tYBSXoBcUT25
Destination Account Balance After Close: 0.00207408