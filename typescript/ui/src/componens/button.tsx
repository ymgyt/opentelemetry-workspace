import { Box, Button, ButtonGroup, Stat, StatLabel } from "@chakra-ui/react";
import { FC } from "react";
import { gql } from "../__generated__";
import { useLazyQuery } from '@apollo/client'

const HELLO_QUERY = gql(`query Hello {
    hello
  } `)
 
const SendButton: FC = ()=>  {

  const [_getHello, { data, refetch }] = useLazyQuery(
    HELLO_QUERY, {variables: {}}
  )

  const onClick = () => {
    refetch()
  }

  return (
    <Box p={3} w="sm" borderWidth="1px" borderRadius="lg" boxShadow="base">
      <Stat mb={1}>
          <StatLabel>{data?.hello ?? "" }</StatLabel> 
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