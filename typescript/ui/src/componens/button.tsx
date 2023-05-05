import { Box, Button, ButtonGroup, Stat, StatLabel } from "@chakra-ui/react";
import { FC, useState } from "react";
import { gql } from "../__generated__";
import { useLazyQuery } from '@apollo/client'
import { SemanticAttributes } from "@opentelemetry/semantic-conventions";
import otel from '@opentelemetry/api'

const HELLO_QUERY = gql(`query Hello {
    hello
  } `)
 
const SendButton: FC = ()=>  {

  const [count, setCount] = useState(0)
  const [_getHello, { data, refetch }] = useLazyQuery(
    HELLO_QUERY, {variables: {}}
  )

  const onClick = () => {
    setCount((prev) => prev + 1)

    const tracer = otel.trace.getTracer('my-component')
    tracer.startActiveSpan(
      'customSpan', 
      { attributes: {[SemanticAttributes.ENDUSER_ID]: 'me' }},
      (span) => {

        // TODO:
        // * Links
        // * Status
        // * Record exceptions
      span.setAttribute("click.count", count)

      span.addEvent('event-x', {
          'custom.xxx': "yyy",
          'custom.yyy': "aaa",
        })
      span.end()
    })
    refetch()
  }

  return (
    <Box p={3} w="sm" borderWidth="1px" borderRadius="lg" boxShadow="base">
      <Stat mb={1}>
          <StatLabel>{data?.hello ?? "" }</StatLabel> 
          <StatLabel>{count}</StatLabel> 
      </Stat>
      <ButtonGroup maxW="xs" m={2} variant="contained" isAttached>
        <Button w="xs" colorScheme="green" variant="solid" onClick={onClick}>
          Send Request
        </Button>
    </ButtonGroup>
    </Box>    
  )
}

export default SendButton