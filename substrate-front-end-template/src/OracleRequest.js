import React, { useEffect, useState } from 'react';
import { Table, Grid } from 'semantic-ui-react';
import { useSubstrate } from './substrate-lib';

export default function Main (props) {
  const { api } = useSubstrate();
  const [requestsMap, setRequestsMap] = useState(new Map());

  useEffect(() => {
    let unsubscribe;

    api.query.aresModule.requests.entries(allEntries => {
      console.log(allEntries);

      for (var [key, value] of allEntries) {
        setRequestsMap(new Map(requestsMap.set(key, value)));
      }
      // for (var [k, v] of requestsMap) {
      //   console.log(k + ' = ' + v);
      // }
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [api.query.aresModule.requests]);

  return (
    <Grid.Column>
      <h1>Oracle Request</h1>
      <Table celled striped size='small'>
        <Table.Header>
          <Table.Row>
            {/* <Table.HeaderCell>ID</Table.HeaderCell> */}
            <Table.HeaderCell>AccountId</Table.HeaderCell>
            <Table.HeaderCell>BlockNumber</Table.HeaderCell>
            <Table.HeaderCell>SpecIndex</Table.HeaderCell>
          </Table.Row>
        </Table.Header>

        <Table.Body>{[...requestsMap.keys()].map(k =>
          <Table.Row key={k}>
            {/* <Table.Cell width={3} textAlign='right'>{k}</Table.Cell> */}
            <Table.Cell width={10}>
              <span style={{ display: 'inline-block', minWidth: '31em' }}>
                {requestsMap.get(k).toHuman()[0]}
              </span>
            </Table.Cell>
            <Table.Cell width={3} textAlign='right'>{requestsMap.get(k).toHuman()[1]}</Table.Cell>
            <Table.Cell width={3} textAlign='right'>{requestsMap.get(k).toHuman()[2]}</Table.Cell>
          </Table.Row>
        )}
        </Table.Body>
      </Table>
    </Grid.Column>
  );
}
