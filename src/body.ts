import { readFileSync } from "node:fs";
import { getStringFlag } from "./args";

const dropUndefined = (obj: Record<string, unknown>): Record<string, unknown> => {
  const result: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(obj)) {
    if (value !== undefined) {
      result[key] = value;
    }
  }
  return result;
};

const parseJsonObject = (raw: string, label: string): Record<string, unknown> => {
  const value: unknown = JSON.parse(raw);
  if (value == null || typeof value !== "object" || Array.isArray(value)) {
    throw new Error(`${label} must be a JSON object.`);
  }

  const result: Record<string, unknown> = {};
  for (const [key, entry] of Object.entries(value)) {
    result[key] = entry;
  }
  return result;
};

const readStdin = async (): Promise<string> => {
  const chunks: Buffer[] = [];
  for await (const chunk of process.stdin) {
    chunks.push(Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk));
  }
  return Buffer.concat(chunks).toString("utf8");
};

/**
 * Resolves a JSON request body from `--body`, `--body @path`, or stdin.
 * Path/query flags in `extra` are merged on top (SDK single request object).
 * Callers cast with `as` to the SDK request type — the API server validates and
 * returns a type error if the body is invalid.
 */
export const resolveRequest = async ({
  flags,
  extra,
  requireBody,
}: {
  flags: Record<string, string | boolean>;
  extra?: Record<string, unknown>;
  requireBody: boolean;
}): Promise<Record<string, unknown>> => {
  const bodyFlag = getStringFlag(flags, "body");
  let parsedBody: Record<string, unknown> = {};

  if (bodyFlag != null) {
    const raw =
      bodyFlag.startsWith("@") && bodyFlag.length > 1
        ? readFileSync(bodyFlag.slice(1), "utf8")
        : bodyFlag;
    parsedBody = parseJsonObject(raw, "--body");
  } else if (requireBody && !process.stdin.isTTY) {
    const raw = (await readStdin()).trim();
    if (raw.length > 0) {
      parsedBody = parseJsonObject(raw, "stdin body");
    }
  } else if (requireBody) {
    throw new Error("Missing request body. Pass --body '{}' or pipe JSON on stdin.");
  }

  return dropUndefined({ ...parsedBody, ...extra });
};
