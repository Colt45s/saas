"use client";

import UrqlProvider from "@/components/providers/urql-provider";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <UrqlProvider>{children}</UrqlProvider>
      </body>
    </html>
  );
}
