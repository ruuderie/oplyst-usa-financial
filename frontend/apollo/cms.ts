import { defineApolloClient } from '@nuxtjs/apollo';

export default defineApolloClient({
  // The GraphQL endpoint.
  httpEndpoint: `https://graphql.contentful.com/content/v1/spaces/${process.env.VUE_APP_CONTENTFUL_SPACE_ID}`,
  httpLinkOptions: {
    headers: {
        Authorization: `Bearer ${process.env.VUE_APP_CONTENTFUL_ACCESS_TOKEN}`,
    }
  },
  authType: 'Bearer',
  authHeader: 'Authorization',
  tokenStorage: 'cookie',
  defaultOptions: {
    watchQuery: {
      fetchPolicy: 'cache-and-network',
    },
  }

});
