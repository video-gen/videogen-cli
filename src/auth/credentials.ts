import { mkdirSync, readFileSync, unlinkSync, writeFileSync, existsSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, join } from "node:path";
import { createHash } from "node:crypto";

export const DEFAULT_API_BASE_URL = "https://api.videogen.io";

export type StoredCredentials = {
  /** Schema version for future migrations. */
  version: 1;
  accessToken: string;
  refreshToken: string | null;
  /** Unix ms when the access token expires. */
  expiresAt: number;
  /** Public OAuth client id used for refresh (from DCR). */
  clientId: string;
  /** API base URL these credentials were issued against. */
  apiBaseUrl: string;
  email: string | null;
};

const trimTrailingSlash = (value: string): string => value.replace(/\/+$/, "");

/**
 * Credential store directory. Override with `VIDEOGEN_CLI_CONFIG_DIR` for tests
 * or to isolate sessions (runtime, not build-time).
 */
export const getCredentialsDir = (): string => {
  const override = process.env["VIDEOGEN_CLI_CONFIG_DIR"];
  if (override != null && override.length > 0) {
    return override;
  }
  return join(homedir(), ".videogen", "cli");
};

export const resolveApiBaseUrl = (baseUrl: string | undefined): string => {
  const fromEnv = process.env["VIDEOGEN_BASE_URL"];
  const raw = baseUrl ?? fromEnv ?? DEFAULT_API_BASE_URL;
  return trimTrailingSlash(raw);
};

/**
 * Credentials are namespaced by API host so DEV / PRERELEASE / PROD sessions
 * never clobber each other when the same machine targets multiple envs.
 */
export const getCredentialsPath = ({ apiBaseUrl }: { apiBaseUrl: string }): string => {
  const hostKey = createHash("sha256").update(trimTrailingSlash(apiBaseUrl)).digest("hex").slice(0, 16);
  return join(getCredentialsDir(), `credentials.${hostKey}.json`);
};

const safeJsonParse = (text: string): unknown => {
  try {
    return JSON.parse(text);
  } catch {
    return null;
  }
};

const parseStoredCredentials = (value: unknown): StoredCredentials | null => {
  if (value == null || typeof value !== "object") {
    return null;
  }

  const record = value as Record<string, unknown>;
  if (record["version"] !== 1) {
    return null;
  }

  const accessToken = record["accessToken"];
  const refreshToken = record["refreshToken"];
  const expiresAt = record["expiresAt"];
  const clientId = record["clientId"];
  const apiBaseUrl = record["apiBaseUrl"];
  const email = record["email"];

  if (typeof accessToken !== "string" || accessToken.length === 0) {
    return null;
  }
  if (refreshToken != null && typeof refreshToken !== "string") {
    return null;
  }
  if (typeof expiresAt !== "number" || !Number.isFinite(expiresAt)) {
    return null;
  }
  if (typeof clientId !== "string" || clientId.length === 0) {
    return null;
  }
  if (typeof apiBaseUrl !== "string" || apiBaseUrl.length === 0) {
    return null;
  }
  if (email != null && typeof email !== "string") {
    return null;
  }

  return {
    version: 1,
    accessToken,
    refreshToken: refreshToken ?? null,
    expiresAt,
    clientId,
    apiBaseUrl,
    email: email ?? null,
  };
};

export const readStoredCredentials = ({
  apiBaseUrl,
}: {
  apiBaseUrl: string;
}): StoredCredentials | null => {
  const path = getCredentialsPath({ apiBaseUrl });
  if (!existsSync(path)) {
    return null;
  }

  try {
    const parsed = safeJsonParse(readFileSync(path, "utf8"));
    const credentials = parseStoredCredentials(parsed);
    if (credentials == null) {
      return null;
    }
    // Ignore credentials stamped for a different base URL (hash collision / hand edit).
    if (trimTrailingSlash(credentials.apiBaseUrl) !== trimTrailingSlash(apiBaseUrl)) {
      return null;
    }
    return credentials;
  } catch {
    return null;
  }
};

export const writeStoredCredentials = ({
  credentials,
}: {
  credentials: StoredCredentials;
}): void => {
  const path = getCredentialsPath({ apiBaseUrl: credentials.apiBaseUrl });
  mkdirSync(dirname(path), { recursive: true, mode: 0o700 });
  writeFileSync(path, `${JSON.stringify(credentials, null, 2)}\n`, { mode: 0o600 });
};

export const clearStoredCredentials = ({ apiBaseUrl }: { apiBaseUrl: string }): boolean => {
  const path = getCredentialsPath({ apiBaseUrl });
  if (!existsSync(path)) {
    return false;
  }

  try {
    unlinkSync(path);
    return true;
  } catch {
    return false;
  }
};
