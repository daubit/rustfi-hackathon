import {
  Button,
  Container,
  Divider,
  Modal,
  ModalBody,
  ModalCloseButton,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalOverlay,
  Spinner,
  Table,
  TableContainer,
  Tbody,
  Td,
  Th,
  Thead,
  Tr,
  useDisclosure,
} from "@chakra-ui/react";
import { usePools } from "../hooks/usePools";

interface Token {
  name: String | null;
  symbol: String | null;
  total_supply: String | null;
  address: String | null;
  decimals: number | null;
}

interface Denom {
  native: String | null;
  cw20: String | null;
}

enum Chain {
  JUNO,
  OSMOSIS,
}
interface Pool {
  chain?: Chain | null;
  pool_address: String | null;
  lp_token_address: String | null;
  lp_token_supply: String | null;
  token1: Token | null;
  token1_denom: Denom | null;
  token1_reserve: String | null;
  token2: Token | null;
  token2_denom: Denom | null;
  token2_reserve: String | null;
}

interface PoolProps {
  pool: Pool;
}

const Pool = (props: PoolProps) => {
  const { isOpen, onOpen, onClose } = useDisclosure();
  const { pool } = props;
  return (
    <>
      <Tr onClick={onOpen}>
        <Td>{pool.token1?.symbol}</Td>
        <Td>{pool.token2?.symbol}</Td>
        <Td>{pool.pool_address}</Td>
        <Td isNumeric>{pool.token1_reserve}</Td>
        <Td isNumeric>{pool.token2_reserve}</Td>
      </Tr>
      <Modal isOpen={isOpen} onClose={onClose}>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>Pool View</ModalHeader>
          <ModalCloseButton />
          <ModalBody>
            <Container gap="2rem">
              <p>Pool Address: {pool.pool_address}</p>
              <Divider />
              <p>
                {pool.token1?.symbol}: {pool.token1?.address}
              </p>
              <Divider />
              <p>
                {pool.token2?.symbol}: {pool.token2?.address}
              </p>
            </Container>
          </ModalBody>
          <ModalFooter>
            <Button colorScheme="blue" mr={3} onClick={onClose}>
              Close
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </>
  );
};

export const Pools = () => {
  const { data, isLoading } = usePools();
  if (isLoading) {
    return (
      <Spinner
        thickness="4px"
        speed="0.65s"
        emptyColor="gray.200"
        color="blue.500"
        size="xl"
      />
    );
  }
  return (
    <TableContainer overflowY={"scroll"} maxHeight={"md"}>
      <Table variant="simple">
        <Thead>
          <Tr>
            <Th>Token 1</Th>
            <Th>Token 2</Th>
            <Th>Address</Th>
            <Th isNumeric>Reserve 1</Th>
            <Th isNumeric>Reserve 2</Th>
          </Tr>
        </Thead>
        <Tbody>
          {data?.map((pool, key) => (
            <Pool key={key} pool={pool} />
          ))}
        </Tbody>
      </Table>
    </TableContainer>
  );
};
