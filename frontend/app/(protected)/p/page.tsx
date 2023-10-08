import { graphql } from "@/gql";
import { execute } from "@/graphql/client";
import { ClientError } from "graphql-request";
import Link from "next/link";
import { redirect } from "next/navigation";

const PAGE_SIZE = 5;

const ProjectsQuery = graphql(/* GraphQL */ `
  query ProjectsQuery(
    $first: Int
    $last: Int
    $before: String
    $after: String
  ) {
    viewer {
      projects(first: $first, last: $last, before: $before, after: $after) {
        edges {
          node {
            id
            name
            slug
          }
        }
        pageInfo {
          hasPreviousPage
          hasNextPage
          startCursor
          endCursor
        }
      }
    }
  }
`);

const Page = async ({
  searchParams,
}: {
  searchParams: { [key: string]: string | string[] | undefined };
}) => {
  const cursor = searchParams.c?.toString() ?? null;
  const first = searchParams.f?.toString() ?? null;
  const last = searchParams.l?.toString() ?? null;
  const { viewer } = await execute({
    query: ProjectsQuery,
    variables: {
      first: first ? parseInt(first) : last ? null : PAGE_SIZE,
      last: last ? parseInt(last) : null,
      before: last ? cursor : null,
      after: first ? cursor : null,
    },
    fetchOptions: {
      next: { tags: ["viewer"] },
      cache: "no-cache",
    },
  }).catch((err) => {
    if (err instanceof ClientError) {
      console.error(err.response.errors);
      redirect("/signin");
    }
    throw err;
  });

  return (
    <main>
      <h1>Your Projects</h1>
      <div>
        {viewer.projects.edges.map(({ node }: { node: any }) => (
          <div key={node.id}>
            <Link href={`/p/${node.slug}`}>{node.name}</Link>
          </div>
        ))}
      </div>
      <div className="flex gap-2 mt-2">
        {viewer.projects.pageInfo.hasPreviousPage &&
          viewer.projects.pageInfo.startCursor && (
            <Link
              href={{
                pathname: "/p",
                query: {
                  l: PAGE_SIZE,
                  c: viewer.projects.pageInfo.startCursor,
                },
              }}
            >
              Previous
            </Link>
          )}
        {viewer.projects.pageInfo.hasNextPage &&
          viewer.projects.pageInfo.endCursor && (
            <Link
              href={{
                pathname: "/p",
                query: {
                  f: PAGE_SIZE,
                  c: viewer.projects.pageInfo.endCursor,
                },
              }}
            >
              Next
            </Link>
          )}
      </div>
    </main>
  );
};

export default Page;
