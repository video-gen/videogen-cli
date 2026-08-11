import { createServer, type IncomingMessage, type ServerResponse } from "node:http";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { createCodeVerifier, createState, deriveCodeChallenge } from "./pkce";
import {
  type StoredCredentials,
  readStoredCredentials,
  resolveApiBaseUrl,
  writeStoredCredentials,
} from "./credentials";

const execFileAsync = promisify(execFile);

/** Matches first-party integrations: omit `openid` (RS256 ID-token requirement). */
export const OAUTH_SCOPE = "email profile";

const ACCESS_TOKEN_SKEW_MS = 60 * 1000;
const LOOPBACK_TIMEOUT_MS = 5 * 60 * 1000;
const LOOPBACK_PATH = "/callback";

export type AuthServerEndpoints = {
  authorizationEndpoint: string;
  tokenEndpoint: string;
  registrationEndpoint: string | null;
};

type TokenResponse = {
  access_token: string;
  refresh_token?: string | null;
  expires_in?: number;
};

const trimTrailingSlash = (value: string): string => value.replace(/\/+$/, "");

const asRecord = (value: unknown): Record<string, unknown> | null => {
  if (value == null || typeof value !== "object") {
    return null;
  }
  return value as Record<string, unknown>;
};

const getString = (record: Record<string, unknown>, key: string): string | null => {
  const value = record[key];
  return typeof value === "string" ? value : null;
};

const fetchJson = async (url: string): Promise<unknown> => {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Request failed (${response.status}) for ${url}`);
  }
  return response.json();
};

const metadataCandidateUrls = (issuer: string): string[] => {
  const url = new URL(issuer);
  const path = url.pathname.replace(/\/$/, "");
  const { origin } = url;

  return [
    `${origin}${path}/.well-known/oauth-authorization-server`,
    `${origin}/.well-known/oauth-authorization-server${path}`,
    `${origin}${path}/.well-known/openid-configuration`,
    `${origin}/.well-known/openid-configuration${path}`,
  ];
};

export const discoverAuthServerEndpoints = async ({
  apiBaseUrl,
}: {
  apiBaseUrl: string;
}): Promise<AuthServerEndpoints> => {
  const prm = asRecord(
    await fetchJson(`${trimTrailingSlash(apiBaseUrl)}/.well-known/oauth-protected-resource`),
  );
  if (prm == null) {
    throw new Error("Protected-resource metadata was not a JSON object.");
  }

  const authServers = prm["authorization_servers"];
  const issuer =
    Array.isArray(authServers) && typeof authServers[0] === "string" ? authServers[0] : null;
  if (issuer == null) {
    throw new Error(
      "No authorization server advertised by the API. OAuth login is not available for this base URL.",
    );
  }

  for (const candidate of metadataCandidateUrls(issuer)) {
    try {
      const meta = asRecord(await fetchJson(candidate));
      if (meta == null) {
        continue;
      }
      const authorizationEndpoint = getString(meta, "authorization_endpoint");
      const tokenEndpoint = getString(meta, "token_endpoint");
      if (authorizationEndpoint == null || tokenEndpoint == null) {
        continue;
      }
      return {
        authorizationEndpoint,
        tokenEndpoint,
        registrationEndpoint: getString(meta, "registration_endpoint"),
      };
    } catch {
      // Try the next discovery URL.
    }
  }

  throw new Error("Could not load authorization server metadata from the advertised issuer.");
};

export const registerDynamicClient = async ({
  registrationEndpoint,
  redirectUri,
  scope,
}: {
  registrationEndpoint: string;
  redirectUri: string;
  scope: string;
}): Promise<string> => {
  const response = await fetch(registrationEndpoint, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      client_name: "VideoGen CLI",
      redirect_uris: [redirectUri],
      grant_types: ["authorization_code", "refresh_token"],
      response_types: ["code"],
      token_endpoint_auth_method: "none",
      scope,
    }),
  });

  if (!response.ok) {
    const detail = await response.text().catch(() => "");
    throw new Error(
      `Dynamic client registration failed (${response.status}): ${detail.slice(0, 300)}`,
    );
  }

  const body = asRecord(await response.json());
  const clientId = body == null ? null : getString(body, "client_id");
  if (clientId == null || clientId.length === 0) {
    throw new Error("Dynamic client registration did not return a client_id.");
  }
  return clientId;
};

const postForm = async ({
  endpoint,
  params,
}: {
  endpoint: string;
  params: Record<string, string>;
}): Promise<TokenResponse> => {
  const response = await fetch(endpoint, {
    method: "POST",
    headers: { "Content-Type": "application/x-www-form-urlencoded" },
    body: new URLSearchParams(params).toString(),
  });

  if (!response.ok) {
    const detail = await response.text().catch(() => "");
    throw new Error(`Token request failed (${response.status}): ${detail.slice(0, 300)}`);
  }

  const body = asRecord(await response.json());
  if (body == null) {
    throw new Error("Token response was not a JSON object.");
  }

  const accessToken = getString(body, "access_token");
  if (accessToken == null) {
    throw new Error("Token response did not include an access_token.");
  }

  const refreshToken = getString(body, "refresh_token");
  const expiresInRaw = body["expires_in"];
  const expiresIn = typeof expiresInRaw === "number" ? expiresInRaw : undefined;

  return {
    access_token: accessToken,
    refresh_token: refreshToken,
    expires_in: expiresIn,
  };
};

export const exchangeCodeForTokens = async ({
  tokenEndpoint,
  code,
  codeVerifier,
  clientId,
  redirectUri,
}: {
  tokenEndpoint: string;
  code: string;
  codeVerifier: string;
  clientId: string;
  redirectUri: string;
}): Promise<TokenResponse> =>
  postForm({
    endpoint: tokenEndpoint,
    params: {
      grant_type: "authorization_code",
      code,
      redirect_uri: redirectUri,
      client_id: clientId,
      code_verifier: codeVerifier,
    },
  });

export const refreshAccessToken = async ({
  tokenEndpoint,
  refreshToken,
  clientId,
}: {
  tokenEndpoint: string;
  refreshToken: string;
  clientId: string;
}): Promise<TokenResponse> =>
  postForm({
    endpoint: tokenEndpoint,
    params: {
      grant_type: "refresh_token",
      refresh_token: refreshToken,
      client_id: clientId,
    },
  });

const fetchAccountEmail = async ({
  apiBaseUrl,
  accessToken,
}: {
  apiBaseUrl: string;
  accessToken: string;
}): Promise<string | null> => {
  try {
    const response = await fetch(`${trimTrailingSlash(apiBaseUrl)}/v1/me`, {
      headers: { Authorization: `Bearer ${accessToken}` },
    });
    if (!response.ok) {
      return null;
    }
    const body = asRecord(await response.json());
    return body == null ? null : getString(body, "email");
  } catch {
    return null;
  }
};

const buildAuthUrl = ({
  authorizationEndpoint,
  params,
}: {
  authorizationEndpoint: string;
  params: Record<string, string>;
}): string => {
  const url = new URL(authorizationEndpoint);
  for (const [key, value] of Object.entries(params)) {
    url.searchParams.set(key, value);
  }
  return url.toString();
};

const loopbackSuccessHtml = (): string =>
  `<!DOCTYPE html><html><head><meta charset="utf-8"><title>VideoGen</title></head><body style="font-family:-apple-system,Segoe UI,Roboto,sans-serif;background:#1e1e1e;color:#e6e6e6;display:flex;align-items:center;justify-content:center;height:100vh;margin:0"><p>Signed in to VideoGen. You can close this tab and return to the terminal.</p></body></html>`;

const loopbackErrorHtml = (): string =>
  `<!DOCTYPE html><html><head><meta charset="utf-8"><title>VideoGen</title></head><body style="font-family:-apple-system,Segoe UI,Roboto,sans-serif;background:#1e1e1e;color:#e6e6e6;display:flex;align-items:center;justify-content:center;height:100vh;margin:0"><p>Sign-in failed. You can close this tab and try again in the terminal.</p></body></html>`;

type LoopbackServer = {
  redirectUri: string;
  waitForCode: Promise<string>;
  close: () => void;
};

/**
 * Binds an ephemeral loopback server on 127.0.0.1. Resolves `redirectUri` as
 * soon as the port is known so DCR can register it before the browser opens.
 */
export const startLoopbackListener = ({
  expectedState,
}: {
  expectedState: string;
}): Promise<LoopbackServer> =>
  new Promise((resolveListen, rejectListen) => {
    let settled = false;
    let timeout: ReturnType<typeof setTimeout> | null = null;

    const waitForCode = new Promise<string>((resolveCode, rejectCode) => {
      const server = createServer((req: IncomingMessage, res: ServerResponse) => {
        try {
          const requestUrl = new URL(req.url ?? "/", "http://127.0.0.1");
          if (requestUrl.pathname !== LOOPBACK_PATH) {
            res.writeHead(404);
            res.end();
            return;
          }

          const error = requestUrl.searchParams.get("error");
          const state = requestUrl.searchParams.get("state");
          const code = requestUrl.searchParams.get("code");

          res.writeHead(200, { "Content-Type": "text/html; charset=utf-8" });
          res.end(error != null ? loopbackErrorHtml() : loopbackSuccessHtml());

          if (timeout != null) {
            clearTimeout(timeout);
          }
          server.close();

          if (error != null) {
            rejectCode(new Error(`Authorization denied: ${error}`));
            return;
          }
          if (state !== expectedState) {
            rejectCode(new Error("OAuth state mismatch; aborting for safety."));
            return;
          }
          if (code == null || code.length === 0) {
            rejectCode(new Error("No authorization code returned."));
            return;
          }

          resolveCode(code);
        } catch (err: unknown) {
          if (timeout != null) {
            clearTimeout(timeout);
          }
          server.close();
          rejectCode(err instanceof Error ? err : new Error(String(err)));
        }
      });

      timeout = setTimeout(() => {
        server.close();
        rejectCode(new Error("Timed out waiting for VideoGen sign-in."));
      }, LOOPBACK_TIMEOUT_MS);

      server.on("error", (err) => {
        if (timeout != null) {
          clearTimeout(timeout);
        }
        if (!settled) {
          settled = true;
          rejectListen(err);
          return;
        }
        rejectCode(err);
      });

      server.listen(0, "127.0.0.1", () => {
        const address = server.address();
        const port =
          address != null && typeof address === "object" ? address.port : null;
        if (port == null) {
          if (timeout != null) {
            clearTimeout(timeout);
          }
          server.close();
          if (!settled) {
            settled = true;
            rejectListen(new Error("Failed to bind loopback server."));
          }
          return;
        }

        settled = true;
        resolveListen({
          redirectUri: `http://127.0.0.1:${port}${LOOPBACK_PATH}`,
          waitForCode,
          close: () => {
            if (timeout != null) {
              clearTimeout(timeout);
            }
            server.close();
          },
        });
      });
    });
  });

export const openInBrowser = async (url: string): Promise<void> => {
  const platform = process.platform;
  try {
    if (platform === "darwin") {
      await execFileAsync("open", [url]);
      return;
    }
    if (platform === "win32") {
      await execFileAsync("cmd", ["/c", "start", "", url]);
      return;
    }
    await execFileAsync("xdg-open", [url]);
  } catch {
    throw new Error(`Could not open a browser automatically. Open this URL manually:\n${url}`);
  }
};

const tokensToStoredCredentials = ({
  tokens,
  clientId,
  apiBaseUrl,
  email,
}: {
  tokens: TokenResponse;
  clientId: string;
  apiBaseUrl: string;
  email: string | null;
}): StoredCredentials => {
  const expiresInSeconds = tokens.expires_in ?? 3600;
  return {
    version: 1,
    accessToken: tokens.access_token,
    refreshToken: tokens.refresh_token ?? null,
    expiresAt: Date.now() + expiresInSeconds * 1000,
    clientId,
    apiBaseUrl: trimTrailingSlash(apiBaseUrl),
    email,
  };
};

/**
 * Interactive OAuth 2.1 authorization-code + PKCE loopback login.
 * Registers a public client via DCR, opens the system browser, caches tokens.
 */
export const runInteractiveLogin = async ({
  baseUrl,
}: {
  baseUrl: string | undefined;
}): Promise<{ email: string | null; apiBaseUrl: string }> => {
  const apiBaseUrl = resolveApiBaseUrl(baseUrl);
  const endpoints = await discoverAuthServerEndpoints({ apiBaseUrl });

  if (endpoints.registrationEndpoint == null) {
    throw new Error(
      "Authorization server does not advertise Dynamic Client Registration. Cannot complete login.",
    );
  }

  const codeVerifier = createCodeVerifier();
  const codeChallenge = deriveCodeChallenge(codeVerifier);
  const state = createState();

  const loopback = await startLoopbackListener({ expectedState: state });

  try {
    const clientId = await registerDynamicClient({
      registrationEndpoint: endpoints.registrationEndpoint,
      redirectUri: loopback.redirectUri,
      scope: OAUTH_SCOPE,
    });

    const authUrl = buildAuthUrl({
      authorizationEndpoint: endpoints.authorizationEndpoint,
      params: {
        response_type: "code",
        client_id: clientId,
        redirect_uri: loopback.redirectUri,
        scope: OAUTH_SCOPE,
        state,
        code_challenge: codeChallenge,
        code_challenge_method: "S256",
      },
    });

    process.stderr.write("Opening browser to sign in to VideoGen…\n");
    process.stderr.write(`If nothing opens, visit:\n${authUrl}\n`);

    try {
      await openInBrowser(authUrl);
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      process.stderr.write(`${message}\n`);
    }

    const code = await loopback.waitForCode;

    const tokens = await exchangeCodeForTokens({
      tokenEndpoint: endpoints.tokenEndpoint,
      code,
      codeVerifier,
      clientId,
      redirectUri: loopback.redirectUri,
    });

    const email = await fetchAccountEmail({
      apiBaseUrl,
      accessToken: tokens.access_token,
    });

    writeStoredCredentials({
      credentials: tokensToStoredCredentials({
        tokens,
        clientId,
        apiBaseUrl,
        email,
      }),
    });

    return { email, apiBaseUrl };
  } catch (err: unknown) {
    loopback.close();
    throw err;
  }
};

/**
 * Returns a usable bearer credential from stored OAuth tokens, refreshing if
 * needed. Returns null when there is no session or refresh fails.
 */
export const getValidStoredAccessToken = async ({
  apiBaseUrl,
}: {
  apiBaseUrl: string;
}): Promise<string | null> => {
  const stored = readStoredCredentials({ apiBaseUrl });
  if (stored == null) {
    return null;
  }

  if (Date.now() + ACCESS_TOKEN_SKEW_MS < stored.expiresAt && stored.accessToken.length > 0) {
    return stored.accessToken;
  }

  if (stored.refreshToken == null || stored.refreshToken.length === 0) {
    return null;
  }

  try {
    const endpoints = await discoverAuthServerEndpoints({ apiBaseUrl });
    const refreshed = await refreshAccessToken({
      tokenEndpoint: endpoints.tokenEndpoint,
      refreshToken: stored.refreshToken,
      clientId: stored.clientId,
    });
    const nextRefresh = refreshed.refresh_token ?? stored.refreshToken;
    const next = tokensToStoredCredentials({
      tokens: { ...refreshed, refresh_token: nextRefresh },
      clientId: stored.clientId,
      apiBaseUrl,
      email: stored.email,
    });
    writeStoredCredentials({ credentials: next });
    return next.accessToken;
  } catch {
    return null;
  }
};
