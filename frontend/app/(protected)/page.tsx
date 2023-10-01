"use client";

import { graphql } from "@/gql";
import { useQuery } from "@urql/next";
import Link from "next/link";
import { redirect } from "next/navigation";
import { startTransition, useCallback, useState } from "react";

const PAGE_SIZE = 10;

const ViewerQuery = graphql(/* GraphQL */ `
  query ViewerQuery($first: Int!, $cursor: String) {
    viewer {
      id
      name
      projects(first: $first, after: $cursor) {
        edges {
          node {
            id
            name
            slug
          }
        }
        pageInfo {
          hasNextPage
          endCursor
        }
      }
    }
  }
`);

const Page = () => {
  const [pageVariables, setPageVariables] = useState<{
    first: number;
    cursor: string | null;
  }>({
    first: PAGE_SIZE,
    cursor: null,
  });
  const [result] = useQuery({
    query: ViewerQuery,
    variables: pageVariables,
  });

  const nextPage = useCallback((nextCursor: string | null) => {
    startTransition(() => {
      setPageVariables((prev) => ({
        ...prev,
        first: PAGE_SIZE,
        cursor: nextCursor,
      }));
    });
  }, []);

  const data = result.data;
  const viewer = data?.viewer;
  console.log(result.error);

  if (!viewer) {
    console.log(result.error?.networkError);
    redirect("/signin");
  }

  const pageInfo = viewer?.projects.pageInfo;

  return (
    <main>
      <h1>hello {viewer.name}</h1>
      <div>
        {viewer.projects.edges.map(({ node }) => (
          <div key={node.id}>
            <Link href={`/p/${node.slug}`}>{node.name}</Link>
          </div>
        ))}
      </div>
      {pageInfo.hasNextPage && pageInfo.endCursor && (
        <button onClick={() => nextPage(pageInfo.endCursor)}>Next</button>
      )}
    </main>
  );
};

export default Page;
