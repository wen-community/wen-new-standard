/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  Codec,
  Decoder,
  Encoder,
  combineCodec,
  getArrayDecoder,
  getArrayEncoder,
  getStructDecoder,
  getStructEncoder,
  getU16Decoder,
  getU16Encoder,
} from '@solana/web3.js';
import {
  CreatorWithShare,
  CreatorWithShareArgs,
  getCreatorWithShareDecoder,
  getCreatorWithShareEncoder,
} from '.';

export type UpdateRoyaltiesArgs = {
  royaltyBasisPoints: number;
  creators: Array<CreatorWithShare>;
};

export type UpdateRoyaltiesArgsArgs = {
  royaltyBasisPoints: number;
  creators: Array<CreatorWithShareArgs>;
};

export function getUpdateRoyaltiesArgsEncoder(): Encoder<UpdateRoyaltiesArgsArgs> {
  return getStructEncoder([
    ['royaltyBasisPoints', getU16Encoder()],
    ['creators', getArrayEncoder(getCreatorWithShareEncoder())],
  ]);
}

export function getUpdateRoyaltiesArgsDecoder(): Decoder<UpdateRoyaltiesArgs> {
  return getStructDecoder([
    ['royaltyBasisPoints', getU16Decoder()],
    ['creators', getArrayDecoder(getCreatorWithShareDecoder())],
  ]);
}

export function getUpdateRoyaltiesArgsCodec(): Codec<
  UpdateRoyaltiesArgsArgs,
  UpdateRoyaltiesArgs
> {
  return combineCodec(
    getUpdateRoyaltiesArgsEncoder(),
    getUpdateRoyaltiesArgsDecoder()
  );
}
