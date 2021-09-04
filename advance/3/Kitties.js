import React, { useEffect, useState } from 'react'
import { Form, Grid } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

import KittyCards from './KittyCards'

export default function Kitties (props) {
  const { api, keyring } = useSubstrate()
  const { accountPair } = props
  const [status, setStatus] = useState('')

  const [kittyDNAs, setKittyDNAs] = useState([])
  const [kitties, setKitties] = useState([])
  const [kittyOwners, setKittyOwners] = useState([])
  const [kittyCount, setKittyCount] = useState(0)

  const fetchKitties = () => {
    // 你需要取得：
    //   - 共有多少只猫咪
    //   - 每只猫咪的主人是谁
    //   - 每只猫咪的 DNA 是什么，用来组合出它的形态

    api.query.kittiesModule.kittiesCount(cnt => {
      if (cnt !== '') {
        setKittyCount(parseInt(cnt, 10))
        const kittyIds = Array.from(Array(parseInt(cnt, 10)), (v, k) => k)
        api.query.kittiesModule.owner.multi(kittyIds, kittyOwners => {
          setKittyOwners(kittyOwners)
        }).catch(console.error)
        api.query.kittiesModule.kitties.multi(kittyIds, kittyDna => {
          setKittyDNAs(kittyDna)
        }).catch(console.error)
      }
    }).then(unsubscribe => {
      return () => unsubscribe()
    }).catch(console.error)

  }

  const populateKitties = () => {
    //  ```javascript
    //  const kitties = [{
    //    id: 0,
    //    dna: ...,
    //    owner: ...
    //  }, { id: ..., dna: ..., owner: ... }]
    //  ```
    // 这个 kitties 会传入 <KittyCards/> 然后对每只猫咪进行处理
    const kitties = []
    for (let i = 0; i < kittyDNAs.length; ++i) {

      kitties[i] = {
        id: i,
        dna: kittyDNAs[i].unwrap(),
        owner: keyring.encodeAddress(kittyOwners[i].unwrap())
      }
    }

    setKitties(kitties)
  }

  useEffect(fetchKitties, [api, keyring])
  useEffect(populateKitties, [])

  return <Grid.Column width={16}>
    <h1>小毛孩 总数:{kittyCount}</h1>
    <KittyCards kitties={kitties} accountPair={accountPair} setStatus={setStatus}/>
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          accountPair={accountPair} label='创建小毛孩' type='SIGNED-TX' setStatus={setStatus}
          attrs={{
            palletRpc: 'kittiesModule',
            callable: 'create',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>
}
