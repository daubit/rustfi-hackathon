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
import { useAtomValue } from "jotai";
import { usePools } from "../hooks/usePools";
import { chainsAtom } from "../state/menu";
import { Pool } from "../types";

interface PoolProps {
  pool: Pool;
}

const Pool = (props: PoolProps) => {
  const { isOpen, onOpen, onClose } = useDisclosure();
  const { pool } = props;
  return (
    <>
      <Tr onClick={onOpen}>
        {pool.chain === "juno" && (
          <>
            <Td>{pool.chain}</Td>
            <Td>{pool.pool_address}</Td>
            <Td>{pool.token1?.symbol}</Td>
            <Td>{pool.token2?.symbol}</Td>
            <Td isNumeric>{pool.token1_reserve}</Td>
            <Td isNumeric>{pool.token2_reserve}</Td>
          </>
        )}
        {pool.chain === "osmosis" && (
          <>
            <Td>{pool.chain}</Td>
            <Td>{pool.pool_address}</Td>
            <Td>{pool.pool_assets?.at(0)?.token.native_name}</Td>
            <Td>{pool.pool_assets?.at(1)?.token.native_name}</Td>
            <Td isNumeric>{pool.pool_assets?.at(0)?.token.amount}</Td>
            <Td isNumeric>{pool.pool_assets?.at(0)?.token.amount}</Td>
          </>
        )}
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
  const chains = useAtomValue(chainsAtom);
  if (isLoading) {
    return (
      <Spinner
        thickness="4px"
        speed="0.65s"
        emptyColor="gray.200"
        color="blue.500"
        size="xl"
        marginTop={"5rem"}
        marginBottom={"5rem"}
      />
    );
  }
  const pools =
    (data as Pool[])?.filter((pool) => chains.includes(pool.chain || "")) || [];
  return (
    <TableContainer overflowY={"scroll"} maxHeight={"md"}>
      <Table variant="simple">
        <Thead>
          <Tr>
            <Th>Chain</Th>
            <Th>Pool Address</Th>
            <Th>Token 1</Th>
            <Th>Token 2</Th>
            <Th isNumeric>Reserve 1</Th>
            <Th isNumeric>Reserve 2</Th>
          </Tr>
        </Thead>
        <Tbody>
          {pools.map((pool, key) => (
            <Pool key={key} pool={pool} />
          ))}
        </Tbody>
      </Table>
    </TableContainer>
  );
};
