import { useQuery } from "react-query";
import pools from "../assets/pools.json";

const fetchPools = async () => {
  //   const parsed = await (await fetch("http://localhost:3000/pools")).json();
  return pools;
};

const usePools = () => {
  return useQuery("pools", fetchPools);
};

export { usePools, fetchPools };
