import { getServerSession } from "next-auth";
import Link from "next/link";
import { authOptions } from "./api/auth/[...nextauth]/route";

const Page = async () => {
  const session = await getServerSession(authOptions);

  console.log("session", session);

  return (
    <main>
      <h1>Session Page</h1>
      <div>
        {session ? (
          JSON.stringify(session)
        ) : (
          <Link href="/api/auth/signin">Login</Link>
        )}
      </div>
    </main>
  );
};

export default Page;
