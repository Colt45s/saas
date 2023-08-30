"use client";

import { client, ssr } from "@/graphql-client/client";
import { UrqlProvider as NextUrqlProvider } from "@urql/next";

export default function UrqlProvider({ children }: React.PropsWithChildren) {
  return (
    <NextUrqlProvider client={client} ssr={ssr}>
      {children}
    </NextUrqlProvider>
  );
}
