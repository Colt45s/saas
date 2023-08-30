import { authOptions } from "@/app/api/auth/[...nextauth]/route";
import { authExchange } from "@urql/exchange-auth";
import { getServerSession } from "next-auth";
import { getSession } from "next-auth/react";

const isServer = typeof window === "undefined";

export const auth = authExchange(async (utilities) => {
  const session = isServer
    ? await getServerSession(authOptions)
    : await getSession();
  const token = session?.user.accessToken;
  return {
    didAuthError: () => {
      // TODO
      return false;
    },
    willAuthError: () => {
      // TODO
      return false;
    },
    refreshAuth: async () => {
      // TODO
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
