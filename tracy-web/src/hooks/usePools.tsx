import { useQuery } from "react-query";

const fetchPools = async () => {
  const pools = await (await fetch("http://localhost:8080/pools")).json();
  return pools;
};

const usePools = () => {
  return useQuery("pools", fetchPools);
};

export { usePools, fetchPools };
