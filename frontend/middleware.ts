import { NextRequest, NextResponse } from "next/server";
import { backendDomain, backendUrl } from "./utils/endpoint";

export function middleware(req: NextRequest) {
  const requestHeaders = new Headers(req.headers);
  requestHeaders.set("host", backendDomain);
  return NextResponse.rewrite(new URL("/graphql", backendUrl), {
    headers: requestHeaders,
  });
}

export const config = {
  api: {
    bodyParser: false,
  },
  matcher: "/api/graphql",
};
