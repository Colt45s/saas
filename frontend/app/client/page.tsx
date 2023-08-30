"use client";

import { Suspense } from "react";
import { useQuery } from "@urql/next";
import { graphql } from "@/gql";

const ViewerQuery = graphql(/* GraphQL */ `
  query ViewerQuery {
    viewer {
      id
      name
    }
  }
`);

const Page = () => {
  return (
    <Suspense>
      <Inner />
    </Suspense>
  );
};

const Inner = () => {
  const [result] = useQuery({ query: ViewerQuery });

  return (
    <main>
      <h1>Client Page</h1>
      <p>{result.data?.viewer.name}</p>
    </main>
  );
};

export default Page;
