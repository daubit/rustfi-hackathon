import { useQuery } from "react-query";
// import pools from "../assets/pools.json";

const fetchPools = async () => {
  const pools = await (await fetch("http://localhost:8080/pools")).json();
  return pools;
};

const usePools = () => {
  return useQuery("pools", fetchPools);
};

export { usePools, fetchPools };
