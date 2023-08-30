import type { Metadata } from "next";
import { Inter } from "next/font/google";
import SessionProvider from "./session-provider";
import UrqlProvider from "./urql-provider";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "SaaS",
  description: "A SaaS Demo",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <SessionProvider>
          <UrqlProvider>{children}</UrqlProvider>
        </SessionProvider>
      </body>
    </html>
  );
}
