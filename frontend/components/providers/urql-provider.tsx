"use client";

import { auth } from "@/graphql-client/auth";
import { backendUrl } from "@/utils/endpoint";
import {
  UrqlProvider as NextUrqlProvider,
  cacheExchange,
  createClient,
  fetchExchange,
  ssrExchange,
} from "@urql/next";
import { useMemo } from "react";

const isServer = typeof window === "undefined";

export default function UrqlProvider({ children }: React.PropsWithChildren) {
  const [client, ssr] = useMemo(() => {
    const ssr = ssrExchange();
    const client = createClient({
      url: isServer ? `${backendUrl}/graphql` : "/api/graphql",
      suspense: true,
      exchanges: [cacheExchange, ssr, auth, fetchExchange],
    });
    return [client, ssr];
  }, []);
  return (
    <NextUrqlProvider client={client} ssr={ssr}>
      {children}
    </NextUrqlProvider>
  );
}
