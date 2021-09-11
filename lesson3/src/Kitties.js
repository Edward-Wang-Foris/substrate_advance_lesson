import React, { useEffect, useState } from 'react'
import { Form, Grid } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

import KittyCards from './KittyCards'

export default function Kitties (props) {
  const { api, keyring } = useSubstrate()
  const { accountPair } = props

  const [kitties, setKitties] = useState([])
  const [status, setStatus] = useState('')

  //定义kittie的DNA
  const [kittyDNAs,setKittyDNAs] = useState([])
  //定义Kitty的所有者
  const [kittyOwnerList,setKittyOwnerList] = useState([])

  const fetchKitties = () => {
    // TODO: 在这里调用 `api.query.kittiesModule.*` 函数去取得猫咪的信息。
    // 你需要取得：
    //   - 共有多少只猫咪
    //   - 每只猫咪的主人是谁
    //   - 每只猫咪的 DNA 是什么，用来组合出它的形态
    let unsubscribe
    api.query.kittiesModule.kittiesCount(
      count => {
        if (count !== ''){
          //kitty的数量
          const kittyIdList = Array.from(Array(parseInt(count,10)),(v,k) => k)
          //kitty的拥有者
          api.query.kittiesModule.owner.multi(kittyIdList,kittyOwnerList => {
            setKittyOwnerList(kittyOwnerList)
          }).catch(console.error)
          //查询所有kitty的DNA
          api.query.kittiesModule.kitties.multi(kittyIdList, kittyDNA => {
            setKittyDNAs(kittyDNA)
          }).catch(console.error)
        }
      }
    ).then(
        unsub => unsubscribe = unsub
    ).catch(console.error)
  }

  const populateKitties = () => {
    // TODO: 在这里添加额外的逻辑。你需要组成这样的数组结构：
    //  ```javascript
    //  const kitties = [{
    //    id: 0,
    //    dna: ...,
    //    owner: ...
    //  }, { id: ..., dna: ..., owner: ... }]
    //  ```
    // 这个 kitties 会传入 <KittyCards/> 然后对每只猫咪进行处理
    const kitties = []
    for(let i = 0; i < kittyDNAs.length; i++){
      const kitty = {}
      kitty.id  = i
      kitty.dna = kittyDNAs[i].unwrap()
      kitty.owner = keyring.encodeAddress(kittyOwnerList[i].unwrap())
    }
    setKitties(kitties)
  }

  useEffect(fetchKitties, [api, keyring])
  useEffect(populateKitties, [])

  return <Grid.Column width={16}>
    <h1>小毛孩</h1>
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
