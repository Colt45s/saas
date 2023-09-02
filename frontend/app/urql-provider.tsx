"use client";

import { getClient } from "@/graphql-client/client";
import { UrqlProvider as NextUrqlProvider, ssrExchange } from "@urql/next";
import { useMemo } from "react";

export default function UrqlProvider({ children }: React.PropsWithChildren) {
  const [client, ssr] = useMemo(() => {
    const ssr = ssrExchange();
    const client = getClient();

    return [client, ssr];
  }, []);

  return (
    <NextUrqlProvider client={client} ssr={ssr}>
      {children}
    </NextUrqlProvider>
  );
}
