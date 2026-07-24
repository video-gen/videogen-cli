import { VideoGen } from "@videogen/sdk";

export type CliClientOptions = {
  apiKey: string | undefined;
  baseUrl: string | undefined;
};

export const createClient = ({ apiKey, baseUrl }: CliClientOptions): VideoGen => {
  const resolvedKey = apiKey ?? process.env["VIDEOGEN_API_KEY"];
  if (resolvedKey == null || resolvedKey.length === 0) {
    throw new Error("Missing API key. Set VIDEOGEN_API_KEY or pass --api-key.");
  }

  const resolvedBaseUrl = baseUrl ?? process.env["VIDEOGEN_BASE_URL"];
  if (resolvedBaseUrl == null || resolvedBaseUrl.length === 0) {
    return new VideoGen({ apiKey: resolvedKey, clientId: "cli" });
  }

  return new VideoGen({ apiKey: resolvedKey, baseUrl: resolvedBaseUrl, clientId: "cli" });
};
