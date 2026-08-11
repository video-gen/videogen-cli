import type { ParsedArgs } from "../args";
import { clearStoredCredentials, resolveApiBaseUrl } from "../auth/credentials";
import { runInteractiveLogin } from "../auth/oauth";
import { printJson } from "../output";
import type { ResourceCommands } from "./types";

export const runAuthLogin = async ({ args }: { args: ParsedArgs }): Promise<void> => {
  const { email, apiBaseUrl } = await runInteractiveLogin({
    baseUrl: args.globals.baseUrl,
  });

  printJson({
    ok: true,
    email,
    apiBaseUrl,
    message:
      email != null
        ? `Signed in as ${email}. Credentials saved for future CLI commands.`
        : "Signed in. Credentials saved for future CLI commands.",
  });
};

export const runAuthLogout = async ({ args }: { args: ParsedArgs }): Promise<void> => {
  const apiBaseUrl = resolveApiBaseUrl(args.globals.baseUrl);
  const cleared = clearStoredCredentials({ apiBaseUrl });

  printJson({
    ok: true,
    apiBaseUrl,
    cleared,
    message: cleared
      ? "Signed out. Cached OAuth credentials removed."
      : "No cached OAuth credentials to remove.",
  });
};

/**
 * Registered for help / surface discovery. Prefer `runAuthLogin` / `runAuthLogout`
 * from `cli.ts` so these never require a client.
 */
export const authCommands: ResourceCommands = {
  login: async ({ args }) => runAuthLogin({ args }),
  logout: async ({ args }) => runAuthLogout({ args }),
};
