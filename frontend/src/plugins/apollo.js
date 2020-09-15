import Vue from 'vue';
import VueApollo from 'vue-apollo';
import {
    ApolloClient
} from 'apollo-client';
import {
    HttpLink
} from 'apollo-link-http';
import {
    InMemoryCache
} from 'apollo-cache-inmemory';

Vue.use(VueApollo);

const httpLink = new HttpLink({
    uri: 'http://api.local.test:8000/graphql',
})

const cache = new InMemoryCache()

const apolloClient = new ApolloClient({
    link: httpLink,
    cache,
})

export default new VueApollo({
    defaultClient: apolloClient,
});
