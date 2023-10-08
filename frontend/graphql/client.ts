"use server";
import { authOptions } from "@/app/(auth)/api/auth/[...nextauth]/route";
import { backendUrl } from "@/utils/endpoint";
import {
  TypedDocumentNode,
  VariablesOf,
  ResultOf,
} from "@graphql-typed-document-node/core";
import { GraphQLClient } from "graphql-request";
import { getServerSession } from "next-auth/next";

type FetchOptions = {
  headers?: HeadersInit;
  next?: NextFetchRequestConfig;
  cache?: RequestCache;
};

const getClient = async (fetchOptions?: FetchOptions) => {
  return new GraphQLClient(`${backendUrl}/graphql`, {
    fetch,
    ...fetchOptions,
  });
};

export const execute = async <Document extends TypedDocumentNode<any, any>>({
  query,
  variables,
  fetchOptions,
}: {
  query: Document;
  variables?: VariablesOf<Document>;
  fetchOptions?: FetchOptions;
}): Promise<ResultOf<Document>> => {
  const client = await getClient(fetchOptions);
  const session = await getServerSession(authOptions);
  if (session) {
    client.setHeader("Authorization", `Bearer ${session.user.accessToken}`);
  }
  return client.request(query, variables);
};
