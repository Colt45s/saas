import { backendUrl } from "@/utils/endpoint";
import { cacheExchange, createClient, fetchExchange } from "@urql/core";
import { registerUrql } from "@urql/next/rsc";
import { auth } from "./auth";

const isServer = typeof window === "undefined";

const makeClient = () => {
  return createClient({
    url: isServer ? `${backendUrl}/graphql` : "/api/graphql",
    exchanges: [cacheExchange, auth, fetchExchange],
    suspense: true,
  });
};

export const { getClient } = registerUrql(makeClient);
