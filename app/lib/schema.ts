const typeDefs = /* GraphQL */ `
  schema {
    query: QueryRoot
  }

  type Addon {
    id: Int!
    name: String!
  }

  type Category {
    id: Int!
    name: String!
  }

  type Game {
    id: ID!
    name: String!
    categoriesForGame: [Category!]!
  }

  type QueryRoot {
    addons: [Addon!]!
    games: [Game!]!
    categories: [Category!]!
  }
`

export default typeDefs
