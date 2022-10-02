import { useQuery } from "react-query";

const fetchPoolForDenom = async () => {
  const pools = await (await fetch("http://localhost:8080/pools_for_denom")).json();
  return pools;
};

const usePoolForDenom = () => {
  return useQuery("pool", fetchPoolForDenom);
};

export { usePoolForDenom, fetchPoolForDenom };
