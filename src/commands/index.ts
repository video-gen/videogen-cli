import { accountCommands } from "./account";
import { assistantCommands } from "./assistant";
import { filesCommands } from "./files";
import { projectsCommands } from "./projects";
import { resourcesCommands } from "./resources";
import { textCommands } from "./text";
import { toolsCommands } from "./tools";
import type { ResourceCommands } from "./types";
import { webhooksCommands } from "./webhooks";
import { workflowsCommands } from "./workflows";

export const RESOURCES: Record<string, ResourceCommands> = {
  account: accountCommands,
  assistant: assistantCommands,
  files: filesCommands,
  projects: projectsCommands,
  resources: resourcesCommands,
  text: textCommands,
  tools: toolsCommands,
  webhooks: webhooksCommands,
  workflows: workflowsCommands,
};

export const listUsage = (): string => {
  const lines: string[] = [
    "Usage: videogen [globals] <resource> <command> [flags]",
    "",
    "Globals:",
    "  --api-key <key>       API key (or VIDEOGEN_API_KEY)",
    "  --base-url <url>      API base URL (or VIDEOGEN_BASE_URL)",
    "  --json / --no-json    JSON stdout (default: on)",
    "",
    "Common flags:",
    "  --body '<json>'       JSON request body (or @file, or stdin)",
    "  --wait                Poll until complete (tools/workflows/export/remix)",
    "",
    "Resources:",
  ];

  for (const resource of Object.keys(RESOURCES).sort()) {
    const commands = RESOURCES[resource];
    if (commands == null) {
      continue;
    }
    const names = Object.keys(commands).sort().join(", ");
    lines.push(`  ${resource}: ${names}`);
  }

  return `${lines.join("\n")}\n`;
};
