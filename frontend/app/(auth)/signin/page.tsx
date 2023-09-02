"use client";

import { firebaseAuth } from "@/libs/firebase/client";
import type { AuthProvider } from "firebase/auth";
import { GoogleAuthProvider, signInWithPopup } from "firebase/auth";
import { signIn } from "next-auth/react";

const googleProvider = new GoogleAuthProvider();

const Page = () => {
  const handleOAuthSignIn = async (provider: AuthProvider) => {
    const credential = await signInWithPopup(firebaseAuth, provider);
    const token = await credential.user.getIdToken(true);
    const refreshToken = credential.user.refreshToken;

    signIn("credentials", {
      idToken: token,
      refreshToken,
      callbackUrl: "/",
    });
  };

  return (
    <main>
      <h1>Signin Page</h1>
      <button onClick={() => handleOAuthSignIn(googleProvider)}>Google</button>
    </main>
  );
};

export default Page;
