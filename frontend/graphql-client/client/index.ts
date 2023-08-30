"use client";

import {
  cacheExchange,
  createClient,
  fetchExchange,
  ssrExchange,
} from "@urql/next";
import { auth } from "../auth";

export const ssr = ssrExchange();

export const client = createClient({
  url: "http://localhost:3000/api/graphql",
  exchanges: [cacheExchange, auth, ssr, fetchExchange],
});
