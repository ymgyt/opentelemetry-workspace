import { FC } from 'react'
import { Heading } from '@chakra-ui/react'
import './App.css'
import SendButton from './componens/button'

const App: FC = () => (
  <>
    <Heading size="lg" as="h1" my={8}>
      🚀
    </Heading>
    <SendButton />
  </>
)

export default App
