import NextAuth from "next-auth/next";
import { AuthOptions, User } from "next-auth";
import CredentialsProvider from "next-auth/providers/credentials";
import { graphql } from "@/gql";
import invariant from "tiny-invariant";
import { logger } from "@/libs/logging/logger";
import { execute } from "@/graphql/client";

const SigninMutation = graphql(/* GraphQL */ `
  mutation Signin($token: String!) {
    signin(token: $token) {
      id
      uid
      name
      email
      emailVerified
      image
    }
  }
`);

export const authOptions: AuthOptions = {
  debug: process.env.NODE_ENV === "development",
  providers: [
    CredentialsProvider({
      type: "credentials",
      credentials: {},
      async authorize(credentials: any) {
        const { idToken, refreshToken } = credentials;

        const data = await execute({
          query: SigninMutation,
          variables: {
            token: idToken,
          },
        });

        invariant(data?.signin, "signin failed");
        const { uid, name, email, emailVerified, image } = data.signin;
        return {
          uid,
          name,
          email,
          email_verified: emailVerified,
          image,
          accessToken: idToken,
        } as any as User;
      },
    }),
  ],
  callbacks: {
    async jwt({ token, account, user, profile }) {
      if (user) {
        return {
          uid: user.uid,
          name: user.name,
          email: user.email,
          emailVerified: user.email_verified,
          image: user.image,
          accessToken: user.accessToken,
        };
      }
      return token;
    },
    async session({ session, token }) {
      session.user = token;
      return session;
    },
  },
  session: {
    strategy: "jwt",
  },
  logger: {
    debug: (code: string, ...message) => {
      logger.debug(code, ...message);
    },
    error: (code: string, ...message) => {
      logger.error(code, ...message);
    },
    warn: (code: string, ...message) => {
      logger.warn(code, ...message);
    },
  },
  pages: {
    signIn: "/auth/signin",
  },
};

const handler = NextAuth(authOptions);

export { handler as GET, handler as POST };
