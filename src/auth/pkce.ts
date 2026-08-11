import { createHash, randomBytes } from "node:crypto";

const base64Url = (input: Buffer): string => input.toString("base64url");

export const createCodeVerifier = (): string => base64Url(randomBytes(32));

export const deriveCodeChallenge = (codeVerifier: string): string =>
  base64Url(createHash("sha256").update(codeVerifier).digest());

export const createState = (): string => base64Url(randomBytes(16));
