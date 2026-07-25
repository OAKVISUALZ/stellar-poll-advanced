import { Buffer } from "buffer";
import {
  xdr,
  hash,
  StrKey,
  Address,
  Operation,
  TransactionBuilder,
  BASE_FEE,
  Networks,
} from "@stellar/stellar-sdk";
import { Client } from "@stellar/stellar-sdk/contract";
import { Server as SorobanRpcServer } from "@stellar/stellar-sdk/rpc";
import { StellarWalletsKit } from "@creit.tech/stellar-wallets-kit/sdk";

const RPC_URL = "https://soroban-testnet.stellar.org";
const NETWORK_PASSPHRASE = Networks.TESTNET;
const CONTRACT_KEY = "stellar_live_poll_contract_id";

const rpcServer = new SorobanRpcServer(RPC_URL);

// eslint-disable-next-line @typescript-eslint/no-explicit-any
let cachedWasm: any = null;

async function signWallet(
  xdrStr: string,
  opts?: { networkPassphrase?: string; address?: string }
): Promise<{ signedTxXdr: string; signerAddress?: string }> {
  return StellarWalletsKit.signTransaction(xdrStr, {
    networkPassphrase: opts?.networkPassphrase ?? NETWORK_PASSPHRASE,
    address: opts?.address,
  });
}

async function getWasm(): Promise<Buffer> {
  if (cachedWasm) return cachedWasm;
  const res = await fetch("/live_poll.wasm");
  if (!res.ok) throw new Error("Failed to load contract WASM");
  const buf = await res.arrayBuffer();
  cachedWasm = Buffer.from(buf);
  return cachedWasm;
}

function computeContractId(
  salt: Buffer,
  deployerAddress: string
): string {
  const networkId = hash(Buffer.from(NETWORK_PASSPHRASE));
  const addr = Address.fromString(deployerAddress);

  const preimageFromAddr = new xdr.ContractIdPreimageFromAddress({
    address: addr.toScAddress(),
    salt: salt,
  });

  const contractIdPreimage =
    xdr.ContractIdPreimage.contractIdPreimageFromAddress(preimageFromAddr);

  const hashPreimageContractId = new xdr.HashIdPreimageContractId({
    networkId,
    contractIdPreimage,
  });

  const hashPreimage =
    xdr.HashIdPreimage.envelopeTypeContractId(hashPreimageContractId);

  const contractIdBytes = hash(hashPreimage.toXDR());
  return StrKey.encodeContract(contractIdBytes);
}

async function waitForConfirmation(txHash: string): Promise<void> {
  for (let i = 0; i < 30; i++) {
    const resp = await rpcServer.getTransaction(txHash);
    if (resp.status === "SUCCESS") return;
    if (resp.status === "FAILED")
      throw new Error("Transaction failed on-chain");
    await new Promise((r) => setTimeout(r, 2000));
  }
  throw new Error("Transaction timed out");
}

export async function deployContract(
  deployerAddress: string
): Promise<{ contractId: string; hash: string }> {
  const wasm = await getWasm();
  const wasmHashBuf = Buffer.from(hash(wasm));

  // Step 1: Upload WASM
  const account1 = await rpcServer.getAccount(deployerAddress);
  const uploadTx = new TransactionBuilder(account1, {
    fee: BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(Operation.uploadContractWasm({ wasm }))
    .setTimeout(300)
    .build();

  const preparedUpload = await rpcServer.prepareTransaction(uploadTx);
  const { signedTxXdr: signedUpload } = await signWallet(
    preparedUpload.toXDR(),
    { address: deployerAddress }
  );
  const uploaded = TransactionBuilder.fromXDR(signedUpload, NETWORK_PASSPHRASE);
  const uploadResult = await rpcServer.sendTransaction(uploaded);
  if (uploadResult.status !== "PENDING")
    throw new Error("Upload submission failed");
  await waitForConfirmation(uploadResult.hash);

  // Step 2: Compute contract ID & deploy
  const salt = hash(Buffer.from(deployerAddress + Date.now().toString()));
  const contractId = computeContractId(salt, deployerAddress);

  const account2 = await rpcServer.getAccount(deployerAddress);
  const createTx = new TransactionBuilder(account2, {
    fee: BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(
      Operation.createCustomContract({
        address: Address.fromString(deployerAddress),
        wasmHash: wasmHashBuf,
        salt,
      })
    )
    .setTimeout(300)
    .build();

  const preparedCreate = await rpcServer.prepareTransaction(createTx);
  const { signedTxXdr: signedCreate } = await signWallet(
    preparedCreate.toXDR(),
    { address: deployerAddress }
  );
  const created = TransactionBuilder.fromXDR(signedCreate, NETWORK_PASSPHRASE);
  const createResult = await rpcServer.sendTransaction(created);
  if (createResult.status !== "PENDING")
    throw new Error("Create submission failed");
  await waitForConfirmation(createResult.hash);

  storeContractId(contractId);
  return { contractId, hash: createResult.hash };
}

export function getStoredContractId(): string | null {
  return localStorage.getItem(CONTRACT_KEY);
}

export function storeContractId(contractId: string): void {
  localStorage.setItem(CONTRACT_KEY, contractId);
}

export function clearContractId(): void {
  localStorage.removeItem(CONTRACT_KEY);
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type AnyClient = Client & Record<string, (...args: any[]) => Promise<any>>;

async function makeClient(
  contractId: string,
  publicKey?: string
): Promise<AnyClient> {
  const opts: Record<string, unknown> = {
    rpcUrl: RPC_URL,
    networkPassphrase: NETWORK_PASSPHRASE,
    contractId,
  };
  if (publicKey) {
    opts.publicKey = publicKey;
    opts.signTransaction = signWallet;
  }
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const client = await Client.from(opts as any);
  return client as AnyClient;
}

export async function initializeContract(
  contractId: string,
  publicKey: string
): Promise<{ hash: string }> {
  const client = await makeClient(contractId, publicKey);
  const tx = await client.initialize();
  const sent = await tx.signAndSend();
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const resp = (sent as any).sendTransactionResponse as
    | { hash: string }
    | undefined;
  return { hash: resp?.hash ?? "" };
}

export async function createPollOnChain(
  contractId: string,
  publicKey: string,
  question: string,
  options: string[]
): Promise<{ hash: string; pollId: number }> {
  const client = await makeClient(contractId, publicKey);
  const tx = await client.create_poll({
    creator: publicKey,
    question,
    options,
  });
  const sent = await tx.signAndSend();
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const resp = (sent as any).sendTransactionResponse as
    | { hash: string }
    | undefined;
  return {
    hash: resp?.hash ?? "",
    pollId: typeof tx.result === "number" ? tx.result : 0,
  };
}

export async function voteOnPoll(
  contractId: string,
  publicKey: string,
  pollId: number,
  optionIndex: number
): Promise<{ hash: string }> {
  const client = await makeClient(contractId, publicKey);
  const tx = await client.vote({
    voter: publicKey,
    poll_id: pollId,
    option_index: optionIndex,
  });
  const sent = await tx.signAndSend();
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const resp = (sent as any).sendTransactionResponse as
    | { hash: string }
    | undefined;
  return { hash: resp?.hash ?? "" };
}

export interface PollData {
  id: number;
  question: string;
  options: string[];
  votes: number[];
  creator: string;
  totalVotes: number;
  active: boolean;
}

export async function getPollData(
  contractId: string,
  pollId: number
): Promise<PollData | null> {
  try {
    const client = await makeClient(contractId);
    const tx = await client.get_poll({ poll_id: pollId });
    const r = tx.result;
    if (!r || typeof r !== "object") return null;
    return {
      id: Number((r as Record<string, unknown>).id ?? pollId),
      question: String((r as Record<string, unknown>).question ?? ""),
      options: Array.isArray((r as Record<string, unknown>).options)
        ? ((r as Record<string, unknown>).options as unknown[]).map(String)
        : [],
      votes: Array.isArray((r as Record<string, unknown>).votes)
        ? ((r as Record<string, unknown>).votes as unknown[]).map(Number)
        : [],
      creator: String((r as Record<string, unknown>).creator ?? ""),
      totalVotes: Number((r as Record<string, unknown>).total_votes ?? 0),
      active: Boolean((r as Record<string, unknown>).active ?? true),
    };
  } catch {
    return null;
  }
}

export async function getPollResults(
  contractId: string,
  pollId: number
): Promise<number[]> {
  try {
    const client = await makeClient(contractId);
    const tx = await client.get_results({ poll_id: pollId });
    return Array.isArray(tx.result) ? (tx.result as number[]).map(Number) : [];
  } catch {
    return [];
  }
}

export async function hasVotedOnPoll(
  contractId: string,
  pollId: number,
  voter: string
): Promise<boolean> {
  try {
    const client = await makeClient(contractId);
    const tx = await client.has_voted({ poll_id: pollId, voter });
    return Boolean(tx.result);
  } catch {
    return false;
  }
}

export async function getPollCount(contractId: string): Promise<number> {
  try {
    const client = await makeClient(contractId);
    const tx = await client.get_poll_count();
    return Number(tx.result);
  } catch {
    return 0;
  }
}

export async function closePollOnChain(
  contractId: string,
  publicKey: string,
  pollId: number
): Promise<{ hash: string }> {
  const client = await makeClient(contractId, publicKey);
  const tx = await client.close_poll({ caller: publicKey, poll_id: pollId });
  const sent = await tx.signAndSend();
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const resp = (sent as any).sendTransactionResponse as
    | { hash: string }
    | undefined;
  return { hash: resp?.hash ?? "" };
}

export function shortenAddress(address: string, chars = 6): string {
  return `${address.slice(0, chars)}...${address.slice(-chars)}`;
}

export function isValidStellarAddress(address: string): boolean {
  return /^G[A-Z0-9]{55}$/.test(address);
}
