# Algorithm Design

This specification describes the first version of YAFO algorithm. You may create your own implementation that is compatible with it.

## Terminology

**Data chunk:** a buffer with the length of 8 bytes, which is the minimum processing unit in this algorithm.

**Key:** a data chunk, derived from the plain text seed phrase.

**Chunk sum:** a byte calculated by xoring all bytes in a data chunk.

**Current key:** a key used for processing the current data chunk.

## Encryption steps

Before starting the encryption, the current key is set to the initial key. Then, for each chunk, perform the following operations:

- Calculate the chunk sum of the plain data, let `c_sum` be it.
- Calculate the chunk sum of the current key, let `k_sum` be it.
- For each byte in the data-key chunk pair, perform:
  - let `factor_a` be `k_sum` xor the key byte.
  - Rotate the data byte left by `factor_a`.
  - Xor the data byte with the key byte.
- Perform the key rotation with `c_sum`.

## Decryption steps

The algorithm is symmetric, and the decryption is the reverse operation of the encryption.

## Key rotation

The current key must be rotated after being used to encrypt a chunk. To rotate a key, the process requires a chunk sum (`c_sum`) as the input along with the key. The whole process has 2 steps:

- For each byte in the key chunk, perform:
  - Xor the byte with `c_sum`.
  - Rotate `c_sum` left once.
- Rotate the bytes in the key chunk left once such that the first byte is move to the end of the key chunk.

## Derivation of the initial key

The initial key is calculated from a plain text that is used as the seed phrase. The string is encoded with UTF-8 and then hashed using SHA-1. Take the first 8 bytes of the hash result and name it **key hash**. The initial key is calculated by bitwise-xoring **key hash** with `[0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x80]`.
