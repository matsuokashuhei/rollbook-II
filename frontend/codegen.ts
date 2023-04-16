import type { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
  overwrite: true,
  schema: "http://backend:3000",
  documents: "src/**/*.tsx",
  generates: {
    "src/gql/": {
      preset: "client",
      plugins: []
    },
    "./graphql.schema.json": {
      plugins: ["introspection"]
    }
  },
  watch: true,
  watchConfig: {
    // Passed directly through to chokidar's file watch configuration
    usePolling: true,
    interval: 1000
  }

};

export default config;
