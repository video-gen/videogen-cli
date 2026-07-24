export type ParsedArgs = {
  globals: {
    apiKey: string | undefined;
    baseUrl: string | undefined;
    json: boolean;
  };
  resource: string | undefined;
  command: string | undefined;
  positionals: string[];
  flags: Record<string, string | boolean>;
};

const kebabToCamel = (value: string): string =>
  value.replace(/-([a-z])/g, (_match, char: string) => char.toUpperCase());

const parseFlagValue = (raw: string): string | boolean => {
  if (raw === "true") {
    return true;
  }
  if (raw === "false") {
    return false;
  }
  return raw;
};

/**
 * Parses `videogen [globals] <resource> <command> [flags] [positionals]`.
 * Globals: --api-key, --base-url, --json.
 * Flags accept `--name value`, `--name=value`, or bare `--bool` (true).
 */
export const parseArgs = (argv: string[]): ParsedArgs => {
  const globals: ParsedArgs["globals"] = {
    apiKey: undefined,
    baseUrl: undefined,
    json: true,
  };
  const flags: Record<string, string | boolean> = {};
  const positionals: string[] = [];
  let resource: string | undefined;
  let command: string | undefined;

  let i = 0;
  while (i < argv.length) {
    const token = argv[i];
    if (token == null) {
      break;
    }

    if (token === "--") {
      positionals.push(...argv.slice(i + 1));
      break;
    }

    if (token.startsWith("--")) {
      const eq = token.indexOf("=");
      const rawName = eq === -1 ? token.slice(2) : token.slice(2, eq);
      const name = kebabToCamel(rawName);

      if (name === "noJson") {
        globals.json = false;
        i += 1;
        continue;
      }

      const inlineValue = eq === -1 ? undefined : token.slice(eq + 1);
      const next = argv[i + 1];
      const hasSeparateValue = inlineValue == null && next != null && !next.startsWith("-");

      const value: string | boolean =
        inlineValue != null
          ? parseFlagValue(inlineValue)
          : hasSeparateValue && next != null
            ? parseFlagValue(next)
            : true;

      if (name === "apiKey") {
        globals.apiKey = typeof value === "string" ? value : undefined;
      } else if (name === "baseUrl") {
        globals.baseUrl = typeof value === "string" ? value : undefined;
      } else if (name === "json") {
        globals.json = value !== false;
      } else {
        flags[name] = value;
      }

      i += hasSeparateValue ? 2 : 1;
      continue;
    }

    if (resource == null) {
      resource = token;
      i += 1;
      continue;
    }

    if (command == null) {
      command = token;
      i += 1;
      continue;
    }

    positionals.push(token);
    i += 1;
  }

  return { globals, resource, command, positionals, flags };
};

export const getStringFlag = (
  flags: Record<string, string | boolean>,
  name: string,
): string | undefined => {
  const value = flags[name];
  return typeof value === "string" ? value : undefined;
};

export const getBooleanFlag = (
  flags: Record<string, string | boolean>,
  name: string,
): boolean | undefined => {
  const value = flags[name];
  if (typeof value === "boolean") {
    return value;
  }
  if (value === "true") {
    return true;
  }
  if (value === "false") {
    return false;
  }
  return undefined;
};

export const getNumberFlag = (
  flags: Record<string, string | boolean>,
  name: string,
): number | undefined => {
  const value = flags[name];
  if (typeof value !== "string") {
    return undefined;
  }
  const parsed = Number(value);
  return Number.isFinite(parsed) ? parsed : undefined;
};
