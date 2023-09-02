import pino from "pino";
import { logflarePinoVercel } from "pino-logflare";

const { stream, send } = logflarePinoVercel({
  apiKey: process.env.NEXT_PUBLIC_LOGFLARE_KEY,
  sourceToken: process.env.NEXT_PUBLIC_LOGFLARE_STREAM,
});

export const logger = pino(
  {
    level: "debug",
    browser: {
      transmit: {
        send,
        level: "info",
      },
    },
    base: {
      env: process.env.NODE_ENV || "development",
      revision: process.env.VERCEL_GITHUB_COMMIT_SHA,
    },
  },
  stream
);
