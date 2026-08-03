import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { getBooleanFlag, parseArgs } from "./args";
import { createClient } from "./client";
import { RESOURCES, listUsage } from "./commands/index";
import { printError } from "./output";

const getPackageVersion = (): string => {
  const packageJsonPath = join(dirname(fileURLToPath(import.meta.url)), "..", "package.json");
  const packageJson: unknown = JSON.parse(readFileSync(packageJsonPath, "utf8"));
  if (
    packageJson != null &&
    typeof packageJson === "object" &&
    "version" in packageJson &&
    typeof packageJson.version === "string"
  ) {
    return packageJson.version;
  }
  throw new Error("Could not read version from package.json.");
};

const main = async (): Promise<void> => {
  const args = parseArgs(process.argv.slice(2));

  if (getBooleanFlag(args.flags, "version") === true) {
    process.stdout.write(`${getPackageVersion()}\n`);
    return;
  }

  if (args.resource == null || args.command == null) {
    process.stdout.write(listUsage());
    process.exitCode = args.resource == null && args.command == null ? 0 : 1;
    return;
  }

  const resourceCommands = RESOURCES[args.resource];
  if (resourceCommands == null) {
    throw new Error(`Unknown resource "${args.resource}".\n${listUsage()}`);
  }

  const handler = resourceCommands[args.command];
  if (handler == null) {
    throw new Error(`Unknown command "${args.resource} ${args.command}".\n${listUsage()}`);
  }

  const client = createClient({
    apiKey: args.globals.apiKey,
    baseUrl: args.globals.baseUrl,
  });

  const wait = getBooleanFlag(args.flags, "wait") === true;

  await handler({ client, args, wait });
};

main().catch((err: unknown) => {
  printError(err);
  process.exitCode = 1;
});
