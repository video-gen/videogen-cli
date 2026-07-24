import { VideoGenError } from "@videogen/sdk";

export const printJson = (value: unknown): void => {
  process.stdout.write(`${JSON.stringify(value, null, 2)}\n`);
};

export const printError = (err: unknown): void => {
  if (err instanceof VideoGenError) {
    const payload = {
      error: err.message,
      status: err.status,
      requestId: err.requestId,
      body: err.body,
    };
    process.stderr.write(`${JSON.stringify(payload, null, 2)}\n`);
    return;
  }

  const message = err instanceof Error ? err.message : String(err);
  process.stderr.write(`${JSON.stringify({ error: message }, null, 2)}\n`);
};
