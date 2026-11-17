import { accountCommands } from "./account";
import { assistantCommands } from "./assistant";
import { authCommands } from "./auth";
import { modelsCommands } from "./models";
import { billingCommands } from "./billing";
import { entitiesCommands } from "./entities";
import { filesCommands } from "./files";
import { legacyProjectsCommands, projectsCommands } from "./projects";
import { resourcesCommands } from "./resources";
import { textCommands } from "./text";
import { toolsCommands } from "./tools";
import type { ResourceCommands } from "./types";
import { webhooksCommands } from "./webhooks";
import { workflowsCommands } from "./workflows";

const visibleResources: Record<string, ResourceCommands> = {
  account: accountCommands,
  assistant: assistantCommands,
  auth: authCommands,
  billing: billingCommands,
  models: modelsCommands,
  entities: entitiesCommands,
  files: filesCommands,
  projects: projectsCommands,
  resources: resourcesCommands,
  text: textCommands,
  tools: toolsCommands,
  webhooks: webhooksCommands,
  workflows: workflowsCommands,
};

export const RESOURCES: Record<string, ResourceCommands> = {
  ...visibleResources,
  projects: { ...projectsCommands, ...legacyProjectsCommands },
};

export const listUsage = (): string => {
  const lines: string[] = [
    "Usage: videogen [globals] <resource> <command> [flags]",
    "       videogen login | videogen logout",
    "",
    "Globals:",
    "  --api-key <key>       API key (or VIDEOGEN_API_KEY)",
    "  --base-url <url>      API base URL (or VIDEOGEN_BASE_URL)",
    "  --json / --no-json    JSON stdout (default: on)",
    "",
    "Auth:",
    "  login                 Sign in with VideoGen (OAuth 2.1 + PKCE)",
    "  logout                Clear cached OAuth credentials",
    "",
    "Common flags:",
    "  --body '<json>'       JSON request body (or @file, or stdin)",
    "  --wait                Poll until complete (tools/workflows/exports/video edits)",
    "",
    "Resources:",
  ];

  for (const resource of Object.keys(visibleResources).sort()) {
    const commands = visibleResources[resource];
    if (commands == null) {
      continue;
    }
    const names = Object.keys(commands).sort().join(", ");
    lines.push(`  ${resource}: ${names}`);
  }

  return `${lines.join("\n")}\n`;
};
