import type { VideoGen } from "@videogen/sdk";
import type { ParsedArgs } from "../args";

export type CommandContext = {
  client: VideoGen;
  args: ParsedArgs;
  wait: boolean;
};

export type CommandHandler = (ctx: CommandContext) => Promise<void>;

export type ResourceCommands = Record<string, CommandHandler>;
