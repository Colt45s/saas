import httpProxyMiddleware from "next-http-proxy-middleware";
import { NextApiHandler } from "next";
import { backendUrl } from "@/utils/endpoint";
import { isProd } from "@/utils/env";

const handler: NextApiHandler = (req, res) => {
  return httpProxyMiddleware(req, res, {
    target: backendUrl,
    pathRewrite: [
      {
        patternStr: "^/api/graphql",
        replaceStr: "/graphql",
      },
    ],
    followRedirects: true,
    changeOrigin: true,
    secure: isProd,
  });
};

export default handler;
