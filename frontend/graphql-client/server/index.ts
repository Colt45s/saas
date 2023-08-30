import { backendUrl } from "@/utils/endpoint";
import { cacheExchange, createClient, fetchExchange } from "@urql/core";
import { registerUrql } from "@urql/next/rsc";
import { auth } from "../auth";

const makeClient = () => {
  return createClient({
    url: `${backendUrl}/graphql`,
    exchanges: [cacheExchange, auth, fetchExchange],
  });
};

export const { getClient } = registerUrql(makeClient);
