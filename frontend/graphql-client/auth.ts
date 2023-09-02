import { authOptions } from "@/app/(auth)/api/auth/[...nextauth]/route";
import { logger } from "@/libs/logging/logger";
import { authExchange } from "@urql/exchange-auth";
import { getServerSession } from "next-auth";
import { getSession } from "next-auth/react";

const isServer = typeof window === "undefined";

export const auth = authExchange(async (utilities) => {
  const session = isServer
    ? await getServerSession(authOptions)
    : await getSession();
  const token = session?.user.accessToken;
  logger.info(
    {
      token,
    },
    "[authExchange] access token"
  );
  return {
    didAuthError: () => {
      return false;
    },
    willAuthError: () => {
      return false;
    },
    refreshAuth: async () => {
      return;
    },
    addAuthToOperation: (operation) => {
      return token
        ? utilities.appendHeaders(operation, {
            Authorization: `Bearer ${token}`,
          })
        : operation;
    },
  };
});
