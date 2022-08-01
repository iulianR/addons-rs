import { useMutation, useQuery } from '@apollo/client'
import { GetAllAddonsDocument } from 'lib/graphql-operations'

const Index = () => {
  const { data } = useQuery(GetAllAddonsDocument)

  const addons = data?.addons;

  return <div>
    <ul>
      {addons?.map((addon) => (
        <li key="{addon.id}">{addon.name}</li>
      ))}
    </ul>
  </div>
}

export default Index
