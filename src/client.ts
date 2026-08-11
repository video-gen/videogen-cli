import { VideoGen } from "@videogen/sdk";
import { DEFAULT_API_BASE_URL, resolveApiBaseUrl } from "./auth/credentials";
import { getValidStoredAccessToken } from "./auth/oauth";

export type CliClientOptions = {
  apiKey: string | undefined;
  baseUrl: string | undefined;
};

/**
 * Resolves the bearer credential for API calls:
 *   1. `--api-key` flag
 *   2. `VIDEOGEN_API_KEY` env
 *   3. OAuth access token from `videogen login` (refreshed if needed)
 */
export const resolveBearerCredential = async ({
  apiKey,
  baseUrl,
}: CliClientOptions): Promise<{ credential: string; apiBaseUrl: string }> => {
  const apiBaseUrl = resolveApiBaseUrl(baseUrl);

  if (apiKey != null && apiKey.length > 0) {
    return { credential: apiKey, apiBaseUrl };
  }

  const envKey = process.env["VIDEOGEN_API_KEY"];
  if (envKey != null && envKey.length > 0) {
    return { credential: envKey, apiBaseUrl };
  }

  const storedToken = await getValidStoredAccessToken({ apiBaseUrl });
  if (storedToken != null && storedToken.length > 0) {
    return { credential: storedToken, apiBaseUrl };
  }

  throw new Error(
    "Not authenticated. Run `videogen login`, set VIDEOGEN_API_KEY, or pass --api-key.",
  );
};

export const createClient = async ({
  apiKey,
  baseUrl,
}: CliClientOptions): Promise<VideoGen> => {
  const { credential, apiBaseUrl } = await resolveBearerCredential({ apiKey, baseUrl });

  if (apiBaseUrl === DEFAULT_API_BASE_URL) {
    return new VideoGen({ apiKey: credential, clientId: "cli" });
  }

  return new VideoGen({ apiKey: credential, baseUrl: apiBaseUrl, clientId: "cli" });
};
