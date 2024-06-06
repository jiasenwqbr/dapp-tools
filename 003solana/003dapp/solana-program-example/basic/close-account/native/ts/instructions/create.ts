import * as borsh from "borsh";
import { Buffer } from "buffer";

import {
  PublicKey,
  SystemProgram,
  SystemInstruction,
  TransactionInstruction,
} from "@solana/web3.js";
import { MyInstruction } from ".";

export class Create {
  instruction: MyInstruction;
  name: String;
  constructor(props: { instruction: MyInstruction; name: String }) {
    this.instruction = props.instruction;
    this.name = props.name;
  }

  toBuffer() {
    return Buffer.from(borsh.serialize(CreateSchma, this));
  }
  static fromBuffer(buffer: Buffer) {
    return borsh.deserialize(CreateSchma, Create, buffer);
  }
}

export const CreateSchma = new Map([
  [
    Create,
    {
      kind: "struct",
      fields: [
        ["instruction", "u8"],
        ["name", "string"],
      ],
    },
  ],
]);

export function createCreateUserInstruction(
  target: PublicKey,
  payer: PublicKey,
  programId: PublicKey,
  name: string
): TransactionInstruction {
  const instructionObject = new Create({
    instruction: MyInstruction.CreateUser,
    name,
  });
  const tx = new TransactionInstruction({
    keys: [
      { pubkey: target, isSigner: false, isWritable: true },
      { pubkey: payer, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: programId,
    data: instructionObject.toBuffer(),
  });

  return tx;
}
