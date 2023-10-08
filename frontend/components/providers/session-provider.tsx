"use client";

import { SessionProvider as NextAuthSessionProvider } from "next-auth/react";

const SessionProvider: typeof NextAuthSessionProvider = (props) => (
  <NextAuthSessionProvider {...props} />
);
export default SessionProvider;
