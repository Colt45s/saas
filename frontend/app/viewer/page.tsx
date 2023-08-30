import { graphql } from "@/gql/gql";
import { getClient } from "@/graphql-client/server";
import Link from "next/link";

const ViewerQuery = graphql(/* GraphQL */ `
  query ViewerQuery {
    viewer {
      id
      name
    }
  }
`);

const Page = async () => {
  const { data } = await getClient().query(ViewerQuery, {});

  return (
    <main>
      <h1>Viewer Page</h1>
      {data ? (
        data.viewer ? (
          <>
            <p>{data.viewer.id}</p>
            <p>{data.viewer.name}</p>
          </>
        ) : (
          <Link href="/api/auth/signin">Login</Link>
        )
      ) : (
        <Link href="/api/auth/signin">Login</Link>
      )}
    </main>
  );
};

export default Page;
