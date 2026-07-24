import { getBooleanFlag, getNumberFlag, getStringFlag } from "../args";

export const requireStringFlag = (
  flags: Record<string, string | boolean>,
  name: string,
): string => {
  const value = getStringFlag(flags, name);
  if (value == null || value.length === 0) {
    const kebab = name.replace(/[A-Z]/g, (c) => `-${c.toLowerCase()}`);
    throw new Error(`Missing required flag --${kebab}.`);
  }
  return value;
};

export const paginationFlags = (
  flags: Record<string, string | boolean>,
): { limit?: number; cursor?: string } => {
  const limit = getNumberFlag(flags, "limit");
  const cursor = getStringFlag(flags, "cursor");
  return {
    ...(limit != null ? { limit } : {}),
    ...(cursor != null ? { cursor } : {}),
  };
};

export const selfOnlyFlag = (flags: Record<string, string | boolean>): { selfOnly?: boolean } => {
  const selfOnly = getBooleanFlag(flags, "selfOnly");
  return selfOnly != null ? { selfOnly } : {};
};
