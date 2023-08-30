import { isProd } from "./env";

const protocol = isProd ? "https" : "http";
const domain = isProd ? process.env.NEXT_DOMAIN : "localhost:4001";
export const backendUrl = `${protocol}://${domain}`;
