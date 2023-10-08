import { graphql } from "@/gql";
import { execute } from "@/graphql/client";
import { ClientError } from "graphql-request";
import Link from "next/link";
import { redirect } from "next/navigation";

const ViewerQuery = graphql(/* GraphQL */ `
  query ViewerQuery {
    viewer {
      id
      name
    }
  }
`);

const Page = async () => {
  const { viewer } = await execute({
    query: ViewerQuery,
    variables: {},
    fetchOptions: {
      next: { tags: ["viewer"] },
      cache: "force-cache",
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
      <h1>hello {viewer.name}</h1>
      <Link href="/p">Projects</Link>
    </main>
  );
};

export default Page;
