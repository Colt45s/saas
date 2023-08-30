import "next-auth/jwt";

declare module "next-auth" {
  interface User {
    id?: string;
    uid?: string;
    name?: string;
    email?: string;
    image?: string;
    accessToken?: string;
    email_verified?: boolean;
  }
  interface Session {
    user: {
      id?: string;
      uid?: string;
      name?: string;
      email?: string;
      image?: string;
      accessToken?: string;
    };
  }
}

declare module "next-auth/jwt" {
  interface JWT {
    id?: string;
    uid?: string;
    name?: string;
    email?: string;
    image?: string;
    accessToken?: string;
  }
}
