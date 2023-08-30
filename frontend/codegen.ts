import { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
  schema: "http://127.0.0.1:4001/graphql",
  documents: ["./**/*.{ts,tsx}"],
  generates: {
    "./gql/": {
      preset: "client",
    },
  },
  config: {
    avoidOptionals: {
      field: true,
      object: true,
      inputValue: false,
      defaultValue: false,
    },
    scalars: {
      DateTime: "Date",
      Date: "Date",
    },
  },
};

export default config;
